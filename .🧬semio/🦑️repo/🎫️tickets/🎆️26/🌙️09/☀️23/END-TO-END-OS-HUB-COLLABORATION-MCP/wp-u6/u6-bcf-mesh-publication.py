#!/usr/bin/env python3
"""🧵️ U6 window-3 set C5 (T2 stdio): two retained-publication faults found by the stdio lib tests.

* bcf `set-cell` declares the `Artifact` publication lane (`BCF_RETAINED_PUBLICATION_CONTRACTS`) but the editor supplies no one-item
  preparation authority, so the retained route fails closed on EVERY cell edit: `interactive-job.publication-authority-missing` "typed
  command 'set-cell' declares the unsupported artifact publication lane" (`set_cell_reaches_the_document_through_its_exact_retained_factory`).
  The editor now grants it exactly as csv/tsv/txt/json/wav do (`bounded_config_store_one_item_preparation_factory`).
* semio mesh `MeshStructuralCopy` counted a primitive match on EVERY step before the primitive copy started — while the mesh id text
  was still being copied the mesh cursor has no primitive yet — so one primitive counted 3× and every `set-vertex` refused
  `stdio-semio-mesh-set-vertex-target-count-1-3` (`structural_copy_pages_large_mesh_and_preserves_untouched_payloads`,
  `registered_set_vertex_publishes…`). A primitive is counted exactly when the mesh cursor is in its primitive phase and has not
  started that primitive, i.e. once per primitive; a duplicate primitive id still counts twice and refuses.

Usage: u6-bcf-mesh-publication.py [--dry-run | --write | --revert] [--root <repo root>]"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
REVERT = "--revert" in sys.argv
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-backup/bcf-mesh-publication")
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/"

SETS = {
    ART + "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/✏️editor/🦀️.rs": [
        (
            """    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }
""",
            """    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 📬️ `set-cell` publishes on the `Artifact` lane; without this authority every cell edit fails closed with
    /// `interactive-job.publication-authority-missing`.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("stdio-bcf-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }
""",
            1,
        )
    ],
    ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/📬️preparation/🦀️.rs": [
        (
            """                    let primitive_not_started = self.mesh.as_ref().is_none_or(|cursor| cursor.primitive.is_none());
                    if primitive_not_started {
""",
            """                    let primitive_starts = self.mesh.as_ref().is_some_and(|cursor| cursor.phase == 1 && cursor.primitive.is_none());
                    if primitive_starts {
""",
            1,
        )
    ],
}


def key(rel: str) -> str:
    return hashlib.sha256(rel.encode()).hexdigest()[:16]


def main() -> int:
    if REVERT:
        for rel in SETS:
            backup = BACKUP / key(rel)
            if backup.exists():
                (ROOT / rel).write_bytes(backup.read_bytes())
                backup.unlink()
                print(f"REVERTED {rel}")
        return 0
    problems, planned = 0, []
    for rel, hunks in SETS.items():
        path = ROOT / rel
        text = path.read_text()
        for old, new, count in hunks:
            found = text.count(old)
            if found != count:
                print(f"PROBLEM {rel}: {'already applied' if new in text else f'anchor count {found} != {count}'}: {old[:70]!r}")
                problems += 1
                continue
            text = text.replace(old, new)
        planned.append((rel, path, text))
        print(f"{'WRITE' if WRITE else 'DRY'} {rel.rsplit('/🗿️artifacts/', 1)[-1][:60]}: {len(hunks)} hunks")
    if WRITE and problems == 0:
        BACKUP.mkdir(parents=True, exist_ok=True)
        for rel, path, text in planned:
            (BACKUP / key(rel)).write_bytes(path.read_bytes())
            path.write_text(text)
    print(f"{'write' if WRITE and problems == 0 else 'dry-run'}: {len(SETS)} files, {problems} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
