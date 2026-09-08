#!/usr/bin/env python3
"""🧬 WP4 mutation payload-schema census and canonicalizer.

Scans every `🧬️mutations/` root outside `✏️s/🔌️plugins/🗄️stdio/` (the sibling worker's partition)
and outside `.🧬semio/` ticket captures. Classifies each aggregate `🔣️.json` as G-A (inline payload
`$defs` / stale artifact-snapshot body) or G-B (pure `$ref` union), and each leaf by payload-schema
filename family (A `🧬️.schema.json`, B `🧬️schema/🔣️.json`, C `🔣️.schema.json`, D vcs payload+wire,
E sequence payload+wire, F missing).

`--apply` relocates every plugin leaf payload schema to the taxonomy default `<leaf>/🧬️schema/🔣️.json`,
merges D/E wire envelopes into `$defs.Wire` beside `$defs.Payload`, stamps draft-07 + `$id` + `title`,
rewrites descriptors, and turns every plugin aggregate into a pure relative-`$ref` `oneOf` union.

`--ids` is the idempotent contract §A `$id`-grammar pass, run on its own (`--ids` reports, `--ids --apply`
writes). It touches nothing but `$id`/`title` and rewrites, for every document of a `🧬️mutations` tree:

    leaf   <leaf>/🧬️schema/🔣️.json   → https://semio.tech/schema/<root scope>/mutation/<semanticKind>/schema.json
    root   <root>/🔣️.json            → https://semio.tech/schema/<root scope>/mutations.json
    facet  <root>/<facet>/🔣️.json    → https://semio.tech/schema/<root scope>/mutations/<facet>.json

A mutation leaf is a scope of its own, so its facet filename is `schema`; the aggregate and its text/binary
facet documents keep the root scope path and vary only the facet filename.

See `<ticket>/📋️execution-contract.md` §A/§B and `<ticket>/📓️wp0-mutation-leaves.md` §6.
"""
from __future__ import annotations
import collections, json, os, re, shutil, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
STDIO = "✏️s/🔌️plugins/🗄️stdio/"
TICKETS = ".🧬semio/"
MUT = "/🧬️mutations/"
DRAFT7 = "http://json-schema.org/draft-07/schema#"
TARGET_REL = "🧬️schema/🔣️.json"
FACETS = {"🔺️diff", "↩️inverse", "📝️text", "💾️binary", "🦠️mutation", "🧩️plan", "🧬️schema", "🧬️wire", "🛜️wire"}
WIRE_DIRS = ("🧬️wire", "🛜️wire")
NON_LEAF_DIRS = {"🧪️tests", "🧪️fixtures", "🧪️descriptor", "🧫️fixtures", "🧫️fixture"}
DROP_SEGMENTS = {"🗿️artifacts", "🏅️standards", "🪆️subsets", "🧬️schema", "🧬️mutations", "🔌️plugins", "🔨️modules", "🛍️products", "🧩️extensions"}


def tracked() -> list[str]:
    """📃️ Working-tree file list: tracked plus untracked-not-ignored, minus deleted."""
    listed = subprocess.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=REPO, capture_output=True, check=True)
    return sorted({p for p in listed.stdout.decode().split("\0") if p and os.path.isfile(os.path.join(REPO, p))})


def load(rel: str):
    with open(os.path.join(REPO, rel), encoding="utf-8") as handle:
        return json.load(handle)


def save(rel: str, doc) -> None:
    path = os.path.join(REPO, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(doc, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def ascii_tail(segment: str) -> str:
    index = 0
    while index < len(segment):
        char, following = segment[index], segment[index + 1] if index + 1 < len(segment) else ""
        if ord(char) > 0x2000 or following in ("️", "⃣"):
            index += 1
        else:
            break
    return segment[index:] or segment


def scope_id(root: str) -> str:
    parts = []
    for segment in root.split("/"):
        if segment in DROP_SEGMENTS:
            continue
        parts.append("s" if segment == "✏️s" else "framework" if segment == "🧰️framework" else ascii_tail(segment))
    return "/".join(parts)


def mutation_roots(files: list[str]) -> dict[str, list[str]]:
    grouped = collections.defaultdict(list)
    for path in files:
        if MUT not in path or path.startswith(STDIO) or path.startswith(TICKETS):
            continue
        grouped[path[: path.index(MUT) + len(MUT) - 1]].append(path)
    return dict(grouped)


def leaf_index(grouped: dict[str, list[str]], present: set[str]) -> dict[str, dict]:
    leaves = {}
    for root, paths in grouped.items():
        for path in paths:
            if not path.endswith("/🔣️.json"):
                continue
            directory = os.path.dirname(path)
            if directory == root:
                continue
            relative = directory[len(root) + 1:].split("/")
            if any(part in FACETS or part in NON_LEAF_DIRS for part in relative):
                continue
            try:
                descriptor = load(path)
            except Exception:
                continue
            if "payloadSchema" not in descriptor or "semanticKind" not in descriptor:
                continue
            leaves[directory] = {"root": root, "descriptorRel": path, "descriptor": descriptor}
    return leaves


def family_of(leaf: str, descriptor: dict, present: set[str]) -> str:
    declared = descriptor["payloadSchema"]
    exists = f"{leaf}/{declared}" in present
    wire = [name for name in WIRE_DIRS if f"{leaf}/{name}/🔣️.schema.json" in present]
    if not exists:
        return "F"
    if wire:
        return "D" if wire[0] == "🧬️wire" else "E"
    return {"🧬️.schema.json": "A", TARGET_REL: "B", "🔣️.schema.json": "C", "📋️.schema.json": "D"}.get(declared, "?")


def strip_dialect(node, depth_shift: int):
    """🧹 Draft-07 normalization: drop `unevaluated*`, lift `prefixItems`, deepen relative `$ref`s."""
    if isinstance(node, dict):
        out = {}
        for key, value in node.items():
            if key in ("unevaluatedProperties", "unevaluatedItems"):
                continue
            if key == "prefixItems":
                out["items"] = [strip_dialect(item, depth_shift) for item in value]
                out["additionalItems"] = False
                continue
            if key == "$ref" and isinstance(value, str) and not value.startswith("#") and not value.startswith("http"):
                out[key] = "../" * depth_shift + value
                continue
            out[key] = strip_dialect(value, depth_shift)
        return out
    if isinstance(node, list):
        return [strip_dialect(item, depth_shift) for item in node]
    return node


def ordered(schema_id: str, title: str, body: dict) -> dict:
    out = {"$schema": DRAFT7, "$id": schema_id, "title": title}
    for key, value in body.items():
        if key in ("$schema", "$id", "title"):
            continue
        out[key] = value
    return out


def apply_leaf(leaf: str, info: dict, family: str, present: set[str], report: dict) -> None:
    descriptor = info["descriptor"]
    declared = descriptor["payloadSchema"]
    scope = scope_id(info["root"])
    schema_id = f"https://semio.tech/schema/{scope}/mutation/{descriptor['semanticKind']}.json"
    title = descriptor["aggregateVariant"]
    target = f"{leaf}/{TARGET_REL}"
    old_paths = []
    if family == "B":
        body = strip_dialect(load(target), 0)
        doc = ordered(schema_id, title, body)
    elif family in ("A", "C"):
        source = f"{leaf}/{declared}"
        body = strip_dialect(load(source), 1)
        doc = ordered(schema_id, title, body)
        old_paths.append(source)
    elif family in ("D", "E"):
        source = f"{leaf}/{declared}"
        wire_dir = "🧬️wire" if family == "D" else "🛜️wire"
        wire_rel = f"{leaf}/{wire_dir}/🔣️.schema.json"
        payload_raw = load(source)
        payload_id = payload_raw.get("$id")
        payload = strip_dialect({k: v for k, v in payload_raw.items() if k not in ("$schema", "$id", "title")}, 1)
        wire_text = json.dumps(strip_dialect({k: v for k, v in load(wire_rel).items() if k not in ("$schema", "$id")}, 0), ensure_ascii=False)
        if payload_id:
            wire_text = wire_text.replace(payload_id + "#", "#/$defs/Payload")
        doc = ordered(schema_id, title, {"$ref": "#/$defs/Payload", "$defs": {"Payload": payload, "Wire": json.loads(wire_text)}})
        old_paths += [source, wire_rel]
    else:
        return
    save(target, doc)
    for old in old_paths:
        if old != target:
            os.remove(os.path.join(REPO, old))
            report["removed"].append(old)
    for wire_dir in WIRE_DIRS:
        directory = os.path.join(REPO, leaf, wire_dir)
        if os.path.isdir(directory) and not os.listdir(directory):
            os.rmdir(directory)
    if descriptor["payloadSchema"] != TARGET_REL:
        descriptor["payloadSchema"] = TARGET_REL
        save(info["descriptorRel"], descriptor)
        report["descriptors"] += 1
    report["relocated"][family] += 1


def apply_aggregate(root: str, leaves: dict[str, dict], report: dict, aggregate_rel: str) -> None:
    members = sorted(((leaf, info) for leaf, info in leaves.items() if info["root"] == root), key=lambda item: item[1]["descriptor"]["semanticKind"])
    if not members:
        return
    previous = load(aggregate_rel) if os.path.exists(os.path.join(REPO, aggregate_rel)) else {}
    scope = scope_id(root)
    doc = {"$schema": DRAFT7, "$id": previous.get("$id") or f"https://semio.tech/schema/{scope}/mutation.json", "title": previous.get("title") or "".join(part.capitalize() for part in re.split(r"[/\-]", scope)) + "Mutation"}
    if previous.get("description"):
        doc["description"] = previous["description"]
    doc["oneOf"] = [{"$ref": leaf_schema_id(scope, info["descriptor"]["semanticKind"])} for _, info in members]
    save(aggregate_rel, doc)
    report["aggregates"] += 1


FACET_DOC_DIRS = ("📝️text", "💾️binary")


def leaf_schema_id(scope: str, semantic_kind: str) -> str:
    """🆔️ Contract §A: a mutation leaf is its own scope, facet `schema`."""
    return f"https://semio.tech/schema/{scope}/mutation/{semantic_kind}/schema.json"


def aggregate_schema_id(scope: str) -> str:
    """🆔️ Contract §A: the union document is the root scope's `mutations` facet."""
    return f"https://semio.tech/schema/{scope}/mutations.json"


def facet_schema_id(scope: str, facet_dir: str) -> str:
    """🆔️ Contract §A: a facet document never deepens the scope, it only varies the filename."""
    return f"https://semio.tech/schema/{scope}/mutations/{ascii_tail(facet_dir)}.json"


def relref(files: list[str], grouped: dict[str, list[str]], leaves: dict[str, dict], present: set[str], apply: bool) -> dict:
    """🔗️ Turns an aggregate's absolute-URL `$ref` into the contract §B relative path to its own leaf.

    An aggregate that names its leaves by `$id` URL breaks the moment the leaf `$id` grammar changes.
    A leaf whose URL no longer resolves is matched back to a sibling leaf of the same root by
    `semanticKind`; an unresolvable ref with no unique sibling is refused, never guessed.
    """
    index = document_index(files)
    report = {"aggregates": 0, "rewritten": 0, "refused": [], "edits": []}
    for root in sorted(grouped):
        rel = f"{root}/🔣️.json"
        if rel not in present:
            continue
        doc = load(rel)
        by_kind = {info["descriptor"]["semanticKind"]: leaf for leaf, info in leaves.items() if info["root"] == root}
        changed = False

        def rewrite(node):
            nonlocal changed
            if isinstance(node, list):
                return [rewrite(item) for item in node]
            if not isinstance(node, dict):
                return node
            out = {}
            for key, value in node.items():
                if key == "$ref" and isinstance(value, str) and value.startswith("http"):
                    target, _, pointer = value.partition("#")
                    if target not in index:
                        kind = target.rstrip("/").rsplit("/", 1)[-1].removesuffix(".json")
                        leaf = by_kind.get(kind)
                        if leaf is None:
                            report["refused"].append(f"{rel}: {value} resolves to nothing and no sibling leaf is named {kind}")
                            out[key] = value
                            continue
                        replacement = f"./{leaf[len(root) + 1:]}/{TARGET_REL}" + (f"#{pointer}" if pointer else "")
                        report["edits"].append(f"{rel}: {value} → {replacement}")
                        report["rewritten"] += 1
                        changed = True
                        out[key] = replacement
                        continue
                    out[key] = value
                else:
                    out[key] = rewrite(value)
            return out

        updated = rewrite(doc)
        report["aggregates"] += 1
        if changed and apply:
            save(rel, updated)
    return report


def pascal(files: list[str], apply: bool) -> dict:
    """🔠️ Contract §A: an export id is a PascalCase `$defs` key, so `payload` becomes `Payload`.

    A lowercase key is not addressable as an export, which is why the aggregate branches that name it
    read as unresolved. Only the leading character moves; a key that is still not PascalCase afterwards,
    or that would collide with an existing export, is refused. Intra-document `#/$defs/<key>` pointers are
    carried along here; cross-document ones are repaired by `--absref` (see `repoint`).
    """
    report = {"documents": 0, "exports": 0, "pointers": 0, "refused": [], "files": []}
    for rel in sorted(files):
        if MUT not in rel or rel.startswith(STDIO) or rel.startswith(TICKETS) or not rel.endswith(".json"):
            continue
        try:
            doc = load(rel)
        except Exception:
            continue
        if not isinstance(doc, dict) or "$schema" not in doc or not isinstance(doc.get("$defs"), dict):
            continue
        report["documents"] += 1
        renames = {}
        for key in doc["$defs"]:
            if re.fullmatch(r"[A-Z][A-Za-z0-9]*", key):
                continue
            candidate = key[:1].upper() + key[1:]
            if not re.fullmatch(r"[A-Z][A-Za-z0-9]*", candidate) or candidate in doc["$defs"]:
                report["refused"].append(f"{rel}: $defs key {key!r} has no unambiguous PascalCase export id")
                continue
            renames[key] = candidate
        if not renames:
            continue
        pointers = 0

        def rewrite(node):
            nonlocal pointers
            if isinstance(node, list):
                return [rewrite(item) for item in node]
            if not isinstance(node, dict):
                return node
            out = {}
            for key, value in node.items():
                if key == "$ref" and isinstance(value, str) and value.startswith("#/$defs/"):
                    head, _, tail = value[len("#/$defs/"):].partition("/")
                    if head in renames:
                        pointers += 1
                        out[key] = f"#/$defs/{renames[head]}" + (f"/{tail}" if tail else "")
                        continue
                out[key] = rewrite(value)
            return out

        updated = rewrite(doc)
        updated["$defs"] = {renames.get(key, key): value for key, value in updated["$defs"].items()}
        report["exports"] += len(renames)
        report["pointers"] += pointers
        report["files"].append(rel)
        if apply:
            save(rel, updated)
    return report


def stray(files: list[str], apply: bool) -> dict:
    """🧽 Removes a `$schema` declared on a SUBschema, which names a dialect no validator switches to.

    Draft-07 only reads `$schema` at the document root; a nested one is a leftover of the 2020-12 era that
    makes a document look half-migrated to any reader that greps for the dialect string.
    """
    report = {"documents": 0, "keys": 0, "files": []}
    for rel in sorted(files):
        if MUT not in rel or rel.startswith(STDIO) or rel.startswith(TICKETS) or not rel.endswith(".json"):
            continue
        try:
            doc = load(rel)
        except Exception:
            continue
        if not isinstance(doc, dict) or "$schema" not in doc:
            continue
        report["documents"] += 1
        removed = 0

        def prune(node, root: bool):
            nonlocal removed
            if isinstance(node, list):
                return [prune(item, False) for item in node]
            if not isinstance(node, dict):
                return node
            out = {}
            for key, value in node.items():
                if key == "$schema" and not root:
                    removed += 1
                    continue
                out[key] = prune(value, False)
            return out

        updated = prune(doc, True)
        if removed:
            report["keys"] += removed
            report["files"].append(rel)
            if apply:
                save(rel, updated)
    return report


def dropkinds(files: list[str], apply: bool) -> dict:
    """🧹 Cross-partition row 23: the aggregate's `$ref` union IS the identity, so the restated kind list goes.

    Unblocked once the structural aggregate check landed (`policyMutationAggregateMembers` in the root
    `📜️script.ts`, and `mutation-aggregate-kinds-redundant` in `📚️library/🔍️discovery`). With row 79 every
    branch names `…/mutation/<semanticKind>/schema.json`, so even the textual identity fallbacks still read
    the kind out of the union. Refuses any aggregate whose declared kinds are not exactly what its own
    branches name, because dropping the list would then lose information.
    """
    report = {"aggregates": 0, "dropped": 0, "refused": []}
    for rel in sorted(files):
        if MUT not in rel or rel.startswith(STDIO) or rel.startswith(TICKETS) or not rel.endswith(f"{MUT[:-1]}/🔣️.json"):
            continue
        try:
            doc = load(rel)
        except Exception:
            continue
        if not isinstance(doc, dict) or "x-semio-mutationKinds" not in doc:
            continue
        report["aggregates"] += 1
        declared = doc["x-semio-mutationKinds"]
        branches = [branch.get("$ref", "") for branch in doc.get("oneOf", []) if isinstance(branch, dict)]
        carried = [kind for kind in declared if any(f"/mutation/{kind}/schema.json" in ref for ref in branches)]
        if len(carried) != len(declared):
            report["refused"].append(f"{rel}: {sorted(set(declared) - set(carried))} are not named by any branch $ref")
            continue
        report["dropped"] += 1
        if apply:
            save(rel, {k: v for k, v in doc.items() if k != "x-semio-mutationKinds"})
    return report


def repoint(base: str, pointer: str, rel: str, report: dict) -> str:
    """🔤️ Keeps a `#/$defs/<Export>` pointer valid across an export-id re-casing in the target module.

    Contract §A export ids are PascalCase; peers re-case a module's `$defs` keys as their scope lands,
    which silently invalidates every cross-document pointer that still spells the old key. A pointer whose
    key is gone is repaired ONLY when exactly one key of the target differs from it by case; anything else
    is reported, never guessed.
    """
    match = re.fullmatch(r"/\$defs/([^/]+)", pointer or "")
    if not match:
        return pointer
    if "_index" not in report:
        report["_index"] = {identity: doc for identity, (_, doc) in document_index(report["_files"]).items()}
    target = report["_index"].get(base)
    if not isinstance(target, dict):
        return pointer
    exports = target.get("$defs") or {}
    if match.group(1) in exports:
        return pointer
    equal = [key for key in exports if key.lower() == match.group(1).lower()]
    if len(equal) != 1:
        report["refused"].append(f"{rel}: {base}#{pointer} names an export the target does not declare ({sorted(exports)})")
        return pointer
    report["repointed"] = report.get("repointed", 0) + 1
    return f"/$defs/{equal[0]}"


def absref(files: list[str], apply: bool, remap: dict[str, str] | None = None) -> dict:
    """🔗️ Cross-partition row 79: every filesystem-relative `$ref` names the target document's `$id`.

    A mutation leaf is a scope of its own (contract §A), so an aggregate branch is a CROSS-document
    reference and must address the target by `$id`, never by a path. The rewrite is mechanical and
    lossless: the JSON pointer is carried over untouched, and a target that declares no `$id` is
    refused rather than guessed at, because there is no id to name it by.
    """
    report = {"documents": 0, "refs": 0, "rewritten": 0, "aggregates": 0, "leaves": 0, "refused": [], "edits": [], "_files": files}
    for rel in sorted(files):
        if MUT not in rel or rel.startswith(STDIO) or rel.startswith(TICKETS) or not rel.endswith(".json"):
            continue
        try:
            doc = load(rel)
        except Exception:
            continue
        if not isinstance(doc, dict) or "$schema" not in doc:
            continue
        report["documents"] += 1
        changed = 0

        def rewrite(node):
            nonlocal changed
            if isinstance(node, list):
                return [rewrite(item) for item in node]
            if not isinstance(node, dict):
                return node
            out = {}
            for key, value in node.items():
                if key == "$ref" and isinstance(value, str) and value.startswith("http"):
                    base, _, pointer = value.partition("#")
                    base = (remap or {}).get(base, base)
                    pointer = repoint(base, pointer, rel, report)
                    replacement = base + (f"#{pointer}" if pointer else "")
                    if replacement != value:
                        changed += 1
                        report["remapped"] = report.get("remapped", 0) + 1
                        report["edits"].append(f"{rel}: {value} → {replacement}")
                    out[key] = replacement
                    continue
                if key == "$ref" and isinstance(value, str) and not value.startswith("#") and not value.startswith("http"):
                    report["refs"] += 1
                    base, _, pointer = value.partition("#")
                    target = os.path.normpath(os.path.join(os.path.dirname(rel), __import__("urllib.parse", fromlist=["unquote"]).unquote(base)))
                    try:
                        identity = load(target).get("$id")
                    except Exception:
                        identity = None
                    if not isinstance(identity, str):
                        report["refused"].append(f"{rel}: {value} → {target} declares no $id")
                        out[key] = value
                        continue
                    replacement = identity + (f"#{pointer}" if pointer else "")
                    if replacement != value:
                        changed += 1
                        report["rewritten"] += 1
                        report["edits"].append(f"{rel}: {value} → {replacement}")
                    out[key] = replacement
                    continue
                out[key] = rewrite(value)
            return out

        updated = rewrite(doc)
        if changed:
            report["aggregates" if rel.endswith(f"{MUT[:-1]}/🔣️.json") else "leaves"] += 1
            if apply:
                save(rel, updated)
    return report


DIALECT_2020 = "https://json-schema.org/draft/2020-12/schema"
ANNOTATIONS = {"$id", "$schema", "$defs", "definitions", "title", "description", "$comment", "examples", "default", "deprecated", "readOnly", "writeOnly"}


class Unresolvable(Exception):
    """🚧️ A `$ref` whose evaluated property set cannot be established statically."""


def document_index(files: list[str]) -> dict[str, tuple[str, dict]]:
    """🗂️ Every JSON Schema document in the tree, keyed by `$id`, for cross-file `$ref` resolution."""
    index: dict[str, tuple[str, dict]] = {}
    for rel in files:
        if not rel.endswith(".json") or rel.startswith(".🧬semio/") or "/node_modules/" in rel:
            continue
        try:
            with open(os.path.join(REPO, rel), "rb") as probe:
                if b'"$id"' not in probe.read(4096):
                    continue
            doc = load(rel)
        except Exception:
            continue
        if isinstance(doc, dict) and isinstance(doc.get("$id"), str):
            index.setdefault(doc["$id"], (rel, doc))
    return index


def resolve_ref(ref: str, base_rel: str, index: dict[str, tuple[str, dict]]) -> tuple[str, dict]:
    """🔗️ Resolves a `$ref` (absolute `$id` URL or repo-relative path, with optional JSON pointer)."""
    target, _, pointer = ref.partition("#")
    if target == "":
        doc_rel, doc = base_rel, load(base_rel)
    elif target.startswith("http"):
        if target not in index:
            raise Unresolvable(ref)
        doc_rel, doc = index[target]
    else:
        doc_rel = os.path.normpath(os.path.join(os.path.dirname(base_rel), __import__("urllib.parse", fromlist=["unquote"]).unquote(target)))
        if not os.path.isfile(os.path.join(REPO, doc_rel)):
            raise Unresolvable(ref)
        doc = load(doc_rel)
    node = doc
    for step in [part for part in pointer.split("/") if part]:
        step = step.replace("~1", "/").replace("~0", "~")
        if not isinstance(node, dict) or step not in node:
            raise Unresolvable(ref)
        node = node[step]
    return doc_rel, node


def evaluated_names(node, base_rel: str, index: dict[str, tuple[str, dict]], seen: frozenset = frozenset()) -> set[str]:
    """🔑️ The property names a draft 2020-12 `unevaluatedProperties` would count as evaluated here.

    Only the statically decidable applicators appear in this partition: `properties`, `$ref`, `allOf`.
    Anything else (`patternProperties`, `additionalProperties`, `if`/`then`, `oneOf`, `anyOf`, `not`)
    raises instead of guessing, so a migration never loosens a schema it did not understand.
    """
    if not isinstance(node, dict):
        raise Unresolvable(repr(node)[:80])
    names: set[str] = set(node.get("properties", {}))
    for keyword in ("patternProperties", "additionalProperties", "if", "then", "else", "anyOf", "not", "dependentSchemas"):
        if keyword in node:
            raise Unresolvable(f"{base_rel}: unevaluated closure over {keyword}")
    if "oneOf" in node:
        raise Unresolvable(f"{base_rel}: unevaluated closure over oneOf")
    if isinstance(node.get("$ref"), str):
        key = (base_rel, node["$ref"])
        if key in seen:
            raise Unresolvable(f"{base_rel}: cyclic $ref {node['$ref']}")
        target_rel, target = resolve_ref(node["$ref"], base_rel, index)
        names |= evaluated_names(target, target_rel, index, seen | {key})
    for branch in node.get("allOf", []):
        names |= evaluated_names(branch, base_rel, index, seen)
    return names


def to_draft07(node, base_rel: str, index: dict[str, tuple[str, dict]], report: dict, root: bool):
    """🕰️ Rewrites one node to draft-07 without loosening it.

    Two 2020-12-only behaviours are in play. `$ref` beside other keywords is evaluated in 2020-12 and
    IGNORED in draft-07, so every such `$ref` moves into an `allOf` branch. `unevaluatedProperties:
    false` has no draft-07 keyword; the same closure is `propertyNames` over the enumerated set of
    names the composition evaluates — exact here because every branch constrains `properties` only.
    Per-branch `additionalProperties: false` would NOT be equivalent: it would reject the sibling
    branch's own properties.
    """
    if isinstance(node, list):
        return [to_draft07(item, base_rel, index, report, False) for item in node]
    if not isinstance(node, dict):
        return node
    out = {key: to_draft07(value, base_rel, index, report, False) for key, value in node.items()}
    if root:
        out["$schema"] = DRAFT7
    if "unevaluatedItems" in out:
        raise Unresolvable(f"{base_rel}: unevaluatedItems")
    closed = out.pop("unevaluatedProperties", None)
    if isinstance(out.get("$ref"), str) and any(key not in ANNOTATIONS and key != "$ref" for key in out) | (closed is not None):
        out["allOf"] = [{"$ref": out.pop("$ref")}, *out.get("allOf", [])]
        report["refsLifted"] += 1
    if closed is not None:
        if closed is not False:
            raise Unresolvable(f"{base_rel}: unevaluatedProperties is not false")
        branches = out.get("oneOf")
        if isinstance(branches, list) and branches and all(isinstance(branch, dict) and ("propertyNames" in branch or branch.get("additionalProperties") is False) for branch in branches):
            # 🪆️A closure over a `oneOf` whose every branch already closes can never fire: the winning
            # branch's evaluated set is also the parent's, so the outer keyword is dropped, not translated.
            report["redundantClosures"] = report.get("redundantClosures", 0) + 1
            return out
        names = sorted(evaluated_names(out, base_rel, index))
        if "additionalProperties" in out:
            raise Unresolvable(f"{base_rel}: additionalProperties beside unevaluatedProperties")
        # 🈳️An empty evaluated set means "no property is allowed"; draft-07's meta-schema forbids an
        # empty `enum`, and the boolean schema says the same thing without one.
        out["propertyNames"] = {"enum": names} if names else False
        report["closuresRewritten"] += 1
    return out


def draft07(files: list[str], grouped: dict[str, list[str]], present: set[str], apply: bool) -> dict:
    """🕰️ Migrates every 2020-12 document of the partition's mutation trees to draft-07."""
    index = document_index(files)
    report = {"documents": 0, "migrated": 0, "refsLifted": 0, "closuresRewritten": 0, "refused": [], "files": []}
    for rel in sorted(files):
        if MUT not in rel or rel.startswith(STDIO) or rel.startswith(TICKETS) or not rel.endswith(".json"):
            continue
        try:
            doc = load(rel)
        except Exception:
            continue
        if not isinstance(doc, dict) or doc.get("$schema") != DIALECT_2020:
            continue
        report["documents"] += 1
        before = {"refsLifted": report["refsLifted"], "closuresRewritten": report["closuresRewritten"]}
        try:
            out = to_draft07(doc, rel, index, report, True)
        except Unresolvable as refusal:
            report["refsLifted"], report["closuresRewritten"] = before["refsLifted"], before["closuresRewritten"]
            report["refused"].append(str(refusal))
            continue
        report["migrated"] += 1
        report["files"].append(rel)
        if apply:
            save(rel, out)
    return report


ID_BASE = "https://semio.tech/schema/"


def module_scope_path(root: str, present: set[str]) -> str | None:
    """🧭️ The scope path a mutation root inherits, read from its own module's `🧬️schema/🔣️.json` `$id`.

    Contract §A: a scope id comes from the module `$id`, never from a second path heuristic — so the
    leaf grammar of row 10/78 (`<root module $id scope path>/mutation/<kind>/schema.json`) reads the one
    declaration instead of re-deriving it. A root whose module declares no `$id` has no scope path, and
    the caller refuses rather than inventing one.
    """
    module_rel = f"{os.path.dirname(root)}/🔣️.json"
    if module_rel not in present:
        return None
    try:
        identity = load(module_rel).get("$id")
    except Exception:
        return None
    if not isinstance(identity, str) or not identity.startswith(ID_BASE) or "/" not in identity[len(ID_BASE):]:
        return None
    return identity[len(ID_BASE):].rsplit("/", 1)[0]


def reidentify(grouped: dict[str, list[str]], leaves: dict[str, dict], present: set[str], apply: bool) -> dict:
    """🆔️ Idempotent `$id` rewrite of every mutation document to the contract §A grammar.

    Injectivity is established BEFORE anything is written: the whole `$id → [document]` multimap is built
    first, and every id claimed twice is refused together with all its claimants, so a collision can never
    be written and then discovered.
    """
    report = {"leaves": 0, "leavesChanged": 0, "aggregates": 0, "aggregatesChanged": 0, "facets": 0, "facetsChanged": 0, "titles": 0, "collisions": [], "unowned": [], "mapping": {}}
    seen: dict[str, str] = {}

    claims: dict[str, list[str]] = collections.defaultdict(list)
    for leaf, info in sorted(leaves.items()):
        scope = module_scope_path(info["root"], present)
        if scope is None:
            continue
        claims[leaf_schema_id(scope, info["descriptor"]["semanticKind"])].append(f"{leaf}/{TARGET_REL}")
    for root in sorted(grouped):
        scope = module_scope_path(root, present)
        if scope is None:
            continue
        if f"{root}/🔣️.json" in present:
            claims[aggregate_schema_id(scope)].append(f"{root}/🔣️.json")
        for facet in FACET_DOC_DIRS:
            if f"{root}/{facet}/🔣️.json" in present:
                claims[facet_schema_id(scope, facet)].append(f"{root}/{facet}/🔣️.json")
    refused = {identity for identity, claimants in claims.items() if len(claimants) > 1}
    for identity in sorted(refused):
        report["collisions"].append(f"{identity} is claimed by {len(claims[identity])} documents: {', '.join(claims[identity])}")

    def stamp(rel: str, schema_id: str, title: str | None, dialect: str | None = None) -> tuple[bool, bool]:
        doc = load(rel)
        if dialect is not None and "$schema" not in doc:
            doc = {**doc, "$schema": dialect}
            report["dialectStamped"] = report.get("dialectStamped", 0) + 1
        before_id, before_title = doc.get("$id"), doc.get("title")
        body = {k: v for k, v in doc.items() if k not in ("$schema", "$id", "title")}
        if title is not None and isinstance(before_title, str) and before_title != title and "description" not in body:
            # 🏷️A leaf `title` is the export id; prose that sat there is kept as the document's description.
            body = {"description": before_title, **body}
            report["prosePreserved"] = report.get("prosePreserved", 0) + 1
        head = {}
        if "$schema" in doc:
            head["$schema"] = doc["$schema"]
        head["$id"] = schema_id
        head["title"] = title if title is not None else (before_title if before_title is not None else None)
        if head["title"] is None:
            del head["title"]
        out = {**head, **body}
        if before_id in seen and seen[before_id] != rel:
            report["collisions"].append(f"{rel}: shares previous $id {before_id} with {seen[before_id]}")
        if before_id:
            seen[before_id] = rel
            if before_id != schema_id:
                report["mapping"][before_id] = schema_id
        if apply and out != doc:
            save(rel, out)
        return before_id != schema_id, before_title != head.get("title")

    for leaf, info in sorted(leaves.items()):
        rel = f"{leaf}/{TARGET_REL}"
        if rel not in present:
            continue
        report["leaves"] += 1
        scope = module_scope_path(info["root"], present)
        if scope is None:
            report["unowned"].append(rel)
            continue
        identity = leaf_schema_id(scope, info["descriptor"]["semanticKind"])
        if identity in refused:
            continue
        changed, title_changed = stamp(rel, identity, info["descriptor"].get("aggregateVariant"))
        report["leavesChanged"] += int(changed)
        report["titles"] += int(title_changed)

    for root in sorted(grouped):
        scope = module_scope_path(root, present)
        rel = f"{root}/🔣️.json"
        if rel in present:
            report["aggregates"] += 1
            if scope is None:
                report["unowned"].append(rel)
            elif aggregate_schema_id(scope) not in refused:
                changed, _ = stamp(rel, aggregate_schema_id(scope), None, DRAFT7)
                report["aggregatesChanged"] += int(changed)
        if scope is None:
            continue
        for facet in FACET_DOC_DIRS:
            facet_rel = f"{root}/{facet}/🔣️.json"
            if facet_rel not in present:
                continue
            report["facets"] += 1
            changed, _ = stamp(facet_rel, facet_schema_id(scope, facet), None, DRAFT7)
            report["facetsChanged"] += int(changed)

    final = collections.Counter()
    for leaf in sorted(leaves):
        rel = f"{leaf}/{TARGET_REL}"
        if rel not in present:
            continue
        try:
            identity = load(rel).get("$id")
        except Exception:
            identity = None
        if isinstance(identity, str):
            final[identity] += 1
    report["duplicateTargets"] = {k: v for k, v in final.items() if v > 1}
    return report


def main() -> int:
    apply = "--apply" in sys.argv
    only_plugins = "--plugins-only" in sys.argv
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else None
    files = tracked()
    present = set(files)
    grouped = mutation_roots(files)
    leaves = leaf_index(grouped, present)
    families = {leaf: family_of(leaf, info["descriptor"], present) for leaf, info in leaves.items()}

    aggregates = {}
    for root in sorted(grouped):
        rel = f"{root}/🔣️.json"
        if rel not in present and f"{root}/🔣️.schema.json" not in present:
            aggregates[root] = {"present": False, "scope": scope_id(root)}
            continue
        rel = rel if rel in present else f"{root}/🔣️.schema.json"
        doc = load(rel)
        defs = doc.get("$defs") or doc.get("definitions") or {}
        inline = [k for k, v in defs.items() if not (isinstance(v, dict) and "$ref" in v)]
        shape = "oneOf-union" if "oneOf" in doc else "allOf" if "allOf" in doc else "uninhabited" if "not" in doc else "object-snapshot" if doc.get("type") == "object" and "properties" in doc else "other"
        kind = "G-B" if not inline and shape in ("oneOf-union", "allOf", "uninhabited") else "G-A"
        aggregates[root] = {"present": True, "file": rel, "kind": kind, "shape": shape, "inlineDefs": inline, "scope": scope_id(root)}

    if "--relref" in sys.argv:
        rewired = relref(files, grouped, leaves, present, apply)
        print(json.dumps(rewired, ensure_ascii=False, indent=1))
        return 1 if rewired["refused"] else 0

    if "--pascal" in sys.argv:
        cased = pascal(files, apply)
        print(json.dumps({k: (v if k != "files" else len(v)) for k, v in cased.items()}, ensure_ascii=False, indent=1))
        return 1 if cased["refused"] else 0

    if "--stray" in sys.argv:
        pruned = stray(files, apply)
        print(json.dumps(pruned, ensure_ascii=False, indent=1))
        return 0

    if "--dropkinds" in sys.argv:
        cleaned = dropkinds(files, apply)
        print(json.dumps(cleaned, ensure_ascii=False, indent=1))
        return 1 if cleaned["refused"] else 0

    if "--absref" in sys.argv:
        remap = None
        if "--remap" in sys.argv:
            with open(sys.argv[sys.argv.index("--remap") + 1], encoding="utf-8") as handle:
                remap = json.load(handle)["mapping"]
        rewired = absref(files, apply, remap)
        for internal in ("_files", "_index"):
            rewired.pop(internal, None)
        print(json.dumps({k: (v if k not in ("edits",) else len(v)) for k, v in rewired.items()}, ensure_ascii=False, indent=1))
        if "--out" in sys.argv:
            with open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8") as handle:
                json.dump(rewired, handle, ensure_ascii=False, indent=1)
        return 1 if rewired["refused"] else 0

    if "--draft07" in sys.argv:
        migration = draft07(files, grouped, present, apply)
        print(json.dumps({k: (v if k not in ("files",) else len(v)) for k, v in migration.items()}, ensure_ascii=False, indent=1))
        if "--out" in sys.argv:
            with open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8") as handle:
                json.dump(migration, handle, ensure_ascii=False, indent=1)
        return 1 if migration["refused"] else 0

    if "--ids" in sys.argv:
        identity = reidentify(grouped, leaves, present, apply)
        print(json.dumps({k: v for k, v in identity.items() if k != "mapping"}, ensure_ascii=False, indent=1))
        if "--out" in sys.argv:
            with open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8") as handle:
                json.dump(identity, handle, ensure_ascii=False, indent=1)
        return 1 if identity["collisions"] or identity["duplicateTargets"] else 0

    report = {"relocated": collections.Counter(), "removed": [], "descriptors": 0, "aggregates": 0}
    if apply:
        for leaf, info in sorted(leaves.items()):
            if only_plugins and not leaf.startswith("✏️s/"):
                continue
            if only and not leaf.startswith(only):
                continue
            apply_leaf(leaf, info, families[leaf], present, report)
        for root, meta in sorted(aggregates.items()):
            if only_plugins and not root.startswith("✏️s/"):
                continue
            if only and not root.startswith(only):
                continue
            if not meta["present"]:
                continue
            apply_aggregate(root, leaves, report, meta["file"])
        print(json.dumps({"relocated": dict(report["relocated"]), "removedFiles": len(report["removed"]), "descriptorsRewritten": report["descriptors"], "aggregatesRewritten": report["aggregates"]}, ensure_ascii=False, indent=1))

    summary = {
        "mutationRoots": len(grouped),
        "aggregatesPresent": sum(1 for a in aggregates.values() if a["present"]),
        "aggregateKinds": collections.Counter(a["kind"] for a in aggregates.values() if a["present"]),
        "aggregateShapes": collections.Counter(a["shape"] for a in aggregates.values() if a["present"]),
        "leaves": len(leaves),
        "families": collections.Counter(families.values()),
        "gaByScope": collections.Counter(a["scope"].split("/")[1] if a["scope"].startswith("s/") else a["scope"].split("/")[0] for a in aggregates.values() if a.get("kind") == "G-A"),
    }
    if "--out" in sys.argv:
        detail = {"summary": {k: (dict(v) if isinstance(v, collections.Counter) else v) for k, v in summary.items()},
                  "aggregates": aggregates,
                  "leaves": {leaf: {"root": info["root"], "family": families[leaf], "payloadSchema": info["descriptor"]["payloadSchema"], "semanticKind": info["descriptor"]["semanticKind"], "aggregateVariant": info["descriptor"]["aggregateVariant"], "scope": scope_id(info["root"])} for leaf, info in leaves.items()}}
        with open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8") as handle:
            json.dump(detail, handle, ensure_ascii=False, indent=1)
    print(json.dumps({k: (dict(v) if isinstance(v, collections.Counter) else v) for k, v in summary.items()}, ensure_ascii=False, indent=1))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
