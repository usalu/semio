"""📐 Checks the native page boundary parser, inspector values and viewport against the shared fixture."""
import json
from pathlib import Path
import subprocess
import sys

root, fixture, generated = map(Path, sys.argv[1:])
document = (root / "✏️editor/📄️document/🦀️.rs").read_text()
inspector = (root / "✏️editor/🖼️page/🔎️inspector/🦀️.rs").read_text()
coordinates = root / "✏️editor/🖼️page/🧭️geometry/🧭️coordinates/🦀️.rs"
parser = document[document.index("fn page_box_kind("):document.index("pub fn page_layout(")]
fields = inspector[inspector.index("pub(super) fn page_boundary_fields("):inspector.index("fn resource_number(")].replace("pub(super) ", "")
case = json.loads(fixture.read_text())["pageBoundaries"]


def rect(values):
    return "[" + ", ".join(repr(float(value)) for value in values) + "]"


checks = ""
for edit in case["edits"]:
    kind = edit["object"].removeprefix("box.")
    checks += f'''
    let edit = set_page_boundary(&snapshot, 0, {json.dumps(kind)}, {json.dumps(edit["coordinates"])}).unwrap();
    snapshot.apply(edit.into_iter().next().unwrap());
    assert_eq!(page_boundary_fields(&snapshot.pages[0]).find(|field| field.object_key.as_deref() == Some({json.dumps(edit["object"])})).unwrap().value, {json.dumps(edit["coordinates"])});
    assert_eq!(snapshot.boundary({json.dumps(kind)}).unwrap(), {rect(edit["value"])});
    '''
clear = case["clear"]["object"].removeprefix("box.")
checks += f'''
    let scale = set_page_user_unit(&snapshot, 0, {case["userUnit"]}).unwrap();
    snapshot.apply(scale.into_iter().next().unwrap());
    assert_eq!(snapshot.view_size(), {rect(case["viewSize"])});
    assert!(set_page_boundary(&snapshot, 0, "media", "").is_err());
    assert!(set_page_boundary(&snapshot, 0, "crop", "0 0 10 NaN").is_err());
    assert!(set_page_boundary(&snapshot, 0, "crop", "0 0 10 10 3").is_err());
    assert!(set_page_boundary(&snapshot, 0, "crop", "10 0 0 10").is_err());
    let clear = set_page_boundary(&snapshot, 0, {json.dumps(clear)}, "").unwrap();
    snapshot.apply(clear.into_iter().next().unwrap());
    assert_eq!(snapshot.boundary({json.dumps(clear)}), None);
    assert_eq!(snapshot.view_size(), {rect(case["clearedViewSize"])});
'''
preamble = '''
#![allow(dead_code)]
#[path = %s]
mod coordinates;
use coordinates::PageViewport;
type Fault = String;
type PdfRect = [f64; 4];

fn fault(message: impl Into<String>) -> Fault { message.into() }
fn require_page(snapshot: &PdfSnapshot, page: usize) -> Result<(), Fault> {
    if snapshot.pages.get(page).is_some() { Ok(()) } else { Err(fault("missing page")) }
}

struct PdfSnapshot { pages: Vec<PdfPage> }
#[derive(Default)]
struct PdfPage {
    media_box: PdfRect, crop_box: Option<PdfRect>, bleed_box: Option<PdfRect>,
    trim_box: Option<PdfRect>, art_box: Option<PdfRect>, user_unit: Option<f64>,
}
enum PdfPageBox { Crop, Bleed, Trim, Art }
struct SetPageBox { index: usize, kind: PdfPageBox, rect: Option<PdfRect> }
struct SetPageMediaBox { index: usize, media_box: PdfRect }
struct SetPageUserUnit { index: usize, user_unit: Option<f64> }
enum PdfMutation { SetPageBox(SetPageBox), SetPageMediaBox(SetPageMediaBox), SetPageUserUnit(SetPageUserUnit) }
mod ui { pub enum InputKind { Text } }
struct InspectorField {
    id: &'static str, label_en: &'static str, label_de: &'static str, action: &'static str, key: &'static str,
    kind: ui::InputKind, value: String, numbers: Vec<(&'static str, f64)>, text: Option<String>, object_key: Option<String>,
}
impl PdfSnapshot {
    fn apply(&mut self, mutation: PdfMutation) {
        match mutation {
            PdfMutation::SetPageMediaBox(set) => self.pages[set.index].media_box = set.media_box,
            PdfMutation::SetPageUserUnit(set) => self.pages[set.index].user_unit = set.user_unit,
            PdfMutation::SetPageBox(set) => {
                let page = &mut self.pages[set.index];
                match set.kind {
                    PdfPageBox::Crop => page.crop_box = set.rect, PdfPageBox::Bleed => page.bleed_box = set.rect,
                    PdfPageBox::Trim => page.trim_box = set.rect, PdfPageBox::Art => page.art_box = set.rect,
                }
            }
        }
    }
    fn boundary(&self, kind: &str) -> Option<PdfRect> {
        let page = &self.pages[0];
        match kind {
            "media" => Some(page.media_box), "crop" => page.crop_box, "bleed" => page.bleed_box,
            "trim" => page.trim_box, "art" => page.art_box, _ => None,
        }
    }
    fn view_size(&self) -> [f64; 2] {
        let page = &self.pages[0];
        let [_, _, width, height] = PageViewport::new(page.media_box, page.crop_box, 0, page.user_unit, 0.0).clip_rect();
        [width, height]
    }
}
''' % json.dumps(str(coordinates.resolve()), ensure_ascii=False)
width, height = case["initialSize"]
main = f'''
fn main() {{
    let mut snapshot = PdfSnapshot {{ pages: vec![PdfPage {{ media_box: [0.0, 0.0, {float(width)}, {float(height)}], ..Default::default() }}] }};
    {checks}
    println!("Native source verified five edit controls, invalid boxes, scaling and crop removal.");
}}
'''
source = generated / "page-boundaries-native.rs"
binary = generated / ("page-boundaries-native.exe" if sys.platform == "win32" else "page-boundaries-native")
source.write_text(preamble + parser + fields + main)
subprocess.run(["rustc", "--edition", "2024", str(source), "-o", str(binary)], check=True)
subprocess.run([str(binary)], check=True)
