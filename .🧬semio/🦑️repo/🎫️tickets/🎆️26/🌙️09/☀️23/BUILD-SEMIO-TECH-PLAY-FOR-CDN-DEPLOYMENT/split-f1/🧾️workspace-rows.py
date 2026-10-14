#!/usr/bin/env python3
"""🧾️ Replaces a cut crate's `[workspace.dependencies]` rows/tables with rows for exactly the owner crates the workspace's
members inherit (`<key> = { workspace = true }` anywhere under the workspace directory); catalog workspaces listed in
`"catalogs"` get a row for every owner.

usage: 🧾️workspace-rows.py <config.json> plan|apply
config: {"old": <old package>, "aliases": {<workspace key>: <owner package>}, "owners": {<package>: <repo-relative dir>},
         "catalogs": [<repo-relative workspace manifest>]}
"""
import json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
config = json.load(open(sys.argv[1], encoding="utf-8"))
mode = sys.argv[2]
old, owners, aliases = config["old"], config["owners"], config.get("aliases", {})
listed = subprocess.run(["git", "ls-files", "--cached", "--others", "--exclude-standard", "--", "*Cargo.toml"], cwd=REPO, capture_output=True, text=True).stdout.split("\n")
listed = [p for p in listed if p and not p.startswith(".🧬semio")]
row = re.compile(r'(?m)^[ \t]*(?:"?' + re.escape(old) + r'"?|(?P<alias>[A-Za-z0-9_-]+))[ \t]*=[ \t]*\{[^\n]*\}[ \t]*\n')
for path in listed:
    text = open(os.path.join(REPO, path), encoding="utf-8").read()
    if "[workspace" not in text:
        continue
    base = os.path.dirname(path)
    tables = list(re.finditer(r'(?ms)^([ \t]*)\[workspace\.dependencies\.(' + re.escape(old) + r'|[A-Za-z0-9_-]+)\]\n(.*?)(?=^[ \t]*\[|\Z)', text))
    hits = [t for t in tables if t.group(2) == old or (t.group(2) in aliases and re.search(r'package\s*=\s*"' + re.escape(old) + '"', t.group(3)))]
    ws_block = re.search(r'(?ms)^\[workspace\.dependencies\]\n(.*?)(?=^\[|\Z)', text)
    inline = [m for m in re.finditer(r'(?m)^' + re.escape(old) + r'[ \t]*=[ \t]*\{[^\n]*\}\n', ws_block.group(1))] if ws_block else []
    if not hits and not inline:
        continue
    members = [p for p in listed if (p.startswith(base + "/") or base == "") and p != path]
    needed = set()
    for member in members:
        mt = open(os.path.join(REPO, member), encoding="utf-8").read()
        for key in re.findall(r'(?m)^([A-Za-z0-9_-]+)[ \t]*=[ \t]*\{[ \t]*workspace[ \t]*=[ \t]*true', mt):
            if key in owners or key in aliases:
                needed.add(key)
    if path in config.get("catalogs", []):
        needed = set(owners)
    print(path, "needs", sorted(needed))
    for t in sorted(hits, key=lambda t: t.start(), reverse=True):
        indent = t.group(1)
        keys = sorted(needed, key=lambda k: list(owners).index(aliases.get(k, k)))
        out = ""
        for key in keys:
            package = aliases.get(key, key)
            out += f'{indent}[workspace.dependencies.{key}]\n{indent}path = "{os.path.relpath(os.path.join(REPO, owners[package]), os.path.join(REPO, base))}"\n' + (f'{indent}package = "{package}"\n' if key != package else "") + "\n"
        text = text[:t.start()] + out + text[t.end():]
    if inline:
        start = ws_block.start(1)
        for m in sorted(inline, key=lambda m: m.start(), reverse=True):
            keys = sorted(needed, key=lambda k: list(owners).index(aliases.get(k, k)))
            out = "".join(f'{k} = {{ path = "{os.path.relpath(os.path.join(REPO, owners[aliases.get(k, k)]), os.path.join(REPO, base))}"' + (f', package = "{aliases[k]}"' if k in aliases else "") + " }\n" for k in keys)
            text = text[:start + m.start()] + out + text[start + m.end():]
    if mode == "apply":
        open(os.path.join(REPO, path), "w", encoding="utf-8").write(text)
