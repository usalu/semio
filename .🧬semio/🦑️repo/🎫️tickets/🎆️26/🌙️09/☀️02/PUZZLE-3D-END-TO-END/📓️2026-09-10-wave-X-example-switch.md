# Wave W-X — `setActiveExample` one-emit switch + scene republish

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Builds on Wave U. Evidence that opened this wave:
`📓️2026-09-10-wasm27-browser-verification.md` §0 (wasm #28).

## 1. Causes

### (a) Architecture / perf

`Puzzle3dSetActiveExampleWork` was already cursorized (hostile law) and already accumulated every
mutation into one `Complete` emit at `Publish`. Two things still made a Nakagin switch a minutes-long
churn in the browser:

1. **One document item per `step()`.** Each delete/create attraction, object, volume, reference, and
   compatibility row returned `Progress` after a single push. Nakagin's extent is 150+ items, so the
   Migrated interactive job took 150+ command-ingress round trips before it even published. Each
   ingress in the probe was ~1.5 s (`thunk start → command ingress settled → completion apply op N →
   applyHostEffects refresh {windowBodies:["puzzle3d.play.composite"]}`).
2. **No coalesce key on the Complete emit.** The store batch machinery Wave U fixed for gumball
   (`begin_apply_batch` + `publication.set_coalesce_key`) was unused. The emit used `Default` for
   `coalesce_key`, so a whole-document replacement had no named one-undo identity.

The docstring at the Artifact one-item factory (~:6935, was ~:6884) explained the old decomposition:
each mutation kind has a fixed semantic work capacity, so the work must not become a single-shot
`BoundedFirstStepCommandWork` whole-document reset. That constraint stays. The fix is to **size
chunks to the capacity** (8 items — under the 2 ms `PUZZLE3D_MEASURED_STEP_BUDGET` / 8 ms interactive
ceiling) **per mutation kind**, then publish **one** coalesced emit.

Publication still folds one mutation per grant (store contract: `forwards.len() == 1`). That is cheap
in-process. The 1.5 s cost was the **host refresh of the composite window on every ingress**, which
re-encoded a scene against the still-committed previous fixture because the batch has not `Published`
yet. Fewer worker ingresses + one completion is what makes Nakagin land in a few seconds.

### (b) Render

Native Wave-P scene-lane encode laws already passed. The browser still showed the old platform in
both viewports after ~100 partial `puzzle3d.play.composite` refreshes.

Two stacked reasons:

1. **Staging vs committed document.** Intermediate refreshes during the 150-item grind ran against
   the *committed* Concrete Forest snapshot. The navbar combobox already read "Nakagin Capsule Tower"
   (the command arg), so the chrome lied while the world was honest-but-stale. This is why (a) must
   finish in a handful of ingresses: the world can only change after `Published`.
2. **Interpreter lane cache served the previous fixture across a hash change.**
   `world3dSurfaceLaneTexts` (`🗣️Interpreter/🟦️.tsx`) cached `${surfaceNodeId}:${lane}`. When a
   successor spine declared a new hash and the new carrier had not finished arriving, the walk fell
   through to `else if (cached)` and returned the **previous** complete text. That is exactly a
   document swap on a stable surface node: Forest stays on screen until a later assemble sees
   byte-exact Nakagin text. Concrete Forest (small, one-patch arrival) updated; Nakagin (paged)
   could keep Forest forever.

The native guest encode after settle was already correct (`world_surface_carrier_census` on
`puzzle3d.play.composite` yields a new instance-lane hash and 180 Nakagin instances). The missing
wiring was the renderer applying a new lane carrier outside a full refresh / complete arrival.

## 2. Changes

| File | Lines | What |
| --- | --- | --- |
| `✏️editor/🦀️.rs` | :2059 | `setActiveExample` is an explicit `Puzzle3dScopeClass::Chrome` (`UiDirtyScope::Full`) — honest whole-document + both viewports |
| `✏️editor/🦀️.rs` | :5149–:5150 | `PUZZLE3D_SET_ACTIVE_EXAMPLE_CHUNK = 8`, `COALESCE_KEY = "set-active-example"` |
| `✏️editor/🦀️.rs` | :5190–:5198 | `take_chunk` — advances the stage cursor by at most one chunk |
| `✏️editor/🦀️.rs` | :5235–:5398 | every delete/create stage drains a chunk of that kind, then transitions; `Publish` emits all mutations with `coalesce_key` + Chrome scope |
| `✏️editor/🦀️.rs` | :6933–:6936 | Artifact-factory docstring: chunked kinds, one coalesced document-replacement gesture |
| `✏️editor/🦀️.rs` | :8051–:8053 | includes `🧪️tests/🔬️example-switch/🦀️.rs` (new file — unit.rs was hot under W-F2c) |
| `🗣️Interpreter/🟦️.tsx` | :467–:471, :533–:543 | successor hash that is not byte-complete is **omitted**; cache is reused only when `cached.hash === ref.hash` |
| `🗣️Interpreter/…/surface-scene-lanes/🟦️.tsx` | :133–:148, :179–:189 | law updated + new successor-hash replacement law |

Hostile-static stage names, `Puzzle3dSetActiveExampleWork::default()` wiring, and one-mutation-per-item
in the emit are unchanged. `⏳️precompute/🦀️.rs`, testkit, and `with_puzzle3d_app_for` were not touched.
Wasm was not rebuilt; `:6013` was not probed.

## 3. Laws

| Law | Layer | Verdict |
| --- | --- | --- |
| `set_active_example_chunks_by_kind_and_emits_one_coalesced_gesture` | guest work | **pass** — 216 mutations, `coalesce_key = set-active-example`, Chrome scope, 42 steps ≪ 216 items |
| `set_active_example_lands_as_one_edit_and_republishes_the_world_scene` | tool-job + scene encode | **pass** — one undo restores Forest; composite census instance-lane hash changes; assembled JSON has `NAKAGIN_EXAMPLE_FIXTURE.objects.len()` instances |
| existing `set_active_example_*` (hostile, multi-step, tool-job swap, undo empty, ceiling) | guest | **pass** (7/7 with the two new = the `set_active_example` filter) |
| `world-3d paged scene carrier` (9 vitest) | Interpreter | **pass** — successor hash omitted until complete; completed successor replaces Forest on the same surface node id |

The Interpreter successor-hash law **fails before** the cache change (old `else if (cached)` returned
Forest under hash `bbbb`) and **passes after**.

## 4. Tails

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
    Finished `dev` profile [unoptimized] target(s) in 49.32s
```

```
cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle -j 4
    Finished `dev` profile [unoptimized] target(s) in 48.22s
```

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 set_active_example -- --test-threads=1
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 623 filtered out; finished in 1.37s
```

```
SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts -t "world-3d paged scene|successor hash|last complete text"
Interpreter world-3d paged scene carrier: 9 passed (including successor-hash omit + replace)
```

(The vitest process also loaded `@semio-tech/framework-renderer-react` package-integration, which
failed `self is not defined` — foreign, not this wave.)

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -j 4 -- --test-threads=1
test result: FAILED. 620 passed; 10 failed; 0 ignored; 0 measured; 0 filtered out; finished in 76.69s
```

Honest vs baseline **618/13**:

- **+2 passed** = the two new W-X laws.
- **10 failed**, all foreign: five `fill_*` / `set_fill_count_*` editor tests, four
  `precompute::…::fill_worker_*` / `bounded_fill_job_*` tests (live W-F2 / W-F2c), and
  `two_instances_converge_disjoint_object_edits_via_backbone` (fail-closed remote merge).
- Wave-P `a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh` **ok**.
- `setSpacing` / stale-verb and the previous wave-P republish miss are not in this 10.

## 5. What the coordinator's browser probe still needs

Do **not** treat in-process green as browser-green. After the coordinator rebuilds wasm (next
number after #28) and serves `:6013`:

1. `bun 🔍️browser-probe.ts --interact --example=Nakagin` (tour dismissed).
2. **Ingress bound:** `setActiveExample` should settle in a **handful** of command-ingress /
   completion-apply cycles (tens, not 150+ at 1.5 s each). Worker steps are 42 for Forest→Nakagin;
   publication folds remain one mutation per grant but must not each trigger a 1.5 s composite
   refresh if the shell only applies host effects on the **terminal** completion.
3. **World body:** both Top and Perspective show the Nakagin tower, not the previous platform.
   Navbar already showed the name on #28; the world must match.
4. **One undo** in the history panel restores the previous fixture (one row, not 150).
5. **Zero** `ui.fixed-capacity` / `history-panel.row-action-args` / `intake-budget-exhausted` faults
   (Wave U paging + Wave P lanes + this wave's one-history-entry).

Rebuild **#29** confirmed (a)+(b) of §§1–4: coalesced Publish (completion apply operation 94,
scope full, a handful of ingresses). The world still showed Concrete Forest because
`retainedUiRefreshEffects` then refused the new surface — see **§6**. That cluster is fixed
in-process; the coordinator must rebuild wasm (next number after #29) and re-probe. Do **not**
treat a still-stale canvas after that rebuild as a ShellHost lane-walk gap until the console
is free of `SurfaceReconcileFault::Credits` on `puzzle3d-main`, `framework.panel.artifact`, and
`framework.section.measures`.

## 6. Surface reconcile capacity (probe #29)

Browser evidence: `🗑️generated/probe-2026-09-10T01-57-47.md` (wasm #29). After the coalesced
Publish landed, `retainedUiRefreshEffects` refused three surfaces and both viewports kept
Concrete Forest.

`SurfaceReconcileLimits::default()` lives at ui-runtime `♻️reconcile.rs` :575–:584
(`max_nodes` = `UI_DOCUMENT_NODES` = 128, `max_items` = 4097, `max_bytes` =
`UI_RESIDENT_SURFACE_BYTES` = 8 MiB). Usage is accumulated per presented node as
`size_of::<FlatPresentedNode>() + semantic_usage.bytes` with
`SURFACE_RECONCILE_SEMANTIC_COPIES = 3` on binding-arg strings (`♻️reconcile.rs` :716,
:788–:790, :1431–:1447). `reserve_surface_reconcile` (`:2215–:2220`) **rejects** any limit
larger than Default — raising 128 fans out into many `SurfaceFixedVec`s. This wave did **not**
edit `♻️reconcile.rs` (W-O4c / coordinator own that region) and did **not** bump the caps.

### (a) `1:puzzle3d-main` — 129 nodes / max 128

**Cause:** node count **scales with scene size**. `paged_text_carrier` / `section_text_chunks`
emitted one `Component::Text` node per `UI_TEXT_MAX_BYTES` (512). `UiText` is `[u8; 512]` —
the leaf byte cap cannot rise. Nakagin instances ≈ 55 154 B ≈ 108 unpacked slices, plus page
nodes, the other lanes, and the surface itself, presented **129** nodes. This is not constant
overhead sitting just above 128.

**Fix:** pack `1 + UI_FIXED_LIST_ITEMS` (33) UTF-8 slices into ONE text node — `value` plus
ascending `data_attributes` keys `01`..`32` = 16 896 bytes/leaf. Envelope is
`O(lanes + ceil(bytes / pack))`. A Nakagin world surface now presents **26** nodes. Caps stay
at 128.

Fail-before: unpacked 512 B leaves → 129 > 128 (probe #29). Pass-after: 26 ≤ 128.

### (b) `1:framework.panel.artifact` — 18 nodes / 1466 items / 8 419 293 B

**Cause:** Wave U already pages the **guest** outliner. The **host** still credits every
*presented* node. 1466 items are binding-arg string credits (`× 3` semantic copies), not 1466
row nodes. 18 × ~467 KiB ≈ 8.42 MiB — ~30 KiB over the 8 MiB byte cap — triggered on
`interactionSelect` after the switch.

**Fix:** documents past the SDK page (Nakagin, 180 objects) use a **16-node** host envelope
(`PANEL_RECONCILE_NODE_BUDGET`). Small documents that fit `panel_page_rows()` keep the SDK
page so nested vortices stay complete. Nakagin now presents **13** nodes; 13 × ~467 KiB stays
under 8 MiB.

Fail-before: 18 nodes / 8.42 MiB (probe #29). Pass-after: 13 ≤ 16.

### (c) `1:framework.section.measures` — `NodeCapacity`

**Cause:** the same unpacked `paged_text_carrier`. A ~70–100 KiB measures JSON at 512 B/leaf
overflows 128 nodes.

**Fix:** the packed carrier. A 69 601-byte measures map now presents **6** nodes.

Fail-before: payload `> UI_TEXT_MAX_BYTES * 128` would need >128 unpacked leaves. Pass-after:
6 ≤ 128.

### Changes

| File | Lines | What |
| --- | --- | --- |
| `🔌️plugin/🦀️.rs` | :377–:439, :6520–:6568 | packed `section_text_chunks` / `paged_text_carrier`; `built_carrier_text` + `fixture_carrier_text` concat `value` then `data_attributes` |
| `🗣️Interpreter/🟦️.tsx` | :505–:527 | `surfaceSceneLaneText` walks packed `dataAttributes` (same inverse) |
| `🗣️Interpreter/…/surface-scene-lanes/🟦️.tsx` | :32–:48, :114–:137 | TS `carrier()` packs `packChunks`; paging law expects 2 leaves at depth 1 |
| `🎬️scene/…/world3d-scene-lanes/🔣️.json` | `carrier.packChunks: 33`, `packBytes: 16896`; `paging.expectedLeaves: 2`, `expectedLeafDepth: 1` | language-neutral pack contract (`leafBytes` stays 512) |
| `📌️panels/🗿️artifact/🦀️.rs` | :39, :210–:220, :309–:310 | `PANEL_RECONCILE_NODE_BUDGET = 16`; `page_rows_for` tightens large docs |
| `📌️panels/🗿️artifact/…/🔬️unit/🦀️.rs` | :353–:367 | Nakagin panel envelope law |
| `✏️editor/…/🔬️example-switch/🦀️.rs` | :81–:89 | Nakagin world surface node-cap law (included from `✏️editor/🦀️.rs` :8319) |
| `✏️editor/…/🔬️testkit/🦀️.rs` | :571–:653 | `WorldSceneCarrierCensus.nodes` + `count_built_nodes` |
| `🔌️plugin/…/plugin-builder-contract/🦀️.rs` | :4164–:4310 | projected-node census; Nakagin-scale world + 100 KiB measures + packed oversized-lane laws (included from `🔌️plugin/🦀️.rs` :33195) |

`♻️reconcile.rs`, `⏳️precompute/🦀️.rs`, `with_puzzle3d_app_for`, and crate-root `🦀️.rs` were
not touched. Wasm was not rebuilt; `:6013` was not probed.

### Laws

| Law | Layer | Fail-before / pass-after |
| --- | --- | --- |
| `nakagin_world3d_surface_fits_reconcile_node_cap` | native guest encode | 129 > 128 / **26 ≤ 128** |
| `nakagin_scale_world3d_surface_fits_document_node_cap` | plugin `scene_surface` (600 instances) | unpacked overflow / **17 ≤ 128** |
| `nakagin_artifact_panel_fits_reconcile_node_envelope` | host panel surface | 18 nodes / 8.42 MiB / **13 ≤ 16** |
| `a_100kib_measures_section_fits_document_node_cap` | reserved section carrier | payload > `512 * 128` unpacked / **6 nodes / 69 601 B** |
| `world3d_scene_surface_pages_one_oversized_lane_instead_of_faulting` | packed lane | 114 001 B instances → **7 packed leaves**, depth 1 |
| `reserved_section_carrier_pages_a_payload_past_one_node_of_children` | packed section | concat via `fixture_carrier_text` (includes sorted `dataAttributes`); count **text nodes**, not slices |
| vitest `world-3d paged scene carrier` (9) | Interpreter | packed `carrier()` + successor-hash laws still **pass** |

### Tails

```
nakagin_world3d_surface_fits_reconcile_node_cap
[DEBUG] Nakagin world-3d surface presented 26 nodes (cap 128) meshes=907B/1leaves instances=55154B/4leaves selection=212B/1leaves vortices=2B/1leaves attractions=2B/1leaves targetVolumes=2B/1leaves references=2B/1leaves interaction=256B/1leaves lod=125B/1leaves chunking=40B/1leaves environment=92B/1leaves
```

```
nakagin_artifact_panel_fits_reconcile_node_envelope
[DEBUG] Nakagin artifact panel presented 13 nodes (budget 16)
```

```
nakagin_scale_world3d_surface_fits_document_node_cap
[DEBUG] Nakagin-scale world-3d surface presented 17 nodes (cap 128)
```

```
a_100kib_measures_section_fits_document_node_cap
[DEBUG] 100 KiB measures section presented 6 nodes for 69601 bytes
```

```
world3d_scene_surface_pages_one_oversized_lane_instead_of_faulting
[DEBUG] one 114001-byte instances lane packed into 7 text leaves at depth 1
```

```
cargo check -p semio-framework-plugin -j 4
    Finished `dev` profile [unoptimized] target(s) in 12.59s
cargo check -p semio-framework-ui-runtime -j 4
    Finished `dev` profile [unoptimized] target(s) in 2.43s
```

```
SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts -t "world-3d paged scene|successor hash|last complete text"
Tests  9 passed | 742 skipped
```

(`@semio-tech/framework-renderer-react` package-integration still fails `self is not defined` —
foreign, same as §4.)

`bun x tsc --noEmit` in the renderer-react package: **zero new** errors in
`🗣️Interpreter/🟦️.tsx` (walk) or `surface-scene-lanes/🟦️.tsx`. Pre-existing
`ImportMeta.dir` at the Interpreter test export and the docklayoutstore suite are unchanged.

```
cargo test -p semio-framework-plugin --lib -- nakagin_scale_world3d_surface_fits_document_node_cap a_100kib_measures_section_fits_document_node_cap world3d_scene_surface_pages_one_oversized_lane reserved_section_carrier_pages_a_payload
test result: ok. 4 passed; 0 failed
```

Puzzle crate compile is **red this turn**, foreign to W-X (live W-Y):

- `E0428` `puzzle3d_fixture_from_snapshot` defined twice (`✏️editor/🦀️.rs` :301 PlaySnapshot vs :441 Snapshot)
- `E0599` `PastePlacement::from_value`
- `E0277` `JobFault: From<dsl::Fault>` (×2)
- `E0308` `Puzzle3dPlaySnapshot` vs `Puzzle3dSnapshot`

So `cargo check -p semio-s-artifact-puzzle-3d` and `--target wasm32-wasip2 -p semio-s-plugin-puzzle`
are 101. The Nakagin world + panel laws last ran green **this session** (26 / 13) before that
collision landed; they cannot be re-run until W-Y unblocks the crate. Full `--lib` suite last
honest result remains **620 passed / 10 failed** (§4). Expected +2 (world node-cap + panel
envelope) once the crate compiles again. The 10 failures stay foreign (W-F2 fill/precompute +
`two_instances_converge_disjoint_object_edits_via_backbone`).


### Unpack asymmetry (wasm #30 boot block)

Cause: pack wrote 33 slices (`value` + `data_attributes` `01`–`32`) into one text leaf, but
`sectionValueFromBuiltNode` still `JSON.parse`d only `component.value`. Any reserved-section
payload > 512 bytes truncated at column 512 (`Unterminated string in JSON at position 511`).
Wasm #30 never booted (0 windows / 180 s; probe `🗑️generated/probe-2026-09-10T03-23-39.md`).
The same value-only walk made
`reserved_refresh_section_payloads_admit_into_the_retained_section_carrier` red natively
(misattributed as foreign).

#### Consumer inventory

| Consumer | Language | Before | After |
| --- | --- | --- | --- |
| `sectionValueFromBuiltNode` (`PluginRuntime/🟦️.tsx` :1360) | TS boot path | `value` then `JSON.parse` | `packedTextLeaf(value, dataAttributes)` then `JSON.parse` |
| `surfaceSceneLaneText` (`Interpreter/🟦️.tsx` :509) | TS lanes | inline attribute concat | `packedTextLeaf` |
| `read_paged_text_document` (Interpreter wgpu :1206) | Rust host | value-only / now `text.packed_payload()` | shared helper |
| `built_carrier_text` / `fixture_carrier_text` (`plugin/🦀️.rs` :6519) | Rust plugin | inline concat | `TextProps::packed_payload` / `packed_text_leaf` |
| reserved-refresh law (`✏️editor/…/🔬️unit/🦀️.rs` :842) | Rust guest | local value-only walk | `fixture_carrier_text` |
| Shell wgpu section reader | Rust | `read_paged_text_document` | inherits `packed_payload` |
| Puzzle testkit widest-leaf census | Rust | `text.value` **length only** (not reassembly) | left as-is |

No other `JSON.parse` of built-node / section leaf text. Scene/action JSON in World3dHost /
Board2d / plugin-bridge is unrelated.

#### Shared helpers (one per language)

- TS: `PluginRuntime/packed-text.ts` `packedTextLeaf(value, dataAttributes)` — `value` then
  sorted keys (`01`…`32`).
- Rust: `semio-framework-ui-contract` `packed_text_leaf` + `TextProps::packed_payload`
  (`component.rs` :226–:241). `UiFixedMap` iteration is ascending key order.

Every consumer above calls the helper. No per-site reimplementation of the unpack.

#### Vitest (fail-before / pass-after)

`plugin-runtime` `"reassembles a PACKED carrier leaf"`: fixture payload 1226 B >
`textMaxBytes` 512; `JSON.parse(slices[0])` throws; `sectionValueFromBuiltNode` round-trips
the full packed leaf. Filter `PACKED carrier|reserved refresh sections|chunked text carrier|not valid JSON`:
**4 passed** / 749 skipped. Foreign suite fail: `package-integration` `self is not defined`.

#### Law tails

```
reserved_refresh_section_payloads_admit_into_the_retained_section_carrier
[DEBUG] puzzle3d engagements section payload is 529 bytes
[DEBUG] puzzle3d measures section payload is 10462 bytes
[DEBUG] puzzle3d tools section payload is 2581 bytes
ok
```

(Engagements 529 > 512 — this is the truncation that was red. Measures 10462 and tools 2581
also exceed one slice.)

```
cargo test -p semio-framework-plugin --lib -- nakagin_scale_world3d_surface_fits_document_node_cap a_100kib_measures_section_fits_document_node_cap world3d_scene_surface_pages_one_oversized_lane reserved_section_carrier_pages_a_payload
test result: ok. 4 passed; 0 failed
[DEBUG] 100 KiB measures section presented 6 nodes for 69601 bytes
[DEBUG] Nakagin-scale world-3d surface presented 17 nodes (cap 128)
[DEBUG] section carrier packed 34801 bytes into 68 text leaves at depth 1 (4 nodes)
[DEBUG] one 114001-byte instances lane packed into 7 text leaves at depth 1
```

Editor laws live behind `--features component-app-assembly` (without it, `--lib` lists only
the 286 schema tests). Binary size with the feature: **640** tests.

```
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
test result: FAILED. 639 passed; 1 failed
```

Vs last honest **638/2**: `reserved_refresh_section_payloads_admit_into_the_retained_section_carrier`
flipped green (engagements 529 B > 512). The remaining red is foreign
`two_instances_converge_disjoint_object_edits_via_backbone` (fail-closed remote snapshot merge).

#### Checks

```
cargo check -p semio-framework-ui-contract -p semio-framework-os-renderer-wgpu -p semio-framework-plugin -p semio-s-artifact-puzzle-3d --features semio-s-artifact-puzzle-3d/component-app-assembly -j 4
    Finished `dev` profile [unoptimized] target(s) in 39.05s
cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle -j 4
    Finished `dev` profile [unoptimized] target(s) in 38.81s
```

`bun x tsc --noEmit` in renderer-react: **zero new** errors in `packed-text.ts` or
`PluginRuntime/🟦️.tsx`. Pre-existing: `Interpreter/🟦️.tsx:1409` `ImportMeta.dir`;
unknown-component-placeholder `UiDocumentStore` as type; docklayoutstore suite.

Wasm was **not** rebuilt; `:6013` was **not** probed.

#### Coordinator probe (after next wasm, number after #30)

In addition to the capacity bullets above:

6. Shell **boots** (windows appear; not 0 / 180 s).
7. **Zero** repeating `sectionValueFromBuiltNode` `SyntaxError: Unterminated string in JSON at position 511`.

### What the coordinator's browser probe still needs

After the next wasm (number after #29) and `:6013` — **do not rebuild here**:

1. Repeat the #29 tour: `bun 🔍️browser-probe.ts --interact --example=Nakagin`.
2. Coalesced Publish still holds (handful of ingresses, one undo).
3. **Zero** `SurfaceReconcileFault::Credits` on `1:puzzle3d-main` (must be ≪ 128 nodes),
   `1:framework.panel.artifact` (must be ≤ 8 MiB), and `1:framework.section.measures`.
4. Both Top and Perspective show Nakagin, not Concrete Forest.
5. Artifact panel still virtualized (continuation rows); hide/lock still invert.
