#!/usr/bin/env python3
"""🩹️ S3-STDIO: adds the path-scoped `🩹️patch-snapshot` leaf (design §20.3) to one stdio mutation aggregate.

Usage (repo root): python3 T/🧪️s3-stdio-patch-leaves.py <subset dir relative to ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts> [--apply]

Writes the schema-first leaf (descriptor, payload schema, Rust leaf over `snapshot_patch_leaf!`), its wire witness, the aggregate's
module mount + variant + KINDS row, the catalog branch, the TypeScript union member, the binary protocol record, the text grammar
rule and the oracle catalog kind + manifest row. Codec match arms of hand-rolled aggregates are left to the compiler. Every step is
idempotent, so a partial run resumes. Without --apply it only prints the plan.
"""
import json, os, re, sys

ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
CONTRACT_TS = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts"
REGISTRY = "https://json.schemas.assets.semio-tech.com/s/stdio/registry/schema.json#/$defs/SnapshotPatch"
LABEL = {"label": {"en": "Snapshot edit", "de": "Änderung der Momentaufnahme"}, "description": {"en": "One path-scoped edit of the snapshot; its value is typed by the snapshot schema at the path.", "de": "Eine pfadbezogene Änderung der Momentaufnahme; ihr Wert ist durch das Schema der Momentaufnahme am Pfad typisiert."}}


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text, apply):
    if apply:
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(text)
    print(("wrote " if apply else "would write ") + path)


def dump(value):
    return json.dumps(value, indent=2, ensure_ascii=False) + "\n"


def wire(rename_all):
    return {"camelCase": "patchSnapshot", "kebab-case": "patch-snapshot", "snake_case": "patch_snapshot"}.get(rename_all, "PatchSnapshot")


def snapshot_schema(subset):
    own = os.path.join(subset, "🧬️schema/📸️snapshot/🔣️.json")
    if os.path.exists(own):
        return json.loads(read(own))["$id"]
    standard = os.path.dirname(subset)
    found = [os.path.join(standard, name, "🧬️schema/📸️snapshot/🔣️.json") for name in sorted(os.listdir(standard))]
    found = [path for path in found if os.path.exists(path)]
    if len(found) != 1:
        raise SystemExit(f"no unique snapshot schema for {subset}: {found}")
    return json.loads(read(found[0]))["$id"]


def leaf_imports(aggregate, enum, snapshot_type, diff_type):
    lines = [f"use super::{enum};"]
    for name in (snapshot_type, diff_type):
        statement = next((found for found in re.findall(r"^use [^;]*;", aggregate, re.M) if re.search(r"\b" + name + r"\b", found)), None)
        if statement is None or "super::" in statement:
            lines.append(f"use super::{name};")
            continue
        path = re.match(r"use (.*);", statement, re.S).group(1)
        lines.append(f"use {path[: path.index('{')]}{name};" if "{" in path else f"use {path};")
    return "\n".join(lines)


def main():
    subset = os.path.join(ROOT, sys.argv[1])
    apply = "--apply" in sys.argv
    mutations = os.path.join(subset, "🧬️schema/🧬️mutations")
    aggregate_path = os.path.join(mutations, "🦀️.rs")
    aggregate = read(aggregate_path)
    match = re.search(r"#\[derive\([^\]]*Mutations[^\]]*\)\]\s*((?:(?:#\[[^\]]*\]|//[^\n]*)\s*)*)pub enum (\w+)\s*\{", aggregate)
    attrs, enum = match.group(1), match.group(2)
    value = re.search(r"#\[value\(([^\]]*)\)\]", attrs)
    value = value.group(1) if value else ""
    tag = re.search(r'tag = "(\w+)"', value)
    content = re.search(r'content = "(\w+)"', value)
    rename_all = re.search(r'rename_all = "([\w-]+)"', value)
    layout = "external" if not tag else ("adjacent" if content else "internal")
    name = wire(rename_all.group(1) if rename_all else None)
    mutations_attr = re.search(r"#\[mutations\(snapshot = (\w+), diff = (\w+)", attrs)
    snapshot_type, diff_type = mutations_attr.group(1), mutations_attr.group(2)
    set_dir = next(entry for entry in sorted(os.listdir(mutations)) if entry.endswith("set-snapshot") and os.path.isdir(os.path.join(mutations, entry)))
    set_descriptor = json.loads(read(os.path.join(mutations, set_dir, "🔣️.json")))
    set_schema = json.loads(read(os.path.join(mutations, set_dir, "🧬️schema/🔣️.json")))
    leaf_id = f"{set_schema['$id'].rsplit('/mutation/', 1)[0]}/mutation/patch-snapshot/schema.json"
    leaf = os.path.join(mutations, "🩹️patch-snapshot")
    descriptor_path = os.path.join(leaf, "🔣️.json")
    protocol_path = os.path.join(mutations, "💾️binary/📡️.protocol.semio")
    if os.path.exists(descriptor_path):
        binary_tag = json.loads(read(descriptor_path)).get("binaryTag")
    else:
        tags = [json.loads(read(os.path.join(mutations, entry, "🔣️.json"))).get("binaryTag") for entry in os.listdir(mutations) if os.path.isfile(os.path.join(mutations, entry, "🔣️.json"))]
        tags = [found for found in tags if isinstance(found, int)]
        if tags and os.path.exists(protocol_path):
            tags += [int(found) for found in re.findall(r"^record [\w-]+ tag=(\d+)", read(protocol_path), re.M)]
        binary_tag = max(tags) + 1 if tags else None
    schema_id = snapshot_schema(subset)
    print(f"{enum}: layout={layout} wire={name} snapshot={snapshot_type} diff={diff_type} tag={binary_tag} leaf={leaf_id} snapshotSchema={schema_id}")

    if not os.path.exists(descriptor_path):
        write(descriptor_path, dump({
            "schemaVersion": 1, "owner": leaf, "semanticKind": "patch-snapshot", "displayName": "Patch Snapshot", "emoji": "🩹️",
            "aggregateVariant": "PatchSnapshot", "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": "patch-snapshot" if binary_tag else None, "binaryTag": binary_tag,
            "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "rejected"], "composition": "atomic",
            "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"] if binary_tag else set_descriptor["requiredLanguageSurfaces"],
        }), apply)
    if not os.path.exists(os.path.join(leaf, "🧬️schema/🔣️.json")):
        properties = {}
        if layout == "internal":
            properties[tag.group(1)] = {"const": name, "description": f"Aggregate discriminator spliced in by {enum}; absent when the payload stands alone."}
        properties["patch"] = {"$ref": REGISTRY, "x-semio-ui": LABEL}
        write(os.path.join(leaf, "🧬️schema/🔣️.json"), dump({
            "$schema": "http://json-schema.org/draft-07/schema#", "$id": leaf_id, "title": "PatchSnapshot",
            "description": f"🩹️ One path-scoped {enum} snapshot edit (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).",
            "type": "object", "additionalProperties": False, "required": ["patch"], "properties": properties,
            "$defs": {"Snapshot": {"$ref": schema_id, "description": "The snapshot document the path addresses; its sub-schema at the path types the edited value."}},
        }), apply)
    dsl_ops = "DslOps" in match.group(0)
    derives = "value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf" if dsl_ops else "value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf"
    keyword = '#[dsl(keyword = "patch-snapshot")]\n' if dsl_ops else ""
    block = "    #[dsl(block)]\n" if dsl_ops else ""
    write(os.path.join(leaf, "🦀️.rs"), f"""//! 🩹️ Path-scoped `{enum}` snapshot patch: one pointer operation with its exact inverse, its value typed by the snapshot
//! sub-schema at the pointer (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

{leaf_imports(aggregate, enum, snapshot_type, diff_type)}
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, {derives})]
#[mutation_leaf(contract = ::protocol, input_schema = Self::input_schema_at_path)]
{keyword}pub struct PatchSnapshot {{
{block}    pub patch: editing::SnapshotPatch,
}}

semio_s_artifact_stdio_contract::snapshot_patch_leaf! {{ leaf: PatchSnapshot, snapshot: {snapshot_type}, mutation: {enum}, diff: {diff_type}, snapshot_schema: "{schema_id}" }}
""", apply)
    witness_path = os.path.join(subset, "🧫️fixtures/🧬️mutations/🩹️patch-snapshot/🧾️wire-witness/🦠️mutation/🔣️.json")
    if not os.path.exists(witness_path):
        payload = {"patch": {"operation": "set", "path": "/schema", "value": "stdio.patch-snapshot.witness"}}
        witness = {"internal": lambda: {tag.group(1): name, **payload}, "adjacent": lambda: {tag.group(1): name, content.group(1): payload}, "external": lambda: {name: payload}}[layout]()
        write(witness_path, dump(witness), apply)

    module = '#[path = "🩹️patch-snapshot/🦀️.rs"]\npub mod patch_snapshot;\n'
    variant_path = "patch_snapshot::PatchSnapshot"
    set_module = re.search(r'#\[path = "' + re.escape(set_dir) + r'/🦀️\.rs"\]\n', aggregate)
    if set_module:
        if "pub mod patch_snapshot;" not in aggregate:
            aggregate = aggregate[:set_module.start()] + module + aggregate[set_module.start():]
    else:
        artifact = subset[len(ROOT) + 1:].split("/")[0]
        crate_root = os.path.join(ROOT, artifact, "🦀️.rs")
        relative = os.path.relpath(os.path.join(mutations, set_dir, "🦀️.rs"), os.path.join(ROOT, artifact))
        root_text = read(crate_root)
        mount = re.search(r'\n([ \t]*)#\[path = "' + re.escape(relative) + r'"\]\n', root_text)
        if mount is None:
            raise SystemExit(f"no mount of {relative} in {crate_root}")
        patch_relative = relative.replace(set_dir, "🩹️patch-snapshot")
        if patch_relative not in root_text:
            indent = mount.group(1)
            root_text = root_text[:mount.start()] + f'\n{indent}#[path = "{patch_relative}"]\n{indent}pub mod patch_snapshot;' + root_text[mount.start():]
            write(crate_root, root_text, apply)
        variant_path = "super::patch_snapshot::PatchSnapshot"
    if not re.search(r"\bPatchSnapshot\(", aggregate):
        enum_at = aggregate.index(f"pub enum {enum}")
        variant = re.search(r"\n(\s+)SetSnapshot\(([^)]*)\),\n", aggregate[enum_at:])
        start = enum_at + variant.end()
        aggregate = aggregate[:start] + f"{variant.group(1)}PatchSnapshot({variant_path}),\n" + aggregate[start:]
    if '"patch-snapshot"' not in aggregate:
        aggregate = re.sub(r'(pub const KINDS: &\[&str\] = &\[[^\]]*?"set-snapshot",)', r'\1 "patch-snapshot",', aggregate, count=1)
    write(aggregate_path, aggregate, apply)

    catalog_path = os.path.join(mutations, "🔣️.json")
    catalog = json.loads(read(catalog_path))
    if leaf_id not in json.dumps(catalog, ensure_ascii=False):
        branch = {"internal": lambda: {"$ref": leaf_id},
                  "adjacent": lambda: {"type": "object", "additionalProperties": False, "required": [tag.group(1), content.group(1)], "properties": {tag.group(1): {"const": name}, content.group(1): {"$ref": leaf_id}}},
                  "external": lambda: {"type": "object", "additionalProperties": False, "required": [name], "properties": {name: {"$ref": leaf_id}}}}[layout]()
        set_index = next(index for index, entry in enumerate(catalog["oneOf"]) if "/mutation/set-snapshot/" in json.dumps(entry, ensure_ascii=False))
        catalog["oneOf"].insert(set_index + 1, branch)
        write(catalog_path, dump(catalog), apply)

    ts_path = os.path.join(mutations, "🟦️.ts")
    if os.path.exists(ts_path) and "SnapshotPatch" not in read(ts_path):
        ts = read(ts_path)
        member = {"internal": lambda: f"{{ readonly {tag.group(1)}: '{name}'; readonly patch: SnapshotPatch }}",
                  "adjacent": lambda: f"{{ readonly {tag.group(1)}: '{name}'; readonly {content.group(1)}: {{ readonly patch: SnapshotPatch }} }}",
                  "external": lambda: f"{{ readonly {name}: {{ readonly patch: SnapshotPatch }} }}"}[layout]()
        lines = ts.split("\n")
        anchor = next((index for index, line in enumerate(lines) if line.lstrip().startswith("|") and re.search(r"set-?[Ss]napshot|SetSnapshot", line)), None)
        if anchor is not None:
            indent = lines[anchor][: len(lines[anchor]) - len(lines[anchor].lstrip())]
            lines.insert(anchor + 1, f"{indent}| {member}")
            imports = [index for index, line in enumerate(lines) if line.startswith("import ")]
            lines.insert((imports[-1] + 1) if imports else 0, f"import type {{ SnapshotPatch }} from '{os.path.relpath(CONTRACT_TS, mutations)}';")
            write(ts_path, "\n".join(lines), apply)
        else:
            print(f"TS union anchor not found in {ts_path}: add by hand: {member}")

    if binary_tag and os.path.exists(protocol_path) and "record patch-snapshot" not in read(protocol_path):
        protocol = read(protocol_path)
        record = re.search(r"^record set-snapshot tag=\d+\n((?:field [^\n]*\n)*)", protocol, re.M)
        write(protocol_path, protocol.rstrip("\n") + f"\nrecord patch-snapshot tag={binary_tag}\n{record.group(1) if record else 'field payload bytes' + chr(10)}", apply)

    grammar_path = os.path.join(mutations, "📝️text/📖️.grammar.semio")
    if binary_tag and os.path.exists(grammar_path) and '"patch-snapshot"' not in read(grammar_path):
        grammar = read(grammar_path)
        rule = re.search(r'^([\w-]*set-snapshot[\w-]*) = "set-snapshot"', grammar, re.M)
        if rule:
            name_rule = rule.group(1).replace("set-snapshot", "patch-snapshot")
            grammar = re.sub(r"^(\w[\w-]* = [^\n]*\b" + re.escape(rule.group(1)) + r"\b)", lambda found: found.group(1) + " | " + name_rule, grammar, count=1, flags=re.M)
            grammar = grammar.replace(rule.group(0), f'{name_rule} = "patch-snapshot" "patch" "=" hex\n{rule.group(0)}', 1)
            write(grammar_path, grammar, apply)
        else:
            print(f"grammar rule for set-snapshot not found in {grammar_path}")

    oracle_path = os.path.join(subset, "🔮️oracles/🔣️.json")
    if os.path.exists(oracle_path):
        oracle = json.loads(read(oracle_path))
        changed = False
        for catalog_entry in oracle.get("mutationCatalogs", []):
            kinds = catalog_entry.get("kinds", [])
            if "set-snapshot" in kinds and "patch-snapshot" not in kinds:
                kinds.insert(kinds.index("set-snapshot") + 1, "patch-snapshot")
                changed = True
        for manifest in oracle.get("mutationManifests", []):
            rows = manifest["mutations"]
            template = next((row for row in rows if row["id"] == "set-snapshot"), None)
            if template and not any(row["id"] == "patch-snapshot" for row in rows):
                row = json.loads(json.dumps(template))
                row["id"] = "patch-snapshot"
                row["outcomes"] = ["applied", "rejected"]
                row["productionDispatch"] = {**row.get("productionDispatch", {}), "operation": "patch-snapshot", "variant": "PatchSnapshot"}
                if "payloadSchema" in row:
                    row["payloadSchema"] = row["payloadSchema"].replace("SetSnapshot", "PatchSnapshot")
                rows.insert(rows.index(template) + 1, row)
                changed = True
        if changed:
            write(oracle_path, dump(oracle), apply)


if __name__ == "__main__":
    main()
