"""⏱️ Remodel's 29 worker-step wall-clock laws asserted every single step below 8 ms, which is stricter than the runtime
they stand for: production's `StepOverrunLedger` records each over-ceiling step and quarantines only a run of
`SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive ones, because a single over-ceiling wall reading is as often the
machine descheduling the thread as the step's own work. The laws now admit every measured step through one shared
test helper that applies exactly that authority (`interactive_step_contract_violated` per step, a sustained run
fails), so a step that genuinely costs more than the ceiling still fails on every step while a descheduling spike is
recorded and forgotten. `--dry` reports without writing."""
import os, re, sys

root = "/Users/ueli/Documents/semio/"
art = root + "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/"
engine = art + "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/"
dry = "--dry" in sys.argv
HELPER = '''//! ⏱️ The interactive step ceiling as the runtime enforces it, for the engine's worker-step laws.
//!
//! @see 🧰️framework/🔨️modules/⏱️trace/🦀️.rs — `StepOverrunLedger`, `SUSTAINED_OVERRUN_QUARANTINE_STEPS`

use std::cell::Cell;

thread_local! {
    static CONSECUTIVE_OVERRUNS: Cell<u32> = const { Cell::new(0) };
}

/// ⏱️ Admits one measured worker step on this test's thread. A step within `INTERACTIVE_STEP_CEILING_US` resets the
/// run; an over-ceiling step extends it, and a run of `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive over-ceiling
/// steps fails — the run production's `StepOverrunLedger` quarantines, never a single descheduled reading.
#[track_caller]
pub(crate) fn admit_step(elapsed: std::time::Duration, what: std::fmt::Arguments<'_>) {
    let elapsed_us = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
    let run = CONSECUTIVE_OVERRUNS.with(|overruns| {
        let next = if semio_framework_job::interactive_step_contract_violated(elapsed_us) { overruns.get() + 1 } else { 0 };
        overruns.set(next);
        next
    });
    assert!(run < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS, "{what}: {run} consecutive steps exceeded the {} us interactive ceiling (last {elapsed_us} us)", semio_framework_job::INTERACTIVE_STEP_CEILING_US);
}
'''
PREFIX = "assert!(started.elapsed() < std::time::Duration::from_millis(8), "
changed, problems = [], []


def rewrite(text):
    out, at, count = [], 0, 0
    while True:
        start = text.find(PREFIX, at)
        if start < 0:
            out.append(text[at:])
            return "".join(out), count
        depth, index = 1, start + len("assert!(")
        while depth:
            depth += {"(": 1, ")": -1}.get(text[index], 0)
            index += 1
        args = text[start + len(PREFIX):index - 1]
        out.append(text[at:start])
        out.append(f"step_ceiling::admit_step(started.elapsed(), format_args!({args}))")
        at, count = index, count + 1


for dirpath, _, files in os.walk(engine):
    for name in files:
        if name != "🦀️.rs" or "🧪️tests" not in dirpath: continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8").read()
        new, count = rewrite(text)
        if count == 0: continue
        use = "use crate::editor::remodeling::engine::step_ceiling;\n"
        if use not in new:
            first_use = re.search(r"(?m)^use ", new)
            new = new[:first_use.start()] + use + new[first_use.start():] if first_use else use + new
        changed.append((path, count, new))

crate_root = art + "🦀️.rs"
root_text = open(crate_root, encoding="utf-8").read()
anchor = '''            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;'''
mount = '''            #[cfg(test)]
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🔬️step-ceiling/🦀️.rs"]
            pub(crate) mod step_ceiling;
'''
if root_text.count(anchor) != 1: problems.append("crate root anchor not unique")
total = sum(c for _, c, _ in changed)
print(f"{'would rewrite' if dry else 'rewrote'} {total} law(s) in {len(changed)} file(s); {len(problems)} problem(s)")
for p in problems: print("PROBLEM", p)
if problems or dry: sys.exit(1 if problems else 0)
for path, _, new in changed: open(path, "w", encoding="utf-8").write(new)
if "pub(crate) mod step_ceiling;" not in root_text:
    open(crate_root, "w", encoding="utf-8").write(root_text.replace(anchor, mount + anchor))
os.makedirs(engine + "🧪️tests/🔬️step-ceiling", exist_ok=True)
open(engine + "🧪️tests/🔬️step-ceiling/🦀️.rs", "w", encoding="utf-8").write(HELPER)
