//! 📦️ Package and manifest nodes: the artifact references, the referenced model, the link pins, the blob references and the manifest of the layout export JSON expansion. See [`super`].

use super::*;

impl TypedJsonCursor {
    /// 🪡 Expands the `BlobRef` node into its output fragments.
    pub(super) fn expand_blob_ref(&mut self, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let blob = match &snapshot.referenced_model.as_ref().ok_or("layout-export-json-reference")?.pin {
            store::LinkPin::Snapshot { blob } => blob,
            _ => return Err("layout-export-json-reference".into()),
        };
        self.push_sequence(vec![
            Self::static_node(b"{\"hash\":"),
            Self::string(StringSource::ReferencedBlobHash),
            Self::static_node(b",\"size\":"),
            Self::scalar(&blob.size)?,
            Self::static_node(b",\"mediaType\":"),
            Self::string(StringSource::ReferencedBlobMediaType),
            Self::static_node(b"}"),
        ]);
        Ok(())
    }

    /// 🪢 Expands the `MissingLinks` node into its output fragments.
    pub(super) fn expand_missing_links(&mut self, index: usize, opened: bool, emitted: bool, snapshot: &LayoutSnapshot) -> Result<(), String> {
        if !opened {
            self.push_sequence(vec![Self::static_node(b"["), TypedJsonNode::MissingLinks { index, opened: true, emitted }]);
        } else if let Some(link) = snapshot.links.get(index) {
            if link.state.as_deref() == Some("missing") {
                self.push_sequence(vec![if emitted { Self::static_node(b",") } else { Self::static_node(b"") }, TypedJsonNode::MissingLink(index), TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted: true }]);
            } else {
                self.stack.push(TypedJsonNode::MissingLinks { index: index + 1, opened: true, emitted });
            }
        } else {
            self.stack.push(Self::static_node(b"]"));
        }
        Ok(())
    }

    /// 🪵 Expands the `ManifestLink` node into its output fragments.
    pub(super) fn expand_manifest_link(&mut self, index: usize, snapshot: &LayoutSnapshot) -> Result<(), String> {
        let link = snapshot.links.get(index).ok_or("layout-export-json-index")?;
        let hash = if link.hash.is_empty() { Self::owned_string(format!("sha256:{}", semio_framework_hash::sha256_hex(link.path.as_bytes()))) } else { Self::string(StringSource::LinkHash(index)) };
        self.push_sequence(vec![
            Self::static_node(b"{\"id\":"),
            Self::string(StringSource::LinkId(index)),
            Self::static_node(b",\"path\":"),
            Self::string(StringSource::LinkPath(index)),
            Self::static_node(b",\"hash\":"),
            hash,
            Self::static_node(b",\"state\":"),
            Self::optional_string(link.state.is_some(), StringSource::LinkState(index)),
            Self::static_node(b",\"missing\":"),
            Self::scalar(&(link.state.as_deref() == Some("missing")))?,
            Self::static_node(b"}"),
        ]);
        Ok(())
    }
}
