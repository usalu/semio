use crate::actual_content::{FontTable, PdfFixedCodeWidth};
use crate::actual_fonts::FontCodec;
use crate::actual_lexer::CosGrant;
use crate::content_operators::{PdfOp, PdfTextArrayItem, PdfTextString};
use crate::object_model::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};
use crate::page_content::{PageContentBarrier, PageContentEvent as Event, PageProgramCursor};
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;
use std::mem::size_of;

struct FontBinding(Option<PdfFixedCodeWidth>);
impl FontTable for FontBinding {
    fn font(&self, _: &str) -> Option<&FontCodec> { None }
    fn controlled_code_width(&self, name: &str) -> Option<PdfFixedCodeWidth> {
        if name == "F1" { self.0 } else { None }
    }
}

fn page(program: &str) -> [PdfIndirectObject; 2] {
    let contents = ObjRef { num: 2, gen: 0 };
    [
        PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry::new("Contents", PdfObject::Ref(contents))]) },
        PdfIndirectObject { id: contents, value: PdfObject::Stream { dict: Vec::new(), data: program.as_bytes().into(), filters: Vec::new() } },
    ]
}

fn roundtrip_encoding(width: usize) -> FontCodec {
    use crate::standards::v1_7::subsets::base::{
        io::foreign_artifacts::NativePdfArtifactResources,
        schema::snapshot::{PdfCMap, PdfCidFont, PdfFont, PdfFontKind, PdfSimpleEncoding},
    };

    let kind = if width == 1 {
        PdfFontKind::TrueType {
            base_font: "NativeFixture".into(), encoding: PdfSimpleEncoding::default(),
            first_char: 0, widths: Vec::new(), descriptor: None, program: None,
        }
    } else {
        PdfFontKind::Type0 {
            base_font: "NativeFixture".into(), cmap: PdfCMap::Predefined { name: "Identity-H".into() },
            descendant: PdfCidFont {
                true_type: true, base_font: "NativeFixture".into(), system_info: Default::default(),
                descriptor: Default::default(), default_width: 1000., widths: Vec::new(), default_vertical: None,
                vertical_metrics: Vec::new(), cid_to_gid: None, program: None, extra: Vec::new(),
            },
        }
    };
    let font = PdfFont { id: "F1".into(), kind, to_unicode: None, extra: Vec::new() };
    FontCodec::new(&font, &NativePdfArtifactResources::from_objects(&[])).unwrap()
}

fn print_and_reopen(operators: &[PdfOp], width: usize, binding: &FontBinding) {
    struct Wire(FontCodec);
    impl FontTable for Wire {
        fn font(&self, name: &str) -> Option<&FontCodec> { (name == "F1").then_some(&self.0) }
    }
    let raw = crate::actual_content::print_content(operators, &Wire(roundtrip_encoding(width))).unwrap();
    let graph = page(std::str::from_utf8(&raw).unwrap());
    let mut source = GraphSource::new(&graph);
    let mut program = PageProgramCursor::new(graph[0].id);
    let grant = CosGrant { work: 1, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    for _ in 0..16_384 {
        let turn = program.advance_with_fonts(&mut source, binding, grant, false);
        match turn.event {
            Ok(Event::More) => {}
            Ok(Event::Complete) => {
                assert_eq!(program.take_program().unwrap(), operators);
                return;
            }
            result => panic!("{result:?}"),
        }
    }
    panic!("reopened controlled content program did not complete");
}

#[test]
fn controlled_font_codes_contract_preserves_physical_codes_and_exact_grants() {
    type Case = (&'static str, usize, &'static str, &'static [u32], &'static [&'static [u32]], &'static [f64]);
    let cases: &[Case] = &[
@CASES@
    ];
    fn retained(program: &[PdfOp]) -> usize {
        program.iter().map(|operator| match operator {
            PdfOp::SetFont { name, .. } => name.capacity(),
            PdfOp::ShowText { text: PdfTextString::Codes { codes } } => codes.capacity() * size_of::<u32>(),
            PdfOp::ShowTextArray { items } => items.capacity() * size_of::<PdfTextArrayItem>()
                + items.iter().map(|item| match item { PdfTextArrayItem::Codes { codes } => codes.capacity() * size_of::<u32>(), _ => 0 }).sum::<usize>(),
            _ => 0,
        }).sum()
    }
    for &(name, width, source, show, array, adjustments) in cases {
        let binding = FontBinding(Some(if width == 1 { PdfFixedCodeWidth::One } else { PdfFixedCodeWidth::Two }));
        let graph = page(source);
        let mut source = GraphSource::new(&graph);
        let mut cursor = PageProgramCursor::new(graph[0].id);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut held = 0;
        let mut work = 0;
        for _ in 0..16384 {
            let turn = cursor.advance_with_fonts(&mut source, &binding, grant, false);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            held += turn.allocated_bytes;
            held -= turn.released_bytes;
            work += turn.work;
            grant = CosGrant { work: 1, ..Default::default() };
            match turn.event {
                Ok(Event::More) => {}
                Ok(Event::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(Event::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                Ok(Event::Complete) => {
                    let operators = cursor.take_program().unwrap();
                    let show_op = operators.iter().find(|operator| matches!(operator, PdfOp::ShowText { .. })).unwrap();
                    assert!(matches!(show_op, PdfOp::ShowText { text: PdfTextString::Codes { codes } } if codes == show), "{name}");
                    let mut chunks = Vec::new();
                    let mut shifts = Vec::new();
                    if let Some(PdfOp::ShowTextArray { items }) = operators.iter().find(|operator| matches!(operator, PdfOp::ShowTextArray { .. })) {
                        for item in items {
                            match item {
                                PdfTextArrayItem::Codes { codes } => chunks.push(codes.as_slice()),
                                PdfTextArrayItem::Adjust { amount } => shifts.push(*amount),
                                _ => panic!("{name}: text alias lost its physical code"),
                            }
                        }
                    }
                    assert_eq!(chunks, array, "{name}");
                    assert_eq!(shifts, adjustments, "{name}");
                    assert_eq!(held, operators.capacity() * size_of::<PdfOp>() + retained(&operators), "{name}");
                    print_and_reopen(&operators, width, &binding);
                    println!("[DEBUG] {name}: {work} one-work turns, {held} transferred allocation bytes");
                    break;
                }
                _ => panic!("{name}: {:?}", turn.event),
            }
        }
        assert!(work < 16384);
    }
}

#[test]
fn controlled_font_codes_contract_refuses_partial_codes_and_releases_on_cancellation() {
    for (program, binding, outcome) in [
        ("BT /F1 12 Tf <004100420043> Tj ET", FontBinding(Some(PdfFixedCodeWidth::Two)), Ok(Event::Cancelled)),
        ("BT /F1 12 Tf <0041ff> Tj ET", FontBinding(Some(PdfFixedCodeWidth::Two)), Err("PDF text ends inside its admitted fixed-width codespace")),
        ("BT /F1 12 Tf <0041> Tj ET", FontBinding(None), Ok(Event::Deferred(PageContentBarrier::ResourceContext))),
    ] {
        let graph = page(program);
        let mut source = GraphSource::new(&graph);
        let mut cursor = PageProgramCursor::new(graph[0].id);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut held = 0;
        let mut reached = false;
        for _ in 0..16384 {
            let cancel = outcome == Ok(Event::Cancelled) && matches!(cursor.pending_operator(), Some(PdfOp::ShowText { text: PdfTextString::Codes { .. } }));
            let turn = cursor.advance_with_fonts(&mut source, &binding, grant, cancel);
            assert!(turn.work <= 1 && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
            held += turn.allocated_bytes;
            held -= turn.released_bytes;
            grant = CosGrant { work: 1, ..Default::default() };
            match turn.event {
                Ok(Event::More) => {}
                Ok(Event::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(Event::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                result => { assert_eq!(result, outcome); reached = true; break; }
            }
        }
        assert!(reached && cursor.take_program().is_none());
        cursor.begin_close();
        let mut reached = false;
        for _ in 0..16384 {
            let turn = cursor.close_step(grant);
            assert!(turn.work <= 1 && turn.released_bytes <= grant.release_bytes);
            held -= turn.released_bytes;
            grant.release_bytes = match turn.event {
                Ok(Event::More) => 0,
                Ok(Event::NeedsRelease { bytes }) => bytes,
                Ok(Event::Closed) => { reached = true; break; }
                result => panic!("{result:?}"),
            };
        }
        assert!(reached);
        assert_eq!(held, 0);
    }
}
