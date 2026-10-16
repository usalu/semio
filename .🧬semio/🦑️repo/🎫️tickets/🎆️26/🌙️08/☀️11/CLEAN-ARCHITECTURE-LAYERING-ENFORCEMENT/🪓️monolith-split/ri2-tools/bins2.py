#!/usr/bin/env python3
# 🧪️ Final placement of the Shell tests: singletons that resolve against the crates live in the bins; the tests that share helper modules with the monolith-only fixtures stay mounted in the Shell tree.
import json, os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/w1")
sys.path.insert(0, HERE)
import bins as B
import flipfc as X
import flip as F
from flip import *

SN = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/ri2/snap0"
ALL = json.load(open(HERE + "/bins-tests-all.json"))
COMPS = json.load(open(HERE + "/bins-components.json"))
STAY_COMP = {d for c in COMPS if len(c) > 1 for d in c}
BIN_OF = {}
for b, dirs in B.BINS.items():
    for d in dirs:
        BIN_OF[d] = b


def orig_text(path):
    return open(SN + "/" + os.path.relpath(path, E), encoding="utf8").read()


def host_uses(host):
    """names the chunk module the test was mounted in imports on its own (visible to the test through `use super::*`)."""
    res = collections.defaultdict(list)
    for cfg, vis, p, local, glob in uses(read(host), top_only=True):
        if glob or not local or not p or p[0] in ("super", "self"):
            continue
        try:
            if p[0] == "crate":
                p = X.map_root_path(p)
                if p is None:
                    continue
        except KeyError:
            continue
        if p[0] in X.P.MODULE_TO_PATH and p[0] not in ("shell",):
            p = X.map_root_path(p) or p
        res[local].append((cfg, p))
    return res


def placed(t):
    """True when the test is currently mounted by a bin root."""
    for b in B.BINS:
        for m in B.existing_mounts(b):
            if m["target"] == t["target"]:
                return b
    return None


def restore(t, b):
    suffix = B.bin_crate(b)
    # test file: original text
    write(t["target"], orig_text(t["target"]))
    # mount block back in its host, behind the line that preceded it in the snapshot
    host = t["host"]
    snap_host = orig_text(host)
    m = re.search(r"((?:#\[cfg[^\n]*\]\n)*)#\[path\s*=\s*\"[^\"\n]*" + re.escape(t["dir"]) + r"[^\"\n]*\"\]\n(?:pub(?:\([a-z]+\))? )?mod " + re.escape(t["name"]) + r";\n", snap_host)
    assert m, t["name"]
    block = m.group(0)
    prev = snap_host[:m.start()].rstrip("\n").split("\n")[-1]
    live = read(host)
    if live.count(block) == 0:
        if live.count(prev + "\n") == 1 and prev.strip():
            live = live.replace(prev + "\n", prev + "\n" + block, 1)
        else:
            live = live.rstrip("\n") + "\n\n" + block
        F.backup(suffix, [host])
        write(host, live)


def main(apply=False):
    stay, move, restore_l = [], [], []
    for t in ALL:
        if t["name"] == "section_fixtures":
            continue
        b = BIN_OF.get(t["dir"])
        cur = placed(t)
        want = None
        if b is None or t["dir"] in STAY_COMP:
            want = "stay"
        else:
            # transformable?
            try:
                X.CURRENT[0] = B.bin_crate(b)
                B.transform_test_probe(orig_text(t["target"]))
                want = "move"
            except KeyError as e:
                want = "stay"
                t["reason"] = str(e)
        t["want"], t["cur"], t["bin"] = want, cur, b
    return ALL


def apply_all():
    all_ = main()
    # 1. restore the tests that must stay
    for t in all_:
        if t["name"] == "section_fixtures":
            continue
        if t["want"] == "stay" and t["cur"]:
            restore(t, t["cur"])
            print("restored", t["dir"], "from", t["cur"])
    # 2. rebuild the bin roots
    for b in B.BINS:
        mounts = []
        for t in all_:
            if t["name"] == "section_fixtures" or t["want"] != "move" or t["bin"] != b:
                continue
            snap_host = orig_text(t["host"])
            m = re.search(r"((?:#\[cfg[^\n]*\]\n)*)#\[path\s*=\s*\"[^\"\n]*" + re.escape(t["dir"]) + r"[^\"\n]*\"\]\n((?:pub(?:\([a-z]+\))? )?)mod " + re.escape(t["name"]) + r";\n", snap_host)
            mounts.append(dict(host=t["host"], cfg=m.group(1), target=t["target"], name=t["name"], vis=m.group(2), text=m.group(0), dirname=t["dir"]))
        if not mounts:
            continue
        B.EXTRA_P1.clear()
        for m in mounts:
            for k, v in host_uses(m["host"]).items():
                B.EXTRA_P1.setdefault(k, [])
                for e in v:
                    if e not in B.EXTRA_P1[k]:
                        B.EXTRA_P1[k].append(e)
        suffix = B.bin_crate(b)
        X.CURRENT[0] = suffix
        folder = X.TABLE[suffix]
        F.backup(suffix, [m["target"] for m in mounts] + [m["host"] for m in mounts] + [T + "/" + folder + "/🦀️.rs", T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml"])
        for m in mounts:
            cur = read(m["target"])
            nt = B.transform_test_probe(cur)
            if nt != cur:
                write(m["target"], nt)
        rt = B.root_text(b, mounts)
        write(T + "/" + folder + "/🦀️.rs", rt)
        for m in mounts:
            h = read(m["host"])
            if h.count(m["text"]) == 1:
                write(m["host"], h.replace(m["text"], ""))
        used = first_party_used([m["target"] for m in mounts]) | {x.replace("_", "-") for x in re.findall(PFX + r"([a-z0-9_]+)", rt)}
        print(b, len(mounts), ensure_deps(suffix, used, rt))
    # bins left without tests: back to the skeleton root
    for b in B.BINS:
        if not any(t.get("want") == "move" and t.get("bin") == b and t["name"] != "section_fixtures" for t in all_):
            suffix = B.bin_crate(b)
            root = T + "/" + X.TABLE[suffix] + "/🦀️.rs"
            if "#![cfg(test)]" in read(root):
                head = [l for l in read(root).split("\n") if l.startswith("//!")][:1]
                write(root, "\n".join(head + ["//!", "//! 🧭️ No Shell test moved into this bin: its tests share helper modules with the monolith-only fixtures and stay mounted in the Shell tree (RESULT-RENDERER-INTEGRATION-2 section on the test bins).", ""]))
                print("emptied", b)
    return all_


if __name__ == "__main__":
    apply_all()
