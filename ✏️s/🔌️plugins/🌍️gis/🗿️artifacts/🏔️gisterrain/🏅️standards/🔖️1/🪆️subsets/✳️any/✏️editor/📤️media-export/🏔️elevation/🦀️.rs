use crate::schema::snapshot::ElevationSource;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree, JsonWriteNode, JsonWriteSource};
use semio_framework_value::{Number, ValueError};

/// 🏔️ Lends the authored elevation bounds and rows in the elevation interchange schema.
pub struct ElevationJsonView<'a>(pub &'a ElevationSource);

impl ElevationJsonView<'_> {
    fn at(&self, path: &[usize]) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> {
        let mut node: &dyn ArtifactCanonicalJsonTree = self.0;
        for &field in path {
            node = node.canonical_tree_child(field)?;
        }
        Ok(node)
    }
}

impl JsonWriteSource for ElevationJsonView<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode<'_>, ValueError> {
        Ok(match self.at(path)?.canonical_tree_node()? {
            ArtifactCanonicalJsonNode::Object(count) => JsonWriteNode::Object(count),
            ArtifactCanonicalJsonNode::Array(count) => JsonWriteNode::Array(count),
            ArtifactCanonicalJsonNode::F64(value) => JsonWriteNode::Number(Number::Float(value)),
            ArtifactCanonicalJsonNode::I64(value) => JsonWriteNode::Number(Number::Int(value)),
            ArtifactCanonicalJsonNode::String(value) | ArtifactCanonicalJsonNode::Text(ArtifactCanonicalJsonText::Contiguous(value)) => JsonWriteNode::String(value),
            _ => return Err(absent()),
        })
    }

    fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        match self.at(path)?.canonical_tree_key(index)? {
            ArtifactCanonicalJsonText::Contiguous(text) => Ok(text),
            _ => Err(absent()),
        }
    }
}

fn absent() -> ValueError {
    ValueError::invariant("the original GIS elevation JSON projection path is absent")
}
