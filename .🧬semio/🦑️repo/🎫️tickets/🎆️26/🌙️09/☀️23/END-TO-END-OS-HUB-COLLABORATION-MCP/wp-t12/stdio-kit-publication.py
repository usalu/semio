#!/usr/bin/env python3
"""📤️ T12 / S15 stdio ×9, step 2: gives every stdio editor an artifact one-item publication authority.
The kit verb's `Artifact` lane otherwise fails closed with `interactive-job.publication-authority-missing`
(measured by the live kit-verb law). Usage: stdio-kit-publication.py [--write]"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
EDITORS = {
    "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any": "csv",
    "📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any": "tsv",
    "🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any": "txt",
    "📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any": "md",
    "🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any": "html",
    "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base": "json",
    "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json": "json-i-json",
    "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base": "xml",
    "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid": "xml-valid",
}
ANCHOR = """    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
"""
ADDITION = """
    /// 📤️ The artifact lane's one-item publication authority. The kit verb's route declares the
    /// `Artifact` lane, and without this authority every such route fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {{
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-{label}-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }}
"""
write = "--write" in sys.argv
for relative, label in EDITORS.items():
    path = ROOT / relative / "✏️editor" / "🦀️.rs"
    text = path.read_text()
    if "fn build_artifact_store_one_item_preparation_factory" in text or text.count(ANCHOR) != 1:
        raise SystemExit(f"{path}: anchor")
    if write:
        path.write_text(text.replace(ANCHOR, ANCHOR + ADDITION.format(label=label)))
        print(f"wrote {relative}")
