# Prior Tickets Digest: Demonstrator & Apps Work (08/06–08/28)

## TICKETS ANALYZED

### 1. **26/08/17 — FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG** (open)
**Status:** open | **Emoji:** 🧪  
**Summary:** Demonstrator panes boot end-to-end: transitive plugin deps, hot-swap watch SSE, appId session on owning plugin, worker wire guard, Canvas duplicate export fix.

**KNOWN-FIXED:**
- Transitive plugin dependencies resolved
- Hot-swap watch SSE ordering (extensions published before SSE)
- AppId session wired to owning plugin, not plugin-instance
- Worker wire frame decoding guards
- Canvas duplicate export eliminated

**KNOWN-BROKEN:**
- None explicitly listed; ticket describes fixes applied

**DEFERRED:**
- Full end-to-end app content acceptance (per all-app-acceptance-boundary notes)
- Native patch transfer and guest memory admission
- Final live demonstration with all six panes interactive

---

### 2. **26/08/13 — UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION** (open)
**Status:** open | **Emoji:** 🧭️  
**Summary:** Target architecture: 4 state mechanisms (artifacts, config, presence, transient). Three workstreams: state-lane unification, composition made real, demonstrator restoration. Extensive plan covering PresenceStore/TransientStore, child persistence, IO-registration ownership, pane dissolution into apps bundle.

**KNOWN-FIXED:**
- (Current session ongoing; no fixes reported yet)

**KNOWN-BROKEN:**
- IO-registration ownership unclear (CAD/GIS ownership boundaries)
- Panes not dissolved into apps bundles
- Playground conformance incomplete
- Boot proof on port 6029 not validated
- Two-tab presence/config/transient demo not implemented
- Static ship deployment not ready

**DEFERRED:**
- Full three-workstream implementation (state-lane unification, composition, demonstrator restoration)
- Coordination with sibling tickets UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM and DISSOLVE-KERNELS-AND-MODULES
- PolicyRatchet enforcement of 4-lane exhaustiveness

---

### 3. **26/08/07 — GET-ALL-APPS-WORKING-END-TO-END** (open)
**Status:** open | **Emoji:** 🎫️  
**Summary:** Fixed standalone apps collapsing/growing vertically and hanging. Root cause: OS-dev globals.css declared no Tailwind v4 @source globs; compiled stylesheet shipped ~96 class rules without flex-1/min-h-0/overflow-hidden. Also fixed contextMenuContentClassName render error.

**KNOWN-FIXED:**
- Tailwind v4 @source globs added to UI react and OS renderer globals.css
- Flex/min-height/overflow layout classes now compile (285KB vs prior 174KB)
- ContextMenuChrome import issue fixed
- Layout probes (1440x900): procedural3d, gis2d, fem2d, demonstrator all validated
- UI react tests: 37 passed
- Extension crates published; extension 404s and worker crashes eliminated

**KNOWN-BROKEN:**
- (Per summary: empty procedural windows appear after some time — attempted fix via hot-swap ordering; status unclear if still broken)

**DEFERRED:**
- Full integration test of all apps over time
- Demonstrator 2x3 grid end-to-end validation

---

### 4. **26/08/06 — APPS-RUNNING-END-TO-END** (closed)
**Status:** closed | **Emoji:** 🚀  
**Summary:** Restored end-to-end app boots after monorepo restructure. Electron Forge finds electron under bun.lock (patch applied). Retargeted build scripts from removed @semio-tech/framework-playground-dev to @semio-tech/framework-os-dev.

**KNOWN-FIXED:**
- Electron Forge bun.lock discovery (patch applied)
- build:* scripts retargeted to framework-os-dev
- Root DevScript regenerates playground catalog when empty
- Stale ⚡️implementations path aliases retargeted to Shape-V2 packages
- bun run dev:procedural:3d serves HTTP 200
- 10 priority playground apps smoke OK
- compose-desktop launches Electron

**KNOWN-BROKEN:**
- @semio-tech/animate-present-core still missing for mit-bestand presentation

**DEFERRED:**
- None listed

---

### 5. **26/08/06 — APPS-RUNNING-END-TO-END-AFTER-RESTRUCTURE** (closed)
**Status:** closed | **Emoji:** 🚀  
**Summary:** Duplicate of open ticket 26/08/06/APPS-RUNNING-END-TO-END; work continued on existing ticket.

**KNOWN-FIXED:**
- (Consolidated into APPS-RUNNING-END-TO-END)

**KNOWN-BROKEN:**
- (Consolidated into APPS-RUNNING-END-TO-END)

**DEFERRED:**
- (Consolidated into APPS-RUNNING-END-TO-END)

---

### 6. **26/08/06 — DEMONSTRATOR-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION** (closed)
**Status:** closed | **Emoji:** 🎪️  
**Summary:** Migrated demonstrator plugin (LAST in crate-consolidation initiative) to one crate at ✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust, built to Shape V2. Owns NO document schema, NO app; purely re-exports six source plugins' apps (procedural, cad, puzzle, sourcing, process, gis) via seven pane components.

**KNOWN-FIXED:**
- Demonstrator consolidated to single crate (Shape V2 taxonomy)
- Seven #[path] declarations for pane components (generator, koordinator, aggregator, aussuchen, bearbeiten, verfolgen, bundle)
- All dependency paths verified against six plugins' current 📦️lib.rs
- Two legacy aliases renamed (puzzle_3d_ui→puzzle, process_3d→process)
- Two dead deps removed (semio-framework-core, infinite_canvas)
- All verification green: cargo check (native + wasm32-wasip2), clippy, tests (6/6 pass), bundle-manifest diff (10,949 lines byte-for-byte identical)
- Temporary verification overlays cleaned up; old crate deleted after verification

**KNOWN-BROKEN:**
- **ONE REAL BLOCKER (NOT FIXED):** `cargo build --target wasm32-wasip2 --profile wasm-release` fails at link: duplicate symbol `semio_plugin_install_bundle` / `semio_plugin_bundle_installer_link_shim`. Plugin_exports! expanded by semio_plugin! emits those two #[no_mangle] symbols; demonstrator links six expansions (from deps) plus its own. Structural consequence of consolidation; demonstrator is the ONLY crate in repo that can hit it. Every possible fix requires editing six source plugins' Cargo.toml/lib.rs and/or shared plugin_exports! macro (outside this ticket's ownership). Recommended fix: gate #[no_mangle] fns on `#[cfg(not(feature = "semio-plugin-embedded"))]`, declare in six deps, enable from demonstrator.

**DEFERRED:**
- **Registrar handoff:** Remove root Cargo.toml line "✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust", add "✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust"
- Duplicate-symbol fix (requires changes outside this ticket)
- Final workspace verification after registrar step

---

### 7. **26/08/14 — SCOPED-COMMANDS-AND-WINDOW-LOCAL-ACTIONS** (closed)
**Status:** closed | **Emoji:** 🎮️  
**Summary:** Implemented containment-derived OS, plugin, app, and mode command ownership; owner-qualified command and exact-window action dispatch; structured shortcut resolution; React/WGPU registry parity; Flow Generate and Animate migrations; canonical cross-platform fullscreen.

**KNOWN-FIXED:**
- Command ownership scoping (OS, plugin, app, mode levels)
- Owner-qualified dispatch for commands
- Exact-window action dispatch
- Structured shortcut resolution
- React/WGPU registry parity
- Flow Generate and Animate migrations completed
- Canonical cross-platform fullscreen implemented

**KNOWN-BROKEN:**
- None listed

**DEFERRED:**
- None listed

---

### 8. **26/08/17 — FIX-EDITORAPP-VIEWERAPP-APP-ID-PLACEHOLDER** (open—no JSON, markdown only)
**Status:** open (inferred) | **Emoji:** 🪪️  
**Summary:** `VcsArtifactApp::with_registry` used compile-time `A::APP_ID` placeholder ("surface") instead of runtime canonical id from `app.instance_id()`. Fixed four envelope IDs (document, config, draft, interaction) to use real derived id.

**KNOWN-FIXED:**
- VcsArtifactApp::with_registry now uses app.instance_id() for all four envelope IDs
- config_store and draft_store promoted to pub(crate) for test access
- Two new unit tests: editor_app_envelopes_carry_the_real_canonical_surface_app_id, viewer_app_envelopes_carry_the_real_canonical_surface_app_id
- Verification: cargo test -p semio-framework-plugin --lib surface_testkit_tests (8/8 ok), cargo test -p semio-s-plugin-space --lib (210/210 ok)

**KNOWN-BROKEN:**
- None listed

**DEFERRED:**
- None listed

---

### 9. **26/08/17 — FIX-SINGLE-WINDOW-SELF-DOCK-GHOST** (open)
**Status:** open | **Emoji:** 🪟  
**Summary:** When only one window is active and dragged, mode dock shows meaningless self-split drop affordance. Suppress self-split and root-split drop zones when no other windows remain after lifting drag source.

**KNOWN-FIXED:**
- (No details yet; ticket open, work pending)

**KNOWN-BROKEN:**
- Self-dock ghost affordance appears when dragging single window
- Drop zones show split options that don't work

**DEFERRED:**
- Implementation pending

---

### 10. **26/08/13 — INTRODUCTION-STEPS-WRONGLY-BEHIND-WINDOWS** (open—no JSON, markdown only)
**Status:** open (inferred; has markdown notes) | **Emoji:** (none)  
**Summary:** Introduction steps rendered behind target windows due to z-index stacking context. Removed z-tutorial from data-semio-portal-layer, moved portal layer after FrameworkOsShellInner, portaled elements now use root stacking context (fullscreen veil: 10000, elevated window: 10001, info box: 10002, popovers: 10003).

**KNOWN-FIXED:**
- Removed z-tutorial from portal layer container
- Portal layer repositioned after FrameworkOsShellInner
- Stacking context corrected (10000/10001/10002/10003 layers)
- Unit test added to verify unconstrained z-tutorial

**KNOWN-BROKEN:**
- None (appears resolved per summary)

**DEFERRED:**
- None listed

---

### 11. **26/08/17 — CORNER-WINDOW-CHIPS-WITH-INLINE-ACTIONS** (open)
**Status:** open | **Emoji:** 🥟  
**Summary:** Replace mode dock's separate right-side controls chip with per-tab inline actions (focus/maximize, new window, close) carrying localized tooltips and hotkeys. Give every window stack four corner tab groups (top-left, top-right, bottom-left, bottom-right) with drag-between-corners, one active tab per stack—across React, wgpu and TUI renderers plus shared layout schema.

**KNOWN-FIXED:**
- (No details; ticket open, work pending)

**KNOWN-BROKEN:**
- Current control chip design needs replacement
- Corner tab groups not implemented
- Inline actions not localized

**DEFERRED:**
- Full implementation across React, WGPU, TUI renderers
- Shared layout schema updates
- Hotkey and tooltip localization

---

### 12. **26/08/17 — DASHBOARD-WIZARD-WINDOWS** (open—incomplete JSON)
**Status:** open (inferred) | **Emoji:** (none)  
**Summary:** Rewrite semio TUI dashboard: single default wizard window, runtime command-tree discovery, PTY output fills window, fix chrome close/maximize/+ tab strip.

**KNOWN-FIXED:**
- (No details; work pending)

**KNOWN-BROKEN:**
- TUI dashboard needs rewrite
- Command-tree discovery not implemented
- PTY output not filling window
- Tab strip controls (close/maximize/+) broken

**DEFERRED:**
- Full TUI dashboard rewrite implementation

---

## ADDITIONAL RELATED TICKETS ON 26/08/17 (demonstrator/app/window/fixture-related)

### 13. **26/08/17 — END-TO-END-TAXONOMY-NORMALIZATION** (open)
**Status:** open  
**Summary:** Extend taxonomy mechanism: language-neutral assets at owner root, schemaFacetKinds, package purity gate, hoist WIT. Multi-stage verification including Kernel/actor/UI pool composition, resident ledger policies, node/WGPU renderer strict checks.

**KNOWN-FIXED:**
- ShardClientOptions.residentLedger required; constructor validates private field
- Shared actor binding verification complete (138/138 tests)
- UI resident pool envelope updated (192 bytes / 4 slots / 4 owners)
- Kernel caller regression suite: 49/49 passed (2.16 s)
- Renderer strict check: 52 diagnostics (7 tutorial, 4 actor UI-pool fixtures, 22 UI property conversions, 19 UI fixtures)

**KNOWN-BROKEN:**
- Renderer strict check shows 52 active diagnostics (tutorial and fixture-related)
- Fresh renderer test failed on intentionally absent policy module
- WGPU renderer and guest not executed
- Full app integration not proven

**DEFERRED:**
- Native pre-instantiation capacity proposal (queued for read-only review)
- Full renderer/guest proof (end-to-end app content not established)
- Host ledger retrospective funding (Wasm static memory not addressed)

---

## ALL TICKETS 26/08/06–28 WITH STATUS

| Date | Ticket Name | Status |
|------|-------------|--------|
| 06 | APPS-RUNNING-END-TO-END | closed |
| 06 | DEMONSTRATOR-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION | closed |
| 06 | FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION | open |
| 06 | FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION | open |
| 06 | FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION | open |
| 06 | FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH | open |
| 06 | FRAMEWORK-SURFACE-FAMILY-CRATE-CONSOLIDATION | open |
| 06 | PERIPHERY-PROJECT-JSON-AND-FINALIZATION | open |
| 06 | S-AND-PLUGINS-END-TO-END | open |
| 07 | DEFAULT-COLLAPSE-PANELS-AND-PANES | open |
| 07 | EXAMPLE-SHAPE-ASSETS-AND-TESTS | open |
| 07 | FIX-APPS-VERTICAL-LAYOUT-GROWTH-AND-HANG | closed |
| 07 | FIX-DEMONSTRATOR-FOCUS-TRANSITION-FLICKER | closed |
| 07 | FIX-DEMONSTRATOR-INTRODUCTION-STEPS-ON-WRONG-APP | closed |
| 07 | FIX-DEMONSTRATOR-MOBILE-BLURRED-BACKGROUND-ON-CARD | closed |
| 07 | GET-ALL-APPS-WORKING-END-TO-END | open |
| 13 | EXCLUDE-COMPILED-CODE-FROM-VCS | closed |
| 13 | SEMANTIC-COMMAND-NAMES | open |
| 13 | UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION | open |
| 14 | FIX-DEMONSTRATOR-DOCUMENT-PANEL-TOGGLE-DUPLICATION | open |
| 14 | FIX-RIGHT-BOTTOM-PANEL-GROUP-CUTOUT | closed |
| 14 | FLOW-CONTENT-THROUGH-GLASS-CHIPS | open |
| 14 | SCOPED-COMMANDS-AND-WINDOW-LOCAL-ACTIONS | closed |
| 17 | CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM | open |
| 17 | CORNER-WINDOW-CHIPS-WITH-INLINE-ACTIONS | open |
| 17 | DASHBOARD-WIZARD-WINDOWS | open |
| 17 | END-TO-END-TAXONOMY-NORMALIZATION | open |
| 17 | FINISH-HUB-SPACES-COLLABORATION-END-TO-END | open |
| 17 | FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG | open |
| 17 | FIX-EDITORAPP-VIEWERAPP-APP-ID-PLACEHOLDER | open |
| 17 | FIX-SINGLE-WINDOW-SELF-DOCK-GHOST | open |
| 17 | LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY | open |
| 17 | MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME | open |
| 17 | SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION | open |
| 17 | UNLABELED-CONTROL-HOVER-TOOLTIPS | open |
| 17 | ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS | closed |
| 18 | SERVER-FRAMEWORK-PRODUCT | open |
| 20 | COMPOSE-TO-PUZZLE5D-MIGRATION | open |
| 20 | INTERACTIVE-JOB-RUNTIME-REFACTOR | open |
| 20 | SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY | open |
| 23 | END-TO-END-TESTING-REFACTOR | open |
| 27 | SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING | open |
| 28 | DEMONSTRATOR-END-TO-END-ALL-APPS | open |

---

## CONSOLIDATED "STILL OPEN / LIKELY STILL BROKEN" LIST

### Critical for Demonstrator End-to-End

1. **FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG (26/08/17)** — open
   - Assumed fixed but not fully validated; may have residual issues
   - Full acceptance boundary (six panes interactive, version control) still required

2. **UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION (26/08/13)** — open
   - Architecture not yet implemented; three workstreams (state-lane, composition, demonstrator) incomplete
   - Port 6029 boot proof not validated
   - Two-tab presence/config/transient demo not implemented

3. **GET-ALL-APPS-WORKING-END-TO-END (26/08/07)** — open
   - Empty procedural windows after some time (fix attempted via hot-swap; status unclear)
   - May still be broken

4. **FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG (26/08/17)** — open
   - Shared renderer composition tests show 52 active diagnostics (fixtures, pool configs)
   - Native patch transfer and guest memory admission still missing
   - Full app content acceptance not proven

### UI/Window Interactions

5. **FIX-SINGLE-WINDOW-SELF-DOCK-GHOST (26/08/17)** — open
   - Self-split affordance in mode dock not yet suppressed

6. **CORNER-WINDOW-CHIPS-WITH-INLINE-ACTIONS (26/08/17)** — open
   - Not yet implemented; four corner tab groups, inline actions, drag-between-corners needed

7. **DASHBOARD-WIZARD-WINDOWS (26/08/17)** — open
   - TUI dashboard rewrite incomplete

### Other Demonstrator/App Issues

8. **FIX-DEMONSTRATOR-DOCUMENT-PANEL-TOGGLE-DUPLICATION (26/08/14)** — open

9. **FLOW-CONTENT-THROUGH-GLASS-CHIPS (26/08/14)** — open

10. **CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (26/08/17)** — open

11. **END-TO-END-TAXONOMY-NORMALIZATION (26/08/17)** — open
    - Renderer strict check shows 52 diagnostics requiring resolution
    - WGPU renderer and guest not yet executed

12. **FINISH-HUB-SPACES-COLLABORATION-END-TO-END (26/08/17)** — open

13. **SEMANTIC-COMMAND-NAMES (26/08/13)** — open

14. **DEFAULT-COLLAPSE-PANELS-AND-PANES (26/08/07)** — open

15. **EXAMPLE-SHAPE-ASSETS-AND-TESTS (26/08/07)** — open

### Framework/Infrastructure Still Open

- FRAMEWORK-OS-HOST-AND-DEV-CRATE-CONSOLIDATION
- FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION
- FRAMEWORK-REPO-PRODUCT-CRATE-CONSOLIDATION
- FRAMEWORK-SINGLETONS-AND-CORE-DE-SANDWICH
- FRAMEWORK-SURFACE-FAMILY-CRATE-CONSOLIDATION
- S-AND-PLUGINS-END-TO-END
- PERIPHERY-PROJECT-JSON-AND-FINALIZATION

---

## KEY HANDOVER NOTES FOR SESSION 08/28

1. **Demonstrator is BLOCKED** on duplicate wasm symbol `semio_plugin_install_bundle` (see ticket 26/08/06 demonstrator-plugin-migration). Registrar step + duplicate-symbol fix (requires changes outside that ticket) are prerequisites.

2. **State architecture still not implemented** (ticket 26/08/13). Three workstreams (state-lane unification, composition, demonstrator restoration) must complete before acceptance.

3. **Empty procedural windows** may still appear after time (ticket 26/08/07). Hot-swap SSE ordering applied but not fully validated.

4. **UI/window features incomplete**: Self-dock ghost, corner window chips, TUI dashboard all still open.

5. **Renderer composition** has 52 active strict diagnostics (ticket 26/08/17 end-to-end-taxonomy-normalization) blocking full proof.

6. **Demonstrator six panes** (Generator, Koordinator, Aggregator, Aussuchen, Bearbeiten, Verfolgen) are all wired but acceptance boundary (fresh artifacts, guest lifetime, window content, example interaction, close/reopen cycle) not yet proven.

