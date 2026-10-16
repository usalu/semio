#!/usr/bin/env python3
"""🧱️ Group G1 of PLUGIN-INTEGRATION-1: child, composition-emission, composition-publication, publication-mounted (the include tangle of the child module)."""
import sys, os, json
from lib import *
import flipcore as F
import world as Wd

GROUP = ["child", "composition-emission", "composition-publication", "publication-mounted"]
ROOTS_B = {"child": PLUG + "/🪆️child/🦀️.rs", "composition-publication": PLUG + "/🧩️composition/📬️publication/🦀️.rs"}
PROMOTE = {PLUG + "/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs": "command_mounted"}
SKIP_FILES = {
    PLUG + "/🧩️composition/📨️emission/📦️preparation/🎟️receipt/🧪️tests/🦀️.rs",
    PLUG + "/🧩️composition/📨️emission/📦️preparation/🦀️.rs",
}
SKIP_MOUNTS = {"original_tests", "child_emit_preparation"}


def plan():
    pl = F.Plan(GROUP, ROOTS_B, promote=PROMOTE)
    pl.build(skip_files=SKIP_FILES)
    out = pl.finalize(skip_mounts=SKIP_MOUNTS, skip_files=SKIP_FILES)
    return pl, out


if __name__ == "__main__":
    pl, out = plan()
    prev = SCRATCH + "/pi1/preview-g1"
    for f, t in out.items():
        dst = prev + "/" + rel(f)
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        write(dst, t)
    print("files", len(out))
    for K, n in pl.fnotes.items():
        print(K, len(n))
        for x in n:
            print("   ", x)


# ---------------------------------------------------------------------------------------------------------------
# overlay (nothing is written to the repository until `apply`)
# ---------------------------------------------------------------------------------------------------------------
import re
import rsparse as P
from rs import blank
import widen as WD
import manifest as MF
import owners as O

HUB3 = PLUG + "/🪝️mounts/👥️wp3-owners/🦀️.rs"
DELETE = [PLUG + "/🧩️composition/📨️emission/📦️preparation/🦀️.rs"]
CHILD_CONTENT = """pub(crate) mod child_content {
    pub use semio_framework_plugin_child::*;
    pub use semio_framework_plugin_composition_emission::{metadata::*, root::*};
    pub use semio_framework_plugin_composition_publication::group::*;
    pub use semio_framework_plugin_publication_mounted::{command_mounted::*, owner::*, mounted_child_group_identity, mounted_child_group_receipt, mounted_child_group_receipt_group};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_identity::{MountedChildGroupIdentityIssuer, MountedChildGroupIdentitySource};
    pub use semio_framework_plugin_publication_mounted::mounted_child_group_receipt_group::command::PagedCommandLog;
}"""
HUB_UNITS = {
    "child_emit_preparation": "pub(crate) use semio_framework_plugin_decl_emission::child_emit_preparation;",
    "child_content": CHILD_CONTENT,
    "private_child_input": "pub(crate) use semio_framework_plugin_composition_emission::private_child_input;",
    "store_publication": "pub(crate) extern crate semio_framework_plugin_composition_publication as store_publication;",
}


def replace_units(hub_text, units):
    clean = blank(hub_text)
    edits = []
    for st in P.statements(hub_text, clean, 0, len(hub_text)):
        if st.kind == "mod" and not st.inline and st.name in units:
            edits.append((st.start, st.end, units[st.name]))
    got = {n for n in units}
    found = {st.name for st in P.statements(hub_text, clean, 0, len(hub_text)) if st.kind == "mod" and st.name in units and not st.inline}
    assert found == got, (got - found)
    edits.sort(reverse=True)
    t = hub_text
    for a, b, rep in edits:
        t = t[:a] + rep + t[b:]
    return t


def build_overlay(pl, out, hub_units=None, extra_hub=""):
    ov = {}
    for f, t in out.items():
        ov[os.path.relpath(f, REPO)] = t
    for f in DELETE:
        ov[os.path.relpath(f, REPO)] = None
    flipped = set(GROUP) | set(__import__("uo").BASE_REAL)
    hub = read(HUB3)
    hub = replace_units(hub, hub_units or HUB_UNITS)
    for K in GROUP:
        ht = pl.host_text(K, flipped)
        if ht:
            hp = pl.host_path(K)
            ov[os.path.relpath(hp, REPO)] = ht
            rel_host = os.path.relpath(hp, os.path.dirname(HUB3))
            hub = hub.rstrip("\n") + f'\n#[cfg(test)]\n#[path = "{rel_host}"]\nmod {pl.host_module_name(K)};\n'
    ov[os.path.relpath(HUB3, REPO)] = hub + extra_hub
    return ov


def widen_overlay(pl, ov):
    """name-keyed widening of every group crate against all files outside it"""
    kfiles = collections.defaultdict(set) if False else {}
    import collections as C
    kfiles = C.defaultdict(set)
    for relp in ov:
        f = REPO + "/" + relp
        if ov[relp] is None or not f.startswith(PLUG + "/"):
            continue
        k = O.owner_of(rel(f)) if f not in pl.file_part else pl.LY.part_owner(pl.file_part[f])
        if k in pl.gset and "🪝️mounts" not in f:
            kfiles[k].add(relp)
    live = {os.path.relpath(f, REPO): f for f in WD.live_rs_files()}
    report = {}
    for K in GROUP:
        outside = set()
        for relp, f in live.items():
            if relp in kfiles[K] or ov.get(relp, 1) is None:
                continue
            if relp in ov and ov[relp] is not None:
                outside |= set(WD.IDENT.findall(blank(ov[relp])))
            else:
                outside |= WD.file_idents(f)
        for relp, t in ov.items():
            if relp not in live and t is not None and relp not in kfiles[K]:
                outside |= set(WD.IDENT.findall(blank(t)))
        changed = 0
        for relp in sorted(kfiles[K]):
            t = ov[relp]
            clean = blank(t)
            defs = WD.collect(t, clean, 0, len(t))
            names = {d[1] for d in defs} & outside
            nt = WD.apply_widen(t, defs, names)
            if nt != t:
                ov[relp] = nt
                changed += 1
        report[K] = changed
    return report


def full_overlay():
    pl, out = plan()
    ov = build_overlay(pl, out)
    rep = widen_overlay(pl, ov)
    return pl, ov, rep


if __name__ == "__main__" and "--overlay" in sys.argv:
    pl, ov, rep = full_overlay()
    path = SCRATCH + "/pi1/overlay-g1.json"
    os.makedirs(os.path.dirname(path), exist_ok=True)
    json.dump({os.path.relpath(REPO + "/" + k, REPO): v for k, v in ov.items()}, open(path, "w"), ensure_ascii=False)
    print("overlay", len(ov), "files; widened files per crate", rep, "->", path)


def manifests(pl, ov):
    """new Cargo.toml text per crate, from the overlay texts"""
    res = {}
    for K in GROUP:
        prod, test = [], []
        for relp, t in ov.items():
            f = REPO + "/" + relp
            if t is None or not f.startswith(PLUG + "/") or "🪝️mounts" in f:
                continue
            k = pl.LY.part_owner(pl.file_part[f]) if f in pl.file_part else O.owner_of(rel(f))
            if k != K or not f.endswith(".rs"):
                continue
            a, b = pl.split_prod_test(f, t)
            prod.append(a)
            test.append(b)
        new, rep = MF.plan_manifest(K, prod, test, flip_lib=(K in ROOTS_B))
        res[K] = (new, rep)
    return res


def apply(pl, ov, mans):
    import ed
    for relp, t in ov.items():
        path = REPO + "/" + relp
        if t is None:
            ed.delete(path)
        else:
            ed.put(path, t)
    for K, (text, rep) in mans.items():
        man = MF.pkg_dir(K) + "/Cargo.toml"
        ed.put(man, text)
        if K in ROOTS_B:
            stub = MF.pkg_dir(K) + "/🦀️.rs"
            if os.path.exists(stub):
                ed.delete(stub)
    cur = json.load(open(TOOLS + "/flipped.json")) if os.path.exists(TOOLS + "/flipped.json") else []
    for K in GROUP:
        if K not in cur:
            cur.append(K)
    write(TOOLS + "/flipped.json", json.dumps(cur))
