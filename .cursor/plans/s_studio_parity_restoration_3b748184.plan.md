---
name: S studio parity restoration
overview: "Restore pre-migration S studio feature parity: fix the currently-broken shared plugin build, fix two confirmed runtime regressions (broken keybindings, window layout getting wiped on every command), restore lost S UI affordances using existing generic plugin infra, correct the demo fixture's cross-technology reference, and implement real studio persistence plus a presence-peers data contract."
todos:
 - id: phase0-fix-build
   content: Fix World3dScene missing-fields compile error in framework/plugin/rs/scaffold.rs:241 blocking all plugins; verify with cargo check
   status: completed
 - id: phase1-keybinding-mod-bug
   content: Fix keybinding matcher in os-shell.tsx to recognize 'mod' token and ignore keydown when focus is in an editable text field
   status: completed
 - id: phase1-layout-reset-bug
   content: Fix refreshUi() unconditionally resetting shellLayout on every command; only reseed on actual app/session identity change
   status: completed
 - id: phase1-mobile-panel
   content: Wire a mobilePanel (merged left+right tabs) into os-shell.tsx's Layout so panels are reachable on mobile viewports
   status: completed
 - id: phase2-toolbar-tools
   content: Populate mode_tools() for S home (createStudio/importStudio) and studio (undo/redo/checkpoint) in s/plugin/rs/lib.rs; remove redundant hardcoded footer branch in os-shell.tsx
   status: completed
 - id: phase2-context-menu
   content: Restore richer media-graph context menu (copy/paste/duplicate/rename/group/delete) in render_media_graph
   status: completed
 - id: phase2-inspector-fields
   content: Restore Program/App (batch, 'Mixed'-aware) and single-selection Instance-id readonly rows in build_inspector_tree
   status: completed
 - id: phase2-parameter-constraints
   content: Make numeric parameter Min/Max/Step fields always render (empty when unset) in parameter_constraint_fields
   status: completed
 - id: phase2-categorical-options
   content: Restore per-option row UI (individual Remove + typed Add-option input) for categorical parameters
   status: completed
 - id: phase2-dynamic-examples
   content: Restore glob-based discovery of s/example/*.s.json fixtures as selectable studio examples (S-only, no cross-tech)
   status: completed
 - id: phase2-settings-panel
   content: Restore meaningful App-identity settings content alongside the existing theme/compact/expertise tab
   status: completed
 - id: phase3-fixture-sketchpad
   content: Replace compose.sketchpad instance in s/example/demo.s.json with an in-technology (e.g. second note) instance
   status: completed
 - id: phase4-persistence
   content: Implement browser-persistent OsBackbonePort (localStorage), wire CATALOG_PORT and per-studio sync/load_backbone calls, extend s-plugin tests
   status: completed
 - id: phase4-presence
   content: Reintroduce PresencePeer type, presence_peers() accessor, and avatar-overlay UI on the media graph (empty list absent a real transport)
   status: completed
 - id: verify-all
   content: Run cargo test -p s-plugin, framework-renderer-react vitest, S studio E2E, and manual repro checks for both confirmed bugs plus mobile/persistence
   status: completed
isProject: false
---

## Audit summary

Full three-way audit (S command surface, shell/chrome, build health) completed via parallel `explore` subagents comparing pre-migration TypeScript (`git show f8376e848:s/core/js/index.ts`, `s/react/index.tsx`, and the old `PlatformView`/`PlaygroundView`) against the current Rust/WASM `s/plugin/rs/lib.rs` + `framework/renderer/react/os-shell.tsx`. Most CQRS command-level parity (spawn/undo/redo/checkpoint/parameter bind, catalogue/parameters/inspector panels) genuinely made it over and is test-covered. The gaps below were independently verified against actual source, not the (partially stale) migration-ticket docs in `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/`.

Two of the "missing features" turned out, on manual reproduction, to be confirmed runtime bugs rather than simple omissions — these are called out separately since they're regressions, not gaps.

## Phase 0 — Unblock the build

A concurrent edit added 6 fields to `World3dScene` (`framework/core/rs/ui.rs`) that `framework/plugin/rs/scaffold.rs:241` wasn't updated for, breaking compilation for **every** plugin crate (confirmed via `cargo check -p draw-plugin`).

- Fix the `World3dScene { ... }` literal at `framework/plugin/rs/scaffold.rs:241` — either default the 6 new fields to `None` or delegate to the existing `world3d_host::world3d_scene(...)` helper (used correctly elsewhere, e.g. `framework/plugin/rs/world3d_host.rs:125`).
- Re-verify with `cargo check -p s-plugin -p semio-framework-plugin -p semio-framework-core --target wasm32-unknown-unknown` (this may already be fixed if concurrent edits landed it — check first).

## Phase 1 — Fix confirmed regressions

1. **Keybinding matcher doesn't understand `"mod"` and has no text-input guard** (`framework/renderer/react/os-shell.tsx:1139-1148`). S declares `mod+n`, `mod+o`, `mod+z`, `mod+shift+z`, `mod+s` (`s/plugin/rs/lib.rs:2035-2036, 2084-2086`), but `matches()` only treats the literal tokens `ctrl`/`meta` as needing a modifier — `"mod"` is neither, so `needsCtrl` stays `false`. Net effect: a bare `z`, `n`, `o`, or `s` keypress **anywhere on the page, including inside text inputs**, fires undo / createStudio / importStudio / commitCheckpoint.
   Fix: treat `"mod"` as requiring `ctrlKey || metaKey` (same as today's `ctrl`/`meta` handling), and skip dispatch when `event.target` is an editable field (`input`, `textarea`, `[contenteditable]`, `[role="textbox"]`) — matching the ignore-when-typing default that the old `useCommandHotkey`/`react-hotkeys-hook` path already provides for other shortcuts.

2. **Manual window-layout changes are wiped out by the very next command.** Root-caused and reproduced empirically: `refreshUi()` (`os-shell.tsx` ~line 706) unconditionally calls `setShellLayout(convertFrameworkLayoutToModeLayout(nextSession.app.defaultLayout, windowIds))` on **every** UI refresh, and `refreshUi` runs after essentially every dispatched command (`os-shell.tsx:764, 779, 811, 932`). Reproduced live: clicking the dock's "Close" control on one of the 3 studio windows caused all 3 windows to vanish, then reappear on the next click (3→0→3 flicker), because the very next command-triggered refresh restored the full default layout.
   Fix: only (re)seed `shellLayout` when the session's app identity actually changes (e.g. track previous `session.app.id` in a ref and skip the reset when it's unchanged), not on every per-command refresh. This also fixes window-close, pane resize, and tab-drag persistence for **all** plugins, not just S.

3. **Mobile side panels are completely unreachable.** `Layout` (`ui/js/react/index.tsx:4669-4677`) only renders `mobilePanel` (not `leftSidePanel`/`rightSidePanel`) when `mobile` is true, but `os-shell.tsx:1606` passes `mobile={mobile}` without ever building a `mobilePanel`. Catalogue/Parameters/Inspector/Settings are unreachable on any viewport ≤767px for every plugin.
   Fix: construct `mobilePanel={{ tabs: [...leftPanelTabs, ...rightPanelTabs], visible: leftPanelVisible || rightPanelVisible, activeTabId, onActiveTabChange }}` (shape defined at `ui/js/react/index.tsx:14164-14171`) and pass it alongside `mobile`.

## Phase 2 — Restore S UI/UX affordances (via existing generic plugin infra)

4. **No toolbar/footer actions for New/Import Studio (Home); Undo/Redo/Checkpoint already has hardcoded footer buttons but isn't plugin-declarative.** The generic `mode_tools()` builder (`framework/plugin/rs/app.rs:85`) and `tool_button`/`tool_collection` helpers (`framework/core/rs/tools.rs`) already exist and already render via `footerToolbar`/`ToolTree` (`os-shell.tsx:1484-1487`) with **zero shell changes needed**.
   Fix: populate `.mode_tools(mode_id, vec![tool_button(...), ...])` for both Home (createStudio, importStudio) and Studio (undo, redo, checkpoint) in `s/plugin/rs/lib.rs`'s `create_home_app()`/`create_studio_app()`, then remove the now-redundant hardcoded `S_PLAY_CONTROLLER_ID` branch from `footerItems` (`os-shell.tsx:1459-1480`). This simultaneously fixes the "footer isn't plugin-extensible" architectural gap using infra that already exists, rather than adding a new field.

5. **Media-graph context menu reduced to 3 hardcoded items.** Restore richer entries (copy/paste/duplicate/rename/group/delete as applicable to the media-graph domain) in `render_media_graph` (`s/plugin/rs/lib.rs:1174-1176`), matching old `s/react/index.tsx:104-109`'s full flow-canvas menu minus the 5 spotlight-only entries it explicitly filtered.

6. **Inspector lost Program/App/Instance-id readonly rows.** Restore batch Program/App rows (showing "Mixed" when selection spans heterogeneous values) and the single-selection Instance-id row in `build_inspector_tree` (`s/plugin/rs/lib.rs:866-977`), matching old `s/core/js/index.ts:1111-1145`.

7. **Numeric parameter Min/Max/Step only render when already set.** Fix `parameter_constraint_fields` (`s/plugin/rs/lib.rs:591-661`) to unconditionally render all three fields (empty when unset), matching old `s/core/js/index.ts:882-921`, so fixture-authored parameters without constraints can have them added via the UI.

8. **Categorical option editing UX regressed** from per-option rows (individual Remove button, typed Add-option input) to a single CSV text field + "add literal 'New'" button. Restore the old per-row pattern in `s/plugin/rs/lib.rs:663-689`, matching `s/core/js/index.ts:922-951`.

9. **Example switcher hardcoded to `"demo"` only.** Restore glob-based discovery of `s/example/*.s.json` fixtures as selectable studio examples (`create_studio_app()`/`setActiveExample`, `s/plugin/rs/lib.rs:1533-1559, 2111`), scoped to S's own example fixtures only — no cross-technology fixture registration, consistent with AGENTS.md's no-technology-mixing rule.

10. **Settings panel collapsed from 3 tabs to 1.** Restore meaningful "App identity" content alongside the existing theme/compact/expertise tab (`os-chrome-panels.tsx:195-261`) where it still applies to the new architecture; skip anything tied to since-removed concepts (e.g. compute-worker-count, if no longer relevant).

## Phase 3 — Fixture correction

11. **`s/example/demo.s.json` references an unresolved `compose.sketchpad` instance** (`app-sketchpad-1`, `programId: "compose.sketchpad"`) — the cross-tech registration that used to resolve it was dropped during migration, and per AGENTS.md, S must not mix in the `compose` technology. Replace `app-sketchpad-1` with an in-technology instance (e.g. a second `note` instance, since `note` is already a registered S program) — update `programs`, the `appInstances` entry, and its `mediaGraph` node/ports to match the replacement's actual resource-kind shape (mirroring the existing `app-writer-1` node pattern).

## Phase 4 — Infrastructure restoration

12. **Studio persistence is memory-only; reload loses all data.** `RemoteOsBackbone::sync` (`framework/product/os/core/rs/host.rs:1174-1198`) is an explicit "not implemented" stub, and the home catalog's `CATALOG_PORT` (`s/plugin/rs/lib.rs:147-168`) is a plain in-process `MemoryBackbonePort`. However, the `DevJsonBackbone`/`LocalJsonBackbone` abstractions (`framework/product/os/core/rs/host.rs:1047-1137`) already exist and are generic over any `OsBackbonePort` (`read`/`write` by URI) — this is a wiring gap, not a missing architecture.
    - Implement a browser-persistent `OsBackbonePort` (localStorage-backed via `web_sys`, since S runs as WASM in-browser).
    - Wire `CATALOG_PORT` and per-studio backbones in `s/plugin/rs/lib.rs` to use it instead of `MemoryBackbonePort`.
    - Call `sync_backbone()` after every mutation (mirroring old `onAfterMutation`) and `load_backbone()` on studio open/create, matching old `s/core/js/index.ts:130-145`.
    - Extend the existing `s/plugin/rs/lib.rs` test module (per AGENTS.md — no new test files) with round-trip persistence coverage.

13. **Presence/multiplayer peer indicators removed entirely.** Old `s/react/index.tsx:159, 279-288, 483` rendered a `peers: PresencePeer[]` avatar overlay on the media-graph canvas. No real-time multiplayer transport exists anywhere in the current or old dev stack (old `RemoteOsBackbone` was itself effectively unimplemented for live sync), so this is scoped as: reintroduce the `PresencePeer` type and a `presence_peers()`-style accessor plus the avatar-overlay render path in `render_media_graph` (`s/plugin/rs/lib.rs`) and `node-graph-host.tsx`, returning an empty list in the absence of a real transport — restoring the data contract and UI affordance for parity, not inventing a new live-collaboration backend (which would be a separate, much larger effort if actually desired).

## Verification

- `cargo test -p s-plugin` (extend existing tests, don't add new files) after each Rust change.
- `bun nx run @semio-tech/framework-renderer-react:test` after shell changes.
- Re-run `S_STUDIO_URL=http://127.0.0.1:6070/ node .repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`.
- Manual repro checks for the two confirmed bugs: (a) type "checkpoint" or press "z"/"n"/"o"/"s" while focused in a text input — should no longer trigger commands; (b) close a studio window via the dock close button, then dispatch any other command — the window should stay closed.
- Manual mobile-viewport check (resize to ≤767px) that Catalogue/Parameters/Inspector/Settings are reachable via the mobile panel.
- Reload the browser tab after creating/editing a studio — data should survive (Phase 4 persistence).
