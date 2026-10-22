#!/usr/bin/env python3
"""usage: gen3.py NAME [ROOT]   writes the scaffold specs/NAME.spec of an increment from ROOT (the chain overlay), after the increment's prepare() edits."""
import importlib, json, sys
from scopes3 import *
import specgen

name = sys.argv[1]
root = sys.argv[2] if len(sys.argv) > 2 else "/private/tmp/i18n-core/t6"
steps = json.load(open("steps.json"))
module = [m for k, v in steps.items() if k.endswith("-" + name) for m in v][0]
tree = Tree(root)
mod = importlib.import_module(module)
if hasattr(mod, "prepare"):
    mod.prepare(tree)
files = discover(tree, name)
text = specgen.scaffold(tree, list(files), files, KINDS)
open("specs/%s.spec" % name, "w", encoding="utf8").write(text)
print(name, text.count("=> ???"), "entries in", len(files), "files")
