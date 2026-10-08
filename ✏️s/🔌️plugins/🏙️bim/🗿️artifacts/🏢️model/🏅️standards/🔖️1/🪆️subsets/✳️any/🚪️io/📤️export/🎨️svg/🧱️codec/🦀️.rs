//! 🧱️ The one place that names the typed SVG tree of the `s.stdio.svg` artifact and the text writer of the `s.stdio.xml` artifact: typed elements in, the XML 1.0 document text out.
//! 📎 https://www.w3.org/TR/SVG11/

pub use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{CommonAttrs, PathCommand, SvgElement, TransformOp, ViewBox};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::typed_to_svg_document;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDeclaration};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;

/// 🏷️ One attribute that has no typed slot (`data-*`, `aria-label`, `version`, `dy`).
pub fn attr(name: &str, value: impl ToString) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.to_string() }
}

/// 🔤️ A text run: characters XML 1.0 cannot carry (control characters other than tab, line feed and carriage return) are dropped, escaping is the writer's.
pub fn text(content: &str) -> SvgElement {
    SvgElement::TextNode(content.chars().filter(|letter| !letter.is_control() || matches!(letter, '\t' | '\n' | '\r')).collect())
}

/// 🧩️ An element outside the typed set (`title`, `desc`, `style`) with its attributes and children.
pub fn element(name: &str, attrs: Vec<XmlAttr>, children: Vec<SvgElement>) -> SvgElement {
    SvgElement::Unknown { name: name.into(), attrs, children }
}

/// 📄️ The XML document text of an SVG tree: declaration `1.0` and `UTF-8`, then the root, validated by the writer.
pub fn document_text(root: &SvgElement) -> Result<String, String> {
    let mut document = typed_to_svg_document(root, None);
    document.declaration = Some(XmlDeclaration::new("1.0", Some("UTF-8".into()), None));
    xml_document_to_text_checked(&document)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
