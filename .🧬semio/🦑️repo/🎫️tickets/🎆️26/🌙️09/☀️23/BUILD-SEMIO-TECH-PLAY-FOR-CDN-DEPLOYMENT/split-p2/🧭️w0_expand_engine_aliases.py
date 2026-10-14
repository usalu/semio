#!/usr/bin/env python3
"""🧭️ Wave 0: expands the `<topic> as remodeling_<x>` engine module aliases of the split topics (mesh, sfm, video) into
full module paths, so `🧭️canonical-paths.py` can point every item at the sub-topic module that now owns it (lane P2).

usage: 🧭️w0_expand_engine_aliases.py <topic>...
"""
import os, re, sys
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling")
BASE = "crate::editor::remodeling::engine"
topics = sys.argv[1:]
changed = 0
for dirpath, _, files in os.walk(ART):
    for name in files:
        if not name.endswith(".rs"):
            continue
        path = os.path.join(dirpath, name)
        text = original = open(path, encoding="utf-8").read()
        for topic in topics:
            single = re.compile(r"(?m)^[ \t]*use " + re.escape(BASE) + r"::" + topic + r" as ([a-z_]+);\n")
            aliases = [m.group(1) for m in single.finditer(text)]
            text = single.sub("", text)
            group = re.compile(r"(use " + re.escape(BASE) + r"::\{)([^}]*)(\};)")
            def strip_alias(m):
                parts = [p.strip() for p in m.group(2).split(",") if p.strip()]
                kept = []
                for part in parts:
                    a = re.fullmatch(topic + r"\s+as\s+([a-z_]+)", part)
                    if a:
                        aliases.append(a.group(1))
                    else:
                        kept.append(part)
                if not kept:
                    return ""
                return m.group(1) + ", ".join(kept) + m.group(3) if len(kept) > 1 else "use " + BASE + "::" + kept[0] + ";"
            text = group.sub(strip_alias, text)
            for alias in set(aliases):
                text = re.sub(r"(?<![\w:])" + alias + r"::", BASE + "::" + topic + "::", text)
        if text != original:
            open(path, "w", encoding="utf-8").write(text)
            changed += 1
print("files changed", changed)
