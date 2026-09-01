# Store Sync Demo Direct-Leaf Caller Adoption — Plan 72

## Read-Only Result

Recommend one genuine Sync-owned `SetN { n: i32 }` leaf and a one-variant transparent `DemoMutation::SetN(SetN)` aggregate. This is a separate caller-adoption packet after the accepted84-diagnostic signature join, **not another signature repair or a compatibility descriptor**.

Only this new Markdown is authored. No Sync, Store, base trait, derive, taxonomy, canonical fixture/schema, launch, or native source is changed. No Cargo, native test, whole outer namespace, wire generator, global scan or cleanup ran. The frozen [Sync](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs) source remains267795bytes / `62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6`.

The actual base [Mutation contract](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:105) still has default DESCRIPTORS=[] and UNDECLARED_MUTATION_LEAF at119–125. The current Sync implementation at3850–3863 supplies neither. Removing those defaults will require this real caller adoption; adding an empty/default descriptor or leaving a manual semantic switch is not the proposal.

## Exact Current Behavior

| Surface | Actual source / invariant to preserve |
| --- | --- |
| Snapshot | Sync3732–3736: `DemoSnapshot { n: i32 }`, not optional; DSL id `demo.demo`, extension `demo` |
| Diff |3777–3792: `DemoDiff { n: Option<i32> }`; None preserves base; Some replaces; later Some wins absorb; later None preserves earlier diff |
| Payload |3794–3799: one `SetN { n: i32 }`; internally tagged JSON `{"operation":"SetN","n":…}` |
| Outcome |3853–3858: always Some(target), no messages, including equal-value replacement |
| Inverse |3860–3862: exactly one SetN carrying pre-state n; no negation, missing target, deletion or sentinel |
| Text |3801–3819: current generic DslVariants parser/printer; opcode `set-n` |
| Binary |3821–3848: existing generic record codec, format1, **one u8 ordinal0**, then unchanged record body |
| Artifact identity |3869–3891 and all current consumers use schema `demo/v1`; do not substitute the dotted DSL id for that key |
| Timestamp |No override or clock field; base Mutation142–144 returns None |
| Foreign steps |No override; base returns empty Vec, while its conservative may_emit_foreign_steps hint currently returns true |

Current serde has no rename_all or deny_unknown_fields on this aggregate. Preserve exact **SetN**, not `setN` or `set-n`, and preserve flat fields, not a new `payload` wrapper. Unknown fields currently accepted by serde are not grounds to add closure incidentally. Snapshot n remains required; Diff n remains nullable/omittable. No new default payload, integer sentinel or optional snapshot is needed.

## Closest Actual Conventions And Important Differences

The existing [Store SetN leaf](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs) uses exactly `verb:"set", entity:"n", kind:"set-n", record:"SetN"`, direct `🔢️set-n/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`, DslRecord + MutationLeaf, and a transparent derived aggregate. `set` is explicitly approved in [command138](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:138). Reuse that vocabulary and file convention, **not its operation type or semantics**.

That Store fixture has optional n, DeleteN/AddN/RestoreN, and typed missing-target errors. Its descriptor therefore includes error. None of those belong to Sync's total scalar replacement. Sync needs only one leaf, applied-only outcomes and same-kind pre-state inverse. The timestamped Store fixture likewise carries physical_ms; Sync does not. It is not a reusable payload for this task.

The actual [derive](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1624) requires a real canonical external aggregate source at `🧬️mutations/🦀️.rs`, direct newtype variants, approved semantic kinds and source-derived descriptor/provenance. [MutationLeaf578–609](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:578) reads the actual declaration's local_file, adjacent descriptor, project/taxonomy authority and workspace identity. Deriving on the old inline component declaration or a macro-template declaration cannot honestly provide that ownership.

[DslOps1468–1479](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1468) already delegates a one-field tuple variant to the leaf's own RecordSpec/value. [DslRecord1217–1249](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1217) uses the same record_codegen field planning used by the old named variant. Preserve n:i32 and `#[dsl(keyword = "set-n")]` on the leaf, plus the existing aggregate `#[dsl(key = "set-n")]` spelling. This is source evidence of the intended shared lowering, not a native round-trip result.

## Exact Proposed File Footprint

The following are **new proposed paths**, not claimed existing files. The existing Sync directory was observed to contain only its component. The path convention is taken from actual Store fixtures, and taxonomy membership/source admission must be checked by its owner before creating/mounting this separate packet; no membership is assumed.

```text
🏪️store/🔄️sync/
  🦀️component.rs
  🧪️fixtures/🧮️demo/
    🧬️schema/🔣️.json
    🧪️tests/
      🔣️.json
      🧬️schema/🔣️.json
    🧬️mutations/
      🦀️.rs
      🔣️.json
      🔢️set-n/
        🦀️.rs
        🔣️.json
        🧬️schema/🔣️.json
```

Eight new canonical files: shared intrinsic schema; neutral data/schema; aggregate Rust/schema; direct leaf Rust/descriptor/payload schema. Leaf-native laws can be in the leaf's own cfg(test) module, consuming `../../🧪️tests/🔣️.json` from its real source directory. No synthetic fixture bridge, extra operation, derive mirror, shared API or permanent script outside an approved ticket `📜️script.ts` is required.

### Routed Rust Declarations

Use a cfg(test) **sibling mount before** the current outer tests module, following the actual [Store mount19600](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:19600):

```rust
#[cfg(test)]
#[path = "🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs"]
mod demo_mutations;
```

Then import `super::demo_mutations::{DemoMutation, SetN}` inside outer tests. This does not depend on an invented physical `sync/tests` directory or an unresolved inline-module-relative path.

The proposed sibling aggregate's route to existing intrinsic types is `use super::tests::{DemoDiff, DemoSnapshot};`. Under that specific topology, **four test-only visibility edits are proposed, not authorized**: both struct declarations and both n fields become `pub(super)` at current3734/3735 and3778/3779. The two type qualifiers let the sibling aggregate name its snapshot/diff types; the Snapshot.n qualifier lets the sibling leaf read pre-state n, and the DemoDiff.n qualifier lets it construct Some(target). Private fields are not made visible merely by reexporting their type. `pub(super)` limits access to the owning Sync parent and its descendants, which include the sibling leaf; it does not expose these cfg(test) types as a production API. The inspected main Store fixture uses the same sibling-mount/visible-field route, although its types are more broadly pub(crate). Preserve shapes, attributes and implementation bodies. Root must approve this exact scoped access or select another proven private-owner mount; no Rust compilation has validated the proposed topology here.

The external aggregate contains only leaf module/import, the derived roster and any leaf-owned test join:

```rust
use super::tests::{DemoDiff, DemoSnapshot};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, crate::os_dsl::Mutations, crate::os_dsl::DslOps)]
#[serde(tag = "operation")]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "demo/v1")]
pub(super) enum DemoMutation {
    #[dsl(key = "set-n")]
    SetN(SetN),
}
```

Put cfg(test) on the mount, not on the aggregate/variant: Mutations1652/1692 rejects conditional metadata directly on those declarations. Use regions and emoji docstrings in new authored sources. No comments inside definitions are proposed.

The leaf imports `super::{DemoDiff, DemoMutation, DemoSnapshot}`; derives serde, DslRecord and MutationLeaf; uses `#[mutation_leaf(contract = ::protocol)]`; declares public n:i32 without deny_unknown_fields. Its MutationKind methods are the original Some(target) outcome and one SetN(pre-state) inverse, now owned by the leaf. Required semantic methods are `SEMANTICS = { set, n, set-n, SetN }`, label `"Set N"`, target `["n"]`, plus an explicit `fn may_emit_foreign_steps(&self) -> bool { true }` preserving the existing conservative capability. No warning for same value, error branch, delete/restore twin or compatibility alias.

Remove only the old concrete enum3794–3799 and manual Mutation impl3850–3863 after the new schema/native desired RED is retained. **Leave both existing generic OpText/OpBinary implementations3801–3848 byte-identical in place**, implementing the imported new aggregate. They dispatch DslVariants, not concrete semantic cases, and are not a second payload/DSL enum. Keeping them in place avoids new import routing and preserves all error offsets/messages and body handling. The current shared DSL variants_binary helper352–380 reads a varint ordinal; Sync reads one u8. Even though the single valid ordinal0 has identical bytes, substituting that helper would change the accepted framing/error domain. No such incidental modernization is proposed.

### Exact Consumer Join Roster

There are24 `DemoMutation::SetN` occurrences: two in the retiring manual behavior and **22 remaining constructors/assertion values**. Each of the22 changes only from `DemoMutation::SetN { n: value }` to `DemoMutation::SetN(SetN { n: value })` (same shorthand n where present). No helper factory or old constructor alias.

| Existing function/group | Current source lines |
| --- | --- |
| sample_operation_envelope |3880,3881 |
| wire_fixtures_stay_byte_identical_across_rust_and_ts |4027,4028 |
| sample_wire_envelope_for_fixtures |4106,4107 |
| op_envelope_from_stored_edit_round_trips_through_ingest |4211,4212,4219 |
| folder_external_edit_delivers_remote_operations |4294,4318,4319 |
| two_hosts_converge_through_hub |4495 |
| reconnect_since_catch_up_replays_backlog |4533 |
| detach_drains_pending_outbound_operations |4600 |
| command_outcome_accepted_fires_after_hub_ack |4634 |
| folder_event_log_storage_round_trips_undo_position_through_pack_spr |4813,4815 |
| folder_text_storage_round_trips_dsl_and_appends_ops |4850,4854 |
| folder_text_storage_round_trips_pack |4887,4891 |

The22 exact line/value joins were read from current source, including the4634 command-outcome test. No current fixture values/assertion meaning are changed. Actor-test constructor syntax joins do not authorize native_actor/Retained behavior or missing fixture repair.

## Descriptor And Provenance Proposal

Validate this exact fourteen-field descriptor against the **actual canonical** [JSON Schema](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json) (draft07,2406bytes,`db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee`), not a ticket replica:

```json
{
  "schemaVersion": 1,
  "owner": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n",
  "semanticKind": "set-n",
  "displayName": "Set N",
  "emoji": "🔢️",
  "aggregateVariant": "SetN",
  "payloadSchema": "🧬️schema/🔣️.json",
  "textOpcode": "set-n",
  "binaryTag": 0,
  "invertibility": "explicit-mutation",
  "diffParticipation": "apply-only",
  "outcomeClasses": [
    "applied"
  ],
  "composition": "atomic",
  "requiredLanguageSurfaces": [
    "rust",
    "json-schema",
    "text",
    "binary"
  ]
}
```

The descriptor names actual local semantic surfaces: no TypeScript/GraphQL/protobuf implementation is claimed from the unrelated protocol-wire fixture twin. Apply-only is honest; no new released leaf detection seam exists. Explicit-mutation describes constructing a pre-state SetN, even though it uses the same semantic kind. OutcomeClasses is applied only because current diff is total and message-free.

Provenance must be generated from the real leaf declaration and resolve to:

- mutation_root: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations`
- owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n`
- source_path: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs`
- descriptor_path: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json`
- taxonomy_path: the actual project-owned taxonomy locator.

Do not author a workspace_token or hardcode an absolute machine path into metadata. The derive reads project metadata, canonical filenames and taxonomy; the aggregate validates source/descriptor/provenance/variant equality and the complete one-row roster. The emitted registration ID would be `demo/v1#set-n` from the retained actual artifact schema key. Do not add a new global registry call to ensure_demo_codec_registered merely because the macro emits a registration function; the required compile-time/per-value descriptor does not depend on such a call.

### Timestamp And Capability Review

Explicitly assert `MutationKind::timestamp(SetN)=None` and `Mutation::timestamp(DemoMutation)=None` before and after serde/text/binary round trips and for the returned inverse. No clock capture/default, physical_ms field, HLT fabrication or mutation-id field is added. Current sample envelope HLT values and Edit.started_at are outside this leaf payload and remain unchanged. [Mutations1705/1777](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs:1705) forwards the exact leaf Option; ordinary MutationKind219–221 returns None, matching the existing base meaning.

**Preserve the existing true capability explicitly.** Base Mutation.may_emit_foreign_steps currently returns true; MutationKind's default false is therefore not a metadata-only join. The real existing MutationKind hook at command227–230 permits `fn may_emit_foreign_steps(&self) -> bool { true }` on this leaf. The actual aggregate derive forwards that method at1710 and1781–1783, so this requires no parallel API or shared edit. Assert both leaf and aggregate return true before/after codecs and for the inverse, while foreign_steps remains the original empty Vec. Empty current foreign steps alone do not prove equivalence with false, and this plan does not change the hint to false. Any future capability narrowing requires a separate desired law/behavior decision. Root explicitly required this preservation during review; an earlier draft's false-default recommendation was withdrawn before any source edit.

## Schema-First Neutral Proposal

The new shared domain schema defines exact i32 bounds, Snapshot (required n:i32), and Diff (optional n:i32|null). The leaf payload defines required n:i32 and reuses the shared intrinsic. The aggregate requires `operation:"SetN"` plus the same payload definition. Actual payload/aggregate/snapshot/diff schemas remain open to unknown properties where current serde is open. The **neutral test envelope** is closed so wrong/missing expectation fields cannot silently disappear.

Proposed new neutral additions, keeping packet67's authored33 semantic expectations unchanged:

| Case group | Desired evidence |
| --- | --- |
| Exact JSON surface | Flat SetN accepted; lowercase setN, kebab set-n, unknown operation and payload-only wrapper rejected |
| Current permissiveness | Extra field accepted and omitted from reserialization; missing/null n rejected; no incidental strictness |
| Intrinsic options | Diff {} and {n:null} both None; Snapshot missing n rejects; true/string/fraction/out-of-i32 rejects |
| Metadata | Actual14 fields; one descriptor exactly matching leaf; exact variant/opcode/tag; all required-field omissions fail canonical schema |
| Source identity | Real leaf/aggregate paths and common workspace token; cloned sibling descriptor, off-root owner and wrong source filename fail source authority |
| Semantics | Existing33 cases:0/1/2/5/6/9/42, negative, same value, MIN/MAX, None/Some absorption and reverse stored inverse |
| Clock | Null/None before and after all codecs and inverse; no authored timestamp field |
| Text/record | set-n opcode, same n:i32 RecordSpec/field order; all original values round-trip; unknown/missing/malformed text rejected |
| Binary | Existing format1/u8 ordinal0/body; complete old/new bytes equal; empty/truncated, wrong format, unknown ordinal rejected |
| Capability | Preserve explicit true at leaf/aggregate/inverse and the original empty foreign-step list; no true→false behavior change |

Do not fabricate complete text/body bytes from a guessed grammar. Before removing the old declaration, retain selected actual current codec observations and independently authored/committed wire goldens under the new packet. Compare the new leaf-derived RecordSpec and full encoded bytes against those retained facts. Existing committed protocol fixtures provide full-byte inputs; the currently rewriting wire test is not a safe unchanged-input oracle. An unknown-field schema acceptance law must compare the native round-trip normalization result too, not merely Ajv acceptance.

Third-party test proposal: installed Ajv (draft07 descriptor) + Ajv2020 (domain/neutral schemas), jsonc-parser raw duplicate/framing checks, and the existing Decimal.js affine replacement/absorption reference from packet67. None is a runtime dependency. Numeric/reference/schema results are not Rust ownership or codec execution proof. This read-only plan did not execute a new reference/controller or native law.

### Actual Desired Native Tests Before Cutover

First add one small test in the existing outer tests module, using only current APIs and no not-yet-authored SetN leaf type. Its body stays unchanged through adoption:

```rust
#[test]
fn sync_demo_requires_declared_mutation_metadata() {
    let descriptors = <DemoMutation as Mutation<DemoSnapshot>>::DESCRIPTORS;
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].semantic_kind, "set-n");
    assert_eq!(descriptors[0].aggregate_variant, "SetN");
    assert!(descriptors[0].validate().is_ok());
}
```

This is the concrete current-API desired RED: the first assertion should fail against the current empty roster if the native test can execute. It introduces no extra constructor join and cannot hide missing metadata behind a fake leaf. Retain its actual source/terminal before the adoption. Then add leaf-owned tests after the separate source/test authority release, using the same domain neutral data; do not mirror concrete behavior in a second fake Rust model:

1. `direct_sync_set_n_descriptor_and_provenance`: compare actual leaf descriptor/provenance, per-value descriptor and one-row aggregate roster after the direct leaf exists.
2. `direct_sync_set_n_scalar_inverse_and_absorb`: execute real diff/apply/inverse/absorb against all authored33 cases, preserving same-value behavior and executing stored inverse vectors backwards.
3. `direct_sync_set_n_serde_text_binary_and_timestamp`: execute actual serde and existing generic codecs, exact old/new wire observations, malformed paths, and None timestamps.
4. `direct_sync_set_n_foreign_capability_preserved`: real leaf/aggregate/inverse retain capability true and the original empty foreign list; do not infer one from the other.

The current known OS compiler state can prevent those tests reaching their intended semantic RED. If so retain the actual compiler failure and do not call a source-marker mismatch or Decimal run native RED. Only root/runtime may schedule compilation. Keep fixture67's old151 pre-write receipt and controller untouched; it is bound to the earlier370124 signature preimage and is not a caller-adoption gate.

## Preservation And Separate Limits

Future source proof must capture the current62f319 bytes, not recycle the pre-signature370124 source image. Capture the additional native-RED test source as its own subsequent boundary. Use exact declaration/constructor spans; reinsert the old enum/manual impl and undo only the4 visibility qualifiers,22 constructor wrappers, declared mount/import and newly authored test block to recover62f319 byte-for-byte in memory. Keep original codec bodies, Snapshot/Diff implementation bodies and all original nonconstructor assertions unchanged.

The earlier84 join is frozen; no further async/await edit is implied. The FIFO/Backbone test and reserved heartbeat/fixture_runner_handle call remain outside this adoption. The missing actor fixture owner, broad wire generator, Retained/R17/return paths, actual bounded-retirement RED, and mandatory-default removal in the base are independent work. No full direct-leaf goal completion is inferred from this one caller plan.

## Observed Source Boundary

All exact file reads used any-case lexical Compose exclusion before probes, full-ancestry no-follow validation, O_NOFOLLOW/fstat/read endpoint checks. A large combined derive display truncated; narrower reads recovered the complete relevant DslOps/Mutations sections before this report. No nested repository or unrelated source tree was traversed.

| Actual input | Bytes | SHA-256 |
| --- | ---: | --- |
| [🏪️store/🔄️sync/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs) | 267795 | `62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6` |
| [📡️replication/🎮️mutation/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs) | 57347 | `e5f2f9ce74cc305bcbc23c0d99ab70cc2af54cf299a561f7910d56a7dbbd8385` |
| [📡️spr/🎮️command/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs) | 91352 | `fae229ba917547d57a466a10daefc8a415bcf85d74ea481f0fbf3e1e605e89d3` |
| [🔨️modules/🗣️dsl/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs) | 86471 | `5e02c46094f85d87195816406a826b7f93e0f14a7f3562556331b1262c6e2c05` |
| [🗣️dsl/✨️derive/🦀️component.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs) | 132678 | `dd42e0c13ed15e209879461347fb9003589035fded11d73ed89422ad2a5c48ad` |
| [🔨️modules/📚️library/🔣️mutation-descriptor.schema.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json) | 2406 | `db1c30ab7f19ab9a0f46539c71a427ba6ce51789c5c7904ea4d93dd9ea488aee` |
| [🔨️modules/📚️library/🔣️taxonomy.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json) | 386265 | `7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7` |
| [🧮️demo/🧬️mutations/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/🧮️demo/🧬️mutations/🦀️.rs) | 751 | `5cfacf5a47aef3e32e1e0b89b0de5ca140a91e7e11ba32c35289bd07ccd1fc96` |
| [🧬️mutations/🔢️set-n/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🦀️.rs) | 1359 | `59938f2b07056643b6168fbf1674508cfc4855e01167e05638a3be97c354c722` |
| [🧬️mutations/🔢️set-n/🔣️.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/🧮️demo/🧬️mutations/🔢️set-n/🔣️.json) | 523 | `5c98652892be814f1a90c33dfb48bf4963f4325db9b424a5695d6b75f8b2e941` |

Final endpoint checking found nine of these ten inputs byte-identical, including Sync62f319 and both traits/derive. Taxonomy changed from386265bytes/`7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7` to386313bytes/`6d06daeee0874d243a6ea23ad3f73e053edd4f342884e2738eb49805f6e3300d`. The initial table is retained as historical input, not relabeled current. At observation time the cause was not attributed; root subsequently identified6d06 as its coordinated mandatory-test member. No file was restored. Proposed owner admission must still be checked against the then-current taxonomy before implementation. This is not an all-input-stable release or native result.

The inspected canonical source convention is evidence, not a claim that the new paths are already admitted or compiled. Only this Markdown is the requested handoff; all proposed assets and source edits await root approval.
