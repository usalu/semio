"""Builds a node_modules-free COPY overlay (real file copies, never symlinks) of a checkout: every tracked and untracked-unignored file of the given roots (default: the framework) plus the root files and all workspace manifests; usage: make-overlay.py <repo> <dest> [root dirs...]"""
import json, os, shutil, subprocess, sys
repo, dest, *roots = sys.argv[1:]
roots = roots or ["🧰️framework"]
manifest = json.load(open(os.path.join(repo, "package.json"), encoding="utf-8"))
git = lambda *args: subprocess.run(["git", "-C", repo, "-c", "core.quotepath=off", *args], capture_output=True, text=True, check=True).stdout.split("\0")
files = set()
for root in roots:
    files.update(name for name in git("ls-files", "-z", "--", root) if name)
    files.update(name for name in git("ls-files", "-z", "--others", "--exclude-standard", "--", root) if name)
for directory in manifest["workspaces"]:
    files.add(os.path.join(directory, "package.json"))
files.update(["package.json", "📋️project.json", "📜️script.ts", "bunfig.toml", "bun.lock", "tsconfig.json", "AGENTS.md", "nx.json", "🔒️dependencies.json", "📋️build-owners.json"])
copied = missing = 0
for file in sorted(files):
    source = os.path.join(repo, file)
    if not os.path.isfile(source) or os.path.islink(source): missing += 1; continue
    target = os.path.join(dest, file)
    os.makedirs(os.path.dirname(target), exist_ok=True)
    shutil.copyfile(source, target)
    shutil.copymode(source, target)
    copied += 1
print(json.dumps({"copied": copied, "missing": missing, "dest": dest}))
