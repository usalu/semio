#!/usr/bin/env python3
"""🔍️ EX1: unified diff of the planned edits (no write) for the paths matching the given substrings.
usage: python3 ex1-diff.py --root <tree> --only <section…> -- <substring…>"""
import difflib, importlib.util, os, sys
split = sys.argv.index("--")
sys.argv, wanted = [sys.argv[0]] + sys.argv[1:split] + ["--dry-run"], sys.argv[split + 1:]
spec = importlib.util.spec_from_file_location("ex1", os.path.join(os.path.dirname(os.path.abspath(__file__)), "ex1-example-loaders.py"))
ex1 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ex1)
plan = ex1.Plan()
for name, section in ex1.SECTIONS.items():
    if ex1.ONLY is None or name in ex1.ONLY:
        section(plan)
for path, fns in plan.edits.items():
    if not any(w in path for w in wanted):
        continue
    before = ex1.read(path)
    after = before
    for fn in fns:
        after = fn(after)
    sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), path, path, n=2))
print("PROBLEMS", ex1.problems)
