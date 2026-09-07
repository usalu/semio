# 🧩️ One `s.energy.model` mutation kind — the copy-verbatim template

Everything below is the real, landed `🏷️rename-model` with `<PLACEHOLDER>`s. Replace the
placeholders, drop the files in, add the aggregate lines from §3–§6, run the fixture generator (§7),
and the kind is done. Take the number and the emoji from `📓️mutation-tag-ledger.md` FIRST — that file
is the collision authority, this one is only the shape.

> **The fast path is not to copy this by hand.** `🐍️generate-mutation-leaves.py` in this folder emits
> every file below (and every aggregate line in §3–§6, the crate mounts, the oracle catalog, the
> `🥒️.feature` tables and the Python oracle) from one `kind(...)` spec entry. Add your group's
> entries to its spec table and re-run it from the repository root. This document is the contract the
> generator implements, for reading and for reviewing.

## 0. Placeholders

| Placeholder | `🏷️rename-model`'s value | Where it comes from |
|---|---|---|
| `<EMOJI>` | `🏷️` | ledger §3/§5 — must carry U+FE0F and be unique among every sibling in `🧬️mutations/` |
| `<SLUG>` | `rename-model` | kebab `verb-noun`, ≥2 hyphen-joined words, from the vocabulary catalog |
| `<VARIANT>` | `RenameModel` | UpperCamel of `<SLUG>`; `dsl::Mutations` fails to compile if `kebab(<VARIANT>) != <SLUG>` |
| `<MODULE>` | `rename_model` | snake of `<SLUG>` |
| `<VERB>` | `rename` | `📓️taxonomy.md`'s approved verb table |
| `<ENTITY>` | `model` | the noun the verb addresses |
| `<RECORD>` | `RenamedModel` | past tense of `<VERB>` + noun; irregulars fixed in `📓️taxonomy.md` |
| `<DISPLAY>` | `Rename Model` | title case |
| `<NUMBER>` | `1` | ledger §5 |
| `<CASE>` | `✅️renames-the-model` / `⛔️refuses-a-blank-name` | one that moves the document, one that is refused |

Directory: `🧬️schema/🧬️mutations/<EMOJI><SLUG>/`. Five files plus one `🧪️tests/<CASE>/` per vector —
this is `📸️remodel`'s leaf shape: **no per-kind `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`, `📝️text/` or
`💾️binary/`.** Those live once, at the aggregate root, because the aggregate derives `dsl::DslEnum`.

## 1. `<EMOJI><SLUG>/🦀️.rs`

```rust
//! <EMOJI> Energy model mutation — `<VARIANT>`: <one sentence, what it sets>.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// <EMOJI> `<SLUG>` payload. <same sentence>.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "<SLUG>")]
pub struct <VARIANT> {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn <MODULE>(new_name: String) -> EnergyModelMutation {
    EnergyModelMutation::<VARIANT>(<VARIANT> { new_name })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for <VARIANT> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "<VERB>", entity: "<ENTITY>", kind: "<SLUG>", record: "<RECORD>" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Rename energy model to \"{}\"", self.new_name)
    }
}
//#endregion 🔖️Mutation
```

Add `fn target(&self) -> Vec<String> { vec![self.id.0.to_string()] }` whenever the kind addresses a
specific entity — the refusal fixtures compare it against the committed `🎯️outcome`'s `path`.

**Payload field types must implement `dsl::DslField`.** `String`, `bool`, every integer, `f64`,
`Vec<T>`, `[T; N]` and `Option<T>` do; a tuple does not, and neither does `store::ArtifactLink`
(carry a link as its flattened `target_uri: String` and rebuild it with
`store::os_io::ArtifactRef::parse_uri`). `crate::model::EntityId` does — the impl is hand-written next
to its `ToValue` in `🔋️model/🦀️.rs`. A nested model struct needs `#[derive(dsl::DslRecord)]` on
itself; a unit-only model enum needs `#[derive(dsl::DslScalar)]`.

## 2. `<EMOJI><SLUG>/🔺️diff/🦀️.rs` and `<EMOJI><SLUG>/↩️inverse/🦀️.rs`

```rust
//! 🔺️ Sparse diff builder for `<VARIANT>` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::<VARIANT>, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An energy model name must not be blank.", [payload.new_name.clone()]);
    }
    if base.model.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The energy model is already named \"{}\".", payload.new_name));
    }
    let mut model = base.model.clone();
    model.name = payload.new_name.clone();
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
```

```rust
//! ↩️ Inverse for `<VARIANT>` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::<VARIANT>, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.new_name.trim().is_empty() || base.model.name == payload.new_name {
        return Vec::new();
    }
    vec![vocabulary::<MODULE>(base.model.name.clone())]
}
//#endregion 🔖️Inverse
```

Rules that the law tests actually enforce:

- Every field of `Model` this kind touches is written through `diff_from_model(model)`, which
  regenerates the composed `structure`/`zones` handles TOGETHER. A kind that touches a snapshot-level
  link slot instead builds `EnergyModelDiff { weather_link: Some(EnergyLinkSlotDelta::Attached { link }), ..Default::default() }`
  (or `Some(EnergyLinkSlotDelta::Detached)`), never both.
- `inverse` returns `Vec::new()` for exactly the inputs `diff` refuses or treats as a no-op —
  otherwise `assert_inverse` fails, because a refused forward step left the document where it was and
  an undo would move it.
- Cross-kind builders go through `vocabulary::`, never `super::` — `super::` inside a leaf is that
  leaf's own module.

## 3. `<EMOJI><SLUG>/🔣️.json` and `<EMOJI><SLUG>/🧬️.schema.json`

```json
{
  "schemaVersion": 1,
  "owner": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<EMOJI><SLUG>",
  "semanticKind": "<SLUG>",
  "displayName": "<DISPLAY>",
  "emoji": "<EMOJI>",
  "aggregateVariant": "<VARIANT>",
  "payloadSchema": "🧬️.schema.json",
  "textOpcode": "<SLUG>",
  "binaryTag": null,
  "invertibility": "explicit-mutation",
  "diffParticipation": "detect",
  "outcomeClasses": ["applied", "warning", "error"],
  "composition": "atomic",
  "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]
}
```

`binaryTag` stays `null`: with `dsl::DslEnum` the tag IS the variant's ordinal, emitted by the derive.
`outcomeClasses` lists only the classes the leaf can actually produce.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "<VARIANT>",
  "type": "object",
  "additionalProperties": false,
  "required": ["newName"],
  "properties": { "newName": { "type": "string" } }
}
```

`title` MUST equal `aggregateVariant`; the structural-correspondence test asserts it.

## 4. Aggregate one-liners (`🧬️mutations/🦀️.rs`)

```rust
pub use super::<MODULE>::{<MODULE>, <VARIANT>};        // 🔖️Reexports, alphabetical
    <VARIANT>(<VARIANT>),                              // enum body, ordered by <NUMBER>
    "<SLUG>",                                          // KINDS, same order as the enum
    ("<SLUG>", "<EMOJI><SLUG>"),                       // DIRECTORIES, same order
        <MODULE>("Probe".to_string()),                 // wire_probes(), same order
```

## 5. Aggregate mirrors

```jsonc
// 🧬️mutations/🔣️.json — one oneOf entry and one $defs entry
{ "$ref": "#/$defs/<VARIANT>" }
"<VARIANT>": { "title": "<VARIANT>", "type": "object", "additionalProperties": false,
  "required": ["mutation", "newName"],
  "properties": { "mutation": { "const": "<SLUG>" }, "newName": { "type": "string" } } }
```

```ts
/* 🧬️mutations/🟦️.ts */
/** <EMOJI> `<SLUG>` payload. */
export interface <VARIANT> {
  readonly mutation: "<WIRE_TAG>";   // lowerCamel of <VARIANT>, e.g. "renameModel"
  readonly newName: string;
}
// …and one `| <VARIANT>` in `export type EnergyModelMutation`
```

```graphql
# 🧬️mutations/🔗️.graphql
type <VARIANT> {
  newName: String!
}
# …and one `| <VARIANT>` in `union EnergyModelMutation`
```

```proto
// 🧬️mutations/🛰️.proto
message <VARIANT> {
  string new_name = 1;
}
// …and inside `message EnergyModelMutation { oneof kind { … } }`:
    <VARIANT> <MODULE> = <NUMBER>;
```

```
# 🧬️mutations/📖️.grammar.semio — one alternative and one production
op = … | <SLUG>
<SLUG> = "<SLUG>" "new-name" "=" TEXT
```

## 6. Crate mount block (`✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/🦀️.rs`)

Inside `pub mod mutations { … }`, at 32 spaces of indentation, ordered by `<NUMBER>`:

```rust
                                #[path = "."]
                                pub mod <MODULE> {
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<EMOJI><SLUG>/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<EMOJI><SLUG>/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<EMOJI><SLUG>/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<EMOJI><SLUG>/🧪️tests/<CASE>/🦀️.rs"]
                                    mod tests_<case_snake>;
                                }
```

A fixture case that is not mounted here is dead code, invisible to `cargo test`, however complete it
looks on disk.

## 7. Fixture case `<EMOJI><SLUG>/🧪️tests/<CASE>/`

Six files: `🦀️.rs`, `🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`,
`📸️snapshot/➡️after/🔣️.json`, `🔺️diff/🔣️.json`, `🎯️outcome/🔣️.json`. You hand-write only the
`🦀️.rs`; the five JSON files are GENERATED from its `scenario()`:

1. seed the five JSON files with `{}` so `include_str!` compiles;
2. `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy <case-name>` — the
   `writes_the_committed_vector_when_requested` test materializes all five (every other test in the
   case fails on this pass, which is expected);
3. re-run `cargo test` without the variable; all ten laws must pass against the committed bytes.

```rust
//! 🧪️ `<SLUG>` fixture — `<CASE>`: <one sentence>.
//!
//! The committed `(before, mutation, after, diff, outcome)` quintet beside this file IS the
//! specification; `scenario` is the typed source it was generated from
//! (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test -p semio-s-plugin-energy`), and the eight law
//! assertions below read the committed bytes back, never the scenario.

use crate::artifacts::model::mutations::fixtures::{self, link, snapshot, zone, Case};
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

#[allow(unused_variables, unused_mut)]
fn scenario() -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mut model = crate::model::Model { name: "BESTEST 600".into(), version: "1".into(), ..crate::model::Model::default() };
    model.zones.push(zone(1, "ZONE ONE"));
    (snapshot(model), super::<MODULE>("BESTEST 600FF".into()))
}

fn case() -> Case {
    Case { kind: "<SLUG>", directory: "<EMOJI><SLUG>/🧪️tests/<CASE>", before: BEFORE, after: AFTER, mutation: MUTATION, diff: DIFF, outcome: OUTCOME, scenario }
}

#[semio_framework_async_macros::async_test]
async fn writes_the_committed_vector_when_requested() { fixtures::write_when_requested(&case()); }
#[semio_framework_async_macros::async_test]
async fn forward_reaches_the_committed_after_snapshot() { fixtures::assert_forward(&case()); }
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_committed_before_snapshot() { fixtures::assert_inverse(&case()); }
#[semio_framework_async_macros::async_test]
async fn committed_documents_are_canonical() { fixtures::assert_canonical(&case()); }
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() { fixtures::assert_outcome(&case()); }
#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() { fixtures::assert_diff(&case()); }
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() { fixtures::assert_diff_canonical(&case()); }
#[semio_framework_async_macros::async_test]
async fn committed_diff_alone_carries_before_to_after() { fixtures::assert_diff_applies(&case()); }
#[semio_framework_async_macros::async_test]
async fn semantic_descriptor_and_inverse_are_complete() { fixtures::assert_semantics(&case()).await; }
#[semio_framework_async_macros::async_test]
async fn inverse_and_absorb_laws_hold() { fixtures::assert_laws(&case()).await; }
```

The ten law bodies live once, in `🧬️mutations/🦀️.rs`'s `#[cfg(test)] pub mod fixtures` — a case file
declares, it never asserts. `zone`, `snapshot` and `link` are that module's fixture constructors.

## 8. Oracle registration (`✳️any/🔮️oracle/🔣️.json`)

Third registration point after the leaf descriptor and the crate mount — the one that is easy to
forget, and the one the aggregate's structural-correspondence test checks against `KINDS`.

```jsonc
// mutationCatalogs[0].vectors[]
{
  "mutationId": "<SLUG>",
  "sourceMutationDirectoryName": "<EMOJI><SLUG>",
  "mutationDirectoryName": "<EMOJI><SLUG>",
  "scenarios": [
    { "id": "<SLUG>-renames-the-model", "directoryName": "<CASE>" },
    { "id": "<SLUG>-refuses-a-blank-name", "directoryName": "<CASE>" }
  ]
}
// mutationCatalogs[0].kinds[]
"<SLUG>"
// mutationManifests[0].mutations[]
{
  "id": "<SLUG>",
  "capability": "energy-model-1-mutate",
  "payloadSchema": "🧬️.schema.json",
  "outcomes": ["applied", "rejected"],
  "productionDispatch": { "operation": "<SLUG>", "bridgeVersion": 1, "variant": "<VARIANT>" },
  "oracleRequirements": [{ "capability": "energy-model-1-mutate", "qualifyingKind": "verified-native-second-implementation" }]
}
```

## 9. Second implementation (`✳️any/🧪️tests/🏛️mutate-energy-model-1/`)

A kind that is not in the Python oracle is not oracle-covered, and
`nativeSecondImplementationBreaches` will not catch it — the capability is declared once for the whole
vocabulary, so a kind with no vector passes the coarse gate "by construction". Three edits:

```python
# VECTOR_ROOTS — one entry per fixture case
"<SLUG>-renames-the-model": "asset://🧬️schema/🧬️mutations/<EMOJI><SLUG>/🧪️tests/<CASE>",

# one function, written from the payload schema and 📓️taxonomy.md — never from the Rust
def <MODULE>(before, payload):
    """<EMOJI> `<SLUG>{newName}` — taxonomy.md's `<VERB>` verb on …"""
    ...
    return after, applied()          # or  unchanged(before), rejected("mutation.invariant", [...])

# VOCABULARY — one row
"<SLUG>": <MODULE>,
# invert() — one arm returning the undo steps read off BASE
```

and two `Examples` rows per fixture case in `🥒️.feature` (`| <SLUG>-<case> | <EMOJI><SLUG> | <CASE> |`),
plus one `Vector { … }` per case in the Rust adapter `🦀️.rs`.

## 10. Gates

```
bun nx run verify-taxonomy-report
bun nx run verify-taxonomy-enforce
bun nx run mutation-outcome-law
RUSTC_WRAPPER= CARGO_TARGET_DIR=…/target-energy-e2e cargo check -p semio-s-plugin-energy --lib --tests --message-format=short
RUSTC_WRAPPER= CARGO_TARGET_DIR=…/target-energy-e2e cargo test  -p semio-s-plugin-energy <slug>
```

## 11. What `🐍️generate-mutation-leaves.py` refuses (W-D0, custodian)

`audit()` runs before a single file is written, so a bad row fails the whole run rather than half-
landing. Every one of these was probed by injecting the bad row and confirming the message:

| Refusal | Message |
|---|---|
| two kinds on one ledger number | `ledger number 1 is claimed by both rename-model and other-kind` |
| two kinds on one emoji, or on one slug | same shape, `emoji`/`slug` |
| emoji missing (or doubling) U+FE0F | `'🌟' needs exactly one trailing U+FE0F` |
| emoji already used by a non-kind sibling (`💾️📖️📝️🔗️🔣️🛰️🟦️🦀️`) | `🦀️ is a non-kind sibling inside 🧬️mutations/` |
| directory name `taxonomy.json`'s own `mutationDirectoryPattern` rejects (single-word slug, missing FE0F, non-NFC) | `directory '🌟️otherkind' does not match taxonomy.json's mutationDirectoryPattern` — the pattern is READ from `taxonomy.json`, never copied |
| ledger number outside every §2 range | `ledger number 1500 is outside every range in the ledger's §2` |
| `kebab(<VARIANT>) != <SLUG>` (which `dsl::Mutations` would only report as a compile error much later) | `dsl::Mutations requires kebab(RenameModel) == rename-model` |
| a kind without both a `✅️` happy and a `⛔️` refusal fixture | `needs at least one ✅️ happy and one ⛔️ refusal fixture, got ['✅️']` |
| two fixture cases sharing one directory name | `two fixture cases share one directory name` |
| a case path over the 227-UTF-16-unit budget (see the ledger §3 — the number is measured, not the stale 190) | `300 UTF-16 units exceeds the 227-unit path budget` |

Two more custodian properties, both measured by running the generator twice and checksumming:

- **Idempotent.** A second run changes nothing, in `🧬️mutations/` (309 files) or in any of the five
  files it rewrites outside it. Committed fixture JSON is only ever SEEDED (`{}` when absent), never
  overwritten, so re-running after `SEMIO_ENERGY_WRITE_FIXTURES=1` cannot wipe a materialized vector.
- **Stale-case pruning.** A `🧪️tests/<case>/` directory no longer declared by its kind's spec row is
  deleted, so renaming a case leaves no orphan. Kind directories with no spec row are **reported and
  left alone** (`NOTE: …/<dir> has no spec row … left untouched`) — a directory a group has not yet
  added a row for is in-flight work, and guessing between "rename" and "in-flight" by deleting is how
  one worker destroys another's files.
