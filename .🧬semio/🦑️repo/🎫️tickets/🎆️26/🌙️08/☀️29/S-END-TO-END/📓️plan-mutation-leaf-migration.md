# 🧬️ Mutation-Leaf Migration Recipe (stdio artifacts)

Derived by reading the ONE fully-migrated artifact in the repo,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧬️schema/🧬️mutations/`,
and the derive that powers it,
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`.

## Why this migration is forced

`protocol::Mutation<P>` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:105`) now requires

```rust
const DESCRIPTORS: &'static [MutationLeafDescriptor];
fn descriptor(&self) -> &'static MutationLeafDescriptor;
```

42 hand-rolled stdio `impl Mutation` blocks predate that and fail with **E0046**. Nothing in the repo
hand-writes `DESCRIPTORS` for an artifact; the sanctioned way to obtain both items is
`#[derive(dsl::Mutations)]`, which synthesizes them from per-variant **mutation leaves**.

## Target shape (verbatim from tiff/baseline)

### 1. One leaf folder per variant, beside the aggregate

```
🧬️mutations/
  🦀️.rs                      ← the aggregate
  🔧set-snapshot/🦀️.rs        ← leaf payload + MutationKind impl
  🔧set-snapshot/🔣️.json      ← the leaf descriptor (REQUIRED, read by the derive)
  🔩set-compression/…
```

Folder name = `<emoji><kebab-semantic-kind>` — **no** variation selector after the emoji.
A leaf needs exactly two files: `🦀️.rs` and `🔣️.json`. `🔣️payload.schema.json` is *named* by the
descriptor but is not required to exist (tiff's `🔩set-compression` has none).

### 2. Leaf `🦀️.rs`

```rust
//! 🔩️ `set-compression` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCompression {
    compression: u16,
}

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for SetCompression {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "set", entity: "compression", kind: "set-compression", record: "SetCompression",
    };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as protocol::Mutation<TiffSnapshot>>::Diff> {
        agg_diff(&TiffBaselineMutation::SetCompression(self.clone()), base)
    }
    fn inverse(&self, base: &TiffSnapshot) -> Vec<TiffBaselineMutation> {
        agg_inverse(&TiffBaselineMutation::SetCompression(self.clone()), base)
    }
    fn label(&self) -> String { "set-compression".to_string() }
    fn target(&self) -> Vec<String> { Vec::new() }
}
//#endregion 🔖️Payload
```

The payload struct's fields are the variant's named fields, **verbatim** (same names, same types).

### 3. Leaf `🔣️.json`

```json
{
  "schemaVersion": 1,
  "owner": "<repo-relative path of THIS leaf folder>",
  "semanticKind": "set-compression",
  "displayName": "Set Compression",
  "emoji": "🔩",
  "aggregateVariant": "SetCompression",
  "payloadSchema": "🔣️payload.schema.json",
  "textOpcode": null,
  "binaryTag": null,
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": ["applied"],
  "composition": "atomic",
  "requiredLanguageSurfaces": ["rust", "json-schema"]
}
```

`emoji` must equal the folder-name prefix. `owner` must end in the folder name and sit directly
under a `…/🧬️mutations/` segment.

### 4. Aggregate `🦀️.rs`

```rust
//#region 🔖️Leaves
#[path = "🔧set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔩set-compression/🦀️.rs"]
pub mod set_compression;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = TiffSnapshot, diff = TiffDiff, schema = "TiffBaselineMutation")]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TiffBaselineMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetCompression(set_compression::SetCompression),
}
```

The **old `impl Mutation<…> for …` block is deleted**; its `diff`/`inverse` bodies move verbatim
into free functions

```rust
pub(crate) fn agg_diff(this: &XMutation, base: &XSnapshot) -> protocol::MutationOutcome<XDiff> { … }
pub(crate) fn agg_inverse(this: &XMutation, base: &XSnapshot) -> Vec<XMutation> { … }
```

with each match arm's pattern head rewritten from `XMutation::Variant { a, b }` to
`XMutation::Variant(variant_mod::Variant { a, b })`. Nothing else in those bodies changes.

## Hard constraints discovered

1. **`NoMutation` must be dropped.** The derive requires every variant to wrap exactly one payload,
   and it asserts `is_approved_verb(SEMANTICS.verb)`. `no` is not in `APPROVED_VERBS`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:105`). tiff dropped it for
   this reason. Consequences to fix in the same artifact:
   - remove `#[derive(Default)]` + `#[default]` from the enum (a wrapped variant cannot be `#[default]`);
   - drop `"no-mutation"` from `KINDS` and update any `KINDS.len()` assertion;
   - remove the `NoMutation` arms from `kind()`, the test `variants()` list, and the artifact's
     `🧪️tests/mutate-*/🦀️.rs` spec mapper.
2. **`SEMANTICS.kind` must equal the derive's own `to_kebab(VariantIdent)`** — a const assertion, so
   a mismatch is a compile error, not a silent drift. Digit-adjacent names (`SetId3v2`) must be
   checked against the compiler, not guessed.
3. **`SEMANTICS.record` must equal the variant identifier**, and the leaf descriptor's
   `aggregateVariant` must equal it too.
4. **Every variant needs a distinct emoji** in its folder name and descriptor.

## Verification

The crate is the gate:

```
cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib --message-format=short
```

Native `cargo check` does **not** exercise the plugin target; only the wasm target proves it.
