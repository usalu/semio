"""Union typeck oracle for production crate P + companion crate(s) C (sandbox only; the repo is never written).
Baseline A = the ORIGINAL crate tree (pre-edit snapshot, tests mounted in it) compiled with --test.
Current  B = P (cfg(test) neutralised, `crate::` -> `crate::prod::`, `pub(crate)` -> `pub(in crate::prod)`) and each
             companion (`crate::` -> `crate::compN::`, P's extern name -> `crate::prod`) mounted as modules of one synthetic crate.
Both go through the frs stub layer for the external crates. Rustc output goes to a scratch file that is deleted at once.
usage: union.py <crate-id> snapshot | run [cfg,...] [--json]     (config: otcp-tools/cfg/<crate-id>.json)"""
import json, os, re, shutil, subprocess, sys, collections
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import frslib
from frslib import read, write, ROOT, F
from snapshot import copy_tree
import stubgen
from stubgen import mask, rs_files
import modtree

W = F + "/🗑️generated/otcp-work"
OUT = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/otcp-out"
TOOLCHAIN = "+nightly-2026-07-20"
CFGS = {
    "test": ["--test"],
    "lib": [],
    "wasmtest": ["--test", "--target", "wasm32-unknown-unknown"],
    "p2test": ["--test", "--target", "wasm32-wasip2"],
}
PATHRX = re.compile(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])")


def norm(s):
    s = PATHRX.sub("", s)
    s = re.sub(r"`(?:comp\d+|prod)`", "`crate`", s)
    return s.replace("in the crate root", "in `crate`").replace("in module `crate`", "in `crate`").replace("in the root", "in `crate`")


KEEP_EXT = {"crate", "super", "self", "std", "core", "alloc"}


def cfg_of(cid):
    return json.load(open(f"{os.path.dirname(__file__)}/cfg/{cid}.json", encoding="utf-8"))


def under(path):
    rp = os.path.realpath(path)
    base = os.path.realpath(W)
    assert rp.startswith(base + os.sep), f"write outside sandbox: {rp}"


def sw(path, text):
    under(path)
    write(path, text)


def input_rels(cid, unit):
    p = f"{F}/🗑️generated/otcp-work/inputs-base/otcp-inputs-{unit}.txt"
    return [line.rstrip("\n").split("\t")[3] for line in open(p, encoding="utf-8") if line.count("\t") == 3]


def copy_rels(rels, dst_root, base=ROOT):
    for rel in rels:
        src = os.path.join(base, rel)
        if os.path.isfile(src) and not os.path.islink(src):
            dst = os.path.join(dst_root, rel)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)


def populate(cid, dst, source_root=None):
    c = cfg_of(cid)
    for tree in c["trees"]:
        if source_root:
            base = os.path.join(source_root, tree)
            for dp, dn, fn in os.walk(base):
                for f in fn:
                    p = os.path.join(dp, f)
                    rel = os.path.relpath(p, source_root)
                    d = os.path.join(dst, rel)
                    os.makedirs(os.path.dirname(d), exist_ok=True)
                    shutil.copyfile(p, d)
        else:
            copy_tree(tree, dst)
    for unit in c.get("inputs_units", []):
        copy_rels(input_rels(cid, unit), dst, source_root or ROOT)


def snapshot(cid, force=False):
    d = f"{W}/{cid}/snap"
    under(d)
    if os.path.exists(d) and not force:
        print("snapshot exists", d)
        return
    if os.path.exists(d):
        shutil.rmtree(d)
    populate(cid, d)
    print("snapshot", cid)


def sub_masked(text, rx, repl):
    m = mask(text)
    out, last = [], 0
    for mm in rx.finditer(m):
        out.append(text[last:mm.start()])
        out.append(repl(mm))
        last = mm.end()
    out.append(text[last:])
    return "".join(out)


def neutralise_test_cfg(text):
    def fix(m):
        inner = re.sub(r"(?<![\w:\"'])test(?![\w\"':])", "__otcp_test", m.group(2))
        return m.group(1) + inner + m.group(3)
    return re.sub(r"(#!?\[\s*cfg(?:_attr)?\s*\()((?:[^()]|\((?:[^()]|\([^()]*\))*\))*)(\))", fix, text)


CRATE_RX = re.compile(r"(?<![\w$])(?<!::)crate::")
PUBCRATE_RX = re.compile(r"\bpub\s*\(\s*crate\s*\)")


def reach(root, lib_rel):
    lib = os.path.join(root, lib_rel)
    files = {lib}
    for mp, f, d, parent in modtree.tree(lib):
        if f and f != "MISSING-PATH" and os.path.exists(f):
            files.add(f)
    return files


def rewrite_files(files, prefix, neutral, ext_alias=None, pkg=None, pkg_to=None):
    for p in sorted(files):
        t = read(p)
        t2 = CRATE_RX.sub(f"crate::{prefix}::", t)
        if neutral:
            t2 = neutralise_test_cfg(t2)
            t2 = PUBCRATE_RX.sub(f"pub(in crate::{prefix})", t2)
        if pkg:
            t2 = stubgen.rewrite(t2, [pkg]).replace(f"crate::__ext::{pkg}", pkg_to)
        if t2 != t:
            sw(p, t2)


def patch_globs(root, crate_names, glob_names=None, glob_extra=None):
    """frs patch: neutralise glob imports and extern crate lines of external crates (sandbox only); a glob listed in glob_names becomes an explicit import."""
    glob_names = glob_names or {}
    glob_extra = glob_extra or {}
    files = rs_files(root)
    mods = set()
    for p in files:
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", read(p), re.M):
            mods.add(m.group(1))
    keep = KEEP_EXT | mods | set(crate_names)
    rx = re.compile(r"^(\s*)((?:pub(?:\([^)]*\))?\s+)?)use\s+(?:::)?(([A-Za-z_]\w*)(?:::\w+)*)::\*;[ \t]*$", re.M)
    ex = re.compile(r"^(?:[ \t]*#\[[^\n]*\][ \t]*\n)*[ \t]*extern\s+crate\s+(\w+)(?:\s+as\s+\w+)?;[ \t]*$", re.M)

    def globsub(m):
        if m.group(4) in keep:
            return m.group(0)
        names = glob_names.get(m.group(3))
        if names:
            return f"{m.group(1)}{m.group(2)}use {m.group(3)}::{{{', '.join(names)}}};" + "".join(" " + e for e in glob_extra.get(m.group(3), []))
        return m.group(1) + "const _: () = ();"

    for p in files:
        t = read(p)
        t2 = ex.sub(lambda m: m.group(0) if m.group(1) in keep else "", rx.sub(globsub, t))
        if t2 != t:
            sw(p, t2)


def run_rustc(root, lib_rel, crate, extra, feats, crate_type="lib"):
    os.makedirs(OUT, exist_ok=True)
    out = os.path.join(OUT, f"{os.getpid()}.rmeta")
    cfgs = sum([["--cfg", f'feature="{f}"'] for f in feats], [])
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", crate_type, "--crate-name", crate, "--emit=metadata", "-o", out, "--cap-lints", "allow", "--error-format=json", *cfgs, *extra, os.path.join(root, lib_rel)]
    env = dict(os.environ, CARGO_MANIFEST_DIR=root)
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=root, env=env)
    if os.path.exists(out):
        os.remove(out)
    return p.stderr


def diags(stderr, root):
    out = []
    for line in stderr.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") not in ("error",) or str(d.get("message", "")).startswith("aborting") or "previous error" in str(d.get("message", "")):
            continue
        out.append(d)
    return out


def key_of(d, root):
    code = (d.get("code") or {}).get("code") or ""
    spans = [s for s in d.get("spans", []) if s.get("is_primary")]
    label = spans[0].get("label") or "" if spans else ""
    loc = spans[0]["file_name"] if spans else ""
    return (("error[" + code + "]") if code else "error", norm(d["message"]), norm(label)), loc


def seed_stub(c):
    seeds = c.get("seed_found", {})
    mods = c.get("seed_use_modules", {})
    import mirror

    def collect(files, crates):
        found = ORIG_COLLECT(files, crates)
        for crate, lists in seeds.items():
            for segs in lists:
                found.setdefault(crate, {}).setdefault(tuple(segs), "item")
        for crate, modnames in mods.items():
            for p in files:
                t = mask(read(p))
                for mm in re.finditer(r"(?:^|[;{}\s])use\s+([^;]+);", t):
                    for full, nm in mirror.flatten(mm.group(1)):
                        for i, seg in enumerate(full):
                            if seg in modnames and i + 1 < len(full) and full[-1] != "*":
                                found.setdefault(crate, {}).setdefault(tuple(full[i:]), "item")
                                break
        traitnames = set(stubgen.HINT_TRAITS)
        for crate, d in found.items():
            for segs in list(d):
                if len(segs) > 1 and segs[-2] in traitnames:
                    d.pop(segs, None)
                    d[tuple(segs[:-1])] = "trait"
        return found
    stubgen.collect = collect


ORIG_COLLECT = stubgen.collect


def build_baseline(cid, label="A"):
    c = cfg_of(cid)
    seed_stub(c)
    stubgen.HINT_TRAITS.update(c.get("traits", []))
    root = f"{W}/{cid}/{label}"
    under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    populate(cid, root, source_root=f"{W}/{cid}/snap")
    patch_globs(root, [c["pkg"]], c.get("glob_names"), c.get("glob_extra"))
    if c.get("base_prelude"):
        lp = os.path.join(root, c["p_lib"])
        sw(lp, read(lp) + "\n" + "\n".join(c["base_prelude"]) + "\n")
    stubgen.apply([root], c["p_lib"], c["ext"], c.get("extra_stub", ""))
    return root


def build_current(cid, label="B"):
    c = cfg_of(cid)
    seed_stub(c)
    stubgen.HINT_TRAITS.update(c.get("traits", []))
    root = f"{W}/{cid}/{label}"
    under(root)
    if os.path.exists(root):
        shutil.rmtree(root)
    populate(cid, root)
    for comp in c["comps"]:
        copy_rels([comp["lib"]], root)
    plib_b = c.get("p_lib_cur", c["p_lib"])
    pset = reach(root, plib_b)
    csets = [reach(root, comp["lib"]) for comp in c["comps"]]
    rewrite_files(pset, "prod", True)
    for i, cs in enumerate(csets):
        rewrite_files(cs - pset, f"comp{i}", False, pkg=c.get("pkg_cur", c["pkg"]), pkg_to="crate::prod")
    union = f"{root}/__otcp_union.rs"
    body = f'#[path = "{os.path.relpath(os.path.join(root, plib_b), root)}"]\npub mod prod;\n'
    for i, comp in enumerate(c["comps"]):
        body += f'#[path = "{comp["lib"]}"]\nmod comp{i};\n'
    sw(union, body)
    patch_globs(root, ["prod"] + [f"comp{i}" for i in range(len(c["comps"]))], c.get("glob_names"), c.get("glob_extra"))
    if c.get("union_prelude"):
        sw(union, "".join(l + "\n" for l in c["union_prelude"]) + read(union))
    stubgen.apply([root], "__otcp_union.rs", c["ext"] + c.get("comp_ext", []), c.get("extra_stub", ""))
    return root


def run(cid, which=("test",), as_json=False, label_out="run"):
    c = cfg_of(cid)
    A = build_baseline(cid)
    B = build_current(cid)
    res = {}
    for cfgname in which:
        extra = CFGS[cfgname]
        ea = diags(run_rustc(A, c["p_lib"], "crate_a", extra, c.get("base_features", []), c.get("crate_type_a", "lib")), A)
        eb = diags(run_rustc(B, "__otcp_union.rs", "crate_b", extra, c.get("features", ["testkit"])), B)
        ka = collections.Counter(key_of(d, A)[0] for d in ea)
        kb = collections.Counter(key_of(d, B)[0] for d in eb)
        onlyA, onlyB = ka - kb, kb - ka
        status = "IDENTICAL" if not onlyA and not onlyB else "DIFFERENT"
        print(f"== {cfgname}: baseline errors={sum(ka.values())} current errors={sum(kb.values())} {status}")
        for (h, m, l), n in sorted(onlyB.items()):
            print(f"   +current {n}x {h}: {m[:150]} | {l[:90]}")
        for (h, m, l), n in sorted(onlyA.items()):
            print(f"   -current {n}x {h}: {m[:150]} | {l[:90]}")
        res[cfgname] = (ea, eb, onlyA, onlyB)
        write(f"{F}/🗑️generated/otcp-{cid}-{label_out}-{cfgname}-B.json", json.dumps(eb, ensure_ascii=False))
        write(f"{F}/🗑️generated/otcp-{cid}-{label_out}-{cfgname}-A.json", json.dumps(ea, ensure_ascii=False))
    return res, A, B


if __name__ == "__main__":
    cid = sys.argv[1]
    cmd = sys.argv[2]
    if cmd == "snapshot":
        snapshot(cid, "--force" in sys.argv)
    elif cmd == "baseline":
        c = cfg_of(cid)
        A = build_baseline(cid)
        for cfgname in (sys.argv[3].split(",") if len(sys.argv) > 3 else ["test"]):
            ea = diags(run_rustc(A, c["p_lib"], "crate_a", CFGS[cfgname], c.get("base_features", [])), A)
            cnt = collections.Counter(((d.get("code") or {}).get("code") or "-") for d in ea)
            print(cfgname, "errors", len(ea), dict(cnt))
            for d in ea[:8]:
                print("   ", d["message"][:140])
    elif cmd == "run":
        which = tuple(sys.argv[3].split(",")) if len(sys.argv) > 3 and not sys.argv[3].startswith("--") else ("test",)
        run(cid, which)
