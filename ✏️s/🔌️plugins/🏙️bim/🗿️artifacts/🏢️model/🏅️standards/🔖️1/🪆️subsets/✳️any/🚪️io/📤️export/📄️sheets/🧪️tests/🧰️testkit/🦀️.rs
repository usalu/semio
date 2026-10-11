//! 🧰️ Shared helpers of the sheet export tests: English headings, and the committed house with a sheet set placed on it (two plans, a section and two elevations over three sheets) together with the inference of that model.

use super::TitleLabels;
use crate::standards::v1::subsets::any::io::export::svg::testkit::house;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;
use crate::{IsoSize, ModelInference, ModelSnapshot, Orientation, Paper, Point2, Sheet, SheetRevision, Viewport};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

/// 📄️ The committed room with its sheet set: three sheets (A3 landscape, A4 portrait, custom paper), seven cropped viewports and two revision rows.
pub const ROOM: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/📄️sheet-layout/🏠️room/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed sheet exports of the room (one SVG per sheet and the PDF of the set) that the lxml + shapely and pypdf oracles read.
pub const ROOM_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️sheets/🏠️room");

/// 📄️ The decoded room with its sheet set.
pub fn room() -> ModelSnapshot {
    from_json_str(ROOM, JsonMemberPolicy::Reject).expect("the committed room decodes")
}

/// 📖️ A committed file of the sheet export directory of the room.
pub fn read_room(name: &str) -> Vec<u8> {
    std::fs::read(format!("{ROOM_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

/// 🗣️ The English headings.
pub fn labels() -> TitleLabels {
    TitleLabels::english()
}

/// 🏠️ The house with three sheets: `sh-plans` (A3 landscape, the plans of the ground and the first storey), `sh-sections` (A3 portrait, section A) and `sh-elevations` (A2 landscape, the south and east elevations, a custom crop on the east one), with a revision table on the plans.
pub fn house_with_sheets() -> ModelSnapshot {
    let mut model = house();
    let sheet = |number: &str, name: &str, paper: Paper, orientation: Orientation| Sheet { paper, orientation, project: "House on the hill".into(), drawn_by: "UG".into(), checked_by: "AB".into(), date: "2026-10-09".into(), ..Sheet::standard(number, name) };
    model.sheets.insert("sh-plans".into(), sheet("A-101", "Floor plans", Paper::iso(IsoSize::A3), Orientation::Landscape));
    model.sheets.insert("sh-sections".into(), sheet("A-201", "Sections", Paper::iso(IsoSize::A3), Orientation::Portrait));
    model.sheets.insert("sh-elevations".into(), sheet("A-301", "Elevations", Paper::iso(IsoSize::A2), Orientation::Landscape));
    let place = |model: &mut ModelSnapshot, id: &str, sheet: &str, view: &str, x: f64, y: f64, scale: u32| {
        model.viewports.insert(id.into(), Viewport { scale, ..Viewport::standard(sheet, view, Point2 { x, y }) });
    };
    place(&mut model, "vp-ground", "sh-plans", "v-plan-st-ground", 30.0, 30.0, 100);
    place(&mut model, "vp-first", "sh-plans", "v-plan-st-first", 200.0, 30.0, 100);
    place(&mut model, "vp-section", "sh-sections", "v-section-a", 30.0, 30.0, 100);
    place(&mut model, "vp-south", "sh-elevations", "v-elevation-south", 30.0, 30.0, 100);
    place(&mut model, "vp-east", "sh-elevations", "v-elevation-east", 330.0, 30.0, 100);
    model.sheet_revisions.insert("rev-a".into(), SheetRevision { sheet: "sh-plans".into(), number: "A".into(), date: "2026-10-01".into(), description: "Issued for permit".into(), author: "UG".into() });
    model.sheet_revisions.insert("rev-b".into(), SheetRevision { sheet: "sh-plans".into(), number: "B".into(), date: "2026-10-05".into(), description: "Window sizes".into(), author: "AB".into() });
    model
}

/// 🔤️ Every text run of an XML document in order, read by the third-party reader (quick-xml); a well-formedness error fails the test.
pub fn texts(document: &str) -> Vec<String> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(document);
    let mut out = Vec::new();
    loop {
        match reader.read_event().expect("the document is well-formed XML") {
            Event::Text(run) => {
                let run = run.decode().expect("a text run decodes").trim().to_string();
                if !run.is_empty() {
                    out.push(run);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    out
}

/// 🔮️ The inference of `model` through the shared session.
pub fn inferred(model: &ModelSnapshot) -> ModelInference {
    inference::with_inference(None, model, |inference| inference.clone())
}
