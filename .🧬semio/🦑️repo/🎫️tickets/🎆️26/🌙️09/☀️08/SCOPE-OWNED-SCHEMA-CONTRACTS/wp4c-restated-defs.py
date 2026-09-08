#!/usr/bin/env python3
"""🔁️ Row 111 — a scope never restates another scope's `$defs`.

Two restatement shapes live in `✏️s/**`:

* **facet child vs its own module root** — `🔺️diff/🔣️.json`, `📸️snapshot/🔣️.json`, `💡️inferences/🔣️.json`
  copy the root document's `$defs` verbatim. Same scope, so the copy is a `schema-export-id-duplicate`
  and steals the export's catalog row from the root file.
* **surface lane vs the subset module** — `✏️editor/🎚️config|👥️presence|🫧️transient/🧬️schema/🔣️.json`
  copies `$defs` the artifact subset owns. Different scopes, so the copy is a second declaration of a
  contract another scope owns.

Both are repaired the same way: delete the copy, and rewrite every `#/$defs/<Export>` pointer in the
copying document to `<owner $id>#/$defs/<Export>`.

Usage: python3 wp4c-restated-defs.py report|apply [--under PREFIX]
"""
from __future__ import annotations
import json, os, sys, collections

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *[".."] * 7))
LEAF = "🔣️.json"
FACETS = ("🔺️diff", "📸️snapshot", "💡️inferences")
LANES = ("🎚️config", "👥️presence", "🫧️transient")


def read(path: str) -> dict:
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return json.load(handle)


def write(path: str, document: dict) -> None:
    with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def comparable(definition, name: str):
    """A `$defs` entry modulo the redundant `title` a copy adds — the copy states the same contract."""
    if not isinstance(definition, dict):
        return definition
    if definition.get("title") != name:
        return definition
    return {key: value for key, value in definition.items() if key != "title"}


def stub(definition) -> bool:
    """A `$defs` entry that states a name and no contract — `{"type": "object"}` with no properties.

    Restatement waves left these behind on both sides: sometimes the module root holds the stub and the
    facet child the real shape, sometimes the reverse. The substantive side is the contract; the stub is
    an artefact of the copy, never the authority.
    """
    if not isinstance(definition, dict):
        return False
    if any(key in definition for key in ("enum", "const", "oneOf", "anyOf", "allOf", "$ref", "items", "patternProperties")):
        return False
    return definition.get("type") == "object" and not definition.get("properties")


def local_defs_referenced(node, found: set[str] | None = None) -> set[str]:
    found = set() if found is None else found
    if isinstance(node, list):
        for item in node:
            local_defs_referenced(item, found)
    elif isinstance(node, dict):
        for key, value in node.items():
            if key == "$ref" and isinstance(value, str) and value.startswith("#/$defs/"):
                found.add(value[len("#/$defs/"):])
            else:
                local_defs_referenced(value, found)
    return found


def rewrite_refs(node, owner_id: str, names: set[str]) -> int:
    changed = 0
    if isinstance(node, list):
        for item in node:
            changed += rewrite_refs(item, owner_id, names)
        return changed
    if not isinstance(node, dict):
        return 0
    for key, value in list(node.items()):
        if key == "$ref" and isinstance(value, str) and value.startswith("#/$defs/") and value[len("#/$defs/"):] in names:
            node[key] = f"{owner_id}#/$defs/{value[len('#/$defs/'):]}"
            changed += 1
        else:
            changed += rewrite_refs(value, owner_id, names)
    return changed


def pairs(under: str) -> list[tuple[str, str, str]]:
    """(copying document, owning document, relation) for every module root reachable under `under`."""
    found: list[tuple[str, str, str]] = []
    for base, dirs, files in os.walk(os.path.join(REPO, under)):
        dirs[:] = [name for name in dirs if name not in ("node_modules", ".venv")]
        if os.path.basename(base) != "🧬️schema" or LEAF not in files:
            continue
        module = os.path.relpath(base, REPO)
        if "/🧬️mutations/" in f"{module}/":
            continue
        root = f"{module}/{LEAF}"
        children: list[str] = []
        for inner, inner_dirs, inner_files in os.walk(base):
            inner_dirs[:] = [name for name in inner_dirs if name in FACETS or name in ("📝️text", "💾️binary")]
            relative = os.path.relpath(inner, base)
            if relative == "." or LEAF not in inner_files:
                continue
            children.append(f"{module}/{relative}/{LEAF}")
        children.sort(key=lambda path: path.encode())
        for child in children:
            found.append((child, root, "facet"))
        # 🪆️A facet's own representation children, then one facet against the next: the earlier document
        # in byte order keeps the export, exactly as the catalog's first-wins resolution would.
        for index, deeper in enumerate(children):
            for shallower in children[:index]:
                relation = "representation" if deeper.startswith(f"{os.path.dirname(shallower)}/") else "cross-facet"
                found.append((deeper, shallower, relation))
        owner = os.path.dirname(module)
        if os.path.basename(owner) in LANES:
            subset = os.path.dirname(os.path.dirname(owner))
            sibling = f"{subset}/🧬️schema/{LEAF}"
            if os.path.exists(os.path.join(REPO, sibling)):
                found.append((root, sibling, "lane"))
    rank = {"facet": 0, "representation": 1, "cross-facet": 2, "lane": 3}
    return sorted(found, key=lambda row: (row[0], rank[row[2]], row[1]))


def main() -> None:
    mode = sys.argv[1] if len(sys.argv) > 1 else "report"
    under = sys.argv[sys.argv.index("--under") + 1] if "--under" in sys.argv else "✏️s"
    identical = collections.Counter()
    drifted: list[tuple[str, str, str]] = []
    removed = refs = documents = 0
    for copy_path, owner_path, relation in pairs(under):
        try:
            copy_document, owner_document = read(copy_path), read(owner_path)
        except Exception as error:  # noqa: BLE001
            print(f"UNREADABLE {copy_path}: {error}")
            continue
        owner_id = owner_document.get("$id")
        copy_defs, owner_defs = copy_document.get("$defs") or {}, owner_document.get("$defs") or {}
        if not isinstance(owner_id, str) or not copy_defs:
            continue
        shared = [name for name in copy_defs if name in owner_defs]
        same = [name for name in shared if comparable(copy_defs[name], name) == comparable(owner_defs[name], name)]
        promote = [name for name in shared if name not in same and stub(owner_defs[name]) and not stub(copy_defs[name])]
        # 🧷️A promoted definition must keep resolving: every `#/$defs/<helper>` it carries has to exist
        # in the owner too, or the promotion would move a pointer away from what it points at.
        promote = [name for name in promote if all(target in owner_defs or target in promote for target in local_defs_referenced(copy_defs[name]))]
        yielded = [name for name in shared if name not in same and name not in promote and stub(copy_defs[name])]
        for name in shared:
            if name not in same and name not in promote and name not in yielded:
                drifted.append((copy_path, owner_path, name))
        resolved = same + promote + yielded
        if not resolved:
            continue
        identical[relation] += len(same)
        identical[f"{relation}-promoted"] += len(promote)
        identical[f"{relation}-stub-copy"] += len(yielded)
        if mode != "apply":
            continue
        for name in promote:
            owner_document["$defs"][name] = comparable(copy_defs[name], name) | ({"title": name} if owner_defs[name].get("title") == name else {})
        if promote:
            write(owner_path, owner_document)
        names = set(resolved)
        refs += rewrite_refs(copy_document, owner_id, names)
        for name in resolved:
            del copy_document["$defs"][name]
        if not copy_document["$defs"]:
            del copy_document["$defs"]
        write(copy_path, copy_document)
        removed += len(resolved)
        documents += 1
    print(json.dumps({"mode": mode, "under": under, "identicalRestatements": dict(identical), "drifted": len(drifted), "documentsRewritten": documents, "defsRemoved": removed, "refsRepointed": refs}, ensure_ascii=False))
    for row in drifted[:40]:
        print("DRIFTED", row[2], row[0], "vs", row[1])
    if len(drifted) > 40:
        print(f"… and {len(drifted) - 40} more drifted")


if __name__ == "__main__":
    main()
