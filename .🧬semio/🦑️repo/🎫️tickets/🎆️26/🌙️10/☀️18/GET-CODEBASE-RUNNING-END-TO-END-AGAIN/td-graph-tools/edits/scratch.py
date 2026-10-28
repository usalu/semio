"""🧪 [td-graph] batch `scratch`: the workspace graph laws run with checkout scratch (short artifact roots inside the repository, as `build-graph-rust` does): portable CLI Cargo fixtures must lie inside the repository and the playground law bounds fixture paths to 256 units. usage: python3 -I scratch.py <root>"""
import os, sys
rel = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json"
path = os.path.join(sys.argv[1], rel); text = open(path, encoding="utf8").read()
old = '"id": "build-workspace-graph-laws",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "trees"'
if text.count(old) != 1: sys.exit(f"anchor x{text.count(old)}")
text = text.replace(old, '"id": "build-workspace-graph-laws",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "scratch": "checkout",\n      "trees"')
open(path, "w", encoding="utf8").write(text); print("edited", rel)
