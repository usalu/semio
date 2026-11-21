
#[test]
fn genuine_pdf_resolves_shared_images_in_order_and_retires_cancelled_samples() {
    use crate::actual_lexer::{CosGrant, parse_indirect_at, retain_stream_entry};
    use crate::object_model::PdfStreamFilter;
    use document::{ControlledLiftBarrier as Barrier, ControlledLiftEvent as Event};
    use crate::xobjects::PdfImageBody;
    use std::{mem::size_of, path::Path};

    let Some(input) = std::env::var_os("PDF_CONTROLLED_IMAGE_FIXTURE") else { return; };
    let bytes = std::fs::read(&input).unwrap();
    assert!(bytes.len() > 32_768);
    let packed = Path::new(&input).parent().unwrap();
    let footer = bytes.windows(9).rposition(|marker| marker == b"startxref").unwrap();
    let offset = std::str::from_utf8(&bytes[footer..]).unwrap().lines().nth(1).unwrap().parse::<usize>().unwrap();
    let mut lines = std::str::from_utf8(&bytes[offset..]).unwrap().lines();
    assert_eq!(lines.next(), Some("xref"));
    let count = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
    let mut objects = Vec::new();
    for line in lines.take(count).filter(|line| line.ends_with(" n ")) {
        let (id, mut value) = parse_indirect_at(&bytes, line[..10].parse().unwrap()).unwrap();
        if let PdfObject::Stream { dict, data, filters } = &mut value {
            if id.num == 5 {
                *data = std::fs::read(packed.join("5.samples")).unwrap();
                filters.push(PdfStreamFilter::Flate { predictor: None });
            }
            dict.retain(|entry| retain_stream_entry(entry, true));
        }
        objects.push(PdfIndirectObject { id, value });
    }

    let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(objects[0].id))];
    for mode in ["complete", "cancel", "missing-name", "mask", "short", "alias", "codec"] {
        let mut objects = objects.clone();
        match mode {
            "missing-name" => {
                let PdfObject::Stream { data, .. } = &mut objects[3].value else { unreachable!() };
                *data = b"q 72 0 0 72 20 20 cm /None Do Q\n".to_vec();
            }
            "mask" => {
                let PdfObject::Stream { dict, .. } = &mut objects[4].value else { unreachable!() };
                dict.push(PdfDictEntry::new("SMask", PdfObject::Ref(ObjRef { num: 11, gen: 0 })));
            }
            "short" => {
                let PdfObject::Stream { data, .. } = &mut objects[4].value else { unreachable!() };
                data.pop();
            }
            "alias" => {
                let PdfObject::Dict(dict) = &mut objects[5].value else { unreachable!() };
                dict.iter_mut().find(|entry| entry.key == "Resources").unwrap().value = PdfObject::Dict(vec![
                    PdfDictEntry::new("XObject", PdfObject::Dict(vec![
                        PdfDictEntry::new("Alias", PdfObject::Ref(ObjRef { num: 5, gen: 0 })),
                    ])),
                ]);
            }
            "codec" => {
                let PdfObject::Stream { filters, .. } = &mut objects[4].value else { unreachable!() };
                *filters = vec![PdfStreamFilter::Dct { color_transform: None }];
            }
            _ => {}
        }
        let mut source = GraphSource::new(&objects);
        let mut cursor = DocumentLiftCursor::controlled();
        assert!(size_of::<document::ControlledDocumentLiftCursor>() <= 32_768);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut counts = (0, 0, 0);
        let mut sample_reservations = 0;
        let mut release_quotes = 0;
        let full_rgb_lane = 72 * 72 * 3 * size_of::<u32>();
        for _ in 0..350_000 {
            let cancel = mode == "cancel" && sample_reservations == 2 && counts.0 > 15_660;
            let reserved = cursor.reserving_empty_slots();
            let turn = cursor.advance(&trailer, "1.7", &mut source, grant, cancel);
            assert!(turn.work <= grant.work && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            counts.0 += turn.work;
            counts.1 += turn.allocated_bytes;
            counts.2 += turn.released_bytes;
            if turn.allocated_bytes == full_rgb_lane {
                assert!(reserved && full_rgb_lane > 32_768);
                sample_reservations += 1;
            }
            grant = CosGrant { work: 1, ..Default::default() };
            match turn.event {
                Ok(Event::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(Event::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                Ok(Event::More) => {},
                Ok(Event::Complete) => {
                    assert_eq!(mode, "complete");
                    let output = cursor.take_result().unwrap();
                    assert_eq!(output.pages.len(), 4);
                    assert_eq!(output.images.iter().map(|image| image.id.as_str()).collect::<Vec<_>>(), ["Im0", "Im1", "Im2"]);
                    assert_eq!(output.admitted_stream_roles.len(), 7);
                    for (image, role) in output.images.iter().zip(&output.admitted_stream_roles) {
                        let PdfStreamRoleValue::Image { image: role_image } = &role.value else { panic!("image roles precede operators"); };
                        assert_eq!(role_image, image);
                        assert!(role.identity.path.is_empty());
                        assert_eq!(role.dependencies.as_slice(), std::slice::from_ref(&role.identity));
                        let expected = std::fs::read(packed.join(format!("{}.samples", role.identity.owner.num))).unwrap();
                        let PdfImageBody::Samples { values } = &image.body else { unreachable!(); };
                        assert_eq!(values.iter().copied().map(|value| value as u8).collect::<Vec<_>>(), expected);
                    }
                    for ((page, role), name) in output.pages.iter().zip(&output.admitted_stream_roles[3..]).zip(["Im0", "Im1", "Im2", "Im0"]) {
                        let PdfStreamRoleValue::Operators { content } = &role.value else { unreachable!(); };
                        assert_eq!(&page.content, content);
                        assert!(matches!(&content[1..3], [PdfOp::Transform { matrix }, PdfOp::PaintXObject { name: found }]
                            if *matrix == [72., 0., 0., 72., 20., 20.] && found == name));
                    }
                    println!("[DEBUG] {} file bytes, {} semantic work units, {} retained bytes", bytes.len(), counts.0, counts.1 - counts.2);
                    break;
                }
                outcome => {
                    assert_eq!(outcome, match mode {
                        "cancel" => Ok(Event::Cancelled),
                        "missing-name" => Ok(Event::Deferred(Barrier::Program)),
                        "mask" | "alias" | "codec" => Ok(Event::Deferred(Barrier::Resources)),
                        "short" => Err("image packed body does not match its declared geometry"),
                        _ => panic!("{outcome:?}"),
                    });
                    assert!(cursor.take_result().is_none());
                    cursor.begin_close();
                    for _ in 0..1024 {
                        let turn = cursor.close_step(grant);
                        assert!(turn.work <= 1 && turn.released_bytes <= grant.release_bytes);
                        counts.2 += turn.released_bytes;
                        grant.release_bytes = 0;
                        if let Ok(Event::NeedsRelease { bytes }) = turn.event {
                            if bytes == full_rgb_lane { release_quotes += 1; }
                            grant.release_bytes = bytes;
                        }
                        if turn.event == Ok(Event::Closed) { break; }
                    }
                    let partial = cursor.take_partial().snapshot.unwrap();
                    if mode == "cancel" {
                        assert_eq!(release_quotes, 2);
                        assert!(partial.images.is_empty() && partial.admitted_stream_roles.is_empty() && partial.pages.is_empty());
                        let table = partial.images.capacity() * size_of::<crate::xobjects::PdfImage>()
                            + partial.admitted_stream_roles.capacity() * size_of::<PdfAdmittedStreamRole>()
                            + partial.pages.capacity() * size_of::<PdfPage>();
                        assert_eq!(counts.1 - counts.2, table + partial.schema.capacity() + partial.declared_version.capacity());
                    }
                    if mode == "alias" { assert_eq!(partial.pages.len(), 1); }
                    break;
                }
            }
        }
        assert!(counts.0 < 349_999, "semantic lift did not settle with {mode}");
    }
}
