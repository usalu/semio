"""Stub-layer typeck oracle for the hub directory capability split (sandbox only, never writes into the repo).
Baseline = pre-edit snapshot (HubDirectory port), current = live tree (or snapshot + staged overlay with --stage).
For crates that import the port from `semio_hub_directory` the stub layer carries the REAL trait definitions
(old single trait for the baseline, the capability traits for the current tree), so trait impls, method partition,
trait-in-scope resolution and signatures are type-checked."""
import os, re, sys, shutil, subprocess, collections, json
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
sys.path.insert(0, F + "/frs-tools")
import stubgen
from stubgen import rs_files, collect, build_stub, rewrite, strip_ext_attrs, read as sread, write as swrite

stubgen.HINT_TRAITS |= {"FromRow"}
TOOLCHAIN = "+nightly-2026-07-20"
ORC = SC + "/hdc-orc"
OUTDIR = SC + "/hdc-out"
ORIG = SC + "/hdc-orig"
STAGE = SC + "/hdc-stage"
D = "🌎️hub/📇️directory"
TARGETS = {
    "directory": dict(lib=D + "/🦀️.rs", crate="semio_hub_directory", cargo=D + "/📦️packages/🦀️rust/Cargo.toml", real=False, cfgs=["lib", "test", "testkit", "test+testkit"]),
    "sqlite": dict(lib=D + "/🪶️sqlite/🦀️.rs", crate="semio_hub_directory_sqlite", cargo=D + "/🪶️sqlite/📦️packages/🦀️rust/Cargo.toml", real=True, cfgs=["lib", "test"]),
    "postgres": dict(lib=D + "/🐘️postgres/🦀️.rs", crate="semio_hub_directory_postgres", cargo=D + "/🐘️postgres/📦️packages/🦀️rust/Cargo.toml", real=True, cfgs=["lib", "test", "testkit", "test+testkit"]),
    "neo4j": dict(lib=D + "/🌐️neo4j/🦀️.rs", crate="semio_hub_directory_neo4j", cargo=D + "/🌐️neo4j/📦️packages/🦀️rust/Cargo.toml", real=True, cfgs=["lib", "test", "testkit", "test+testkit"]),
    "dispatch": dict(lib=D + "/🔀️dispatch/🦀️.rs", crate="semio_hub_directory_dispatch", cargo=D + "/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml", real=True, cfgs=["lib"]),
}
CFG = {"lib": [], "test": ["--test"], "testkit": ["--cfg", 'feature="testkit"'], "test+testkit": ["--test", "--cfg", 'feature="testkit"'], "conformance": ["--cfg", 'feature="conformance"'], "test+conformance": ["--test", "--cfg", 'feature="conformance"']}
ALL_NEW = [t for sec in SECTIONS.values() for t, _ in sec] + [t for t, _ in EXTENSIONS]


def under_orc(p):
    rp = os.path.realpath(p)
    assert rp.startswith(os.path.realpath(ORC) + os.sep), "write outside sandbox: " + rp


def copy_dir_tree(src_root, dst_root):
    for dp, dn, fn in os.walk(src_root):
        dn[:] = [d for d in dn if d not in ("📦️packages", "node_modules", "🗑️generated")]
        rel = os.path.relpath(dp, src_root)
        for f in fn:
            s = os.path.join(dp, f)
            d = os.path.join(dst_root, rel, f)
            under_orc(d)
            os.makedirs(os.path.dirname(d), exist_ok=True)
            shutil.copy2(s, d)


def cargo_deps(path):
    names = []
    sec = None
    for line in read(path).split("\n"):
        s = line.strip()
        if s.startswith("["):
            sec = s
            continue
        if sec in ("[dependencies]", "[dev-dependencies]") and "=" in s and not s.startswith("#"):
            names.append(s.split("=")[0].strip().replace("-", "_"))
    return sorted(set(names))


GLOB = re.compile(r"^(\s*)(pub(?:\([^)]*\))?\s+)?use\s+([A-Za-z_][A-Za-z0-9_]*)(::[A-Za-z0-9_:{}, ]*)?::\*\s*;", re.M)


def patch_text(t, local):
    t = re.sub(r"^\s*extern crate [^;]+;\s*$", "", t, flags=re.M)
    return GLOB.sub(lambda m: m.group(0) if m.group(3) in local else m.group(1) + "const _: () = ();", t)


def use_statements(text):
    return [m.group(0) for m in re.finditer(r"^use [^;]+;", text, re.M)]


def real_port_module(variant, root, ext):
    """text of `pub mod __realport` built from the port files of the sandbox tree `root`."""
    d = os.path.join(root, D)
    port_root = sread(os.path.join(d, "🔌️port/🦀️.rs"))
    dir_root = sread(os.path.join(d, "🦀️.rs"))
    uses = [u for u in use_statements(dir_root) + use_statements(port_root) if re.match(r"^use (?:::)?(" + "|".join(map(re.escape, ext)) + r")\b", u) and "semio_hub_directory::" not in u.split("{")[0]]
    uses = [u for u in uses if "::*" not in u]
    body = []
    if variant == "old":
        text = sread(os.path.join(d, "🔌️port/🦀️.rs"))
        text = "\n".join(l for l in text.split("\n") if not l.startswith("use ") and not l.startswith("//!"))
        body.append(text)
    else:
        for sec in SECTIONS:
            p = os.path.join(d, "🔌️port", sec, "🦀️.rs")
            body.append("\n".join(l for l in sread(p).split("\n") if not l.startswith("use super") and not l.startswith("//!")))
    own = "use crate::__ext::semio_hub_directory::{ArtifactCasSweepCandidatePage, DirectoryAppendOutcomeV1, ProjectionRebuildControl};"
    mod = ["#[allow(dead_code, unused, non_snake_case, missing_docs, async_fn_in_trait)]", "pub mod __realport {", own]
    for u in uses:
        mod.append(rewrite(u, ext))
    mod += body
    mod.append("}")
    return "\n".join(mod) + "\n"


def inject_real(stub, names):
    for n in names:
        stub = re.sub(r"^(\s*)pub (?:trait|struct) %s(?:<[^>]*>)?(?: \{\}|;|\(.*\);)$" % n, lambda m: m.group(1) + "pub use crate::__realport::%s;" % n, stub, flags=re.M)
    return stub


def build_one(name, which, stage):
    t = TARGETS[name]
    src = ORIG if which == "A" else (STAGE_OVERLAY if stage else None)
    root = f"{ORC}/{name}-{which}"
    under_orc(root)
    shutil.rmtree(root, ignore_errors=True)
    if which == "A":
        copy_dir_tree(ROOT + "/" + D, root + "/" + D)
        for dp, dn, fn in os.walk(root + "/" + D):
            for f in fn:
                if f.endswith(".rs"):
                    q = os.path.join(dp, f)
                    under_orc(q)
                    os.remove(q)
        for dp, dn, fn in os.walk(ORIG + "/" + D):
            for f in fn:
                if f.endswith(".rs"):
                    q = os.path.join(root, D, os.path.relpath(os.path.join(dp, f), ORIG + "/" + D))
                    under_orc(q)
                    os.makedirs(os.path.dirname(q), exist_ok=True)
                    shutil.copy2(os.path.join(dp, f), q)
    else:
        if stage:
            copy_dir_tree(ORIG + "/" + D, root + "/" + D)
            # overlay staged files (stage dir mirrors paths relative to the directory dir)
            for dp, dn, fn in os.walk(STAGE):
                for f in fn:
                    s = os.path.join(dp, f)
                    d = os.path.join(root, D, os.path.relpath(s, STAGE))
                    under_orc(d)
                    os.makedirs(os.path.dirname(d), exist_ok=True)
                    shutil.copy2(s, d)
        else:
            copy_dir_tree(ROOT + "/" + D, root + "/" + D)
    ext = cargo_deps(ROOT + "/" + t["cargo"])
    mods = set()
    files = rs_files(root + "/" + D)
    for p in files:
        mods |= set(re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*[;{]", sread(p), re.M))
    local = {"super", "crate", "self"} | mods | {t["crate"]}
    for p in files:
        under_orc(p)
        tx = sread(p)
        t2 = patch_text(tx, local)
        if t2 != tx:
            swrite(p, t2)
    # analysis files: this crate's tree only (the lib root's module closure), plus port files for stub collection
    libp = os.path.join(root, t["lib"])
    crate_files = module_closure(libp)
    extra = []
    if t["real"]:
        d = root + "/" + D
        extra = [d + "/🦀️.rs"] + rs_files(d + "/🔌️port")
    found = collect(crate_files + extra, ext)
    stub = build_stub(found)
    variant = "old" if which == "A" else "new"
    if t["real"]:
        names = ["HubDirectory"] if variant == "old" else ALL_NEW
        stub = inject_real(stub, names)
        stub += real_port_module(variant, root, ext)
    for p in crate_files:
        tx = sread(p)
        t2 = strip_ext_attrs(rewrite(tx, ext), ext)
        if t2 != tx:
            swrite(p, t2)
    swrite(libp, sread(libp) + "\n" + stub)
    return root


def module_closure(libp):
    seen = []
    todo = [libp]
    while todo:
        p = todo.pop()
        if p in seen or not os.path.exists(p):
            continue
        seen.append(p)
        tx = sread(p)
        for m in re.finditer(r'((?:\s*#\[[^\]]*\]\s*)*)\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', mask(tx)):
            attrs = tx[m.start(1):m.end(1)]
            pa = re.search(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]', attrs)
            if pa:
                todo.append(os.path.normpath(os.path.join(os.path.dirname(p), pa.group(1))))
    return seen


def run_rustc(root, lib_rel, crate, extra):
    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, f"{os.getpid()}.rmeta")
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", out, "--cap-lints", "allow", *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    if os.path.exists(out):
        os.remove(out)
    return p.stderr


HEAD = re.compile(r"^(error(?:\[E\d+\])?): (.*)$")
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def parse(err, root):
    L = err.replace(root, "<S>").split("\n")
    keys = []
    for i, l in enumerate(L):
        m = HEAD.match(l)
        if m and not l.startswith("error: aborting") and "previous error" not in l:
            loc = ""
            label = ""
            for j in range(i + 1, min(i + 14, len(L))):
                if HEAD.match(L[j]) or L[j].startswith("warning"):
                    break
                mm = re.match(r"^\s*--> (.*?):(\d+):(\d+)", L[j])
                if mm and not loc:
                    loc = mm.group(1)
                ml = re.search(r"\^+ ?(.*)$", L[j])
                if ml and not label and "|" in L[j]:
                    label = ml.group(1)
            keys.append((m.group(1), PATHRX.sub("", m.group(2)), PATHRX.sub("", label), loc))
    return keys


def run_warn(root, lib_rel, crate, extra):
    os.makedirs(OUTDIR, exist_ok=True)
    out = os.path.join(OUTDIR, f"{os.getpid()}w.rmeta")
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", out, "-W", "unused-imports", "-W", "unused-qualifications", *extra, os.path.join(root, lib_rel)]
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    if os.path.exists(out):
        os.remove(out)
    L = p.stderr.replace(root, "<S>").split("\n")
    res = []
    for i, l in enumerate(L):
        if l.startswith("warning: unused import") or l.startswith("warning: unused qualif"):
            loc = next((x.strip() for x in L[i + 1:i + 3] if x.strip().startswith("-->")), "")
            res.append((l[len("warning: "):], loc.split(":")[0].split("/")[-2] if "/" in loc else loc))
    return res


def main_warn():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    name = args[0]
    t = TARGETS[name]
    rootA = build_one(name, "A", False)
    rootB = build_one(name, "B", False)
    for c in (args[1].split(",") if len(args) > 1 else t["cfgs"]):
        wa = run_warn(rootA, t["lib"], t["crate"], CFG[c])
        wb = run_warn(rootB, t["lib"], t["crate"], CFG[c])
        ca, cb = collections.Counter(w[0] for w in wa), collections.Counter(w[0] for w in wb)
        print(f"== warnings {name} {c}: baseline={len(wa)} current={len(wb)} new={sum((cb-ca).values())} gone={sum((ca-cb).values())}")
        for k, v in sorted((cb - ca).items()):
            where = sorted({w[1] for w in wb if w[0] == k})
            print("   +", v, k[:200], where)
        for k, v in sorted((ca - cb).items()):
            print("   -", v, k[:200])


def main():
    if "--warn" in sys.argv:
        return main_warn()
    args = sys.argv[1:]
    stage = "--stage" in args
    mut = [a for a in args if a.startswith("--mutate=")]
    args = [a for a in args if not a.startswith("--")]
    name = args[0]
    only = args[1].split(",") if len(args) > 1 else None
    t = TARGETS[name]
    rootA = build_one(name, "A", stage)
    rootB = build_one(name, "B", stage)
    for m in mut:
        rel, old, new = m[len("--mutate="):].split("@@")
        fp = os.path.join(rootB, rel)
        under_orc(fp)
        tx = sread(fp)
        assert tx.count(old) >= 1, "mutation anchor missing: " + old
        swrite(fp, tx.replace(old, new, 1))
        print("mutated", rel)
    bad = 0
    summary = {}
    for c in (only or t["cfgs"]):
        ea = run_rustc(rootA, t["lib"], t["crate"], CFG[c])
        eb = run_rustc(rootB, t["lib"], t["crate"], CFG[c])
        swrite(f"{GEN}/hdc-oracle-{name}-{c}-A.err", ea.replace(rootA, "<S>"))
        swrite(f"{GEN}/hdc-oracle-{name}-{c}-B.err", eb.replace(rootB, "<S>"))
        ka, kb = parse(ea, rootA), parse(eb, rootB)
        ca = collections.Counter(k[:3] for k in ka)
        cb = collections.Counter(k[:3] for k in kb)
        oa, ob = ca - cb, cb - ca
        status = "IDENTICAL" if not oa and not ob else "DIFFERENT"
        if status != "IDENTICAL":
            bad += 1
        print(f"== {name} {c}: baseline errors={len(ka)} current errors={len(kb)} {status}")
        for (h, m, l), n in sorted(ob.items())[:80]:
            print(f"   +current {n}x {h}: {m[:170]} | {l[:90]}")
        for (h, m, l), n in sorted(oa.items())[:80]:
            print(f"   -baseline {n}x {h}: {m[:170]} | {l[:90]}")
        codes = collections.Counter(k[0] for k in kb)
        print("   current codes:", dict(codes.most_common(12)))
        summary[c] = dict(base=len(ka), cur=len(kb), status=status)
    swrite(f"{GEN}/hdc-oracle-{name}-summary.json", json.dumps(summary, ensure_ascii=False, indent=1))
    print("RESULT", name, "ALL IDENTICAL" if bad == 0 else f"{bad} DIFFERENT")


if __name__ == "__main__":
    main()
