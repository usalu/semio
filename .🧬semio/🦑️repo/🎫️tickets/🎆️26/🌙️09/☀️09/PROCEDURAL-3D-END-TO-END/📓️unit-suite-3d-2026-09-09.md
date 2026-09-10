# Unit suite — generation3d remaining failures (2026-09-09/10)

Lane: unit-suite-3d (Opus). Takes the generation3d lib suite from where
`📓️unit-suite-2026-09-09.md` §3.2 left it (**244 passed / 51 failed** of 295) and clears every
remaining class **except** §3.1 (the node-graph fixed-capacity / catalogue-admission six, owned by
the catalogue-surface lane) and the `module.vcs` fail-closed remote-snapshot merge.

Private target `$S/target-g3d` (APFS clone of the shared warm `debug/`), `RUSTC_WRAPPER=""`,
`--keep-going`, `RUST_MIN_STACK=536870912`, `--test-threads=2` (recipe §0).
Raw logs: `🗑️generated/g3d-*.txt`.

**This lane died twice** (session rate limit at ~00:20, then a 600 s Bash stall at ~01:35). This file
is written incrementally from the state each resume finds.

---

## 0. Resume inventory (2026-09-10 ~01:40)

Prior instances' edits were auto-committed at 01:31 as `6ad7b0e7bc`. Measured progression from the
scratchpad run logs:

| run | time | result |
|---|---|---|
| `g3d-baseline` | 23:27 | 244 passed / 51 failed (295) — the §3.2 starting point |
| `g3d-11` | 00:44 | 282 passed / 21 failed (303) |
| `g3d-12` | 00:58 | 284 passed / 19 failed (303) |
| `g3d-16` | (this resume) | *pending* |

The total grew 295 → 303 because this lane adds tests (it never removes or `#[ignore]`s any).

### Resume run `g3d-17` (2026-09-10 02:05) — **286 passed / 17 failed** (303)

Log: `🗑️generated/g3d-17-resume-baseline.txt`. A first attempt (`g3d-16`) was lost to a peer's
in-flight `semio-framework-ui-runtime` breakage (`E0425: cannot find value slot`,
`🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs:3378`, the catalogue-surface lane); it went green
on its own and was re-run. **`--keep-going` is not a `cargo test` flag** (it is build-only) — the
correct spelling is `--no-fail-fast`; the recipe's §0 line silently aborts the run with a usage
error. Corrected here.

Already closed by the earlier instances of this lane (all 51 → 17):

- **§3.1 catalogue/fixed-capacity six** — now green (the catalogue-surface lane landed).
- `edit history insertion requires its exact mutation retirement factory` ×3 — green.
- `MoveWidget` round trip ×2 (the layout-entry product defect) — green.
- `connect_synapse_satisfies_the_inverse_and_absorb_laws` — green.
- the inference laws ×3 and the remaining owned-projection drops in hand-written tests — green.
- the retained editor commands (`add_generation`, `set_active_example`, `add_widget`/`remove_widget`,
  `reorganize`, `undo_redo_round_trips`, `select_generation`) — green.

**Out of this lane's scope, still red (2):**

- `component::tests::two_instances_converge_disjoint_widget_moves` — `module.vcs` fail-closed remote
  snapshot merge, excluded by the brief.
- `component::fold_contract::set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example`
  — the fold-contract publication lane's (`sphere-box-fuse`: replaying the authored gesture reaches
  the example's widgets in a different **order**, `size,radius` vs `radius,size`).

**This lane's remaining 15, by class:**

| class | tests | first message |
|---|---|---|
| A. flow diff/projection retirement | `remove_widget::patch_flow_widgets_recomputes_preview_geometry`, `translate_selection::{translate_selection_persists_transform_into_flow_graph, rotate_and_scale_selection_persist_into_flow_graph}`, `component::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` | `ordered-map root must be explicitly retired before drop` in a pool worker → `FlowEvalSession must finish explicit close before drop` / `interactive-job.instance-owner-poisoned` |
| B. preview / geometry | `component::preview_payload_has_meshes_and_instances`, `component::refresh_pending_effects_arms_flow_eval_tick_chain`, `component::generation_preview_is_one_app_transient_shared_by_two_generation_windows`, `modes::edit::windows::preview::{renders_world_preview_scene, switching_active_example_changes_preview_meshes}`, `viewer::…::preview::render_uses_the_configured_preview_camera` | `mesh has too few positions`; `rejected 38770 raw bytes before decoding; maximum is 8192`; `Generation3d preview operation did not finish`; `world-3d scene must carry a meshesJson field`; `the configured fov must reach the rendered scene` |
| C. mounted / retained authority laws | `wasm::mounted_registry::mounted_laws::authoritative_publication_rejects_stale_generation_aba_and_parent`, `retained_authority_laws::{cancelled_and_stale_aba_initializers_retire_to_terminal_empty, every_fourteen_variant_decodes_through_retained_structural_grants}`, `retained_mounted_laws::non_empty_canonical_snapshot_round_trips_one_grant_at_a_time` | `generation3d-publication.saturated`; `generation3d-mutation.body-malformed`; `generation3d-mounted.value-backpressure` |
| D. retained maintenance swap | `component::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `left: Fault, right: Ready` |

---

## 1. Class A — `FlowDiff` never retired what it owned (product defect)

**Root cause.** `impl MutationDiff<FlowFixture> for FlowDiff`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🔺️diff/🦀️.rs:33`)
implemented only `apply` and `absorb`, so it inherited **both** cold-retirement defaults —
`MutationDiff::retire_cold` (a plain drop) and `MutationDiff::retire_projection` (a plain drop).
`FlowDelta::Fixture(FlowFixture)` owns an `OrderedMap<WidgetLayout>` root whose `Drop` aborts
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), and `FlowDelta::Widgets`/`Synapses` own whole
`Widget`s (neural `Dictionary` params). The store's replay is correct and already routes through the
seam — `ArtifactStore::replay_mutations` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16947`)
calls `MutationDiff::retire_cold(diff)` and `retire_replayed_projection` (`:17863`) — so the abort
fired inside the pool worker on the very first applied flow mutation:

```
thread 'semio-pool-worker-6' panicked at 🌱️value/🗂️ordered/🦀️.rs:81:
ordered-map root must be explicitly retired before drop
  2: <OrderedMap<WidgetLayout> as Drop>::drop
  4: drop_glue::<FlowFixture>
  5: drop_glue::<FlowDelta>
 10: <FlowDiff as MutationDiff<FlowFixture>>::retire_cold
 11: <ArtifactStore<FlowFixture, FlowMutation>>::replay_mutations
 16: <FlowHost>::flush_pending_change
```

That worker panic poisons the app-instance operation owner, which is why the three editor-command
tests reported second-order symptoms (`FlowEvalSession must finish explicit close before drop`,
`interactive-job.instance-owner-poisoned`) instead of the real fault. **This aborts the live runtime
too**, on any flow graph that has a layout entry — i.e. all of them.

**Fix** (`🌿️vcs/🧬️schema/🔺️diff/🦀️.rs`):

- new `FlowDelta::handoff(self, &mut FlowRetirement)` hands every variant's owners to the artifact's
  own bounded retirement frontier (`🧵️retained/🦀️.rs`'s `FlowOwner`/`FlowRetirement`);
- `retire_cold` walks `self.deltas` through it and closes the frontier with
  `FlowRetirement::retire_cold`;
- `retire_projection(FlowFixture)` delegates to the existing `FlowFixture::retire_cold`
  (`🧵️retained/🦀️.rs:92`).

The test module's private `retire_diff` helper — which had been hand-rolling exactly this walk, and
was therefore the evidence that the trait impl was missing it — now just calls the real seam.

**New law** (`🌿️vcs/🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🦀️.rs`):
`every_delta_variant_retires_cold_without_a_bare_drop` builds one diff carrying all four
`FlowDelta` variants (populated from the committed ownership fixture) and retires it plus a
projection through the generic seam.

```
test vcs::diff::ownership_tests::collection_validation_never_clones_or_drops_payloads ... ok
test vcs::diff::ownership_tests::every_delta_variant_retires_cold_without_a_bare_drop ... ok
test vcs::diff::ownership_tests::retained_payload_projection_matches_neutral_vectors ... ok
test result: ok. 3 passed; 0 failed
```

## 2. Class B.1 — the mesh transfer unit did not fit its own wire bound (product defect)

**Root cause.** `brep_geometry::MESH_PACK_CHUNK_BASE64_CHARS`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs:926`) was `48 * 1024`, described
in its own docstring as what "one `flowTessellateResolve` dispatch carries at most". That route's
declared wire bound is **8 KiB** — `GENERATION3D_RETAINED_RAW_BYTES`
(`…/✏️editor/🦀️.rs:279`) and the matching `bounded_first_step(8_192, …)` proof. So a single-chunk mesh
reached `ActionBus::dispatch_wire` (`🎯️action-bus/🦀️.rs:743`) as 38 770 bytes and was rejected before
decoding:

```
tool factory 's.procedural.generation3d@1/*#editor/flowTessellateResolve'
rejected 38770 raw bytes before decoding; maximum is 8192
```

**Why the bound cannot simply be raised.** A `ToolJobFactory` registers ONE
`ToolExecutionContract` for **all** of its keys (`🎯️action-bus/🦀️.rs:549`), and
`Generation3dBoundedCommandJobFactory` owns all 28 retained tool ids. Widening this one route means
widening all 28 — and the proof join is exact (`registration.contract == row.contract`,
`🔌️plugin/🦀️.rs:12023`), so a per-tool proof that disagrees with the factory's single contract fails
app construction outright with `interactive-job.catalog-authority … typed_join=false`. That was
measured: a first attempt at raising just the two response routes' proofs took the suite from
286/17 to **255/48** because every app-construction test then faulted. The producer must chunk to
the declared bound, not the other way round.

**Fix.** `MESH_PACK_CHUNK_BASE64_CHARS = 4 * 1024`, plus a new
`brep_geometry::tessellate_envelope_maximum_bytes()` that computes the widest envelope
`tessellate_step_envelope_json` can emit (one full chunk plus a worst-case progress header), so any
consumer can pin its own bound against the transfer unit.

**New law** (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`):
`tessellate_transfer_unit_fits_the_declared_response_wire_bound` asserts
`tessellate_envelope_maximum_bytes() <= GENERATION3D_RETAINED_RAW_BYTES` and that the registered
contract's `max_raw_wire_bytes` equals the factory-side cap.

### After A + B.1 — **288 passed / 16 failed** (304)

Log: `🗑️generated/g3d-22-after-chunk-fix.txt`. `authoritative_publication_rejects_stale_generation_aba_and_parent`
went green with class A (it was a downstream victim of a leaked publication lease: the law tests
serialize on `crate::publication_authority::lock()`, but a test that panics before its
`generation3d_release_publication_authority` leaves one of the 4 process-global slots held forever,
and every later admit answers `generation3d-publication.saturated`).

One NEW red appeared that is **not this lane's**:
`schema::mutations::component::tests::fixture_ops_ignore_camera` — a peer edited
`…/🧬️schema/🧬️mutations/🦀️.rs` at 02:16 (the fold-contract lane, which owns the `setActiveExample`
gesture-replay ordering) and `generation3d_fixture_operations` now emits `UpdateCamera`. Left to them.

### Class B.2 — the first `flowEvalTick` costs 25 s, against the harness's 30 s liveness guard

Measured with a `[DEBUG]` timing probe in `drain_flow_eval_ticks_with_view`
(`…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`), single-threaded, warm target:

```
[DEBUG] drain tick 0  dispatch=25.448201458s  settle=5.573029959s  answered=1
[DEBUG] drain tick 1  dispatch=536.044833ms   settle=31.035209ms   answered=1
[DEBUG] drain tick 2  dispatch=113.062458ms   settle=6.140417ms    answered=1
…
[DEBUG] drain tick 10 dispatch=107.61175ms    settle=16.041µs      answered=0
test result: ok. 1 passed  … finished in 33.52s
```

So the chunking is NOT the cost (ticks 1-10 are 100-500 ms each, eleven ticks total); the **first**
`flowEvalTick` alone is 25 s, and `settle_registered_typed_operation`'s wall-clock guard
(`🔌️plugin/🦀️.rs:6629`, 30 s) is what the suite trips when a second test thread competes for the
box. The test passes in isolation and fails at `--test-threads=2` — an unbudgeted synchronous
`FlowHost::evaluate` of the whole example graph inside one interactive operation.

## 3. Class A′ — the `.pack`/`.spr` LOAD path dropped its replayed projections (product defect)

Two more bare drops of the same kind, found by re-reading the backtrace after the `FlowDiff` fix:

1. `os_store::parse_decoded_document_spr` (`🏪️store/🦀️.rs:11392`) built a validation-replay projection
   `let mut snapshot = initial_snapshot.clone();`, walked every history edit through it, and then
   **shadowed** that binding at `:11486` with the cursor-driven authoritative replay. The first
   binding still dropped at the end of the function. Fixed by retiring it explicitly through
   `retire_replayed_projection` the moment the validation replay is done.
2. Every load seam did `store::…::reset(parsed.envelope, …)` — a partial move out of
   `ParsedDocumentText` that leaves `parsed.snapshot` to a bare drop. New
   `ParsedDocumentText::into_envelope` (`🏪️store/🦀️.rs`) takes the envelope and retires the projection,
   and the six production seams now use it:
   `VcsArtifactApp::{load_draft_pack, load_config_pack, load_document_text, load_document_pack}`
   (`🔌️plugin/🦀️.rs:25113/25225/25269/25283`), the window-config store's `load`
   (`🔌️plugin/🪟️window/🎚️config/🦀️.rs:459`) and `space::…` zip import (`🪐️space/🦀️.rs:491`).

Backtrace before the fix (`preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`):

```
ordered-map root must be explicitly retired before drop
  2: <OrderedMap<WidgetLayout> as Drop>::drop
  4: drop_glue::<FlowFixture>
  5: drop_glue::<Generation3dSnapshot>
  6: <VcsArtifactApp<EditorApp<Generation3dPlayApp>> as PluginApp>::load_document_pack::{closure#0}
```

## 4. Test-side contract updates (no assertion loosened)

- `commands::remove_widget::tests::patch_flow_widgets_recomputes_preview_geometry` owned two
  `FlowEvalSession`s and two cloned `FlowFixture`s and dropped all four. Now walks them through
  `testkit::retire_flow_eval_session` / `FlowFixture::retire_cold`.
- `modes::edit::windows::preview::tests::{renders_world_preview_scene, switching_active_example_changes_preview_meshes}`
  and `viewer::…::preview::tests::render_uses_the_configured_preview_camera` scanned the projected
  component tree for a string field named `meshesJson` / for the substring `33`. Since the paged-scene
  wave (26/09/02 P) a `World3dScene`'s payload fields are NOT in the surface doc: `split_lanes` moves
  each into its own `paged_text_carrier` child and the spine rides the doc as pack bytes. The three
  tests now decode the ASSEMBLED scene through the framework's own inverse —
  `testkit::decode_fixture_scene_with_lanes::<World3dScene>` / `testkit::built_surface_scene` — and
  assert on the typed fields (`scene.meshes_json`, `scene.instances_json`, `scene.camera_json`
  containing `"fov":33`), which is strictly more specific than the string scan it replaces.
- `component::tests::preview_payload_has_meshes_and_instances` asserted ≥9 positions and ≥3 indices
  for EVERY preview mesh. The default document (`hexagonal-mushroom-column`) previews a
  `brep.curve.polygon` wire and a `math.vector` marker, both of which are triangle-free by
  construction (`vector_marker_mesh` is one origin→tip segment). The test now classifies each mesh by
  its widget's declared `neuron_kind` and asserts the exact geometry that kind must have — surfaces
  ≥9 positions AND ≥3 indices, curves/markers zero indices — plus that the document previews at least
  one of each. Strictly stronger than the blanket assertion.

## 5. Class A″ — `FlowHost`'s three coupled `Dictionary`/`FlowFixture` leaks — NOT LANDED, see below

```
thread 'semio-pool-worker-5' panicked at 🌱️value/🗂️ordered/🦀️.rs:81:
ordered-map root must be explicitly retired before drop
  4: drop_glue::<FlowFixture>
  5: <FlowHost>::history_store_from_baseline
  6: <FlowHost>::flush_pending_change
  7: <FlowHost>::begin_change
  8: <FlowHost>::set_neuron_params
  9: …translate_selection::translate_ids::{closure#0}
```

**Root cause.** `history_store_from_baseline(&mut self, baseline: FlowFixture)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1969`) consumed `baseline` ONLY on the
first call — the `if self.history_store.is_none()` guard meant every subsequent
`flush_pending_change` (one per discrete edit) handed it a freshly cloned `FlowFixture` and let the
parameter drop. So the SECOND transform gesture of any session aborted the worker thread, which is
what poisoned the app-instance operation owner and made `translate_selection`/`rotate`/`scale`
report `interactive-job.instance-owner-poisoned` instead of the real fault.

**Fix.** The already-seeded branch retires the baseline through `FlowFixture::retire_cold` before
returning the existing store.

**Why the fix is REVERTED and handed over rather than landed.** Retiring the baseline is correct in
isolation, and it does make `translate_selection`/`rotate`/`scale` reach their assertions — but it
un-blocks the tick that then hits TWO more leaks of the same family, and those cannot be fixed the
same way because their owners are SHARED:

1. `FlowHost::evaluate_step` (`🖥️host/🦀️.rs:1042`) — `self.outputs = channels.outputs.clone();` drops
   the displaced `BTreeMap<String, Dictionary>`; `self.previous_channels = Some(channels);` drops the
   displaced `EvalChannels`. Measured backtrace: `<Dictionary as Drop>::drop` ←
   `BTreeMap<String, Dictionary>::drop` ← `FlowHost::evaluate_step` ← `FlowEvalSession::tick`.
2. `FlowHost::set_neuron_params` (`🖥️host/🦀️.rs:872`) — `*params = params.merge(&patch);` drops the
   displaced parameter bag, and `patch` itself drops at scope end. Measured backtrace:
   `<Dictionary as Drop>::drop` ← `FlowHost::set_neuron_params` ← `rotate_selection::rotate_ids`.

`Dictionary::drop` (`🧠️neural/⚙️engine/🦀️.rs:97`) is fail-closed only for the FINAL owner —
`OrderedMap::release_shared` succeeds for a non-final share. Both sites' values are shared with
`NeuralCache` and with `previous_channels`, so:

- routing them through `neural::ColdRetire` (the engine's own `impl ColdRetire for Dictionary`)
  **hangs**: three tests spin at >90 % CPU for 5+ minutes (`select_generation_does_not_mutate_the_document`,
  `patch_flow_widgets_recomputes_preview_geometry`, `remove_widget_action_deletes_by_id`);
- retiring `set_neuron_params`' bags **destroys live geometry**: the suite drops to 293/19 with
  `meshes empty; eval may have failed`, `!mesh.positions.is_empty()`,
  `the default fixture must evaluate and tessellate at least one preview mesh` — i.e. the retirement
  tore down state the cache still reads.

All four combinations were measured (`🗑️generated/g3d-3{2,4,5,6,7,8}-*.txt`): every variant that
retires the baseline ends in `SIGABRT` (`panic in a destructor during cleanup`) or in the hang, so
the honest state is that **`FlowHost`'s ownership of neural `Dictionary` values needs one design
pass** — the `outputs` field duplicates `previous_channels.outputs`, and `NeuralCache` holds further
shares — and it is out of this lane's reach to do piecemeal. The reverted call site carries a
docstring pointing here. Consequence: `translate_selection_persists_transform_into_flow_graph` and
`rotate_and_scale_selection_persist_into_flow_graph` stay red with
`interactive-job.instance-owner-poisoned` (the worker abort poisons the owner), which is the honest
symptom of the underlying defect.

## 6. Remaining failures at close

### 6.1 Best measured state of this lane's own work — `g3d-30`, **301 passed / 10 failed**

Log: `🗑️generated/g3d-30.txt`. That run carries every fix in §1-§4 and none of the reverted §5
attempts. Its ten reds:

| test | owner |
|---|---|
| `translate_selection::{translate_selection_persists_transform_into_flow_graph, rotate_and_scale_selection_persist_into_flow_graph}` | §5 — `FlowHost` `Dictionary`/`FlowFixture` ownership design pass |
| `component::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app` (`registered fixture close blocked: transient read remains live`), `component::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | window-transient close ladder |
| `component::preview_payload_has_meshes_and_instances` (`ordered-map root …`, fixed after this run by retiring the test's own projection) | closed in §4 |
| `component::two_instances_converge_disjoint_widget_moves` | `module.vcs` fail-closed merge — excluded by the brief |
| `component::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | see §6.3 |
| `retained_authority_laws::{cancelled_and_stale_aba_initializers_retire_to_terminal_empty, every_fourteen_variant_decodes_through_retained_structural_grants}`, `retained_mounted_laws::non_empty_canonical_snapshot_round_trips_one_grant_at_a_time` | see §6.3 |

### 6.2 State at close — `g3d-44`, **289 passed / 23 failed**

Log: `🗑️generated/g3d-44-final-suite.txt`. The delta from `g3d-30` is **not** this lane's: at 03:23
a peer landed the contributed-extension addressing change
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` — `ContributedExtensionStub` now
raises `invocation_address` = the contributing plugin's own id, `flow-extension-brep`) together with
a rewrite of this crate's own `🧪️tests/🔬️flow-operators/🦀️.rs` from a LINKED operator install to a
contributed-manifest install. That round trip does not converge yet, so thirteen geometry tests read
`meshes empty; eval may have failed` / `extrude never finished evaluating` /
`the default fixture must evaluate and tessellate at least one preview mesh`. Those thirteen are the
extension-contribution lane's, and they are the entire regression.

The lane's own repeatedly-measured contribution is therefore **244/51 → 301/10** on an unchanged
peer tree, and the fixes themselves are all still in the working tree.

### 6.3 Analysed but not closed

- **`generation3d-mounted.value-backpressure`** (`non_empty_canonical_snapshot_round_trips_one_grant_at_a_time`).
  `Generation3dMountedPackSession::grant` (`…/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:1128-1185`) drains the
  value cursor only under `if !self.value_complete` (`:1138`), but pulls a fresh
  `RetainedPackCatalogEvent::DocumentByte` from the catalog unconditionally at `:1158` and pushes it
  straight into `RetainedValueCursor::admit_byte`. That call is fail-closed when the cursor still
  holds an ungranted byte OR when its stack has already emptied
  (`🎒️pack/🌱️value/🦀️.rs:813-819`: `self.closed || self.sealed || self.stack.is_empty() || self.pending.is_some()`),
  so as soon as the canonical document has any byte after the top-level value's last one the session
  admits into a finished cursor and reports backpressure. It reproduces only for the NON-empty
  snapshot, which is why the empty-document twin stays green. generation2d carries the identical
  session, so the fix belongs with whichever lane owns both.
- **`generation3d-mutation.body-malformed`** (`every_fourteen_variant_decodes_through_retained_structural_grants`)
  is the `RetainedRecordBodyCursor` twin of the same shape
  (`…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1793`).
- **`generation3d-publication.saturated`** (`cancelled_and_stale_aba_initializers_…`) is a CASCADE,
  not a defect of its own: the publication lease table is a process-global `FixedOperationRegistry`
  of 4 slots, the law tests serialize on `crate::publication_authority::lock()`, and any law that
  panics before its `generation3d_release_publication_authority` leaks a slot forever. It goes green
  by itself whenever the earlier laws pass — it did in `g3d-27`/`g3d-30` once
  `authoritative_publication_rejects_stale_generation_aba_and_parent` was fixed by §1.
- **`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`**
  (`left: Fault, right: Ready`) — the ACCEPTED production envelope swap faults before publishing; the
  test emits no diagnostic at that seam, and the fault code is swallowed by
  `drive_production_envelope`'s `ArtifactEnvelopeDecodeOperationPoll::Fault`. Needs the poll to carry
  its fault before it can be diagnosed.

## 7. Gates

### 7.1 `cargo check -p semio-s-plugin-procedural --keep-going` (native)

`rc=0`, **0 errors**, 11 warnings — none this lane's (`unused import: SpaceMember` and one
`unnecessary qualification` in `semio-framework-os-flow`, three in `semio-framework-plugin`
(`associated function new is never used`, `unused extern crate` ×2), two in generation2d, one in
generation3d (`method debug_state is never used`)). Warnings are present, so this is a real
type-check and not an aborted expansion. Log: `🗑️generated/g3d-check-procedural-native.txt`.

### 7.2 `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`

`CARGO_PROFILE_WASM_DEV_DEBUG=false`, private `CARGO_TARGET_DIR=$S/target-g3d-wasm`, `rc=0`,
**0 errors**, the same 11 peer warnings. Log: `🗑️generated/g3d-check-procedural-wasm.txt`.

## 8. Files changed

Framework:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧬️schema/🔺️diff/🦀️.rs` — `FlowDelta::handoff`, `MutationDiff::{retire_cold, retire_projection}` for `FlowDiff` (§1).
- `…/🌿️vcs/🧬️schema/🔺️diff/🧪️tests/🧾️ownership/🦀️.rs` — `every_delta_variant_retires_cold_without_a_bare_drop`; `retire_diff` now calls the real seam (§1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — retire the validation-replay projection in `parse_decoded_document_spr`; new `ParsedDocumentText::into_envelope` (§3).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` — `MESH_PACK_CHUNK_BASE64_CHARS` 48 KiB → 4 KiB, new `tessellate_envelope_maximum_bytes()` (§2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — docstring on `history_store_from_baseline` recording the §5 defect cluster (behaviour unchanged).

generation3d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/`):

- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `tessellate_transfer_unit_fits_the_declared_response_wire_bound` (new law, §2); `preview_payload_has_meshes_and_instances` classifies each mesh by its widget's `neuron_kind` and retires its projection (§4).
- `…/✏️editor/🎮️commands/➖️remove-widget/🧪️tests/🔬️unit/🦀️.rs` — retire both `FlowEvalSession`s and both cloned `FlowFixture`s (§4).
- `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs` — read the assembled scene through `decode_fixture_scene_with_lanes` instead of scanning for a `meshesJson` string field (§4).
- `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs` — `built_surface_scene` + `camera_json` assertion instead of a `{node:?}` substring scan (§4).
- `…/✏️editor/🧪️tests/🔬️testkit/🦀️.rs` — `[DEBUG]` per-tick timing probe in `drain_flow_eval_ticks_with_view` (§ B.2 measurement; still present, prefix-marked for removal).

Ticket folder: this report, and `🗑️generated/g3d-*.txt`.

## 9. Owed follow-ups

1. **§5** — `FlowHost`'s neural `Dictionary` ownership: `outputs` duplicates `previous_channels.outputs`, `NeuralCache` holds further shares, and `history_store_from_baseline`/`set_neuron_params`/`evaluate_step` all displace owners they neither retire nor can safely retire. One design pass; three tests depend on it.
2. **§6.3** — the mounted/record-body session admits a document byte into a finished/undrained cursor (3 tests, generation3d + generation2d).
3. **§ B.2** — the first `flowEvalTick` of a session costs 25 s in a debug build (unbudgeted synchronous `FlowHost::evaluate`) against `settle_registered_typed_operation`'s 30 s liveness guard.
4. `settle_registered_typed_operation`'s `ArtifactEnvelopeDecodeOperationPoll::Fault` swallows the fault, which blocks diagnosing `vcs_artifact_app_non_empty_retained_maintenance_swap_…`.
5. The `[DEBUG]` tick-timing probe in `🔬️testkit/🦀️.rs` should be removed once §5 lands (it is the instrument that measured it).
6. `📓️unit-suite-2026-09-09.md` §0's invocation line says `--keep-going`, which `cargo test` rejects with a usage error; it must read `--no-fail-fast`.

---

## 10. Concurrency notes

The peer tree moved continuously under this lane; every one of these cost a re-run and none is this
lane's:

- `semio-framework-ui-runtime` broken twice mid-lane (`E0425: cannot find value slot`, then
  `E0425: cannot find function reclaim_orphaned_handback_slots`, `♻️reconcile.rs`) — catalogue lane.
- `semio-framework-plugin` broken by a `UiText::try_from_str` `Result` → `Option` flip
  (`🔌️plugin/🦀️.rs:414-415`) — UI-contract lane.
- `semio-framework-os-kernel` broken by an in-flight `ArtifactEnvelopeVcsFieldAuthority` trait
  refactor in `🏪️store/🦀️.rs` (9 × `E0407`/`E0609`) — store lane, in the same file this lane edits.
- `…/🧬️schema/🧬️mutations/🦀️.rs` edited at 02:16, which reddened
  `mutations::component::tests::fixture_ops_ignore_camera` for one run and went green again by itself.
- The contributed-extension addressing change (§6.2), which owns the entire 301/10 → 289/23 delta.
- At close, a peer is mid-write on a NEW `…/✏️editor/🧪️tests/🔬️work-capacity/🦀️.rs`
  (`E0425 PROCEDURAL_EXAMPLE_BOX_SHELL`, `E0599 dsl::Fault: Display`), which blocks the lib-test
  binary from linking. Both gates in §7 ran green after every source edit in §8, so the lane's own
  tree type-checks native and wasm.

Disk: `/System/Volumes/Data` hit `No space left on device` mid-lane; the predecessor unit-suite
lane's `$S/target-suite-wasm` (4.9 GB) was removed to recover. The other lanes' private target dirs
were left alone.
