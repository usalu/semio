#!/usr/bin/env python3
"""🔎 Static facts of the staged CI that the workflow laws cannot see: `python3 -I cover-check.py <root with the batch applied> [--live ROOT] [--remote]`.
1 every `hashFiles(...)` argument that names a file exists; 2 every cache directory equals a directory of the cache ledger or the Bun store of the README;
3 every project named by go.yml and products.yml exists in a tracked manifest (wildcards expanded, `!` negation honoured) and owns every target the row names;
4 every pinned action SHA exists upstream (only with --remote and an authenticated gh); 5 every runner label is one of the three pins.
Exit 1 on any finding."""
import fnmatch, json, os, re, subprocess, sys
root = os.path.abspath(sys.argv[1])
live = os.path.abspath(sys.argv[sys.argv.index("--live") + 1]) if "--live" in sys.argv else None
here = os.path.abspath(__file__)
while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
live = live or here
findings = []
def read(path): return open(path, encoding="utf8").read()
files = [os.path.join(root, ".github", "workflows", name) for name in sorted(os.listdir(os.path.join(root, ".github", "workflows")))] + [os.path.join(root, ".github", "actions", name, "action.yml") for name in sorted(os.listdir(os.path.join(root, ".github", "actions")))]
text = {path: read(path) for path in files}
for path, body in text.items():
    for call in re.findall(r"hashFiles\(([^)]*)\)", body):
        for name in re.findall(r"'([^']+)'", call):
            if "*" in name: continue
            if not os.path.exists(os.path.join(live, name)): findings.append(f"hashFiles names a missing file: {name} ({os.path.basename(path)})")
ledger = json.load(open(os.path.join(live, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json"), encoding="utf8"))
base = ledger["roots"]["repository"]["directory"]
declared = {base + "/" + area["directory"] for area in ledger["areas"] if area["root"] == "repository" and area["kind"] == "tree" and "directory" in area}
allowed = declared | {".cache/b/d/m/bun-cache"}
cache_paths = set()
for path, body in text.items():
    for match in re.finditer(r"(?:GO_BUILD_CACHE:|paths:)\s*(\.[^\n]+)", body): cache_paths.add(match.group(1).strip())
    for block in re.findall(r"PRINT_TEX_CACHE: \|\n((?:    [^\n]+\n)+)", body):
        cache_paths.update(line.strip() for line in block.splitlines())
for directory in sorted(cache_paths):
    if directory not in allowed: findings.append(f"cache directory is not a declared loose tree of the ledger: {directory}")
inventory = {}
listed = subprocess.run(["git", "ls-files", "-z", "--", "*📋️project.json"], cwd=live, capture_output=True, text=True).stdout.split("\0")
for rel in filter(None, listed):
    try: manifest = json.load(open(os.path.join(live, rel), encoding="utf8"))
    except Exception: continue
    if isinstance(manifest, dict) and manifest.get("name"): inventory[manifest["name"]] = set((manifest.get("targets") or {}).keys())
rows = 0
for name in ("go.yml", "products.yml"):
    body = text[os.path.join(root, ".github", "workflows", name)]
    for match in re.finditer(r'projects: "([^"]+)"(?:[^}]*?targets: "([^"]+)")?', body):
        rows += 1
        patterns = match.group(1).split(","); targets = (match.group(2) or "test").split(",")
        include = [p for p in patterns if not p.startswith("!")]; exclude = [p[1:] for p in patterns if p.startswith("!")]
        chosen = sorted({n for n in inventory for p in include if fnmatch.fnmatch(n, p)} - {n for n in inventory for p in exclude if fnmatch.fnmatch(n, p)})
        for p in include:
            if not any(fnmatch.fnmatch(n, p) for n in inventory): findings.append(f"{name}: no tracked project matches {p}")
        for project in chosen:
            missing = [t for t in targets if t not in inventory[project]]
            if missing: findings.append(f"{name}: {project} has no target {missing}")
        if "*" in match.group(1): print(f"{name}: {match.group(1)} -> {len(chosen)} projects")
pins = set(re.findall(r"uses: ([\w.-]+/[\w./-]+)@([0-9a-f]{40})", "\n".join(text.values())))
for label in set(re.findall(r"(?:os|runner|runs-on): ((?:ubuntu|macos|windows)[\w.-]*)", "\n".join(text.values()))):
    if label not in ("ubuntu-24.04", "macos-26", "windows-2025"): findings.append(f"runner label is not one of the pins: {label}")
if "--remote" in sys.argv:
    for repository, sha in sorted(pins):
        owner = "/".join(repository.split("/")[:2])
        result = subprocess.run(["gh", "api", f"repos/{owner}/commits/{sha}", "--jq", ".sha"], capture_output=True, text=True)
        if result.stdout.strip() != sha: findings.append(f"pinned SHA does not exist upstream: {repository}@{sha}")
print(f"checked {len(text)} files, {rows} project rows, {len(cache_paths)} cache directories, {len(pins)} pinned actions{' (remote)' if '--remote' in sys.argv else ''}")
for finding in findings: print("FINDING", finding)
sys.exit(1 if findings else 0)
