#!/usr/bin/env python3
"""🧾️ Wave 0: each schema facet owns its five format leaves as a `FacetLeaves` constant (artifact and snapshot in the
model, diff, mutations); the subset root assembles `remodeling_artifact_schema_descriptor` from them. Also drops the
schema module's re-export region (lane P2)."""
import os, re
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
A = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = os.path.join(A, "🧬️schema")
def leaves(name, doc):
    return (f"/// {doc}\n"
            f"pub const {name}: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves {{ rust: include_str!(\"🦀️.rs\"), typescript: include_str!(\"🟦️.ts\"), graphql: include_str!(\"🔗️.graphql\"), json_schema: include_str!(\"🔣️.json\"), proto: include_str!(\"🛰️.proto\") }};\n")
schema_path = os.path.join(S, "🦀️.rs")
schema = open(schema_path, encoding="utf-8").read()
start = schema.index("//#region 🔖️Descriptor\n")
end = schema.index("//#endregion 🔖️Descriptor\n") + len("//#endregion 🔖️Descriptor\n")
schema = schema[:start] + "//#region 🔖️Leaves\n" + leaves("ARTIFACT_LEAVES", "🧾️ The artifact facet's five format leaves.") + "//#endregion 🔖️Leaves\n" + schema[end:]
start = schema.index("//#region 🔁️Re-exports\n")
end = schema.index("//#endregion 🔁️Re-exports\n") + len("//#endregion 🔁️Re-exports\n")
schema = schema[:start].rstrip("\n") + "\n" + schema[end:]
open(schema_path, "w", encoding="utf-8").write(schema)
for rel, name, doc in [("📸️snapshot/🦀️.rs", "SNAPSHOT_LEAVES", "🧾️ The snapshot facet's five format leaves."), ("🔺️diff/🦀️.rs", "DIFF_LEAVES", "🧾️ The diff facet's five format leaves."), ("🧬️mutations/🦀️.rs", "MUTATION_LEAVES", "🧾️ The mutation facet's five format leaves.")]:
    path = os.path.join(S, rel)
    text = open(path, encoding="utf-8").read().rstrip("\n")
    text += "\n\n//#region 🔖️Leaves\n" + leaves(name, doc) + "//#endregion 🔖️Leaves\n"
    open(path, "w", encoding="utf-8").write(text)
subset_path = os.path.join(A, "🦀️.rs")
subset = open(subset_path, encoding="utf-8").read()
old = "schema: SchemaDeclaration { descriptor: schema::remodeling_artifact_schema_descriptor(), "
assert old in subset
subset = subset.replace(old, "schema: SchemaDeclaration { descriptor: remodeling_artifact_schema_descriptor(), ")
subset = subset.replace("/// 🌳️ `standard \"1\" / subset \"any\"`'s complete declaration",
    "/// 🧬️ Descriptor for `s.remodel.remodeling`, assembled from the four facets' own format leaves.\n"
    "pub fn remodeling_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {\n"
    "    semio_framework_schema_registry::ArtifactSchemaDescriptor { id: \"s.remodel.remodeling\", artifact: schema::ARTIFACT_LEAVES, snapshot: schema::snapshot::SNAPSHOT_LEAVES, diff: schema::diff::DIFF_LEAVES, mutations: schema::mutations::MUTATION_LEAVES }\n"
    "}\n\n"
    "/// 🌳️ `standard \"1\" / subset \"any\"`'s complete declaration", 1)
open(subset_path, "w", encoding="utf-8").write(subset)
print("ok")
