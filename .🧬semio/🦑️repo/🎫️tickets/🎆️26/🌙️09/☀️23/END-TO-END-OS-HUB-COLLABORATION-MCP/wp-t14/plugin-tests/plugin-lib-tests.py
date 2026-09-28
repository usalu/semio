#!/usr/bin/env python3
"""🧪️ T14 (rule 22, test-only): `semio-framework-plugin` lib tests compile again after the peer's overnight edits — the
two-axis table fixture's `include_str!` climbs one level too far, `UiValue::Number` binds its `f64` by value, and the
media-export cleanup law reached two private SDK items. The law now goes through two `#[cfg(test)]` doors in the SDK
(`ArtifactDownloadOutput::test_from_media_export`, `VcsArtifactApp::test_segmented_closure_contains`) beside the
existing `test_store`/`test_snapshot` doors; no non-test visibility changes.
usage: plugin-lib-tests.py [--write] [--root <tree>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
M = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
EDITS = [
    (M + "🧪️tests/🔬️app-window-kits/🦀️.rs", 'include_str!("../../../🪟️window-kits/📊️table/🧫️fixtures/↔️two-axis/🔣️.json")', 'include_str!("../../🪟️window-kits/📊️table/🧫️fixtures/↔️two-axis/🔣️.json")'),
    (M + "🧪️tests/🔬️app-window-kits/🦀️.rs", "Some(UiValue::Number(value)) if *value == 400.0));", "Some(UiValue::Number(value)) if value == 400.0));"),
    (M + "🧪️tests/🔬️app-window-kits/🦀️.rs", "Some(UiValue::Number(value)) if *value == 702.0));", "Some(UiValue::Number(value)) if value == 702.0));"),
    (M + "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", 'let output = ArtifactDownloadOutput::from_media_export(handle.clone(), "audio/mpeg", chunks.clone()).expect("exact media output");', 'let output = ArtifactDownloadOutput::test_from_media_export(handle.clone(), "audio/mpeg", chunks.clone()).expect("exact media output");'),
    (M + "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", "        assert!(app.segmented_closures.contains(operation_id.0));\n", "        assert!(app.test_segmented_closure_contains(operation_id.0));\n"),
    (M + "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", "        assert!(!app.segmented_closures.contains(operation_id.0));\n", "        assert!(!app.test_segmented_closure_contains(operation_id.0));\n"),
    (M + "🦀️.rs", """        fn owns_media_export(&self, handle: &ArtifactMediaExportHandle) -> bool {
            self.media_export_handle.as_ref() == Some(handle)
        }
""", """        fn owns_media_export(&self, handle: &ArtifactMediaExportHandle) -> bool {
            self.media_export_handle.as_ref() == Some(handle)
        }

        /// 🧪️ Test-only door to the media-export constructor the SDK keeps private.
        #[cfg(test)]
        pub(crate) fn test_from_media_export(handle: ArtifactMediaExportHandle, mime_type: impl Into<String>, chunks: ArtifactOutputChunks) -> Result<Self, Fault> {
            Self::from_media_export(handle, mime_type, chunks)
        }
"""),
    (M + "🦀️.rs", """        /// ⚔️ Test-only mutable store access for conflict lifecycle fixtures.
        #[cfg(test)]
        pub(crate) async fn test_store_mut(&mut self) -> &mut ArtifactStore<A::Snapshot, A::Mutation> {
            &mut self.store
        }
""", """        /// ⚔️ Test-only mutable store access for conflict lifecycle fixtures.
        #[cfg(test)]
        pub(crate) async fn test_store_mut(&mut self) -> &mut ArtifactStore<A::Snapshot, A::Mutation> {
            &mut self.store
        }

        /// 🧹️ Test-only probe: whether a cancelled segmented output still waits in bounded cleanup.
        #[cfg(test)]
        pub(crate) fn test_segmented_closure_contains(&self, operation_id: u64) -> bool {
            self.segmented_closures.contains(operation_id)
        }
"""),
]
texts, problems = {}, []
for rel, old, new in EDITS:
    path = ROOT / rel
    text = texts.get(path) or path.read_text(encoding="utf-8")
    if new in text and (old in new or old not in text):
        texts.setdefault(path, text)
    elif text.count(old) == 1:
        texts[path] = text.replace(old, new)
    else:
        problems.append(f"{rel}: anchor found {text.count(old)} times: {old.strip()[:70]!r}")
texts = {path: text for path, text in texts.items() if text != path.read_text(encoding="utf-8")}
for path in texts:
    print(("write " if WRITE else "dry-run ") + str(path.relative_to(ROOT)))
if not texts and not problems:
    print("nothing to do (applied)")
for problem in problems:
    print("problem:", problem)
print(f"{len(texts)} files, {len(problems)} problems")
if problems:
    sys.exit(1)
if WRITE:
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
