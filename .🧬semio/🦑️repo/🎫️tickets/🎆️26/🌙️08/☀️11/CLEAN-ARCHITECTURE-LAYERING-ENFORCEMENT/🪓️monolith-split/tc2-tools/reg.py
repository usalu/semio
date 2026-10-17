"""📋 tc2: registers a companion test crate: artifact workspace `members` (byte-sorted), artifact `Cargo.lock` package entry, `📋️build-owners.json` row (name-sorted), artifact `📦️packages/🦀️rust/📜️script.ts` `testPackages`.
usage: reg.py <spec.json> [--apply]"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/ttc")
from ttclib import *
import fixrt

OWNERS = ROOT + "/📋️build-owners.json"


def comp_paths(spec):
    comp_dir = (spec["dir"] if spec["dir"].startswith("/") else ROOT + "/" + spec["dir"])
    return comp_dir, comp_dir + "/📦️packages/🦀️rust"


def ws_of(spec):
    mans = Manifests(Source(None))
    mans.build_index()
    man = mans.manifest(spec["parent"], Source(None))
    sec = man["sections"].get("package", {})
    mdir = ROOT + "/" + man["dir"]
    return os.path.normpath(os.path.join(mdir, sec.get("workspace", '"."').strip().strip('"'))), man


def read_manifest_deps(pkg_dir, wsdir):
    text = open(pkg_dir + "/Cargo.toml", encoding="utf-8").read()
    sec = parse_toml(text)
    wtab = parse_toml(open(wsdir + "/Cargo.toml", encoding="utf-8").read())
    wdeps = {}
    for k, v in wtab.items():
        if k == "workspace.dependencies":
            for kk, vv in v.items():
                wdeps[kk] = vv
        elif k.startswith("workspace.dependencies."):
            wdeps[k[len("workspace.dependencies."):].strip('"')] = "{ " + ", ".join(f"{a} = {b}" for a, b in v.items()) + " }"
    names = []
    for kind, name, specstr in dep_sections(sec):
        mm = re.search(r'package\s*=\s*"([^"]+)"', specstr)
        if mm:
            names.append(mm.group(1))
        elif "workspace" in specstr and name in wdeps:
            mw = re.search(r'package\s*=\s*"([^"]+)"', wdeps[name])
            names.append(mw.group(1) if mw else name)
        else:
            names.append(name)
    return sorted(set(names))


def lock_token(lock_text, pkg_name):
    versions = re.findall(r'\[\[package\]\]\nname = "' + re.escape(pkg_name) + r'"\nversion = "([^"]+)"', lock_text)
    if len(versions) == 1:
        return pkg_name
    if not versions:
        print(f"WARN package {pkg_name} not in the lock (stale lock at base); entry written by name")
        return pkg_name
    parent_tokens = None
    raise SystemExit(f"ambiguous lock versions for {pkg_name}: {versions}")


def register(spec, apply, force=False):
    wsdir, pman = ws_of(spec)
    comp_dir, pkg_dir = comp_paths(spec)
    name = spec["name"]
    out = []
    members_rel = os.path.relpath(pkg_dir, wsdir)
    cargo = wsdir + "/Cargo.toml"
    text = open(cargo, encoding="utf-8").read()
    m = re.search(r"members = \[\n(.*?)\n\]", text, re.S)
    rows = [r.strip().rstrip(",").strip('"') for r in m.group(1).split("\n") if r.strip()]
    if members_rel not in rows:
        rows.append(members_rel)
        rows.sort(key=lambda s: s.encode("utf-8"))
        new = "members = [\n" + "\n".join(f'    "{r}",' for r in rows) + "\n]"
        text2 = text[:m.start()] + new + text[m.end():]
        out.append(("members", cargo, text2, text))
    lock = wsdir + "/Cargo.lock"
    ltext = open(lock, encoding="utf-8").read()
    if force and f'name = "{name}"' in ltext:
        mm = re.search(r'\[\[package\]\]\nname = "' + re.escape(name) + r'"\nversion = "[^"]*"\ndependencies = \[\n(?: "[^"]+",\n)*\]\n\n', ltext)
        ltext = ltext[:mm.start()] + ltext[mm.end():]
    if f'name = "{name}"' not in ltext:
        deps = read_manifest_deps(pkg_dir, wsdir)
        toks = sorted(set(lock_token(ltext, d) for d in deps), key=lambda s: s)
        block = "[[package]]\nname = \"" + name + "\"\nversion = \"0.1.0\"\ndependencies = [\n" + "\n".join(f' "{t}",' for t in toks) + "\n]\n\n"
        pkgs = [(mm.group(1), mm.start()) for mm in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"', ltext)]
        at = None
        for pname, pos in pkgs:
            if pname > name:
                at = pos
                break
        at = len(ltext) if at is None else at
        ltext2 = ltext[:at] + block + ltext[at:]
        out.append(("lock", lock, ltext2, ltext))
    owners = open(OWNERS, encoding="utf-8").read()
    if f'"name": "{name}"' not in owners:
        row = f'    {{\n      "name": "{name}",\n      "root": "{os.path.relpath(pkg_dir, ROOT)}",\n      "manifest": "{os.path.relpath(pkg_dir, ROOT)}/Cargo.toml",\n      "packages": []\n    }},\n'
        rows_ = [(mm.group(1), mm.start()) for mm in re.finditer(r'    \{\n      "name": "([^"]+)"', owners)]
        at = None
        for rname, pos in rows_:
            if rname.encode("utf-8") > name.encode("utf-8"):
                at = pos
                break
        assert at is not None
        owners2 = owners[:at] + row + owners[at:]
        out.append(("owners", OWNERS, owners2, owners))
    if apply:
        for kind, path, new, old in out:
            fixrt.save_orig(path)
            open(path, "w", encoding="utf-8").write(new)
    return [(k, p) for k, p, _, _ in out]


def script_test_packages(artifact_pkg_dir, root_pkg, add, apply):
    path = artifact_pkg_dir + "/📜️script.ts"
    t = open(path, encoding="utf-8").read()
    mm = re.search(r"testPackages\s*:\s*\[([^\]]*)\]", t)
    if mm:
        have = re.findall(r'"([^"]+)"', mm.group(1))
        want = have + [a for a in add if a not in have]
        new = "testPackages: [" + ", ".join(f'"{x}"' for x in want) + "]"
        t2 = t[:mm.start()] + new + t[mm.end():]
    else:
        anchor = f'await runArtifactRustPackageMain(import.meta.dir, "{root_pkg}", {{'
        assert t.count(anchor) == 1, ("anchor", t.count(anchor), path)
        want = [root_pkg] + list(add)
        t2 = t.replace(anchor, anchor + " testPackages: [" + ", ".join(f'"{x}"' for x in want) + "],")
    if apply and t2 != t:
        fixrt.save_orig(path)
        open(path, "w", encoding="utf-8").write(t2)
    return t2 != t


if __name__ == "__main__":
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    res = register(spec, "--apply" in sys.argv, "--force" in sys.argv)
    print("registered:", res)
