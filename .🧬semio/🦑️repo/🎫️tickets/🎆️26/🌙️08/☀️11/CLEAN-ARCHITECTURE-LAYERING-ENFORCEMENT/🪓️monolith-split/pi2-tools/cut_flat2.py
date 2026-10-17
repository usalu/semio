#!/usr/bin/env python3
"""✂️ Re-cuts the time-travel and tool-run test files into pieces plus support modules; every piece gets its own host unit file (header copy + one mount) so it is a test root of its own, the support module is mounted by the hub (`crate::<support>`).
usage: cut_flat2.py tool-run|time-travel [--apply]"""
import sys, collections, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tsplit
from tsplit import *
import ed
fam = sys.argv[1]
APPLY = "--apply" in sys.argv
H3 = PLUG + "/🪝️mounts/👥️wp3-owners"
H7 = PLUG + "/🪝️mounts/🧵️wp7-instance-tier2"
MOUNT_RX = re.compile(r'(?m)^#\[cfg\(test\)\]\n#\[path = "([^"]+)"\]\n((?:pub(?:\([a-z]+\))? )?mod ([A-Za-z0-9_]+);)\n')
LIMIT_BYTES, LIMIT_LINES, LIMIT_FILES = 235000, 3700, 28
if fam == "tool-run":
    ORIG = PLUG + "/🧪️tests/🔬️tool-run/🦀️.rs"
    HUB = H3 + "/🦀️.rs"
    UNITDIR = H3 + "/🧪️test-hosts/👷️job"
    HUBMOD = "job_tool_run_host_"
    BASE, EMOJI, PHRASE = "tool_run_tests", "⏯️", "runtime laws of the tool run ledger and driver"
    SUPMOD = "tool_run_support"
    HDR_FROM = UNITDIR + "/🧱️unit-3/🦀️.rs"
    UNIT_NUM = [3, 5, 6, 7, 8]
else:
    ORIG = PLUG + "/🧪️tests/🧪️time-travel/🦀️.rs"
    HUB = H7 + "/🦀️.rs"
    UNITDIR = H7 + "/🧪️test-hosts/⚖️testing-laws"
    HUBMOD = "testing_laws_unit_"
    BASE, EMOJI, PHRASE = "time_travel_tests", "⏪️", "history-edit session laws of the plugin runtime"
    SUPMOD = "time_travel_support"
    HDR_FROM = UNITDIR + "/🧱️unit-1/🦀️.rs"
    UNIT_NUM = None


def restore_family():
    pre = [rel(os.path.dirname(ORIG))]
    key = "🧪️tests/" + os.path.basename(os.path.dirname(ORIG))
    P = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"
    ed.restore([P + key], remove_created=lambda k: ("-part-" in k or "-core-" in k or "-support-" in k or k.endswith("-x/🦀️.rs")) and key in k)


M = None


def attempt(M, budget, supp):
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
    pieces = cluster(M, tests, universal, budget, max_files=14)
    units = {f"P{i+1}": p["tests"] for i, p in enumerate(pieces)}
    dirs = {u: sibling(ORIG, f"-part-{i+1}") for i, u in enumerate(units)}
    docs = {u: f"Part {i+1} of {len(units)} of the {PHRASE} (cut from `{os.path.basename(os.path.dirname(ORIG))}`; the fixtures several parts use are the support module)." for i, u in enumerate(units)}
    macros = [it.idx for it in its if it.kind == "macro_rules!"]
    files, spaths, smods, groups, private = emit3(M, units, dirs, EMOJI, docs, None, supp, always_core=macros, support_prefix="support", ref="crate")
    sizes = {}
    sup = set(spaths) | {sibling(p, "-x") for p in spaths}
    for u in units:
        n, b, l, texts = closure_size(dirs[u], files)
        own = {f: t for f, t in texts.items() if f not in sup}
        sizes[u] = (len(own), sum(len(t.encode()) for t in own.values()), sum(sum(1 for z in blank(t).split("\n") if z.strip()) for f, t in own.items() if f.endswith(".rs")))
    ssz = [closure_size(p, files)[:3] for p in spaths]
    return units, dirs, files, sizes, ssz, spaths, smods


restore_family()
M = Module.load(ORIG)
tsplit.MERGE_BELOW = 3000
res = None
for budget in range(260000, 60000, -10000):
    out = attempt(M, budget, 215000)
    units, dirs, files, sizes, ssz, spaths, smods = out
    ok = all(s[1] <= LIMIT_BYTES and s[2] <= LIMIT_LINES and s[0] <= LIMIT_FILES for s in sizes.values()) and all(s[1] <= 250000 for s in ssz)
    print("budget", budget, "pieces", len(units), [s[1] for s in sizes.values()], "support", [s[1] for s in ssz], "OK" if ok else "")
    if ok:
        res = out
        break
units, dirs, files, sizes, ssz, spaths, smods = res
probs = verify(M, files)
os.makedirs(SCRATCH, exist_ok=True)
perr = parse_check(files)
print("problems", probs[:5], len(probs), "parse errors", perr)
# host units
hdr_text = read(HDR_FROM)
ms = list(MOUNT_RX.finditer(hdr_text))
header = hdr_text[:ms[0].start()]
names = list(units)
unit_edits, unit_files = {}, {}
if fam == "tool-run":
    nums = UNIT_NUM[:len(names)]
    for i, u in enumerate(names):
        p = UNITDIR + f"/🧱️unit-{nums[i]}/🦀️.rs"
        rp = os.path.relpath(dirs[u], os.path.dirname(p))
        unit_files[p] = header.replace("Unit 3 of the facade test host of `job`", f"Unit {nums[i]} of the facade test host of `job`") + f'#[cfg(test)]\n#[path = "{rp}"]\nmod {BASE}_{i+1};\n'
    hub = read(HUB)
    hub = re.sub(r'#\[cfg\(test\)\]\n#\[path = "🧪️test-hosts/👷️job/🧱️unit-\d+/🦀️\.rs"\]\npub\(crate\) mod job_tool_run_host_\d+;\n', "", hub)
    anchor = '#[cfg(test)]\n#[path = "🧪️test-hosts/👷️job/🦀️.rs"]\npub(crate) mod job_test_hosts;\n'
    assert hub.count(anchor) == 1
    add = "".join(f'#[cfg(test)]\n#[path = "🧪️test-hosts/👷️job/🧱️unit-{nums[i]}/🦀️.rs"]\npub(crate) mod {HUBMOD}{i+1};\n' for i in range(len(names)))
    add += "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(p, os.path.dirname(HUB))}"]\npub(crate) mod {m};\n' for p, m in zip(spaths, smods))
    unit_edits[HUB] = hub.replace(anchor, anchor + add)
else:
    # regenerate the testing-laws units from the current ones
    allm, hdr = {}, None
    for i in range(1, 9):
        pth = UNITDIR + f"/🧱️unit-{i}/🦀️.rs"
        if os.path.exists(pth):
            tx = read(pth)
            mm = list(MOUNT_RX.finditer(tx))
            if hdr is None:
                hdr = tx[:mm[0].start()]
            for m in mm:
                allm[m.group(3)] = m.group(0)
    for k in [k for k in allm if k.startswith("time_travel_tests")]:
        del allm[k]
    others = sorted(allm)
    groups = [["mutation_fixture"], [k for k in others if k != "mutation_fixture"]]
    layout = []
    for g in groups:
        layout.append(("".join(allm[k] for k in g), g))
    for i, u in enumerate(names):
        layout.append((None, [u]))
    hub = read(HUB)
    hub = re.sub(r'#\[cfg\(test\)\]\n#\[path = "🧪️test-hosts/⚖️testing-laws/🧱️unit-\d+/🦀️\.rs"\]\npub\(crate\) mod testing_laws_unit_\d+;\n', "", hub)
    anchor = '#[cfg(test)]\n#[path = "🧪️test-hosts/🎟️testing-fixtures/🦀️.rs"]\npub(crate) mod testing_fixtures_test_hosts;\n'
    assert hub.count(anchor) == 1
    add = ""
    for n, (mtxt, g) in enumerate(layout, 1):
        p = UNITDIR + f"/🧱️unit-{n}/🦀️.rs"
        if mtxt is None:
            u = g[0]
            idx = names.index(u) + 1
            rp = os.path.relpath(dirs[u], os.path.dirname(p))
            mtxt = f'#[cfg(test)]\n#[path = "{rp}"]\nmod {BASE}_{idx};\n'
        unit_files[p] = hdr.replace("Unit 1 of the facade test host", f"Unit {n} of the facade test host") + mtxt
        add += f'#[cfg(test)]\n#[path = "🧪️test-hosts/⚖️testing-laws/🧱️unit-{n}/🦀️.rs"]\npub(crate) mod {HUBMOD}{n};\n'
    add += "".join(f'#[cfg(test)]\n#[path = "{os.path.relpath(p, os.path.dirname(HUB))}"]\npub(crate) mod {m};\n' for p, m in zip(spaths, smods))
    unit_edits[HUB] = hub.replace(anchor, anchor + add)
print("units", {rel(p)[-34:]: len(t) for p, t in unit_files.items()})
if APPLY and not probs and not perr:
    for p, tx in files.items():
        ed.put(p, tx)
    for p, tx in unit_files.items():
        ed.put(p, tx)
    for p, tx in unit_edits.items():
        ed.put(p, tx)
    # stale unit files beyond the new count
    print("applied", len(files), "files")
