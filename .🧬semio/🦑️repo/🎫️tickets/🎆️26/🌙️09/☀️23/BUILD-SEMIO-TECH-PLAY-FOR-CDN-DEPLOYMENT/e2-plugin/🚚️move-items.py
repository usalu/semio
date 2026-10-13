#!/usr/bin/env python3
"""🚚️ Moves named top-level items (with their docs, attributes and every `impl` of them) between two crate files.

Usage: move-items.py <from file> <to file> <name>[,<name>...]   (paths relative to the plugin crate)
"""
import importlib.util, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(HERE, "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)
PLUGIN = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
src, dst, names = os.path.join(PLUGIN, sys.argv[1]), os.path.join(PLUGIN, sys.argv[2]), set(sys.argv[3].split(","))
raw = open(src, encoding="utf-8").read(); cl = ri.clean(raw)
its, _ = ri.items(raw, cl, 0, len(cl))
cut = [it for it in its if (it["name"] in names and it["kind"] not in ("use", "macro_call")) or (it["kind"] == "impl" and (it.get("self") in names))]
missing = names - {it["name"] for it in cut if it["name"]}
if missing: raise SystemExit(f"[move] not found: {missing}")
blocks = []
for it in sorted(cut, key=lambda x: x["start"], reverse=True):
    blocks.append(raw[it["start"]:it["end"]].strip("\n"))
    raw = raw[:it["start"]] + raw[it["end"]:]
blocks.reverse()
blocks = [re.sub(r"^((?:\s*(?:///|//|#\[).*\n)*)\s*(?:pub\(super\)\s+|(?=(?:struct|enum|fn|const|static|type|trait)\b))", lambda m: m.group(1) + ("" if "pub" in m.group(0)[len(m.group(1)):] and "pub(super)" not in m.group(0) else "pub(crate) "), b, count=1) for b in blocks]
open(src, "w", encoding="utf-8").write(re.sub(r"\n{3,}", "\n\n", raw))
d = open(dst, encoding="utf-8").read().rstrip("\n")
open(dst, "w", encoding="utf-8").write(d + "\n\n" + "\n\n".join(blocks) + "\n")
print(f"[move] {len(cut)} items {sys.argv[1]} -> {sys.argv[2]}")
