#!/usr/bin/env python3
"""Exact recovery of the en1995 strength-class fixture values that the v2 repair guessed ("most common pattern"). The fixtures were produced by `en1995-collections/{kinds,nested}.py`: members and connections come from the committed base snapshot
(`change-member-bridge-crowd/apply/before`: GL28h), panels from the entity spec default `strength_class = "C24"` (`spec.py`), and the apply scenario of `change-panel-strength-class` changes it by the generator rule `changed()`
(next member of the option list: C24 -> C27). Only files without a committed version are touched. Paths decide, never the position pattern. The six check-outcome documents were verified against the independent oracle (see the v3 report).
Applied once at 03:20 (before the PATH-BUDGET flattening); it refuses to run on the flat layout.
Usage: python3 -I en1995_strength_recover.py [--dry]"""
import json, os, re, subprocess, sys
REPO = "/home/deck/semio"
ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures"
DRY = "--dry" in sys.argv
RX = re.compile(r'("strengthClass":\s*")((?:[^"\\]|\\.)*)(")')
NV = re.compile(r'("ChangePanelStrengthClass":\s*\{[^}]*"newValue":\s*")([^"]*)(")')
CHANGED = "c27"
def committed(p):
    rel = os.path.relpath(p, REPO)
    return any(subprocess.run(["git", "-C", REPO, "cat-file", "-e", spec + rel], capture_output=True).returncode == 0 for spec in ("HEAD:", ":"))


def walk(o, p=""):
    if isinstance(o, dict):
        for k, v in o.items():
            if k == "strengthClass" and isinstance(v, str):
                yield p + "." + k
            else:
                yield from walk(v, p + "." + k)
    elif isinstance(o, list):
        for v in o:
            yield from walk(v, p + "[]")
def want(path, change_scenario, is_after_or_diff):
    if path.startswith((".members[]", ".connections[]")):
        return "gl28h"
    if path.startswith((".panels[]", ".InsertPanel.panel", ".panels.inserted[].row")):
        return CHANGED if change_scenario and is_after_or_diff else "c24"
    if path == ".panels.modified[].patch.strengthClass":
        return CHANGED
    raise SystemExit("unmapped path " + path)
if any(f.endswith(".before.json") for _, _, fs in os.walk(ROOT + "/🧬️mutations") for f in fs):
    raise SystemExit("one-shot, already applied: the fixture tree is flattened (PATH-BUDGET), so no file has a committed path any more and the committed-version guard cannot protect the hand-made fixtures")
changed, verdict = 0, []
for base, dirs, files in os.walk(ROOT + "/🧬️mutations"):
    for f in files:
        p = os.path.join(base, f)
        t = open(p, encoding="utf-8", newline="").read()
        if "strengthClass" not in t and "ChangePanelStrengthClass" not in t:
            continue
        if committed(p):
            continue
        rel = p[len(ROOT) + 1:]
        scenario = "change-panel-strength-class" in rel
        after = scenario and any(x in rel for x in ("➡️after", "🔺️diff", "🦠️mutation", ".after.json", ".diff.json", ".mutation.json"))
        paths = list(walk(json.loads(t)))
        ms = list(RX.finditer(t))
        if len(paths) != len(ms):
            raise SystemExit("count mismatch " + rel)
        out, pos = [], 0
        for m, path in zip(ms, paths):
            v = want(path, scenario, after)
            out.append(t[pos:m.start(2)] + v)
            pos = m.end(2)
        n = "".join(out) + t[pos:]
        if scenario and ("🦠️mutation" in rel or ".mutation.json" in rel):
            n = NV.sub(lambda m: m.group(1) + CHANGED + m.group(3), n)
        if n != t:
            changed += 1
            if not DRY:
                open(p, "w", encoding="utf-8", newline="").write(n)
print(("dry " if DRY else "") + f"rewritten {changed} fixture files")
