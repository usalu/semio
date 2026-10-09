//! 🧰️ Shared helpers of the SVG export tests: the committed house model, its fixture directory and a third-party XML reader (quick-xml) over exported text.

use crate::ModelSnapshot;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

/// 🏠️ The committed export fixture: one site, one building, four storeys and every element family.
pub const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed SVG export of the house and the table the lxml + shapely oracle measured from it.
pub const HOUSE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️svg/🏠️house");

/// 🪧️ The committed annotated room of the annotation inference: dimensions, tags, a note and a leader around four walls, a window, a grid line and a column.
pub const NOTATED: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🪧️annotation-layout/🏠️room/📸️snapshot/🔣️.json");

/// 📁️ The directory of the committed SVG export of the annotated room and the table the lxml + shapely oracle measured from it.
pub const NOTATED_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🚪️svg/🪧️notated");

/// 🪧️ The decoded annotated room.
pub fn notated() -> ModelSnapshot {
    from_json_str(NOTATED, JsonMemberPolicy::Reject).expect("the committed room decodes")
}

/// 🏠️ The decoded house model.
pub fn house() -> ModelSnapshot {
    from_json_str(HOUSE, JsonMemberPolicy::Reject).expect("the committed house decodes")
}

/// 🎨️ The SVG text of `model`; a refusal of the writer fails the test.
pub fn svg(model: &ModelSnapshot) -> String {
    crate::standards::v1::subsets::any::io::export::svg::export_svg(model).expect("the export writes a valid document")
}

/// 📖️ A committed file of the house fixture directory.
pub fn read(name: &str) -> Vec<u8> {
    std::fs::read(format!("{HOUSE_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

/// 📖️ A committed file of the annotated room fixture directory.
pub fn read_notated(name: &str) -> Vec<u8> {
    std::fs::read(format!("{NOTATED_DIR}/{name}")).unwrap_or_else(|error| panic!("{name}: {error}. Run the test with BIM_BLESS=1 to write the file, then `python 🐍️.py write` of the export case."))
}

/// 🔖️ One start or empty tag as the third-party reader sees it: element name and attributes.
#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub depth: usize,
}

impl Tag {
    /// 🏷️ The value of an attribute.
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }

    /// 🏷️ Whether the `class` attribute lists `class`.
    pub fn has_class(&self, class: &str) -> bool {
        self.attribute("class").is_some_and(|classes| classes.split_whitespace().any(|item| item == class))
    }
}

/// 📖️ Every tag of a document in order, read by quick-xml; any well-formedness error fails the test.
pub fn tags(document: &str) -> Vec<Tag> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(document);
    let (mut out, mut depth) = (Vec::new(), 0_usize);
    loop {
        let event = reader.read_event().expect("the document is well-formed XML");
        let (start, empty) = match &event {
            Event::Start(tag) => (Some(tag), false),
            Event::Empty(tag) => (Some(tag), true),
            Event::End(_) => {
                depth -= 1;
                (None, false)
            }
            Event::Eof => break,
            _ => (None, false),
        };
        if let Some(tag) = start {
            let attributes = tag.attributes().map(|attribute| attribute.expect("a valid attribute")).map(|attribute| (String::from_utf8_lossy(attribute.key.as_ref()).into_owned(), attribute.unescape_value().expect("a valid value").into_owned())).collect();
            out.push(Tag { name: String::from_utf8_lossy(tag.name().as_ref()).into_owned(), attributes, depth });
            if !empty {
                depth += 1;
            }
        }
    }
    out
}
