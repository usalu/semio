"""📤️ S20 export set (session 15, item 4): layout's `layout:out` media export refused "media export owner lacks an exact
bounded snapshot disposer" (SDK default `build_snapshot_disposer() → None`; only raster implements one). The SDK gains
`bounded_snapshot_disposer::<T>()` — the admitted snapshot alias released, the last owner's value through the one-page
bounded retirement the framework's config stores use — and the layout editor opts in; the layout law drives `layout:out`
to completion through the retained media export and back to close emptiness. Framework codes
`media-export.snapshot-retirement-failed` {reason?} / `-incomplete` are catalogued when this lands after row 12.
Idempotent. Usage: python3 s20-patch-layout-export.py [--base <tree>] [--write]   (default: dry run on the live tree)"""
from __future__ import annotations

import sys
from pathlib import Path

ARGS = sys.argv[1:]
BASE = Path(ARGS[ARGS.index("--base") + 1]) if "--base" in ARGS else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in ARGS
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
LAYOUT = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
LAW = "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

DISPOSER = '''    //#region 🧹️BoundedSnapshotDisposer
    /// 🧹️ The media-export snapshot disposer of an app whose snapshot needs no domain-specific paging (raster pages its
    /// pixels itself): the admitted snapshot alias is released, and the last owner's value retires through the one-page
    /// bounded retirement every framework-owned config store uses.
    pub struct BoundedSnapshotDisposer<T> {
        retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        marker: std::marker::PhantomData<fn() -> T>,
    }

    /// 🧹️ An app's `build_snapshot_disposer` for a snapshot without domain-specific retirement ([`BoundedSnapshotDisposer`]).
    pub fn bounded_snapshot_disposer<T: Send + 'static>() -> Box<dyn ArtifactSnapshotDisposer<T>> {
        Box::new(BoundedSnapshotDisposer::<T> { retirement: None, marker: std::marker::PhantomData })
    }

    impl<T: Send + 'static> ArtifactSnapshotDisposer<T> for BoundedSnapshotDisposer<T> {
        fn close_step(&mut self, snapshot: &mut Option<std::sync::Arc<T>>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
            if maximum_items == 0 {
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if let Some(retirement) = self.retirement.as_mut() {
                return Ok(match retirement.close_step(1, maximum_bytes).map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("media-export.snapshot-retirement-failed"), reason))? {
                    store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                        drop(self.retirement.take());
                        PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
                    }
                    store::SnapshotRetirementStep::Complete => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("media-export.snapshot-retirement-incomplete"), "the snapshot retirement reported completion before terminal emptiness")),
                    store::SnapshotRetirementStep::Pending { released_items, released_bytes } => PluginCloseStep::Pending { released_items, released_bytes },
                    store::SnapshotRetirementStep::Blocked => PluginCloseStep::Blocked { reason: "media export snapshot retirement awaits its exact owner" },
                });
            }
            let Some(owner) = snapshot.take() else { return Ok(PluginCloseStep::Complete) };
            if let Some(value) = std::sync::Arc::into_inner(owner) {
                self.retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&BoundedConfigRetirementFactory::<T>::new(), value));
                return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
            }
            Ok(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
        }

        fn terminal_is_empty(&self, snapshot: &Option<std::sync::Arc<T>>) -> bool {
            snapshot.is_none() && self.retirement.is_none()
        }
    }
    //#endregion 🧹️BoundedSnapshotDisposer

'''
HOOK = '''    fn build_snapshot_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactSnapshotDisposer<Self::Snapshot>>> {
        Some(semio_framework_plugin::bounded_snapshot_disposer())
    }

'''
LAW_TEXT = '''

/// 📤️ LAW: `layout:out` completes as a retained media export — the SDK's bounded snapshot disposer releases the admitted
/// snapshot — with non-empty output, the document unchanged, and the app back to close emptiness.
#[semio_framework_async_macros::async_test]
async fn layout_out_media_export_completes_without_changing_the_document() {
    use semio_framework_plugin::{ArtifactMediaExportPoll, PluginApp};
    let mut app = layout_app().await;
    let before = app.snapshot().expect("projection");
    let handle = app.submit_media_export("layout:out").await.expect("registered retained export");
    let mut result = None;
    for _ in 0..100000 {
        semio_framework_async::yield_once().await;
        match app.poll_media_export(&handle).await.expect("poll") {
            ArtifactMediaExportPoll::Running { .. } => {}
            ArtifactMediaExportPoll::Complete(media) => {
                result = Some(media);
                break;
            }
            other => panic!("layout:out did not complete: {other:?}"),
        }
    }
    let media = result.expect("complete export");
    let mut output = Vec::new();
    while let Some(chunk) = media.chunks.take_chunk().expect("chunk") {
        output.extend(chunk);
    }
    assert!(!output.is_empty(), "layout:out carries the rendered layout");
    assert_eq!(before, app.snapshot().expect("projection"), "an export never changes the document");
    drop(media);
    for _ in 0..100000 {
        if app.close_terminal_is_empty() {
            break;
        }
        app.close_step(1, 16384).expect("close step");
        semio_framework_async::yield_once().await;
    }
    assert!(app.close_terminal_is_empty(), "the export's snapshot and job retire to emptiness");
}
'''
EDITS = [
    (SDK, "    //#region 🎛️BoundedConfigStoreOwners\n", DISPOSER + "    //#region 🎛️BoundedConfigStoreOwners\n"),
    (LAYOUT, "    fn build_media_export_job(request: ArtifactMediaExportJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {\n        use crate::editor::layout::engine::export::",
     HOOK + "    fn build_media_export_job(request: ArtifactMediaExportJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {\n        use crate::editor::layout::engine::export::"),
]


def main() -> None:
    for rel, old, new in EDITS:
        path = BASE / rel
        text = path.read_text()
        if new in text:
            print("landed ", rel.split("/")[-2])
            continue
        assert text.count(old) == 1, (rel, text.count(old))
        print("apply  ", rel.split("/")[-2])
        if WRITE:
            path.write_text(text.replace(old, new))
    law = (BASE / LAW).read_text()
    if "fn layout_out_media_export_completes_without_changing_the_document" in law:
        print("landed  law")
    else:
        print("apply   law")
        if WRITE:
            (BASE / LAW).write_text(law.rstrip("\n") + LAW_TEXT)


if __name__ == "__main__":
    main()
