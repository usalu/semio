#!/usr/bin/env python3
"""🧩️ Split the glTF 2.0 `any`-subset's five combined mutation-vocabulary aggregate files (JSON
Schema oneOf, GraphQL enum, Protobuf oneof, TypeScript discriminated union) into eight per-domain
-subset aggregates, one per new subset directory, now that each mutation's own leaf has already
moved there. The Rust barrel is untouched deliberately -- see the shard report for why."""
import json, re, os

REPO = "/Users/ueli/Documents/semio"
TICKET = f"{REPO}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ANY = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any"
STANDARDS_ROOT = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets"

with open(f"{TICKET}/🗑️generated/a6-gltf-subset-mapping.json", encoding="utf-8") as f:
    mapping = json.load(f)  # dirname -> {ascii, subset}

by_ascii = {v["ascii"]: v["subset"] for v in mapping.values()}
by_dirname_subset = {k: v["subset"] for k, v in mapping.items()}
SUBSETS = sorted(set(by_ascii.values()))
assert len(SUBSETS) == 8, SUBSETS

def subset_dir(subset):
    d = f"{STANDARDS_ROOT}/✳️{subset}/🧬️schema/🧬️mutations"
    os.makedirs(d, exist_ok=True)
    return d

def screaming_to_ascii(s):
    return s.strip().lower().replace("_", "-")

def pascal(subset):
    return subset[:1].upper() + subset[1:]

#region JSON Schema
with open(f"{ANY}/🧬️schema/🧬️mutations/🔣️.json", encoding="utf-8") as f:
    schema = json.load(f)

buckets = {s: [] for s in SUBSETS}
for entry in schema["oneOf"]:
    const = entry["properties"]["mutation"]["const"]
    subset = by_ascii[const]
    buckets[subset].append(entry)

json_counts = {}
for subset, entries in buckets.items():
    out = {
        "$schema": schema["$schema"],
        "$id": f"{schema['$id']}/{subset}",
        "title": f"glTF {pascal(subset)} Mutation",
        "oneOf": entries,
    }
    path = f"{subset_dir(subset)}/🔣️.json"
    with open(path, "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=2)
        f.write("\n")
    json_counts[subset] = len(entries)
print("json", json_counts, sum(json_counts.values()))
#endregion

#region TypeScript
with open(f"{ANY}/🧬️schema/🧬️mutations/🟦️.ts", encoding="utf-8") as f:
    ts_text = f.read()

header_match = re.match(r"(/\*\*.*?\*/\n)", ts_text, re.S)
ts_header = header_match.group(1) if header_match else ""

import_re = re.compile(r"^import type \{ (\w+) \} from '\./(.+?)/🟦️\.ts';$", re.M)
union_re = re.compile(r"^  \| \{ readonly mutation: '(\w+)'; readonly payload: (\w+) \}[;,]?$", re.M)

imports_by_dir = {}
for typ, dirname in import_re.findall(ts_text):
    imports_by_dir[dirname] = typ

union_by_tag = {}
for tag, typ in union_re.findall(ts_text):
    union_by_tag[tag] = typ

ts_import_buckets = {s: [] for s in SUBSETS}
for dirname, typ in imports_by_dir.items():
    subset = by_dirname_subset[dirname]
    ts_import_buckets[subset].append(f"import type {{ {typ} }} from './{dirname}/🟦️.ts';")

def camel(ascii_name):
    parts = ascii_name.split("-")
    return parts[0] + "".join(p.title() for p in parts[1:])

ts_union_buckets = {s: [] for s in SUBSETS}
for dirname, info in mapping.items():
    tag = camel(info["ascii"])
    typ = union_by_tag.get(tag)
    if typ is None:
        raise SystemExit(f"no union entry for tag {tag} ({dirname})")
    ts_union_buckets[info["subset"]].append(f"  | {{ readonly mutation: '{tag}'; readonly payload: {typ} }}")

ts_counts = {}
for subset in SUBSETS:
    imports = "\n".join(sorted(ts_import_buckets[subset]))
    union = " |\n".join(u.lstrip(" |") for u in ts_union_buckets[subset])
    body = (
        f"/** 🧬 Transparent TypeScript aggregate for the {subset} slice of the glTF 2.0 mutation vocabulary. */\n"
        f"{imports}\n\n"
        f"export type Gltf{pascal(subset)}Mutation =\n"
        + "\n".join(ts_union_buckets[subset]) + ";\n"
    )
    path = f"{subset_dir(subset)}/🟦️.ts"
    with open(path, "w", encoding="utf-8") as f:
        f.write(body)
    ts_counts[subset] = len(ts_union_buckets[subset])
print("ts", ts_counts, sum(ts_counts.values()))
#endregion

#region GraphQL
with open(f"{ANY}/🧬️schema/🧬️mutations/🔗️.graphql", encoding="utf-8") as f:
    gql_text = f.read()

enum_body = re.search(r"enum GltfMutationKind \{\n(.*?)\n\}", gql_text, re.S).group(1)
enum_values = [l.strip() for l in enum_body.splitlines() if l.strip()]

gql_buckets = {s: [] for s in SUBSETS}
for val in enum_values:
    ascii_name = screaming_to_ascii(val)
    subset = by_ascii[ascii_name]
    gql_buckets[subset].append(val)

gql_counts = {}
for subset in SUBSETS:
    values = gql_buckets[subset]
    comment = "\n".join(f"# {screaming_to_ascii(v)}" for v in values)
    body = (
        f"# 🧬 {pascal(subset)} slice of the glTF direct mutation discriminator roster.\n"
        f"{comment}\n"
        f"enum Gltf{pascal(subset)}MutationKind {{\n"
        + "\n".join(f"  {v}" for v in values) + "\n}\n"
        f"scalar Gltf{pascal(subset)}MutationPayload\n"
        f"input Gltf{pascal(subset)}MutationInput {{ kind: Gltf{pascal(subset)}MutationKind!, payload: Gltf{pascal(subset)}MutationPayload! }}\n"
    )
    path = f"{subset_dir(subset)}/🔗️.graphql"
    with open(path, "w", encoding="utf-8") as f:
        f.write(body)
    gql_counts[subset] = len(values)
print("graphql", gql_counts, sum(gql_counts.values()))
#endregion

#region Protobuf
with open(f"{ANY}/🧬️schema/🧬️mutations/🛰️.proto", encoding="utf-8") as f:
    proto_text = f.read()

import_re = re.compile(r'^import "(.+?)/🛰️\.proto";$', re.M)
proto_imports_by_dir = {}
for dirname in import_re.findall(proto_text):
    proto_imports_by_dir[dirname] = True

field_re = re.compile(r"^    (\w+) (\w+) = (\d+);$", re.M)
fields = field_re.findall(proto_text)  # (TypeName, field_name, number)

proto_buckets = {s: [] for s in SUBSETS}
for typename, field_name, _num in fields:
    ascii_name = screaming_to_ascii_dummy = field_name.replace("_", "-")
    subset = by_ascii.get(ascii_name)
    if subset is None:
        raise SystemExit(f"no subset for proto field {field_name}")
    proto_buckets[subset].append((typename, field_name))

# map ascii name -> directory name for import lines
dirname_by_ascii = {v["ascii"]: k for k, v in mapping.items()}

proto_counts = {}
for subset in SUBSETS:
    entries = proto_buckets[subset]
    imports = "\n".join(
        f'import "{dirname_by_ascii[fn.replace("_", "-")]}/🛰️.proto";' for _t, fn in entries
    )
    oneof_lines = "\n".join(f"    {t} {fn} = {i + 1};" for i, (t, fn) in enumerate(entries))
    body = (
        'syntax = "proto3";\n'
        f"package stdio.gltf.mutation.{subset};\n"
        f"{imports}\n\n"
        f"message Gltf{pascal(subset)}Mutation {{\n"
        "  oneof mutation {\n"
        f"{oneof_lines}\n"
        "  }\n"
        "}\n"
    )
    path = f"{subset_dir(subset)}/🛰️.proto"
    with open(path, "w", encoding="utf-8") as f:
        f.write(body)
    proto_counts[subset] = len(entries)
print("proto", proto_counts, sum(proto_counts.values()))
#endregion
