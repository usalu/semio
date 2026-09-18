"""🧬️ Writes the Rust side of the 📖️pdf 1.7/🧱️base mutation vocabulary from `🔣️mutation-leaves.json`:
each NEW leaf's `🦀️.rs` (payload, diff, inverse, label, target) and unit test, and the aggregate
`🦀️.rs` / `💾️binary/🦀️.rs` / `📝️text/🦀️.rs` registries in declaration order. Leaves marked
`keep` are left untouched (their hand-written Rust stays authoritative).

Usage: python3 🐍️generate-mutation-leaves.py <mutations-dir>
Ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE.
"""
import json, os, re, sys

MUTATIONS_DIR = sys.argv[1]
LEAVES = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🔣️mutation-leaves.json"), encoding="utf-8"))

def snake(pascal):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", pascal).lower()

def camel_variant(pascal):
    return pascal[0].lower() + pascal[1:]

def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)

def expand(leaf):
    """Fills the derived field/diff/inverse spec of `keyed`, `keyed_remove` and `tri` leaves."""
    if "keyed" in leaf:
        field, ty, lane, entity, key = leaf["keyed"]
        setter = snake(leaf["pascal"])
        remove_pascal = "Remove" + leaf["pascal"][3:]
        leaf.update({"fields": [[field, ty]], "verb": "set", "entity": entity, "record": "Set", "diff": f"diff::diff_{setter}(base, self.{field}.clone())",
            "inverse": f"match base.{lane}.iter().find(|item| item.{key} == self.{field}.{key}) {{ Some(previous) => vec![PdfMutation::{leaf['pascal']}({leaf['pascal']} {{ {field}: previous.clone() }})], None => vec![PdfMutation::{remove_pascal}(super::{snake(remove_pascal)}::{remove_pascal} {{ {key}: self.{field}.{key}.clone() }})] }}",
            "label": f'format!("Set {entity} {{}}", self.{field}.{key})', "target": f"vec![self.{field}.{key}.clone()]"})
    if "keyed_remove" in leaf:
        field, ty, lane, set_pascal, set_snake, key = leaf["keyed_remove"]
        remover = snake(leaf["pascal"])
        leaf.update({"fields": [[key, "String"]], "verb": "remove", "entity": leaf["kebab"][7:], "record": "Remove", "diff": f"diff::diff_{remover}(base, &self.{key})",
            "inverse": f"base.{lane}.iter().find(|item| item.{key} == self.{key}).map(|item| PdfMutation::{set_pascal}(super::{set_snake}::{set_pascal} {{ {field}: item.clone() }})).into_iter().collect()",
            "label": f'format!("Remove {leaf["kebab"][7:]} {{}}", self.{key})', "target": f"vec![self.{key}.clone()]"})
    if "tri" in leaf:
        field, ty, lane, entity = leaf["tri"]
        setter = snake(leaf["pascal"])
        leaf.update({"fields": [[field, f"Option<{ty}>"]], "verb": "set", "entity": entity, "record": "Set", "diff": f"diff::diff_{setter}(base, self.{field}.clone())",
            "inverse": f"vec![PdfMutation::{leaf['pascal']}({leaf['pascal']} {{ {field}: base.{lane}.clone() }})]",
            "label": f'"Set {entity}".to_string()', "target": "Vec::new()"})
    return leaf

for leaf in LEAVES:
    expand(leaf)

for leaf in LEAVES:
    if leaf.get("keep"):
        continue
    pascal, kebab, emoji = leaf["pascal"], leaf["kebab"], leaf["emoji"]
    directory = os.path.join(MUTATIONS_DIR, leaf["directory"])
    uses = "\n".join(f"use {u};" for u in leaf.get("uses", []))
    fields = "\n".join(f"    pub {name}: {ty}," for name, ty in leaf["fields"])
    rust = f'''//! {emoji} Authoritative PDF mutation payload, diff, inverse, and tests for `{kebab}`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{{
    diff::{{self, PdfDiff}},
    snapshot::*,
}};
{uses}
use protocol::{{MutationKind, MutationOutcome, SemanticDescriptor}};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct {pascal} {{
{fields}
}}

impl MutationKind<PdfSnapshot, PdfMutation> for {pascal} {{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor {{ verb: "{leaf['verb']}", entity: "{leaf['entity']}", kind: "{kebab}", record: "{leaf['record']}" }};

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {{
        let _ = base;
        MutationOutcome::new({leaf['diff']})
    }}

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {{
        let _ = base;
        {leaf['inverse']}
    }}

    fn label(&self) -> String {{
        {leaf['label']}
    }}

    fn target(&self) -> Vec<String> {{
        {leaf['target']}
    }}
}}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
'''
    rust = rust.replace("\n\nuse protocol", "\nuse protocol") if not uses else rust
    write(os.path.join(directory, "🦀️.rs"), rust)
    if not os.path.exists(os.path.join(directory, "🧪️tests", "🔬️unit", "🦀️.rs")):
        write(os.path.join(directory, "🧪️tests", "🔬️unit", "🦀️.rs"), f'''use super::*;

#[test]
fn semantic_identity_is_owned_by_this_leaf() {{
    assert_eq!(<{pascal} as MutationKind<PdfSnapshot, PdfMutation>>::SEMANTICS.kind, "{kebab}");
}}
''')

# aggregate
mods = "\n".join(f'#[path = "{leaf["directory"]}/🦀️.rs"]\npub mod {snake(leaf["pascal"])};' for leaf in LEAVES)
uses = "\n".join(f"pub use {snake(leaf['pascal'])}::{leaf['pascal']};" for leaf in LEAVES)
variants = "\n".join(f"    {leaf['pascal']}({leaf['pascal']})," for leaf in LEAVES)
aggregate = f'''//! 🧬️ Transparent PDF 1.7/Any mutation dispatch. Every concrete payload, diff, inverse, codec,
//! schema, and test is owned by its direct semantic folder. Ticket 26/09/18/PDF-ARTIFACT-SPEC-
//! COMPLETE widened the vocabulary from the page/COS edits to the whole typed model: page boxes
//! and user units, content operators and annotations by index, every document collection by id
//! (fonts, images, forms, graphics states, shadings, patterns, colour spaces, property lists,
//! embedded files), outlines, named destinations, page labels, output intents, the interactive
//! form, optional content, viewer settings, metadata, identity and encryption, catalog extras —
//! with the retained COS lanes' object/dict/trailer edits kept as they were.

use crate::standards::v1_7::subsets::base::schema::{{diff::PdfDiff, snapshot::PdfSnapshot}};

//#region 🔖️Leaves
{mods}

{uses}
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.7")]
pub enum PdfMutation {{
{variants}
}}
//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies one mutation through its leaf-owned diff.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {{
    use protocol::Mutation;
    let outcome = mutation.diff(snapshot);
    outcome.apply_to(snapshot)
}}

/// ↩️ Delegates inverse planning to the authoritative leaf.
pub fn inverse_pdf_mutation(mutation: &PdfMutation, base: &PdfSnapshot) -> Vec<PdfMutation> {{
    use protocol::Mutation;
    mutation.inverse(base)
}}

/// 🧾️ Returns the derive-owned identity table in declaration and binary-tag order.
pub fn pdf_mutation_kinds() -> &'static [protocol::SemanticDescriptor] {{
    use protocol::SemanticMutation;
    PdfMutation::kinds()
}}
//#endregion 🔖️Delegation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
'''
write(os.path.join(MUTATIONS_DIR, "🦀️.rs"), aggregate)

binary_registry = "\n".join(f'    ("{leaf["pascal"]}", "{camel_variant(leaf["pascal"])}", super::{snake(leaf["pascal"])}::binary::BINARY_TAG),' for leaf in LEAVES)
text_registry = "\n".join(f'    ("{leaf["pascal"]}", super::{snake(leaf["pascal"])}::text::TEXT_OPCODE),' for leaf in LEAVES)
binary_src = open(os.path.join(MUTATIONS_DIR, "💾️binary", "🦀️.rs"), encoding="utf-8").read()
binary_src = re.sub(r"pub const BINARY_TAG_REGISTRY: &\[\(&str, &str, u8\)\] = &\[\n.*?\n\];", "pub const BINARY_TAG_REGISTRY: &[(&str, &str, u8)] = &[\n" + binary_registry + "\n];", binary_src, flags=re.S)
binary_src = binary_src.replace("const MAX_PAYLOAD_BYTES: usize = 256 * 1024;", "const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;")
write(os.path.join(MUTATIONS_DIR, "💾️binary", "🦀️.rs"), binary_src)
text_src = open(os.path.join(MUTATIONS_DIR, "📝️text", "🦀️.rs"), encoding="utf-8").read()
text_src = re.sub(r"pub const TEXT_OPCODE_REGISTRY: &\[\(&str, &str\)\] = &\[\n.*?\n\];", "pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] = &[\n" + text_registry + "\n];", text_src, flags=re.S)
text_src = text_src.replace("const MAX_PAYLOAD_BYTES: usize = 256 * 1024;", "const MAX_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;")
write(os.path.join(MUTATIONS_DIR, "📝️text", "🦀️.rs"), text_src)
kinds = json.dumps([leaf["kebab"] for leaf in LEAVES])
print(f"wrote {sum(1 for l in LEAVES if not l.get('keep'))} leaves; KINDS = {kinds}")
