/// @emoji 🔗️ Identifies the channel a document synchronizes through, when one is attached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBackboneRef {
    pub uri: String,
}

/// @emoji 🔗️ Builds a backbone reference from a channel URI.
pub fn document_backbone_ref(uri: &str) -> DocumentBackboneRef {
    DocumentBackboneRef { uri: uri.to_string() }
}
