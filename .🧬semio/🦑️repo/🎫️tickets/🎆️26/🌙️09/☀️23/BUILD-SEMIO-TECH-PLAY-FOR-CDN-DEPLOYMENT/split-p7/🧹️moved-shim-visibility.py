#!/usr/bin/env python3
"""🧹️ Narrows every `//#region 🚚️Moved` re-export block to the moved items' own visibility: private items are dropped,
`pub(crate)` items are re-exported `pub(crate)`, `pub` items `pub`.

usage: 🧹️moved-shim-visibility.py <moves.json> <file.rs>...   (moves.json: the `.moved.json` record of 🚚️move-items.py)
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
ITEM_SPLIT = os.path.join(HERE, "..", "e3-format-split", "🔀️item-split.py")
ns = {"__file__": ITEM_SPLIT}
exec(compile(open(ITEM_SPLIT, encoding="utf-8").read().split("\ndef analyse")[0], ITEM_SPLIT, "exec"), ns)
moves_path, files = sys.argv[1], sys.argv[2:]
moves = json.load(open(moves_path, encoding="utf-8"))
targets = json.load(open(moves_path.replace(".moved.json", ".json"), encoding="utf-8"))["moves"]
vis = {}
for move in targets:
    src, items = ns["parse"](os.path.join(REPO, move["to"]))
    for it in items:
        code = re.sub(r"^(\s*(#!?\[[^\]]*\]|///[^\n]*|//[^\n]*)\s*)*", "", it.stripped)
        m = re.match(r"\s*(pub(?:\s*\([^)]*\))?)?", code)
        v = (m.group(1) or "").replace(" ", "")
        vis[(move["to_module"], it.name)] = v
for path in files:
    text = open(path, encoding="utf-8").read()
    def fix(m):
        body = m.group(1)
        rows = re.findall(r"(crate::[^,{};]*?)::([A-Za-z_][A-Za-z0-9_]*)(?=[,}])", body.replace("crate::{", "crate::{crate::").replace("{crate::", "{"))
        out = {"pub": [], "pub(crate)": []}
        for module_path, name in re.findall(r"((?:crate::)?(?:[A-Za-z_][A-Za-z0-9_]*::)+)([A-Za-z_][A-Za-z0-9_]*)\b", re.sub(r"pub(\(crate\))?\s+use\s+crate::\{?", "crate::", body)):
            module = module_path.rstrip(":").rstrip(":")
            module = module if module.startswith("crate::") else "crate::" + module
            v = vis.get((module, name))
            if v in out:
                out[v].append(f"{module}::{name}")
        lines = []
        for v, paths in out.items():
            if paths:
                tree = paths[0].replace("crate::", "", 1) if len(paths) == 1 else "{" + ", ".join(p.replace("crate::", "", 1) for p in paths) + "}"
                lines.append(f"{v} use crate::{tree};")
        return "//#region 🚚️Moved\n" + "\n".join(lines) + ("\n" if lines else "") + "//#endregion 🚚️Moved"
    new = re.sub(r"//#region 🚚️Moved\n(.*?)//#endregion 🚚️Moved", fix, text, flags=re.S)
    if new != text:
        open(path, "w", encoding="utf-8").write(new)
        print(f"narrowed {os.path.relpath(path, REPO)}")
