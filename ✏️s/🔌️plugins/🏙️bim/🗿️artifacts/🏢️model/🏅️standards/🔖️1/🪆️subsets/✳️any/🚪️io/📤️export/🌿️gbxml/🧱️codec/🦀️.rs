//! 🧱️ The one place that names the XML tree of the `s.stdio.xml` artifact and its text writer for the gbXML export: elements and leaves in, an indented XML 1.0 document out. Escaping, the declaration and the attribute wrapping are the writer's;
//! the indentation is whitespace between the child elements of an element, which gbXML (element-only content) ignores.
//! 📎 https://www.w3.org/TR/xml/

use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDeclaration, XmlDocument, XmlNode};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;

/// 🏷️ One attribute.
pub fn attr(name: &str, value: impl ToString) -> XmlAttr {
    XmlAttr { name: name.into(), value: value.to_string() }
}

/// 🧩️ An element with attributes and child elements.
pub fn element(name: &str, attrs: Vec<XmlAttr>, children: Vec<XmlNode>) -> XmlNode {
    XmlNode::Element { name: name.into(), attrs, children }
}

/// 🔤️ An element that holds one text run; characters XML 1.0 cannot carry (control characters other than tab, line feed and carriage return) are dropped.
pub fn leaf(name: &str, attrs: Vec<XmlAttr>, content: &str) -> XmlNode {
    let text: String = content.chars().filter(|letter| !letter.is_control() || matches!(letter, '\t' | '\n' | '\r')).collect();
    XmlNode::Element { name: name.into(), attrs, children: vec![XmlNode::Text { text }] }
}

/// 🔢️ A number with at most nine decimals and no trailing zeros, never `-0`.
pub fn number(value: f64) -> String {
    let fixed = format!("{value:.9}");
    let trimmed = fixed.trim_end_matches('0').trim_end_matches('.');
    if trimmed == "-0" || trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

/// 📏️ A leaf holding a number, with a `unit` attribute when given.
pub fn quantity(name: &str, unit: Option<&str>, value: f64) -> XmlNode {
    leaf(name, unit.map(|unit| vec![attr("unit", unit)]).unwrap_or_default(), &number(value))
}

fn indented(node: XmlNode, depth: usize) -> XmlNode {
    match node {
        XmlNode::Element { name, attrs, children } if children.iter().any(|child| matches!(child, XmlNode::Element { .. })) => {
            let pad = |level: usize| XmlNode::Text { text: format!("\n{}", "  ".repeat(level)) };
            let mut spaced = Vec::with_capacity(children.len() * 2 + 1);
            for child in children {
                spaced.push(pad(depth + 1));
                spaced.push(indented(child, depth + 1));
            }
            spaced.push(pad(depth));
            XmlNode::Element { name, attrs, children: spaced }
        }
        other => other,
    }
}

/// 📄️ The document text of a tree: declaration `1.0` and `UTF-8`, the indented root, a final line feed, validated by the writer.
pub fn document_text(root: XmlNode) -> Result<String, String> {
    let document = XmlDocument { root: Some(indented(root, 0)), declaration: Some(XmlDeclaration::new("1.0", Some("UTF-8".into()), None)), ..XmlDocument::default() };
    xml_document_to_text_checked(&document).map(|mut text| {
        text.push('\n');
        text
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
