# W5 — Component-level Storybook coverage for the `📸️remodel` scope

## 1. Scope registration

`.storybook/scopes.ts` had no remodel row. Added one to **`HAND_CURATED_SCOPES`** (after `puzzle/5d`, before
`framework`) — `id: "remodel"`, `titlePrefix: "📸️remodel"`, `sourceRoots: ["✏️s/🔌️plugins/📸️remodel"]`.
Block/cad/animate declare theirs via `[package.metadata.semio.storybook]` in their own `Cargo.toml`
(`GENERATED_SCOPES`); remodel's `📦️packages/🦀️rust/Cargo.toml` is owned by another worker this ticket, so the
hand-curated list is the right home for now — the code comment says so and names the migration.

Verified live, not assumed:

```
$ bun -e '…resolveActiveScopes("remodel") / buildScopeStoryGlobs…'
active: [{"id":"remodel","titlePrefix":"📸️remodel","sourceRoots":["✏️s/🔌️plugins/📸️remodel"]}]
globs:  ["./stories/remodel/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)"]
```

## 2. Files added

| Path | Role |
|---|---|
| `.storybook/stories/remodel/fixture.ts` | Story-local **copies** of the populated document (the shared `🧬️mutations/*/🧪️tests/*/📸️snapshot/⬅️before/🔣️.json`, identical across all 34 cases), of `default_remodeling_scene()` (= the shipped `📚️examples/🎬️demo` DSL, which is an EMPTY scene), of `RemodelingConfig::default()`, and of every `app_labels! { RemodelingLabels }` row in **both** native locales. Copied, not `?raw`-imported — the plugin tree is being renamed under us. |
| `.storybook/stories/remodel/scene.ts` | One projection per window kind (line-for-line against each Rust `render`), one `BuiltNode` document per panel tab, and `reduceRemodelStoryAction` (the config-only `command_from_action` subset). |
| `.storybook/stories/remodel/Model.stories.tsx` | `World3dHost` ⇢ `remodeling-main` — 2 stories |
| `.storybook/stories/remodel/Frames.stories.tsx` | `Canvas2dHost` ⇢ `remodeling-frames` — 3 stories |
| `.storybook/stories/remodel/Report.stories.tsx` | `TableHost` ⇢ `remodeling-report` — 6 stories |
| `.storybook/stories/remodel/Viewer.stories.tsx` | `World3dHost` ⇢ `remodeling-view-model` (viewer) — 2 stories |
| `.storybook/stories/remodel/Panels.stories.tsx` | Real `InterpretedUiNode` over all 7 panel tabs — 10 stories |
| `.storybook/stories/remodel/Modes.stories.tsx` | capture / model / analyze — 3 stories |
| `…/REMODEL-PLUGIN-END-TO-END/tsconfig.w5-remodel-stories.json` | Ticket-local tsc program scoped to the new files (block precedent; no `vite/client` needed since nothing is `?raw`/`?url`-imported). |

`fixture.ts`/`scene.ts` are not `*.stories.*`, so the CSF indexer never treats their exports as stories.
**26 stories total.** Window kind ids are `remodeling-*`, not `remodel-*` (the task brief's shorthand).

## 3. Story list and what each renders

- **`📸️remodel🧊️Model`** — `PopulatedScene`, `BootDocument`. `World3dHost` on the projected `World3dScene`:
  four point-cloud layers (sparse, dense, recovered camera poses, GCP world positions), each toggleable from a
  real `setLayerVisibility` dispatch, plus camera round-trip via `setCamera`. Plain three.js/r3f — no wasm, it
  renders for real.
- **`📸️remodel🖼️Frames`** — `PluginPayloadUnsetCursor`, `PluginPayloadCursoredFrame`, `HostShapedLayers`.
  `Canvas2dHost` (pure `JsonLayersCanvasSession`, no wasm), frame cursor switchable across all four
  (stream, frame) targets via `setFrameCursor`.
- **`📸️remodel📊️Report`** — `FrameList`, `Cameras`, `GroundControlPoints`, `Tracks`, `QcStages`, `Matches`.
  `TableHost` over `report_table_json`'s six datasets (plus a `"nonsense"` button proving the frame-list
  fallback), dataset switched by a real `setReportTable` dispatch; sorting/selection round-trip for real.
- **`📸️remodel👁️Viewer`** — `PopulatedScene`, `BootDocument`. The read-only fourth window: hardcoded camera,
  no layer toggles, every layer unconditionally on; `onAction` is a recorder only (`ViewCommand::Noop`).
- **`📸️remodel📌️Panels`** — `DocumentPipeline`, `Media`, `Results`, `Parameters`, `Calibration`, `Tracks`,
  `Quality`, `QualityGerman`, `TracksEmptyState`, `QualityEmptyState`. Each panel is projected to a `BuiltNode`,
  minted by the framework's own `builtNodeToSnapshot`, loaded into a real `UiDocumentStore` and rendered by the
  real `InterpretedUiNode` — the shell's actual interpreter, no wasm. An en/de button re-renders every line
  through the other label column (`setLocale`).
- **`📸️remodel🕒️Modes`** — `Capture`, `Model`, `Analyze`: the window each mode's layout puts in the main slot.

Light/dark and en/de: theme and chrome locale come from `preview.tsx`'s global `withTheme`/`withLocale`
decorators (as in block) — no story-authored colors anywhere, only token-free structural styles, so both themes
paint correctly. The plugin's *own* label set is locale-switched inside the panel stories as above.

`data-testid` hooks for a future Playwright spec: `remodel-model-debug`, `remodel-model-layer-<layer>`,
`remodel-frames-debug`, `remodel-frames-cursor-<id>`, `remodel-report-debug`, `remodel-report-table-<name>`,
`remodel-viewer-debug`, `remodel-panel-<id>`, `remodel-panel-debug`, `remodel-panel-locale-<tag>`,
`remodel-mode-<mode>-debug`.

## 4. Whole-shell boot story and the generated matrix — untouched, still covering remodel

- `.storybook/stories/framework/os/plugins.stories.tsx:53` `export const Remodel` — **not edited**, still present.
- `.storybook/os-plugins.spec.ts` never names remodel literally; it iterates `PLUGIN_BUILD_TARGETS`, and
  `🤖️generated/🧩️plugins.ts:70` still carries `{ pluginId: "remodel", … }` — confirmed by grep. So remodel keeps
  its automatic boot-outcome + zero-console-error coverage.

## 5. Verification

- **Scope resolution — PASS.** The `bun` run in §1.
- **Parse — PASS.** All 8 files transpile clean (`bun build --no-bundle`, "Transpiled file" on each, zero errors).
- **TypeScript — RUN PENDING.** The 1-min load average was 62 → 66 → 80 → 91 across the session, never below the
  ticket's 60 gate, so no `tsc` was started (it type-checks the whole transitive framework graph, minutes of CPU).
  Command to run when the box is free:
  `bun x tsc --noEmit -p .🧬semio/…/REMODEL-PLUGIN-END-TO-END/tsconfig.w5-remodel-stories.json`
  then read only the errors whose path contains `.storybook/stories/remodel/` — block's equivalent run showed
  1172 PRE-EXISTING errors in imported framework modules, which are not ours.
- **No story has been seen rendering.** No Storybook build, no screenshot. Everything above is source-verified
  (host contracts, export lists, layer schemas, label columns) or `bun`-verified — nothing here claims a pixel.

## 6. What the remodel UI layer lacks that block's stories relied on

1. **`remodeling-frames` emits a layer schema `Canvas2dHost` cannot read** (highest). `frames_layers_json`
   (`✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs:38-67`) keys its layers `"type": "image"` / `"type":
   "points"` and shapes the point list as `[{x, y, label}]`. `JsonLayersCanvasSession.renderFrame`
   (`🧱️elements/📐️Canvas2dHost/🟦️.tsx:486-540`) keys on `kind` (`"image"`/`"polyline"`/`"circle"`/bounds/line)
   and expects `points: [[x, y], …]`. The plugin's payload therefore falls through to the bounds/no-bounds
   fallback and draws nothing recognizable. The `PluginPayload*` vs `HostShapedLayers` stories put the two side
   by side. This is a real runtime defect, not a story limitation.
2. **No document in the repo can render a mesh or a frame image.** Both `world_meshes_json` and
   `remodeling_asset` resolve composed `s.stdio.semio@v1` CHILD handles through `scene.durable_artifacts`, and
   neither the fixture document nor `default_remodeling_scene()` carries that map — so the mesh is `"[]"` while
   the instance is still emitted (gated only on `config.layers.mesh`), and the frames window never gets its
   image layer. Block's stories, by contrast, had real GLBs to resolve through the mesh catalog.
3. **No `UiNode` → retained-document adapter in TypeScript.** The panels still emit the legacy `UiNode`
   (`ui_stack_vertical`/`ui_text`/`ui_import_drop_zone`); `builtNodeToSnapshot` is the only entry into
   `UiDocumentStore` and it starts from `BuiltNode`. The Stack→`container(plain)` / Text→`text` / drop-zone→
   `container` + `dropOverlay` + `Trigger::Drop` mapping in `scene.ts` is the story's own, and the styling/layout
   tokens are story defaults (the legacy `UiNode` carries none). Everything else — text, structure, action id,
   accept list — is the plugin's.
4. **No example is reachable, so there is nothing to switch between.** Remodel has no `setActiveExample` command
   and its one example document is empty anyway, so there is no remodel counterpart to block's
   `AllExamples`/fixture-matrix stories; `BootDocument` vs `PopulatedScene` is the honest substitute.
5. **The report window's columns are not `sortable`.** `report_table_json` emits `{id, label}` only, so
   `TableHost`'s sort is dead in the real app; the story projection adds the flag so the interaction is
   exercisable, and says so.
6. **Pre-existing, out of scope, unchanged:** the stale puzzle stories and the stale `framework/hosts`
   `UiInterpreter` story (`interpretUiNode(node, {onAction})` vs today's `(store, context)`) that block's report
   flagged are still stale. The remodel panel stories deliberately use the current two-argument shape.
