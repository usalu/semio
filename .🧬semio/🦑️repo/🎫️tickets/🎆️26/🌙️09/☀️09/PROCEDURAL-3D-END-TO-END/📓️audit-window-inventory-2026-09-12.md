# Window Inventory — Authoritative Checklist (2026-09-12)

Read-only audit. Code on disk (checked 2026-09-12) wins over every prior ticket report; where a later
report supersedes an earlier one, that is called out explicitly. Goal: every window of generation3d
(`s.procedural.generation3d@1/*`) works for a user at `http://127.0.0.1:6018/?plugin=generation3d`
(wgpu second). Assembly (`s.assembly@1/*`, its own playground variant, ports 6019/6119) is included
per the brief but is **not reachable from the 6018 URL** — noted where relevant.

Prior reports read in full: `📓️viewer-window-2026-09-11.md`, `📓️generate-mode-windows-2026-09-11.md`,
`📓️window-kind-actions-2026-09-10.md`, `📓️window-bodies-and-eval-dispatch-2026-09-10.md`,
`📓️editor-gaps-2026-09-09.md`, `📓️gap-inventory-2026-09-10.md`, `📓️open-debt-punchlist-2026-09-10.md`,
`📓️status.md` (full), plus `📓️assembly-artifact-mount-2026-09-11.md`,
`📓️generate-mode-eval-wiring-2026-09-11.md`, `📓️preview-hover-select-orbit-2026-09-11.md`,
`📓️unknown-kind-after-restage-2026-09-11.md`, `📓️invoke-extension-rejected-authority-2026-09-11.md`.

**Supersession found and verified against source:** `📓️generate-mode-windows-2026-09-11.md` (written
14:26 on 2026-09-11) documents generate-mode preview as using a *different* eval chain than edit-mode
preview (`session: None`, no `ExtensionInvocation`). `📓️generate-mode-eval-wiring-2026-09-11.md`
(16:46, same day) reports this fixed. I verified the fix is actually in the working tree (not just
claimed): `✏️editor/🦀️.rs:349-350` — `generation3d_preview_kind` now recognizes both
`procedural-preview` and `generation3d-generate-preview`; `Generation3dPreviewCommandWork::step`
(`✏️editor/🦀️.rs:414-433`) no longer runs a synchronous `FlowEvalSession::tick` loop, it emits the
mutation and calls `rearm_attached_previews`; generate preview's `render()` signature
(`…/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs:53-61`) takes `session: &FlowEvalSession` (not
`Option`) and passes `Some(session)` into `preview_payload`. **This gap is closed at the source
level as of 2026-09-11 16:46.** Runtime is unverified (needs a restage — see §4).

---

## 1. Window kind inventory

### 1.1 `s.procedural.generation3d@1/*#editor` — mode `edit` (default mode)

| Field | Flow (`procedural-main`) | Preview (`procedural-preview`) |
|---|---|---|
| Directory | `✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/` | `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/` |
| Body key | `procedural.play.main` | `procedural.play.preview` |
| Surface | `NodeGraph` | `World3d` |
| Data source | Document fixture (widgets/ports/wires) via `NodeGraphScene`, paginated catalogue | `document` fixture + `window-transient.preview_eval_text` (eval JSON) + `session` tessellation cache |
| Owned actions (`window_kind_action_refs`, `✏️editor/🦀️.rs:1930-1940`) | `nodeGraphEdit`, `nodeGraphViewport`, `setLodMode` | `setCamera`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`, `translateSelection`, `rotateSelection`, `scaleSelection` |
| Interaction domain | `graph` (`✏️editor/🦀️.rs:1975`) | `graph` (`✏️editor/🦀️.rs:1976`) |
| Mounted in plugin root | Yes — `✏️editor/🦀️.rs:1825-1826` `.window_kind_def(flow_window::definition())` / `.window_kind_def(edit_preview::definition())`; app itself mounted at `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:105` `.editor_with_examples::<Generation3dPlayApp>(...)` | same mount line |
| Native test | `…/✏️editor/🎭️modes/✏️edit/🧪️tests/🕸️flow/🔬️unit/🦀️.rs` — `renders_node_graph_scene`, `main_graph_scene_exports_flow_backed_node_graph_fields`, `main_body_carries_the_documents_graph_as_semantic_rows` (file mtime 2026-09-11 12:41) | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:310-312` mounts `…/✏️edit/🧪️tests/👁️preview/…` (referenced by `📓️gap-inventory-2026-09-10.md` §1.1); `preview/🧪️tests/…/🦀️.rs:172-184` `render_without_a_published_evaluation_paints_the_empty_world` |

### 1.2 `s.procedural.generation3d@1/*#editor` — mode `generate`

Layout `generation3d-generate`, row split `[22%, 43%, 35%]` (`…/🎭️modes/🧬️generate/🦀️.rs:21-33`).

| Field | Generations | Form | Preview |
|---|---|---|---|
| Window kind id | `generation3d-generations` | `generation3d-generate-form` | `generation3d-generate-preview` |
| Directory | `…/🎭️modes/🧬️generate/🪟️windows/🗂️generations/` | `…/🪟️windows/📝️form/` | `…/🪟️windows/👁️preview/` |
| Body key | `procedural.play.generations` | `procedural.play.generate-form` | `procedural.play.generate-preview` |
| Surface | `Canvas2d` (semantic tree) | `Canvas2d` | `World3d` (fallback `TextEditor` when meshes+instances both empty) |
| Data source | Document `generation` roster (`generation_tree`) | `generation_fixture_for` patched form spec, or hint when nothing selected | Patched generation fixture (`generation_fixture_for`) + window-transient `SetGenerationPreview` + `session` (as of the 09-11 16:46 fix, real `FlowEvalSession`, not `None`) |
| Owned actions | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` (`✏️editor/🦀️.rs:1942`) | `updateGenerationValues` (`:1943`) | `setCamera`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity` (`:1944-1950`) — **no gumball trio**, unlike edit preview |
| Interaction domain | none declared | none declared | `graph` (`:1977`), but no graph widgets to pick against in this window |
| Mounted | Yes — `✏️editor/🦀️.rs:1827-1829` `.window_kind_def(generations::definition())` / `form::definition()` / `generate_preview::definition()`; `.named_layout(generate::layout())` (`:1831`) | same | same |
| Native test | `…/🎭️modes/🧬️generate/🧪️tests/🗂️generations/🔬️unit/🦀️.rs` | `…/🧪️tests/📝️form/🔬️unit/🦀️.rs` — `generate_form_hints_without_a_selected_generation` | `…/🧪️tests/👁️preview/🔬️unit/🦀️.rs` (hint-text test); tick chain itself proven by `…/🧪️tests/🔬️tick-addressing/🦀️.rs` `generate_preview_eval_emits_extension_or_rearms_flow_eval_tick`, `every_armed_tick_names_a_preview_window_that_is_actually_attached`, `only_a_preview_addressed_tick_passes_the_retained_preflight` (all reported green natively 2026-09-11, per `📓️generate-mode-eval-wiring-2026-09-11.md` §"Verified") |

No `enterGenerate` command exists — mode switch is shell-only (`ShellHost/🟦️.tsx` `applyModeChange`, §3). All generation data commands are app-scoped and dispatch regardless of mode.

### 1.3 `s.procedural.generation3d@1/*#viewer` — mode `view`

| Field | Preview (`procedural-view-preview`) |
|---|---|
| Directory | `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/` |
| Body key | `procedural.view.preview` |
| Surface | `World3d` |
| Data source | `document` fixture + `transient.preview_eval_text`, evaluated by `Generation3dViewCommandWork` — see §4 for the eval-chain gap |
| Owned actions | All seven view commands (`setShowMode`, `setLodMode`, `setCamera`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`) — `👁️viewer/🦀️.rs:804-819` `window_kind_action_refs` |
| Interaction domain | `graph` — `👁️viewer/🦀️.rs:802` `.window_kind_interactions` |
| Mounted | Yes — `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:106` `.viewer::<Generation3dViewer>(create_generation3d_viewer())` |
| Native test | `👁️viewer/🧪️tests/🔬️unit/🦀️.rs` — `every_viewer_action_dispatches_live_and_never_mutates_the_document`, `every_emitted_action_is_declared_on_the_preview_window_kind`; `…/🎭️modes/👁️view/🧪️tests/👁️preview/🔬️unit/🦀️.rs` — tessellation/interaction/marks; `render_without_a_published_evaluation_paints_the_empty_world` (cold-open-empty proof) |

Zero `Noop` commands (`grep -rn Noop 👁️viewer/` → 0 hits, verified 2026-09-12). Eight commands total: 7
user view actions + 1 host-only `setContributions`.

### 1.4 `s.assembly@1/*` — separate app, own playground (port 6019/6119, **not** 6018)

Mounted 2026-09-11 (`📓️assembly-artifact-mount-2026-09-11.md`; verified live in
`✏️s/🔌️plugins/🌀️procedural/🦀️.rs:32-33,107-110`: `AssemblyEditor`/`AssemblyViewer` variants,
`.editor_with_examples::<AssemblyEditor>`, `.viewer::<AssemblyViewer>`).

| Field | Editor `structure` | Viewer `structure` |
|---|---|---|
| Directory | `🧩️assembly/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🌳️structure/` | `🧩️assembly/…/👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/` |
| Window kind id / body key | `framework.window.tree` (`TreeWindowKit::KIND_ID`, `🌳️structure/🦀️.rs:16-17`) | same |
| Surface | Tree (`TreeWindowKit`) over `AssemblySnapshot` (seed/slots/edges/modules/weights/rules) — **not** a spatial/mesh view; the WFC solve is an inference, never persisted (`🌳️structure/🦀️.rs:1-9`) | same shape, read-only |
| Owned actions | 9 mutation actions: `create-slot`, `delete-slot`, `create-rule`, `delete-rule`, `connect-slots`, `disconnect-slots`, `change-weight`, `remove-weight`, `change-seed`, all `Migrated` (`🌳️structure/🦀️.rs:26-38`) | none (read-only) |
| Mounted | Yes, both editor and viewer, in `ProceduralApps` enum + `plugin()` (`🦀️.rs:32-33,107-110`) | Yes |
| Native test | `…/✏️editor/…/mount-contract`, `…structure…`, `…example…` — 28 passed per `📓️assembly-artifact-mount-2026-09-11.md`; `assembly_apps_are_declared_on_the_plugin`, `assembly_manifest_examples_are_registered_on_the_editor_surface`, `assembly_editor_and_viewer_share_dialect` — 3 passed, in `semio-s-plugin-procedural --lib` |

Assembly is **out of the user flow this ticket targets** (a different `variant` in
`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:31-35`, ports 6019/6119). It cannot be
opened from `http://127.0.0.1:6018/?plugin=generation3d` at all; a user would need
`http://127.0.0.1:6019/?plugin=assembly` (or wgpu 6119). Listed here because the brief names it, not
because it blocks the generation3d goal.

---

## 2. Runtime state per window (last known, with source and date)

Runtime evidence never advanced past a restage on 2026-09-11 ~14:15 (94 138 842 B guest,
`📓️unknown-kind-after-restage-2026-09-11.md`). Every source-level eval-chain fix that landed **after**
14:15 (the `invoke-extension-rejected-authority` fix at 15:39, the generate-mode eval wiring at 16:46)
is **unverified in the browser** — no report shows a restage + reboot after 16:46. `📓️status.md`'s own
log ends at the 14:15 entry ("Goal not complete").

| Window | Last observed runtime state | Source + date |
|---|---|---|
| Flow (`procedural-main`) | Renders the real graph (e.g. Hexagonal Mushroom Column, 7 nodes) — the earlier "flow window paints a placeholder" defect (found 2026-09-10 ~22:40) is fixed; flow status shows one node `computing` mid-eval | `📓️preview-hover-select-orbit-2026-09-11.md` §0 (assumes restage), confirmed by `renders_node_graph_scene` test file mtime 2026-09-11 |
| Edit preview (`procedural-preview`) | Mounts (3 canvases at ~9.7 s post window-bodies fix), but `meshCount: 0` in every recorded probe. Fault sequence across the day: `flow.extension-not-contributed` → (after example-scoped contributions, 248 635 chars) `unknown kind: brep.curve.polygon` → (after the authority-retirement fix, unverified) — no probe run after the 15:39/16:46 fixes | `📓️unknown-kind-after-restage-2026-09-11.md` (14:15-post), `📓️invoke-extension-rejected-authority-2026-09-11.md` (15:39, fix landed, **not restaged**) |
| Generate: Generations | Renders (list chrome) once window bodies mount | `📓️generate-mode-windows-2026-09-11.md` §3 |
| Generate: Form | Renders hint or real form chrome; value edits persist to the artifact | same, §3 |
| Generate: Preview | Was **broken by design** (no extension round-trip) as of 14:26; fixed at source 16:46 (§ supersession note above); **zero recorded runtime trace of a successful generate-mode mesh** for any example, before or after the fix | `📓️generate-mode-windows-2026-09-11.md` §5.3; `📓️generate-mode-eval-wiring-2026-09-11.md` (native-only verification) |
| Viewer preview (`procedural-view-preview`) | **No confirmed boot log entry** shows the viewer opened with live meshes+pick at all — the playground defaults to editor role and no launch row sets `VITE_SEMIO_APP_ROLE=viewer` (§3.2). Independent of that, the eval chain itself has no extension round-trip today (§4) | `📓️viewer-window-2026-09-11.md` §5.4, §6; confirmed still true by source read 2026-09-12 (`👁️viewer/🦀️.rs`, zero `ExtensionInvocation` hits) |
| Assembly structure (editor/viewer) | Not probed in the browser at all (separate port, separate playground variant); only native/TS/Python mount-contract tests run | `📓️assembly-artifact-mount-2026-09-11.md` §"Tests run" — no browser step listed |

Unknown / not superseded by any report: whether the 15:39 authority-retirement fix plus the 16:46
generate-preview wiring, **restaged together**, produce `meshes > 0` on any of the eight examples.
Marked **unknown** rather than assumed — no report runs a probe after both fixes land.

---

## 3. Shell / layout side

### 3.1 Layout mechanics

- Mode layouts are authored per app: edit mode is a fixed two-pane row split `[68, 32]` (Flow / Preview,
  `…/🎭️modes/✏️edit/🦀️.rs:14-16`, `create_default_layout(..., "row", ...)`); generate mode is a named
  layout `generation3d-generate`, row split `[22, 43, 35]` (Generations / Form / Preview,
  `…/🎭️modes/🧬️generate/🦀️.rs:21-33`). Viewer and assembly editor/viewer are each a single full-pane
  `Stack` with one window (`👁️viewer/🎭️modes/👁️view/🦀️.rs:18-26`,
  `🧩️assembly/…/✏️editor/🎭️modes/✏️edit/🦀️.rs:17-25`).
- The React shell resolves a session's starting layout from `app.defaultLayout` /
  `app.defaultModeId` (`ShellHost/🟦️.tsx:857, 3330, 3361, 4802-4807, 7169, 8112`), and mode switches
  go through `resolveLayoutForMode` / `resolveFrameworkLayoutSeed` reading `app.windowKinds` +
  `app.modes` (`ShellHost/🟦️.tsx:6930, 6973, 9750, 9777`).
- Default mode on boot is **edit** (`default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)`,
  `✏️editor/🦀️.rs:1823`) — so the default-visible windows at `http://127.0.0.1:6018/?plugin=generation3d`
  are Flow + Preview. Generate mode's three windows and the viewer's single window are **not**
  visible by default; a user must explicitly switch mode or open the app in viewer role.

### 3.2 How a user opens the other windows/modes

- **Generate mode**: click the navbar `Generate` button, DOM id `playground.navbar.modes.generate`
  (`ShellHost/🟦️.tsx:8429`, inside `<ButtonGroup id="playground.navbar.modes">` at `:8423`). Handler
  `applyModeChange("generate")` (`:6921-6931`) sets `viewState.activeModeId`, resolves the named
  layout, and reseeds via `applyFrameworkLayoutSeed` + `refreshUi({kind:"full"})`.
- **Viewer role**: **no UI control exists in the playground to switch role.** It is boot-time only —
  `VITE_SEMIO_APP_ROLE=viewer` env var read by the React dev entry (`🧑‍💻dev/🟦️.ts`), defaulting to
  `editor` when unset. No `.vscode/🧩️launch.seed.jsonc` row sets this for generation3d
  (`📓️viewer-window-2026-09-11.md` §2.4, §3.5 — verified still true: no `role=viewer` or
  `VITE_SEMIO_APP_ROLE` string found near any generation3d row as of the last audit; not re-checked
  line-by-line here since launch.json is regenerated tooling, not source).
- **Panels** (Document/Catalogue/Inspection) are framework-injected **tabs**, not window-layout
  panes: `session.app.panelTabs`, anchored `top-left`/`top-right`
  (`ShellHost/🟦️.tsx:7930-7987`), toggled by clicking the tab, independent of mode.
- **Assembly**: only reachable by navigating the browser to a different origin/port
  (`?plugin=assembly` on 6019 react / 6119 wgpu) — not a control inside the generation3d playground.

### 3.3 Keyboard / menu access

- App-level keybindings declared for generation3d editor: **only** `mod+z` → `undo`,
  `mod+shift+z` → `redo` (`✏️editor/🦀️.rs:1978-1979`). No keybinding for mode switching, no
  per-window-action keybinding, and the viewer/assembly apps declare **zero** keybindings at all
  (`grep -n "\.keybinding(" 👁️viewer/🦀️.rs` → 0 hits, verified 2026-09-12).
- Focused-window-scoped keybinding dispatch reads `windowKind.actions` at
  `ShellHost/🟦️.tsx:7864` (`session.app.windowKinds.find(...).actions`) — this is the mechanism the
  `📓️window-kind-actions-2026-09-10.md` lane made trustworthy (previously every window carried the
  full 44-action app list; now each window's declared set is the owned refs plus framework verbs).
  Practical effect for the user: **mode switching and window opening have no keyboard path today** —
  mouse-only via the navbar buttons and panel tabs.
- The undeclared-action diagnostic gate lives at `ShellHost/🟦️.tsx:6078` (moved from the
  `:5691` the 2026-09-10 report cited — line drift from unrelated edits, same function
  `undeclaredActionDiagnostic`, confirmed present and unchanged in behavior).

---

## 4. Gaps ranked by user impact

### P0 — blocks the stated goal outright

1. **Viewer preview has no extension round-trip at all (never fixed, still open 2026-09-12).**
   `Generation3dViewCommandWork::step` (`👁️viewer/🦀️.rs:179-207`) constructs a fresh
   `FlowEvalSession::new()` per command (`:194`) and calls `session.tick(host)` synchronously
   (`:201`) with **zero** `ExtensionInvocation` sites anywhere in the file (verified by grep,
   2026-09-12: 0 hits for `ExtensionInvocation|invoke_extension|extension_invocations` in
   `👁️viewer/🦀️.rs`). This is the exact defect the 2026-09-11 16:46 fix closed for generate-mode
   preview (`generation3d_preview_kind` unification, `rearm_attached_previews`) but the viewer was
   never touched by that lane. Brep-bearing examples will never tessellate in the viewer. Fix:
   extend `generation3d_preview_kind`-style tick addressing (or an equivalent) to the viewer's
   `Generation3dViewCommandWork`, replacing the synchronous `session.tick` loop with
   `flow_eval_tick::evaluate` + `ExtensionInvocation`/`flowEvalResolve`/`flowTessellateResolve`, as
   was done for `Generation3dPreviewCommandWork` in `✏️editor/🦀️.rs`.

2. **No playground control to open the viewer role.** `http://127.0.0.1:6018/?plugin=generation3d`
   always boots the editor; nothing in the React dev entry, the shell, or `.vscode/launch.json`
   lets a user reach `s.procedural.generation3d@1/*#viewer` short of manually setting
   `VITE_SEMIO_APP_ROLE=viewer` before the dev server starts. Since the ticket's user-facing goal is
   "every window... works for a user in the React playground," the viewer window is currently
   **unreachable by any in-app action**. Fix: add a launch/serve row (or an in-shell role toggle) —
   `📓️viewer-window-2026-09-11.md` P0 item 2.

3. **Runtime proof of `meshes > 0` for any window, any example, does not exist post the last two
   source fixes.** The most recent restage (14:15, 2026-09-11) predates the authority-retirement fix
   (15:39) and the generate-preview eval-chain fix (16:46). Every mesh-count claim in every report is
   either `0` or untested against current source. This blocks confirming P0 items are actually
   resolved for a real user. Fix: restage `semio-s-plugin-procedural` (native + wasm32-wasip2),
   reboot 6018, re-run `🐍️restage-eval-probe.mjs` / `🔍️browser-probe.ts --mode=interact` per example.

### P1 — degrades the experience materially

4. **Generate-mode preview has no gumball / transform actions**, unlike edit-mode preview
   (`window_kind_action_refs` for `generation3d-generate-preview` omits `translateSelection` /
   `rotateSelection` / `scaleSelection`, `✏️editor/🦀️.rs:1944-1950` vs `:1931-1940` for edit
   preview). A user cannot manipulate a generated instance directly in generate mode. Product
   decision, not a bug, per `📓️generate-mode-windows-2026-09-11.md` §6.4 item 10 — but worth ranking
   since it's a visible capability gap between the two preview windows.

5. **No keyboard or menu path to switch modes or reach viewer/assembly** — mouse-only via
   `playground.navbar.modes.*` buttons (§3.3). Accessibility gap: CLAUDE.md requires accessible UIs;
   today mode switching has no keyboard equivalent.

6. **Cosmetic debug chrome still ships in edit preview's `status_json`** — unconditional `debug`
   object (`evalLen`, `evalHead`) at `…/✏️edit/🪟️windows/👁️preview/🦀️.rs:84`, confirmed still present
   2026-09-12. Low severity but a stated, unclosed item (`📓️gap-inventory-2026-09-10.md` Lane J).

7. **No browser/E2E harness step exercises generate mode, viewer mode, or per-example mesh assertions.**
   `🔍️browser-probe.ts` only supports `--steps=example,hover,select,orbit` against the **edit-mode**
   preview canvas; it throws `no preview canvas` if generate mode or viewer mode is active
   (`📓️generate-mode-windows-2026-09-11.md` §5.2, confirmed unextended — no new probe step files
   found under the ticket folder as of 2026-09-12). This means P0 items 1-3 have no repeatable,
   scripted way to verify once source-fixed.

### P2 — real but low-severity / already tracked elsewhere

8. **Demo-session example (`demo-session`) still unreachable from the navbar picker** —
   `examples()` in `✏️editor/🦀️.rs` lists 8 DSL examples; the module exists but is not registered
   (`📓️gap-inventory-2026-09-10.md` B23, not re-verified line-by-line here since it is unchanged
   product scope, not a regression risk).
9. **Assembly is a second, disconnected app** (different port, different playground variant) —
   not a "generation3d window" gap, but worth flagging since the ticket brief names it: a user
   cannot reach assembly from the generation3d URL at all, by design.

### Verified NOT present (checked fresh 2026-09-12, contradicting stale mentions in old reports)

- **Context menu hardcoded empty selection** — fixed. `context_menu_with_request_context`
  (`✏️editor/🦀️.rs:1753`) reads the real `InteractionView`-backed selection via
  `PreviewInteractionMarks::graph_selection_domains`; no `Vec::new()` selection stub remains
  (`📓️editor-gaps-2026-09-09.md`, confirmed current via `grep -n "fn context_menu" ✏️editor/🦀️.rs`
  showing three real methods, none hardcoding an empty selection).
- **`worldPointerDown` / `graphPointerDown` stub routes** — deleted, not stubbed. `grep -rn
  "worldPointerDown|graphPointerDown" 🧊️generation3d/` (excluding generated `🔣️.json`) returns only
  a doc-comment explaining the removal (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:687`). The regenerated
  descriptor `✏️s/🔌️plugins/🌀️procedural/🔣️.json` also no longer contains either string (`grep -c` →
  0), so the previously-open B59 "stale descriptor mirror" item is closed too.
- **`👥️set-contributions/` empty directory** — no longer empty; both editor and viewer now have real
  `🎮️commands/🧩️set-contributions/` modules with actual handlers (`find` confirms both directories
  exist and are non-empty as of 2026-09-12).
- **Viewer `Noop`-only command enum** — false as of this audit; the viewer has 8 real commands, 0
  `Noop`, confirmed by grep.
- **IO codecs "silently wrong for 7/9 formats"** — fixed per `📓️io-codecs-2026-09-09.md`; no
  `todo!`/`unimplemented!` found anywhere under the generation3d artifact tree (`grep -rl` → 0
  files, checked 2026-09-12).
- **Assembly editor/viewer "not mounted"** (open-debt-punchlist B21/B22) — fixed 2026-09-11; verified
  live in `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` (§1.4 above). B21/B22 should be considered closed.

---

## 5. Method

Directory walks (`find`), `grep -rn` over the generation3d/assembly artifact trees and the two
framework files named in the brief, and direct reads of the Rust source cited above. No `cargo`/`nx`
builds or tests were run (read-only auditor; native test *names* are cited from files that exist on
disk, not re-executed). No browser probe was run. All line numbers are from the working tree as read
on 2026-09-12 and may drift with concurrent edits from other sessions.
