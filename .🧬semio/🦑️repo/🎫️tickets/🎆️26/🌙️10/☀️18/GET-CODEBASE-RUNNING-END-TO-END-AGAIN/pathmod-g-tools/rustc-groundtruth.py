#!/usr/bin/env python3 -I
"""🔬 rustc-groundtruth.py <census.json> <out.json> [--only NAME,...] [--jobs N] : the COMPILER as oracle for the Rust source set of every crate. For each crate entry point (lib, bins, tests, benches, examples, build script) `rustc --emit=dep-info` (parse + module loading + include expansion, no analysis, errors tolerated) is run with every Cargo feature of the manifest enabled and, for the named flavor, with `--test`; the union of the files in the dep-info is compared with the files the LIVE derivation declares (a crate that fell back declares `<dir>/**/*`). Streams, RSS guard 500 MB, runs from a scratch directory so no toolchain file of the checkout applies. Output: per-crate gaps."""
import json, os, re, resource, subprocess, sys, tempfile, concurrent.futures as cf
W = "/Users/ueli/Documents/semio"
TOOLCHAIN = "nightly-2026-07-20"
args = sys.argv[1:]
census_path, out_path = args[0], args[1]
only = set(args[args.index("--only") + 1].split(",")) if "--only" in args else None
jobs = int(args[args.index("--jobs") + 1]) if "--jobs" in args else 3
rows = [r for r in json.load(open(census_path, encoding="utf-8")) if "name" in r and (only is None or r["name"] in only)]
def toml_section(text, name):
    m = re.search(r"^\[" + re.escape(name) + r"\]\s*\n((?:(?!\[).*\n?)*)", text, re.M)
    return m.group(1) if m else ""
def package_edition(base):
    text = open(os.path.join(base, "Cargo.toml"), encoding="utf-8").read()
    m = re.search(r'^edition\s*=\s*"(\d+)"', toml_section(text, "package"), re.M)
    if m: return m.group(1)
    directory = base
    while directory.startswith(W):
        path = os.path.join(directory, "Cargo.toml")
        if os.path.exists(path):
            t = open(path, encoding="utf-8").read()
            if "[workspace" in t:
                m = re.search(r'^edition\s*=\s*"(\d+)"', toml_section(t, "workspace.package"), re.M)
                if m: return m.group(1)
        directory = os.path.dirname(directory)
    return "2021"
def features(base):
    text = open(os.path.join(base, "Cargo.toml"), encoding="utf-8").read()
    names = re.findall(r"^([A-Za-z0-9_\-]+)\s*=", toml_section(text, "features"), re.M)
    deps = re.findall(r"^([A-Za-z0-9_\-]+)\s*=\s*\{[^}]*optional\s*=\s*true", toml_section(text, "dependencies"), re.M)
    return sorted(set(names + deps))
def entries(base):
    text = open(os.path.join(base, "Cargo.toml"), encoding="utf-8").read()
    out = []
    lib = re.search(r'^path\s*=\s*"([^"]+)"', toml_section(text, "lib"), re.M)
    out.append(("lib", os.path.join(base, lib.group(1) if lib else "src/lib.rs")))
    out.append(("bin", os.path.join(base, "src/main.rs")))
    for table in re.finditer(r"^\[\[(bin|test|bench|example)\]\]\s*\n((?:(?!\[).*\n?)*)", text, re.M):
        p = re.search(r'^path\s*=\s*"([^"]+)"', table.group(2), re.M)
        if p: out.append((table.group(1), os.path.join(base, p.group(1))))
    for kind, directory in (("bin", "src/bin"), ("test", "tests"), ("bench", "benches"), ("example", "examples")):
        d = os.path.join(base, directory)
        if os.path.isdir(d):
            for e in sorted(os.listdir(d)):
                p = os.path.join(d, e, "main.rs") if os.path.isdir(os.path.join(d, e)) else os.path.join(d, e)
                if p.endswith(".rs") and os.path.exists(p): out.append((kind, p))
    out.append(("build", os.path.join(base, "build.rs")))
    return [(k, p) for k, p in out if os.path.exists(p)]
def parse(text):
    first = next((l for l in text.split("\n") if ": " in l), "")
    body = first[first.index(": ") + 2:] if ": " in first else ""
    out, cur, i = [], "", 0
    while i < len(body):
        c = body[i]
        if c == "\\" and i + 1 < len(body) and body[i + 1] == " ": cur += " "; i += 2; continue
        if c == " ":
            if cur: out.append(cur); cur = ""
        else: cur += c
        i += 1
    if cur: out.append(cur)
    return out
known = {}
def stub(scratch, name):
    """🧱️ an empty rlib standing in for a crate the probe cannot find, so the compiler still expands modules and includes and writes dep-info."""
    source = os.path.join(scratch, "stub_" + name + ".rs"); open(source, "w").write("")
    subprocess.run(["rustc", "--edition", "2021", "--crate-type", "rlib", "--crate-name", name, "-o", os.path.join(scratch, "lib" + name + ".rlib"), source], cwd=scratch, env={**os.environ, "RUSTUP_TOOLCHAIN": TOOLCHAIN}, capture_output=True)
    known[scratch].add(name)
def stubs(scratch):
    out = ["-L", "dependency=" + scratch]
    for name in sorted(known[scratch]): out += ["--extern", name + "=" + os.path.join(scratch, "lib" + name + ".rlib")]
    return out
def run(row):
    base = os.path.join(W, row["root"])
    edition, feats = package_edition(base), features(base)
    union = {"named": set(), "dep": set()}
    errors = []
    with tempfile.TemporaryDirectory() as scratch:
        known[scratch] = set()
        for kind, source in entries(base):
            for flavor in ("dep", "named"):
                if flavor == "dep" and kind not in ("lib", "bin", "build"): continue
                if flavor == "dep" and row["dep"]["status"] != "ok" and False: continue
                dep = os.path.join(scratch, "o.d")
                if os.path.exists(dep): os.remove(dep)
                cmd = ["rustc", "--edition", edition, "--crate-type", "proc-macro" if kind == "lib" and False else "lib", "--crate-name", "probe", "--emit=dep-info=" + dep, "--cap-lints", "allow"]
                for f in feats: cmd += ["--cfg", 'feature="%s"' % f]
                if flavor == "named" or kind in ("test", "bench"): cmd += ["--test"]
                cmd.append(source)
                for attempt in range(40):
                    p = subprocess.run(cmd + stubs(scratch), cwd=scratch, env={**os.environ, "RUSTUP_TOOLCHAIN": TOOLCHAIN}, capture_output=True, text=True, timeout=120)
                    missing = re.search(r"E0463\]: can't find crate for `([A-Za-z0-9_]+)`", p.stderr)
                    if os.path.exists(dep) or not missing or missing.group(1) in known[scratch]: break
                    stub(scratch, missing.group(1))
                if not os.path.exists(dep): errors.append((kind + "/" + flavor, os.path.relpath(source, base), re.sub(r"`[^`]*`", "`_`", p.stderr.strip().split("\n")[0])[:120])); continue
                for entry in parse(open(dep, encoding="utf-8", errors="replace").read()):
                    absolute = os.path.normpath(entry if entry.startswith("/") else os.path.join(scratch, entry))
                    if absolute.startswith(W + "/"): union[flavor].add(absolute[len(W) + 1:])
    union["named"] |= union["dep"]
    return row, union, errors
def declared(row, flavor):
    v = row[flavor]
    return set(v.get("files", [])), (row["root"] + "/" if v["status"] != "ok" else None)
report = {"crates": 0, "gaps": 0, "errorCrates": 0, "errorKinds": {}, "rows": []}
with cf.ThreadPoolExecutor(jobs) as pool:
    for row, union, errors in pool.map(run, rows):
        report["crates"] += 1
        if errors: report["errorCrates"] += 1
        for kind, _, message in errors: report["errorKinds"][kind + " " + message] = report["errorKinds"].get(kind + " " + message, 0) + 1
        gap = {}
        for flavor in ("named", "dep"):
            files, prefix = declared(row, flavor)
            miss = sorted(f for f in union[flavor] if f not in files and not (prefix and f.startswith(prefix)) and not f.startswith(".") and "/node_modules/" not in f)
            if miss: gap[flavor] = miss
        if gap:
            report["gaps"] += 1
            report["rows"].append({"name": row["name"], "root": row["root"], "status": {f: row[f]["status"] for f in ("named", "dep")}, "reason": row["named"].get("reason"), "gap": gap, "probeErrors": errors[:3]})
        if report["crates"] % 200 == 0: print(json.dumps({"progress": report["crates"], "gaps": report["gaps"]}), flush=True)
if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1048576: raise SystemExit("RSS above 500 MB")
json.dump(report, open(out_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: report[k] for k in ("crates", "gaps", "errorCrates")}))
