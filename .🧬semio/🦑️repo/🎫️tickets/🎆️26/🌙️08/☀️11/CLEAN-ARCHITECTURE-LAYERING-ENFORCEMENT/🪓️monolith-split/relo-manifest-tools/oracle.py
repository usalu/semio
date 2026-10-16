"""🔬️ Patched full-resolution differential (📓️RESOLUTION-ORACLE.md): runs analysis-only rustc (`--emit=metadata -o
/dev/null`) on a crate root in a sandbox holding the baseline files and in a sandbox of the current tree, both with the
coordinator patches (missing `extern crate` lines removed, external globs neutralised), and compares the error multisets.

Usage: oracle.py --name CRATE_SNAKE --root REPO_REL_LIB_ROOT --cargo REPO_REL_CARGO_TOML --baseline BASELINE_DIR
                 --out OUT_PREFIX [--config native|test|wasip2|wasip2-test|wasm32|wasm32-test]... [--cfg 'feature="x"']...
"""
import argparse, json, os, re, shutil, subprocess, sys, tempfile, collections, pathlib
REPO = pathlib.Path("/Users/ueli/Documents/semio")
ap = argparse.ArgumentParser()
ap.add_argument("--name", required=True)
ap.add_argument("--root", required=True)
ap.add_argument("--cargo", required=True)
ap.add_argument("--baseline", required=True)
ap.add_argument("--out", required=True)
ap.add_argument("--config", action="append", default=[])
ap.add_argument("--cfg", action="append", default=[])
ap.add_argument("--crate-type", default="lib")
ap.add_argument("--freeze", action="store_true")
a = ap.parse_args()
configs = a.config or ["native"]
SCRATCH = pathlib.Path(tempfile.mkdtemp(prefix="relo-oracle-", dir="/private/tmp/claude-501/-Users-ueli-Documents-semio/35792af2-9c1e-4f70-b4e3-d5fa94336422/scratchpad"))

def externs():
    text = (REPO / a.cargo).read_text()
    names = set()
    section = None
    for line in text.split("\n"):
        s = line.strip()
        if s.startswith("["):
            section = s
            continue
        if section and "dependencies" in section:
            m = re.match(r"([A-Za-z0-9_-]+)\s*=", s)
            if m:
                names.add(m.group(1).replace("-", "_"))
                pk = re.search(r'package\s*=\s*"([^"]+)"', s)
                if pk:
                    names.add(pk.group(1).replace("-", "_"))
    return names
EXT = externs()
KEEP_EXTERN = {"std", "core", "alloc", "proc_macro", "test", "self"}

def patch(text):
    lines = text.split("\n")
    out = []
    for line in lines:
        m = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(\w+)", line)
        if m and m.group(1) not in KEEP_EXTERN:
            while out and out[-1].strip().startswith("#[") and not out[-1].strip().startswith("#[path"):
                out.pop()
            out.append("")
            continue
        g = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?(\w+)(?:::\w+)*::\*\s*;\s*$", line)
        if g and g.group(1) in EXT:
            out.append("const _: () = ();")
            continue
        out.append(line)
    return "\n".join(out)

def strip_comments(text):
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    return "\n".join(re.sub(r"//.*$", "", line) if '"' not in line.split("//")[0] or line.count('"') % 2 == 0 else line for line in text.split("\n"))

def walk_modules(sandbox, root_rel):
    seen = set()
    todo = [pathlib.Path(root_rel)]
    while todo:
        rel = todo.pop()
        key = os.path.normpath(str(rel))
        if key in seen:
            continue
        full = sandbox / key
        if not full.is_file():
            continue
        seen.add(key)
        text = strip_comments(full.read_text(errors="replace"))
        base = pathlib.Path(key).parent
        stack = []
        depth = 0
        pending_path = None
        for token in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]|\binclude(?:_str)?!\s*\(\s*"([^"]+)"|\bmod\s+(\w+)\s*([;{])|[{}]', text):
            if token.group(1) is not None:
                pending_path = token.group(1)
            elif token.group(2) is not None:
                cur = stack[-1][0] if stack else base
                if token.group(0).startswith("include!"):
                    todo.append(cur / token.group(2))
            elif token.group(3) is not None:
                cur = stack[-1][0] if stack else base
                if token.group(4) == ";":
                    if pending_path is not None:
                        todo.append(cur / pending_path)
                    else:
                        todo.append(cur / (token.group(3) + ".rs"))
                        todo.append(cur / token.group(3) / "mod.rs")
                    pending_path = None
                else:
                    inner = cur / pending_path if pending_path is not None else cur / token.group(3)
                    stack.append((inner, depth))
                    depth += 1
                    pending_path = None
            elif token.group(0) == "{":
                depth += 1
                pending_path = None
            elif token.group(0) == "}":
                depth -= 1
                if stack and stack[-1][1] == depth:
                    stack.pop()
    return seen

def build_sandbox(dest, overrides):
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    real_dirs = {pathlib.Path(".")}
    for rel in overrides:
        p = pathlib.Path(rel).parent
        while True:
            real_dirs.add(p)
            if p == pathlib.Path("."):
                break
            p = p.parent
    for d in sorted(real_dirs, key=lambda x: len(x.parts)):
        (dest / d).mkdir(parents=True, exist_ok=True)
        src = REPO / d
        if src.is_dir():
            for entry in os.listdir(src):
                child = d / entry
                if child in real_dirs or str(child) in overrides:
                    continue
                target = dest / child
                if not target.exists() and not target.is_symlink():
                    os.symlink(src / entry, target)
    for rel, content in overrides.items():
        target = dest / rel
        if target.is_symlink():
            target.unlink()
        target.write_bytes(content if isinstance(content, bytes) else content.encode())

def rustc_args(config):
    args = ["rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", a.crate_type, "--crate-name", a.name, "--cap-lints", "allow"]
    if "test" in config:
        args.append("--test")
    if config.startswith("wasip2"):
        args += ["--target", "wasm32-wasip2"]
    if config.startswith("wasm32"):
        args += ["--target", "wasm32-unknown-unknown"]
    for c in a.cfg:
        args += ["--cfg", c]
    return args

def closure(sandbox, config):
    root = sandbox / a.root
    dep = SCRATCH / "dep.d"
    subprocess.run(["perl", "-e", "alarm 600; exec @ARGV", "--"] + rustc_args(config) + [f"--emit=dep-info={dep}", root.name], cwd=root.parent, capture_output=True)
    files = set()
    if dep.exists():
        text = dep.read_text().replace("\\\n", " ")
        first = text.split("\n")[0]
        for token in first.split(": ", 1)[1].split():
            full = os.path.normpath(os.path.join(root.parent, token.replace("\\ ", " ")))
            rel = os.path.relpath(full, sandbox)
            if not rel.startswith(".."):
                files.add(rel)
    files |= {f for f in walk_modules(sandbox, a.root) if f.endswith(".rs")}
    return files

def analyse(tree, overrides, config):
    sandbox = SCRATCH / f"{tree}-{config}"
    build_sandbox(sandbox, overrides)
    files = closure(sandbox, config)
    patched = dict(overrides)
    for rel in files:
        path = sandbox / rel
        if rel.endswith(".rs") and path.exists():
            text = path.read_text()
            new = patch(text)
            if new != text:
                patched[rel] = new
    build_sandbox(sandbox, patched)
    root = sandbox / a.root
    env = {**os.environ, "CARGO_MANIFEST_DIR": str(root.parent), "CARGO_PKG_NAME": a.name.replace("_", "-"), "CARGO_PKG_VERSION": "0.1.0", "CARGO_CRATE_NAME": a.name}
    res = subprocess.run(["perl", "-e", "alarm 900; exec @ARGV", "--"] + rustc_args(config) + ["--emit=metadata", "-o", "/dev/null", "--error-format=json", root.name], cwd=root.parent, capture_output=True, text=True, env=env)
    errors = []
    for line in res.stderr.split("\n"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error":
            continue
        code = (d.get("code") or {}).get("code") or ""
        span = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
        snippet = ""
        label = ""
        fname = ""
        if span:
            label = span.get("label") or ""
            fname = os.path.relpath(os.path.normpath(os.path.join(root.parent, span["file_name"])), sandbox)
            text = span.get("text") or []
            if text:
                t = text[0]
                snippet = t["text"][t["highlight_start"] - 1:t["highlight_end"] - 1]
        msg = re.sub(re.escape(str(sandbox)), "<S>", d.get("message", ""))
        errors.append({"code": code, "message": msg, "label": label, "snippet": snippet, "file": fname})
    shutil.rmtree(sandbox, ignore_errors=True)
    return errors, len(files), sum(1 for r in patched if r not in overrides)

base_dir = pathlib.Path(a.baseline)
baseline_overrides = {}
for path in base_dir.rglob("*"):
    if path.is_file() and not path.name.startswith("."):
        baseline_overrides[str(path.relative_to(base_dir))] = path.read_bytes()
frozen = {}
if a.freeze:
    for rel in walk_modules(REPO, a.root):
        frozen[rel] = (REPO / rel).read_bytes()
    baseline_overrides = {**frozen, **baseline_overrides}
summary = {}
for config in configs:
    cur, nfiles_c, npatch_c = analyse("current", dict(frozen), config)
    base, nfiles_b, npatch_b = analyse("baseline", baseline_overrides, config)
    key = lambda e: (e["code"], e["message"], e["label"], e["snippet"])
    cb, cc = collections.Counter(map(key, base)), collections.Counter(map(key, cur))
    new = cc - cb
    gone = cb - cc
    summary[config] = {"baseline_errors": len(base), "current_errors": len(cur), "baseline_files": nfiles_b, "current_files": nfiles_c, "patched_files": [npatch_b, npatch_c], "new": [list(k) + [n] for k, n in new.items()], "removed": [list(k) + [n] for k, n in gone.items()]}
    rel_new = [e for e in cur if key(e) in new]
    summary[config]["new_detail"] = rel_new[:200]
    print(f"{a.name} {config}: baseline {len(base)} errors ({nfiles_b} files, {npatch_b} patched) / current {len(cur)} ({nfiles_c} files, {npatch_c} patched); new keys {sum(new.values())}, removed keys {sum(gone.values())}")
    for k, n in list(new.items())[:40]:
        print("   NEW", n, k)
    for k, n in list(gone.items())[:15]:
        print("   GONE", n, k)
json.dump(summary, open(a.out + ".json", "w"), ensure_ascii=False, indent=1)
shutil.rmtree(SCRATCH, ignore_errors=True)
