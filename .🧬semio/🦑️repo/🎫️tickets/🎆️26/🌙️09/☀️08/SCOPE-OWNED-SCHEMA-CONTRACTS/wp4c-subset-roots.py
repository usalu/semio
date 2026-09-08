#!/usr/bin/env python3
"""🌱 Row 136 — the subset and extension modules that declare no `🧬️schema/🔣️.json` root.

Without a root document the module has no `$id`, so its mutation leaves have no scope path to
inherit and `schema check` reports every one of them. The root this script writes states what a
subset actually is: **the owning artifact's document, mutated by this subset's leaves**. It therefore

- takes its `$id` from the scope path the leaves already declare (`…/mutation/<kind>/schema.json`
  minus the mutation tail), so no leaf has to move, and
- states the document by reference to the owning artifact scope's export instead of restating it
  (execution contract §A: a scope never restates another scope's `$defs`), which leaves the subset
  with zero exports of its own — a subset owns mutations, not types.

Usage: python3 wp4c-subset-roots.py report|apply
"""
from __future__ import annotations

import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
DRAFT07 = "http://json-schema.org/draft-07/schema#"
JSON_LEAF = "🔣️.json"


def module_candidates() -> list[str]:
    found = []
    for base, dirs, _ in os.walk(os.path.join(REPO, "✏️s/🔌️plugins")):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv", "target", "dist")]
        if os.path.basename(base) != "🧬️schema":
            continue
        module = os.path.relpath(base, REPO)
        if "/🧬️mutations/" in f"{module}/" or "/🧪️" in f"/{module}" or "/🧫️" in f"/{module}" or "/📦️packages/" in module:
            continue
        if os.path.exists(os.path.join(base, JSON_LEAF)) or "/🗄️stdio/" in f"/{module}":
            continue
        if not os.path.isdir(os.path.join(base, "🧬️mutations")):
            continue
        found.append(module)
    return sorted(found)


def leaf_scope_path(module: str) -> str | None:
    """🧭 The scope path this module's own mutation leaves already inherit from it."""
    prefixes = set()
    mutations = os.path.join(REPO, module, "🧬️mutations")
    for leaf in sorted(os.listdir(mutations)):
        path = os.path.join(mutations, leaf, "🧬️schema", JSON_LEAF)
        if not os.path.exists(path):
            continue
        try:
            with open(path, encoding="utf-8") as handle:
                declared = json.load(handle).get("$id")
        except Exception:  # noqa: BLE001
            continue
        if isinstance(declared, str) and "/mutation/" in declared:
            prefixes.add(declared.split("/mutation/")[0])
    return prefixes.pop() if len(prefixes) == 1 else None


def owner_document(module: str) -> tuple[str, str, str] | None:
    """📎 (owner json path, owner `$id`, owner artifact export) for the artifact this module subsets."""
    parts = module.split("/")
    if "🪆️subsets" in parts:
        index = parts.index("🪆️subsets")
        siblings = os.path.join(REPO, *parts[: index + 1])
        candidates = [os.path.join(siblings, name, "🧬️schema", JSON_LEAF) for name in sorted(os.listdir(siblings))]
    elif "🧩️extensions" in parts:
        plugin = os.path.join(REPO, *parts[:3], "🗿️artifacts")
        candidates = []
        for artifact in sorted(os.listdir(plugin)):
            for base, dirs, files in os.walk(os.path.join(plugin, artifact)):
                dirs[:] = [name for name in dirs if not name.startswith(("🧪️", "🧫️"))]
                if os.path.basename(base) == "🧬️schema" and JSON_LEAF in files and "🧬️mutations" not in base:
                    candidates.append(os.path.join(base, JSON_LEAF))
    else:
        return None
    for candidate in candidates:
        if not os.path.exists(candidate) or os.path.relpath(candidate, REPO).startswith(module):
            continue
        with open(candidate, encoding="utf-8") as handle:
            document = json.load(handle)
        title = document.get("title")
        identity = document.get("$id")
        if not isinstance(title, str) or not isinstance(identity, str):
            continue
        # 🎯️The artifact export is the document root unless it also sits in `$defs`; a fragment that
        # names no `$defs` node passes no validator, so the whole document is the honest target.
        fragment = f"#/$defs/{title}" if title in (document.get("$defs") or {}) else ""
        return os.path.relpath(candidate, REPO), f"{identity}{fragment}", title
    return None


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    written, skipped = [], []
    for module in module_candidates():
        scope = leaf_scope_path(module)
        owner = owner_document(module)
        if scope is None:
            skipped.append((module, "no single leaf-declared scope path"))
            continue
        facet = "schema" if "/🧩️extensions/" in f"/{module}" else "artifact"
        subject = module.split("/🧬️schema")[0].split("/")[-1]
        document = {
            "$schema": DRAFT07,
            "$id": f"{scope}/{facet}.json",
            "description": f"🪆️ The {re.sub(r'^[^A-Za-z0-9]+', '', subject)} subset of "
                           f"{owner[2] if owner else 'this artifact'}: the same document, mutated by this module's leaves. "
                           f"The subset owns its mutations, never a second copy of the artifact's types.",
        }
        if owner:
            document["allOf"] = [{"$ref": owner[1]}]
        else:
            skipped.append((module, "no owning artifact document — root carries the scope id only"))
        target = os.path.join(REPO, module, JSON_LEAF)
        written.append((module, document["$id"], owner[2] if owner else None))
        if mode == "apply":
            with open(target, "w", encoding="utf-8") as handle:
                json.dump(document, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
    print(json.dumps({"mode": mode, "written": len(written), "withoutOwner": len([row for row in skipped if "owning" in row[1]])}, ensure_ascii=False))
    for module, identity, export in written:
        print(f"  {identity:70} ← {module}   owner-export={export}")
    for module, why in skipped:
        print(f"  SKIP {module}: {why}")


if __name__ == "__main__":
    main()
