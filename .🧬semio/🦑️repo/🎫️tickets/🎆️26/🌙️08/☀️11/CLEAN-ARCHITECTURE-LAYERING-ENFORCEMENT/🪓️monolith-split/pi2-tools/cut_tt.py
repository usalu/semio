#!/usr/bin/env python3
"""✂️ Cuts `🧪️tests/🧪️time-travel/🦀️.rs` into two pieces plus a core fragment and re-mounts them in the testing-laws host. usage: cut_tt.py [--apply]"""
import sys, collections, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tsplit import *
import ed
path = PLUG + "/🧪️tests/🧪️time-travel/🦀️.rs"
HOST = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⚖️testing-laws/🧱️unit-1/🦀️.rs"
MOUNT = '#[cfg(test)]\n#[path = "../../../../../🧪️tests/🧪️time-travel/🦀️.rs"]\nmod time_travel_tests;\n'
M = Module.load(path)
its = M.items
tests = [i.idx for i in its if i.has_tests]
use = collections.Counter()
for t in tests:
    for x in M.closure([t]):
        if not its[x].has_tests:
            use[x] += 1
universal = {x for x, c in use.items() if c >= 0.5 * len(tests)}
for x in list(universal):
    universal |= M.closure([x])
universal = {x for x in universal if not its[x].has_tests}
ub = sum(its[x].bytes for x in universal)
pieces = cluster(M, tests, universal, 235000 - ub)
units = {f"P{i+1}": p["tests"] for i, p in enumerate(pieces)}
piece_dirs = {u: sibling(path, f"-part-{i+1}") for i, u in enumerate(units)}
docs = {u: f"Part {i+1} of {len(units)} of the history-edit session laws of the plugin runtime (cut from `🧪️tests/🧪️time-travel`; the shared fixtures stay in the original file)." for i, u in enumerate(units)}
files, core, shared, private = emit(M, units, piece_dirs, "⏪️", docs)
mounts = "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(piece_dirs[u], os.path.dirname(HOST))}"]\nmod time_travel_tests_{i+1};\n' for i, u in enumerate(units))
orig_tests = sorted(re.findall(r"(?:async )?fn ([a-z_0-9]+)", "".join(its[t].body for t in tests if its[t].is_test)))
new_tests = sorted(re.findall(r"(?:async )?fn ([a-z_0-9]+)", "".join(files[piece_dirs[u]] for u in units)))
host_text = read(HOST)
assert host_text.count(MOUNT) == 1
probs = verify(M, files)
print("problems", probs[:5], len(probs))
os.makedirs(SCRATCH, exist_ok=True)
print("parse errors", parse_check(files))
print("pieces", {u: len(t) for u, t in units.items()}, "files", {rel(p)[-50:]: len(t) for p, t in files.items()})
if "--apply" in sys.argv:
    for p, tx in files.items():
        ed.put(p, tx)
    ed.put(HOST, host_text.replace(MOUNT, mounts))
    print("applied")
