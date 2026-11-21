
pub fn run_embedded_opentype() {
    use document::{ControlledLiftEvent as Lift};
    use crate::controlled_graph::{PdfByteGraphEvent as Graph, PdfFilePreparationCursor};
    use crate::font_model::{PdfFontKind, PdfFontProgram};
    use crate::content_operators::{PdfTextArrayItem, PdfTextString};
    const COPY: usize = 32_768;
    const RELEASE: usize = 1 << 20;
    const BODY: usize = 128;

    struct Borrow<'a> { objects: &'a [PdfIndirectObject], program_reads: usize }
    impl ObjectSource for Borrow<'_> {
        fn get(&mut self, id: ObjRef) -> Option<PdfObject> {
            assert!(id.num != 7 && id.num != 8, "native program and map must be borrowed");
            self.objects.iter().find(|object| object.id == id).map(|object| object.value.clone())
        }

        fn inspect(&mut self, id: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
            if id.num == 7 { self.program_reads += 1; }
            read(self.objects.iter().find(|object| object.id == id).map(|object| &object.value));
        }
    }

    for (pdf, expected, cancel) in [
        (include_bytes!("@ASSETS@/raw.pdf").as_slice(), "@RAW@", false),
        (include_bytes!("@ASSETS@/flate.pdf").as_slice(), "@FLATE@", false),
        (include_bytes!("@ASSETS@/raw.pdf").as_slice(), "@RAW@", true),
    ] {
        assert!(pdf.len() > COPY);
        let mut cursor = PdfFilePreparationCursor::new(256, COPY - BODY);
        let mut turns = 0;
        let file = loop {
            let empty = cursor.reserving_empty_backing();
            let grant = CosGrant { work: 1, allocation_bytes: if empty { RELEASE } else { COPY - BODY }, release_bytes: RELEASE };
            let turn = cursor.advance(pdf, grant, false);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            assert!(turn.work * BODY + if empty { 0 } else { turn.allocated_bytes } <= COPY);
            turns += turn.work;
            match turn.event {
                Ok(Graph::More | Graph::Object) => assert!(turns < 1_000_000),
                Ok(Graph::Complete) => break cursor.take().unwrap(),
                other => panic!("OpenType graph stopped at {turns}: {other:?}"),
            }
        };
        let mut source = Borrow { objects: &file.graph.objects, program_reads: 0 };
        let mut lift = DocumentLiftCursor::controlled();
        let mut held = 0;
        let mut work = 0;
        loop {
            let empty = lift.reserving_empty_slots();
            let body = lift.next_inline_body_bytes().max(BODY);
            let grant = CosGrant { work: 1, allocation_bytes: if empty { RELEASE } else { COPY - body }, release_bytes: RELEASE };
            let cancelled = cancel && source.program_reads > 128;
            let step = lift.advance(&file.trailer, file.header.version(), &mut source, grant, cancelled);
            assert!(step.work <= 1 && step.allocated_bytes <= grant.allocation_bytes && step.released_bytes <= grant.release_bytes);
            assert!(step.work * body + if empty { 0 } else { step.allocated_bytes } <= COPY);
            work += step.work;
            held += step.allocated_bytes;
            held -= step.released_bytes;
            match step.event {
                Ok(Lift::More) => assert!(work < 2_000_000),
                Ok(Lift::Complete) => {
                    assert!(!cancel);
                    let result = lift.take_result().unwrap();
                    assert_eq!((result.pages.len(), result.fonts.len(), result.admitted_stream_roles.len()), (48, 1, 50));
                    let font = &result.fonts[0];
                    let PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { reference }), widths, .. } = &font.kind else { unreachable!() };
                    assert_eq!(reference.artifact_id, format!("pdf:s.stdio.font.opentype:sha256:{expected}"));
                    assert_eq!(widths, &vec![500.; 169]);
                    assert!(matches!(&result.admitted_stream_roles[0].value, PdfStreamRoleValue::UnicodeMap { mapping } if Some(mapping) == font.to_unicode.as_ref()));
                    assert!(matches!(&result.admitted_stream_roles[1].value, PdfStreamRoleValue::FontProgram { program: PdfFontProgram::OpenType { reference: held } } if held == reference));
                    assert_eq!((result.admitted_stream_roles[0].identity.owner.num, result.admitted_stream_roles[1].identity.owner.num), (8, 7));
                    for (page, role) in result.pages.iter().zip(&result.admitted_stream_roles[2..]) {
                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 6));
                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 7));
                        assert!(role.dependencies.iter().any(|identity| identity.owner.num == 8));
                        assert!(matches!(&role.value, PdfStreamRoleValue::Operators { content } if content == &page.content));
                        assert!(page.content.iter().any(|operator| matches!(operator, PdfOp::ShowText { text: PdfTextString::Codes { codes } } if codes == &[65, 103, 82, 233])));
                        assert!(page.content.iter().any(|operator| matches!(operator, PdfOp::ShowTextArray { items } if items == &[
                            PdfTextArrayItem::Codes { codes: vec![65, 103] }, PdfTextArrayItem::Adjust { amount: -5. }, PdfTextArrayItem::Codes { codes: vec![82, 233] },
                        ])));
                    }
                    if expected == "@RAW@" {
                        use crate::actual_content::{print_content, FontTable};
                        use crate::actual_fonts::FontCodec;
                        use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort;
                        use crate::writer::{serialize_document, DocumentTrailer, WriteOptions};
                        struct Port<'a>(&'a crate::ArtifactRef, &'a PdfObject);
                        impl PdfArtifactResourcePort for Port<'_> {
                            fn admit(&mut self, _: &str, _: PdfObject) -> crate::standards::v1_7::subsets::base::io::error::PResult<crate::ArtifactRef> { unreachable!() }
                            fn resolve(&self, reference: &crate::ArtifactRef) -> crate::standards::v1_7::subsets::base::io::error::PResult<PdfObject> {
                                assert_eq!(reference, self.0);
                                Ok(self.1.clone())
                            }
                        }
                        struct Table(FontCodec);
                        impl FontTable for Table {
                            fn font(&self, name: &str) -> Option<&FontCodec> { (name == "F1").then_some(&self.0) }
                        }
                        let mut export = file.graph.objects.clone();
                        let program = &export.iter().find(|entry| entry.id.num == 7).unwrap().value;
                        let fonts = Table(FontCodec::new(font, &Port(reference, program)).unwrap());
                        assert_eq!(fonts.0.decode_text(&[65, 103, 82, 233]).as_deref(), Some("AgRé"));
                        let page_streams: Vec<ObjRef> = export.iter().filter_map(|entry| {
                            let PdfObject::Dict(entries) = &entry.value else { return None; };
                            let PdfObject::Ref(reference) = &entries.iter().find(|entry| entry.key == "Contents")?.value else { return None; };
                            Some(*reference)
                        }).collect();
                        assert_eq!(page_streams.len(), 48);
                        for (page, owner) in result.pages.iter().zip(page_streams) {
                            let stream = &mut export.iter_mut().find(|entry| entry.id == owner).unwrap().value;
                            let PdfObject::Stream { data, .. } = stream else { unreachable!() };
                            *data = print_content(&page.content, &fonts).unwrap();
                        }
                        let trailer = DocumentTrailer { root: ObjRef { num: 1, gen: 0 }, info: None, id: None, extra: Vec::new() };
                        let emitted = serialize_document("1.7", &export, &trailer, &WriteOptions::default());
                        std::fs::write("@ASSETS@/emitted.pdf", emitted).unwrap();
                    }
                    println!("[DEBUG] embedded OpenType: {} source bytes, {turns} graph work, {work} semantic work, {held} retained bytes", pdf.len());
                    break;
                }
                Ok(Lift::Cancelled) => {
                    assert!(cancel && lift.take_result().is_none());
                    lift.begin_close();
                    for _ in 0..8192 {
                        let close = lift.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: RELEASE });
                        assert!(close.work <= 1 && close.released_bytes <= RELEASE && close.allocated_bytes == 0);
                        held -= close.released_bytes;
                        if close.event == Ok(Lift::Closed) { break; }
                        assert_eq!(close.event, Ok(Lift::More));
                    }
                    let partial = lift.take_partial().snapshot.unwrap();
                    assert!(partial.fonts.is_empty() && partial.admitted_stream_roles.is_empty());
                    let retained = partial.admitted_stream_roles.capacity() * std::mem::size_of::<PdfAdmittedStreamRole>()
                        + partial.pages.capacity() * std::mem::size_of::<PdfPage>()
                        + partial.schema.capacity() + partial.declared_version.capacity();
                    assert_eq!(held, retained);
                    println!("[DEBUG] cancelled OpenType font hashing: active font backing retired");
                    break;
                }
                other => panic!("OpenType semantic lift stopped at {work} with inline reserve {body}: {other:?}"),
            }
        }
    }
}
