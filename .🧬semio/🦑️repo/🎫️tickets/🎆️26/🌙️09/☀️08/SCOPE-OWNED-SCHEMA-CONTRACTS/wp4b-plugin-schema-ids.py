#!/usr/bin/env python3
"""🆔 WP4b helper — contract §A/§B `$id` grammar and draft-07 dialect for the `✏️s/**` schema modules.

Every `🧬️schema` module outside a `🧬️mutations`/`🧪️`/`🧫️` tree gets

  `$id` = `https://semio.tech/schema/<scope path>/<facet>.json`   (scope path derived from the module path)
  `$schema` = `http://json-schema.org/draft-07/schema#`           (2020-12 keywords migrated)

and every facet child (`📸️snapshot`, `🔺️diff`, `💡️inferences`, `📝️text`, `💾️binary`) keeps the module's
scope path and varies only the facet filename. Surface modules (`✏️editor/🎚️config|👥️presence|🫧️transient`)
take the surface as the last scope segment so config and presence stop colliding on one scope id.
`🗄️stdio` subset modules take the full `s/stdio/<artifact>/<standard>/<subset>` path the mutation leaves
(`wp4-stdio-schemas.py`) already write, so the leaf grammar resolves against its enclosing module.

Usage: wp4b-plugin-schema-ids.py plan|apply|refs
  plan   — print every `$id`/dialect change without writing
  apply  — write the modules
  refs   — rewrite `$ref`s repo-wide that name an id this pass moved (reads `🗑️generated/wp4b-id-map.json`)
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter, OrderedDict

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..", ".."))
ROOT = "✏️s"
ID_BASE = "https://semio.tech/schema/"
DRAFT07 = "http://json-schema.org/draft-07/schema#"
SCHEMA_DIR = "🧬️schema"
FACET_DIRS = {"📸️snapshot": "snapshot", "🔺️diff": "diff", "💡️inferences": "inference", "🧬️mutations": "mutations"}
REPRESENTATION_DIRS = {"📝️text": "text", "💾️binary": "binary"}
SURFACES = {"🎚️config": "config", "👥️presence": "presence", "🫧️transient": "transient"}
ID_MAP = os.path.join(os.path.dirname(__file__), "🗑️generated", "wp4b-id-map.json")


def strip_leading_emoji(name: str) -> str:
    """🏷️ Drops one directory name's leading emoji/keycap marker, keeping the semantic remainder."""
    markers = ("️", "︎", "⃣", "‍")
    index = 0
    while index < len(name):
        character = name[index]
        if character in markers:
            index += 1
            continue
        if index + 1 < len(name) and name[index + 1] in ("️", "⃣"):
            index += 1
            continue
        if ord(character) >= 0x1F000 or 0x2190 <= ord(character) <= 0x2BFF:
            index += 1
            continue
        break
    return name[index:]


def to_draft07(node):
    """♻️ 2020-12 → draft-07: tuple validation, and drop keywords draft-07 does not know."""
    if isinstance(node, list):
        return [to_draft07(item) for item in node]
    if not isinstance(node, dict):
        return node
    out = OrderedDict()
    prefix = node.get("prefixItems")
    for key, value in node.items():
        if key == "prefixItems":
            out["items"] = [to_draft07(item) for item in value]
            continue
        if key == "items" and prefix is not None:
            out["additionalItems"] = False if value is False else to_draft07(value)
            continue
        if key in ("unevaluatedProperties", "unevaluatedItems", "$anchor", "$dynamicRef", "$dynamicAnchor"):
            continue
        out[key] = to_draft07(value)
    return out


def module_paths():
    """📇 Every eligible `🧬️schema` module directory in the plugins partition."""
    found = []
    for directory, names, files in os.walk(os.path.join(REPO, ROOT)):
        names[:] = [name for name in names if name not in ("target", "node_modules", ".venv", "dist")]
        relative = os.path.relpath(directory, REPO)
        segments = relative.split(os.sep)
        if segments[-1] != SCHEMA_DIR:
            continue
        if any(segment == "🧬️mutations" for segment in segments):
            continue
        if any(segment.startswith("🧪️") or segment.startswith("🧫️") for segment in segments):
            continue
        found.append(relative)
    return sorted(found)


def scope_path(module: str, current: list[str] | None) -> list[str]:
    """🧭 The scope path this module owns, derived from its own directory, never from an override table."""
    segments = module.split(os.sep)[:-1]
    plugin = strip_leading_emoji(segments[2]) if len(segments) > 2 else None
    if segments[-1] in SURFACES and current:
        return [*current, SURFACES[segments[-1]]]
    if "🗿️artifacts" in segments and plugin == "stdio":
        index = segments.index("🗿️artifacts")
        if len(segments) >= index + 6 and segments[index + 2] == "🏅️standards" and segments[index + 4] == "🪆️subsets":
            return ["s", "stdio", strip_leading_emoji(segments[index + 1]),
                    strip_leading_emoji(segments[index + 3]), strip_leading_emoji(segments[index + 5])]
    if current:
        return current
    if len(segments) == 3:
        return ["s", plugin]
    return ["s", plugin, strip_leading_emoji(segments[-1])]


def parse_id(value):
    """🔍 The `(scope path, facet)` halves of one contract `$id`, or `None` when it is not one."""
    if not isinstance(value, str) or not value.startswith(ID_BASE):
        return None
    parts = value[len(ID_BASE):].split("/")
    if len(parts) < 2 or not parts[-1].endswith(".json"):
        return None
    return parts[:-1], parts[-1]


def facet_name(module: str, document: str, root_facet: str) -> str:
    """🏷️ The facet filename one module-relative document declares (`snapshot/text.json`, `diff.json`, …)."""
    relative = os.path.relpath(document, module).split(os.sep)[:-1]
    if not relative:
        return root_facet
    names = []
    for segment in relative:
        if segment in FACET_DIRS:
            names.append(FACET_DIRS[segment])
        elif segment in REPRESENTATION_DIRS:
            names.append(REPRESENTATION_DIRS[segment])
        else:
            return None
    return "/".join(names) + ".json"


def documents(module: str):
    """📄 Every canonical JSON leaf this module owns, root first, facet children after."""
    found = []
    for directory, names, files in os.walk(os.path.join(REPO, module)):
        names[:] = [name for name in names
                    if name in FACET_DIRS or name in REPRESENTATION_DIRS or name == "🧬️mutations"]
        if "🔣️.json" in files:
            found.append(os.path.relpath(os.path.join(directory, "🔣️.json"), REPO))
    return sorted(found, key=lambda path: path.count(os.sep))


def default_root_facet(module: str) -> str:
    segments = module.split(os.sep)[:-1]
    if segments[-1] in SURFACES:
        return "schema.json"
    if "🪆️subsets" in segments:
        return "artifact.json"
    return "schema.json"


def main():
    action = sys.argv[1] if len(sys.argv) > 1 else "plan"
    if action == "refs":
        return rewrite_refs()
    moved = {}
    counts = Counter()
    for module in module_paths():
        leaves = documents(module)
        if not leaves:
            continue
        root = os.path.join(module, "🔣️.json")
        if root not in leaves:
            continue
        with open(os.path.join(REPO, root), encoding="utf-8") as handle:
            root_document = json.load(handle, object_pairs_hook=OrderedDict)
        current = parse_id(root_document.get("$id"))
        path = scope_path(module, current[0] if current else None)
        root_facet = default_root_facet(module) if module.split(os.sep)[-2] in SURFACES else (current[1] if current else default_root_facet(module))
        for leaf in leaves:
            facet = facet_name(module, leaf, root_facet)
            if facet is None:
                counts["facet-unknown"] += 1
                print(f"  SKIP (facet not derivable) {leaf}")
                continue
            with open(os.path.join(REPO, leaf), encoding="utf-8") as handle:
                document = json.load(handle, object_pairs_hook=OrderedDict)
            wanted = f"{ID_BASE}{'/'.join(path)}/{facet}"
            before_id, before_dialect = document.get("$id"), document.get("$schema")
            if before_id != wanted:
                counts["id"] += 1
                if isinstance(before_id, str):
                    moved.setdefault(before_id, []).append(wanted)
                if action == "plan":
                    print(f"  ID   {leaf}\n       {before_id} -> {wanted}")
            if before_dialect != DRAFT07:
                counts["dialect"] += 1
                if action == "plan":
                    print(f"  DIAL {leaf}: {before_dialect!r} -> draft-07")
            if action != "apply":
                continue
            document = to_draft07(document)
            ordered = OrderedDict()
            ordered["$schema"] = DRAFT07
            ordered["$id"] = wanted
            for key, value in document.items():
                if key in ("$schema", "$id"):
                    continue
                ordered[key] = value
            with open(os.path.join(REPO, leaf), "w", encoding="utf-8") as handle:
                json.dump(ordered, handle, ensure_ascii=False, indent=2)
                handle.write("\n")
    print(f"modules: {len(module_paths())}  id-rewrites: {counts['id']}  dialect-rewrites: {counts['dialect']}  "
          f"skipped-facets: {counts['facet-unknown']}  action: {action}")
    if action == "apply":
        os.makedirs(os.path.dirname(ID_MAP), exist_ok=True)
        with open(ID_MAP, "w", encoding="utf-8") as handle:
            json.dump(moved, handle, ensure_ascii=False, indent=2, sort_keys=True)
        print(f"wrote {len(moved)} moved ids to {os.path.relpath(ID_MAP, REPO)}")


def rewrite_refs():
    """🔗 Repoints every `$ref`/reader literal that names an id this pass moved."""
    with open(ID_MAP, encoding="utf-8") as handle:
        moved = json.load(handle)
    ambiguous = sorted(old for old, targets in moved.items() if len(set(targets)) > 1)
    for old in ambiguous:
        print(f"  AMBIGUOUS (was a duplicate id, refs left alone): {old}")
    moved = {old: sorted(set(targets))[0] for old, targets in moved.items()
             if len(set(targets)) == 1 and old != sorted(set(targets))[0]}
    touched = Counter()
    for directory, names, files in os.walk(REPO):
        names[:] = [name for name in names
                    if name not in ("target", "node_modules", ".venv", "dist", ".git") and not name.startswith(".git")]
        if os.sep + ".🧬semio" in os.sep + os.path.relpath(directory, REPO):
            continue
        for name in files:
            if not name.endswith((".json", ".ts", ".rs", ".js", ".tsx", ".md", ".graphql", ".proto")):
                continue
            path = os.path.join(directory, name)
            try:
                with open(path, encoding="utf-8") as handle:
                    text = handle.read()
            except (UnicodeDecodeError, OSError):
                continue
            changed = text
            for old, new in moved.items():
                if old in changed:
                    changed = changed.replace(old, new)
            if changed != text:
                with open(path, "w", encoding="utf-8") as handle:
                    handle.write(changed)
                touched[os.path.relpath(path, REPO)] += 1
    for path in sorted(touched):
        print(f"    {path}")
    print(f"ref rewrites in {len(touched)} files over {len(moved)} moved ids")


if __name__ == "__main__":
    main()
