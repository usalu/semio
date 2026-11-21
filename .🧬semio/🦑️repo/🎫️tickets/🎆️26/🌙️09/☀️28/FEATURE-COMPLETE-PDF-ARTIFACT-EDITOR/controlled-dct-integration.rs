pub fn run_coded_image_pipeline() {
    use document::{ControlledLiftBarrier as Barrier, ControlledLiftEvent as Lift};
    use crate::{
        controlled_graph::{PdfByteGraph, PdfByteGraphCursor, PdfByteGraphEvent as Event},
        xobjects::PdfImageBody,
    };

@GRAPH@

    struct Borrowed<'a> {
        objects: &'a [PdfIndirectObject],
        image_reads: usize,
    }
    impl ObjectSource for Borrowed<'_> {
        fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
            let original = self.objects.iter().find(|value| value.id == reference)?;
            assert!(reference.num != 5 && reference.num != 8, "image must be borrowed");
            Some(original.value.clone())
        }

        fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
            if reference.num == 5 || reference.num == 8 { self.image_reads += 1; }
            read(self.objects.iter().find(|value| value.id == reference).map(|value| &value.value));
        }
    }

    for (pdf, mode) in [
        (include_bytes!("@ASSETS@/large.pdf").as_slice(), "complete"),
        (include_bytes!("@ASSETS@/large.pdf").as_slice(), "cancel"),
        (include_bytes!("@ASSETS@/UnknownXX.pdf").as_slice(), "opaque"),
        (include_bytes!("@ASSETS@/JPXDecode.pdf").as_slice(), "opaque"),
    ] {
        let file = graph(pdf, None).expect("qualified original byte graph");
        let trailer = file.trailer.into_entries();
        let mut source = Borrowed { objects: &file.graph.objects, image_reads: 0 };
        let mut cursor = DocumentLiftCursor::controlled();
        assert!(std::mem::size_of::<document::ControlledDocumentLiftCursor>() <= COPY);
        let mut work = 0;
        let mut live = 0;
        for _ in 0..350_000 {
            let inline = cursor.next_inline_body_bytes().max(BODY);
            assert!(inline < COPY);
            let empty = cursor.reserving_empty_slots();
            let grant = CosGrant { work: 1, allocation_bytes: if empty { RELEASE } else { COPY - inline }, release_bytes: RELEASE };
            let cancel = mode == "cancel" && source.image_reads >= 256;
            let step = cursor.advance(&trailer, file.header.version(), &mut source, grant, cancel);
            assert!(step.work <= 1 && step.allocated_bytes <= grant.allocation_bytes && step.released_bytes <= grant.release_bytes);
            assert!(step.work * inline + if empty { 0 } else { step.allocated_bytes } <= COPY);
            work += step.work;
            live += step.allocated_bytes;
            live -= step.released_bytes;
            match step.event {
                Ok(Lift::More) => {}
                Ok(Lift::Complete) => {
                    assert_eq!(mode, "complete");
                    let result = cursor.take_result().unwrap();
                    assert_eq!(result.images.len(), 2);
                    assert_eq!(result.admitted_stream_roles.len(), 4);
                    assert_eq!(result.pages.len(), 2);
                    for (index, (owner, name, expected)) in [(5, "Photo", "@RGB@"), (8, "Gray", "@GRAY@")].into_iter().enumerate() {
                        let image = &result.images[index];
                        assert_eq!((image.id.as_str(), image.width, image.height, image.bits_per_component), (name, 176, 176, 8));
                        let PdfImageBody::Artifact { reference } = &image.body else { panic!("DCT semantic image requires foreign custody"); };
                        assert_eq!(reference.artifact_id, format!("pdf:s.stdio.jpeg:sha256:{expected}"));
                        assert!(matches!(&result.admitted_stream_roles[index].value, PdfStreamRoleValue::Image { image: admitted } if admitted == image));
                        assert_eq!(result.admitted_stream_roles[index].identity.owner, ObjRef { num: owner, gen: 0 });
                        assert!(result.pages[index].content.iter().any(|op| matches!(op, PdfOp::PaintXObject { name: actual } if actual == name)));
                    }
                    assert!(result.objects.is_empty(), "borrowed image backing is owned by the original graph until publish");
                    if let Some(out) = std::env::var_os("PDF_DCT_EMITTED") {
                        use crate::writer::{serialize_document, DocumentTrailer, WriteOptions};
                        use crate::xref::ObjectSink;
                        struct NativeImageSink<'a>(&'a [PdfIndirectObject]);
                        impl ObjectSink for NativeImageSink<'_> {
                            fn add(&mut self, _: PdfObject) -> ObjRef { unreachable!() }
                            fn resolve_artifact(&self, reference: &crate::ArtifactRef) -> crate::standards::v1_7::subsets::base::io::error::PResult<PdfObject> {
                                use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfDctResourceIdentity;
                                for object in self.0 {
                                    let PdfObject::Stream { data, filters, .. } = &object.value else { continue; };
                                    let [crate::object_model::PdfStreamFilter::Dct { color_transform }] = filters.as_slice() else { continue; };
                                    let mut digest = PdfDctResourceIdentity::new(*color_transform).unwrap();
                                    digest.advance(data, data.len());
                                    if digest.reference() == *reference { return Ok(object.value.clone()); }
                                }
                                panic!("DCT resource reference failed to resolve");
                            }
                        }
                        let mut export = file.graph.objects.clone();
                        let mut sink = NativeImageSink(&file.graph.objects);
                        for (owner, image) in [5, 8].into_iter().zip(&result.images) {
                            let original = export.iter_mut().find(|entry| entry.id.num == owner).unwrap();
                            let lowered = crate::images::lower_image(image, &mut sink, &mut |_| None, &mut |_| None).unwrap();
                            let PdfObject::Stream { data: new, filters: new_pipeline, .. } = &lowered else { unreachable!() };
                            let PdfObject::Stream { data: old, filters: old_pipeline, .. } = &original.value else { unreachable!() };
                            assert_eq!((new, new_pipeline), (old, old_pipeline));
                            original.value = lowered;
                        }
                        let trailer = DocumentTrailer { root: ObjRef { num: 1, gen: 0 }, info: None, id: None, extra: Vec::new() };
                        let emitted = serialize_document("1.7", &export, &trailer, &WriteOptions::default());
                        std::fs::write(out, &emitted).unwrap();
                        let reopened = graph(&emitted, None).unwrap();
                        for owner in [5, 8] {
                            let value = |objects: &[PdfIndirectObject]| objects.iter().find(|entry| entry.id.num == owner).unwrap().value.clone();
                            assert_eq!(value(&reopened.graph.objects), value(&export));
                        }
                        println!("[DEBUG] lowered both DCT roles and wrote/reopened the native PDF byte graph");
                    }
                    println!("[DEBUG] two typed DCT images and page operators, semantic work={work}, retained={live}");
                    break;
                }
                Ok(Lift::Cancelled | Lift::Deferred(Barrier::Resources)) => {
                    assert_ne!(mode, "complete");
                    assert_eq!(step.event, if mode == "cancel" { Ok(Lift::Cancelled) } else { Ok(Lift::Deferred(Barrier::Resources)) });
                    assert!(cursor.take_result().is_none());
                    cursor.begin_close();
                    let mut release = 0;
                    for _ in 0..2048 {
                        let closed = cursor.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: release });
                        assert!(closed.work <= 1 && closed.released_bytes <= release);
                        live -= closed.released_bytes;
                        release = match closed.event {
                            Ok(Lift::More) => 0,
                            Ok(Lift::NeedsRelease { bytes }) => bytes,
                            Ok(Lift::Closed) => break,
                            outcome => panic!("{outcome:?}"),
                        };
                    }
                    let partial = cursor.take_partial().snapshot.unwrap();
                    if mode == "cancel" {
                        assert!(partial.images.is_empty() && partial.admitted_stream_roles.is_empty() && partial.pages.is_empty());
                        let retained = partial.images.capacity() * std::mem::size_of::<crate::xobjects::PdfImage>()
                            + partial.admitted_stream_roles.capacity() * std::mem::size_of::<PdfAdmittedStreamRole>()
                            + partial.pages.capacity() * std::mem::size_of::<PdfPage>()
                            + partial.schema.capacity() + partial.declared_version.capacity();
                        assert_eq!(live, retained);
                    }
                    println!("[DEBUG] {mode} semantic turn closed, remaining published-partial bytes={live}");
                    break;
                }
                outcome => panic!("{mode} at semantic work {work}: {outcome:?}"),
            }
        }
        assert!(work < 349_999);
    }
}
