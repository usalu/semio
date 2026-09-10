# Fold Contract — `batched item candidate failed its exact fixed fold contract`

Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, lane: fold contract. 2026-09-10.

## 1. The live finding

`📓️runtime-verification-2026-09-09.md` (boot #2) recorded that on the served procedural 3d app
**every** typed operation failed:

```
typed-operation failed: validation failed: batched item candidate failed its exact fixed fold contract
trace: operations=["27:Retiring:true:true"] latest_wins_empty=true
```

`setActiveExample` (the app's own retained command) and `interactionSelect` (the framework-owned
local-observation route) both failed with the *same* message, which is the tell that the defect is
in the declaration side shared by both lanes and not in either command's body.

## 2. The contract, read from the store

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16367`, inside `ArtifactStore::fold_batch_item`:

```rust
|| candidate.edit.inverse.len().saturating_add(candidate.edit.forwards.len()) > publication.footprint.work_items
{
    return Err(VcsError::ValidationFailed("batched item candidate failed its exact fixed fold contract".into()));
}
```

So `ArtifactStoreOneItemFootprint::work_items` counts **staged edit ROWS**, not mutations:

> one durable item costs its single `forwards` row **plus every row its `inverse` yields**.

The same arithmetic gates the gesture as a whole in `PreflightingCommit`
(`batched prepared candidate failed its exact fixed commit contract`), against the *merged*
declaration of all admitted items.

Consequences:

- a **point-invertible** durable mutation (the normal case: `Mutation::inverse` yields exactly one
  row) costs **2**;
- declaring `work_items: 1` therefore leaves **zero** inverse capacity and fail-closes *every*
  single-mutation durable gesture — including the very first action of the app;
- the store's contract does **not** exceed its own schema: `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS`
  and the store's own batched fixtures are consistent with row-counting. The store is right; the
  declarations were wrong.

The reserved transient/ephemeral lanes are unaffected: a transient mutation's `inverse` is empty, so
`1 + 0 <= 1` holds and `work_items: 1` remains correct there.

## 3. Root cause

Two declaration sites wrote the literal `work_items: 1` for **point-invertible durable** mutations:

1. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs` — the Artifact-lane
   preflight (`admit_generation3d_artifact_mutation`) and the Config-lane preflight
   (`admit_generation3d_config_mutation`), i.e. **all** of generation3d's retained routes,
   `setActiveExample` first among them.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `admit_bounded_config_mutation`, the
   single declaration every *framework-reserved* bounded config lane makes, which is what
   `interactionSelect` (and `interactionHover`, `selectAll`, `clearSelection`,
   `setInteractionGranularity`, `setSelectionMode`, …) publishes through.

That is why one message covered both symptoms.

## 4. Fix, by layer

- **store** (`🏪️store/🦀️.rs`): make the contract impossible to mis-state at a call site — publish the
  row arithmetic as named constructors and document the row semantics on the type.
  - `pub const ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS: usize = 2;` (line 13383)
  - `ArtifactStoreOneItemFootprint::for_one_invertible_item(retained_bytes)`
  - `ArtifactStoreOneItemFootprint::for_one_item(inverse_rows, retained_bytes)`
  - docstring on `ArtifactStoreOneItemFootprint` stating `work_items` counts staged edit rows.
- **generation3d app**: ONE shared builder `generation3d_one_item_footprint(retained_bytes)`
  delegating to `for_one_invertible_item`, used by *both* durable preflights — so the two lanes
  cannot drift, and no retained route can regain a hand-written literal.
- **framework plugin** (`🔌️plugin/🦀️.rs`): `admit_bounded_config_mutation` declares through
  `for_one_invertible_item`, fixing `interactionSelect` and every sibling reserved route at once.

## 5. Candidate, before and after

| lane | gesture | forward rows | inverse rows | declared before | declared after |
| --- | --- | --- | --- | --- | --- |
| generation3d Artifact | `setActiveExample` (per item) | 1 | 1 | 1 → **reject** | 2 → admit |
| generation3d Config | `setActiveExample` config snapshot | 1 | 1 | 1 → **reject** | 2 → admit |
| framework reserved config | `interactionSelect` | 1 | 1 | 1 → **reject** | 2 → admit |

## 6. Tests

`✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs` — a language-agnostic law set:

- `set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example` — walks the
  full 8-example picker cycle, replaying each authored gesture against the store's *running post
  root* (via the app's real `prepare_generation3d_artifact`), asserting per item AND gesture-wide
  that folded rows ≤ declared, and that the replay actually reaches the example's own fixture.
- `set_active_example_config_gesture_fits_its_declared_fold_envelope` — the single-item case.
- `a_one_work_item_declaration_cannot_carry_a_point_invertible_item` — hostile control: pins that
  `work_items: 1` must NOT admit forward+inverse, and that the shared builder yields the constant.
- `both_durable_lanes_declare_through_the_one_shared_footprint_builder` — source law: exactly one
  builder, both preflights route through it, no bare struct literal returns.
- `set_active_example_publishes_and_swaps_the_document_for_every_bundled_example` — the real retained
  typed path (`dispatch_typed` + bounded publication/ACK), all 8 examples, no fault lane, document
  actually swaps.
- `interaction_select_publishes_through_the_retained_typed_path` — framework route publishes and the
  selection is readable afterwards.

## 7. Three further defects the laws uncovered on the same path

Establishing the contract required replaying `setActiveExample`'s authored gesture against the
store's own running post root. That exposed three more defects, all fixed here.

### 7.1 Orphaned layout overrides were never retired

`🧬️schema/🧬️mutations/🦀️.rs` authored `delete-widget-position` for every layout key absent from the
target — but AFTER the `delete-widget` operations. `delete_widget`'s diff leaves the widget's `layout`
entry behind, and `delete_widget_position`'s diff fail-closes with `mutation.target-missing` once the
widget is gone (`…/🧹️delete-widget-position/🔺️diff/🦀️.rs:11`), whose empty outcome the preparation
applies as a silent no-op. Every example swap therefore accumulated the previous example's layout
keys forever. Fixed by authoring the orphan removals FIRST, while their widgets still exist.

### 7.2 Surviving entries were never reordered

The mutation vocabulary has no reorder verb — `update-widget`/`update-synapse` replace in place — so
two survivors that cross each other between fixtures kept the SOURCE order. `box-fillet-preview →
sphere-box-fuse` published `[size, radius, …]` where the example authored `[radius, size, …]`. Fixed
with `reordered_survivors`: the survivors outside a longest run of already-ascending target positions
are authored as a delete plus a create at the target index — no new schema, and the create pass's
ascending insert then provably reconstructs the exact target order. Applied to widgets and synapses
alike.

### 7.3 The camera is deliberately NOT authored — checked, and left alone

`FlowFixture::camera` is part of the artifact document and `update-camera` exists in the vocabulary,
so a replayed gesture never reaches the target camera. That is **declared behaviour**, not a gap:
`mutations::tests::fixture_ops_ignore_camera` pins it, and the viewport camera is a Config-lane facet
throughout (`commands/📷️set-camera` emits `Emit::config` only; `config_after_example_load` carries the
example's camera onto the Config lane). Authoring it on the artifact lane was tried here, broke that
law, and was reverted; the fold-contract test now asserts the *exclusion* instead, citing the law.

### 7.4 Every retained publication test aborted on an 8 MiB stack

Not caused by these laws: the PRE-EXISTING
`commands::set_active_example::tests::set_active_example_via_string_action_loads_fixture` aborts the
same way. A live `VcsArtifactApp` plus the host's bounded publication/ACK state machine is a
multi-megabyte future in a debug build, and `libtest`'s 8 MiB-per-test thread cannot hold one:
`fatal runtime error: stack overflow, aborting` kills the whole test binary with no panic and no
failing-test name past the one that tripped it — so every later test in the process silently never
ran.

No repo change was needed: the sanctioned runner already handles it —
`runCargoTestBudgeted` (`🧰️framework/🛍️products/🦑️repo/…/🟦️.ts:1661`) defaults `RUST_MIN_STACK` to
`134217728`, which this lane measured as sufficient. A raw `cargo test` invocation does not, which is
how both this lane and the unit-suite lane met it. A `[env]` block in `.cargo/config.toml` was tried
and reverted: cargo exports `[env]` to *every* process it runs, including the `-Z threads=8` rustc
processes, and the repo's build scripts deliberately keep the build-time value at 32 MiB.

The laws additionally keep exactly one app per test thread (one test per bundled example, body
expanded by macro, never an awaited helper) — an awaited helper nests two of these futures and
overflows even at 8 MiB.

Follow-up worth its own ticket: the debug-build future size itself. The wasm guests size their stacks
explicitly (`PLUGIN_WASM_STACK_BYTES` = 8 MiB), so the same growth reaching the release/wasm path
would trap in the browser with a bare `RuntimeError`, exactly as the renderer did before its
`-zstack-size` flag.

## 8. Runs

All under `CARGO_TARGET_DIR=$S/target-fold`, `RUSTC_WRAPPER=""`. Raw logs in `🗑️generated/fold-*.txt`.

### 8.1 The fold-contract laws — `🗑️generated/fold-14-run.txt`

```
running 13 tests
… a_one_work_item_declaration_cannot_carry_a_point_invertible_item ... ok
… both_durable_lanes_declare_through_the_one_shared_footprint_builder ... ok
… interaction_select_publishes_through_the_retained_typed_path ... ok
… set_active_example_artifact_gesture_fits_its_declared_fold_envelope_for_every_example ... ok
… set_active_example_config_gesture_fits_its_declared_fold_envelope ... ok
… set_active_example_publishes_{box_fillet,box_shell,face_sweep_extrude,hex_column,
   rect_extrude,rectangle_wire,sphere_box_fuse,sphere_torus} ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 298 filtered out; finished in 1.57s
```

The envelope walk, hop by hop — every gesture now folds EXACTLY its declaration, and reaches the
target fixture:

```
[DEBUG] fold envelope box-shell-preview      -> hexagonal-mushroom-column: 29 items, 58 rows, 58 declared
[DEBUG] fold envelope hexagonal-mushroom-column -> rectangle-extrude-volume: 30 items, 60 rows, 60 declared
[DEBUG] fold envelope rectangle-extrude-volume  -> sphere-cut-with-torus:    38 items, 76 rows, 76 declared
[DEBUG] fold envelope sphere-cut-with-torus     -> box-fillet-preview:       34 items, 68 rows, 68 declared
[DEBUG] fold envelope box-fillet-preview        -> sphere-box-fuse:          20 items, 40 rows, 40 declared
[DEBUG] fold envelope sphere-box-fuse           -> face-sweep-extrude:       31 items, 62 rows, 62 declared
[DEBUG] fold envelope face-sweep-extrude        -> rectangle-wire-preview:   17 items, 34 rows, 34 declared
[DEBUG] fold envelope rectangle-wire-preview    -> box-shell-preview:        20 items, 40 rows, 40 declared
```

And the publications, with the real lane set the host produced:

```
[DEBUG] setActiveExample box-fillet-preview        published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=5
[DEBUG] setActiveExample box-shell-preview         published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=4
[DEBUG] setActiveExample face-sweep-extrude        published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=7
[DEBUG] setActiveExample hexagonal-mushroom-column published: lanes=[Config, Transient, Ui, Terminal]           widgets=7
[DEBUG] setActiveExample rectangle-extrude-volume  published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=7
[DEBUG] setActiveExample rectangle-wire-preview    published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=3
[DEBUG] setActiveExample sphere-box-fuse           published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=6
[DEBUG] setActiveExample sphere-cut-with-torus     published: lanes=[Artifact, Config, Transient, Ui, Terminal] widgets=6
```

`hexagonal-mushroom-column` carries **no** Artifact lane on purpose — the boot document IS that
fixture, so its artifact gesture is empty; the first law asserts exactly that.

### 8.2 Crate suites

| suite | baseline (`📓️unit-suite-2026-09-09.md` §1) | now | log |
|---|---|---|---|
| `--lib` (default features) | 138 passed / 13 failed (151) | **149 passed / 3 failed** (152) | `fold-17-run.txt` |
| `--features component-app-assembly --lib` | 244 passed / 51 failed (295) | **290 passed / 22 failed** (312) | `fold-18-run.txt` |

**None of the remaining failures belongs to this lane.** Each maps to a class the unit-suite lane
already recorded and assigned in its §3.2 table: retained editor commands never driven through the
job ladder (editor-gaps lane), preview/geometry after kernel install (tessellation lane), owned-
projection drops in hand-written tests, the `ArtifactStore` retirement-factory fixtures, the declared
fail-closed `module.vcs` remote merge, and the mounted-registry / retained-authority laws
(`generation3d-mounted.value-backpressure`, `generation3d-mutation.body-malformed`,
`RetainedJobPayload requires one-page close`) — the mounted/retained lane's.

### 8.3 Compile gates — `🗑️generated/fold-18-run.txt`, `fold-20-wasm.txt`

```
=== PLUGIN CHECK ===  cargo check -p semio-framework-plugin --tests --keep-going
warning: `semio-framework-plugin` (lib test) generated 75 warnings
    Finished `dev` profile [unoptimized] target(s) in 23.42s          → 0 errors

=== PROCEDURAL NATIVE ===  cargo check -p semio-s-plugin-procedural --keep-going
    Finished `dev` profile [unoptimized] target(s) in 1m 59s          → 0 errors

=== PROCEDURAL WASM ===  --target wasm32-wasip2 --profile wasm-dev (CARGO_PROFILE_WASM_DEV_DEBUG=false)
    Finished `wasm-dev` profile [unoptimized] target(s) in 1m 20s     → 0 errors
```

Warning counts are quoted deliberately: a zero error-class count means nothing if the crate aborted
at module expansion. The wasm gate matters on its own — the editor module is behind the
`component-app-assembly` feature, and a plain `cargo check -p …-generation3d --tests` compiles **none
of it** and reports a vacuous green (this cost one round here).

## 9. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | `ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`, `for_one_invertible_item`, `for_one_item`, row-semantics docstring |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `admit_bounded_config_mutation` declares through `for_one_invertible_item`; removed the temporary `[DEBUG] typed-operation publication` trace and its `TYPED_PUBLICATION_TRACE` counter from the hot publication path |
| `✏️s/…/🧊️generation3d/…/✏️editor/🦀️.rs` | one shared `generation3d_one_item_footprint` for both durable lanes / all 29 retained routes |
| `✏️s/…/🧊️generation3d/…/🧬️schema/🧬️mutations/🦀️.rs` | orphan layout removals authored first; `reordered_survivors` + survivor rebuild for widgets and synapses |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs` | the 13 laws |

## 10. Status log

- 2026-09-10 ~01:31 — store constant + constructors, generation3d shared builder, framework
  `admit_bounded_config_mutation`, and the fold-contract test module landed (auto-commit `6ad7b0e7bc`).
- 2026-09-10 — orphan-layout ordering, survivor reorder, camera exclusion re-asserted, per-example
  publication laws, debug-trace removal. All 13 laws green; three compile gates green; both crate
  suites well ahead of their baselines with no lane-owned failures left.
