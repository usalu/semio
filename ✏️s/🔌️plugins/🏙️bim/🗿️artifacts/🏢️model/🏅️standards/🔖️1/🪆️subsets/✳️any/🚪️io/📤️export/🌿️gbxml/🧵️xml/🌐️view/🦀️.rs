//! 🌐️ Retains the XML structure while borrowing polygon coordinates directly from the exporter plan.

use crate::standards::v1::subsets::any::io::export::gbxml::{GbxmlAttribute, GbxmlContent, GbxmlElement, GbxmlText, GbxmlTreeSource};
use semio_s_artifact_bim_model_examples::standards::v1::subsets::any::io::export::gbxml::codec;
use semio_s_artifact_bim_model_curtain_walls::standards::v1::subsets::any::io::export::gbxml::geometry::P3;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};

pub(crate) enum Markup<'a> {
    Element { name: &'static str, attrs: Vec<XmlAttr>, children: Vec<Markup<'a>> },
    Ready(XmlNode),
    PolyLoop(&'a [P3]),
}

pub(crate) fn element<'a>(name: &'static str, attrs: Vec<XmlAttr>, children: Vec<Markup<'a>>) -> Markup<'a> {
    Markup::Element { name, attrs, children }
}

pub(crate) fn leaf<'a>(name: &str, attrs: Vec<XmlAttr>, value: &str) -> Markup<'a> {
    Markup::Ready(codec::leaf(name, attrs, value))
}

pub(crate) fn quantity<'a>(name: &str, unit: Option<&str>, value: f64) -> Markup<'a> {
    Markup::Ready(codec::quantity(name, unit, value))
}

impl Markup<'_> {
    pub(crate) fn into_node(self) -> XmlNode {
        match self {
            Self::Element { name, attrs, children } => codec::element(name, attrs, children.into_iter().map(Self::into_node).collect()),
            Self::Ready(node) => node,
            Self::PolyLoop(polygon) => codec::element("PolyLoop", Vec::new(), polygon.iter().map(|point|
                codec::element("CartesianPoint", Vec::new(), point.iter().map(|value|
                    with_number_text(*value, |number| codec::leaf("Coordinate", Vec::new(), number))
                ).collect())
            ).collect()),
        }
    }
}

impl GbxmlTreeSource for Markup<'_> {
    fn with_element<R>(&self, path: &[usize], read: impl FnOnce(GbxmlElement<'_>) -> R) -> Result<R, &'static str> {
        let shallow = |name, contents| GbxmlElement { name, attributes: &[], contents };
        match self {
            Self::Ready(node) => node.with_element(path, read),
            Self::Element { name, attrs, children } => {
                if let Some((index, remaining)) = path.split_first() {
                    return children.get(*index).ok_or("gbXML element child is absent")?.with_element(remaining, read);
                }
                let mut attributes = [GbxmlAttribute::default(); 16];
                let active = attributes.get_mut(..attrs.len()).ok_or("gbXML element has too many attributes")?;
                for (target, source) in active.iter_mut().zip(attrs) {
                    *target = GbxmlAttribute { name: &source.name, value: GbxmlText::Text(&source.value) };
                }
                let contents = if children.is_empty() { GbxmlContent::Empty } else { GbxmlContent::Elements(children.len()) };
                Ok(read(GbxmlElement { name, attributes: active, contents }))
            }
            Self::PolyLoop(points) => match path {
                [] => Ok(read(shallow("PolyLoop", if points.is_empty() { GbxmlContent::Empty } else { GbxmlContent::Elements(points.len()) }))),
                [index] if *index < points.len() => Ok(read(shallow("CartesianPoint", GbxmlContent::Elements(3)))),
                [index, axis] => {
                    let value = points.get(*index).and_then(|point| point.get(*axis)).ok_or("gbXML polygon coordinate is absent")?;
                    Ok(with_number_text(*value, |text| read(GbxmlElement { name: "Coordinate", attributes: &[], contents: GbxmlContent::Text(GbxmlText::Text(text)) })))
                }
                _ => Err("gbXML path is outside the polygon"),
            },
        }
    }
}

pub(super) fn with_number_text<R>(value: f64, read: impl FnOnce(&str) -> R) -> R {
    use std::fmt::Write;
    struct Number { bytes: [u8; 320], len: usize }
    impl Write for Number {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            let end = self.len + text.len();
            self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?.copy_from_slice(text.as_bytes());
            self.len = end;
            Ok(())
        }
    }
    let mut fixed = Number { bytes: [0; 320], len: 0 };
    write!(&mut fixed, "{value:.9}").expect("any f64 decimal with nine places fits in 320 bytes");
    let text = std::str::from_utf8(&fixed.bytes[..fixed.len]).expect("decimal is ASCII");
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    read(if trimmed == "-0" || trimmed.is_empty() { "0" } else { trimmed })
}
