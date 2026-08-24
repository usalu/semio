use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioPoint2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortRef {
    pub node: String,
    pub port: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowParam {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    #[serde(default)]
    pub params: Vec<FlowParam>,
    pub position: SemioPoint2,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEdge {
    pub id: String,
    pub from: PortRef,
    pub to: PortRef,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioFlowSnapshot {
    pub schema: String,
    #[serde(default)]
    pub nodes: Vec<FlowNode>,
    #[serde(default)]
    pub edges: Vec<FlowEdge>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioFlowMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioFlowSnapshot,
    },
    InsertNode {
        node: FlowNode,
    },
    RemoveNode {
        id: String,
    },
    SetNodeKind {
        id: String,
        kind: String,
    },
    SetNodeLabel {
        id: String,
        label: String,
    },
    SetNodePosition {
        id: String,
        position: SemioPoint2,
    },
    SetNodeParam {
        id: String,
        key: String,
        value: String,
    },
    RemoveNodeParam {
        id: String,
        key: String,
    },
    InsertEdge {
        edge: FlowEdge,
    },
    RemoveEdge {
        id: String,
    },
    SetEdgeEndpoints {
        id: String,
        from: PortRef,
        to: PortRef,
    },
    SetEdgeKind {
        id: String,
        kind: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CadEntity {
    Line { a: SemioPoint2, b: SemioPoint2 },
    Arc { center: SemioPoint2, radius: f64, start_angle: f64, end_angle: f64 },
    Circle { center: SemioPoint2, radius: f64 },
    Ellipse { center: SemioPoint2, major_axis_end: SemioPoint2, ratio: f64, start_param: f64, end_param: f64 },
    Polyline { vertices: Vec<SemioPoint2>, closed: bool },
    Text { position: SemioPoint2, height: f64, rotation: f64, content: String },
    Insert { block_name: String, insertion_point: SemioPoint2, scale: SemioPoint2, rotation: f64 },
    Solid { p1: SemioPoint2, p2: SemioPoint2, p3: SemioPoint2, p4: SemioPoint2 },
    Dimension { def_point: SemioPoint2, text_position: SemioPoint2, measurement: f64, text: String },
}

impl Default for CadEntity {
    fn default() -> Self { CadEntity::Line { a: SemioPoint2::default(), b: SemioPoint2::default() } }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadLayer {
    pub name: String,
    pub color_index: i32,
    pub line_type: String,
    pub visible: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadEntityRecord {
    pub handle: String,
    pub layer: String,
    pub entity: CadEntity,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadBlock {
    pub name: String,
    pub base_point: SemioPoint2,
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioCadSnapshot {
    pub schema: String,
    #[serde(default)]
    pub layers: Vec<CadLayer>,
    #[serde(default)]
    pub blocks: Vec<CadBlock>,
    #[serde(default)]
    pub entities: Vec<CadEntityRecord>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioCadMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioCadSnapshot,
    },
    AddLayer {
        layer: CadLayer,
    },
    RemoveLayer {
        name: String,
    },
    SetLayer {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color_index: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        line_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        visible: Option<bool>,
    },
    AddBlock {
        block: CadBlock,
    },
    RemoveBlock {
        name: String,
    },
    SetBlockBasePoint {
        name: String,
        base_point: SemioPoint2,
    },
    AddEntity {
        entity: CadEntityRecord,
    },
    RemoveEntity {
        handle: String,
    },
    SetEntityLayer {
        handle: String,
        layer: String,
    },
    SetEntityGeometry {
        handle: String,
        entity: CadEntity,
    },
    AddBlockEntity {
        block_name: String,
        entity: CadEntityRecord,
    },
    RemoveBlockEntity {
        block_name: String,
        handle: String,
    },
    SetBlockEntityLayer {
        block_name: String,
        handle: String,
        layer: String,
    },
    SetBlockEntityGeometry {
        block_name: String,
        handle: String,
        entity: CadEntity,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStyle {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub font: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocRun {
    pub text: String,
    #[serde(default)]
    pub style: RunStyle,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocStyle {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub based_on: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocImage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocListItem {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableCell {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableRow {
    #[serde(default)]
    pub cells: Vec<DocTableCell>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocBlock {
    Paragraph {
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    Heading {
        level: u8,
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    List {
        #[serde(default)]
        ordered: bool,
        #[serde(default)]
        items: Vec<DocListItem>,
    },
    Table {
        #[serde(default)]
        rows: Vec<DocTableRow>,
    },
    Code {
        #[serde(default)]
        language: Option<String>,
        #[serde(default)]
        text: String,
    },
    Quote {
        #[serde(default)]
        blocks: Vec<DocBlock>,
    },
    Image {
        image_id: String,
        #[serde(default)]
        alt: String,
        #[serde(default)]
        width: Option<f64>,
        #[serde(default)]
        height: Option<f64>,
    },
    #[default]
    PageBreak,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioDocumentSnapshot {
    pub schema: String,
    #[serde(default)]
    pub styles: Vec<DocStyle>,
    #[serde(default)]
    pub images: Vec<DocImage>,
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocPathSegment {
    Quote { block_index: usize },
    ListItem { block_index: usize, item: usize },
    TableCell { block_index: usize, row: usize, cell: usize },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocBlockPath {
    #[serde(default)]
    pub segments: Vec<DocPathSegment>,
    pub index: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioDocumentMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SemioDocumentSnapshot,
    },
    InsertBlock {
        path: DocBlockPath,
        block: DocBlock,
    },
    RemoveBlock {
        path: DocBlockPath,
    },
    SetBlockContent {
        path: DocBlockPath,
        block: DocBlock,
    },
    SetParagraphStyle {
        path: DocBlockPath,
        style_id: Option<String>,
    },
    SetHeadingLevel {
        path: DocBlockPath,
        level: u8,
    },
    SetListOrdered {
        path: DocBlockPath,
        ordered: bool,
    },
    SetRunText {
        path: DocBlockPath,
        run_index: usize,
        text: String,
    },
    SetRunStyle {
        path: DocBlockPath,
        run_index: usize,
        style: RunStyle,
    },
    SetImageBlock {
        path: DocBlockPath,
        image_id: String,
        alt: String,
        width: Option<f64>,
        height: Option<f64>,
    },
    InsertStyle {
        style: DocStyle,
    },
    RemoveStyle {
        id: String,
    },
    SetStyleName {
        id: String,
        name: String,
    },
    SetStyleBasedOn {
        id: String,
        based_on: Option<String>,
    },
    InsertImage {
        image: DocImage,
    },
    RemoveImage {
        id: String,
    },
    SetImageBytes {
        id: String,
        mime: String,
        bytes: Vec<u8>,
    },
}


fn check<T: for<'a> Deserialize<'a> + Serialize + PartialEq + std::fmt::Debug>(label: &str, text: &str) -> bool {
    match serde_json::from_str::<T>(text) {
        Err(error) => { println!("  x {label}: {error}"); false }
        Ok(value) => {
            let round = serde_json::to_string(&value).expect("serialize");
            let reparsed: T = serde_json::from_str(&round).expect("reparse");
            if reparsed != value { println!("  x {label}: serde round trip changed the value"); return false; }
            let a: serde_json::Value = serde_json::from_str(text).expect("json");
            let b: serde_json::Value = serde_json::from_str(&round).expect("json");
            if a != b { println!("  x {label}: re-serialization differs from the committed JSON\n      committed {a}\n      produced  {b}"); return false; }
            true
        }
    }
}

fn run<S, M>(case: &str, kinds: &[&str]) -> usize
where
    S: for<'a> Deserialize<'a> + Serialize + PartialEq + std::fmt::Debug,
    M: for<'a> Deserialize<'a> + Serialize + PartialEq + std::fmt::Debug,
{
    let mut bad = 0usize;
    for kind in kinds {
        let path = format!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/{case}/🧫️fixtures/🦠️{kind}.json");
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let vector: serde_json::Value = serde_json::from_str(&text).expect("vector json");
        if !check::<S>(&format!("{case}/{kind}/before"), &vector["before"].to_string()) { bad += 1; }
        if !check::<S>(&format!("{case}/{kind}/after"), &vector["after"].to_string()) { bad += 1; }
        if !check::<M>(&format!("{case}/{kind}/mutation"), &vector["mutation"].to_string()) { bad += 1; }
    }
    println!("{case}: {} kinds checked, {bad} problem(s)", kinds.len());
    bad
}

fn main() {
    let flow = ["no-mutation","set-snapshot","insert-node","remove-node","set-node-kind","set-node-label","set-node-position","set-node-param","remove-node-param","insert-edge","remove-edge","set-edge-endpoints","set-edge-kind"];
    let cad = ["no-mutation","set-snapshot","add-layer","remove-layer","set-layer","add-block","remove-block","set-block-base-point","add-entity","remove-entity","set-entity-layer","set-entity-geometry","add-block-entity","remove-block-entity","set-block-entity-layer","set-block-entity-geometry"];
    let doc = ["no-mutation","set-snapshot","insert-block","remove-block","set-block-content","set-paragraph-style","set-heading-level","set-list-ordered","set-run-text","set-run-style","set-image-block","insert-style","remove-style","set-style-name","set-style-based-on","insert-image","remove-image","set-image-bytes"];
    let mut bad = 0;
    bad += run::<SemioFlowSnapshot, SemioFlowMutation>("mutate-semio-flow", &flow);
    bad += run::<SemioCadSnapshot, SemioCadMutation>("mutate-semio-cad", &cad);
    bad += run::<SemioDocumentSnapshot, SemioDocumentMutation>("mutate-semio-document", &doc);
    println!("TOTAL PROBLEMS: {bad}");
    std::process::exit(if bad == 0 { 0 } else { 1 });
}
