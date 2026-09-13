//! 🔒️ THE process-global serial lock of this crate's test binary, and the only one.
//!
//! Every law in this binary that touches process-global state — the flow extension registry and its
//! linked operator packs, the neural kernel cache `FlowHost::evaluate` reads, `tessellate_geometry`'s
//! mesh cache, and the fixed `GENERATION3D_PUBLICATION_SLOTS` lease table — must be serialised
//! against every OTHER such law, not merely against the ones that happen to sit on the same surface.
//!
//! 🐛️ It used to be three independent mutexes: the editor's `serial_execution::lock`, the viewer's
//! `context::lock` and `publication_authority::lock`. Three mutexes over ONE shared state is not
//! serialisation — an editor law and a viewer law held different locks and ran at the same time, and
//! a law that admitted a publication lease while an editor law did the same saturated the four-slot
//! table and lost with `generation3d-publication.contended`, an outcome that has nothing to do with
//! what either law asserts. Two disjoint locks over one resource is also the shape a lock-order
//! inversion needs (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
//!
//! 🪤 A non-reentrant mutex taken twice by one thread does not fail — it HANGS, forever, with the
//! whole binary's remaining laws queued behind it at zero CPU, which is exactly the symptom the
//! featured `--lib` suite showed (`📓️gates-green-2026-09-13.md` §5.1). [`lock`] therefore refuses a
//! nested acquisition loudly instead, so a discipline mistake costs one named failure rather than
//! the run.
//!
//! 🧹️ The guard releases on EVERY exit — normal return, early return, `?`, and a panicking law's
//! unwind — because release is `Drop`, and a poisoned mutex is taken anyway: a law that panicked
//! while holding it has already reported its own failure and must not cascade into every later law.
//! `🧪️tests/🔬️unit/🦀️.rs` states all three as laws.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

/// 🪪️ The token of the thread holding [`TEST_SERIAL`] right now, or `0` for nobody. Written only
/// under the mutex, cleared by [`TestSerialGuard::drop`].
static OWNER: AtomicU64 = AtomicU64::new(0);

static NEXT_THREAD_TOKEN: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static THREAD_TOKEN: u64 = NEXT_THREAD_TOKEN.fetch_add(1, Ordering::Relaxed);
}

/// 🪪️ A stable non-zero id for the calling thread. `ThreadId::as_u64` is unstable, and a thread's
/// address is reused after it exits, so the token is minted once per thread from a counter.
fn thread_token() -> u64 {
    THREAD_TOKEN.with(|token| *token)
}

/// 🔒️ The held lock. Releasing is `Drop`, so every exit path releases and the owner record is
/// cleared with it.
pub(crate) struct TestSerialGuard {
    _inner: MutexGuard<'static, ()>,
}

impl Drop for TestSerialGuard {
    fn drop(&mut self) {
        OWNER.store(0, Ordering::Release);
    }
}

/// 🔒️ Takes the one serial lock and installs the packaged `brep`/`math` operator sets behind it.
///
/// 🌿️ The installation is INSIDE the critical section on purpose: it mutates the process-global
/// extension registry, which is one of the very things this lock serialises. Doing it before the
/// mutex — as the editor's old door did — let a first-ever caller rewrite the registry while another
/// law was already asserting on it.
pub(crate) fn lock() -> TestSerialGuard {
    let token = thread_token();
    assert_ne!(OWNER.load(Ordering::Acquire), token, "this thread already holds the generation3d test serial lock — a nested acquisition would hang the whole binary, so it is refused: take the lock once, at the top of the law");
    let inner = TEST_SERIAL.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    OWNER.store(token, Ordering::Release);
    crate::flow_operators::installed();
    TestSerialGuard { _inner: inner }
}

/// 🪪️ Whether the calling thread holds the lock right now — the laws' own witness that a guard
/// really was released.
pub(crate) fn held_by_this_thread() -> bool {
    OWNER.load(Ordering::Acquire) == thread_token()
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
