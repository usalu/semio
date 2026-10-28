//! 🏗️ replace-document produces a structural diff and retains its semantic inverse.

use semio_s_artifact_layout_layout_diff::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ReplaceDocument {
    pub document: LayoutSnapshot,
}

impl<Op> MutationKind<LayoutSnapshot, Op> for ReplaceDocument
where Op: protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::replace_document::ReplaceDocument> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "document", kind: "replace-document", record: "ReplacedDocument" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Replace layout document", "Layoutdokument ersetzen") }
    fn target(&self) -> Vec<String> { Vec::new() }
}

impl<Op> protocol::ColdMutationKind<LayoutSnapshot, Op> for ReplaceDocument
where Op: protocol::ColdMutation<LayoutSnapshot, ReferenceDiff = semio_s_artifact_layout_layout_diff::LayoutDiff> + protocol::Mutation<LayoutSnapshot, Diff = LayoutDiff> + From<crate::replace_document::ReplaceDocument> {
    fn diff(&self, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> { diff_replace_document(self, base) }
    fn inverse(&self, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { inverse_replace_document(self, base) }
}

/// 🔺️ Resolves a semantic payload into a sparse document diff.
pub fn diff_replace_document(payload: &ReplaceDocument, base: &LayoutSnapshot) -> protocol::MutationOutcome<LayoutDiff> {
    let document = &payload.document;
    if let Err(message) = semio_s_artifact_layout_layout_model::validation::validate_document(document) {
        return protocol::MutationOutcome::fatal("mutation.invariant", message, std::iter::empty::<String>());
    }
    if document == base { return protocol::MutationOutcome::empty().warning("mutation.no-op", "The document is already set.") }
    let entries = document.data_fields.as_ref().map(|fields| LayoutDataEntriesDelta { inserted: fields.entries.iter().cloned().enumerate().map(|(index, row)| LayoutDataEntryInsertion { index, row }).collect(), ..Default::default() });
    let data_fields = match (&base.data_fields, &document.data_fields) {
        (None, None) => None,
        (_, None) => Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None }),
        (None, Some(_)) => Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries }),
        (Some(_), Some(_)) => Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Replaced), entries }),
    };
    protocol::MutationOutcome::new(LayoutDiff {
        schema: Some(document.schema.clone()), name: Some(document.name.clone()),
        grid: Some(GridPatch { baseline_grid: Some(document.grid.baseline_grid), baseline_offset: Some(document.grid.baseline_offset), snap_to_baseline: Some(document.grid.snap_to_baseline) }),
        paragraph_styles: Some(LayoutParagraphStylesDelta { removed: base.paragraph_styles.iter().enumerate().map(|(index, row)| LayoutParagraphStyleRemoval { id: row.id.clone(), index }).collect(), inserted: document.paragraph_styles.iter().cloned().enumerate().map(|(index, row)| LayoutParagraphStyleInsertion { index, row }).collect(), ..Default::default() }),
        character_styles: Some(LayoutCharacterStylesDelta { removed: base.character_styles.iter().enumerate().map(|(index, row)| LayoutCharacterStyleRemoval { id: row.id.clone(), index }).collect(), inserted: document.character_styles.iter().cloned().enumerate().map(|(index, row)| LayoutCharacterStyleInsertion { index, row }).collect(), ..Default::default() }),
        parent_pages: Some(LayoutParentPagesDelta { removed: base.parent_pages.iter().enumerate().map(|(index, row)| LayoutParentPageRemoval { id: row.id.clone(), index }).collect(), inserted: document.parent_pages.iter().cloned().enumerate().map(|(index, row)| LayoutParentPageInsertion { index, row }).collect(), ..Default::default() }),
        spreads: Some(LayoutSpreadsDelta { removed: base.spreads.iter().enumerate().map(|(index, row)| LayoutSpreadRemoval { id: row.id.clone(), index }).collect(), inserted: document.spreads.iter().cloned().enumerate().map(|(index, row)| LayoutSpreadInsertion { index, row }).collect(), ..Default::default() }),
        pages: Some(LayoutPagesDelta { removed: base.pages.iter().enumerate().map(|(index, row)| LayoutPageRemoval { id: row.id.clone(), index }).collect(), inserted: document.pages.iter().cloned().enumerate().map(|(index, row)| LayoutPageInsertion { index, row }).collect(), ..Default::default() }),
        stories: Some(LayoutStoriesDelta { removed: base.stories.iter().enumerate().map(|(index, row)| LayoutStoryRemoval { id: row.id.clone(), index }).collect(), inserted: document.stories.iter().cloned().enumerate().map(|(index, row)| LayoutStoryInsertion { index, row }).collect(), ..Default::default() }),
        links: Some(LayoutLinksDelta { removed: base.links.iter().enumerate().map(|(index, row)| LayoutLinkRemoval { id: row.id.clone(), index }).collect(), inserted: document.links.iter().cloned().enumerate().map(|(index, row)| LayoutLinkInsertion { index, row }).collect(), ..Default::default() }),
        print_target: Some(PrintTargetChange { target: document.print_target.clone() }), data_fields,
        background_drawing: Some(document.background_drawing.clone()), referenced_model: Some(document.referenced_model.clone()),
    })
}

/// ↩️ Restores every affected field and its original position.
pub fn inverse_replace_document<Op: From<crate::replace_document::ReplaceDocument>>(payload: &ReplaceDocument, base: &LayoutSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if diff_replace_document(payload, base).diff() == &LayoutDiff::default() { return Ok(Vec::new()) }
    Ok(vec![Op::from(ReplaceDocument { document: base.clone() })])
}

#[cfg(any(test, feature = "testkit"))]
impl serde::Serialize for ReplaceDocument {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&serde_json::Value::from(semio_framework_value::ToValue::to_value(self)), serializer)
    }
}

#[cfg(any(test, feature = "testkit"))]
impl<'de> serde::Deserialize<'de> for ReplaceDocument {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <serde_json::Value as serde::Deserialize>::deserialize(deserializer)?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_value::DslValue::from(&value)).map_err(serde::de::Error::custom)
    }
}
