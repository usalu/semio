use crate::annotations::{PdfAnnotation, PdfAnnotationKind, PdfBorderStyle, PdfMarkupAnnotation};
use crate::navigation::{PdfAction, PdfActionKind};
use crate::actual_lexer::{dict_get, dict_i64, dict_name, CosGrant};
use crate::object_model::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject};
use crate::page_content::{PageContentBarrier, PageContentEvent, PageContentReceipt, PageProgramCursor};
use crate::standards::v1_7::subsets::base::io::error::PdfEngineError;
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::content_operators::PdfOp;
use crate::document_metadata::PdfDate;
use crate::actual_content::{parse_content, NoFonts};
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources;
use crate::standards::v1_7::subsets::base::schema::graph_source::GraphSource;

#[path = "@PAGE_TREE@"] mod tree;
pub use tree::{PageAttribute, PageTreeCursor, PageTreeEvent, PageTreeLeaf, PageTreeReceipt};

pub const STDIO_PDF17_DOCUMENT_SCHEMA: &str = "stdio.pdf.1.7";
pub type PdfRect = [f64; 4];

#[derive(Debug, PartialEq)]
pub enum PdfColorSpace { DeviceGray, DeviceRgb, DeviceCmyk }

#[derive(Debug, PartialEq)]
pub struct PdfTransparencyGroup { color_space: Option<PdfColorSpace>, isolated: bool, knockout: bool }

#[derive(Debug, Default, PartialEq)]
pub struct PdfPage {
    media_box: PdfRect, crop_box: Option<PdfRect>, bleed_box: Option<PdfRect>,
    trim_box: Option<PdfRect>, art_box: Option<PdfRect>, rotate: i32, user_unit: Option<f64>,
    struct_parents: Option<u32>, duration: Option<f64>, group: Option<PdfTransparencyGroup>,
    content: Vec<PdfOp>, annotations: Vec<PdfAnnotation>,
}

#[derive(Debug, PartialEq)]
pub enum PdfPageMode { UseNone }
#[derive(Debug, PartialEq)]
pub enum PdfPageLayout { SinglePage }
#[derive(Debug, PartialEq)]
pub struct PdfMarkInfo { marked: bool, user_properties: bool, suspects: bool }

fn page_layout_from_name(value: &str) -> Option<PdfPageLayout> { (value == "SinglePage").then_some(PdfPageLayout::SinglePage) }
fn page_mode_from_name(value: &str) -> Option<PdfPageMode> { (value == "UseNone").then_some(PdfPageMode::UseNone) }

#[derive(Debug, Default, PartialEq)]
pub struct PdfInfo {
    title: Option<String>, author: Option<String>, subject: Option<String>, keywords: Option<String>,
    creator: Option<String>, producer: Option<String>, trapped: Option<String>,
    creation_date: Option<PdfDate>, modification_date: Option<PdfDate>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PdfGraphPath { Entry { key: String } }
#[derive(Debug, Clone, PartialEq)]
pub struct PdfGraphIdentity { owner: ObjRef, path: Vec<PdfGraphPath> }
#[derive(Debug, PartialEq)]
pub enum PdfStreamRoleValue { Operators { content: Vec<PdfOp> } }
#[derive(Debug, PartialEq)]
pub struct PdfAdmittedStreamRole { identity: PdfGraphIdentity, dependencies: Vec<PdfGraphIdentity>, value: PdfStreamRoleValue }

#[derive(Debug, PartialEq)]
pub struct PdfSnapshot {
    schema: String, declared_version: String, pages: Vec<PdfPage>, info: PdfInfo,
    document_id: Option<[Vec<u8>; 2]>, admitted_stream_roles: Vec<PdfAdmittedStreamRole>, objects: Vec<()>,
    fonts: Vec<crate::font_model::PdfFont>,
    page_layout: Option<PdfPageLayout>, page_mode: Option<PdfPageMode>, language: Option<String>, mark_info: Option<PdfMarkInfo>,
}

impl Default for PdfSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(), declared_version: "1.7".into(),
            pages: Vec::new(), info: PdfInfo::default(), document_id: None, admitted_stream_roles: Vec::new(),
            objects: Vec::new(), fonts: Vec::new(), page_layout: None, page_mode: None, language: None, mark_info: None,
        }
    }
}

#[derive(Default)]
pub struct Artifacts;
impl Artifacts { fn into_objects(self) -> Vec<()> { Vec::new() } }

#[derive(Default)]
pub struct DocumentLiftState {
    snapshot: PdfSnapshot,
    admission_error: Option<PdfEngineError>,
    page_refs: Vec<ObjRef>,
    admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
    scope: usize,
    artifacts: Artifacts,
}

pub struct Lifter<'a> {
    source: &'a mut dyn ObjectSource,
    snapshot: PdfSnapshot,
    admission_error: Option<PdfEngineError>,
    page_refs: Vec<ObjRef>,
    admitted_stream_roles: Vec<PdfAdmittedStreamRole>,
    scope: usize,
    artifacts: Artifacts,
}

impl<'a> Lifter<'a> {
    fn bind(source: &'a mut dyn ObjectSource, state: DocumentLiftState) -> Self {
        let DocumentLiftState { snapshot, admission_error, page_refs, admitted_stream_roles, scope, artifacts } = state;
        Self { source, snapshot, admission_error, page_refs, admitted_stream_roles, scope, artifacts }
    }

    fn unbind(self) -> DocumentLiftState {
        let Self { source: _, snapshot, admission_error, page_refs, admitted_stream_roles, scope, artifacts } = self;
        DocumentLiftState { snapshot, admission_error, page_refs, admitted_stream_roles, scope, artifacts }
    }
    fn lift_info(&self, _: &[PdfDictEntry]) -> PdfInfo { PdfInfo::default() }
    fn lift_optional_content(&self, _: &[PdfDictEntry]) {}
    fn lift_embedded_files(&self, _: &[PdfDictEntry]) {}
    fn attribute_value(&self, _: PageAttribute) -> Option<PdfObject> { None }
    fn lift_resources(&self, _: &PdfObject) {}
    fn lift_page(&self, _: u32, _: PageTreeLeaf, prepared: Option<Vec<PdfOp>>) -> PdfPage { PdfPage { content: prepared.expect("prepared fixture page"), ..PdfPage::default() } }
    fn lift_catalog(&self, _: &[PdfDictEntry]) {}
}

#[path = "@DOCUMENT@"] mod document;
use document::{DocumentLiftAction, DocumentLiftCursor, DocumentLiftGrant, DocumentLiftStatus};

struct Graph(Vec<PdfObject>);

impl ObjectSource for Graph {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> { self.0.get(reference.num as usize).cloned() }
    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) { read(self.0.get(reference.num as usize)); }
}

#[test]
fn exact_document_stage_publishes_only_after_controlled_program_and_cold_grants() {
    let reference = |num| ObjRef { num, gen: 0 };
    let mut graph = Graph(vec![
        PdfObject::Dict(vec![PdfDictEntry::new("Pages", PdfObject::Ref(reference(1)))]),
        PdfObject::Dict(vec![PdfDictEntry::new("Contents", PdfObject::Ref(reference(2)))]),
        PdfObject::Stream { dict: Vec::new(), data: b"20 10 m 40 10 l S\n".to_vec(), filters: Vec::new() },
    ]);
    let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(reference(0)))];
    let mut cursor = DocumentLiftCursor::new();
    let grant = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };
    let mut prepared = false;
    for _ in 0..1024 {
        let turn = cursor.advance(&trailer, "1.7", &mut graph, DocumentLiftGrant { traversal: grant, semantic: false }, false);
        if let Some(content) = turn.content {
            if content.event == Ok(PageContentEvent::Complete) { prepared = true; }
            assert!(!matches!(content.event, Ok(PageContentEvent::NeedsAllocation { .. } | PageContentEvent::NeedsRelease { .. })), "{content:?}");
        }
        if let DocumentLiftStatus::NeedsSemantic(action) = turn.status {
            if matches!(action, DocumentLiftAction::ProjectPage { .. }) { assert!(prepared); }
            let turn = cursor.advance(&trailer, "1.7", &mut graph, DocumentLiftGrant { traversal: CosGrant::default(), semantic: true }, false);
            if turn.status == DocumentLiftStatus::Complete {
                let output = cursor.take_result().unwrap();
                assert!(matches!(output.snapshot.pages[0].content[0], PdfOp::MoveTo { x: 20., y: 10. }));
                return;
            }
        }
    }
    panic!("document did not finish");
}


fn lift_document(_: &[PdfDictEntry], _: &str, graph: &mut GraphSource<'_>) -> Result<PdfSnapshot, ()> {
    let stream = |number| match graph.objects[(number - 1) as usize].value.clone() {
        PdfObject::Stream { data, .. } => String::from_utf8(data).unwrap(),
        _ => unreachable!(),
    };
    let content = parse_content(format!("{}\n{}", stream(5), stream(6)).as_bytes(), &NoFonts, &mut NativePdfArtifactResources::from_objects(&[])).unwrap();
    let identity = PdfGraphIdentity { owner: ObjRef { num: 3, gen: 0 }, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] };
    let mut dependencies = vec![identity.clone()];
    dependencies.extend((4..=6).map(|num| PdfGraphIdentity { owner: ObjRef { num, gen: 0 }, path: Vec::new() }));
    Ok(PdfSnapshot {
        pages: vec![PdfPage { media_box: [0., 0., 100., 90.], crop_box: Some([5., 2., 98., 89.]), rotate: 90, content: content.clone(), ..PdfPage::default() }],
        document_id: Some([vec![1, 2], vec![3, 4]]), language: Some("en".into()),
        page_layout: Some(PdfPageLayout::SinglePage), page_mode: Some(PdfPageMode::UseNone),
        info: PdfInfo { title: Some("Controlled vector".into()), ..PdfInfo::default() },
        admitted_stream_roles: vec![PdfAdmittedStreamRole { identity, dependencies, value: PdfStreamRoleValue::Operators { content } }],
        ..PdfSnapshot::default()
    })
}


#[test]
fn genuine_classic_xref_vector_pages_reach_the_retained_semantic_lifter() {
    let Some(path) = std::env::var_os("PDF_CONTROLLED_LIFT_FIXTURE") else { return };
    let bytes = std::fs::read(path).unwrap();
    let footer = bytes.windows(b"startxref".len()).rposition(|part| part == b"startxref").unwrap();
    let xref = std::str::from_utf8(&bytes[footer..]).unwrap().lines().nth(1).unwrap().parse::<usize>().unwrap();
    let section = std::str::from_utf8(&bytes[xref..]).unwrap();
    let mut lines = section.lines();
    assert_eq!(lines.next(), Some("xref"));
    let count = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
    let mut graph = Vec::new();
    for line in lines.take(count) {
        if !line.contains(" n ") { continue; }
        let offset = line[..10].parse::<usize>().unwrap();
        let (id, mut value) = crate::actual_lexer::parse_indirect_at(&bytes, offset).unwrap();
        if let PdfObject::Stream { dict, .. } = &mut value { dict.retain(|entry| crate::actual_lexer::retain_stream_entry(entry, true)); }
        graph.push(PdfIndirectObject { id, value });
    }
    let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 }))];
    let mut source = GraphSource::new(&graph);
    let mut cursor = DocumentLiftCursor::controlled();
    let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
    let mut work = 0;
    let mut retained = 0;
    for _ in 0..1_000_000 {
        let turn = cursor.advance(&trailer, "1.7", &mut source, grant, false);
        assert!(turn.work <= grant.work && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
        work += turn.work;
        retained += turn.allocated_bytes;
        retained -= turn.released_bytes;
        match turn.event.unwrap() {
            document::ControlledLiftEvent::Complete => {
                let snapshot = cursor.take_result().unwrap();
                assert_eq!(snapshot.pages.len(), 48);
                assert_eq!(snapshot.admitted_stream_roles.len(), 48);
                for (index, (page, role)) in snapshot.pages.iter().zip(&snapshot.admitted_stream_roles).enumerate() {
                    let PdfStreamRoleValue::Operators { content } = &role.value;
                    let PdfObject::Stream { data, .. } = &graph[3 + index * 2].value else { unreachable!() };
                    let mut resources = NativePdfArtifactResources::from_objects(&[]);
                    assert_eq!(page.content, parse_content(data, &NoFonts, &mut resources).unwrap());
                    assert_eq!(&page.content, content);
                    assert_eq!(page.media_box, [0., 0., 360., 480.]);
                    assert_eq!(role.dependencies.len(), 2);
                }
                println!("{} native bytes, {} pages, {work} one-unit turns, {retained} typed bytes, inline cursor {} bytes",
                    bytes.len(), snapshot.pages.len(), std::mem::size_of::<document::ControlledDocumentLiftCursor>());
                return;
            }
            document::ControlledLiftEvent::More => {}
            event => panic!("unexpected native vector semantic stage {event:?}"),
        }
    }
    panic!("large vector document did not complete");
}


#[test]
fn genuine_classic_xref_text_pages_preserve_standard_font_operators_and_roles() {
    use crate::actual_content::extract_text;
    use crate::actual_fonts::FontCodec;
    use crate::font_model::{PdfBaseEncoding, PdfFontKind};
    use std::collections::HashMap;

    let Some(path) = std::env::var_os("PDF_CONTROLLED_TEXT_FIXTURE") else { return };
    let decoded = std::env::var_os("PDF_CONTROLLED_TEXT_STREAMS").expect("MuPDF decoded streams");
    let bytes = std::fs::read(path).unwrap();
    let footer = bytes.windows(b"startxref".len()).rposition(|part| part == b"startxref").unwrap();
    let xref = std::str::from_utf8(&bytes[footer..]).unwrap().lines().nth(1).unwrap().parse::<usize>().unwrap();
    let mut lines = std::str::from_utf8(&bytes[xref..]).unwrap().lines();
    assert_eq!(lines.next(), Some("xref"));
    let count = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
    let mut graph = Vec::new();
    for line in lines.take(count) {
        if !line.contains(" n ") { continue; }
        let (id, mut value) = crate::actual_lexer::parse_indirect_at(&bytes, line[..10].parse().unwrap()).unwrap();
        if let PdfObject::Stream { dict, data, .. } = &mut value {
            if crate::actual_lexer::dict_get(dict, "Filter").is_some() {
                *data = std::fs::read(std::path::Path::new(&decoded).join(id.num.to_string())).unwrap();
            }
            dict.retain(|entry| crate::actual_lexer::retain_stream_entry(entry, true));
        }
        graph.push(PdfIndirectObject { id, value });
    }
    let reference = |num| ObjRef { num, gen: 0 };
    let trailer = [PdfDictEntry::new("Root", PdfObject::Ref(reference(1)))];
    let mut source = GraphSource::new(&graph);
    let mut cursor = DocumentLiftCursor::controlled();
    let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
    let mut work = 0;
    let mut retained = 0;
    let snapshot = loop {
        let turn = cursor.advance(&trailer, "1.7", &mut source, grant, false);
        assert!(turn.work <= grant.work && turn.allocated_bytes <= grant.allocation_bytes && turn.released_bytes <= grant.release_bytes);
        work += turn.work;
        retained += turn.allocated_bytes;
        retained -= turn.released_bytes;
        match turn.event.unwrap() {
            document::ControlledLiftEvent::Complete => break cursor.take_result().unwrap(),
            document::ControlledLiftEvent::More => assert!(work < 1_000_000),
            event => panic!("unexpected native Type1 semantic stage {event:?}"),
        }
    };
    assert_eq!(snapshot.pages.len(), 48);
    assert_eq!(snapshot.admitted_stream_roles.len(), 48);
    assert_eq!(snapshot.fonts.len(), 1);
    let font = &snapshot.fonts[0];
    assert_eq!(font.id, "helv");
    assert!(matches!(&font.kind, PdfFontKind::Type1 { base_font, encoding, .. }
        if base_font == "Helvetica" && encoding.base == Some(PdfBaseEncoding::WinAnsi)));
    let mut resources = NativePdfArtifactResources::from_objects(&[]);
    let fonts = HashMap::from([(font.id.clone(), FontCodec::new(font, &resources).unwrap())]);
    for (index, (page, role)) in snapshot.pages.iter().zip(&snapshot.admitted_stream_roles).enumerate() {
        let PdfStreamRoleValue::Operators { content } = &role.value;
        let owner = role.identity.owner;
        let authored_page = &graph.iter().find(|entry| entry.id == owner).unwrap().value;
        let authored = authored_page.dict_get("Contents").unwrap();
        let resource_owner = authored_page.dict_get("Resources").and_then(PdfObject::as_ref).unwrap();
        let resource = &graph.iter().find(|entry| entry.id == resource_owner).unwrap().value;
        let font_owner = resource.dict_get("Font").and_then(|value| value.dict_get("helv")).and_then(PdfObject::as_ref).unwrap();
        let streams = authored.as_array().unwrap().iter().map(|value| value.as_ref().unwrap()).collect::<Vec<_>>();
        let joined = streams.iter().map(|owner| {
            let PdfObject::Stream { data, .. } = &graph.iter().find(|record| record.id == *owner).unwrap().value else { unreachable!() };
            data.clone()
        }).collect::<Vec<_>>().join(&b'\n');
        assert_eq!(page.content, parse_content(&joined, &fonts, &mut resources).unwrap());
        assert_eq!(&page.content, content);
        assert_eq!(extract_text(content, &fonts).trim(), format!("Page {:02} of 48", index + 1));
        assert_eq!(page.media_box, [0., 0., 360., 480.]);
        let mut dependencies = vec![role.identity.clone(), PdfGraphIdentity {
            owner: resource_owner, path: vec![PdfGraphPath::Entry { key: "Font".into() }],
        }];
        dependencies.extend(streams.into_iter().chain([font_owner]).map(|owner| PdfGraphIdentity { owner, path: Vec::new() }));
        assert_eq!(role.dependencies, dependencies);
    }
    println!("{} native bytes, {} Type1 pages, {work} one-unit turns, {retained} typed bytes", bytes.len(), snapshot.pages.len());
}

#[test]
fn genuine_pdf_projects_indirect_notes_and_uri_link_under_metered_turns() {
    use crate::actual_lexer::parse_indirect_at;
    use crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text::admit_retained_text;
    use document::{ControlledLiftBarrier, ControlledLiftEvent as Event};
    let Some(path) = std::env::var_os("PDF_CONTROLLED_ANNOTATION_FIXTURE") else { return; };
    let bytes = std::fs::read(path).unwrap();
    let footer = bytes.windows(b"startxref".len()).rposition(|part| part == b"startxref").unwrap();
    let xref = std::str::from_utf8(&bytes[footer..]).unwrap().lines().nth(1).unwrap().parse::<usize>().unwrap();
    let mut lines = std::str::from_utf8(&bytes[xref..]).unwrap().lines();
    assert_eq!(lines.next(), Some("xref"));
    let count = lines.next().unwrap().split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
    let mut objects = Vec::new();
    for line in lines.take(count) {
        if !line.ends_with(" n ") { continue; }
        let (id, value) = parse_indirect_at(&bytes, line[..10].parse().unwrap()).unwrap();
        objects.push(PdfIndirectObject { id, value });
    }
    assert_eq!(objects[4].id, ObjRef { num: 5, gen: 2 });
    let mut trailer = [PdfDictEntry::new("Root", PdfObject::Ref(objects[0].id))];
    let position = |id: ObjRef| (1..=8).contains(&id.num).then(|| id.num as usize - 1);
    admit_retained_text(&mut objects, &mut trailer, position).unwrap();
    for mode in ["complete", "cancel", "appearance"] {
        let mut graph = objects.clone();
        if mode == "appearance" {
            let PdfObject::Dict(dict) = &mut graph[5].value else { unreachable!() };
            dict.push(PdfDictEntry::new("AP", PdfObject::Dict(Vec::new())));
        }
        let mut source = GraphSource::new(&graph);
        let mut cursor = DocumentLiftCursor::controlled();
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut note_published = false;
        let mut held = 0;
        let mut completed = false;
        for _ in 0..1024 {
            let inline_body = cursor.next_annotation_body_bytes();
            let step = cursor.advance(&trailer, "1.7", &mut source, grant, mode == "cancel" && note_published);
            assert!(step.work <= grant.work && step.allocated_bytes <= grant.allocation_bytes && step.released_bytes <= grant.release_bytes);
            held += step.allocated_bytes;
            held -= step.released_bytes;
            match step.event.unwrap() {
                Event::NeedsAllocation { bytes } => { assert_eq!(step.work, 0); grant.allocation_bytes = bytes; }
                Event::NeedsRelease { bytes } => grant.release_bytes = bytes,
                Event::More => {
                    if step.allocated_bytes == 62 {
                        assert_eq!(inline_body, std::mem::size_of::<PdfAnnotation>());
                        note_published = true;
                    }
                    grant = CosGrant { work: 1, ..Default::default() };
                }
                Event::Complete => {
                    assert_eq!(mode, "complete");
                    let snapshot = cursor.take_result().unwrap();
                    assert!(snapshot.admitted_stream_roles.is_empty());
                    assert_eq!(snapshot.pages.iter().map(|page| page.annotations.len()).collect::<Vec<_>>(), [2, 1]);
                    let [note, link] = snapshot.pages[0].annotations.as_slice() else { unreachable!() };
                    assert_eq!(note.rect, [10., 10., 30., 30.]);
                    assert_eq!(note.contents.as_deref(), Some("Review note"));
                    assert_eq!(note.name.as_deref(), Some("review-1"));
                    assert_eq!(note.flags, 4);
                    assert_eq!(note.color, [1., 0.5, 0.]);
                    assert_eq!(note.markup.as_ref().unwrap().title.as_deref(), Some("Reviewer"));
                    assert_eq!(note.markup.as_ref().unwrap().subject.as_deref(), Some("Plan"));
                    assert!(matches!(&note.kind, PdfAnnotationKind::Text { open: true, icon: Some(icon), .. } if icon == "Comment"));
                    assert!(matches!(&link.kind, PdfAnnotationKind::Link { highlight: Some(highlight), action: Some(PdfAction { kind: PdfActionKind::Uri { uri, is_map: false }, .. }), .. } if highlight == "I" && uri == "https://example.com/plan"));
                    assert_eq!((link.rect, link.border.as_ref().unwrap().width), ([40., 10., 85., 25.], 0.));
                    let later = &snapshot.pages[1].annotations[0];
                    assert_eq!((later.rect, later.contents.as_deref()), ([15., 45., 35., 65.], Some("Second page note")));
                    assert_eq!(held, STDIO_PDF17_DOCUMENT_SCHEMA.len() + 3
                        + 2 * (std::mem::size_of::<PdfPage>() + std::mem::size_of::<PdfAdmittedStreamRole>())
                        + 3 * std::mem::size_of::<PdfAnnotation>() + 62 + 25 + 16);
                    completed = true;
                    break;
                }
                event @ (Event::Cancelled | Event::Deferred(ControlledLiftBarrier::Page)) => {
                    assert_eq!(event, if mode == "cancel" { Event::Cancelled } else { Event::Deferred(ControlledLiftBarrier::Page) });
                    assert!(cursor.take_result().is_none());
                    cursor.begin_close();
                    while cursor.close_step(CosGrant { release_bytes: usize::MAX, ..grant }).event != Ok(Event::Closed) {}
                    let partial = cursor.take_partial();
                    assert!(partial.snapshot.unwrap().pages.is_empty());
                    let page = partial.page.unwrap();
                    assert_eq!(page.annotations.len(), 1);
                    assert_eq!(page.annotations[0].contents.as_deref(), Some("Review note"));
                    completed = true;
                    break;
                }
                event => panic!("unexpected annotation state {event:?}"),
            }
        }
        assert!(completed);
    }
}
