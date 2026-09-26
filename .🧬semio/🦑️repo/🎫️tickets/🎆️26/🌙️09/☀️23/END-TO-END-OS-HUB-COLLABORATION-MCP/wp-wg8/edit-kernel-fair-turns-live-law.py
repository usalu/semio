#!/usr/bin/env python3
"""WG8 s12: the live law of the fair kernel turns — a long guest `InstanceOpen` (hub-resolved puzzle component) completes
while another app keeps answering, and dropping a second open (its owner's cancel) ends that turn."""
import pathlib

RENDERER = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
LAWS = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")

text = RENDERER.read_text()
old = """    const TURN_REQUESTS_BETWEEN_SLICES: usize = 1;
"""
assert text.count(old) == 1
text = text.replace(old, old + """
    /// 🧪️ How many mid-flight turns their owners cancelled in this test process ([`KernelPoolState::retire_cancelled_turn`]).
    #[cfg(test)]
    pub(crate) static CANCELLED_KERNEL_TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
""")
old = """            self.begin_fault_close(actor);
            format!("kernel: actor {}'s turn was cancelled by its owner", actor.0)"""
assert text.count(old) == 1
text = text.replace(old, """            self.begin_fault_close(actor);
            #[cfg(test)]
            CANCELLED_KERNEL_TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            format!("kernel: actor {}'s turn was cancelled by its owner", actor.0)""")
RENDERER.write_text(text)

laws = LAWS.read_text()
ANCHOR = "/// 📒️ Pumps one shell's frames ([`frame_pump`]) and re-reads its edit ledger"
assert laws.count(ANCHOR) == 1
LAW = r'''/// ⚖️ A long guest turn shares the native kernel and only its owner's cancel ends it (ticket 26/09/23 WG8,
/// coordinator-approved design): the hub-resolved puzzle component's `InstanceOpen` — longer than the old 30 s
/// wall-clock kill in a debug host under load (cross-shell run 10) — completes while the staged block2d app keeps
/// answering history reads between its slices, and a second open dropped by its owner mid-turn is retired.
///
/// 🔌️ `#[ignore]`d: needs the staged native runtime (`SEMIO_PLUGIN_MODULES`/`SEMIO_PLUGIN`, the journey fixture's
/// app answers) and the long-opening component (`SEMIO_LONG_TURN_PLUGIN`, `SEMIO_LONG_TURN_APP`,
/// `SEMIO_LONG_TURN_COMPONENT`, `SEMIO_LONG_TURN_DESCRIPTOR`, `SEMIO_LONG_TURN_SHA`).
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native runtime and a long-opening component; see this test's own doc comment"]
fn a_long_guest_turn_shares_the_kernel_and_only_its_owners_cancel_ends_it() {
    let journey = native_guest_journey();
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let plugins = drive(crate::program_bridge::load_wasm_plugins(&live_env("SEMIO_PLUGIN"), &modules)).expect("the staged native runtime loads");
    let answering_program = plugins.iter().find(|entry| entry.plugin_id == journey.plugin_id).cloned().expect("the staged runtime carries the answering app");
    let answering = drive(answering_program.create_app(&journey.app_id)).expect("the answering app opens");
    let long = drive(crate::program_bridge::load_resolved_program(
        &live_env("SEMIO_LONG_TURN_PLUGIN"),
        std::path::Path::new(&live_env("SEMIO_LONG_TURN_COMPONENT")),
        std::path::Path::new(&live_env("SEMIO_LONG_TURN_DESCRIPTOR")),
        &live_env("SEMIO_LONG_TURN_SHA"),
    ))
    .expect("the long-opening component loads");
    let long_app = live_env("SEMIO_LONG_TURN_APP");
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    let pump = || {
        let _ = crate::pump_renderer_io_sessions(1);
        let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    };

    let started = std::time::Instant::now();
    let mut opening = Box::pin(long.create_app(&long_app));
    let mut answers: Vec<std::time::Duration> = Vec::new();
    let mut answer = None;
    let opened = loop {
        pump();
        if let std::task::Poll::Ready(result) = std::future::Future::poll(opening.as_mut(), &mut context) {
            break result;
        }
        let (asked, reading) = answer.get_or_insert_with(|| (std::time::Instant::now(), Box::pin(answering_program.read_history(answering))));
        if let std::task::Poll::Ready(read) = std::future::Future::poll(reading.as_mut(), &mut context) {
            read.expect("the answering app answers while the long turn is in flight");
            answers.push(asked.elapsed());
            answer = None;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    let open_took = started.elapsed();
    drop(answer);
    let longest_answer = answers.iter().max().copied().unwrap_or_default();
    println!("fair-turns long open took={open_took:?} result={opened:?} answers-while-open={} longest-answer={longest_answer:?}", answers.len());
    let long_instance = opened.expect("the long InstanceOpen completes: no wall-clock limit ends a progressing turn");
    assert!(open_took < std::time::Duration::from_secs(2) || !answers.is_empty(), "another app answered between the long turn's slices ({open_took:?} open, no answer)");

    let cancelled_before = crate::kernel_runtime::CANCELLED_KERNEL_TURNS.load(std::sync::atomic::Ordering::Relaxed);
    let cancelled_open_started = std::time::Instant::now();
    let mut cancelled_open = Box::pin(long.create_app(&long_app));
    let mut finished_before_cancel = false;
    while cancelled_open_started.elapsed() < std::time::Duration::from_millis(800) {
        pump();
        if std::future::Future::poll(cancelled_open.as_mut(), &mut context).is_ready() {
            finished_before_cancel = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    drop(cancelled_open);
    let answered_after_cancel = std::time::Instant::now();
    let mut reading = Box::pin(answering_program.read_history(answering));
    let read = loop {
        pump();
        if let std::task::Poll::Ready(read) = std::future::Future::poll(reading.as_mut(), &mut context) {
            break read;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    let cancelled_after = crate::kernel_runtime::CANCELLED_KERNEL_TURNS.load(std::sync::atomic::Ordering::Relaxed);
    println!("fair-turns cancel finished-before-cancel={finished_before_cancel} cancelled-turns={cancelled_before}->{cancelled_after} next-answer-after={:?} ok={}", answered_after_cancel.elapsed(), read.is_ok());
    read.expect("the kernel keeps answering after a cancelled turn");
    assert!(finished_before_cancel || cancelled_after == cancelled_before + 1, "dropping the open mid-turn retired exactly that turn");
    long.destroy_app(long_instance);
    answering_program.destroy_app(answering);
}

'''
laws = laws.replace(ANCHOR, LAW + ANCHOR)
LAWS.write_text(laws)
print("live fair-turns law applied")
