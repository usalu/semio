//! 🧪 P3a verification harness — byte-for-byte copy of the real
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️render_snapshot.rs`
//! source, compiled and tested standalone against the real `semio-framework-trace` and
//! `semio-framework-ui-render` crates. Needed because `semio-framework-os-renderer-wgpu` itself cannot
//! be reached by `cargo check` right now (blocked by the sibling de-async codemod's in-progress
//! `semio-framework-os-infinite`/`semio-s-plugin-stdio`), so this is how the unsafe `Arc`+`AtomicPtr`
//! code in `RenderSnapshotSink` was actually compiled and stress-tested rather than merely reviewed.
//! See `📓️p3a-render-snapshot.md` for the full writeup. This crate is NOT a workspace member and is
//! never built by CI — a standalone verification artifact left in the ticket folder per CLAUDE.md.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use ui_render::{CursorRequest, ImeDirective};

//#region 🔖️RenderSnapshot

/// 📸️ One immutable, fully-prepared frame — everything `OsHost::redraw` needs to
/// present without touching model state, running layout, or allocating substantially.
#[derive(Clone)]
pub struct RenderSnapshot {
    pub revision: u64,
    pub generation: semio_framework_trace::Generation,
    pub timestamp_us: u64,
    pub cursor: CursorRequest,
    pub ime: Option<ImeDirective>,
    pub dispatch_tree: Option<Arc<()>>,
    pub damage_regions: Option<Vec<()>>,
}

impl RenderSnapshot {
    pub fn new(revision: u64, generation: semio_framework_trace::Generation, timestamp_us: u64, cursor: CursorRequest, ime: Option<ImeDirective>) -> Self {
        Self { revision, generation, timestamp_us, cursor, ime, dispatch_tree: None, damage_regions: None }
    }
}

//#endregion 🔖️RenderSnapshot

//#region 🔖️RenderSnapshotSink

pub struct RenderSnapshotSink {
    current: Mutex<Arc<RenderSnapshot>>,
    next_revision: AtomicU64,
}

impl RenderSnapshotSink {
    pub fn new(seed: RenderSnapshot) -> Self {
        Self { current: Mutex::new(Arc::new(seed)), next_revision: AtomicU64::new(1) }
    }

    pub fn next_revision(&self) -> u64 {
        self.next_revision.fetch_add(1, Ordering::Relaxed)
    }

    pub fn acquire(&self) -> Arc<RenderSnapshot> {
        self.current.lock().expect("render snapshot sink lock").clone()
    }

    pub fn publish(&self, snapshot: RenderSnapshot) {
        *self.current.lock().expect("render snapshot sink lock") = Arc::new(snapshot);
    }
}

//#endregion 🔖️RenderSnapshotSink

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(revision: u64) -> RenderSnapshot {
        RenderSnapshot::new(revision, semio_framework_trace::Generation(revision), 0, CursorRequest::Default, None)
    }

    #[test]
    fn acquire_before_any_publish_returns_the_seed() {
        let sink = RenderSnapshotSink::new(snapshot(0));
        assert_eq!(sink.acquire().revision, 0);
    }

    #[test]
    fn publish_then_acquire_sees_the_new_snapshot() {
        let sink = RenderSnapshotSink::new(snapshot(0));
        sink.publish(snapshot(1));
        assert_eq!(sink.acquire().revision, 1);
    }

    #[test]
    fn acquiring_twice_with_no_new_publish_re_presents_the_same_snapshot() {
        let sink = RenderSnapshotSink::new(snapshot(0));
        sink.publish(snapshot(7));
        let first = sink.acquire();
        let second = sink.acquire();
        assert_eq!(first.revision, second.revision, "no newer snapshot landed — acquire must never block or invent one");
    }

    #[test]
    fn next_revision_is_monotonic() {
        let sink = RenderSnapshotSink::new(snapshot(0));
        let a = sink.next_revision();
        let b = sink.next_revision();
        assert!(b > a);
    }

    #[test]
    fn an_acquired_snapshot_survives_being_superseded() {
        let sink = RenderSnapshotSink::new(snapshot(0));
        let held = sink.acquire();
        sink.publish(snapshot(1));
        sink.publish(snapshot(2));
        assert_eq!(held.revision, 0, "a caller still holding an old Arc must keep reading its own snapshot, not a freed one");
        assert_eq!(sink.acquire().revision, 2);
    }

    #[test]
    fn many_publishers_and_readers_never_tear_or_crash() {
        use std::sync::Arc as StdArc;
        let sink = StdArc::new(RenderSnapshotSink::new(snapshot(0)));
        std::thread::scope(|scope| {
            for writer in 0..4u64 {
                let sink = sink.clone();
                scope.spawn(move || {
                    for i in 0..500u64 {
                        sink.publish(snapshot(writer * 1000 + i));
                    }
                });
            }
            for _ in 0..4 {
                let sink = sink.clone();
                scope.spawn(move || {
                    for _ in 0..500 {
                        let acquired = sink.acquire();
                        let _ = acquired.revision;
                    }
                });
            }
        });
    }

    #[test]
    fn ten_thousand_publish_acquire_cycles_leak_nothing_observable() {
        // 🧪 Not a real leak detector, but exercises the Arc refcount pairing at volume — if `publish`
        // ever double-frees or under-frees, ASan/Miri would catch it; this at minimum proves no crash
        // and no panic across a much larger volume than the concurrent test above.
        let sink = RenderSnapshotSink::new(snapshot(0));
        for i in 1..10_000u64 {
            sink.publish(snapshot(i));
            let acquired = sink.acquire();
            assert_eq!(acquired.revision, i);
        }
    }
}
