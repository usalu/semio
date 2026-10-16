"""Patched full-resolution differential with the sandbox stub layer for the os-pack record codec dedupe (orc).
A = frozen snapshot of the os-pack tree before the edit, B = the live os-pack tree; both get the same stub layer for every external crate (pack, pack-record-*, dsl-record, value, ...).
Writes only under orc-work in the generated folder; rustc output goes to a scratch rmeta that is deleted at once.
Usage: oracle.py snapshot | run <label> [cfg,cfg] [--nostubs] [--mutate rel@@old@@new]"""
import os, re, shutil, subprocess, sys, collections, json
sys.path.insert(0, os.path.dirname(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "frs-tools"))
from frslib import read, write
import stubgen

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/orc-work"
SKIP = {"node_modules", "target", "🗑️generated", ".git", "dist"}
PK = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack"
FPK = "🧰️framework/🔨️modules/🎒️pack"
TREES = [PK]
LIB = PK + "/🦀️.rs"
CRATE = "semio_framework_os_pack"
EXT = ["semio_framework_dsl_record", "semio_framework_value", "semio_framework_os_pack", "serde_json", "semio_framework_async_macros", "semio_framework_async", "semio_framework_os_kernel", "semio_framework_dsl", "protocol", "pack", "semio_framework_pack_source", "semio_framework_pack_json", "semio_framework_io_base64", "semio_framework_hash", "blake3", "serde", "semio_framework_pack_schema_index", "semio_framework_pack_record_token", "semio_framework_pack_intrinsic_media_laws", "semio_framework_pack_record", "semio_framework_diagnostic", "semio_framework_pack_record_schema", "semio_framework_trace_memory_testing", "semio_framework_pack_error", "semio_framework_replication_codec"]
stubgen.HINT_TRAITS |= {"NativeSchemaControl"}
TOOLCHAIN = "+nightly-2026-07-20"
CFG = {
    "lib": [], "test": ["--test"],
    "wasm": ["--target", "wasm32-unknown-unknown"], "p2": ["--target", "wasm32-wasip2"],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"], "p2test": ["--test", "--target", "wasm32-wasip2"],
}
FEATURES = ["deflate", "record-codec-laws"]
KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}
OUTDIR = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/orc-out"


def under_sandbox(path):
    rp = os.path.realpath(path)
    assert rp.startswith(os.path.realpath(W) + os.sep), f"write outside sandbox: {rp}"


def copy_tree(rel, dst_root, only=None):
    base = os.path.join(ROOT, rel)
    n = 0
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            p = os.path.join(dp, f)
            if os.path.islink(p) or (not f.endswith(".rs") and os.path.getsize(p) > 300_000):
                continue
            dst = os.path.join(dst_root, os.path.relpath(p, ROOT))
            under_sandbox(dst)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(p, dst)
            n += 1
    return n


def patch_tree(tree_root, crate):
    files, mods = [], set()
    for dp, dn, fn in os.walk(tree_root):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                files.append(p)
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", read(p), re.M):
                    mods.add(m.group(1))
    keep = KEEP_EXT | mods | {crate}
    rx = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    ex = re.compile(r"^\s*extern\s+crate\s+(\w+)(?:\s+as\s+\w+)?;[ \t]*$", re.M)
    n = [0, 0]
    for p in files:
        under_sandbox(p)
        t = read(p)

        def sub(m):
            if m.group(2) in keep:
                return m.group(0)
            n[0] += 1
            return m.group(1) + "const _: () = ();"

        def subex(m):
            if m.group(1) in keep:
                return m.group(0)
            n[1] += 1
            return ""

        t2 = ex.sub(subex, rx.sub(sub, t))
        if t2 != t:
            write(p, t2)
    return n


def run_rustc(root, extra, caplints=True):
    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, f"{os.getpid()}.rmeta")
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", CRATE, "--emit=metadata", "-o", out, *(["--cap-lints", "allow"] if caplints else []), *extra, os.path.join(root, LIB)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    if os.path.exists(out):
        os.remove(out)
    return p.stderr


HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def norm(s):
    return PATHRX.sub("", s)


def parse(err):
    L = err.split("\n")
    keys, i = [], 0
    while i < len(L):
        m = HEAD.match(L[i])
        if m and not L[i].startswith("error: aborting") and "previous error" not in L[i]:
            msg, loc, label = m.group(2), "", ""
            for j in range(i + 1, min(i + 14, len(L))):
                if HEAD.match(L[j]) or L[j].startswith("warning"):
                    break
                mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
                if mm and not loc:
                    loc = mm.group(1)
                ml = re.search(r"\^+ ?(.*)$", L[j])
                if ml and not label and "|" in L[j]:
                    label = ml.group(1)
            keys.append((m.group(1), norm(msg), norm(label), loc))
        i += 1
    return keys


def snapshot():
    d = W + "/snap"
    assert os.path.exists(d), "the pre-edit snapshot was taken before the first edit and must not be recreated"
    print("snapshot exists", d)


def block_span(text, header, indent):
    start = text.index(header)
    end_marker = "\n" + indent + "}\n"
    end = text.index(end_marker, start) + len(end_marker)
    return start, end


def graft_owner(root):
    return


def alias_protocol_value(root):
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                under_sandbox(p)
                s = read(p)
                s2 = s.replace("protocol::value", "semio_framework_value")
                if s2 != s:
                    write(p, s2)


def inject_laws(root):
    macro = read(os.path.join(ROOT, FPK + "/🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs"))
    macro = "\n".join(l for l in macro.split("\n") if not l.startswith("//!")).replace("#[macro_export]\n", "")
    lp = os.path.join(root, LIB)
    under_sandbox(lp)
    lines = read(lp).split("\n")
    at = 0
    while at < len(lines) and (lines[at].startswith("//!") or lines[at].strip() == ""):
        at += 1
    lines[at:at] = macro.split("\n")
    write(lp, "\n".join(lines))
    vp = os.path.join(root, PK + "/🌱️value/🦀️.rs")
    if os.path.exists(vp):
        s = read(vp)
        assert s.count("semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!") == 1
        write(vp, s.replace("semio_framework_pack_intrinsic_media_laws::intrinsic_media_wire_laws!", "intrinsic_media_wire_laws!"))


def fresh(which, nostubs):
    root = f"{W}/oracle-{which}"
    under_sandbox(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    if which == "A":
        shutil.copytree(W + "/snap", root, symlinks=True)
    else:
        for t in TREES:
            copy_tree(t, root)
    alias_protocol_value(root)
    seed = os.path.join(root, "__seed", "🦀️.rs")
    under_sandbox(seed)
    write(seed, "use semio_framework_trace_memory_testing::{observe, observe_backing, RequestedAllocator};\n")
    if "--nolaws" not in sys.argv:
        inject_laws(root)
    return root, patch_tree(root, CRATE)


def main():
    if sys.argv[1] == "snapshot":
        return snapshot()
    label = sys.argv[2]
    only = sys.argv[3].split(",") if len(sys.argv) > 3 and not sys.argv[3].startswith("--") else None
    nostubs = "--nostubs" in sys.argv
    rootA, pa = fresh("A", nostubs)
    rootB, pb = fresh("B", nostubs)
    print(f"patched A={pa} B={pb}")
    if not nostubs:
        stubgen.apply([rootA, rootB], LIB, EXT, "")
        graft_owner(rootB)
    for i, a_ in enumerate(sys.argv):
        if a_ == "--mutate":
            rel, old, new = sys.argv[i + 1].split("@@")
            fp = os.path.join(rootB, rel)
            under_sandbox(fp)
            tx = read(fp)
            assert tx.count(old) >= 1, f"mutation anchor missing: {old}"
            write(fp, tx.replace(old, new, 1))
            print(f"   mutated {rel}: {old!r} -> {new!r}")
    runs = []
    for c in (only or list(CFG)):
        runs.append((c, CFG[c]))
        runs.append((c + "+deflate", CFG[c] + ["--cfg", 'feature="deflate"']))
        runs.append((c + "+all", CFG[c] + sum([["--cfg", f'feature="{ft}"'] for ft in FEATURES], [])))
    if "--warn" in sys.argv:
        for c, extra in runs:
            wa = [l for l in run_rustc(rootA, extra, False).split("\n") if l.startswith("warning: unused") or "more private than" in l or l.startswith("warning: unreachable pub")]
            wb = [l for l in run_rustc(rootB, extra, False).split("\n") if l.startswith("warning: unused") or "more private than" in l or l.startswith("warning: unreachable pub")]
            ca, cb = collections.Counter(wa), collections.Counter(wb)
            print(f"== warnings {c}: baseline={len(wa)} current={len(wb)} new={sum((cb - ca).values())} gone={sum((ca - cb).values())}")
            for k, n in sorted((cb - ca).items())[:40]:
                print(f"   +current {n}x {k[:200]}")
            for k, n in sorted((ca - cb).items())[:40]:
                print(f"   -current {n}x {k[:200]}")
        return
    summary, bad = {}, 0
    for c, extra in runs:
        errA = run_rustc(rootA, extra).replace(rootA, "<S>")
        errB = run_rustc(rootB, extra).replace(rootB, "<S>")
        write(f"{F}/🗑️generated/orc-oracle-{label}-{c}-A.err", errA)
        write(f"{F}/🗑️generated/orc-oracle-{label}-{c}-B.err", errB)
        ka, kb = parse(errA), parse(errB)
        ca = collections.Counter((k[0], k[1], k[2]) for k in ka)
        cb = collections.Counter((k[0], k[1], k[2]) for k in kb)
        onlyA, onlyB = ca - cb, cb - ca
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        if status != "IDENTICAL":
            bad += 1
        RES = {"E0432", "E0433", "E0412", "E0425", "E0422", "E0423", "E0424", "E0426", "E0603", "E0616", "E0624", "E0451", "E0404", "E0405", "E0407", "E0583", "E0599", "E0609", "E0560", "E0063"}
        code = lambda k: (re.match(r"error\[(E\d+)\]", k[0]) or [None, ""])[1]
        fa = collections.Counter((k[0], k[1], k[2]) for k in ka if code(k) in RES)
        fb = collections.Counter((k[0], k[1], k[2]) for k in kb if code(k) in RES)
        rA, rB = fa - fb, fb - fa
        print(f"== {c}: baseline errors={len(ka)} current errors={len(kb)} {status}; resolution/privacy-class: baseline={sum(fa.values())} current={sum(fb.values())} only-current={sum(rB.values())} only-baseline={sum(rA.values())}")
        for (h, m, l), n in sorted(rB.items())[:80]:
            print(f"   !res+current {n}x {h}: {m[:150]} | {l[:90]}")
        for (h, m, l), n in sorted(rA.items())[:80]:
            print(f"   !res-current {n}x {h}: {m[:150]} | {l[:90]}")
        for (h, m, l), n in sorted(onlyB.items())[:80]:
            print(f"   +current {n}x {h}: {m[:150]} | {l[:90]}")
        for (h, m, l), n in sorted(onlyA.items())[:80]:
            print(f"   -current {n}x {h}: {m[:150]} | {l[:90]}")
        summary[c] = dict(base=len(ka), cur=len(kb), status=status)
    write(f"{F}/🗑️generated/orc-oracle-{label}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
