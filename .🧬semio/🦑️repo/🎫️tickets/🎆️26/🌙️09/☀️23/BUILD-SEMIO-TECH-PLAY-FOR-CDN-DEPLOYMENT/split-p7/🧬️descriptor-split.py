#!/usr/bin/env python3
"""🧬️ Splits an artifact's schema descriptor into facet-owned leaf constants (split lane P7, wave 0; the P2 precedent).

`<x>_artifact_schema_descriptor()` in `🧬️schema/🦀️.rs` embeds the rust/ts/graphql/json/proto leaves of four facets. Each
facet now owns its leaves: `ARTIFACT_LEAVES` in the schema module, `SNAPSHOT_LEAVES` in `📸️snapshot`, `DIFF_LEAVES` in
`🔺️diff` and `MUTATION_LEAVES` in `🧬️mutations`, so no facet crate embeds another facet's files. The descriptor function
moves to the artifact root (the aggregator) and assembles the four constants; the move is recorded for the family cut.

usage: 🧬️descriptor-split.py <artifact dir (repo-relative)> <out moves record> plan|apply
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
artifact, record, mode = sys.argv[1:4]
A = os.path.join(REPO, artifact)
S = os.path.join(A, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
MOD = "crate::standards::v1::subsets::any::schema"
FACETS = [("artifact", "ARTIFACT_LEAVES", "", MOD, "🧬️ The artifact facet's schema leaves (rust, typescript, graphql, json schema, proto)."),
          ("snapshot", "SNAPSHOT_LEAVES", "📸️snapshot", MOD + "::snapshot", "📸️ The snapshot facet's schema leaves (rust, typescript, graphql, json schema, proto)."),
          ("diff", "DIFF_LEAVES", "🔺️diff", MOD + "::diff", "🔺️ The diff facet's schema leaves (rust, typescript, graphql, json schema, proto)."),
          ("mutations", "MUTATION_LEAVES", "🧬️mutations", MOD + "::mutations", "🧬️ The mutation facet's schema leaves (rust, typescript, graphql, json schema, proto).")]


def block(text, start):
    i = text.index("{", start)
    depth, j = 0, i
    while True:
        depth += text[j] == "{"
        depth -= text[j] == "}"
        if depth == 0:
            return i, j + 1
        j += 1


schema_path = os.path.join(S, "🦀️.rs")
schema = open(schema_path, encoding="utf-8").read()
m = re.search(r"(?m)^(?:///[^\n]*\n)*pub fn (\w+_artifact_schema_descriptor)\(\) -> semio_framework_schema_registry::ArtifactSchemaDescriptor \{", schema)
name = m.group(1)
_, end = block(schema, m.end() - 1)
fn_text = schema[m.start():end]
artifact_id = re.search(r'id:\s*"([^"]+)"', fn_text).group(1)
writes = {}
for facet, const, sub, module, doc in FACETS:
    fm = re.search(r"\b" + facet + r":\s*semio_framework_schema_registry::FacetLeaves\s*\{", fn_text)
    a, b = block(fn_text, fm.end() - 1)
    body = fn_text[a:b]
    target = os.path.join(S, sub, "🦀️.rs") if sub else schema_path
    body = re.sub(r'include_str!\("([^"]+)"\)', lambda x: 'include_str!("' + os.path.relpath(os.path.normpath(os.path.join(S, x.group(1))), os.path.dirname(target)) + '")', body)
    body = re.sub(r"\s+", " ", body).replace("{ ", "{ ").strip()
    item = f"\n/// {doc}\npub const {const}: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {body};\n"
    writes.setdefault(target, []).append(item)
root_path = os.path.join(A, "🦀️.rs")
assembled = (f"\n/// 🧬️ Descriptor for `{artifact_id}`, assembled from the facet-owned schema leaves.\n"
             f"pub fn {name}() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {{\n"
             f"    semio_framework_schema_registry::ArtifactSchemaDescriptor {{\n        id: \"{artifact_id}\",\n"
             + "".join(f"        {facet}: {module}::{const},\n" for facet, const, _, module, _ in FACETS) + "    }\n}\n")
print(f"{name}: id {artifact_id}; consts into {[os.path.relpath(p, A) for p in writes]}")
if mode == "apply":
    schema = schema.replace(fn_text, "").rstrip("\n") + "\n"
    open(schema_path, "w", encoding="utf-8").write(schema)
    for path, items in writes.items():
        text = open(path, encoding="utf-8").read()
        open(path, "w", encoding="utf-8").write(text.rstrip("\n") + "\n" + "".join(items))
    root = open(root_path, encoding="utf-8").read()
    open(root_path, "w", encoding="utf-8").write(root.rstrip("\n") + "\n" + assembled)
    json.dump([{"from": MOD, "to": "crate", "items": [name]}], open(record, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
