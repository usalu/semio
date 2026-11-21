use super::{GisMapSnapshot, MapFeature};
use semio_framework_pack_json::{JsonWriteNode, JsonWriteSource};
use semio_framework_value::{DslValue, ValueError, ValueRefusalKind};

/// 🗺️ Lends the authored Map transport families and exact feature values.
pub struct MapJsonView<'a>(pub &'a GisMapSnapshot);

const FIELD_NAMES: [&str; 3] = ["positions", "routes", "regions"];

impl MapJsonView<'_> {
    fn family(&self, index: usize) -> Result<&[MapFeature], ValueError> {
        [&self.0.positions, &self.0.routes, &self.0.regions]
            .get(index).map(|features| features.as_slice()).ok_or_else(absent)
    }

    fn feature(&self, family: usize, index: usize) -> Result<&DslValue, ValueError> {
        self.family(family)?.get(index).map(|feature| &feature.data).ok_or_else(absent)
    }

    fn member(&self, family: usize, index: usize, path: &[usize]) -> Result<&DslValue, ValueError> {
        let mut value = self.feature(family, index)?;
        for &index in path {
            value = match value {
                DslValue::Array(values) => values.get(index),
                DslValue::Object(fields) => fields.get(index).map(|(_, value)| value),
                _ => None,
            }.ok_or_else(absent)?;
        }
        Ok(value)
    }
}

impl JsonWriteSource for MapJsonView<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        Ok(match path {
            [] => JsonWriteNode::Object(FIELD_NAMES.len()),
            [family] => JsonWriteNode::Array(self.family(*family)?.len()),
            [family, index, rest @ ..] => {
                let value = self.member(*family, *index, rest)?;
                if matches!(value, DslValue::Bytes(_)) {
                    return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "map data cannot be represented exactly as JSON"));
                }
                value.node_at_path(&[])?
            }
        })
    }

    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        match path {
            [] => FIELD_NAMES.get(index).copied().ok_or_else(absent),
            [family, feature, rest @ ..] => self.member(*family, *feature, rest)?.object_key_at_path(&[], index),
            _ => Err(absent()),
        }
    }
}

fn absent() -> ValueError {
    ValueError::invariant("the original GIS Map JSON projection path is absent")
}
