#!/usr/bin/env python3
"""🔧️ Moves a document's module-internal shapes out of `$defs` and into `definitions`.

Execution contract §A: `$defs` is the module's **exported** surface, `definitions` is where
module-internal helpers live, and only cross-document references have to address an export. A shape
that

- no other document references by `<$id>#/$defs/<Name>`,
- no fixture binds with `schema://<scope>/<Name>`,
- no sibling format file of the module declares under that name, and
- is not the document's own root export (`title`)

is therefore not an export at all: it is a piece of this document's own composition. Keeping it in
`$defs` makes every format of the scope owe an entity for it — a debt the module never took on.

The shape itself is not touched: it moves node-for-node, and every `#/$defs/<Name>` pointer in the
same document becomes `#/definitions/<Name>`.

Usage: python3 wp4c-helper-demote.py report|apply [--under ✏️s/🔌️plugins/🏛️architect]
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
JSON_LEAF = "🔣️.json"
LEAF = {"🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🛰️protobuf": "🛰️.proto"}
GRAPHQL_KEYWORDS = "type|input|enum|interface|union|scalar"
EXPORT_RE = re.compile(r"^[A-Z][A-Za-z0-9]*$")


def declares(fmt: str, source: str, name: str) -> bool:
    escaped = re.escape(name)
    if fmt == "🛰️protobuf":
        return re.search(rf"^\s*message\s+{escaped}\b", source, re.M) is not None
    if fmt == "🔗️graphql":
        return re.search(rf"^\s*(?:{GRAPHQL_KEYWORDS})\s+{escaped}\b", source, re.M) is not None
    if fmt == "🦀️rust":
        return re.search(rf"^\s*pub\s+(?:struct|enum|type)\s+{escaped}\b", source, re.M) is not None or re.search(rf"^\s*pub use\s+[A-Za-z_][A-Za-z0-9_:]*::{escaped};", source, re.M) is not None
    return re.search(rf"^\s*export\s+(?:interface|type|const|class)\s+{escaped}\b", source, re.M) is not None


def module_of(path: str) -> str | None:
    parts = path.split("/")
    for index in range(len(parts) - 1, -1, -1):
        if parts[index] == "🧬️schema":
            return "/".join(parts[: index + 1])
    return None


def repo_json() -> list[str]:
    out = subprocess.run(["git", "ls-files", "-z"], cwd=REPO, capture_output=True, check=True).stdout
    return [entry.decode() for entry in out.split(b"\0") if entry.endswith(b".json")]


def pointer_sites() -> dict[str, set[str]]:
    """📌 Every file that spells `#/$defs/<Name>`, whatever its language: a TypeScript oracle that
    compiles `<id>#/$defs/X` addresses the export just as a JSON `$ref` does."""
    out = subprocess.run(["git", "grep", "-oh", "-E", r"#/\$defs/[A-Za-z0-9]+", "--", "."], cwd=REPO, capture_output=True)
    sites: dict[str, set[str]] = {}
    listing = subprocess.run(["git", "grep", "-l", "-E", r"#/\$defs/[A-Za-z0-9]+", "--", "."], cwd=REPO, capture_output=True).stdout.decode().splitlines()
    for path in listing:
        try:
            with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                text = handle.read()
        except (OSError, UnicodeDecodeError):
            continue
        for name in set(re.findall(r"#/\$defs/([A-Za-z0-9]+)", text)):
            sites.setdefault(name, set()).add(path)
    return sites


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    addressed: set[tuple[str, str]] = set()
    bound: set[str] = set()
    for path in repo_json():
        try:
            with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                text = handle.read()
        except OSError:
            continue
        for target, name in re.findall(r'"(https://semio\.tech/schema/[^"#]+)#/\$defs/([A-Za-z0-9]+)"', text):
            addressed.add((target, name))
        for _, name in re.findall(r'"schema://([a-z0-9.\-]+)/([A-Za-z0-9]+)"', text):
            bound.add(name)

    sites = pointer_sites()
    counts = collections.Counter()
    moved_rows: list[str] = []
    touched: set[str] = set()
    for base, dirs, files in os.walk(os.path.join(REPO, under)):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv", "target", "dist")]
        if JSON_LEAF not in files:
            continue
        path = os.path.relpath(os.path.join(base, JSON_LEAF), REPO)
        module = module_of(path)
        if module is None or "/🧬️mutations/" in f"{path}" or "/🧪️" in f"/{path}" or "/🧫️" in f"/{path}":
            continue
        try:
            with open(os.path.join(REPO, path), encoding="utf-8") as handle:
                text = handle.read()
                document = json.loads(text)
        except Exception:  # noqa: BLE001
            continue
        exports = document.get("$defs")
        identity = document.get("$id")
        if not isinstance(exports, dict) or not isinstance(identity, str):
            continue
        facet = os.path.dirname(path[len(module) + 1:])
        sources = {}
        for fmt, leaf in LEAF.items():
            candidate = os.path.join(REPO, module, facet, leaf) if facet else os.path.join(REPO, module, leaf)
            if os.path.exists(candidate):
                with open(candidate, encoding="utf-8") as handle:
                    sources[fmt] = handle.read()
        internal = []
        for name in list(exports):
            if not EXPORT_RE.match(name) or name == document.get("title"):
                continue
            if (identity, name) in addressed or name in bound:
                continue
            definition = exports[name]
            if isinstance(definition, dict) and "x-semio-formats" in definition:
                continue
            if sites.get(name, set()) - {path}:
                continue
            if any(declares(fmt, source, name) for fmt, source in sources.items()):
                continue
            internal.append(name)
        if not internal:
            continue
        counts["documents"] += 1
        counts["demoted"] += len(internal)
        moved_rows.extend(f"{name} {path}" for name in internal)
        if mode != "apply":
            continue
        helpers = document.get("definitions") if isinstance(document.get("definitions"), dict) else {}
        for name in internal:
            helpers[name] = exports.pop(name)
        document["definitions"] = dict(sorted(helpers.items()))
        if not exports:
            document.pop("$defs")
        rendered = json.dumps(document, ensure_ascii=False, indent=2)
        for name in internal:
            rendered = rendered.replace(f'"#/$defs/{name}"', f'"#/definitions/{name}"')
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(f"{rendered}\n")
        touched.add(path)
    print(json.dumps({"mode": mode, "under": under, **counts, "filesTouched": len(touched)}, ensure_ascii=False))
    for row in moved_rows[:25]:
        print("  DEMOTE", row)


if __name__ == "__main__":
    main()
