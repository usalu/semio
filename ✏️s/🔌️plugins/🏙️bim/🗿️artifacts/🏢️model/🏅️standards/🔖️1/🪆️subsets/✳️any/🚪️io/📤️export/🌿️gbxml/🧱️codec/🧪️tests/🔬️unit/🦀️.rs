use super::*;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text;

#[test]
fn numbers_have_at_most_nine_decimals_and_no_trailing_zeros() {
    assert_eq!(number(8.0), "8");
    assert_eq!(number(2.5), "2.5");
    assert_eq!(number(1.0 / 3.0), "0.333333333");
    assert_eq!(number(-0.000_000_000_1), "0", "no negative zero");
    assert_eq!(number(-2.25), "-2.25");
}

#[test]
fn a_leaf_drops_control_characters_and_the_writer_escapes_markup() {
    let root = element("a", vec![attr("name", "x\"<y>&")], vec![leaf("b", Vec::new(), "1 < 2 & \u{1}3")]);
    let text = document_text(root).expect("a document");
    assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<a name=\"x&quot;&lt;y>&amp;\">"), "{text}");
    assert!(text.contains("<b>1 &lt; 2 &amp; 3</b>"), "{text}");
    assert!(text.ends_with("</a>\n"));
}

#[test]
fn elements_with_elements_are_indented_and_a_leaf_stays_on_one_line() {
    let root = element("a", Vec::new(), vec![element("b", Vec::new(), vec![leaf("c", Vec::new(), "x")]), leaf("d", vec![attr("unit", "m")], "2")]);
    let text = document_text(root).expect("a document");
    assert_eq!(text, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<a>\n  <b>\n    <c>x</c>\n  </b>\n  <d unit=\"m\">2</d>\n</a>\n");
}

#[test]
fn the_text_reads_back_as_the_same_elements_apart_from_the_indentation() {
    let root = element("a", vec![attr("id", "x")], vec![element("b", Vec::new(), vec![leaf("c", Vec::new(), "x")])]);
    let text = document_text(root).expect("a document");
    let read = xml_document_from_text(&text).expect("parses");
    let XmlNode::Element { name, attrs, children } = read.root.expect("a root") else { panic!("an element") };
    assert_eq!((name.as_str(), attrs.len()), ("a", 1));
    assert_eq!(children.iter().filter(|child| matches!(child, XmlNode::Element { .. })).count(), 1);
}

#[test]
fn a_quantity_carries_its_unit_only_when_given() {
    assert_eq!(document_text(quantity("Area", None, 9.99)).expect("a document"), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Area>9.99</Area>\n");
    assert!(document_text(quantity("U-value", Some("WPerSquareMeterK"), 0.5)).expect("a document").contains("<U-value unit=\"WPerSquareMeterK\">0.5</U-value>"));
}
