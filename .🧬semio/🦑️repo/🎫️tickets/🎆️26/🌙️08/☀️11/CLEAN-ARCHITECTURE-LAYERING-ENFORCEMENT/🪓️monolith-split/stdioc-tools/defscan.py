#!/usr/bin/env python3
# 🔍️ Definition scan: names that are NOT public in an upstream producer yet appear in a downstream producer that depends on it (fields, methods, items the resolution oracle cannot see).
import os
import re
import sys

sys.path.insert(0, os.path.dirname(__file__))
from lib import *  # noqa: E402,F401,F403

DECL = re.compile(r"(?m)^(\s*)(pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(fn|struct|enum|trait|type|static|const|union|mod|macro_rules!)\s+([A-Za-z_][A-Za-z0-9_]*)")
FIELD = re.compile(r"(?m)^\s{4}(pub(?:\([^)]*\))?\s+)?([a-z_][A-Za-z0-9_]*)\s*:")


def non_public(root):
    out = {}
    for path in closure(root):
        text = strip_comments(read(path))
        if "#[cfg(test)]" in text.split("\n", 3)[0:1]:
            pass
        for indent, vis, kind, name in DECL.findall(text):
            if vis is None or vis.strip() != "pub":
                if indent == "" or kind == "fn":
                    out.setdefault(name, []).append((os.path.relpath(path, C), kind, (vis or "").strip() or "private"))
        for vis, name in FIELD.findall(text):
            if vis is None or vis.strip() != "pub":
                out.setdefault(name, []).append((os.path.relpath(path, C), "field", (vis or "").strip() or "private"))
    return out


def tokens(root, tests=False):
    found = set()
    for path, is_test in closure(root).items():
        if is_test and not tests:
            continue
        found |= set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", strip_comments(read(path))))
    return found


def imports_from(root, upstream_lib):
    names = set()
    for path in closure(root):
        text = strip_comments(read(path))
        for match in re.finditer(r"use\s+" + upstream_lib + r"::(\{[^}]*\}|[A-Za-z_0-9:]+)", text):
            names |= set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_]*\b", match.group(1)))
        for match in re.finditer(r"(?<![\w:])" + upstream_lib + r"::([A-Za-z_][A-Za-z0-9_:]*)", text):
            names |= set(match.group(1).split("::"))
    return names


if __name__ == "__main__":
    for downstream in CRATES:
        files = closure(downstream["root"])
        text = "\n".join(strip_comments(read(path)) for path in files)
        for upstream in CRATES:
            if upstream is downstream or upstream["lib"] not in text:
                continue
            private = non_public(upstream["root"])
            tok = tokens(downstream["root"], tests=True)
            hits = sorted(name for name in private if name in tok and len(name) > 3)
            imported = imports_from(downstream["root"], upstream["lib"])
            direct = sorted(name for name in imported if name in private)
            print(f"{downstream['key']} <- {upstream['key']}: imported-private={direct} shared-name-private={len(hits)}")
            if hits and len(hits) <= 60:
                print("    ", hits)
