#!/usr/bin/env python3
"""🔎 plugfix static sweep: per plugin crate, a sandbox copy of the live source tree, `extern crate` lines stripped and external globs neutralised, then
`rustc +nightly-2026-07-20 --emit=metadata -o /dev/null` (lib and --test) and a classification of the resolve-level diagnostics that do not depend on externals:
duplicate definitions (E0428 E0252 E0254 E0255 E0201 E0592), undeclared first path segments that are neither a dependency nor an alias (E0433), unresolved local imports (E0432),
missing module files (E0583), private local items (E0603) and unresolved names that the crate itself defines (E0412 E0425 E0422 E0423 E0405 E0531 E0532 E0574 E0433 E0432).
usage: sweep.py [--mutate FILE] [--cfgs lib,test] <manifest-dir-relative-to-repo>...   writes only below /Users/ueli/.cache/plugfix"""
import collections, json, os, re, shutil, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
SBX = "/Users/ueli/.cache/plugfix/sbx"
TOOLCHAIN = "+nightly-2026-07-20"
DUP = {"E0428", "E0252", "E0254", "E0255", "E0259", "E0260", "E0201", "E0592", "E0124"}
NAMES = {"E0412", "E0425", "E0422", "E0423", "E0405", "E0531", "E0532", "E0574", "E0433", "E0432", "E0407", "E0426"}
IGNORE = shutil.ignore_patterns("target", "node_modules", ".git", "🗑️generated", "dist", "pkg", "📦️packages")
STD = {"std", "core", "alloc", "proc_macro", "test", "crate", "self", "super", "Self"}
RX_EXT = re.compile(r"^\s*(?:#\[[^\]]*\]\s*)?extern\s+crate\s+([A-Za-z_]\w*)(?:\s+as\s+([A-Za-z_]\w*))?;[ \t]*$", re.M)
RX_GLOB = re.compile(r"^(\s*)((?:#\[[^\]]*\]\s*)*)(?:pub(?:\([^)]*\))?\s+)?use\s+(::)?([A-Za-z_]\w*)(?:::\w+)*::\*;[ \t]*$", re.M)
RX_PATH = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
RX_INC = re.compile(r'include(?:_str|_bytes)?!\(\s*(?:concat!\(\s*)?"([^"]+)"')


def norm(name):
    return name.replace("-", "_")


def manifest_info(mdir):
    m = os.path.join(REPO, mdir, "Cargo.toml")
    t = open(m, encoding="utf-8").read()
    name = norm(re.search(r'^name\s*=\s*"([^"]+)"', t, re.M).group(1))
    pm = re.search(r"\[lib\][^\[]*?path\s*=\s*\"([^\"]+)\"", t, re.S)
    lib = os.path.normpath(os.path.join(REPO, mdir, pm.group(1))) if pm else os.path.join(REPO, mdir, "src", "lib.rs")
    deps = set()
    for sec in re.finditer(r"^\[((?:dev-|build-)?dependencies)\]\n(.*?)(?=^\[|\Z)", t, re.M | re.S):
        for line in sec.group(2).splitlines():
            km = re.match(r"\s*([A-Za-z0-9_-]+)(?:\.[a-z-]+)?\s*=", line)
            if km:
                deps.add(norm(km.group(1)))
    for sec in re.finditer(r"^\[target\.[^\]]*dependencies\]\n(.*?)(?=^\[|\Z)", t, re.M | re.S):
        for line in sec.group(1).splitlines():
            km = re.match(r"\s*([A-Za-z0-9_-]+)(?:\.[a-z-]+)?\s*=", line)
            if km:
                deps.add(norm(km.group(1)))
    feats = set()
    fm = re.search(r"^\[features\]\n(.*?)(?=^\[|\Z)", t, re.M | re.S)
    if fm:
        for line in fm.group(1).splitlines():
            km = re.match(r"\s*([A-Za-z0-9_-]+)\s*=", line)
            if km:
                feats.add(km.group(1))
    return name, lib, deps, feats


CACHE_DIR = "/Users/ueli/.cache/plugfix"


def manifests():
    for top in os.listdir(REPO):
        if top.startswith(".") or not os.path.isdir(os.path.join(REPO, top)):
            continue
        for dp, dn, fn in os.walk(os.path.join(REPO, top)):
            dn[:] = [x for x in dn if x not in ("target", "node_modules", ".git", "🗑️generated", "dist")]
            if "Cargo.toml" in fn:
                yield dp, open(os.path.join(dp, "Cargo.toml"), encoding="utf-8", errors="ignore").read()


def cached(name, build):
    path = os.path.join(CACHE_DIR, name)
    if os.path.exists(path):
        return json.load(open(path, encoding="utf-8"))
    os.makedirs(CACHE_DIR, exist_ok=True)
    value = build()
    json.dump(value, open(path, "w", encoding="utf-8"), ensure_ascii=False)
    return value


def libnames():
    """package name (underscored) -> `[lib] name` for packages whose extern prelude name differs"""
    def build():
        out = {}
        for _, t in manifests():
            pn = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
            ln = re.search(r'^\[lib\]\n(?:[^\[]*?\n)?name\s*=\s*"([^"]+)"', t, re.M)
            if pn and ln:
                out[norm(pn.group(1))] = ln.group(1)
        return out
    return cached("libnames.json", build)


def libroots():
    """repo-relative directory of a crate's lib root -> manifest directories (repo-relative) that compile it"""
    def build():
        out = {}
        for dp, t in manifests():
            if "[package]" not in t:
                continue
            pm = re.search(r"\[lib\][^\[]*?path\s*=\s*\"([^\"]+)\"", t, re.S)
            lib = os.path.normpath(os.path.join(dp, pm.group(1))) if pm else os.path.join(dp, "src", "lib.rs")
            out.setdefault(os.path.relpath(os.path.dirname(lib), REPO), []).append(os.path.relpath(dp, REPO))
        return out
    return cached("libroots.json", build)


def sandbox_path(sid):
    return os.path.join(SBX, sid)


def copy_closure(lib, sb):
    tree = os.path.dirname(lib)
    rel = os.path.relpath(tree, REPO)
    shutil.copytree(tree, os.path.join(sb, rel), symlinks=True, ignore=IGNORE)
    seen = set()
    changed = True
    while changed:
        changed = False
        for dp, dn, fn in os.walk(sb):
            for f in fn:
                p = os.path.join(dp, f)
                if p in seen or not (f.endswith(".rs")):
                    continue
                seen.add(p)
                t = open(p, encoding="utf-8", errors="ignore").read()
                for m in list(RX_PATH.finditer(t)) + list(RX_INC.finditer(t)):
                    target = os.path.normpath(os.path.join(os.path.dirname(p), m.group(1)))
                    relp = os.path.relpath(target, sb)
                    if relp.startswith(".."):
                        continue
                    src = os.path.join(REPO, relp)
                    if not os.path.exists(target) and os.path.isfile(src):
                        os.makedirs(os.path.dirname(target), exist_ok=True)
                        shutil.copyfile(src, target)
                        changed = True


def patch_tree(sb, mutate):
    aliases, mods, texts = set(), set(), {}
    for dp, dn, fn in os.walk(sb):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                t = open(p, encoding="utf-8", errors="ignore").read()
                texts[p] = t
                for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)", t, re.M):
                    mods.add(m.group(1))
    keep = {"crate", "super", "self"} | mods | STD
    nglob = 0
    for p, t in texts.items():
        for m in RX_EXT.finditer(t):
            aliases.add(m.group(2) or m.group(1))
        t2 = RX_EXT.sub("", t)

        def sub(m):
            nonlocal nglob
            if m.group(4) in keep:
                return m.group(0)
            nglob += 1
            return m.group(1) + "const _: () = ();"

        t2 = RX_GLOB.sub(sub, t2)
        for old, new in mutate.get(os.path.relpath(p, sb), []):
            assert old in t2, ("mutation anchor missing", p, old)
            t2 = t2.replace(old, new)
        if t2 != t:
            open(p, "w", encoding="utf-8").write(t2)
    return aliases, texts, nglob


def defined_names(texts):
    names = set()
    rx = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|unsafe|const|extern(?:\s+\"[^\"]*\")?)\s+)*(?:fn|struct|enum|trait|type|union|static|mod|macro_rules!|const)\s+([A-Za-z_]\w*)", re.M)
    for t in texts.values():
        for m in rx.finditer(t):
            names.add(m.group(1))
        for m in re.finditer(r"^\s*(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]*?;", t, re.M | re.S):
            for w in re.findall(r"(?:\bas\s+|::)([A-Za-z_]\w*)\s*(?=[,;}]|$)", m.group(0)):
                names.add(w)
    return names


def run_rustc(crate, lib_in_sb, cfg, feats):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--error-format=json", "--cap-lints", "allow"]
    if cfg == "test":
        cmd.append("--test")
    for f in sorted(feats):
        cmd += ["--cfg", f'feature="{f}"']
    cmd.append(lib_in_sb)
    p = subprocess.run(cmd, capture_output=True, text=True, cwd=os.path.dirname(lib_in_sb))
    out = []
    for line in p.stderr.splitlines():
        if not line.startswith("{"):
            continue
        try:
            out.append(json.loads(line))
        except json.JSONDecodeError:
            pass
    return out


def primary(d):
    for s in d.get("spans", []):
        if s.get("is_primary"):
            return s
    return (d.get("spans") or [None])[0]


def main():
    args = sys.argv[1:]
    mutate, cfgs, raw = {}, ["lib", "test"], "/Users/ueli/.cache/plugfix/raw"
    while args and args[0].startswith("--"):
        a = args.pop(0)
        if a == "--mutate":
            mutate = json.load(open(args.pop(0), encoding="utf-8"))
        elif a == "--cfgs":
            cfgs = args.pop(0).split(",")
        elif a == "--raw":
            raw = args.pop(0)
    os.makedirs(raw, exist_ok=True)
    for mdir in args:
        crate, lib, deps, feats = manifest_info(mdir)
        sid = re.sub(r"\W", "", crate)[-60:] + str(sum(map(ord, mdir)) % 10000)
        sb = sandbox_path(sid)
        shutil.rmtree(sb, ignore_errors=True)
        os.makedirs(sb)
        copy_closure(lib, sb)
        aliases, texts, nglob = patch_tree(sb, mutate.get(mdir, {}))
        known = deps | aliases | STD
        defined = defined_names(texts)
        lib_sb = os.path.join(sb, os.path.relpath(lib, REPO))
        res = {"crate": crate, "mdir": mdir, "files": len(texts), "externs": len(aliases), "globs": nglob, "known": sorted(known), "defined": sorted(defined), "errors": {}}
        for cfg in cfgs:
            diags = run_rustc(crate, lib_sb, cfg, set())
            out, seen = [], set()
            for d in diags:
                if d.get("level") != "error":
                    continue
                sp = primary(d)
                where = f'{os.path.relpath(sp["file_name"], sb) if sp and not sp["file_name"].startswith("<") else "?"}:{sp["line_start"] if sp else 0}'
                item = ((d.get("code") or {}).get("code"), d.get("message", ""), where)
                if item not in seen:
                    seen.add(item)
                    out.append(item)
            res["errors"][cfg] = out
        shutil.rmtree(sb, ignore_errors=True)
        json.dump(res, open(os.path.join(raw, crate + "-" + str(sum(map(ord, mdir)) % 10000) + ".json"), "w", encoding="utf-8"), ensure_ascii=False)
        print(crate, {c: len(v) for c, v in res["errors"].items()}, flush=True)


if __name__ == "__main__":
    main()
