#!/usr/bin/env python3
"""🏁️ H13 session 14c: a runner's terminal transition wakes its committed retirement cursor (kernel-db, hub-closure-only).

Root cause of `database shutdown deadline elapsed in phase PoolUse … artifact_retirement: 1` (hold 6 sqlite run 3): the
retirement hook's turn found the runner still closing (a normal turn was polling it) and went idle; that normal turn then
completed the close inside `ArtifactRunner::finish`, whose terminal transition published `Terminal` without requesting the
retirement hook, and the turn guard returns on `Terminal` without a wake. The cursor kept its `WorkerPoolUse` forever and the
shutdown spun in `PoolUse` until its deadline. The transition now lives in `ArtifactRunnerHandoff::enter_terminal`, which
requests the retirement hook after publishing `Terminal`. Law:
`a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement`. Idempotent; `--dry-run` writes nothing.
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ARTIFACT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"
TESTS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"

LAW = r'''#[semio_framework_async_macros::async_test]
async fn a_runner_that_turns_terminal_after_its_retirement_went_idle_wakes_the_retirement() {
    use std::sync::atomic::Ordering;
    let pool = Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 2)));
    let pool_use = pool.acquire_use().unwrap();
    let handoff = Arc::new(ArtifactRunnerHandoff {
        pool: pool.clone(),
        pool_use: std::sync::Mutex::new(None),
        terminal_job: std::sync::Mutex::new(None),
        close_runner: std::sync::Mutex::new(None),
        retirement_maintenance: std::sync::Mutex::new(None),
        close_error: std::sync::Mutex::new(None),
        close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
        close_faults: std::sync::atomic::AtomicUsize::new(0),
        close_polls: std::sync::atomic::AtomicUsize::new(0),
        retirement_turns: std::sync::atomic::AtomicUsize::new(0),
        active_history: std::sync::atomic::AtomicBool::new(false),
        driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::Polling as u8),
        terminal: std::sync::atomic::AtomicBool::new(false),
    });
    let reservation = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).unwrap();
    let (index, generation) = (reservation.index, reservation.generation);
    let observed = Arc::downgrade(&handoff);
    reservation.commit(Arc::new(move || observed.upgrade().is_some_and(|handoff| handoff.terminal.load(Ordering::Acquire))), handoff.clone(), pool_use.clone());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while handoff.retirement_turns.load(Ordering::Acquire) == 0 || ARTIFACT_RUNNER_RETIREMENTS[index].lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
        assert!(std::time::Instant::now() < deadline, "the committed retirement never took its first turn");
        semio_framework_async::yield_once().await;
    }
    assert_eq!(ARTIFACT_RUNNER_RETIREMENT_GENERATIONS[index].load(Ordering::Acquire), generation, "a runner still polling keeps its retirement cursor");
    handoff.enter_terminal();
    await_retirement_slot_release(&handoff, index, generation, deadline).await;
    assert_eq!(Arc::strong_count(&pool_use), 1, "the retired cursor released its WorkerPoolUse");
    drop(pool_use);
    assert_eq!(pool.shutdown(), Ok(()));
}

'''

EDITS = [
    (
        ARTIFACT,
        "            if self.handoff.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {\n"
        "                ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);\n"
        "            }\n"
        "            self.handoff.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);\n"
        "            self.handoff.terminal.store(true, std::sync::atomic::Ordering::Release);\n"
        "            if let Some(done) = self.done.lock()",
        "            self.handoff.enter_terminal();\n"
        "            if let Some(done) = self.done.lock()",
    ),
    (
        ARTIFACT,
        "    fn request_retirement_maintenance(&self) {\n",
        "    /// 🏁️ The runner's terminal transition: surrenders the handoff's pool use, publishes `Terminal`, then wakes a committed\n"
        "    /// retirement cursor. Its last turn may have found the runner still closing and gone idle, and a terminal runner makes\n"
        "    /// no further progress that would wake it — a stranded cursor keeps its `WorkerPoolUse` and a database shutdown waits in\n"
        "    /// its `PoolUse` phase until the deadline.\n"
        "    fn enter_terminal(&self) {\n"
        "        if self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {\n"
        "            ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);\n"
        "        }\n"
        "        self.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);\n"
        "        self.terminal.store(true, std::sync::atomic::Ordering::Release);\n"
        "        self.request_retirement_maintenance();\n"
        "    }\n\n"
        "    fn request_retirement_maintenance(&self) {\n",
    ),
    (
        TESTS,
        "#[semio_framework_async_macros::async_test]\nasync fn artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal() {",
        LAW + "#[semio_framework_async_macros::async_test]\nasync fn artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal() {",
    ),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {ARTIFACT, TESTS}}
    pending = problems = 0
    for path, old, new in EDITS:
        text = texts[path]
        if new in text:
            print(f"applied already: {path.parent.name}/{path.name} :: {old.strip()[:70]}")
            continue
        count = text.count(old)
        if count != 1:
            print(f"PROBLEM ({count} matches): {path.parent.name}/{path.name} :: {old.strip()[:70]}")
            problems += 1
            continue
        pending += 1
        print(f"{'would apply' if dry else 'apply'}: {path.parent.name}/{path.name} :: {old.strip()[:70]}")
        texts[path] = text.replace(old, new, 1)
    if problems:
        print(f"{problems} problems — nothing written")
        return 1
    if not dry:
        for path, text in texts.items():
            if text != path.read_text(encoding="utf-8"):
                path.write_text(text, encoding="utf-8")
    print(f"{'dry run: ' if dry else ''}{pending} pending")
    return 0


if __name__ == "__main__":
    sys.exit(main())
