# Runtime defects — retirement trap + surface wire bound (2026-09-09)

Lane: runtime-defects (Opus). Fixes the two defects boot #1 exposed
(`📓️runtime-verification-2026-09-09.md`): the plugin actor trapping on its first `pending_effects`,
and the one fault card that replaced every window body.

Private target dir `$S/target-rtfix` (APFS clone of the editor lane's warm `debug/`),
`RUSTC_WRAPPER=""`, `--keep-going`. Raw logs in `🗑️generated/`.

---

## 1. Defect 1 — `ordered-map root must be explicitly retired before drop`

### 1.1 Root cause

`OrderedMap<V>` (`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`) is a `#[must_use]` root whose
`Drop` **asserts** its tree is already empty:

```rust
impl<V> Drop for OrderedMap<V> {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.root.is_none(), "ordered-map root must be explicitly retired before drop"); } }
}
```

`FlowFixture.layout` is an `OrderedMap<WidgetLayout>`, and **every bundled generation3d example
declares a non-empty `layout={…}` block** (e.g.
`📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/…/🗣️.dsl.semio:6`). So any owned `FlowFixture` —
and anything that transitively owns one — panics the moment it is dropped instead of retired.

`semio_framework_os_flow::flow_host_with_session`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2680`) does
`FlowHost::from_fixture_with_cache(fixture.clone(), …)` — it **clones** the document fixture into the
returned host. generation3d built such a host in ~20 places and dropped every one of them. The first
one the runtime reaches on boot is:

```
✏️editor/🦀️.rs  Generation3dPlayApp::pending_effects
  └─ with_scratch_session(|session| { let host = flow_host_with_session(&doc.snapshot.fixture, session); … })   ← host dropped here
       └─ drop glue FlowHost → drop glue FlowFixture → OrderedMap<WidgetLayout>::drop → panic → wasm `unreachable` → actor procedural#1 trapped
```

Everything after that turn failed with `runtime instance authority is busy`, which is why
`setActiveExample` and every local interaction observation also reported a fault.

`FlowHost` has a full retained retirement ladder (`FlowHostRetirement`, same file) — it was simply
never reachable as a cold one-shot, and nothing in generation3d used it.

### 1.2 Fix

**Framework (`🌊️flow/🖥️host/🦀️.rs`)**

| addition | why |
|---|---|
| `FlowHost::retire_cold(self)` | the cold twin of `FlowFixture::retire_cold`; drains `FlowHostRetirement` in one pass |
| `FlowHost::with_fixture(&FlowFixture, body)` | scoped build-use-retire, so a host cannot outlive its scope |
| `FlowHostRetirement::close_page` → `pub` (was `pub(crate)`), `FlowHostRetirementFault` → `pub` | a plugin's `InteractiveJobCloseStep` ladder can now drive the host close under its own grant without minting a `StepContext` |

**generation3d / generation2d**

`host_from_fixture(&fixture) -> FlowHost` and `host_from_fixture_with_session(...) -> FlowHost` are
**deleted**. They are replaced by scopes that cannot leak:

```rust
pub fn with_host<R>(fixture: &FlowFixture, body: impl FnOnce(&mut FlowHost) -> R) -> R
pub fn with_host_session<R>(fixture: &FlowFixture, session: &mut FlowEvalSession, body: impl FnOnce(&mut FlowHost, &mut FlowEvalSession) -> R) -> R
```

Converted call sites (all of generation3d + generation2d):

| site | file |
|---|---|
| `pending_effects` (the live trap) | `🧊️generation3d/…/✏️editor/🦀️.rs:1369` |
| `generation3d_port_ids_by_node` (interaction topology) | `…/✏️editor/🦀️.rs:210` |
| `export_mesh_from_document` | `…/✏️editor/🦀️.rs:2313` |
| `flowEvalTick` (`take_pending_extension_eval` then `retire_cold`) | `…/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs:16` |
| retained `Generation3dPreviewCommandWork` (`self.host.take()` used to DROP the host) | `…/✏️editor/🦀️.rs:300-405` |
| retained `Generation3dViewCommandWork` (same defect) | `…/👁️viewer/🦀️.rs:125-232` |
| `generation3d_view_port_ids_by_node` | `…/👁️viewer/🦀️.rs:305` |
| `evaluate_fixture` (view preview window) | `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs:283` |
| node-graph window `render` | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:57` |
| `commit_fixture` (the host-normalized baseline was dropped) | `…/🧬️schema/🦀️.rs:361` |
| `evaluate_generation_preview` | `…/🧬️schema/🦀️.rs:405` |
| 10 command handlers: add/remove-widget, delete-selection, node-graph-edit, reorganize, move-media-node, patch-flow-widgets, translate/rotate/scale-selection | `…/✏️editor/🎮️commands/**` |
| generation2d: `with_host`/`with_host_session`, `host_operations`, `generation_preview_host` consumer, `flowEvalTick`, `pending_effects`, `add-widget`, node-graph window, retained `Generation2dPreviewCommandWork` | `🌀️generation2d/**` |

Two retained `Work` types previously did `self.host.take();` in `close_step`, which drops the host.
Both now move it into a `FlowHostRetirement` and drive `close_page(maximum_items, maximum_bytes)`
under the caller's grant, and their `terminal_is_empty` covers the new field.

**Owned projections.** The same law applies one level up: `Generation3dSnapshot` owns the fixture, so
`set_active_example::emit`'s freshly-loaded `example_snapshot(...)`, `example_document_json`,
`generation3d_document_from_mesh` and `generation3d_export_mesh` all dropped a projection. Added
`Generation3dSnapshot::retire_cold()` (`🧬️schema/📸️snapshot/🦀️.rs`) — `fixture.retire_cold()` +
`generation.retire_cold()` — and closed those four sites with it.

### 1.3 Making the editor suite runnable

**Before this lane the suite executed exactly ONE of its 295 tests** — it aborted (`SIGABRT`) inside
the first one. It now runs to completion. Four distinct causes, all instances of the same law:

1. **`interactive-job.catalog-authority` → abort.** `testkit::app()` used the registryless
   `semio_framework_plugin::testkit::new_app`. generation3d publishes
   `bounded_first_step_tool_proofs!` factories, which that constructor cannot satisfy, so `app()`
   panicked at construction; the unwind then hit `ArtifactStoreCursorDisposer::drop`
   (`🏪️store/🦀️.rs:2117`, an **unguarded** assert), giving a double panic and `SIGABRT`. `app()` now
   delegates to `app_with_registry()`.
2. **A plainly-dropped app fixture.** `VcsArtifactApp` owns an `ArtifactStore` whose disposer asserts
   terminal-empty ownership, and the projection behind it owns the ordered layout root. Added
   `Generation3dAppFixture` (`✏️editor/🧪️tests/🔬️testkit/🦀️.rs`): derefs to the app, and on `Drop`
   drains the real `PluginApp::close_step` ladder — the same law the runtime uses.
3. **Owned projections in tests.** `VcsArtifactApp::snapshot()`/`ArtifactStore::snapshot()` hand back
   an **owned** `Generation3dSnapshot`; 55 test sites dropped one. Added
   `Generation3dSnapshotRead` (`#[cfg(test)]`, `🧬️schema/📸️snapshot/🦀️.rs`) — derefs to the
   projection, retires it on drop, delegates `PartialEq`/`Debug` — plus `testkit::snapshot(&app)` in
   both the editor and the viewer testkit, and rewrote every site to use it. Also retired the
   directly-parsed example projections in the editor/viewer unit tests, and routed the two
   registryless `VcsArtifactApp::new` tests and `assert_two_instances_converge` onto their
   registry-backed twins.
4. **Unguarded `assert!` inside `Drop`.** THE reason a single failing test killed the binary:
   `🏪️store/🦀️.rs` carried **44** fail-closed `Drop` asserts with no `std::thread::panicking()`
   guard (`ArtifactStoreCursorDisposer` at `:2117`, the displaced-owner authority at `:1753`, and 42
   siblings), plus one in generation3d's own `Generation3dMutationSession`
   (`🧬️mutations/💾️binary/🦀️.rs:1841`). Any test that panicked while holding a store hit a second
   panic in a destructor during cleanup — a **non-unwinding abort**, which discards the failure
   report and every later test. All 45 now read
   `assert!(std::thread::panicking() || (<witness>), "…")`, exactly the guard `OrderedMap::drop`,
   `FlowRetirement`, `RootRetirement` and `ArtifactStore`'s own shell assert (`:17664`) already
   carried. Fail-closed behaviour on a genuine live drop is unchanged.

### 1.4 New tests

`✏️editor/🧪️tests/🔬️unit/🦀️.rs`:

- `the_first_turn_sequence_retires_every_flow_host_it_builds` — `pending_effects` + a render of all
  eight bodies + two `setActiveExample` fixture replacements + the retained
  `Generation3dInstanceOperationOwner`'s own `close_step` to `Complete`, **in one process**.
- `every_window_and_panel_surface_fits_the_resident_surface_bound` — see §2.4.

---

## 2. Defect 2 — `plugin.internal: surface context exceeds its wire bound`

### 2.1 What is measured, and against what

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30154`, in `plugin_mount_surface` — the
handler for `Event::SurfaceVisible`:

```rust
if view_state.len() > MAX_PUBLIC_ACTION_BODY_BYTES || body_key.len() > MAX_PUBLIC_ACTION_STRING_BYTES {
    return Err(plugin_internal_fault("surface context exceeds its wire bound"));
}
```

- `view_state` = the **pack-encoded `ViewModel`** the shell sends with the surface-mount event
  (React target: `🧵️backbone-worker.ts:2171`, `encodePackValue(windowViewContext(hostView, windowId))`).
- `body_key` = the window/panel body key.
- `MAX_PUBLIC_ACTION_BODY_BYTES = 262_144`, `MAX_PUBLIC_ACTION_STRING_BYTES = 4_096`
  (`🔌️plugin/🦀️.rs:29555-29556`).

Neither constant has anything to do with a surface view context. They are the **DFF public-action
admission caps**, declared for `validate_public_json_envelope` (hostile *action body* JSON,
`🔌️plugin/🦀️.rs:29686`) and reused here. The message reported neither number, so a boot could not
say which side tripped.

The view context has its own, explicit, **language-neutral** wire contract:
`🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json` — validated on the TypeScript
side by `parseResolvedPluginViewState` (`🛂️manifest/🟦️.ts:810`) and pinned against that same file by
an Ajv oracle (`🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts`). Rust had no mirror of it at
all.

### 2.2 Measurements

| quantity | declared where | value |
|---|---|---|
| `Identifier` maxLength | `🪟️view-context/🧬️schema/🔣️.json` `$defs.Identifier` | 256 characters |
| `panelJson` / `contributionsJson` maxLength | same, `properties.*.maxLength` | 65 536 characters **each** |
| `activeUtilityByWindowId` maxProperties | same | 64 entries |
| `windowInstances` maxItems | same | 64 entries |
| body key length cap (JS side) | `🧵️backbone-worker.ts:1273` `window.bodyKey.length > 256` | 256 characters |
| **old** `view_state` bound | `🔌️plugin/🦀️.rs:29555` (borrowed) | **262 144 B** |
| **old** `body_key` bound | `🔌️plugin/🦀️.rs:29556` (borrowed) | 4 096 B |
| **new** `MAX_SURFACE_VIEW_CONTEXT_BYTES` (derived) | `🛂️manifest/🦀️.rs` | **808 640 B** |
| **new** `MAX_SURFACE_BODY_KEY_BYTES` (derived) | `🛂️manifest/🦀️.rs` | 1 024 B |
| measured capacity-filled context (JSON, ASCII) | `🔬️view-context-capacity` `[STATS]` | 200 519 B |

Derivation (all in the constant expression itself, so it cannot drift from the schema):

```
(5 identifier fields ×256  +  2 long strings ×65 536  +  64 utility entries ×2 ×256  +  64 window instances ×2 ×256)
   × 4 bytes/char (worst-case UTF-8)
 + 267 encoded values × 64 B pack framing
 = 808 640 B
```

Two independent facts make the old constant wrong:

1. `panelJson` + `contributionsJson` alone are 131 072 **characters**; the schema places no
   restriction on their alphabet, so at worst-case UTF-8 width they are 524 288 B — **twice** the old
   262 144 B cap. A context the contract explicitly permits was rejected.
2. The aggregated `contributionsJson` the dev shell builds for the generation3d closure measures
   **303 505 B** today (`buildContributionsJson` over the 11-crate closure's
   `manifest.topicContributions`; `📐️brep` alone is 193 711 B, `🧮️math` 58 503 B) — also above the
   old cap on its own.

**Not yet pinned to a single live field.** A capacity-filled but pure-ASCII context measures
200 519 B (§3.2), i.e. under the old cap; only non-ASCII long-string content, or a `contributionsJson`
of today's size reaching the surface-mount path, crosses it. The shell's browser-actor view context
(`postBrowserActorViewState` → `parseResolvedPluginViewState` → `windowViewContext`) is the producer,
and it is built in the browser, which this lane cannot observe. That is exactly why the fault message
now carries both measured lengths: **the next boot's fault card, if it still appears, names the
offending side and the overshoot**. What is already proven here is that the guard was holding a
surface view context to a constant that is neither derived from nor large enough for that value's own
declared wire contract.

### 2.3 Fix

- `🛂️manifest/🦀️.rs`, new `//#region 📏️ViewContextCapacity`: `VIEW_CONTEXT_IDENTIFIER_CHARS`,
  `VIEW_CONTEXT_LONG_STRING_CHARS`, `VIEW_CONTEXT_UTILITY_ENTRIES`,
  `VIEW_CONTEXT_WINDOW_INSTANCES`, `VIEW_CONTEXT_IDENTIFIER_FIELDS`,
  `VIEW_CONTEXT_LONG_STRING_FIELDS`, and the two derived bounds
  `MAX_SURFACE_VIEW_CONTEXT_BYTES` / `MAX_SURFACE_BODY_KEY_BYTES`.
- `🔌️plugin/🦀️.rs:30154` now holds `view_state` and `body_key` to **those** bounds, and the fault
  **reports the measurement**:

  ```
  surface context exceeds its wire bound: view state {n} B of {max} B, body key {n} B of {max} B
  ```

  so a boot names the offending side and the overshoot instead of only the verdict.
- The public-action constants stay exactly where they belong — `validate_public_json_envelope` and
  the DFF admission tests are untouched.

Nothing was moved behind a handle: the rendered surfaces are not the oversized value here (§2.4), the
view context is, and it is the host that produces it.

### 2.4 Measured surfaces — the rendered trees are NOT the oversized value

All 64 surfaces (8 bundled examples × 5 windows + 3 panels), rendered through the same
`generation3d_render_body` every live window goes through:

| body | min B | max B | bound |
|---|---|---|---|
| `procedural.play.main` | 6 198 | **14 625** | 8 388 608 |
| `procedural.play.preview` | 3 593 | 3 593 | 8 388 608 |
| `procedural.play.catalogue` | 1 732 | 1 732 | 8 388 608 |
| `procedural.play.generate-preview` | 1 208 | 1 208 | 8 388 608 |
| `procedural.play.document` | 504 | 916 | 8 388 608 |
| `procedural.play.generations` | 596 | 596 | 8 388 608 |
| `procedural.play.inspection` | 437 | 437 | 8 388 608 |
| `procedural.play.generate-form` | 103 | 103 | 8 388 608 |

The largest surface uses **0.17 %** of the framework's per-surface capacity. Nothing needed to move
behind a handle, and the mesh encoding was left entirely to the tessellation lane.

### 2.5 New tests

`🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️view-context-capacity/🦀️.rs` (3 tests, language-agnostic
— every constant is read back out of the shared `🔣️.json` the TypeScript mirror and its Ajv oracle
validate against):

- `capacities_match_the_neutral_schema` — each constant equals its schema declaration, and the
  identifier/long-string **field counts** are counted out of `properties` rather than hard-coded.
- `the_admission_bound_covers_every_schema_valid_context` — the bound admits the schema's own
  characters at worst-case UTF-8 width, and is strictly greater than the old 262 144 B cap.
- `a_capacity_filled_context_fits_the_bound` — builds a `ViewModel` filled to every schema capacity
  and asserts the encoding fits, printing `[STATS] capacity-filled view context encoded=… bound=…`.

`✏️editor/🧪️tests/🔬️unit/🦀️.rs`:

- `every_window_and_panel_surface_fits_the_resident_surface_bound` — the five window bodies and the
  three panel bodies, for **all eight** bundled examples (64 surfaces), each measured against the
  framework's real per-surface capacity `ui_contract::UI_RESIDENT_SURFACE_BYTES` (8 MiB, the value
  `ui_runtime::SURFACE_RECONCILE_SURFACE_BYTES` is defined as), printing
  `[STATS] surface example=… body=… bytes=… bound=…`.

---

## 3. Runs

### 3.1 `cargo check -p semio-s-artifact-procedural-generation3d --features component-app-assembly --keep-going`

```
    Checking semio-framework-os-flow v0.1.0 (…)
warning: unused import: `SpaceMember`
  --> 🧰️framework/…/🌊️flow/🖥️host/🦀️.rs:24:108        ← pre-existing, peer's line
warning: `semio-framework-os-flow` (lib) generated 1 warning
    Checking semio-s-artifact-procedural-generation3d v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 59.07s
```

0 errors; warnings present (so this is a real type-check, not an aborted expansion). The one warning
is a peer's unused import in the flow host, not this lane's.

### 3.2 `cargo test -p semio-framework --lib view_context_capacity -- --nocapture`

```
running 3 tests
test manifest::view_context_capacity_tests::the_admission_bound_covers_every_schema_valid_context ... ok
test manifest::view_context_capacity_tests::capacities_match_the_neutral_schema ... ok
[STATS] capacity-filled view context encoded=200519 bound=808640
test manifest::view_context_capacity_tests::a_capacity_filled_context_fits_the_bound ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 219 filtered out; finished in 0.01s
```

### 3.3 `cargo check -p semio-s-plugin-procedural --keep-going` — native and `wasm32-wasip2`

| lane | result |
|---|---|
| native | `rc=0`, **0 errors**, 4 warnings (all peers': `stdio-semio` `Curve2Id`, `os-flow` `SpaceMember`) |
| `--target wasm32-wasip2 --profile wasm-dev` (`CARGO_PROFILE_WASM_DEV_DEBUG=false`) | `rc=0`, **0 errors**, same 4 peer warnings |

Logs: `🗑️generated/rtfix-native-procedural.txt`, `🗑️generated/rtfix-wasm-procedural.txt`.

### 3.4 generation3d lib suite

```
$ cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1
test result: FAILED. 127 passed; 168 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.74s
```

**No abort.** Before this lane the same command died at test 1 of 295 with `SIGABRT`; it now runs
every test and reports. Both of this lane's new tests pass:

```
test editor::generation3d::component::tests::the_first_turn_sequence_retires_every_flow_host_it_builds ... ok
test editor::generation3d::component::tests::every_window_and_panel_surface_fits_the_resident_surface_bound ... ok
```

Log: `🗑️generated/rtfix-suite-complete.txt`; the surface table is in
`🗑️generated/rtfix-surface-budget.txt`.

The 168 failures are pre-existing debt this lane made VISIBLE, not caused by it — none of them
existed as observations before, because the binary never got past test 1. Classified:

| class | count | example / cause |
|---|---|---|
| committed mutation-fixture tests dropping their decoded projections | ~130 (14 mutations × ~5 fixtures × 7 tests) | `🧬️mutations/*/🧪️tests/*/🦀️.rs` `before()`/`expected_after()` hand back an owned `Generation3dSnapshot` and drop it. These files are **generated** by `fixtures generate`; the fix belongs in that template, not in 100+ hand edits. |
| viewer command tests | 9 | `dispatch: interactive-job.missing-factory — typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof` — the viewer's bounded factory publishes no per-tool proof. |
| retained editor commands never driven in tests | ~12 | `setActiveExample`/`addGeneration`/`reorganize`/`translateSelection` are retained jobs; `dispatch_typed` alone does not drive them, so the document never changes (`add_generation … left: 0 right: 1`). |
| stale test tables | 1 | `command_ids_are_unique_and_cover_every_row`: `every_command()` covers 27 of 29 rows — the round-trip lane added `flowEvalResolve`/`flowTessellateResolve` without extending it. |
| real product failures | ~10 | `preview_payload_has_meshes_and_instances`: `meshes_json was empty`; `all_bundled_examples_emit_preview_meshes`; `rectangle_wire_preview_emits_edge_only_mesh`. These belong to the tessellation/example lanes. |
| framework fail-closed refusal | 1 | `two_instances_converge_disjoint_widget_moves`: `attach b: module.vcs — remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized`. |

---

## 4. Concurrency notes for the coordinator

- `semio-framework-os-flow` was mid-refactor by the tessellation lane at 20:0x
  (`PreviewTessellateOutcome`/`preview_mesh_pack_has_geometry` unresolved, 26 errors); it went green
  again before this lane's first successful check. `semio-s-artifact-stdio-semio`
  (`exact_pcurve` private) and `semio-framework-plugin` (`live_render_operation` missing) were each
  transiently broken by peers during this lane's later runs — none of those symbols are touched here.
- The scratchpad volume hit **0 bytes free** mid-lane (six ~90 GB private target dirs). Removed the
  three stale ones (`target-rt`, `target-extest`, `target-bool` — no lock holders, owning lanes
  reported done); `target-tess` was live and left alone.
- The mesh encoding was **not** touched: the tessellation lane owns it.

## 5. Owed / recommended follow-ups

1. `fixtures generate` (`📜️script.ts`) emits the per-mutation fixture tests; its template must retire
   the projections `before()`/`expected_after()` hand back, or regenerate them through a
   self-retiring read. ~130 of the 168 failures collapse with that one template change.
2. The viewer's bounded command factory publishes no per-tool proof
   (`interactive-job.missing-factory` on every viewer action) — editor-gaps/viewer lane.
3. `every_command()` in `✏️editor/🧪️tests/🔬️unit/🦀️.rs` is two rows behind `Generation3dCommand`.
4. The next boot's fault card, if it still appears, now names the measured bytes and the offending
   side — paste that line into the boot report.

## 6. Files changed

Framework:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowHost::retire_cold`,
  `FlowHost::with_fixture`, `close_page`/`FlowHostRetirementFault` made public.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs` — `flow_backed_node_graph_extras`
  built a host per node-graph render and dropped it (the second live leak, on every main-window
  repaint of generation3d, generation2d and flow).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` — 44 fail-closed `Drop` asserts guarded
  against unwinding.
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` — `📏️ViewContextCapacity` region.
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️view-context-capacity/🦀️.rs` — **new**.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `plugin_mount_surface` bound + message.

generation3d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

- `🧬️schema/🦀️.rs`, `🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
- `✏️editor/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`
- `✏️editor/🎮️commands/{🎨️set-active-example,✏️node-graph-edit,❌️delete-selection,➖️remove-widget,🧩️add-widget,🗺️reorganize,🚚️move-media-node,🩹️patch-flow-widgets,↔️translate-selection,🔄️rotate-selection,📏️scale-selection,⏱️flow-eval-tick}/🦀️.rs`
- `👁️viewer/🦀️.rs`, `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`
- tests: `✏️editor/🧪️tests/{🔬️testkit,🔬️unit}/🦀️.rs`, `👁️viewer/🧪️tests/{🔬️testkit,🔬️unit}/🦀️.rs`,
  `🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, `✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`,
  and the per-command `🧪️tests/🔬️unit/🦀️.rs` of add-generation, remove-widget, reorganize,
  set-active-example, set-lod-mode, toggle-sun, translate-selection, plus the seven viewer command
  test files.

generation2d (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

- `🧬️schema/🦀️.rs`, `✏️editor/🦀️.rs`,
  `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`,
  `✏️editor/🎮️commands/{⏱️flow-eval-tick,🧩️add-widget}/🦀️.rs`
