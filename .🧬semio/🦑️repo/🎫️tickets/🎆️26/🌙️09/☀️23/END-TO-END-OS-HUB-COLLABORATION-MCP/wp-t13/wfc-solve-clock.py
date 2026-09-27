"""🧮️ wfc solve-law clock codemod: job `logical_now_us`, `solve_with_clock` in the five wfc inference
modules, and every correctness law under `🧪️tests` switched to the logical clock (timing laws keep the
real clock). Usage: python3 wfc-solve-clock.py [--write]"""
import pathlib, re, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
WFC = ROOT / "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts"
JOB = ROOT / "🧰️framework/🔨️modules/🧵️job/🦀️.rs"
WRITE = "--write" in sys.argv
TIMING_LAWS = []
problems, changed = [], {}
GLOB_IMPORTED_FILL_TESTS = ["🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🧪️tests", "🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🧪️tests"]

job = JOB.read_text()
anchor = "pub fn default_now_us() -> Option<u64> {\n    semio_framework_trace::try_now_us()\n}\n"
logical = anchor + """
/// 🧮️ Deterministic per-thread clock that advances one microsecond per read: correctness laws drive
/// jobs with it so a descheduled test thread never trips the wall-clock overrun quarantine, while
/// timing laws and production keep [`default_now_us`].
pub fn logical_now_us() -> Option<u64> {
    thread_local! { static LOGICAL_NOW_US: std::cell::Cell<u64> = const { std::cell::Cell::new(0) }; }
    Some(LOGICAL_NOW_US.with(|now| { now.set(now.get() + 1); now.get() }))
}
"""
if "fn logical_now_us" not in job:
    if job.count(anchor) != 1: problems.append("job anchor")
    else: changed[JOB] = job.replace(anchor, logical)

sig = re.compile(r"(?P<vis>pub(?:\(crate\))?) fn solve_with_job\(snapshot: &(?P<ty>\w+)\) -> (?P<ret>Result<\w+, String>) \{\n")
for module in sorted(WFC.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs")):
    text = module.read_text()
    if "fn solve_with_clock" in text: continue
    found = list(sig.finditer(text))
    if len(found) != 1: problems.append(f"{module}: {len(found)} signatures"); continue
    m = found[0]
    body_start = m.end()
    body_end = text.index("\n}\n", body_start)
    body = text[body_start:body_end]
    if body.count("now_us: semio_framework_job::default_now_us,") != 1: problems.append(f"{module}: clock field"); continue
    body = body.replace("now_us: semio_framework_job::default_now_us,", "now_us,")
    head = (f"{m['vis']} fn solve_with_job(snapshot: &{m['ty']}) -> {m['ret']} {{\n"
            f"    solve_with_clock(snapshot, semio_framework_job::default_now_us)\n}}\n\n"
            f"/// 🧮️ The same headless adapter driven by an injected clock, so correctness laws run on\n"
            f"/// [`semio_framework_job::logical_now_us`] and never on a descheduled thread's wall clock.\n"
            f"{m['vis']} fn solve_with_clock(snapshot: &{m['ty']}, now_us: fn() -> Option<u64>) -> {m['ret']} {{\n")
    changed[module] = text[:m.start()] + head + body + text[body_end:]

call = re.compile(r"\bsolve_with_job\(")
def rewrite_calls(text, path):
    out, i = [], 0
    for m in call.finditer(text):
        line_start = text.rfind("\n", 0, m.start()) + 1
        line_end = text.find("\n", m.start())
        line = text[line_start:line_end]
        if line.lstrip().startswith("use ") or any(t in line for t in TIMING_LAWS): continue
        depth, j = 1, m.end()
        while depth:
            depth += {"(": 1, ")": -1}.get(text[j], 0); j += 1
        args = text[m.end():j - 1]
        out.append(text[i:m.start()] + f"solve_with_clock({args}, semio_framework_job::logical_now_us)")
        i = j
    out.append(text[i:])
    return "".join(out)

for test in sorted(WFC.rglob("🦀️.rs")):
    if "🧪️tests" not in str(test): continue
    text = test.read_text()
    if not call.search(text): continue
    new = rewrite_calls(text, test)
    if "solve_with_job(" in re.sub(r"\w+solve_with_job\(", "", new):
        keeps = [l for l in new.splitlines() if re.search(r"\bsolve_with_job\(", l) and not l.lstrip().startswith("use ")]
        if not keeps: new = re.sub(r"(use [^;]*)\bsolve_with_job\b", r"\1solve_with_clock", new)
        elif any(not any(t in l for t in TIMING_LAWS) for l in keeps): problems.append(f"{test}: leftover {keeps}")
        else:
            new = re.sub(r"(use [^;]*)\bsolve_with_job\b([^;]*;)", r"\1solve_with_job, solve_with_clock\2", new) if re.search(r"use [^;]*\bsolve_with_job\b", new) else new
    else:
        new = re.sub(r"(use [^;]*)\bsolve_with_job\b", r"\1solve_with_clock", new)
    if any(part in str(test) for part in GLOB_IMPORTED_FILL_TESTS) and "use crate::schema::inferences::solve_with_clock;" not in new:
        new = new.replace("use super::*;\n", "use super::*;\nuse crate::schema::inferences::solve_with_clock;\n", 1)
    if new != text: changed[test] = new

for path, new in changed.items():
    print(("WRITE " if WRITE else "DRY ") + str(path.relative_to(ROOT)))
    if WRITE: path.write_text(new)
print(f"{len(changed)} files, {len(problems)} problems")
for p in problems: print("PROBLEM", p)
