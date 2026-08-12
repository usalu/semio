# B2 — store composition (`🔖️Composition` + `🔖️CompositionCoordinator`) report

Scope: `semio-framework-os-kernel`. Primary file —
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — plus the sanctioned collateral
`CompositionPin` type correction in `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`.

## TASK 1 — `🔖️Composition` region in store

New region `🏪️store/🦀️component.rs:166-571`, inserted immediately after `//#endregion 🔖️Schemas`
(`:164`), before `//#region 🔖️Authority`.

- `:200` — `pub struct ArtifactChild<S> { child_id: String, target: crate::os_io::ArtifactRef,
  _snapshot: PhantomData<S> }`. `Serialize`/`Deserialize` are `#[derive(..)]`'d with
  `#[serde(bound = "")]` (clears the auto-generated `S: Serialize + Deserialize` bound serde would
  otherwise add for a `PhantomData<S>` field). `Clone`/`Debug`/`PartialEq` are **hand-implemented**
  (`:220-238`), not derived — deriving those three on a `PhantomData<S>`-carrying struct adds an
  unwanted `S: Trait` bound to the generated impl even though `S` never appears in any
  stored/compared data, which would silently break the "works for any `S`" requirement the moment a
  caller picked an `S` that itself lacked one of those traits. `ArtifactChild::new`/`to_child_ref`
  at `:206-217`.
- `:243` — `pub struct ChildRef { slot, child_id, target }` — the type-erased projection dropping
  `S`, so `ArtifactRefs::child_refs` can return one homogeneous `Vec` across differently-`S`-typed
  child-slot fields.
- `:255` — `pub struct OwnerRef { parent: ArtifactRef, slot: String, child_id: String }` — the
  ownership stamp placed on the CHILD's own envelope.
- `🏪️store/🦀️component.rs:102` — `ArtifactEnvelope` gains `#[serde(default,
  skip_serializing_if = "Option::is_none")] pub owner: Option<OwnerRef>`.
- `:267` — `ArtifactLink { target, pin: LinkPin, role: String }`; `:278` —
  `LinkPin { Head, Checkpoint { id }, Snapshot { blob: BlobRef } }` (reuses the file's existing
  `BlobRef` at `:3869` — found it already exists exactly as the task hinted, `🔖️BlobStore`
  region — no new type invented).
- `:289` — `ArtifactRefs` trait, `child_refs`/`links` both defaulting to `Vec::new()`.
- `:302` — `LinkState { Resolved { pack_bytes, dialect }, Missing, PinnedOnly { blob } }`; `:312` —
  `LinkResolver { fn resolve(&self, &ArtifactLink) -> LinkState }`.
- `:324` — `ChildStoreFactory { fn create(...) -> Result<Box<dyn SpaceMember>, VcsError>; fn
  open(...) -> Result<Box<dyn SpaceMember>, VcsError> }` — `Result`-wrapped (the task brief's bare
  `Box<dyn SpaceMember>` shorthand loosened to match every other fallible constructor in this file,
  e.g. `ArtifactCodec::of`'s bridge fns). Registry at `:328-345` mirrors `register_document_codec`
  exactly (`OnceLock<RwLock<HashMap<...>>>`, `register_child_store_factory`/`child_store_factory`),
  keyed by `ArtifactKindId`.
- `:347-570` (`🔖️CompositionDsl` sub-region) — hand-crafted `crate::os_dsl::DslField` impls for
  `ArtifactChild<S>`/`OwnerRef`/`LinkPin`/`ArtifactLink`, each an ordinary `Shape::Record` (NOT a
  new `Shape` variant, per design doc deviation D1). `crate::os_io::ArtifactRef` fields encode as
  their `to_uri()` wire string (`Shape::Text`), the same "own the codec at this edge" reasoning
  `CompositionPin` already uses for the identical field shape.

## TASK 2 — `🔖️CompositionCoordinator` region

New region `🏪️store/🦀️component.rs:4494-5003`, inserted immediately after `//#endregion 🔖️Space`
(`:4492`), before `//#region 🔖️TestSupport`.

### `SpaceMember` extension (`:3487-3973` trait, `:3510-4092` blanket impl over `ArtifactStore<P, Mutation>`)

Eight new object-safe methods. Three are the task's own literal list; **five are deliberate,
necessary additions beyond it** — documented inline at the trait (`:3913-3925` preamble comment)
and here:

| Method | Kind | Why |
|---|---|---|
| `validate_wire(&self, ops: &[Vec<u8>]) -> Result<(), String>` (`:3933`, spec'd) | decode+validate | Decodes each op as an individually-`OpBinary`-encoded `Mutation` (task hint: "using `Mutation::validate`"), threads a cloned snapshot forward across the whole slice, never applies. |
| `dispatch_wire(&mut self, cmd_bytes: &[u8]) -> Result<CommandReceipt, VcsError>` (`:3941`, spec'd) | apply | `self.dispatch_binary(cmd_bytes)` verbatim — `cmd_bytes` is one full `ArtifactCommand::Apply` binary blob, built by `CompositionCoordinator` (see `build_apply_command_bytes` below) by replicating `write_command_ops`'s exact byte layout WITHOUT decoding individual ops, so the coordinator never needs to know any member's concrete `Mutation` type. |
| `tail_group_id(&self) -> Option<String>` (`:3946`, spec'd) | getter | `MutationMeta.group_id` of the tail applied edit's last op. |
| `tail_edit_id(&self) -> Option<String>` (`:3950`, **new**) | getter | `tail_group_id`'s companion — `GroupReceipt`/`GroupUndoReport` need the actual edit id, not just group membership. |
| `redo_tail(&self) -> Option<(String, Option<String>)>` (`:3955`, **new**) | getter | `(tail_group_id, tail_edit_id)`'s redo-direction mirror, powering `redo_group`. |
| `stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError>` (`:3964`, **new**) | setter | The mechanism that makes "each stamping `group_id = Some(invocation_id)`" (design doc, Phase 2) actually work: an ordinary `Apply` (`ArtifactStore::replay_mutations`, `🏪️store/🦀️component.rs:3095`) hard-codes `group_id: None` on every `MutationMeta` it builds — there is no way to pass an externally-minted group id INTO the normal apply path, so `dispatch_group` dispatches each member's `Apply` first, then stamps the shared id onto that member's just-created tail edit via this method. |
| `set_owner(&mut self, owner: Option<OwnerRef>)` (`:3972`, **new**) | setter | Genesis needs a way to write `ArtifactEnvelope.owner` on a freshly-created child through the type-erased `Box<dyn SpaceMember>` interface — no ordinary `Apply` mutation can reach envelope metadata (only the document snapshot `P`), so this needed its own object-safe setter. |

Without the five additions, "each stamping `group_id`" and "OwnerRef stamp on the CHILD's own
envelope" (both explicit design requirements) would have had no implementable path — they are not
scope creep, they are the mechanism the two spec'd requirements above them depend on. All five are
mechanical one-liners over existing `ArtifactStore` fields (`applied_edit_ids`/`redo_edit_ids`/
`envelope.vcs.edits`/`envelope.owner`), not new state or new invariants.

### New value types

- `ChildDispatch { child: ArtifactRef, ops: Vec<Vec<u8>>, op_schema: SchemaId, labels: Vec<String>
  }` (`:4514`). **Design decision**: each element of `ops` is one individually-`OpBinary`-encoded
  `Mutation` — i.e. the SAME per-op wire shape `ArtifactCommand::Apply.mutations` bundles (proven by
  `write_command_ops`, `🏪️store/🦀️component.rs:2366`: count-varint + per-op length-varint +
  already-encoded op bytes). This is what makes `build_apply_command_bytes` (`:4707`) possible
  WITHOUT the coordinator ever decoding a `Mutation` — it just replicates that exact byte layout
  from raw `Vec<u8>`s. `op_schema`/`labels` are accepted and carried through but **not yet
  interpreted** by `dispatch_group` itself (forward-compat audit/diagnostic metadata) — flagged
  under `sharedFileRequests` below.
- `ChildGenesis { slot: String, dialect: ArtifactDialect, initial_pack: Vec<u8> }` (`:4526`).
- `GroupReceipt { invocation_id: String, member_edits: Vec<(ArtifactRef, String)>,
  created_children: Vec<(ArtifactRef, Box<dyn SpaceMember>)> }` (`:4540`). **Design decision**:
  added `created_children` beyond the task's literal 2-field shape — a `ChildGenesis`-created member
  has no pre-existing caller-held reference the way `children`'s entries do (the caller couldn't
  have passed `&mut dyn SpaceMember` for something that doesn't exist yet), so without this field
  every freshly-created child would be silently dropped the moment `dispatch_group` returns, making
  `ChildGenesis` pointless. No `Clone`/`Debug`/`PartialEq` (a `Box<dyn SpaceMember>` supports none).
- `GroupMeta { actor: Option<String>, description: Option<String>, coalesce_key: Option<String> }`
  (`:4553`). `description` feeds every dispatched member's own `Apply.description`. **Scoping
  decision**: `actor`/`coalesce_key` are accepted but NOT wired — `SpaceMember` has no
  `set_local_actor_id`/`AmendLast` object-safe seam today (only `ArtifactStore`'s own inherent API
  does), so honoring them would mean either growing `SpaceMember` further (beyond the 8 additions
  already justified above) or reaching through `as_any_mut()` per-technology, both out of this
  wave's scope. Flagged under `sharedFileRequests`.
- `GroupUndoReport { undone: Vec<(ArtifactRef, String)>, skipped: Vec<(ArtifactRef, VcsError)> }`
  (`:4567`).
- `CompositionGraph { owns: HashMap<String, (String, String)>, links: HashMap<String,
  HashSet<String>> }` (`:4580-4658`). **Design decision**: `CompositionCoordinator::dispatch_group`'s
  literal signature (per the task brief) takes no graph parameter, yet phase 1 must do "ownership/
  cycle checks" against SOMETHING — `SpaceMember` is fully type-erased (no `ArtifactRefs` access), so
  a global cross-document graph cannot be derived from the call's own arguments alone. Resolved by
  making `CompositionCoordinator` itself STATEFUL (`graph: CompositionGraph` field,
  `graph()`/`graph_mut()` accessors, `:4798-4809`), incrementally maintained across calls
  (`CompositionGraph::sync_member`, `:4649-4658`, callable by a host like `SpaceHost` after any
  dispatch that might change an artifact's own `ArtifactRefs`). `dispatch_group`'s phase 1 therefore
  checks `self.graph.owner_of(child) == Some(parent)` for every `ChildDispatch` (rejecting with
  `OwnershipViolation` otherwise) and `self.graph.would_cycle_owns(parent, minted_child_id)` for
  every genesis slot (rejecting with `CompositionCycle`). `would_cycle_owns`/`would_cycle_links`
  (`:4592`/`:4614`) are the two methods the task named explicitly; `insert_owns`/`remove_owns`/
  `insert_link`/`remove_link`/`owner_of`/`slot_of`/`links_from` round out a genuinely reusable
  standalone graph (usable by a UI doing "would this drag-and-drop cycle" checks with no live
  `CompositionCoordinator` at all).
- `CompositionCoordinator { graph: CompositionGraph }` (`:4798`).

### `dispatch_group` (`:4866-4943`)

Two-phase exactly as specified:
- **Phase 1** (`:4867-4900`, region `Phase1Validate`): `parent.validate_wire(parent_ops)`; per
  child, ownership check against `self.graph` then `member.validate_wire(dispatch.ops)`; per
  genesis, mint the deterministic id (see below), reject on `would_cycle_owns`, reject if no
  `ChildStoreFactory` is registered for the kind. Any failure returns immediately — nothing
  dispatched anywhere (proven by `dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing`,
  test list below).
- **Phase 2** (`:4902-4941`, region `Phase2Apply`): geneses via `factory.create` +
  `member.set_owner(...)` + `self.graph.insert_owns(...)` → child edits via `dispatch_wire` +
  `stamp_tail_group_id` (borrow-checker note: iterates `children` by INDEX, `for index in
  0..children.len()`, not `.iter_mut()`, specifically so the borrow of `children[index]` ends
  before a failure branch needs to re-borrow all of `children` for `compensate`) → parent ops the
  same way.
- **Compensation** (`compensate`, `:4811-4827`, private — deliberately kept private, see
  "Design decisions" below): on any phase-2 failure, undoes already-applied members in REVERSE order
  (parent first if it was itself applied, then children in reverse dispatch order), collecting a
  `GroupUndoReport` rather than propagating the first undo failure. `fold_compensation_error`
  (`:4762-4770`) folds the ORIGINAL error unchanged when every rollback succeeded, or wraps both
  into the new `VcsError::CompensationFailed` (see below) when `report.skipped` is non-empty.
- **Deterministic ids**: `mint_child_id` (`:4748`, `pub`) — `content_addressed_entity_id("child",
  parent_id || 0 || slot || 0 || parent_edit_fingerprint || 0 || ordinal)`, exactly the design doc's
  formula, reusing `🌿️vcs`'s `content_addressed_entity_id`. This one id is used for BOTH the new
  child's `ArtifactRef.artifact_id` and its `ArtifactChild<S>.child_id`/`OwnerRef.child_id`.
  `mint_invocation_id` (`:4762`, private) hashes the parent id, `parent_ops`' fingerprint, and every
  dispatched child's `(child_id, ops fingerprint)` pair sorted by child id — two replicas performing
  the identical composite gesture converge on the identical `GroupReceipt.invocation_id` /
  `MutationMeta.group_id` stamp with zero coordination. `concat_ops_fingerprint` (`:4736`) is the
  shared order-and-length-sensitive fingerprint both mint functions consume.

### `undo_group`/`redo_group` (`:4967`/`:4988`)

Free-standing associated functions on `CompositionCoordinator` (no `&self`/`&mut self` needed — they
don't touch the ownership graph, only per-member tail/redo state), taking caller-ordered
`&mut [(&ArtifactRef, &mut dyn SpaceMember)]`. `undo_group`: undoes every member whose
`tail_group_id() == Some(group_id)`, skips (never aborts) every other member, recording
`VcsError::ForeignEdit` for a mismatched/absent group and the member's own error for a failed
`undo()` — generalizes the existing benign `NothingToUndo`/`ForeignEdit` collapse
(`🔌️plugin/🦀️component.rs:5595,5620,5642`) to the multi-member case. `redo_group` mirrors it via
`redo_tail()`.

## `CompositionPin` type correction (per the ticket's IMPORTANT CORRECTION)

`🌿️vcs/🦀️component.rs:112` — `CompositionPin.child_ref` changed from `String` (the prior wave's
wire-URI fallback) to the real `crate::os_io::ArtifactRef`. Confirmed the prior wave's stated reason
(cross-crate dependency direction) was wrong: `io/🦀️component.rs` is dual-mounted — `semio-framework`
mounts it as `io`, `semio-framework-os-kernel` mounts the identical file as `os_io`
(`💻️os/📦️packages/🦀️rust/📦️glue.rs:238`, `pub mod os_io;`) — no cross-crate import needed at all,
just `crate::os_io::ArtifactRef`, exactly as `store` already does for `ArtifactDialect` at
`🏪️store/🦀️component.rs:88/105`.

- `content_addressed_checkpoint_id` (`🌿️vcs/🦀️component.rs:405`) — the pin-ordering sort now keys on
  `pin.child_ref.to_uri()` (`:401-409`) rather than `Ord` on the `String` field directly (`ArtifactRef`
  has no `Ord` impl) — deterministic ordering preserved, byte-for-byte hash input unchanged for the
  same logical pin set (verified by the existing
  `content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible` test,
  updated in place to build `ArtifactRef`s via `parse_uri` instead of raw strings, still passing).
- All 5 `CompositionPin { child_ref: "...".into(), .. }` test literals
  (`🌿️vcs/🦀️component.rs:588-609`, was `593-608`) updated to
  `crate::os_io::ArtifactRef::parse_uri(...).expect(...)`.
- Doc comment at `CompositionPin`'s own definition (`:97-111`) rewritten to record the correction and
  point at this report.
- No other construction site exists anywhere in the workspace (`grep -rn "CompositionPin"` — every
  hit is inside `🌿️vcs/🦀️component.rs` itself; the two mentions in `🏪️store/🦀️component.rs:741,2542`
  are comments, not code).

## Envelope-dialect scope-discipline decision — **DEFERRED**

Assessed blast radius before touching anything (per the ticket's SCOPE DISCIPLINE instruction):
- `ArtifactEnvelope { .. }` raw struct-literal construction sites, whole workspace: exactly **3**
  files — `🏪️store/🦀️component.rs` (mine, 3 sites, already fixed for `owner`), plus
  `🧰️framework/🛍️products/💻️os/🦀️component.rs` and `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
  (2 sites each). The latter two are **already missing `dialect`/`migrated_from`** — i.e. already
  broken against the CURRENT `Option<ArtifactDialect>` shape, predating this wave entirely (see
  Concurrent-churn observations below for why).
- The real blast radius is `create_document_envelope` (`🏪️store/🦀️component.rs:1205`, was `:789`) —
  the constructor every real document kind actually calls, whose signature has NO `dialect`
  parameter at all today (it hardcodes `dialect: None`). **106 files / 168 call sites** across
  `🧰️framework/` + `✏️s/` call it. Making `dialect` required would force every one of those 168 call
  sites to be updated with a real, meaningful `ArtifactDialect` — a repo-wide semantic decision
  (each plugin declaring its own canonical `s.<plugin>.<artifact>` kind) that A1's report explicitly
  scoped OUT of this ticket ("renaming existing artifact ids to this grammar is a later wave").
- **Decision: DEFERRED.** `ArtifactEnvelope.dialect` stays `Option<crate::os_io::ArtifactDialect>`,
  unchanged. Recorded under `sharedFileRequests` below for whichever later wave actually threads
  `ArtifactKindId`/dialect canonicalization through the plugin ecosystem.

## Tests added (15, all passing — see Verification)

All inside the existing `#[cfg(test)] mod tests` (`🏪️store/🦀️component.rs`), new
`//#region 🔖️CompositionTests` sub-region right before the module's closing brace — no new test
files, per policy.

- `artifact_child_dsl_field_round_trips_via_pack_and_value`, `owner_ref_dsl_field_round_trips_via_pack`,
  `artifact_link_dsl_field_round_trips_every_link_pin_variant` (all 3 `LinkPin` variants) — DSL/Pack
  round-trip proof for every new value type, via `crate::os_pack::encode_record_body`/
  `decode_record_body` directly (the same pack path `BackboneMessage`'s handcrafted `OpBinary` uses)
  plus the `DslField` trait surface itself.
- `artifact_refs_defaults_to_empty_for_a_leaf_snapshot`, `link_resolver_reports_resolved_missing_and_pinned_only_states`.
- `composition_graph_owns_forest_rejects_second_owner_and_cycle`,
  `composition_graph_links_reject_cycle_but_allow_converging_dag_edges` — forest/acyclicity law.
- `mint_child_id_converges_across_two_replicas_and_varies_by_ordinal_and_slot` — bare-helper
  determinism.
- `dispatch_group_validate_all_atomicity_one_bad_member_applies_nothing`,
  `dispatch_group_rejects_a_child_the_graph_does_not_track_as_owned` — validate-all atomicity +
  ownership-check law (uses a dedicated `ValidatedMutation` test fixture with a real `validate`
  override, since `DemoMutation` always accepts and cannot exercise this path).
- `compensate_undoes_applied_members_in_reverse_order`,
  `compensate_reports_skipped_when_a_members_own_undo_fails_and_folds_to_compensation_failed` —
  compensation on late failure, calling the real (private, test-module-visible) `compensate`/
  `fold_compensation_error` directly, including the "compensation itself fails" path.
- `dispatch_group_mints_genesis_child_ids_deterministically_across_replicas` — end-to-end
  determinism through `dispatch_group` + a real `ChildStoreFactory` fixture (`DemoChildFactory`),
  two independent coordinators/parents converging on the identical child id and `invocation_id`.
- `undo_group_skips_a_foreign_tail_member_but_still_undoes_the_rest`,
  `redo_group_skips_a_foreign_tail_member_but_still_redoes_the_rest` — best-effort group undo/redo.

## Verification (actually run)

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo check -p semio-framework-os-kernel
```
Result: **clean, 0 errors.** `scratch-w1b2-check-1.txt` shows the ONE real mistake this wave made
and self-caught (`error[E0753]`: two `//!` inner-doc-comment region-overview blocks placed mid-file,
invalid outside a genuine `mod`/crate-root position — fixed by converting both to plain `//`
comments). `scratch-w1b2-check-2.txt`/`-3.txt` (re-run after the fix, and again as the final
verification pass) are both clean. Warning count: **49** — `grep "generated .* warnings"
scratch-w1b2-check-3.txt` → `semio-framework-os-kernel (lib) generated 49 warnings`, matching the
stated baseline exactly (0 regressions). `grep -c "^error"` on the final log → `0`.

```
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/🎯️target" cargo test -p semio-framework-os-kernel --lib
```
Result (`scratch-w1b2-test-1.txt`): **817 passed; 2 failed** — the baseline's stated `802 passed, 2
failed` **+ exactly the 15 new tests above, all passing** (`grep -c "#\[test\]"` inside my new
region → 15; `817 - 802 = 15`). The 2 failures are the SAME two the baseline documented:
`os_dsl::fixture_sweep::m5_handcrafted_grammar_conformance::all_discovered_snapshot_grammars_recognize_their_shipped_fixtures`
and
`os_dsl::fixture_sweep::m5_production_coverage::all_discovered_grammars_report_uncovered_productions_for_their_shipped_fixture`,
confirmed still failing on the exact same three artifacts (`grep` in the log):
`[DEBUG] 🏗️fem::◻2d::🔖️1: uncovered productions (4) = document, header, body, payload`,
`[DEBUG] 📕️norm::📘️en1992::🔖️1: ...`, `[DEBUG] 🕸️dag::🕸️dag::🔖️1: ...` — the concurrent
SEMANTIC-MUTATIONS-OVERHAUL fan-out B1's report already identified, not mine to fix, no change
in shape from B1's report. Every one of my own targeted new tests appears in the log as `... ok`
(explicit list in "Tests added" above, confirmed via `grep -E "<15 test names>"`).

**Honest status**: `cargo check` fully clean, matches baseline exactly. `cargo test --lib` is the
same non-regression shape B1 reported (2/819 fail, both pre-existing/concurrent, not mine) — 15/15
new tests pass.

## Concurrent-churn observations

- `git status --porcelain` at report time shows continued heavy `SEMANTIC-MUTATIONS-OVERHAUL`
  fan-out under `✏️s/🔌️plugins/**/🧬️mutations/**` (norm/energy/space/puzzle/animate/gis/flow/etc.) —
  none of it touches `🏪️store/🦀️component.rs` or `🌿️vcs/🦀️component.rs`, so no action needed.
- While investigating the envelope-dialect blast radius, found `🧰️framework/🛍️products/💻️os/🦀️component.rs`
  and `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` construct `ArtifactEnvelope { .. }` literals
  that are **already missing `dialect`/`migrated_from`** (fields that landed in an EARLIER,
  unrelated ticket, `26/08/10` D4 evolution slice — well before this one). These two files belong to
  `semio-framework-os-flow` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust`,
  package name `semio-framework-os-flow`, a real workspace member per the root `Cargo.toml`), NOT
  `semio-framework-os-kernel` — confirmed via `📦️glue.rs`'s own comment,
  `"Infinite/flow component files exist under 🔨️modules/ but are unwired pending dep-DAG cleanup"`.
  This crate appears to already be red independent of anything in this ticket; `cargo check -p
  semio-framework-os-kernel`/`--lib` (my mandated verification, which does not build
  `semio-framework-os-flow`) cannot observe it either way, so I neither touched these two files nor
  ran a check against that crate (out of my boundary this wave) — flagging as a pre-existing,
  unrelated observation, not `blocked-mechanism` (nothing of mine failed).
- No other churn observed touching `🏪️store/**`/`🌿️vcs/**` — both stayed exclusively mine for the
  duration of this wave.

## sharedFileRequests

Everything the plugin-layer / later waves will need from this wave:

1. **`ArtifactEnvelope.dialect` stays `Option<ArtifactDialect>`** (see "Envelope-dialect
   scope-discipline decision" above) — 106 files / 168 `create_document_envelope` call sites would
   need updating to make it required. Whichever wave adopts `ArtifactKindId`/canonical dialects
   per-plugin should thread a real `dialect` argument through `create_document_envelope`'s signature
   (`🏪️store/🦀️component.rs:1205`) at the same time it flips this field, not before.
2. **`ChildDispatch.op_schema`/`GroupMeta.actor`/`GroupMeta.coalesce_key` are accepted but not yet
   interpreted** by `dispatch_group` (see the type docs at `🏪️store/🦀️component.rs:4507-4512`,
   `:4550-4552`). A later wave that wants schema-checked ops, per-member actor stamping, or
   `AmendLast`-style coalescing through the composite path will need to extend `SpaceMember`
   further (a `set_local_actor_id`/`dispatch_wire_amend` object-safe seam) — not done here to avoid
   growing the already-8-method-larger `SpaceMember` trait speculatively.
3. **`ArtifactEnvelope.owner`/`Checkpoint.composition_pins` are both in-memory-only** — neither
   `.spr` (`crate::os_spr::HistoryLog`/`HistoryCheckpoint`) nor the `.ops` text grammar carries them
   yet (same deferral B1 already flagged for `composition_pins`; `owner` follows the identical
   pattern — see the three `owner: None` sites at `🏪️store/🦀️component.rs:1209`
   (`create_document_envelope`), `:1812` (`parse_document_spr`'s `HistoryCheckpoint → Checkpoint`
   map), and `:1974` (the `.ops` text parse path). A save/load round trip through `.spr`/`.ops` currently silently drops `owner`. The
   `CompositionCoordinator`-adjacent wave that extends `HistoryCheckpoint` for `composition_pins`
   (per B1's own `sharedFileRequests`) should extend `HistoryOpMeta`-style encode/decode for `owner`
   in the same pass, mirroring the `group_id` bitmask-presence-bit pattern B1 used in
   `📡️spr/📜️history/🦀️component.rs`.
4. **`ChildStoreFactory` registrations are entirely unseeded** — this wave lands the trait +
   registry (`register_child_store_factory`/`child_store_factory`,
   `🏪️store/🦀️component.rs:337-345`) but registers nothing real; every plugin that wants its
   artifacts to be genesis-creatable as children needs a real `ChildStoreFactory` impl registered at
   program-init time, mirroring `register_document_codec_for_app`'s existing per-app wrapper
   pattern. Test fixture `DemoChildFactory` (test-only,
   `🏪️store/🦀️component.rs`'s `🔖️CompositionTests` region) is the minimal shape to copy.
5. **`GroupReceipt.created_children`** — genesis-created live `Box<dyn SpaceMember>`s are returned
   here and must be registered into whatever host (typically a `SpaceHost`) the caller maintains;
   `dispatch_group` itself never registers them anywhere (it has no notion of `SpaceHost`).
6. **`CompositionGraph` is not automatically kept in sync** — a host must call
   `CompositionCoordinator::graph_mut().sync_member(artifact_id, &snapshot)` after any dispatch that
   might have changed an artifact's own `ArtifactRefs::child_refs()`/`links()` (a real
   `#[derive(DslArtifact)]` snapshot won't implement `ArtifactRefs` with real children until a later
   wave adds the `#[child(kind = "...")]` schema facet + derive support the design doc describes
   under "Codecs/facets"). Until then, `dispatch_group`'s ownership check will reject every
   `ChildDispatch` whose parent/child pair was never explicitly seeded via `insert_owns`/
   `sync_member` — this is intentional (fail-closed), not a bug, but worth flagging so the next
   wave doesn't mistake it for one.
7. **`semio-framework-os-flow`** (`🌊️flow/📦️packages/🦀️rust`) appears pre-existing-broken against
   `ArtifactEnvelope`'s current shape (missing `dialect`/`migrated_from`, now also `owner`) — see
   Concurrent-churn observations. Whoever eventually un-wires/fixes that crate will need
   `owner: None` added to its 2 `ArtifactEnvelope { .. }` literals
   (`🧰️framework/🛍️products/💻️os/🦀️component.rs:367,620` and
   `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:370,623`) alongside whatever fixes the
   pre-existing `dialect`/`migrated_from` gap.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — `PhantomData` import; new
  `🔖️Composition` region (`:166-571`); `ArtifactEnvelope.owner` field + 3 construction-site fixes;
  8 new `SpaceMember` trait methods + blanket impl; new `🔖️CompositionCoordinator` region
  (`:4494-5003`); 15 new tests in `mod tests`' new `🔖️CompositionTests` sub-region.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs` — `CompositionPin.child_ref: String →
  ArtifactRef` correction + doc comment + `content_addressed_checkpoint_id` sort/hash update + 5
  test-literal fixes; two new `VcsError` variants (`ValidationFailed`, `CompensationFailed`) +
  updated doc comments on `CompositionCycle`/`OwnershipViolation` (no longer "not yet raised by any
  call site" — both now raised by this wave's `CompositionGraph`/`dispatch_group`).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/scratch-w1b2-check-1.txt`
  through `-3.txt`, `scratch-w1b2-test-1.txt` (cargo check/test output).
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM/📓️wave1-reports/b2-store-composition-report.md`
  (this report).

`📓️status.md` not touched. Ticket left open (not closed).
