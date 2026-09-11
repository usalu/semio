# Generation3d user-facing gap inventory — 2026-09-10

Read-only audit of the **current working tree** (reports under this ticket are history; code wins on conflict). Coordinator session ⚪9f5f6952. Prior coordinator log: `📓️status.md` (boot #12 last full runtime pass).

## Executive summary

Edit-mode **procedural-preview** has a complete preview pipeline in source (flowEvalTick → extension evaluate → flowEvalResolve → tessellate → meshes). **Generate-mode preview**, **viewer preview**, and **every browser boot after #12** still have material gaps. The dominant meta-blocker is that **dozens of fixes landed after the last verified restage** (extension-result paging, contributions re-arm, example-switch re-arm, idle-turn reactor, window-kind descriptor, close ladder, flow-host ownership, etc.) — boot #12 still traps on `cabi_realloc` during extension delivery per `📓️extension-result-realloc-2026-09-10.md`.

Native suite (last measured in sibling lanes): generation3d **~334/2** (`📓️contributions-rearm-2026-09-10.md`) after flow-host ownership; **5** remaining reds are framework envelope/VCS/transient-close, not user-preview core.

---

## 1. Windows

### 1.1 Editor — edit mode

| Window | id | Surface | Render path | Actions (window-owned) | Gaps |
|---|---|---|---|---|---|
| Flow | `procedural-main` | `NodeGraph` (`procedural.play`) | `modes/edit/windows/flow/🦀️.rs:55-77` — real `NodeGraphScene` with eval/session | `nodeGraphEdit`, `nodeGraphViewport`, `setLodMode` (`✏️editor/🦀️.rs:1904`) | **Blocker (runtime):** until restage, graph draws but eval chain traps on extension delivery (boot #12). **Degraded:** catalogue rows paginated; continuation rows when panel budget spent (`panels/catalogue/🦀️.rs:78-84`). |
| Preview | `procedural-preview` | `World3d` (`procedural.play.preview`) | `modes/edit/windows/preview/🦀️.rs:65-103` — `World3dScene` + session tessellation | camera, show, sun, gumball transforms (`✏️editor/🦀️.rs:1905-1914`) | **Blocker (runtime):** empty/faulted meshes until extension round-trip + contributions re-arm restaged. **Cosmetic:** `status_json` always injects debug block (`preview/🦀️.rs:72-91`). |
| *(modes)* | — | — | `modes/edit/🦀️.rs` layout only | — | No gaps in source. |

### 1.2 Editor — generate mode

| Window | id | Surface | Render path | Actions | Gaps |
|---|---|---|---|---|---|
| Generations | `generation3d-generations` | `Canvas2d` | `modes/generate/windows/generations/🦀️.rs:34-36` — `generation_tree` | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | **Degraded:** list renders; preview depends on separate window. |
| Form | `generation3d-generate-form` | `Canvas2d` | `modes/generate/windows/form/🦀️.rs:37-43` — `generation_form` | `updateGenerationValues` | Works for form chrome; preview update path broken (see §6). |
| Preview | `generation3d-generate-preview` | `World3d` / fallback `TextEditor` | `modes/generate/windows/preview/🦀️.rs:48-80` | show + sun only (no gumball actions) | **Blocker:** `Generation3dPreviewCommandWork` (`✏️editor/🦀️.rs:421-427`) ticks `FlowEvalSession` **without** emitting `ExtensionInvocation` or tessellate resolves; brep graphs never finish. Render passes `session: None` (`preview/🦀️.rs:65`) so handles never tessellate. Falls back to hint text when meshes empty (`preview/🦀️.rs:71-74`). |

### 1.3 Viewer

| Window | id | Surface | Render path | Actions | Gaps |
|---|---|---|---|---|---|
| Preview | `procedural-view-preview` | `World3d` | `viewer/modes/view/windows/preview/🦀️.rs` | show, LOD, camera, sun (`👁️viewer/🦀️.rs` manifest) | **Blocker:** `Generation3dViewCommandWork` (`👁️viewer/🦀️.rs:200-207`) same synchronous tick without extension/tessellate chain as generate preview. No `flowEvalTick` / `setContributions` in viewer command enum. |

### 1.4 Assembly artifact (authored, not mounted)

| Window | Status |
|---|---|
| Assembly editor/viewer windows under `🗿️artifacts/🧩️assembly/` | **Blocker (feature):** not registered in `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:104-108` — comment documents missing codec bounds. Lane `📓️assembly-mount-2026-09-10.md` in flight (17 taxonomy violations). User cannot open assembly at all. |

### 1.5 Stub / noop audit (windows)

No `todo!`, `unimplemented!`, or `Noop` command arms in generation3d window Rust. `world-pointer-down` / `graph-pointer-down` routes were removed; interaction is framework-owned (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:689-696`).

---

## 2. Panels

| Panel | body_key | Render | Actions | Gaps |
|---|---|---|---|---|
| Document (artifact tree) | `procedural.play.document` | `panels/artifact/🦀️.rs:41-43` | framework `graph` selection (no per-item actions) | None in source. |
| Catalogue | `procedural.play.catalogue` | `panels/catalogue/🦀️.rs:62-86` — paginated | `addWidget` per row | **Degraded:** only first page of operators per group; continuation row names omitted count (`catalogue/🦀️.rs:82-84`). Canvas spotlight carries full catalogue (per catalogue-surface lane). |
| Inspection | `procedural.play.inspection` | `panels/inspection/🦀️.rs:27-100` | `patchFlowWidgets` on slider widgets | None when selection exists; empty state when none. |
| History | *(framework-injected)* | `🔌️plugin/🦀️.rs:5192-5195`, `ui_history_panel` | undo/redo keybindings + framework history verbs | **Cosmetic:** generation3d does not declare a custom history panel; framework default applies. No generation3d-specific history chrome. |

No dedicated generation3d history/catalogue/inspection stubs.

---

## 3. Examples

### 3.1 Bundled picker examples (8) — all reachable

Registered in `examples()` at `✏️editor/🦀️.rs:1963-1973` and mirrored in `setActiveExample` select options (`✏️editor/🦀️.rs:1875-1885`):

| Example id | `.dsl.semio` | `.op.semio` | `.spr` / `.pack.semio` | Picker |
|---|---|---|---|---|
| `hexagonal-mushroom-column` | Real flow DSL | Envelope only (3 lines) | **None** | Yes |
| `rectangle-extrude-volume` | Real | Envelope | None | Yes |
| `sphere-cut-with-torus` | Real | Envelope | None | Yes |
| `box-fillet-preview` | Real | Envelope | None | Yes |
| `sphere-box-fuse` | Real | Envelope | None | Yes |
| `face-sweep-extrude` | Real | Envelope | None | Yes |
| `rectangle-wire-preview` | Real | Envelope | None | Yes |
| `box-shell-preview` | Real | Envelope | None | Yes |

`.op.semio` format (verified `box-shell-preview.op.semio`):

```
semio procedural.generation3d.op v1
edit reuse started="0" actor=example
```

No geometry — **empty envelope**, unchanged from early audit. **No `.spr.semio` or `.pack.semio`** files exist anywhere under generation3d (glob 0). Primary load path is `.dsl.semio` via `include_str` in each example's `🦀️.rs`.

Geometry oracle: `📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — 8/8 pass natively (per `📓️blend-kernel-2026-09-09.md`).

### 3.2 Unreachable example

| Example | Location | Formats | Picker |
|---|---|---|---|
| `demo-session` | `✏️editor/📚️examples/🎬️demo-session/` | `.cmd.semio` only (`action=demo`, 2 lines) | **No** — not in `examples()` or plugin `editor_with_examples` (`🦀️.rs:100` lists 8 DSL examples only). Still **unreachable** from navbar picker. |

### 3.3 Example switch (edit mode)

Source fix landed (`set-active-example/🦀️.rs:55-65` re-arms previews; `✏️editor/🦀️.rs:403-404`). **Blocker (runtime):** unverified until post-#12 restage (`📓️example-switch-2026-09-10.md` boot #11 symptom: label changes, graph unchanged).

---

## 4. Interactions

| Interaction | DOM → command → result | Status |
|---|---|---|
| **Hover (graph)** | `NodeGraph/🟦️.tsx` → `interactionHover` / `nodeGraphActions.hover` → framework interaction store → `PreviewInteractionMarks::from_interaction` → graph highlight + world instance hover | **Source complete** for edit preview + flow window. |
| **Hover (world)** | `World3dHost/🟦️.tsx:4700+` → `interactionSelect`/`interactionHover` with `graph` domain | **Source complete** (React + wgpu after engine-surfaces lane). |
| **Selection** | Same chain; gumball uses `translateSelection`/`rotateSelection`/`scaleSelection` (`World3dHost/🟦️.tsx:1914-1930`) | **Source complete** in edit mode. **Degraded** in generate preview (no gumball actions on window). |
| **Camera orbit/pan/zoom** | World3dHost local camera + `setCamera` on settle | Works when preview has meshes. |
| **Context menu** | `context_menu_with_request_context` (`✏️editor/🦀️.rs:1727-1768`) reads real graph selection | Fixed (was empty selection). |
| **Node drag** | `NodeGraph/🟦️.tsx:1031` → `nodeGraphActions.edit` → `nodeGraphEdit` | Wired (`nodeGraphActions.edit = "nodeGraphEdit"` in `🔺️mesh/🟦️.ts:578`). |
| **Catalogue drag** | `NodeGraph` spawn/drop → `addWidget` | Wired. |
| **Gumball drag** | World3dHost → transform actions → gumball neurons in fixture | Wired in edit preview window only. |
| **Graph editing** | `nodeGraphEdit` retained route | **Blocker (runtime)** if fold/restage issues; source migrated. |

---

## 5. Editor actions

| Metric | Count | Notes |
|---|---|---|
| `Generation3dCommand` rows | **29** | `✏️editor/🦀️.rs:64-93 |
| Retained tool ids | **28** | `GENERATION3D_RETAINED_TOOL_IDS` (`✏️editor/🦀️.rs:259-288`) |
| Contributions tool | **1** | `setContributions` separate factory |
| `InteractiveJobClassification::Migrated` | **29/29** | All `.action_interactive_job(..., Migrated)` (`✏️editor/🦀️.rs:1838-1866`) |
| Viewer commands | **8** | All Migrated (`👁️viewer/🦀️.rs:49-57`) |

**Declared-but-internal (not palette):** `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`, `setContributions` — runtime/host chain, not user menu items.

**Declared with UI affordance but no toolbar button:** `cancelPreviewEval` — exposed via `preview_status_json` `cancelAction` field (`✏️editor/🦀️.rs:2232`), consumed by status chrome when eval is long-running.

**Dead-but-declared:** none found (no `BatchOnly`, no missing `command_from_action` arms).

**Undeclared:** framework interaction verbs (`interactionSelect`, etc.) auto-inject per manifest comment (`✏️editor/🦀️.rs:1826-1828`).

---

## 6. Preview pipeline (edit-mode `procedural-preview`)

| Hop | Source state | User symptom if broken |
|---|---|---|
| 1. Example load | `setActiveExample` → fixture mutations + re-arm (`set-active-example/🦀️.rs:37-52, 55-65`) | Picker label without graph change (fixed in source, unverified runtime) |
| 2. Contributions install | Paged `setContributions` + invalidate + re-arm (`set-contributions/🦀️.rs:54-59`) | `Geometry extension unavailable` forever (fixed in source, boot #11) |
| 3. `pending_effects` / tick arm | `✏️editor/🦀️.rs:1657-1669` — only `procedural-preview` window ids | No eval if preview window not mounted |
| 4. `flowEvalTick` | `flow-eval-tick/🦀️.rs:49-76` — emits `ExtensionInvocation` | Stuck "computing" on first brep node |
| 5. Host extension evaluate | `PluginRuntime` + paging (`PluginRuntime/🟦️.tsx:2343+`) | **Boot #12:** guest abort `cabi_realloc` — **fixed in framework source**, needs restage |
| 6. Pack decode | `extension_response_args` decodes pack (`📓️extension-result-realloc-2026-09-10.md` §4.5) | Mojibake eval if only trap fixed |
| 7. `flowEvalResolve` | `flow-eval-resolve/🦀️.rs:19-23` seeds cache, re-arms tick | Stalled eval |
| 8. Tessellate invoke | `preview_tessellate_invocations` (`✏️editor/🦀️.rs:2539+`) | No meshes after eval completes |
| 9. `flowTessellateResolve` | retained route | Empty preview |
| 10. `preview_payload` + session cache | `✏️editor/🦀️.rs:2615+`, `mesh_data_for_preview_handle` | Empty instances |
| 11. Window transient publish | `Generation3dPreviewWindowTransientOwner` | Stale/empty eval text between windows |
| 12. Renderer | `World3dHost` / wgpu scene bridge | wgpu: was blank before bridge lane; React: works when meshes present |

**Generate-mode parallel path:** `Generation3dPreviewCommandWork` (`✏️editor/🦀️.rs:394-427`) stops at hop 4 — **no hops 5-9**. **Viewer** same (`👁️viewer/🦀️.rs:200-207`).

**Open adjacent:** extension **fault** arm still pack-encoded at shell but `decode_fault_bytes` at guest (`📓️extension-result-realloc-2026-09-10.md` §4.5 end) — **degraded** when operators fault.

---

## 7. Platform gaps

| Platform | Gap | Severity |
|---|---|---|
| React dev (6018) | Served wasm predates extension paging + contributions re-arm + example re-arm | **Blocker** until restage |
| wgpu dev (6118) | Boot stalls silent at shell-boot 86% (`📓️wgpu-shell-boot-silence-2026-09-10.md` — lane in flight, report incomplete) | **Blocker** for wgpu E2E |
| wgpu | intake-budget fixed (`📓️wgpu-intake-budget-2026-09-10.md`) | served |
| Taxonomy | 22 procedural violations remain (assembly) | **Degraded** for CI `verify taxonomy` |

---

## 8. Work lanes (file-disjoint)

### Lane A — Mega restage + runtime verification (coordination)

**Owns:** activate/serve scripts only (no source edits).

**Blockers:** Confirm boot #13+ clears cabi_realloc trap, contributions re-arm paints meshes, example switch updates graph, interactionSelect works. Blocks all "fixed in source" claims.

Depends on: all guest/framework changes since boot #12 restage (~10:54).

---

### Lane B — Extension fault pack symmetry

**Owns:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (fault encode path only)

**Blockers:** none. **Degraded:** when brep evaluate returns fault, guest may not decode diagnostic (extension-result-realloc §4.5 STILL OPEN).

---

### Lane C — Generate-mode preview eval chain

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (`Generation3dPreviewCommandWork`, `generation3d_preview_window_ids`)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️generation/🦀️.rs`

**Blockers:** Generate mode 3D preview never completes brep evaluation (no extension/tessellate round-trip); `session: None` in render prevents mesh tessellation.

---

### Lane D — Viewer preview eval chain

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` (`Generation3dViewCommandWork`, contributions)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs`

**Blockers:** Read-only preview cannot evaluate brep graphs (same synchronous-tick gap as Lane C).

---

### Lane E — wgpu shell-boot progress + liveness

**Owns:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🐚️plugin-bridge.ts` (if needed)

**Blockers:** wgpu playground hangs at shell-boot 86% with no fault UI (`📓️wgpu-shell-boot-silence-2026-09-10.md`).

---

### Lane F — Assembly artifact mount

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (registration only)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/**` (entire subtree per `📓️assembly-mount-2026-09-10.md`)

**Blockers:** Assembly editor/viewer not in `ProceduralApps` — user cannot use WFC/assembly surfaces.

---

### Lane G — Demo-session example registration

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/**`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (`examples()` + `setActiveExample` options only — coordinate with Lane F on `🦀️.rs` if both touch)

**Degraded:** `demo-session` exists as fixture/tests but not in picker.

---

### Lane H — Example alternate formats (.op envelopes)

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/*/🖼️assets/*.op.semio` (8 files)

**Degraded:** `.op` sidecars are still 3-line empty envelopes; no `.spr`/`.pack` assets. IO codecs for export work (`📓️io-codecs-2026-09-09.md`); import paths use DSL primary.

---

### Lane I — Envelope decode fault surfacing

**Owns:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (envelope load poll)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` (replacement poll)

**Blockers:** `vcs_artifact_app_non_empty_retained_maintenance_swap` fails with swallowed fault (`📓️flow-host-ownership-2026-09-10.md` §9) — may affect document hot-swap UX.

---

### Lane J — Preview debug chrome removal

**Owns:**
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:72-91`

**Cosmetic:** Unconditional `debug` object in `status_json` (evalLen, evalHead) visible to status consumers.

---

## 9. Resolved since early audits (do not re-dispatch)

- Continuation routes `flowEvalResolve` / `flowTessellateResolve` in command enum
- Context menu empty selection
- Pointer-down stub routes (removed; framework-owned)
- Fold contract blocking all gestures
- Catalogue surface 111KB overflow
- Node-graph 300×150 / attach
- Boolean kernel / example geometry 8/8
- JSON-512 shell truncation
- Selection stack overflow
- Extension addressing / contributions delivery paging
- wgpu World3d mesh bridge (source)
- Window-kind action ownership + descriptor

---

*Audit agent: read-only, 2026-09-10. Full detail in this file; coordinator summary in chat.*
