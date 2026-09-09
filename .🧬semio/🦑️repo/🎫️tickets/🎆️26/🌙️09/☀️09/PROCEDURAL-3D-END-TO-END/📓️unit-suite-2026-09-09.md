# Unit suite — generation3d + generation2d lib tests (2026-09-09)

Lane: unit-suite (Opus). Takes the generation3d lib suite from the runnable-but-red state
`📓️runtime-defects-2026-09-09.md` left it in (127 passed / 168 failed of 295) and clears the
mechanical classes; does the same for generation2d, whose suite could not even run to completion.

Private target dirs `$S/target-suite` (APFS clone of the runtime-defects lane's warm `debug/`) and
`$S/target-suite-wasm`, `RUSTC_WRAPPER=""`, `--keep-going`. Raw logs in `🗑️generated/suite-*.txt`.

---

## 0. How the suite must be invoked

```
RUST_MIN_STACK=536870912 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- --test-threads=2
```

`--test-threads=1` runs every test on libtest's **main** thread, whose 8 MiB macOS stack the
generation3d dispatch path overruns as soon as the whole binary is loaded:

```
test editor::generation3d::commands::add_generation::tests::add_generation_records_an_undoable_generation_operation ...
thread '…' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

That is a **hard abort**, so the run reports nothing at all — this is how the suite presented at the
start of this lane, and it is NOT the `interactionSelect` overflow the selection lane owns (the same
test passes on the main thread when it is the only one filtered in). `--test-threads=2` (or more)
puts each test on a spawned thread whose stack `RUST_MIN_STACK` sizes, and the suite runs to
completion. The repo's own runner already exports `RUST_MIN_STACK` (`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:8`,
8 MiB) — **that value is too small for this crate** and is the next thing to raise if the suite is
wired into `nx`.

## 1. Before → after

| suite | command | before | after |
|---|---|---|---|
| generation3d | `--features component-app-assembly --lib` | **127 passed / 168 failed** (295) | **244 passed / 51 failed** (295) |
| generation3d | `--lib` (default features) | not previously measured | **138 passed / 13 failed** (151) |
| generation2d | `--features component-app-assembly --lib` | **SIGABRT** — 29 ok / 34 failed of 225 before a non-unwinding abort | **179 passed / 46 failed** (225) |

generation2d's "before" is not a pass/fail count: the binary aborted at
`retained_authority_laws::every_fourteen_variant_decodes_through_retained_structural_grants` with
`panic in a destructor during cleanup` and discarded the rest of the run (§2.1).

## 2. Failure classes

### 2.1 generation2d aborted the whole binary — unguarded `Drop` asserts — 1 abort (all 225 tests)

**Root cause.** `Generation2dMutationSession`'s destructor
(`🌀️generation2d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1837`) and 24 sibling fail-closed `Drop`
asserts across generation2d and generation3d had no `std::thread::panicking()` guard, so any test
that panicked while holding one hit a **second** panic in a destructor during cleanup — a
non-unwinding abort that discards the failure report and every later test. The runtime-defects lane
had already guarded the framework's 44 store asserts and generation3d's mutation session; these 25
were the remainder.

**Fix.** All 25 now read `assert!(std::thread::panicking() || (<witness>), "…")`, exactly the guard
`OrderedMap::drop` and the framework's retirement roots carry. Fail-closed behaviour on a genuine
live drop is unchanged.

- `🌀️generation2d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (10), `…/📸️snapshot/💾️binary/🦀️.rs` (2), `…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` (1)
- `🧊️generation3d/…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (9), `…/📸️snapshot/💾️binary/🦀️.rs` (2), `…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` (1)

### 2.2 Committed mutation fixtures were stale — 98 failures (3d), the whole per-mutation lane

The 14 × 7 generated fixture tests (`🧬️mutations/*/🧪️tests/*/🦀️.rs`) failed in three distinct ways.
None of them was the test's fault.

| sub-class | count | root cause | fix |
|---|---|---|---|
| `mutation decodes: ValueError("missing tag field \`mutation\`")` | 56 | `Generation3dMutation` declares `#[value(tag = "mutation", rename_all = "camelCase")]` (`🧊️generation3d/…/🧬️schema/🧬️mutations/🦀️.rs:158`) — internally tagged, the repo-wide canonical shape (`🧩️puzzle/…/🖐️5d/…/🦠️mutation/🔣️.json` is `{"mutation":"rotatePart3d", …}`). Every committed `🦠️mutation/🔣️.json` was still the pre-`dsl::Mutations` **externally** tagged `{"UpdateWidget":{…}}`. | 14 fixtures retagged onto the declared contract (`🐍️fixture-canonicalize.py`). |
| `committed … JSON is not canonical` | 28 | `dsl::json::to_json_string` writes an `f64` `0.0` as `0.0` (serde_json-identical, `🎒️pack/🔤️json/🦀️.rs` `🔖️FloatFormat`); the committed before/after/diff JSON carried hand-written integer literals (`"zoom": 1`, `"min": 0`), so decode→encode was not a fixed point. | 79 fixture files re-emitted from the artifact's **own** encoder (decode → re-encode → 2-space pretty print). Only the ENCODING is normalised — every value is preserved exactly, so the committed expectations stay the contract. |
| `ordered-map root must be explicitly retired before drop` | 14 | `before()`/`expected_after()`/the decoded diff hand back owned projections the template dropped. | See §2.3. |

**generation2d is NOT the same contract.** `Generation2dMutation`
(`🌀️generation2d/…/🧬️schema/🧬️mutations/🦀️.rs:39-41`) carries **no** `#[value(tag = …)]` — it is
externally tagged, and its committed fixtures were already correct. The retag was reverted there and
only the float-encoding canonicalisation applied (39 files). *This is a real cross-artifact
inconsistency and a follow-up: the two sibling artifacts encode the same mutation vocabulary in two
different wire shapes.*

### 2.3 The per-mutation fixture test template dropped every owned projection — 14 + the tail of 2.2

**Root cause.** The generated 7-function template (`before()`, `expected_after()`,
`committed_diff_is_canonical`, `committed_diff_applies_to_after`, …) returns owned
`Generation{N}dSnapshot` / `Generation{N}dDiff` values and drops them. Both reject a bare drop —
`fixture.layout` is an `OrderedMap<WidgetLayout>` whose root must be retired
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) and `generation` owns its own ladder.

**Fix — the template, not 28 hand edits.** There is **no live `fixtures generate` command**: the only
thing in the repo that says those words is a lint *message*
(`✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:214,235,288`, `fixtures lint` only), so the
28 on-disk files ARE the artifact. All 28 were re-emitted from one canonical template
(`🐍️fixture-tests-template.py`, kept in this ticket folder) so they stay byte-consistent with each
other; each file keeps its own docstring and `include_str!` quintet and gets an identical body that

- wraps every decoded projection in `Generation{N}dSnapshotRead` and every decoded delta in the new
  `Generation{N}dDiffRead`, both of which retire on drop;
- routes the raised `MutationOutcome` through `into_parts()` into a `Generation{N}dDiffRead` (the
  outcome OWNS its diff — `📡️replication/🎮️mutation/🦀️.rs:1073`, so dropping it leaks too);
- encodes diagnostic severities through the artifact's own encoder rather than `format!("{:?}")`
  (generation2d's template already did; generation3d's did not — they now agree).

New supporting API:

- `Generation3dSnapshotRead`: `DerefMut` added (`🧊️generation3d/…/🧬️schema/📸️snapshot/🦀️.rs`).
- `Generation2dSnapshot::retire_cold` + `Generation2dSnapshotRead` — the 2d twins, which did not exist.
- `Generation{2,3}dDiff::retire_cold` + `Generation{2,3}dDiffRead` (`…/🧬️schema/🔺️diff/🦀️.rs`).

### 2.4 `MutationDiff::apply` and `apply_generation{N}d_mutation` LEAKED on every applied mutation — product defect

**Root cause (product, not test).** `🧬️schema/🔺️diff/📝️text/🦀️.rs:124-130`:

```rust
let mut next = snapshot.clone();
if let Some(fixture) = &self.fixture { next.fixture = fixture.clone(); }   // ← drops the cloned base fixture
if let Some(generation) = &self.generation { next.generation = generation.clone(); }
```

The plain assignment drops the displaced `FlowFixture` (non-empty `OrderedMap` root) and
`GenerationPlayRoot`. `🧬️schema/🧬️mutations/🦀️.rs`'s `apply_generation{N}d_mutation` did the same with
`*projection = next;`, and additionally routed through `vcs::apply_mutation`
(`🌿️vcs/🦀️.rs:1141-1142`), which builds a diff and **drops it** at the end of the expression.
Every applied mutation therefore aborts the moment a layout entry exists — in the live runtime as
well as in tests.

**Fix.** Both sides now retire what they displace:
`std::mem::replace(&mut next.fixture, fixture.clone()).retire_cold()`, and
`apply_generation{N}d_mutation` builds the diff, applies it, retires the diff, then retires the
displaced projection instead of delegating to the generic `vcs::apply_mutation`. `absorb` was
rewritten the same way (it did `*self = other` and moved fields out of a consumed `Self`, leaking on
both paths). Applies to generation3d and generation2d.

### 2.5 Every viewer action was undispatchable — 8 failures (3d)

Three stacked defects, each hiding the next:

1. **`interactive-job.missing-factory`, verb `'typed-command'`.** `Generation3dViewer` never
   overrode `ArtifactViewer::command_id`, whose default is the generic `"typed-command"`
   (`🔌️plugin/🦀️.rs:26479`, and `:10785` for the app twin). `qualified_tool_proof`
   (`🔌️plugin/🦀️.rs:18192-18205`) looks the bounded first-step proof up **by that verb**, so all
   seven declared tools were unreachable. Fixed by adding the override
   (`🧊️generation3d/…/👁️viewer/🦀️.rs`), exactly the way the editor has it (`✏️editor/🦀️.rs:1210`).
   generation2d's viewer declares no actions at all and needs none.
2. **`interactive-job.publication-authority-missing: 'setCamera' declares the unsupported config
   publication lane`.** Every viewer tool publishes on the **config** lane, but
   `build_config_store_one_item_preparation_factory` was `None`, which
   `🔌️plugin/🦀️.rs:18347-18371` turns into a blanket refusal. The framework offered
   `bounded_config_store_owners` (retirement) but **no publication twin** — so no viewer in the repo
   could ever own a config lane. Added
   `semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<C, M>(prefix, maximum_bytes)`
   (`🔌️plugin/🦀️.rs`, beside `bounded_config_store_owners`) and declared it on the 3d viewer with
   `GENERATION3D_VIEW_CONFIG_STORE_MAXIMUM_BYTES = GENERATION3D_VIEW_RAW_BYTES` (8 KiB).
3. **`Generation3d viewer fixture did not reach its terminal-empty close witness`.** With dispatch
   working, `PluginApp::close_step` answered
   `interactive-job.close-owned-disposer-missing: app owner did not provide the required bounded
   disposer for document-store` — the viewer declared config/presence/transient disposers but not
   the **document** one, so no viewer fixture could ever close. Added
   `build_document_store_disposer` (`👁️viewer/🦀️.rs`), the same
   `ArtifactDocumentStoreDisposer` the editor declares.

### 2.6 Framework testkit round-trip helpers dropped what they decoded — 6 failures (3d), 4 (2d)

`store::test_support::assert_dsl_round_trip` / `assert_dsl_pack_equivalence` /
`assert_pack_round_trip` (`🏪️store/🦀️.rs:20813+`) decode a `P` and drop it. Fine for the ~50
artifacts whose projections are droppable; fatal for one that owns an `OrderedMap` root. 206 call
sites repo-wide make a signature change untenable, so **cold twins** were added beside them —
`assert_dsl_round_trip_cold`, `assert_dsl_pack_equivalence_cold`, `assert_pack_round_trip_cold`,
each taking a `retire` closure and comparing before handing the value over — and both procedural
artifacts' snapshot round-trip tests routed onto them.

### 2.7 The lib suite never installed the geometry kernels — 14 failures (3d)

**Root cause.** `FlowHost::evaluate` answered `unknown kind: brep.curve.polygon` /
`unknown kind: math.vector` for **every** node of every bundled fixture, so the preview payload came
back `[]` and every geometry assertion failed on a registry gap rather than on geometry. The
packaged `brep`/`math` operator sets are `[dev-dependencies]` and are installed only by the separate
`[[test]] example-geometry` binary (`📚️examples/🧪️tests/🧩️geometry/🦀️.rs:103-114`); the `--lib`
binary installed nothing.

**Fix.** New `🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs` performs the identical one-time
`install_flow_extension`, mounted at the crate root as `#[cfg(test)] pub(crate) mod flow_operators`
and invoked from both serial locks (`✏️editor/🧪️tests/🔬️test-support/🦀️.rs`,
`👁️viewer/🧪️tests/🔬️testkit/🦀️.rs`). The registry is process-global, so this has to be global —
a per-test opt-in would make the suite order-dependent.

**It exposes a real production defect — see §3.1.** Five surface-render tests that previously passed
against an *empty* operator registry now fail, because the rendered node-graph surface is 111 KB
against a 32 KiB fixed capacity as soon as real operators exist. That is the live configuration.

### 2.8 Stale command tables — 3 failures (3d)

`every_command()` covered 27 of the 28 declared `Generation3dCommand` rows (missing
`CancelPreviewEval`) and three assertions pinned the count at a literal **29**, which was never the
row count. `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: `CancelPreviewEval` added to `every_command()` and to
`expected_keywords` (`"cancel-preview-eval"`), the coverage assertion now reads
`GENERATION3D_RETAINED_TOOL_IDS.len()` instead of a literal, and the three `29`s became `28`.

### 2.9 `pack_round_trips` asserted a magic neither artifact emits — 1 failure each

3d asserted `bytes.starts_with(b"P3D3")` — but `P3D3` is `GENERATION3D_MOUNTED_PREFIX`, the
**mounted streaming** prefix, while `encode` is `ArtifactPack::encode_pack`, whose container leads
with `\x89SEM\r\n\x1a\n` (`🧬️semio/🦀️.rs:110` `BINARY_MAGIC`). Both tests now assert their
artifact's real header from the declared constant, and the corrupted-header case zeroes that same
header. **The two artifacts genuinely differ**: 3d emits `\x89SEM…`, 2d emits `P2D2` + `\x89SPK…`
(the `.spk` container behind its own mounted prefix) — a second cross-artifact codec inconsistency
worth a follow-up.

---

## 3. Remaining failures, with owners

### 3.1 NEW, and the most important finding: the node-graph surface cannot render with real operators

| tests | message |
|---|---|
| `component::tests::every_window_and_panel_surface_fits_the_resident_surface_bound`, `component::tests::the_first_turn_sequence_retires_every_flow_host_it_builds`, `modes::edit::windows::flow::tests::{renders_node_graph_scene, main_graph_scene_exports_flow_backed_node_graph_fields}` | `ui.fixed-capacity: fixed UI admission failed at scene-surface.encode: surface payload exceeds fixed capacity with 111031 bytes` |
| `component::tests::generation3d_labels_translate_catalogue_and_inspector_in_german`, `panels::catalogue::tests::generation3d_labels_resolve_native_english_by_default` | `ui.catalogue.items: fixed UI catalogue admission failed` |

**Root cause.** `flow_backed_node_graph_extras`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs:282-286`) embeds
`operators: flow_operator_catalogue_records()` **and** `catalogue_json: flow_catalogue_sections()` —
the ENTIRE registered operator catalogue — into every node-graph surface payload. That payload is
admitted against `ui_contract::UI_FIXED_BYTES = 32 * 1_024`
(`🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs:22`) by `ui_scene::encode`
(`🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs:66`). With the `brep`/`math` sets installed — which is
the live configuration — the catalogue alone is ~100 KB and the surface is **111 031 B, 3.4× the
capacity**. The measurement in `📓️runtime-defects-2026-09-09.md` §2.4 (largest surface 14 625 B)
was taken against an EMPTY operator registry and is therefore not representative.

Consequence: with real operators installed, the generation3d/generation2d/flow **node-graph window
cannot render at all**, and the catalogue panel exceeds its fixed item admission. The fix is a
design change — the operator catalogue is app-static, not per-surface, and belongs behind a handle
or a separate app-scoped surface — and it is the flow/UI lane's, not this one's. **Do not raise
`UI_FIXED_BYTES`**: it is a preallocated fixed capacity paid per surface.

### 3.2 generation3d — 51 remaining

| class | count | owner / next step |
|---|---|---|
| §3.1 fixed-capacity + catalogue admission | 6 | flow/UI lane (framework) |
| retained editor commands never driven by `dispatch_typed` (`add_generation`, `set_active_example`, `add_widget`/`remove_widget`, `translate/rotate/scale-selection`, `reorganize`, `undo_redo_round_trips`, `select_generation`) | 12 | editor-gaps lane — these are retained jobs; the testkit must drive the job ladder, not just dispatch |
| preview/geometry still red after the kernels are installed (`renders_world_preview_scene`, `switching_active_example_changes_preview_meshes`, `render_uses_the_configured_preview_camera`, `sun_measures_are_exposed_on_preview_windows`, `preview_payload_has_meshes_and_instances` "mesh has too few positions", `each_example_loads_distinct_fixture_and_preview_geometry`, `generate_*_renders_surfaces`) | 10 | tessellation/example lanes — now REAL geometry failures, no longer a registry gap |
| remaining owned-projection drops in hand-written tests (`document_from_mesh_returns_valid_default_snapshot`, `generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`, `wireframe_show_mode_strips_shaded_triangles`, `box_fillet_preview_tests::inference_determinism_law`, `small_move_widget_feature_matches_the_test_only_third_party_oracle`) + 3 `final Dictionary ownership …` | 8 | same law as §2.3, one site at a time; mechanical |
| `apply: ValidationFailed("edit history insertion requires its exact mutation retirement factory")` | 3 | `ArtifactStore` publication path — the store test fixtures build an envelope without the artifact's retirement factory |
| `MoveWidget` round trip loses the layout entry (`move_widget_round_trip_{inserts_when_absent,replaces_when_present}`) | 2 | **real product failure** in the move-widget diff/apply path — the layout delta does not survive `diff → apply` |
| `connect_synapse_satisfies_the_inverse_and_absorb_laws` — forward outcome carries an Error/Fatal message | 1 | mutation-laws lane |
| `two_instances_converge_disjoint_widget_moves` — `module.vcs` remote snapshot merge is fail-closed | 1 | framework vcs (pre-existing, declared fail-closed) |
| mounted-registry / retained-authority laws (`authoritative_publication_rejects_stale_generation_aba_and_parent`, `domain_local_static_verifier_…`, `cancelled_and_stale_aba_initializers_…`, `every_fourteen_variant_decodes_through_retained_structural_grants`, `non_empty_canonical_snapshot_round_trips_one_grant_at_a_time`) | 5 | mounted/retained lane |
| inference laws (`inference_default_law` ×2, `diff_absorb_prefers_incoming_fixture_and_preserves_generation`) | 3 | inference/diff lane |

None of the 51 belongs to the two mid-flight lanes named in this lane's brief: nothing under
`🧊️brep/🧬️schema/🔺️diff/🎨️blend/**` is in this crate's suite, and the framework selection path
(`interactionSelect`, `🔌️plugin/🦀️.rs` selection, `context_menu`) is green here
(`context_menu_grouped_disclosure_stays_within_budget`, `context_menu_reads_the_framework_owned_graph_selection`,
`generation3d_interaction_selection_owns_its_persisted_history` all pass).

### 3.3 generation2d — 46 remaining

| class | count | root cause / owner |
|---|---|---|
| `interactive-job.catalog-authority: tool proof catalog must exactly join migrated generated declarations to live concrete factories` | 24 | `migrated={}` — **empty**, while `generated_ids` lists all 21 rows. `migrated_tool_ids()` (`🔌️plugin/🦀️.rs:11659-11667`) reads `AppActionRegistry`, whose `actions` map `from_definition` (`:11541-11546`) builds **only** from `window_kinds[].actions`; generation2d's `.action_interactive_job(…)` calls (`🌀️generation2d/…/✏️editor/🦀️.rs:1072-1093`) never reach it, so every bounded proof is rejected and the app cannot be constructed at all. Owner: generation2d editor / plugin app-definition lane. This one class is >half of 2d's remaining failures. |
| owned-projection drops in hand-written 2d tests (`ordered-map root …` ×8, `nonempty generation root …` ×4, `final Dictionary ownership …`, `artifact store reached Drop …`) | 14 | same law as §2.3 — the 2d twins of the sites fixed in 3d; mechanical |
| `command_envelope_round_trip_holds_for_an_applied_operation` — `mutation.target-missing: Widget "note-9" does not exist` | 1 | the test applies an update to a widget the 2d default document does not contain |
| retained/mounted laws, inference laws, `mounted_registry` | 7 | same owners as §3.2 |

---

## 4. Runs

### 4.1 `cargo check -p semio-s-plugin-procedural --keep-going` (native)

`rc=0`, **0 errors**, 7 warnings — none of them this lane's:
`semio-framework-os-flow`'s `unused import: SpaceMember`, two `unused extern crate`, one
`unnecessary qualification` in generation2d, one in generation3d.
Log: `🗑️generated/suite-check-procedural-native.txt`.

### 4.2 `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`

`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `rc=0`, **0 errors**, the same 7 peer warnings.
Log: `🗑️generated/suite-check-procedural-wasm.txt`.

Warnings are present in both, so both are real type-checks and not aborted expansions.

### 4.3 Suite tails

generation3d, `--features component-app-assembly`:

```
test result: FAILED. 244 passed; 51 failed; 0 ignored; 0 measured; 0 filtered out; finished in 34.10s
```

generation3d, default features:

```
test result: FAILED. 138 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

generation2d, `--features component-app-assembly`:

```
test result: FAILED. 179 passed; 46 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

Logs: `🗑️generated/suite-3d-before.txt`, `suite-3d-after.txt`,
`suite-3d-after-default-features.txt`, `suite-2d-before.txt`, `suite-2d-after.txt`.

---

## 5. Concurrency notes

- `semio-framework-plugin` was transiently broken by a peer mid-lane (`E0592: duplicate definitions
  with name with_children`, `🔌️plugin/🦀️.rs:7275/7283`); it went green again on its own. One wasm
  check was run against this lane's own half-applied edit and was rerun.
- The working tree carries a large amount of unrelated peer churn (226 changed paths at close);
  everything this lane touched is listed in §6.

## 6. Files changed

Framework:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `bounded_config_store_one_item_preparation_factory` + its `BoundedConfigPreparation`/`BoundedConfigPreparationFactory` and re-export (§2.5).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — `assert_dsl_round_trip_cold`, `assert_dsl_pack_equivalence_cold`, `assert_pack_round_trip_cold` (§2.6).

generation3d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/`):

- `🦀️.rs` — mounts `flow_operators`; `🧪️tests/🔬️flow-operators/🦀️.rs` — **new** (§2.7).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs` (`DerefMut`), `…/🔺️diff/🦀️.rs` (`retire_cold` + `Generation3dDiffRead`), `…/🔺️diff/📝️text/🦀️.rs` (apply/absorb retirement), `…/🧬️mutations/🦀️.rs` (`apply_generation3d_mutation`).
- `…/🧬️mutations/💾️binary/🦀️.rs`, `…/📸️snapshot/💾️binary/🦀️.rs`, `…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` — `Drop` guards (§2.1).
- `…/👁️viewer/🦀️.rs` — `command_id`, `build_config_store_one_item_preparation_factory`, `build_document_store_disposer`, `GENERATION3D_VIEW_CONFIG_STORE_MAXIMUM_BYTES`.
- tests: `…/✏️editor/🧪️tests/{🔬️test-support,🔬️unit}/🦀️.rs`, `…/👁️viewer/🧪️tests/🔬️testkit/🦀️.rs`, `…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, `…/🧬️schema/📸️snapshot/{📝️text,💾️binary}/🧪️tests/🔬️unit/🦀️.rs`, and all **14** `…/🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️.rs` re-emitted from the template.
- fixtures: **40** files under `…/🧫️fixtures/🧬️mutations/**` (14 `🦠️mutation/🔣️.json` retagged, the rest canonicalised).

generation2d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/`):

- `…/🧬️schema/📸️snapshot/🦀️.rs` (`retire_cold` + `Generation2dSnapshotRead`, both new), `…/🔺️diff/🦀️.rs` (`retire_cold` + `Generation2dDiffRead`), `…/🔺️diff/📝️text/🦀️.rs`, `…/🧬️mutations/🦀️.rs`.
- `…/🧬️mutations/💾️binary/🦀️.rs`, `…/📸️snapshot/💾️binary/🦀️.rs`, `…/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` — `Drop` guards.
- tests: `…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, `…/🧬️schema/📸️snapshot/{📝️text,💾️binary}/🧪️tests/🔬️unit/🦀️.rs`, and all **14** `…/🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️.rs`.
- fixtures: **39** canonicalised files under `…/🧫️fixtures/🧬️mutations/**` (the mutation retag was reverted — §2.2).

Ticket folder:

- `🐍️fixture-canonicalize.py`, `🐍️fixture-tests-template.py` — the two re-emission inputs, kept.
- `🗑️generated/suite-*.txt` — raw logs.

## 7. Owed follow-ups

1. **§3.1** — the flow node-graph surface embeds the whole operator catalogue per surface and blows
   the 32 KiB fixed UI admission with real operators installed. Highest-value remaining defect: the
   node-graph window is unrenderable in the live configuration.
2. **§3.3** — `AppActionRegistry::from_definition` ignores top-level app actions, which makes every
   generation2d bounded tool proof fail closed (24 tests, and the app cannot be constructed).
3. `MoveWidget`'s layout delta is lost across `diff → apply` (2 tests, real product failure).
4. The two artifacts disagree on their mutation wire tagging (§2.2) and on their pack container
   header (§2.9). Both should converge on one contract.
5. `RUST_MIN_STACK` at 8 MiB (`🧩️puzzle/📦️packages/🦀️rust/📜️script.ts:8`) is too small for this
   crate's dispatch depth; raise it before wiring the suite into `nx`.
6. There is still no `fixtures generate` command — only `fixtures lint`. The template used here is
   `🐍️fixture-tests-template.py`; promoting it into `📜️script.ts` is the clean long-term home.
