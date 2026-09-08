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

See `<ticket>/📋️execution-contract.md` §B and `<ticket>/📓️wp0-mutation-leaves.md` §6.
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
    doc["oneOf"] = [{"$ref": f"./{leaf[len(root) + 1:]}/{TARGET_REL}"} for leaf, _ in members]
    doc["x-semio-mutationKinds"] = [info["descriptor"]["semanticKind"] for _, info in members]
    save(aggregate_rel, doc)
    report["aggregates"] += 1


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
