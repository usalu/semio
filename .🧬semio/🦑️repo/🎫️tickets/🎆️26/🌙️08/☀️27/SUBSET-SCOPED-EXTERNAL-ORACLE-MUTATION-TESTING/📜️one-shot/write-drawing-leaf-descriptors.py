#!/usr/bin/env python3
# 🧵️ One-shot: writes the six missing 🔣️.json descriptors for ✳️drawing's single-word mutation leaves
# and adds `dsl::MutationLeaf` to each. These are the leaves the old `mutation_leaf_kebab` rule made
# unrepresentable by demanding a hyphen; with that clause removed they validate.
import io, json, os

M = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/🧬️mutations"

# dir, emoji, semanticKind, displayName, aggregateVariant (must match the enum at 🦀️.rs:51-57)
LEAVES = [
    ("🔄rotate", "🔄", "rotate", "Rotate", "Rotate"),
    ("📏scale", "📏", "scale", "Scale", "Scale"),
    ("🧷group", "🧷", "group", "Group Nodes", "Group"),
    ("💫ungroup", "💫", "ungroup", "Ungroup Node", "Ungroup"),
    ("🫓flatten", "🫓", "flatten", "Flatten Node", "Flatten"),
    ("🎈unflatten", "🎈", "unflatten", "Unflatten Node", "Unflatten"),
]

written = derived = 0
for folder, emoji, kind, display, variant in LEAVES:
    d = os.path.join(M, folder)
    descriptor = {
        "schemaVersion": 1,
        "owner": d,
        "semanticKind": kind,
        "displayName": display,
        "emoji": emoji,
        "aggregateVariant": variant,
        "payloadSchema": "🔣️payload.schema.json",
        "textOpcode": None,
        "binaryTag": None,
        "invertibility": "explicit-mutation",
        "diffParticipation": "detect",
        "outcomeClasses": ["applied"],
        "composition": "atomic",
        "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"],
    }
    p = os.path.join(d, "🔣️.json")
    io.open(p, "w", encoding="utf8").write(json.dumps(descriptor, ensure_ascii=False, indent=2) + "\n")
    written += 1

    leaf = os.path.join(d, "🦀️.rs")
    s = io.open(leaf, encoding="utf8").read()
    old = "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]"
    new = "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]"
    if old in s and "dsl::MutationLeaf" not in s:
        s = s.replace(old, new, 1)
        io.open(leaf, "w", encoding="utf8").write(s)
        derived += 1

print(f"wrote {written} descriptor(s), added the derive to {derived} leaf/leaves")
