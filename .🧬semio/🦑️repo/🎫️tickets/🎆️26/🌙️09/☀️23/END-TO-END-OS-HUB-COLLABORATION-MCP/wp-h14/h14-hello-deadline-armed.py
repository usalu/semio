#!/usr/bin/env python3
"""⏳ H14 follow-up to h14-hello-tail-reader.py: the stall-bounded hello deadline re-arms only when a frame RENEWED it after the
callback was armed (`deadline_armed_ms`); a callback that finds no renewal expires the hello exactly as before (the law
`retained_sync_hello_deadline_retry_drop_close_retains_registry_until_worker_service` drives the callback directly and requires
the expiry). Idempotent; `--dry-run` reports only."""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"
DRY = "--dry-run" in sys.argv
text = open(PATH, encoding="utf-8").read()
problems, states = [], []


def edit(old, new, label):
    global text
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    text = text.replace(old, new)
    states.append("replace")


edit("""    progress_deadline_ms: std::sync::atomic::AtomicU64,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,""", """    progress_deadline_ms: std::sync::atomic::AtomicU64,
    deadline_armed_ms: std::sync::atomic::AtomicU64,
    progress: std::sync::Arc<std::sync::atomic::AtomicU8>,""", "armed field")

edit("""            progress_deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,""", """            progress_deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            deadline_armed_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,""", "armed init")

edit("""        let renewed = self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if renewed > self.pool.now_ms() {
            let deadline = std::sync::Arc::downgrade(self);""", """        let renewed = self.progress_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if renewed > self.deadline_armed_ms.swap(renewed, std::sync::atomic::Ordering::AcqRel) {
            let deadline = std::sync::Arc::downgrade(self);""", "re-arm only on renewal")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    open(PATH, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
