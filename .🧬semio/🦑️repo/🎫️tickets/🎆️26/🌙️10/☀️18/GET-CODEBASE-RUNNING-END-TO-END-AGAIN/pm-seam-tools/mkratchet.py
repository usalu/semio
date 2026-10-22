#!/usr/bin/env python3
"""⚖️ Cuts the ratchet allowlist: how many package manager lock, configuration, flag and runtime literals each production file of the library still holds. usage: mkratchet.py ROOT [--write]"""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))
import paths
from winpath import place
PATTERN = re.compile(r"bun\.lock|bunfig|--frozen-lockfile|--lockfile-only|process\.versions\.bun|Bun\.JSONC") if paths.ROUND == 1 else re.compile(r"bun\.lock|bun\.lockb|bunfig|--frozen-lockfile|--lockfile-only|BUN_INSTALL|bun-(?:cache|global|bin|home|config|import|require)")
SKIP = {"node_modules", "🧪️tests", "🧫️fixtures", ".git", "target", "dist", ".cache", "🔌️managers"}
def exempt(library):
    if paths.ROUND == 1:
        return set(), set()
    managers = os.path.join(library, "⚡️caching", "🚀️bootstrap", "📦️dependencies", "🔌️managers")
    modules, data = set(), set()
    catalog = json.load(open(os.path.join(managers, "🔣️catalog.json"), encoding="utf-8"))["adapters"]
    for folder in catalog.values():
        modules.update(json.load(open(os.path.join(managers, folder, "🔣️.json"), encoding="utf-8")).get("ownedModules", []))
    data.update(json.load(open(os.path.join(managers, "🔣️owned.json"), encoding="utf-8"))["data"])
    return modules, data
def scan(root):
    library = os.path.join(root, paths.LIBRARY)
    modules, data = exempt(library)
    found = {}
    for current, directories, names in os.walk(library):
        directories[:] = [d for d in directories if d not in SKIP]
        for name in names:
            if not re.search(r"\.(ts|mjs|cjs|json)$", name):
                continue
            path = os.path.join(current, name)
            relative = os.path.relpath(path, library).replace(os.sep, "/")
            if os.path.islink(path) or relative in data or any(relative == m or relative.startswith(m + "/") for m in modules):
                continue
            count = len(PATTERN.findall(open(path, encoding="utf-8", errors="replace").read()))
            if count:
                found[os.path.relpath(path, library).replace(os.sep, "/")] = count
    return dict(sorted(found.items()))
def main():
    root = os.path.abspath(sys.argv[1])
    found = scan(root)
    print(len(found), "files,", sum(found.values()), "literals")
    if "--write" in sys.argv:
        paths.seam_write(paths.MANAGERS + "/🧪️tests/⚖️ratchet/🔣️.json", json.dumps({"version": 1, "files": found}, ensure_ascii=False, indent=1) + "\n")
main()
