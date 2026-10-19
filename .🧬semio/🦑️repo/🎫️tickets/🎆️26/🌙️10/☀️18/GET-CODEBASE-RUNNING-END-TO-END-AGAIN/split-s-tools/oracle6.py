#!/usr/bin/env python3
"""🔬 C5B scoped patched differential (descendant of c4p47-tools/oracle.py, RESOLUTION-ORACLE.md): per owning crate, a sandbox copy with the ORIGINAL file restored (snapshot, children removed) against a sandbox copy of the live tree;
`rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` from the crate lib root; extern crate lines of missing crates removed and external globs neutralised identically on both sides; error and warning multisets compared.
usage: oracle5.py <group> [cfg,...]   groups: see GROUPS. Sandboxes: /Users/ueli/.cache/c7w/sbx. Writes only inside the sandbox."""
import collections, json, os, re, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
SBX = "/Users/ueli/.cache/c7w/sbx"
OUT = "/Users/ueli/.cache/c7w/orc"
TOOLCHAIN = "+nightly-2026-07-20"
PL = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
files_tsv = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
T = PL + "/🧪️tests/"
C = PL + "/🧷️companions/"
def tdir(tag):
    return os.path.dirname(files_tsv[tag])
import json as _json
S6 = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
CRATES = _json.load(open(HERE + "/crates.json", encoding="utf-8"))


def crate_info(name):
    d = CRATES[name]
    toml = open(os.path.join(REPO, d, "📦️packages", "🦀️rust", "Cargo.toml"), encoding="utf-8").read()
    lib = re.search(r"^\[lib\][^\[]*?path\s*=\s*\"([^\"]+)\"", toml, re.M | re.S).group(1)
    root = os.path.normpath(os.path.join(d, "📦️packages", "🦀️rust", lib))
    feats = [m.group(1) for m in re.finditer(r"^([A-Za-z0-9_-]+)\s*=\s*\[", toml, re.M)]
    return dict(dir=d, root=root, features=feats)


def tags_of(name):
    d = CRATES[name]
    out = []
    for tag, rel in files_tsv.items():
        if rel.startswith(d + "/") and os.path.isfile(os.path.join(REPO, rel)):
            owner = None
            for n, dd in sorted(CRATES.items(), key=lambda kv: -len(kv[1])):
                if rel.startswith(dd + "/"):
                    owner = n
                    break
            if owner == name:
                out.append(tag)
    return out


class _G(dict):
    pass


def make_group(name):
    i = crate_info(name)
    copy = [i["dir"]]
    rd = os.path.dirname(i["root"])
    if not (rd + "/").startswith(i["dir"] + "/"):
        copy.append(rd)
    return dict(crate=name.replace("-", "_"), copy=copy, root=i["root"], tags=tags_of(name), cfgs=["lib", "test"])


class _Cfg(dict):
    def __missing__(self, key):
        test = key.startswith("test")
        feats = key.split("+", 1)[1].split(",") if "+" in key else []
        args = (["--test"] if test else [])
        for f in feats:
            args += ["--cfg", 'feature="%s"' % f]
        return args


CONFIGS = _Cfg()
_base_ignore = shutil.ignore_patterns("target", "node_modules", ".git", "🗑️generated", "dist", "pkg")


def _ignore_rs(d, names):
    skip = set(_base_ignore(d, names))
    for n in names:
        p = os.path.join(d, n)
        if n not in skip and not os.path.isdir(p) and not n.endswith(".rs"):
            skip.add(n)
    return skip


def under(path):
    rp, rr = os.path.realpath(path), os.path.realpath(SBX)
    assert rp == rr or rp.startswith(rr + os.sep), ("write outside the sandbox", path)


def write(path, text):
    under(os.path.dirname(path))
    open(path, "w", encoding="utf-8").write(text)


def copy_tree(g, dst):
    under(dst)
    shutil.rmtree(dst, ignore_errors=True)
    for c in g["copy"]:
        src, out = os.path.join(REPO, c), os.path.join(dst, c)
        if os.path.isdir(src):
            shutil.copytree(src, out, symlinks=True, ignore=_ignore_rs)


def child_folders(tag):
    if tag == "proj":
        import projsplit
        projsplit.GROUPS[:] = projsplit.build_groups(projsplit.parse(open(f"{HERE}/orig/proj.rs", encoding="utf-8").read())[3])
        return [g[0] + projsplit.V + g[1] for g in projsplit.GROUPS]
    import importlib
    sys.path.insert(0, HERE)
    try:
        import modsplit
        return modsplit.rel_folders(importlib.import_module("specs." + tag).SPEC)
    except (ModuleNotFoundError, KeyError):
        p = f"{HERE}/staged-{tag}.json"
        return json.load(open(p, encoding="utf-8"))["folders"] if os.path.exists(p) else []


def baseline_into(g, dst):
    for tag in g["tags"]:
        rel = files_tsv[tag]
        p = os.path.join(dst, rel)
        under(os.path.dirname(p))
        shutil.copyfile(f"{HERE}/orig/{tag}.rs", p)
        for f in child_folders(tag):
            d = os.path.join(os.path.dirname(p), f)
            under(d)
            shutil.rmtree(d, ignore_errors=True)


MODS = {}


def neutralize_text(t, mods):
    keep = {"crate", "super", "self"} | mods
    rx_ext = re.compile(r"^\s*extern\s+crate\s+\w+\s+as\s+\w+;[ \t]*$", re.M)
    rx_glob = re.compile(r"^(\s*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
    n = [0, 0]
    t2 = rx_ext.sub(lambda m: (n.__setitem__(0, n[0] + 1), "")[1], t)
    def sub(m):
        if m.group(3) in keep:
            return m.group(0)
        n[1] += 1
        return m.group(1) + "const _: () = ();"
    return rx_glob.sub(sub, t2), n


def neutralize(tree):
    files, mods, texts = [], set(), {}
    for dp, dn, fn in os.walk(tree):
        for f in fn:
            if f.endswith(".rs"):
                files.append(os.path.join(dp, f))
    for p in files:
        t = open(p, encoding="utf-8", errors="ignore").read()
        texts[p] = t
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", t, re.M):
            mods.add(m.group(1))
    MODS[tree] = mods
    tot = [0, 0]
    for p, t in texts.items():
        t2, n = neutralize_text(t, mods)
        tot[0] += n[0]; tot[1] += n[1]
        if t2 != t:
            write(p, t2)
    return tot


def pull_missing(tree, err):
    pulled = 0
    for m in list(re.finditer(r"couldn't find file `([^`]+)`", err)) + list(re.finditer(r"couldn't read `([^`]+)`", err)):
        path = os.path.normpath(m.group(1))
        prefix = os.path.realpath(tree)
        if not (path.startswith(tree + os.sep) or path.startswith(prefix + os.sep)):
            continue
        rel = path[len(tree) + 1:] if path.startswith(tree + os.sep) else path[len(prefix) + 1:]
        src = os.path.join(REPO, rel)
        if os.path.isfile(src) and not os.path.exists(path):
            under(os.path.dirname(path))
            os.makedirs(os.path.dirname(path), exist_ok=True)
            shutil.copyfile(src, path)
            if path.endswith(".rs"):
                t = open(path, encoding="utf-8", errors="ignore").read()
                t2, _ = neutralize_text(t, MODS.get(tree, set()))
                if t2 != t:
                    write(path, t2)
            pulled += 1
    return pulled


def run(g, tree, cfg, label, tag):
    root = os.path.join(tree, g["root"])
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", g["crate"], "--emit=metadata", "-o", "/dev/null", "--error-format=short", "-W", "unused-imports"] + CONFIGS[cfg] + [root]
    os.makedirs(OUT, exist_ok=True)
    for attempt in range(80):
        env = dict(os.environ, CARGO_MANIFEST_DIR=os.path.join(os.path.dirname(root), "📦️packages", "🦀️rust"))
        p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(root), env=env)
        if ("couldn't find file" in p.stderr or "couldn't read" in p.stderr) and pull_missing(tree, p.stderr):
            continue
        break
    open(os.path.join(OUT, "%s-%s-%s.err" % (tag, label, cfg)), "w", encoding="utf-8").write(p.stderr)
    return p.stderr


def parse(err):
    errs, warns = collections.Counter(), collections.Counter()
    for line in err.split("\n"):
        m = re.match(r"^(?:[^\s:][^:]*:\d+:\d+: )?(error(?:\[E\d+\])?|warning): (.*)$", line)
        if not m:
            continue
        kind, msg = m.group(1), re.sub(r"/dev/rmeta\w+", "/dev/rmeta", m.group(2))
        if os.environ.get("C5B_NORM"):
            msg = re.sub(r"\b(?:super::)+super\b", "super", msg)
            msg = re.sub(r"\b(?:[a-z_][a-z0-9_]*::)+(?=[A-Za-z_])", "", msg)
        if msg.startswith("aborting due to") or re.search(r"generated \d+ warning", msg):
            continue
        (errs if kind.startswith("error") else warns)[kind + ": " + msg] += 1
    return errs, warns


def main():
    tag = sys.argv[1]
    g = make_group(tag)
    cfgs = sys.argv[2].split(",") if len(sys.argv) > 2 else g["cfgs"]
    A, B = os.path.join(SBX, tag, "orig"), os.path.join(SBX, tag, "new")
    os.makedirs(os.path.join(SBX, tag), exist_ok=True)
    copy_tree(g, A); baseline_into(g, A); copy_tree(g, B)
    print("neutralized extern/glob", neutralize(A), neutralize(B))
    bad = 0
    for cfg in cfgs:
        ea, eb = run(g, A, cfg, "orig", tag), run(g, B, cfg, "new", tag)
        a_e, a_w = parse(ea); b_e, b_w = parse(eb)
        oa, ob, wa, wb = a_e - b_e, b_e - a_e, a_w - b_w, b_w - a_w
        print("== %s %s: errors orig=%d new=%d only-orig=%d only-new=%d | warnings orig=%d new=%d only-orig=%d only-new=%d" % (tag, cfg, sum(a_e.values()), sum(b_e.values()), sum(oa.values()), sum(ob.values()), sum(a_w.values()), sum(b_w.values()), sum(wa.values()), sum(wb.values())))
        for k, v in sorted(oa.items())[:25]: print("  -orig %dx %s" % (v, k[:200]))
        for k, v in sorted(ob.items())[:25]: print("  +new  %dx %s" % (v, k[:200]))
        for k, v in sorted(wb.items())[:15]: print("  +warn %dx %s" % (v, k[:180]))
        for k, v in sorted(wa.items())[:15]: print("  -warn %dx %s" % (v, k[:180]))
        if ob or wb: bad += 1
    print("ORACLE", tag, "CLEAN" if not bad else "DIFFERENCES")


if __name__ == "__main__":
    main()
