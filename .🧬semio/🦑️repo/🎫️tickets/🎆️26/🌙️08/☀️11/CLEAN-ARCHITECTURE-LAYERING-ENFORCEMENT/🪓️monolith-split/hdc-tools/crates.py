"""Maps hub Rust files to their Cargo crate and reports whether the crate depends on semio-hub-directory."""
import os, re, sys, json, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from scope import *


def cargo_files():
    a = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--", "🌎️hub/*Cargo.toml"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    b = subprocess.run(["git", "-c", "core.quotepath=false", "-c", "core.fsmonitor=false", "ls-files", "--others", "--exclude-standard", "--", "🌎️hub"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    return sorted({f for f in a + b if f.endswith("Cargo.toml") and f and "node_modules" not in f and os.path.exists(os.path.join(ROOT, f))})


def parse_cargo(path):
    t = read(path)
    name = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M)
    roots = []
    for m in re.finditer(r'^\[(lib|\[bin\]|\[test\]|bin|test)\]?\]?\s*\n((?:(?!\[).*\n)*)', t, re.M):
        pass
    sec = None
    for line in t.split("\n"):
        s = line.strip()
        if s.startswith("["):
            sec = s
            continue
        if sec in ("[lib]", "[[bin]]", "[[test]]") and s.startswith("path"):
            mm = re.match(r'path\s*=\s*"([^"]+)"', s)
            if mm:
                roots.append(os.path.normpath(os.path.join(os.path.dirname(path), mm.group(1))))
    sections = {}
    cur = None
    for line in t.split("\n"):
        s = line.strip()
        if s.startswith("["):
            cur = s
            sections.setdefault(cur, [])
        elif cur:
            sections[cur].append(s)
    def dep(sec):
        return [l.split("=")[0].strip() for l in sections.get(sec, []) if "=" in l]
    return dict(name=name.group(1) if name else None, roots=roots, deps=dep("[dependencies]"), devdeps=dep("[dev-dependencies]"))


def main():
    files, texts, parent = build()
    root_of = {}
    crates = {}
    for cf in cargo_files():
        p = os.path.join(ROOT, cf)
        info = parse_cargo(p)
        info["cargo"] = cf
        if not info["name"]:
            continue
        for r in info["roots"]:
            root_of[r] = info
        crates[info["name"]] = info

    def crate_of(p, seen=()):
        if p in root_of:
            return {root_of[p]["name"]}
        res = set()
        for q in parent.get(p, []):
            if q not in seen:
                res |= crate_of(q, seen + (p,))
        return res

    plan = json.load(open(SC + "/hdc-callers-plan.json"))
    out = []
    for r in plan:
        p = os.path.join(ROOT, r["file"])
        cs = crate_of(p)
        for c in sorted(cs):
            info = crates[c]
            has = "semio-hub-directory" in info["deps"]
            hasdev = "semio-hub-directory" in info["devdeps"]
            out.append((r["file"], c, has, hasdev, info["cargo"]))
    for o in out:
        print(("OK  " if o[2] else ("DEV " if o[3] else "MISS")), o[1], "|", o[0])
    json.dump(out, open(SC + "/hdc-crates.json", "w"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
