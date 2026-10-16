#!/usr/bin/env python3
"""🔬️ sf3 target 2: stub-layer resolution differential of the four generation3d geometry test supports (frs-tools oracle machinery).

Baseline A = the pre-edit support file as lib root (its own `#[path]` mount of the oracle support resolved inside the sandbox).
Current  B = the edited support file mounted by a synthesized lib root, the oracle-support crate mounted as module `__o` and
`semio_s_artifact_procedural_3d_geometry_oracle_support::` rewritten to `crate::__o::`.
usage: oracle_support.py <O|C|P|H> [cfg,cfg] [--label x] [--mutate rel@@old@@new]
"""
import os, re, shutil, sys, collections

FRS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools"
sys.path.insert(0, FRS)
import frslib, oracle, stubgen

REPO = frslib.ROOT
F = frslib.F
SCR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
W = F + "/🗑️generated/sf3-work"
oracle.W = W
frslib.W = W
read, write = frslib.read, frslib.write
G = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry"
DOM = {"O": "🧪️tests/🧰️oracle-support", "C": "🧪️tests/🧰️construction-support", "P": "⏱️phased-job/🧪️tests/🧰️support", "H": "🧱️mesh-support/🧪️tests/🧰️mesh-harness"}
OID = "semio_s_artifact_procedural_3d_geometry_oracle_support"


def sandbox_ok(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), rp


def put(root, rel, text):
    p = os.path.join(root, rel)
    sandbox_ok(p)
    os.makedirs(os.path.dirname(p), exist_ok=True)
    write(p, text)


def build(key, which):
    root = f"{W}/support-{key}/{which}"
    sandbox_ok(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    os.makedirs(root)
    pre = lambda d: read(f"{SCR}/sf3pre3d/{d}/🦀️.rs")
    live = lambda d: read(f"{REPO}/{G}/{d}/🦀️.rs")
    if which == "A":
        put(root, f"{G}/{DOM['O']}/🦀️.rs", pre(DOM["O"]))
        if key != "O":
            put(root, f"{G}/{DOM[key]}/🦀️.rs", pre(DOM[key]))
        return root, f"{G}/{DOM[key]}/🦀️.rs"
    put(root, f"{G}/{DOM['O']}/🦀️.rs", live(DOM["O"]))
    if key == "O":
        return root, f"{G}/{DOM['O']}/🦀️.rs"
    text = live(DOM[key])
    text = re.sub(r"(?<![A-Za-z0-9_])" + OID + r"(?=\s*(::|as\b))", "crate::__o", text)
    put(root, f"{G}/{DOM[key]}/🦀️.rs", text)
    lib = "lib.rs"
    put(root, lib, f'#![allow(dead_code)]\n#[path = "{G}/{DOM[key]}/🦀️.rs"]\nmod component;\npub use component::*;\n#[path = "{G}/{DOM["O"]}/🦀️.rs"]\npub mod __o;\n')
    return root, lib


def ext_crates(root):
    names = set()
    rx = re.compile(r"(?<![A-Za-z0-9_\.:])(semio_[a-z0-9_]+|pack|store|protocol|serde_json|serde)(?=\s*::|\s+as\b)")
    for p in stubgen.rs_files(root):
        names |= set(rx.findall(stubgen.mask(read(p))))
    return sorted(names)


def main():
    key = sys.argv[1]
    cfgs = sys.argv[2].split(",") if len(sys.argv) > 2 and not sys.argv[2].startswith("--") else ["lib", "test"]
    label = sys.argv[sys.argv.index("--label") + 1] if "--label" in sys.argv else "run"
    rootA, libA = build(key, "A")
    rootB, libB = build(key, "B")
    for r in (rootA, rootB):
        oracle.patch_tree(r, "support_crate")
        for p in stubgen.rs_files(r):
            t = read(p)
            t2 = re.sub(r"#\[\s*dsl\s*\([^\]\n]*\)\s*\]", "", t)
            if t2 != t:
                write(p, t2)
    crates = sorted(set(ext_crates(rootA)) | set(ext_crates(rootB)))
    stubgen.HINT_TRAITS |= set(os.environ.get("SF3_TRAITS", "").split(",")) - {""}
    stubgen.apply([rootA], libA, crates, "")
    stubgen.apply([rootB], libB, crates, "")
    for i, a in enumerate(sys.argv):
        if a == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(rootB, rel)
            sandbox_ok(fp)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print("   mutated", rel)
    bad = 0
    for c in cfgs:
        extra = oracle.CFG[c]
        eA = oracle.run_rustc(rootA, libA, "support_crate", extra).replace(rootA, "<S>")
        eB = oracle.run_rustc(rootB, libB, "support_crate", extra).replace(rootB, "<S>")
        write(f"{F}/🗑️generated/sf3-support-{key}-{label}-{c}-A.err", eA)
        write(f"{F}/🗑️generated/sf3-support-{key}-{label}-{c}-B.err", eB)
        ka, kb = oracle.parse(eA), oracle.parse(eB)
        ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
        onlyA, onlyB = ca - cb, cb - ca
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        bad += status != "IDENTICAL"
        print(f"== {key} {c}: baseline errors={len(ka)} current errors={len(kb)} {status}")
        for (h, m, l), n in sorted(onlyB.items())[:60]:
            print(f"   +current {n}x {h}: {m[:170]} | {l[:90]}")
        for (h, m, l), n in sorted(onlyA.items())[:60]:
            print(f"   -current {n}x {h}: {m[:170]} | {l[:90]}")
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


main()
