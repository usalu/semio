use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::tags;

fn root(children: Vec<SvgElement>) -> SvgElement {
    SvgElement::Svg { common: CommonAttrs::new().with_class("plan"), view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 20.0, height: 10.5 }), width: Some("20mm".into()), height: Some("10.5mm".into()), xmlns: Some("http://www.w3.org/2000/svg".into()), children }
}

#[test]
fn text_runs_drop_characters_xml_cannot_carry_and_keep_tabs_and_line_breaks() {
    assert_eq!(text("a\u{1}b\tc\nd\u{7f}"), SvgElement::TextNode("ab\tc\nd".into()));
}

#[test]
fn the_document_has_the_declaration_the_namespace_and_escaped_values_for_a_third_party_reader() {
    let title = element("title", vec![], vec![text("R&D <lab>")]);
    let mut common = CommonAttrs::new().with_class("storey").with_id("s1");
    common.extra_attrs = vec![attr("data-name", "say \"hi\" & \t go")];
    let group = SvgElement::Group { common, children: vec![title] };
    let document = document_text(&root(vec![group])).expect("the writer accepts the tree");
    assert!(document.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg "));
    assert!(document.contains("&amp;") && document.contains("&lt;lab"));
    let found = tags(&document);
    assert_eq!((found[0].name.as_str(), found[0].attribute("xmlns")), ("svg", Some("http://www.w3.org/2000/svg")));
    assert_eq!((found[0].attribute("viewBox"), found[0].attribute("width"), found[0].attribute("class")), (Some("0 0 20 10.5"), Some("20mm"), Some("plan")));
    assert_eq!((found[1].name.as_str(), found[1].attribute("id"), found[1].attribute("data-name")), ("g", Some("s1"), Some("say \"hi\" & \t go")));
    assert_eq!(found[2].name, "title");
}

#[test]
fn typed_primitives_print_as_the_attributes_of_their_element() {
    let path = SvgElement::Path {
        common: CommonAttrs::new().with_class("line cut"),
        d: vec![
            PathCommand::MoveTo { x: 0.0, y: 10.0, relative: false },
            PathCommand::LineTo { x: 10.0, y: 10.5, relative: false },
            PathCommand::Arc { rx: 6.25, ry: 6.25, x_axis_rotation: 0.0, large_arc: true, sweep: false, x: 0.0, y: 0.0, relative: false },
            PathCommand::ClosePath,
        ],
    };
    let label = SvgElement::Text { common: CommonAttrs::new().with_transform(vec![TransformOp::Rotate { angle: -90.0, center: Some((4.0, 2.5)) }]), x: Some(4.0), y: Some(2.5), children: vec![text("A")] };
    let document = document_text(&root(vec![path, label])).expect("the writer accepts the tree");
    let found = tags(&document);
    assert_eq!(found[1].attribute("d").map(|d| d.split_whitespace().collect::<Vec<_>>().join(" ")), Some("M 0 10 L 10 10.5 A 6.25 6.25 0 1 0 0 0 Z".to_string()));
    assert_eq!((found[2].attribute("x"), found[2].attribute("y"), found[2].attribute("transform")), (Some("4"), Some("2.5"), Some("rotate(-90,4,2.5)")));
}
