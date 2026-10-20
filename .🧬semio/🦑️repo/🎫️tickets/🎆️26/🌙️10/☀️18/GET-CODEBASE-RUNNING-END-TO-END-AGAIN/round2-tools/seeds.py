#!/usr/bin/env python3
"""Regenerates seed-extra.txt (files that sim.py must copy into the overlay because a tool reads them only after an earlier refusal): sync-incr edit targets, input-precision hook/law targets, course wrapper files."""
import importlib.util, os, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
rows = []
source = open(C + "/sync-incr-tools/apply.py", encoding="utf8").read().rstrip()
assert source.endswith("\nmain()")
ns = {"__file__": C + "/sync-incr-tools/apply.py", "__name__": "seeds"}
exec(compile(source[:-len("main()")], "apply.py", "exec"), ns)
rows += [relative for relative, _ in ns["edits"]()]
L = ns["LIBRARY"]
rows += [L + "/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"] + [L + "/⚖️laws/🔒️cache-correctness/" + r for r in ("🟦️.ts", "🧪️tests/🔒️rules/🧬️schema/🔣️.json", "🧪️tests/🔒️rules/🔣️.json")]
sys.argv = ["x"]
spec = importlib.util.spec_from_file_location("apply_course", C + "/course-bzl-tools/staged/apply-course.py")
module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
rows += module.COURSE_FILES + module.GRAPH_FILES
import glob, json
for trace in glob.glob(C + "/🗑️generated/round2-apply/trace/extra-*.json"):
    rows += [p for p in json.load(open(trace, encoding="utf8")) if not p.startswith(".🧬semio/") and not p.startswith("/")]
if os.path.isfile(C + "/bzl-nocache-tools/staged/ops.json"):
    rows += sorted({op["manifest"] for op in json.load(open(C + "/bzl-nocache-tools/staged/ops.json", encoding="utf8"))})
open(HERE + "/seed-extra.txt", "w", encoding="utf8").write("\n".join(dict.fromkeys(rows)) + "\n")
print(len(rows), "seed rows")
