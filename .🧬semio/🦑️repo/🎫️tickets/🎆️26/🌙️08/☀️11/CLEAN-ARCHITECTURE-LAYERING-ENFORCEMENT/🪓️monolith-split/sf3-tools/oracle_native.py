#!/usr/bin/env python3
"""🔬️ sf3: stub-layer full-resolution differential (frs-tools oracle machinery) for the generation-native extraction.

Baseline A  = the pre-edit io/text tree of one artifact with the original template mounted by `#[path]` (pre-edit copies from the scratchpad `sf3pre` and `sf3-native-pre`).
Current  B  = the live io/text tree with the new crate `semio_s_procedural_core_generation_sqlite_native` mounted as module `__native` and every `semio_s_procedural_core_generation_sqlite_native::` rewritten to `crate::__native::`.
Both sandboxes are patched and stubbed identically (frs-tools/oracle.py, stubgen.py); rustc writes its metadata into the scratchpad and the file is deleted at once.

usage: oracle_native.py <2d|3d> [cfg,cfg] [--mutate rel@@old@@new] [--warn] [--label x]
"""
import os, re, shutil, sys, collections, json

FRS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/frs-tools"
sys.path.insert(0, FRS)
import frslib
import oracle
import stubgen

REPO = frslib.ROOT
F = frslib.F
SCR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
W = F + "/🗑️generated/sf3-work"
oracle.W = W
frslib.W = W
read, write = frslib.read, frslib.write

P = "✏️s/🔌️plugins/🌀️procedural"
TEXT = {"2d": ("🌀️generation2d", "semio_s_artifact_procedural_2d_io_text"), "3d": ("🧊️generation3d", "semio_s_artifact_procedural_3d_io_text")}
STD = "🏅️standards/🔖️1/🪆️subsets/✳️any"
NATIVE_REL = "🫀️core/🧬️generation/🪶️sqlite/🚦️native"
NAME = "semio_s_procedural_core_generation_sqlite_native"
SKIP_DIR = {"node_modules", "target", "dist"}


def copy_filtered(src, dst):
    sandbox_ok(dst)
    shutil.copytree(src, dst, symlinks=True, ignore=shutil.ignore_patterns(*SKIP_DIR))


def sandbox_ok(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


def build(art, which):
    name, crate = TEXT[art]
    root = f"{W}/{art}/{which}"
    sandbox_ok(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    os.makedirs(root)
    text_rel = f"{P}/🗿️artifacts/{name}/{STD}/🚪️io/📝️text"
    ex_rel = f"{P}/🗿️artifacts/{name}/{STD}/📚️examples"
    for rel in (text_rel, ex_rel):
        copy_filtered(f"{REPO}/{rel}", f"{root}/{rel}")
    nat_rel = f"{P}/{NATIVE_REL}"
    if which == "A":
        copy_filtered(f"{SCR}/sf3-native-pre", f"{root}/{nat_rel}")
        pre = f"{SCR}/sf3pre/{text_rel}/📸️snapshot/🦀️.rs"
        shutil.copyfile(pre, f"{root}/{text_rel}/📸️snapshot/🦀️.rs")
    else:
        copy_filtered(f"{REPO}/{nat_rel}", f"{root}/{nat_rel}")
        lib = f"{root}/{text_rel}/🦀️.rs"
        up = "../" * 8
        write(lib, read(lib) + f'\n#[path = "{up}{NATIVE_REL}/🦀️.rs"]\npub mod __native;\n')
        for dp, dn, fn in os.walk(f"{root}/{text_rel}"):
            for f in fn:
                if f.endswith(".rs"):
                    p = os.path.join(dp, f)
                    t = read(p)
                    t2 = re.sub(r"(?<![A-Za-z0-9_])" + NAME + r"(?=\s*::)", "crate::__native", t)
                    if t2 != t:
                        write(p, t2)
    return root, f"{text_rel}/🦀️.rs", crate


def ext_crates(root):
    names = set()
    rx = re.compile(r"(?<![A-Za-z0-9_\.:])(semio_[a-z0-9_]+|pack|store|protocol|serde_json|serde)(?=\s*::|\s+as\b)")
    for p in stubgen.rs_files(root):
        names |= set(rx.findall(stubgen.mask(read(p))))
    return sorted(names)


def fix_variants(rootA, rootB, lib_rel, crate):
    """Sandbox-only: the stub enums emit unit variants; variants the code uses as tuple patterns or calls become one-field tuple variants (found by a probe run on A)."""
    for _ in range(4):
        err = oracle.run_rustc(rootA, lib_rel, crate, [])
        pairs = set(re.findall(r"expected tuple struct or tuple variant, found unit variant `(?:[\w:]*::)?(\w+)::(\w+)`", err))
        if not pairs:
            return
        for r in (rootA, rootB):
            lp = os.path.join(r, lib_rel)
            tx = read(lp)
            for enum, variant in pairs:
                m = re.search(r"pub enum " + enum + r"(?:<[^>]*>)? \{ ([^}]*) \}", tx)
                if not m:
                    continue
                body = m.group(1)
                body2 = re.sub(r"(?<![\w(])" + variant + r"(?![\w(])", variant + "(__Any)", body)
                tx = tx.replace(m.group(0), m.group(0).replace(body, body2))
            if "pub struct __Any;" not in tx:
                tx += "\n#[allow(dead_code)]\npub struct __Any;\n"
            write(lp, tx)


def fix_snapshot_struct(art, rootA, rootB, lib_rel):
    """Sandbox-only: the model snapshot is a struct with exactly two pub fields (read in `🧬️schema/📸️snapshot/🦀️.rs`); the stub emits a unit struct or an enum for it."""
    name = f"Generation{art}Snapshot"
    body = f"pub struct {name} {{ pub host_snapshot: crate::__ext::semio_framework_artifact_flow_flow::FlowHostSnapshot, pub generation: crate::__ext::semio_framework_artifact_playbook_playbook::GenerationPlayRoot }}"
    rx = re.compile(r"pub (?:struct|enum) " + name + r"(?![A-Za-z0-9_])(?:\s*;|\s*\{[^}]*\})")
    for r in (rootA, rootB):
        lp = os.path.join(r, lib_rel)
        tx = read(lp)
        n = len(rx.findall(tx))
        tx = rx.sub(body, tx)
        write(lp, tx)
        print(f"   stub {name}: {n} definition(s) replaced by the two-field struct in {os.path.basename(r)}")


def main():
    art = sys.argv[1]
    cfgs = sys.argv[2].split(",") if len(sys.argv) > 2 and not sys.argv[2].startswith("--") else ["lib", "test"]
    label = sys.argv[sys.argv.index("--label") + 1] if "--label" in sys.argv else "run"
    rootA, libA, crate = build(art, "A")
    rootB, libB, _ = build(art, "B")
    assert libA == libB
    for r in (rootA, rootB):
        for pth in stubgen.rs_files(f"{r}/{P}/{NATIVE_REL}"):
            tx = read(pth)
            t2 = re.sub(r"(?<![A-Za-z0-9_])semio_framework_os_flow_contract(?=\s*::)", "semio_framework_artifact_flow_flow", tx)
            t2 = re.sub(r"(?<![A-Za-z0-9_])semio_framework_os_playbook_contract(?=\s*::)", "semio_framework_artifact_playbook_playbook", t2)
            if t2 != tx:
                write(pth, t2)
    pa = oracle.patch_tree(rootA, crate)
    pb = oracle.patch_tree(rootB, crate)
    for r in (rootA, rootB):
        for pth in stubgen.rs_files(r):
            tx = read(pth)
            t2 = re.sub(r"#\[\s*dsl\s*\([^\]\n]*\)\s*\]", "", tx)
            if t2 != tx:
                write(pth, t2)
    print(f"[{art}] patched A={pa} B={pb}")
    crates = sorted(set(ext_crates(rootA)) | set(ext_crates(rootB)))
    print("ext crates:", len(crates))
    stubgen.HINT_TRAITS |= set(os.environ.get("SF3_TRAITS", "").split(",")) - {""}
    stubgen.apply([rootA, rootB], libA, crates, "")
    fix_snapshot_struct(art, rootA, rootB, libA)
    fix_variants(rootA, rootB, libA, crate)
    for i, a in enumerate(sys.argv):
        if a == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(rootB, rel)
            sandbox_ok(fp)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print(f"   mutated {rel}: {old!r} -> {new!r}")
    runs = [(c, oracle.CFG[c]) for c in cfgs]
    if "--warn" in sys.argv:
        def unused(err, root):
            L = err.replace(root, "<S>").split("\n")
            out = []
            for i, l in enumerate(L):
                if l.startswith("warning: unused import") or l.startswith("warning: unused qualif") or "more private than" in l or l.startswith("warning: private ") or l.startswith("warning: unreachable pub"):
                    loc = next((x.strip() for x in L[i + 1:i + 3] if x.strip().startswith("-->")), "")
                    names = re.findall(r"`([^`]+)`", l)
                    out.append((re.sub(r":\d+:\d+$", "", loc.replace("--> <S>/", ""))[-90:], l[len("warning: "):], tuple(names)))
            return out
        for c, extra in runs:
            wa = unused(oracle.run_rustc(rootA, libA, crate, extra, False), rootA)
            wb = unused(oracle.run_rustc(rootB, libA, crate, extra, False), rootB)
            ca, cb = collections.Counter(w[1] for w in wa), collections.Counter(w[1] for w in wb)
            print(f"== warnings {c}: baseline={len(wa)} current={len(wb)} new={sum((cb - ca).values())} gone={sum((ca - cb).values())}")
            for k in sorted(cb - ca):
                print("   +", k[:300], sorted({w[0] for w in wb if w[1] == k}))
            for k in sorted(ca - cb):
                print("   -", k[:300])
        return
    bad = 0
    for c, extra in runs:
        eA = oracle.run_rustc(rootA, libA, crate, extra).replace(rootA, "<S>")
        eB = oracle.run_rustc(rootB, libA, crate, extra).replace(rootB, "<S>")
        write(f"{F}/🗑️generated/sf3-oracle-{art}-{label}-{c}-A.err", eA)
        write(f"{F}/🗑️generated/sf3-oracle-{art}-{label}-{c}-B.err", eB)
        ka, kb = oracle.parse(eA), oracle.parse(eB)
        ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
        onlyA, onlyB = ca - cb, cb - ca
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        bad += status != "IDENTICAL"
        print(f"== {c}: baseline errors={len(ka)} current errors={len(kb)} {status}")
        for (h, m, l), n in sorted(onlyB.items())[:80]:
            print(f"   +current {n}x {h}: {m[:170]} | {l[:90]}")
        for (h, m, l), n in sorted(onlyA.items())[:80]:
            print(f"   -current {n}x {h}: {m[:170]} | {l[:90]}")
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


main()
