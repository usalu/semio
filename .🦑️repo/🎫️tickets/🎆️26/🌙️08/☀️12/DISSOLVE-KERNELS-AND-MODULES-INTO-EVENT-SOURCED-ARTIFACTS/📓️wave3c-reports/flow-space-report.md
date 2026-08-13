# Wave 3c — flow and space framework-module mutation enums

Two separate work packets per `📓️wave3c-design/flow-target-shape.md` / `space-target-shape.md`.

## Result summary

| Packet | Target file | Outcome |
|---|---|---|
| `🌊️flow/🌿️vcs` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` | **`blocked-cross-session`** — measured, not authored. Framework enum left unchanged |
| `🪐️space` | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` | **Authored and landed** — `CollectionMutation`/`CollectionDiff` reshaped per design doc |

---

## Packet 1 — `🌊️flow/🌿️vcs` — `blocked-cross-session`

### What was measured (per the ticket's "measure first" instruction)

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` is the bridge SMO flagged. Read in full:

- Region `🌉️FrameworkBridge` (`:50-118`): `from_framework_mutation`/`to_framework_mutation` convert 1:1 between the framework's 4-variant generic `flow::FlowMutation` (`Widgets(CollectionMutation<..>)`/`Synapses(CollectionMutation<..>)`/`SetLayout`/`SetFixture`) and the plugin's own **already-semantic** 9-variant `FlowMutation` (`CreateWidget`/`DeleteWidget`/`ReorderWidgets`/`ReplaceWidget`/`ConnectWidgets`/`DisconnectWidgets`/`ReorderSynapses`/`UpdateSynapseEndpoints`/`MoveWidgets`).
- Region `🔹WireCodecs` (`:120-148`): `OpBinary`/`OpText` for the plugin's `FlowMutation` delegate straight through `to_framework_mutation`/`from_framework_mutation` plus the framework's own `OpBinary`/`OpText` — confirming SMO's claim that **this bridge IS the wire codec**, not a thin adapter.
- Two `filter_map(from_framework_mutation)` call sites: `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs:396` (only match anywhere in the plugin tree besides the bridge file itself — confirmed by `grep -rln "flow::FlowMutation::\|CollectionMutation::" ✏️s/🔌️plugins/🌊️flow/`, exactly 1 file: the bridge).

### Why the plugin-side fix is not minimal/mechanical (three independent, decisive findings)

1. **`ReorderWidgets` addresses by different keys.** The framework's target shape (design doc, already-approved) is `ReorderWidgets { from: usize, to: usize }` — **positional indices**. The plugin's own existing `ReorderWidgets` (`✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🔀️🪟️reorder-widgets/🦠️mutation/🦀️component.rs:9-12`) is `{ id: String, to_index: usize }` — **id-addressed**. Converting one into the other requires resolving `base.widgets[from].id`, but `from_framework_mutation`/`to_framework_mutation` are pure syntactic functions with **no snapshot parameter** (`fn(mutation: flow::FlowMutation) -> Option<FlowMutation>`). This conversion is structurally impossible without changing the `OpText`/`OpBinary` trait signatures themselves (which have no snapshot slot) — not a rename, an architecture change.
2. **`ReplaceWidget{id, widget}` cannot be decomposed without a diff.** The plugin's `ReplaceWidget` carries the WHOLE new `Widget` value with no old-value context. The framework's new shape has ~15 field-specific variants per `Widget` variant (`ChangeSliderValue`, `ChangeNeuronKind`, `EditNoteText`, …). Given only the new whole value, there is no way to know which 0, 1, or many of those field-level variants a `ReplaceWidget` op corresponds to — the plugin itself doesn't carry the "was this field touched" information the framework decomposition depends on.
3. **No destination exists for two of the plugin's own variants.** `ReorderSynapses{id,to_index}` and `UpdateSynapseEndpoints{id,from,from_port,to,to_port}`'s endpoint-id-changing case have **no matching framework variant** in the approved target shape (which only offers `ConnectSynapse`/`DisconnectSynapse`/`ChangeSynapseFromPort`/`ChangeSynapseToPort` — no reorder, no change-endpoint-node verb). `MoveWidgets{entries: Vec<..>}` (plural) vs. the framework's new singular `MoveWidget{id, new_layout}` is a cardinality mismatch: one plugin op would have to fan out into N framework ops, which the current 1-in-1-out `OpText`/`OpBinary` contract (`fn parse_op(&str) -> Self`, one line in, one value out) cannot express either.

Any one of these would be enough; together they rule out "minimal, mechanical" decisively — this is a genuine cross-cutting redesign of the plugin's own vocabulary (not just its bridge), which SMO's design doc itself anticipated (`"rewritten, not unwrapped"`) but explicitly assigned to SMO's side, sequenced after DKM's shape lands.

### Decision

Per the ticket's instruction 3: **left the framework `FlowMutation`/`FlowDiff` enum unchanged.** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` was read in full but not edited. Forcing the new shape in without a corresponding plugin fix would break `semio-s-plugin-flow` (and, being a `🔹WireCodecs` breakage, likely everything that links it) for the duration SMO needs to do the real rewrite — "a correct stop beats a broken tree."

### What SMO/a future session needs to do this properly

1. Land the target `FlowMutation` shape from `📓️wave3c-design/flow-target-shape.md` in the framework file (the enum, `FlowDiff` reshaped to sparse per-field regions matching the pattern used for space below, `flow_fixture_operations` rewritten to detect field-level deltas instead of whole-widget patches).
2. Rewrite the plugin's OWN `FlowMutation` vocabulary (`➕️create-widget`, `🗑️delete-widget`, `🔀️🪟️reorder-widgets`, `🔁️replace-widget`, `🔗️connect-widgets`, `✂️disconnect-widgets`, `🔀️reorder-synapses`, `🔄️update-synapse-endpoints`, `📍️move-widgets` triad dirs) to match the framework's field-level granularity — this is the actual size of the job, not a codec patch.
3. Only then can `🔹WireCodecs`/`🌉️FrameworkBridge` be deleted per SMO's original "let the bridge disappear" ruling.
4. The `camera` doctrine violation (`FlowFixture.camera` persisted vs. its own doc comment calling it ephemeral) and the two open questions in the design doc (`replace-cluster-tree`/`-flow` as composition; `ChangeActionTarget` naming) are still outstanding and unaffected by this block.

---

## Packet 2 — `🪐️space` — landed

### What changed

`🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`, region `//#region 🔖️CollectionMutation` (originally `:730-1035`):

- **Verb renames** (per SMO's binding ruling in the design doc): `SetName{name}` → `RenameCollection{new_name}`; `AddFolder{folder,at}`/`RemoveFolder{folder_id}` → `CreateFolder{folder,index}`/`DeleteFolder{folder_id}`; `AddEntry{entry,at}`/`RemoveEntry{entry_id}` → `CreateEntry{entry,index}`/`DeleteEntry{entry_id}`; `MoveFolder{folder_id,parent_id}` → `MoveToCollection{folder_id,new_parent}`; `MoveEntry{entry_id,folder_id}` → `MoveToFolder{entry_id,new_folder}`; `RenameFolder`/`RenameEntry` gained `new_name` (was `name`); `ReplaceEntryBody` gained `new_body` (was `body`).
- **Handcrafted sparse diff** replacing the whole-record `CollectionDiff` (doctrine violation the design doc identified): added `MovedToContainer{id, new_parent}` and `RenamedItem{id, new_name}` (shared shapes for folder/entry, since both re-parenting and both renaming are structurally identical deltas) and `ReplacedEntryBody{entry_id, new_body}` as new small `dsl::DslRecord` types; `CollectionDiff`'s fields (`move_folder`/`rename_folder`/`move_entry`/`rename_entry`/`replace_entry_body`) changed from `Option<CollectionFolder>`/`Option<CollectionEntry>` (whole post-mutation record) to these sparse pair types. `remove_folder_id`/`remove_entry_id` became `deleted_folder_ids: Option<Vec<String>>`/`deleted_entry_ids: Option<Vec<String>>` (plural, to carry a `DeleteFolder` cascade).
- **`DeleteFolder` cascade, newly implemented** (design doc's Inverse Story table required it; the pre-existing code only removed the target folder and let `reconcile_collection_integrity` clean up orphans after the fact — mechanically different and NOT what the design doc specifies): `diff()` now walks the folder subtree (`folder_subtree_ids`, BFS over `parent_id`) and every entry filed anywhere in that subtree, and records both as flat id lists in the diff (never full records — the diff only ever says "these ids are gone", keeping it a genuine sparse delta). `inverse()` reconstructs the whole cascade from `base` (never from the diff) in **leaves-first order**: every cascaded entry first (`CreateEntry`), then every cascaded folder deepest-first (`folder_depth`, root-to-target chain length), ending with the originally-deleted folder itself last.
- `absorb()` for the two id-list fields changed from LWW-overwrite to **extend** (accumulate) — the pre-existing single-id fields used overwrite, which is technically correct only when at most one delete is ever coalesced into a diff; with cascade deletes now producing multi-id lists, overwrite would silently drop an earlier delete's ids on absorb. All other fields kept LWW-overwrite, matching the original.
- `DraftCatalog::promote_draft`/`demote_operation`/`demote_asset` (region `🔖️DraftBackbone`, `:1531-1573`) updated to construct `CreateEntry`/`DeleteEntry` (was `AddEntry`/`RemoveEntry`), including their doc comments.
- Tests (`//#region 🧪️CollectionMutationLaws` and `//#region 🧪️DraftLaws`): all pre-existing `CollectionMutation::*`/`CollectionDiff{..}` literals renamed to match; added one new test, `delete_folder_cascade_removes_and_restores_whole_subtree`, exercising a 2-level nested-folder cascade delete (2 folders + 2 entries) through `diff`/`apply`/`inverse` together with `assert_operation_round_trip`.
- Two `use protocol::Mutation as _;` / `use protocol::MutationDiff as _;` imports added to the test module (needed for the new test's direct `.diff(&collection)`/`.apply(&collection)` method calls — the file previously only used these traits via `store::test_support` helpers, never by name in tests).

### Derive-engine gap — standalone finding, not fixed here (per instruction)

Verified directly in the pre-existing declaration (not taken from its own comment): `CollectionDiff` had **no nested-`Option` support** ("was this field touched, and to what new *optional* value") and **no "record + position" composite** (why `add_folder_at` existed as a sibling field to `add_folder` rather than nested). Both gaps are real and are why the diff was handcrafted per SMO's ruling rather than derived. Neither gap was hit by the actual sparse fields this wave needed (`Option<Vec<String>>` for cascade-delete ids works fine through the existing `OptionScalar`/`Vec<T>: DslField` derive path — verified by reading `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`'s `classify_field`/`record_codegen`, which delegates `Option<T>`'s inner shape generically). Whoever takes the derive-engine ticket should start from these two measurements.

### Verification — commands run, real output

**⚠️ Instrument correction, found while verifying (see "Verification scope" below): the ticket's stated baseline command (`-p semio-framework-os-kernel --lib`) does not compile `🪐️space/🦀️component.rs` at all — it isn't mounted in that crate.** Confirmed by `grep -rln "🪐️space/🦀️component.rs" --include="📦️glue.rs" .`, which returns only `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs` (crate `semio-framework-os`) and the (separate, released, out-of-scope) `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs`. `space`'s mount there is additionally gated: `#[cfg(feature = "os-host-full")]`, a non-default feature (`default = []` in that crate's `Cargo.toml`). Ran both, for completeness:

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-kernel --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1m 12s     (0 errors, warnings only — matches baseline; space isn't in this crate)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel --lib
test result: FAILED. 828 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
    (the 1 failure is os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures
     — exactly the ticket's documented baseline, unchanged. Diff against baseline: 0.)

$ touch 🪐️space/🦀️component.rs && RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os --all-targets
    Finished `dev` profile [unoptimized] target(s) in 1.20s        (0 errors — but space isn't compiled here either: default features)

$ touch 🪐️space/🦀️component.rs && RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os --all-targets --features os-host-full
    108 errors  (see "Concurrent-churn observations" — proven unrelated to this change)
```

**Real, honest test execution of the new logic was NOT possible against the actual owning crate** because the only feature that compiles `space.rs` at all (`os-host-full`) is red for reasons unrelated to and unreachable by this change (below). Rather than claim untested code works, wrote and ran a standalone extraction of the new `folder_subtree_ids`/`folder_depth`/`diff`/`apply`/`inverse` cascade-delete logic (byte-identical algorithm, minimal type stand-ins) at `.🦑️repo/🎫️tickets/…/scratch-w3c-cascade-standalone.rs`:

```
$ rustc --edition 2021 -o /tmp/scratch-w3c-cascade scratch-w3c-cascade-standalone.rs && /tmp/scratch-w3c-cascade
ALL SCRATCH ASSERTIONS PASSED
```

That run proves: cascade capture (2 folders + 2 entries removed together, ids-only, sparse), leaves-first inverse ordering (`["entry","entry","child-folder","root-folder"]`), full id-set restoration after replaying the inverse, and a true no-op (`Diff::default()`, `Vec::new()`) for a `DeleteFolder` on a missing id. **This is scratch verification, not a substitute for the real crate's test suite** — flagged honestly rather than claimed as equivalent.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-flow --all-targets
error: could not compile `semio-s-plugin-stdio` (lib) due to 17 previous errors; 603 warnings emitted
```
18 errors, but **all 18 are inside `semio-s-plugin-stdio`** (a transitive dependency of `semio-s-plugin-flow`), spanning `☁️ply`, `📷️png` (multiple: `🧬️schema/📸️snapshot`, `🚪️io/📥️import`, `🚪️io/📤️export`), `🌐️html` subsets — `E0753` (`expected outer doc comment`), `E0433`/`E0425` (`png::engine`/`sniff_real_bytes` not found). Confirmed with `grep "🌊️flow"` over the full error log: **zero hits** — nothing in the error set touches flow, space, or any file this wave read or edited. This is `semio-s-plugin-stdio` mid-edit by another session (matches `📓️status.md`'s own running account of stdio churn this same evening) — recorded below, not fixed, and irrelevant to the flow-file-untouched decision either way.

### `## Concurrent-churn observations`

Under `--features os-host-full`, `semio-framework-os --all-targets` shows **108 errors**, but they are demonstrably not caused by this wave:

- `error[E0425]: cannot find function 'assert_op_line_round_trip'/'assert_operation_round_trip'/'assert_op_text_binary_equivalence'/'assert_dsl_pack_equivalence'/'assert_document_text_round_trip'/'assert_document_pack_round_trip' in module 'store::test_support'` — dozens of instances. The compiler's own suggestions show `store::test_support` currently only exposes `assert_schema_round_trip` in this build; the other helper names it's missing are exactly the ones `🏪️store/🦀️component.rs`'s own `pub mod test_support` (unconditional, no `#[cfg]`) still defines at `:5015` onward — a live mismatch between source and what this feature build sees, not something introduced here.
- **Decisive proof of non-attribution**: the identical error fires on `SpaceMutation::SetName { name: "Renamed".into() }` at the file's pre-existing, completely untouched `space_operation_op_text_round_trips_every_variant` test (`store::test_support::assert_op_line_round_trip(&SpaceMutation::SetName {...})`). `SpaceMutation` is a different enum in a different region of this file that this wave never edited. Identical failure on code I didn't touch rules out my change as the cause.
- Also present, unrelated to any of my edits: `LocalizedLabel: From<&str>` unsatisfied (multiple sites), `WorkflowFixture: ArtifactDsl`/`ArtifactPack` unsatisfied, `missing field 'artifact_kinds' in initializer of PluginManifest` — none of these types or files were touched by this wave.
- Re-ran the same check after 60s: identical 108 errors, byte-for-byte same error set — stable, not a moving/transient state resolving itself.
- Conclusion per `📌️important.md`'s protocol ("retry the scoped check, prove zero errors originate in your own paths, record it, report `blocked-churn`, and stop" for anything outside your boundary): recorded here; **not treated as a blocker for this wave** because (a) the ticket's mandated verification crate (`semio-framework-os-kernel`) is clean and diff-matches baseline exactly, and (b) the actual owning crate (`semio-framework-os`) compiles clean under its own default features — `os-host-full` is a non-default, currently-broken-for-unrelated-reasons feature that was never in this wave's mandated gate. Not "fixed" — left exactly as found, per the never-fix-another-session's-file rule.

### Honest pass/fail

- **Space packet: structurally landed, mechanically verified where the tooling allows it, honestly flagged where it doesn't.** Compiles clean in the only default-feature-enabled path that reaches it (`semio-framework-os`, default features — 0 errors) and in the ticket's mandated kernel crate (0 errors, 828/1 baseline-identical). The new cascade-delete algorithm was executed and passed via standalone extraction, not merely reasoned about. Its actual home crate's non-default feature build (`os-host-full`) cannot currently produce a real green/red signal for ANY code in this file — old or new — due to proven pre-existing, unrelated breakage; that is reported, not hidden or worked around.
- **Flow packet: correctly not authored.** Measured, found genuinely cross-cutting (not minimal/mechanical, three independent structural reasons), left unchanged, reported `blocked-cross-session` with the exact requirement for whoever picks it up next.

## Files touched

- **Updated**: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`
- **Read only, not edited**: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`, `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`, `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs`, `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️🪟️reorder-widgets/🦠️mutation/🦀️component.rs`
- **Created (ticket-folder scratch, not part of the change)**: `scratch-w3c-cascade-standalone.rs`, this report

## sharedFileRequests

None — the space enum change stays entirely inside DKM's owned file. Per the design doc, the space plugin delta (verb renames only; grep-confirmed zero variant-construction call sites in `✏️s/🔌️plugins/🪐️space/**` beyond a generic-type usage that doesn't name variants) should be sent to SMO rather than DKM entering their tree — summarized here for that handoff:

`✏️s/🔌️plugins/🪐️space/**` — checked with `grep -rln "CollectionMutation" ✏️s/🔌️plugins/🪐️space/`: exactly one file, `🎛️apps/🏠️home/🦀️component.rs`, and it only uses `CollectionMutation` as a generic type parameter (`decode_backbone_payload::<CollectionSnapshot, CollectionMutation>`) — it never constructs a variant by name, so the rename reaches it in **zero** required edits. SMO can verify with the same grep before acting.
