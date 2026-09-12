# Examples Belong To The Dialect — Viewer Example Picker + `setActiveExample` (2026-09-12)

Closes the gap in `📓️audit-examples-2026-09-12.md` §2 / `📓️viewer-eval-chain-2026-09-12.md` §3: the
generation3d **viewer** had no example picker and no way to open one, because an example was a
property of ONE app (`ExampleDefinition.app_id`) instead of the artifact's dialect.

Raw command output: `🗑️generated/viewer-examples/`.

---

## 1. What changed, in one sentence per layer

| layer | change |
|---|---|
| schema | `ExampleDefinition.app_id: String` → **`dialect: ArtifactDialect`** (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`), mirrored in the typegen table and the generated TS |
| framework Rust | new `manifest::examples_for_dialect` / `examples_for_app`; `App.examples` is now `Vec<ExampleSource>` and `Plugin::register_app_factory` stamps each with the registering surface's own `AppDefinition.dialect` |
| framework TS | new `examplesForDialect`/`examplesForApp` + `ManifestExample` (`🛂️manifest/🟦️.ts`); `exampleArtifactSources` is dialect-keyed and lost its app-id-stem fallback |
| host | `ShellHost`'s `exampleOptions` resolves through `examplesForApp(manifest.examples, session.app)`; the contributions scoping call passes `session.app.dialect` |
| viewer | new `setActiveExample` command + config leaf + `Generation3dViewedDocument` resolution on every read path |
| tests | new language-agnostic `🧫️fixtures/📚️example-picker.json` + `ExamplePickerFixture` schema, answered by a Rust law AND the TS twin; four new viewer laws; existing viewer laws extended |

### 1.1 Why the dialect and not the app id

`SubsetDeclaration.examples` (`🔌️plugin/🦀️.rs`) already models examples as a property of the SUBSET,
and `exampleArtifactSources` already had to fall back to `appId.split("#")[0]` — the dialect
coordinate — to make the viewer see the editor's graphs. Both were symptoms of the same thing: an
example is a *document of an artifact's subset*, and every surface bound to that subset opens it.
`ExampleDefinition.dialect` states that directly; `surface_app_id(dialect, role)` already guarantees
an app id contains its dialect, so nothing is lost. There is deliberately no
`From<ExampleSource> for ExampleDefinition` any more: a source alone cannot know its dialect, and an
unstamped row would be a manifest entry no picker could resolve.

`editor_with_examples` keeps its name and signature — the editor row is still the one registration a
subset's fixtures travel with, exactly as `SubsetDeclaration` does it — but its stamp is now the
dialect, so the **same eight sources serve both surfaces with no duplication**.

## 2. The viewer's `setActiveExample` — loading, not mutating

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs`

The editor's switch replaces the artifact's fixture through the document lane and records an
undoable operation. A viewer owns no write authority at all (`ViewEmit` has only config/effects/
ui_dirty), so the read-only twin:

1. writes the picked id onto the surface's **own config** — new leaf
   `👁️viewer/🎚️config/🧬️schema/🧬️mutations/🎨️set-active-example/` (binary tag 5, opcode
   `active-example`) and new field `Generation3dViewConfig::active_example_id`, mirrored in all six
   schema surfaces (`🔣️.json`, `🟦️.ts`, `📜️.wit`, `🔗️.graphql`, `🛰️.proto`, `🧬️schema/🦀️.rs`);
2. resolves that id to the example's projection in ONE place —
   `Generation3dViewedDocument::resolve` (`👁️viewer/🦀️.rs`) — which every read path now goes
   through: `render`, `render_with_request_context`, `interaction_topology` and the
   `flowEvalTick` window work. `Example` owns a real `Generation3dSnapshot`, so every caller ends
   with `retire()` (a bare drop aborts the process on the neural `Dictionary` roots);
3. re-arms the attached preview chain through the **shared** `preview_eval::rearm_attached_previews`
   (read-only use of the peer lane's module), so the switch's consequence never depends on a host
   `refresh-ui` round trip;
4. refuses an id the dialect never published rather than silently blanking the view.

Routing: its own tool-id list `GENERATION3D_VIEW_EXAMPLE_TOOL_IDS`, its own
`Generation3dViewExampleJobFactory` (payload schema `generation.3d.view-example-command.v1`, 8 KiB,
`Migrated`) with publication lanes **Config + Presence + Transient and nothing else**, and its own
bounded first-step proofs. It is kept out of `GENERATION3D_VIEW_TOOL_IDS` because that list is the
statement of what this viewer's ONE window dispatches; `setActiveExample` is an app-scoped navbar
verb, and `build_definition` copies an unowned action onto every window kind, which is what carries
it past `ShellHost`'s `declaredAction` gate.

**`ActionKind::View`, never `Mutation`** — the sibling surface declares `Mutation`, but `ShellHost`
refuses any `mutation`-kind action on a viewer session outright (`🏛️ShellHost/🟦️.tsx` read-only
gate), so a `Mutation` row here would show the picker and then swallow every pick.

### 2.1 Not touched (peer lanes)

`pending_effects` is owned by the `tick-arming-latch` lane and was left alone — it still gates on
`doc.snapshot.fixture`, so the FIRST arming after a boot is decided by the opened document while the
tick itself evaluates the viewed one. Harmless today (both carry brep kinds) but worth folding into
`Generation3dViewedDocument` when that lane settles. `with_scratch_session` is now dead code from
that lane's rewrite (one `dead_code` warning, not mine).

## 3. Tests — language-agnostic first

### 3.1 `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json` (new)

Five example rows (including a duplicate id and a sibling-subset row) and five cases: editor of the
dialect, **viewer of the same dialect (identical expectation)**, another artifact's surface, a
sibling subset, and a dialect with no authored example. Each case's `app.id` must equal
`surface_app_id(dialect, role)`, so a row can never claim a coordinate its own id contradicts.
Schema: `$defs/ExamplePickerFixture` in `🛂️manifest/🧬️schema/🔣️.json` (Ajv-validated by the TS twin).

Answered by two implementations:

- Rust `🛂️manifest/🧪️tests/🔬️example-picker/🦀️.rs::every_surface_of_a_dialect_resolves_the_same_example_picker`
- TypeScript `🔬️engine-contract/🟦️.ts::"resolves the example picker by dialect, so an editor and its viewer offer exactly the same examples"`

### 3.2 Commands run and their results

```
RUST_MIN_STACK=33554432 cargo test -p semio-framework --lib -- example_picker_tests:: --nocapture
```
→ **1 passed, 0 failed.**

```
RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib -- example_source_tests:: --nocapture
```
→ **2 passed, 0 failed.**

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-plugin-procedural --lib -- manifest_examples --nocapture
```
→ **2 passed, 0 failed** —
`generation3d_manifest_examples_are_registered_on_the_dialect_for_both_surfaces` asserts all eight
ids, in authored order, resolve for `…#editor` AND `…#viewer`.

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib -- viewer --test-threads=1 --nocapture
```
→ **47 passed, 0 failed** (`🗑️generated/viewer-examples/viewer-suite.txt`). Key evidence:

```
[STATS] viewer viewed-document switch examples=8 opened=["height","radius","sides","profile","extrusion-axis","extrude","column-preview"]
[STATS] viewer example rectangle-extrude-volume nodes=["width","height","distance","rect","vector","extrude","volume"]
[STATS] viewer example sphere-cut-with-torus  nodes=["slider_2","brep_prim3d_sphere_3","brep_prim3d_torus_4","brep_bool_cut_5","brep_measure_volume_2","preview_3"]
[STATS] viewer example box-fillet-preview     nodes=["size","radius","box","fillet","preview"]
[STATS] viewer example sphere-box-fuse        nodes=["radius","size","sphere","box","fuse","preview"]
[STATS] viewer example face-sweep-extrude     nodes=["width","height","distance","rect","face","vector","extrude"]
[STATS] viewer example rectangle-wire-preview nodes=["width","height","rect"]
[STATS] viewer example box-shell-preview      nodes=["size","thickness","box","shell"]
[STATS] viewer setActiveExample "box-shell-preview" lanes=[Config, Presence, Presence, Effect, Ui, Terminal] armed=["view-preview"]      (× all 8 + the empty id)
[STATS] viewer switch latch first=["view-preview"] second=[]
[STATS] viewer setActiveExample refusal=registered fixture typed operation fault: retained command reducer rejected operation
[STATS] window-actions kind=procedural-view-preview declared=27 emitted=7
```

New viewer laws (`👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs`):
`every_bundled_example_switches_the_viewed_document` (all 8, structurally distinct graphs; the hex
column legitimately equals the opened document because it IS the default),
`every_bundled_example_dispatches_live_rearms_the_preview_and_never_mutates_the_document` (fresh
surface per example: Config lane yes, Artifact/Draft lanes never, `view-preview` re-armed, document
byte-identical), `consecutive_switches_arm_once_until_the_chain_answers` (pins the peer lane's
per-window latch), `an_unpublished_example_id_is_refused`, `an_unattached_surface_arms_no_tick`.

Extended existing laws: `every_viewer_action_dispatches_live_and_never_mutates_the_document` now
includes `SetActiveExample` for a published id AND the empty id and still passes;
`every_viewer_tool_id_is_declared_in_all_four_tables`, `no_viewer_tool_publishes_on_the_artifact_lane`
and `every_declared_viewer_action_is_migrated` all chain the new factory/list.

```
cd 🧰️framework/…/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts \
  --testNamePattern="example picker"
```
→ **2 passed, 916 skipped** (the new dialect law + the picker-dispatch law, which now also pins
`buildActiveExampleAction` against the VIEWER controller id).

```
cd 🧰️framework/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts \
  --testNamePattern="exampleArtifactSources"
```
→ **2 passed** (`reads neuron-kind from published example artifactJson`,
`uses the same dialect graphs when the open app is the viewer of that artifact`).

```
cd 🧰️framework/…/🎯️targets/⚛️react && SEMIO_TEST_LEVEL=long bunx vitest run --config vitest.config.ts \
  --testNamePattern="published example graphs"
```
→ **1 passed** (`🩺️window-fault`'s genesis-recovery law, rewritten onto the dialect shape).

```
SEMIO_TYPEGEN_OUT=… cargo test -p semio-framework --features typegen --lib -- exports_typescript_bindings
```
→ regenerated `🛂️manifest/🤖️generated/🪪️manifest.ts`; re-run without the env var → **ok** (the
generated mirror matches the owned metadata table exactly).

```
RUST_MIN_STACK=33554432 cargo check -p semio-s-artifact-procedural-generation3d \
  --features component-app-assembly --lib [--profile test]
```
→ clean. Two warnings, neither from this lane: the pre-existing `unused extern crate … as vcs`, and
`with_scratch_session is never used` left by the tick-arming-latch lane's `pending_effects` rewrite.

### 3.3 Failures that are NOT this lane

- `editor::generation3d::component::fold_contract::set_active_example_publishes_box_shell` →
  `retained command reducer rejected operation`. Reproduced in isolation (23 passed / 1 failed in
  `🗑️generated/viewer-examples/editor-example-switch.txt`). `git diff HEAD` shows the
  `tick-arming-latch` peer is mid-flight across `✏️editor/🦀️.rs` (370 lines), `🔬️fold-contract`,
  `🔬️testkit`, `🔬️work-capacity` and the editor's own `rearm_attached_previews` signature. This lane
  edited **nothing** under `✏️editor/`.
- Repo-wide `bunx tsc --noEmit -p tsconfig.json` from the react renderer package reports 1095
  errors; none falls in a line this lane changed (checked per file: `ShellHost` 1968/1980/8042/8043/
  8750, `🎠️kernel` 3146, `🩺️window-fault` 115/179–181 and `🛂️manifest` 1148 all pre-date it — the
  `as PluginManifest` casts were already missing the required `examples` key, and `import.meta.dir`
  was already unknown).

## 4. Runtime probe — and the restage this lane now REQUIRES

`http://127.0.0.1:6019/?plugin=generation3d` answers `200` and boots, but **neither surface renders
any chrome right now**, so the picker `select` DOM could not be captured:

```
http://127.0.0.1:6019/?plugin=generation3d   →  body: "semio · procedural · 3d / Viewer / Panel /
                                                 view context rejected at refresh-ui: view context: invalid panel data"
                                                 document.querySelectorAll('select')  →  []
                                                 [id^="playground.navbar"]            →  []
http://127.0.0.1:6018/?plugin=generation3d   →  identical banner, "Editor" instead of "Viewer"
```

The banner comes from `parseResolvedPluginViewState` (`🛂️manifest/🟦️.ts:890`, a `panelJson`/
`contributionsJson` > 65 536 code-point guard) reached from `ShellHost`'s `resolvedTargetViewState`
— a path this lane does not touch, and it reproduces identically on the **editor** port, whose
viewer work is untouched. Not this lane's; flagged for whoever owns the view-context/panel budget.

**Restage is required and the pre-restage state is a visible regression.** The served wasm still
emits the pre-change `examples[].appId` shape with no `dialect`, so the host now resolves ZERO
examples for the dialect. Measured live on 6019 after the host change:

```
[DEBUG] contributions document sources {"packBytes":873,"sprBytes":280,"opsChars":92,
  "opsHead":"doc \"s.procedural.generation3d@1/*#viewer\" schema=generation.3d…","status":"unresolved",
  "reason":"no-operator-graph","kinds":[]}
[DEBUG] contributions push skipped unresolved document operators
  {"plugin":"procedural","app":"s.procedural.generation3d@1/*#viewer","reason":"no-operator-graph"}
```

i.e. the example-graph fallback that used to scope the contributions push now finds nothing, exactly
as the empty picker would. Both halves recover on the same restage — the guest must be rebuilt for
`examples[].dialect`, for the viewer's `setActiveExample` command declaration, and for the viewer
config's new `activeExampleId` leaf. No compatibility shim was added (CLAUDE.md: greenfield, no
legacy support). `performInvocation` for `setActiveExample` on the viewer instance therefore has no
runtime line yet; the native proof is §3.2's nine live dispatches with `armed=["view-preview"]`.

The generated per-plugin descriptors (`✏️s/🔌️plugins/*/🔣️.json`, 15 files carrying
`examples[].appId`) are emitted by the describe pipeline and regenerate with the restage; they were
deliberately NOT hand-edited. Nothing in the OS parses `examples` out of a descriptor
(`📇️directory`/`🧵️backbone-worker` never read the field), so a stale descriptor is inert.

## 5. Follow-ups

1. **Restage the procedural plugin** (coordinator), then re-probe 6019 for the `select` options and
   the `setActiveExample` `performInvocation` line.
2. The `view context: invalid panel data` refresh-ui fault blocks BOTH 6018 and 6019 today — needs
   an owner; it is upstream of every runtime proof left in this ticket.
3. `Generation3dViewedDocument::resolve` re-parses the example DSL per read (render, topology,
   tick). Cheap next to a brep solve, but the parsed projection belongs in
   `Generation3dViewInstanceOperationOwner` keyed by example id once the tick-arming-latch lane
   settles — and the same lane's `pending_effects` should gate on the VIEWED fixture, not the
   opened one.
4. `with_scratch_session` (`👁️viewer/🦀️.rs`) is dead after the latch lane's rewrite — theirs to
   remove.

## 6. Files created / changed

**Created**
- `🧰️framework/🔨️modules/🛂️manifest/🧫️fixtures/📚️example-picker.json`
- `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️example-picker/🦀️.rs`
- `✏️s/…/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs`
- `✏️s/…/✳️any/👁️viewer/🎮️commands/🎨️set-active-example/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/…/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/🎨️set-active-example/{🦀️.rs,🔣️.json,🧬️schema/🔣️.json}`

**Changed**
- `🧰️framework/🔨️modules/🛂️manifest/{🦀️.rs,🟦️.ts,🧬️schema/🔣️.json,🤖️generated/🪪️manifest.ts}`
- `🧰️framework/📦️packages/🦀️rust/🦀️.rs` (typegen metadata for `ExampleDefinition`)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/{🦀️.rs,🏗️builder/🦀️.rs,🧪️tests/🔬️app-example-source/🦀️.rs}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/{🏛️ShellHost/🟦️.tsx,🐚️Shell/🟦️.tsx}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/{🔬️engine-contract/🟦️.ts,🩺️window-fault/🟦️.ts}`
- `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️surface/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs` (mounts the new viewer command)
- `✏️s/…/✳️any/👁️viewer/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `✏️s/…/✳️any/👁️viewer/🎚️config/{🦀️.rs,🧬️schema/🦀️.rs,🧬️schema/🔣️.json,🧬️schema/🟦️.ts,🧬️schema/📜️.wit,🧬️schema/🔗️.graphql,🧬️schema/🛰️.proto,🧬️schema/🧬️mutations/🦀️.rs}`
