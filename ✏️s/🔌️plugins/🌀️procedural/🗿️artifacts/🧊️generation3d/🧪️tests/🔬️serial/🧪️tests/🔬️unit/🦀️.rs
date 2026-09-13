//! ⚖️ The lock discipline's own laws. Every one of them would HANG rather than fail if the
//! discipline regressed, so each is written to reach its verdict in bounded time on this thread.

use super::{held_by_this_thread, lock};

/// ⚖️ LAW: a law that PANICS while holding the serial lock releases it.
///
/// The unwind runs `TestSerialGuard::drop`, so the mutex is free and the owner record is clear for
/// the next law. Without this the first panicking law in the binary would strand every later one at
/// zero CPU — the featured `--lib` suite's whole "deadlock"
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️gates-green-2026-09-13.md` §5.1).
#[test]
fn a_panicking_law_releases_the_serial_lock() {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(|| {
        let _serial = lock();
        panic!("a law under test fails while holding the serial lock");
    });
    std::panic::set_hook(previous_hook);
    assert!(outcome.is_err(), "the law under test must actually have panicked");
    assert!(!held_by_this_thread(), "the panicking law's guard must have cleared the owner record");
    let _serial = lock();
    assert!(held_by_this_thread(), "the lock must be takeable again after a law panicked while holding it");
}

/// ⚖️ LAW: a law that RETURNS EARLY while holding the serial lock releases it — the `?`/`return`
/// path a bare `drop(guard)` at the bottom of a body would miss.
#[test]
fn an_early_returned_law_releases_the_serial_lock() {
    fn law_that_returns_early() -> Option<u64> {
        let _serial = lock();
        assert!(held_by_this_thread());
        None?;
        unreachable!("the early return above is the point of this law")
    }
    assert_eq!(law_that_returns_early(), None);
    assert!(!held_by_this_thread(), "the early-returned guard must have cleared the owner record");
    let _serial = lock();
}

/// ⚖️ LAW: a NESTED acquisition is refused, loudly and immediately.
///
/// `std::sync::Mutex` is not reentrant, so the alternative is not a second lock — it is a permanent
/// hang of the whole binary with no attribution. A named panic on the offending thread costs one
/// law and names the mistake.
#[test]
fn a_nested_acquisition_is_refused_instead_of_hanging() {
    let serial = lock();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let nested = std::panic::catch_unwind(|| drop(lock()));
    std::panic::set_hook(previous_hook);
    let message = *nested.expect_err("a nested acquisition must be refused, not granted").downcast::<String>().expect("the refusal names itself");
    assert!(message.contains("already holds the generation3d test serial lock"), "unexpected refusal: {message}");
    assert!(held_by_this_thread(), "the refused nested acquisition must leave the outer guard intact");
    drop(serial);
    assert!(!held_by_this_thread());
}

/// ⚖️ LAW: a guard that mutates process-global state restores it when the law it guards FAILS, and
/// does so without taking the binary down.
///
/// 💣️ A `Drop` that panics while its thread is already unwinding is a non-unwinding panic: the
/// process aborts and every remaining law is never run. The featured `--lib` suite died that way at
/// test 395 of 438 — a close-ladder law failed, and `UnlinkedFlowExtensions`'s registry restore then
/// raised `flow.registry-retirement-full` inside that unwind, costing 43 laws to a diagnostic about
/// a law that had already reported itself (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[test]
fn a_failing_law_does_not_abort_the_binary_through_the_registry_guard() {
    let _serial = lock();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::take();
        panic!("a law under test fails while the linked operator packs are retired");
    }));
    std::panic::set_hook(previous_hook);
    assert!(outcome.is_err(), "the law under test must actually have panicked");
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_ok(),
        "the guard must have put the linked packs back even though the law it guarded panicked"
    );
}

/// ⚖️ LAW: the lock really is ONE lock — the editor's door, the viewer's door and the publication
/// door all open it, so a thread holding any one of them is refused by the other two. Three mutexes
/// over one shared state is not serialisation, and this is the law that keeps them from drifting
/// apart again.
#[test]
fn every_surfaces_door_opens_the_same_lock() {
    let serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for door in [
        std::panic::catch_unwind(|| drop(crate::viewer::generation3d::unit_tests::context::lock())),
        std::panic::catch_unwind(|| drop(crate::publication_authority::lock())),
        std::panic::catch_unwind(|| drop(lock())),
    ] {
        assert!(door.is_err(), "a second door must not hand out a second lock over the same process-global state");
    }
    std::panic::set_hook(previous_hook);
    drop(serial);
    assert!(!held_by_this_thread());
}
