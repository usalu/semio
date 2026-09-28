//! 📥️ `AdmitLocalDocument` is the authoritative direct Rust leaf for listing one document in this device's local catalog.

use super::retire_local_document::RetireLocalDocument;
use super::LocalCatalogConfigMutation;
use protocol::{MutationDiff, MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_os_kernel::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Schema
/// 🗄️ Where a local document's events persist on this device: a folder event log or one file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum LocalDocumentStorage {
    Folder,
    File,
}

/// 📄️ One document this device keeps locally: its id, artifact schema, name and where its events persist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct LocalDocument {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub storage: LocalDocumentStorage,
    pub target: String,
    pub admitted_at_ms: u64,
}

/// 🗂️ `os.config.local-catalog` — every document this device keeps locally (persisted local-only), ordered by id.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase", default)]
pub struct LocalCatalog {
    pub documents: Vec<LocalDocument>,
}

/// 🗂️ The schema id for the local document catalog config facet.
pub const LOCAL_CATALOG_CONFIG_SCHEMA: &str = "os.config.local-catalog";

impl MutationDiff<LocalCatalog> for LocalCatalog {
    fn apply(&self, _base: &LocalCatalog) -> protocol::MutationApplyResult<LocalCatalog> {
        Ok(self.clone())
    }

    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Schema

//#region 🔖️Mutation
/// 📥️ Lists one document in the local catalog, replacing any entry with the same id.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct AdmitLocalDocument {
    pub document_id: String,
    pub schema: String,
    pub name: String,
    pub storage: LocalDocumentStorage,
    pub target: String,
    pub admitted_at_ms: u64,
}

/// 🏗️ Wraps an admit payload in the local-catalog dispatch enum.
pub fn admit_local_document(document: LocalDocument) -> LocalCatalogConfigMutation {
    LocalCatalogConfigMutation::AdmitLocalDocument(AdmitLocalDocument::from(document))
}

impl From<LocalDocument> for AdmitLocalDocument {
    fn from(document: LocalDocument) -> Self {
        Self { document_id: document.document_id, schema: document.schema, name: document.name, storage: document.storage, target: document.target, admitted_at_ms: document.admitted_at_ms }
    }
}

impl From<&AdmitLocalDocument> for LocalDocument {
    fn from(payload: &AdmitLocalDocument) -> Self {
        Self { document_id: payload.document_id.clone(), schema: payload.schema.clone(), name: payload.name.clone(), storage: payload.storage, target: payload.target.clone(), admitted_at_ms: payload.admitted_at_ms }
    }
}

impl MutationKind<LocalCatalog, LocalCatalogConfigMutation> for AdmitLocalDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "local-document", kind: "admit-local-document", record: "Set" };

    fn diff(&self, base: &LocalCatalog) -> MutationOutcome<LocalCatalog> {
        let admitted = LocalDocument::from(self);
        if base.documents.iter().any(|entry| *entry == admitted) {
            return MutationOutcome::new(base.clone()).warn("mutation.no-op", format!("\"{}\" is already listed in the local catalog.", self.document_id));
        }
        let mut documents: Vec<LocalDocument> = base.documents.iter().filter(|entry| entry.document_id != self.document_id).cloned().collect();
        documents.push(admitted);
        documents.sort_by(|left, right| left.document_id.cmp(&right.document_id));
        MutationOutcome::new(LocalCatalog { documents })
    }

    fn inverse(&self, base: &LocalCatalog) -> Vec<LocalCatalogConfigMutation> {
        match base.documents.iter().find(|entry| entry.document_id == self.document_id) {
            Some(prior) => vec![admit_local_document(prior.clone())],
            None => vec![LocalCatalogConfigMutation::RetireLocalDocument(RetireLocalDocument { document_id: self.document_id.clone() })],
        }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Keep \"{}\" on this device", self.name), &format!("\"{}\" auf diesem Gerät behalten", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.document_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🌉️MutationCodecBridge
/// 🧮️ Applies one local-catalog mutation through its whole-record diff.
pub fn apply_local_catalog_config_mutation(snapshot: &mut LocalCatalog, mutation: &LocalCatalogConfigMutation) -> protocol::MutationApplyResult<()> {
    use protocol::{Mutation as _, MutationDiff as _};
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

/// ↩️ Computes the mutation's inverse steps from the pre-mutation catalog.
pub fn inverse_local_catalog_config_mutation(snapshot: &LocalCatalog, mutation: &LocalCatalogConfigMutation) -> Vec<LocalCatalogConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(snapshot)
}

/// 📥️ Decodes the internally tagged local-catalog mutation JSON projection.
pub fn decode_local_catalog_config_mutation_json(text: &str) -> Result<LocalCatalogConfigMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ Encodes the local catalog to its canonical camel-case JSON projection.
pub fn encode_local_catalog_json(snapshot: &LocalCatalog) -> String {
    serde_json::to_string(snapshot).expect("LocalCatalog serialization is infallible")
}

/// 📥️ Decodes the canonical local catalog JSON projection.
pub fn decode_local_catalog_json(text: &str) -> Result<LocalCatalog, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ Applies a mutation and returns its diagnostic `(code, severity)` pairs.
pub fn apply_local_catalog_config_mutation_reporting(snapshot: &mut LocalCatalog, mutation: &LocalCatalogConfigMutation) -> Vec<(String, String)> {
    use protocol::Mutation as _;
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ Returns the mutation's own inverse steps for an external fixture adapter.
pub fn inverse_local_catalog_config_mutation_steps(mutation: &LocalCatalogConfigMutation, base: &LocalCatalog) -> Vec<LocalCatalogConfigMutation> {
    use protocol::Mutation as _;
    mutation.inverse(base)
}
//#endregion 🌉️MutationCodecBridge

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📥️lists-a-persisted-studio-beside-an-imported-one/🦀️.rs"]
mod tests_lists_a_persisted_studio_beside_an_imported_one;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
