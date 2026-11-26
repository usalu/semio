//! 🌿️ Exercises the production fixed-state XML and polygon writers without linking the full app.
extern crate self as semio_s_artifact_stdio_xml;
extern crate self as semio_s_artifact_bim_model_wall_layout;

mod schema {
    pub mod snapshot {
        pub struct XmlAttr { pub name: String, pub value: String }
        pub enum XmlNode {
            Element { name: String, attrs: Vec<XmlAttr>, children: Vec<XmlNode> },
            Text { text: String },
        }
    }
}

mod standards { pub mod v1 { pub mod subsets { pub mod any { pub mod schema { pub mod inferences {
    pub mod opening_frames { pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 } }
} } } } } }

#[path = "../../../../../../../✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/🧵️xml/🦀️.rs"]
mod writer;
#[path = "../../../../../../../✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🌿️gbxml/📐️geometry/🦀️.rs"]
mod geometry;
use schema::snapshot::{XmlAttr, XmlNode};

fn element(name: &str, attributes: &[(&str, &str)], children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element {
        name: name.to_owned(),
        attrs: attributes.iter().map(|(name, value)| XmlAttr { name: (*name).into(), value: (*value).into() }).collect(),
        children,
    }
}

#[test]
fn every_byte_boundary_preserves_unicode_entity_and_wrap_rules() {
    let root = element("root", &[("sample", "München &\"\n雪\r\t")], vec![
        element("leaf", &[("alfa", "foo"), ("beta", "bar"), ("long", "abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz")], vec![XmlNode::Text{ text: "A>B\r<雪".into() }]),
        element("empty", &[], vec![]),
    ]);
    let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root sample=\"München &amp;&quot;&#10;雪&#13;&#9;\">\n  <leaf alfa=\"foo\" beta=\"bar\"\n        long=\"abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz-abcdefghijklmnopqrstuvwxyz\">A&gt;B&#13;&lt;雪</leaf>\n  <empty/>\n</root>\n";
    for size in [1, 2, 4, 15, 4096] {
        let mut cursor = writer::GbxmlTextCursor::default();
        let mut output = Vec::new();
        let mut page = vec![0; size];
        for _ in 0..10_000 {
            if cursor.is_complete() { break; }
            let written = cursor.write(&root, &mut page, 1).unwrap();
            output.extend_from_slice(&page[..written]);
        }
        assert!(cursor.is_complete());
        assert_eq!(String::from_utf8(output).unwrap(), expected, "page {size}");
    }
}

#[test]
fn one_vertex_per_slice_reproduces_full_surface_geometry() {
    let n = 2817;
    let polygon: Vec<_> = (0..n).map(|i| {
        let angle = i as f64 * std::f64::consts::TAU / n as f64;
        [angle.cos(), angle.sin(), 3.0]
    }).collect();
    let mut cursor = geometry::PolygonProjection::new(n);
    let result = (0..2 * n).find_map(|_| cursor.advance(1, |i| polygon[i]))
        .expect("two passes suffice").expect("valid ring");
    let frame = geometry::Frame::of(&polygon).unwrap();
    assert_eq!(result.frame, frame);
    assert_eq!(result.rect, geometry::rect_in(&frame, &polygon));
    assert_eq!(result.area, geometry::area(&polygon));
}
