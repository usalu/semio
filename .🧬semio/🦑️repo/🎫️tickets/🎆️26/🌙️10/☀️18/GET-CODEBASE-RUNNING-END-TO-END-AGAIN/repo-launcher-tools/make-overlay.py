"""Copies exactly the live files the plan touches (plus the fixtures the tests read) into an overlay root, so the plan can be applied and the launcher tested without touching the live tree. Usage: make-overlay.py <live-root> <overlay-root>"""
import json, os, shutil, sys
live, ov = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
here = os.path.dirname(os.path.abspath(__file__))
plan = json.load(open(os.path.join(here, "plan.json"), encoding="utf-8"))
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
paths = {"package.json", "📋️project.json", "🧰️framework/🛍️products/🦑️repo/package.json", M + "/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema/🔣️.json",
 "🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🔣️.json", M + "/📚️library/🏗️build/⚙️operations/🔣️catalog.json", "🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🧪️tests/🔣️.json",
 "🧰️framework/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🧪️tests/🧫️fixtures/🔣️.json"}
for group in plan["groups"].values():
    for kind in ("edits", "rewrites", "deletes"):
        for row in group[kind]: paths.add(row["path"])
for rel in sorted(paths):
    source = os.path.join(live, rel)
    if not os.path.exists(source): continue
    target = os.path.join(ov, rel); os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copy2(source, target)
for name in ("📜️script.ts",):
    target = os.path.join(ov, name); open(target, "w").close()
build = os.path.join(ov, M + "/📚️library/🏗️build/📜️script.ts"); os.makedirs(os.path.dirname(build), exist_ok=True); open(build, "w").close()
print(len(paths), "files into", ov)
