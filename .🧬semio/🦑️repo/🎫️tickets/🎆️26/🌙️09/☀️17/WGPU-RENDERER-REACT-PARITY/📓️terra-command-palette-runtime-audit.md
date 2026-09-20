# Terra — Command Palette Runtime Audit

## Finding

The WGPU command-palette failure is a **missing shell-overlay view path**, not a lost trusted chord, missing command data, or a stalled retained-frame lifecycle.

The recorded full-checkpoint-15 diagnostic is consistent with current source: step 38 sent trusted Meta+p, WGPU painted the centred glass and Search title, but exposed neither controls nor a dialog surface and left the browser active element as DIV. React exposed a dialog, focused ui.search.input, and command rows. The old artifact is diagnostic evidence only; this audit did not start a browser, activation, build, or generator, so it does not claim a new-source runtime pass.

The direct defect is the state-to-visible-controls transition:

1. WGPU's shortcut producer sets search_open, OverlayState::Search, clears the query, and puts an internal focus id at shell.search.input: 🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15464-15474.
2. The data producer is present. filtered_search_items ranks with React-equivalent weights, threshold, and limit; activate_search_item dispatches the selected action or command and closes the overlay: the same file:14539-14622.
3. The resumable chrome walker reaches Overlay and promotes hits only at terminal PersistPreferences: :21748-21835, :21928-22229, and :12665-12679. There is no source evidence that this lifecycle loses palette state.
4. render_overlay_step paints shared glass and title for Search/Find at phases 0–1 (:24138-24180). Its only row phase is limited to OverlayState::Dropdown("example") (:24182-24208). For Search, rows is empty and the glass immediately closes at :24194-24199. No input or command row is laid out, painted, or registered.
5. chrome_surface_census publishes a dialog only for the introduction tour, never Search or Find (:12681-12697). Chrome accessibility is derived from published hits; an Input hit becomes a textbox, but the palette supplies no input hit (:28398-28465). The generic accessibility Value route only knows widget metadata and has no shell-query owner (:13063-13117).

| Observed WGPU state | Source cause |
| --- | --- |
| Glass and Search title exist | render_overlay_step phases 0–1 run for OverlayState::Search. |
| No command rows | Phase 2 obtains rows only for the example dropdown. |
| No input control or accessible input | No HitKind::Input is registered; accessibility projects the published hit registry. |
| No dialog in dumpChrome | chrome_surface_census adds only the tour dialog. |
| Browser focus remains DIV | The chord writes worker-local focus, but no projected input-backed control exists for the browser accessibility mirror to focus. |

## React contract and usable WGPU pieces

React UISearch ranks, groups, and renders a CommandDialog, CommandInput id ui.search.input, and one CommandItem per result: 🧱️elements/🔎️ShellSearch/🟦️.tsx:28-95. The React Studio test waits for that dialog input, types into it, and asserts matching rows: 🧑‍💻dev/🧪️tests/🎬️studio/🟦️.ts:69-90 and :136-155.

| Piece | Status |
| --- | --- |
| Chord producer and internal query editing | Present. ToggleSearch establishes Search state and focus (:15464-15474); overlay keys update search_query and selected index (:14857-14905). |
| Ranked items and action resolution | Present in filtered_search_items and activate_search_item (:14542-14622). |
| Resumable render/publish discipline | Present. render_chrome_step carries its cursor across opportunities, then publish_retained_hit_registry atomically promotes the completed hit/accessibility frame (:21928-22229, :12665-12679). |
| WGPU unit coverage for chord/data semantics | Present but insufficient. 🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs:185-198 verifies the Find state toggle; its palette tests around :300-400 verify rows and activation, not a rendered Overlay frame or published hits/accessibility. |

The production fix should add a small first-class shell overlay model. It should not create a second command catalogue or a DOM-only exception.

## Smallest production regression packet

1. Extend the resumable render_overlay_step sequence with Search and Find branches. Across normal cursor opportunities, each must paint the dialog input, placeholder/current query, empty state, groups, and ranked rows; register an Input hit for ui.search.input or ui.find.input plus selectable hits; and route row activation through the existing activate_search_item or activate_find_item funnels. Route accessibility Value into the shell query and reset selected index.

   Keep a typed shell-overlay control/row plan shared by painting, hit registration, accessibility, and event routing. Do not encode command execution into paint-only text or rederive commands in the browser.

2. Make focus and surface projection concrete:

   - use the parity input ids in SHELL_OVERLAY_INPUT_IDS and shortcut focus assignment;
   - set the chrome accessibility-focused id when the palette opens;
   - add Search and Find to chrome_surface_census as a dialog while open;
   - project the current shell-query value rather than relying on absent widget_maps.input_metas.

3. Add one Rust regression beside the current shell shortcut/palette tests. Build a shell with a known palette item, open Search through the actual shortcut, and advance render_chrome_step until terminal frame publication. Assert published ui.search.input HitKind::Input and a known row hit; a Search dialog census entry; a focused textbox accessibility node with current query; filtering after a query; and existing action dispatch plus dialog close after selection.

4. Add one browser physical assertion to the paired journey after fresh activation. Send real Ctrl/⌘+P, then wait for a dialog in both observations, focused ui.search.input in WGPU accessibility and document.activeElement.id, and one deterministic row. Query a known fresh-session panel row such as panel.framework.category.display, select it, then assert the palette is absent **and** that panel is active. A blank glass, a received chord, or zero tool errors is not a command-palette pass.

After implementation lands, activate both canonical variants through the generated target scheme documented at 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs:1024-1053:

    bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-react-dev
    bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-wgpu-dev

Then run the ticket-owned journey against newly served URLs, limiting the first physical regression to the palette step:

    cd /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY
    SEMIO_PROBE_OUT=command-palette-regression SEMIO_PROBE_TARGETS=react,wgpu SEMIO_PROBE_REACT_URL=<fresh-react-url> SEMIO_PROBE_WGPU_URL=<fresh-wgpu-url> SEMIO_PROBE_ONLY=chord-command-palette bun 🐍️parity-interact-probe.mjs

The runner's declared command shape and URL inputs are at 🐍️parity-interact-probe.mjs:1-48. Add the postconditions above before treating this focused command as acceptance. Do not use the currently owned React/WGPU/Storybook sessions as acceptance of changed source. Put new evidence under this ticket's 🗑️generated/astra-runtime directory.

## Full journey probe classification

The journey schedules these actions at 🐍️parity-interact-probe.mjs:684-714. Its full-checkpoint-15 steps.json records resolution/rect/chord detail only; those receipts are not product outcomes.

| Full journey steps | Current helper behavior | Classification |
| --- | --- | --- |
| pane-chip-engagement-toggle and close; pane-chip-search-toggle and close; pane-chip-utilitybar-unfold; pane-chip-pane-fold; pane-chip-measures-unfold | clickPaneChip finds a suffix control, verifies the hit point, clicks, and returns the old rect. It does not wait for folded state, visible body, focus, or layout (:867-874). | Target delivery only. |
| split-gutter-drag | dragGutter locates a gutter, drags 120 px, and returns source rect/destination. It never reads a split ratio, changed bounds, or persisted layout (:859-866). | Gesture delivery only. |
| window-cap-focus | clickWindowCap hovers, resolves, verifies, clicks, and returns. It does not assert active-window or focus state (:780-800). | Click only. |
| window-cap-close | The individual step uses that same helper and does not assert that this cap removed a tab or selected a live window. | Click only. |
| window-close-last | closeAllWindows waits for tab count to decrease after every close (:801-811). | Real later sequence-level close outcome; it does not validate the preceding individual cap-close step. |
| window-reopen | It waits for exactly one added tab, a new window identity, and a live World3d body (:813-857). | Real consequence assertion. |
| chord-command-palette | The step only sends the chord (:707). | Event delivery only. The screenshot/structure delta exposed the missing feature; the helper could return without error while it stayed absent. |
| chord-fullscreen and exit | toggleFullscreen waits for browser fullscreen state to change (:774-778). | Real consequence assertion. |

The full probe is useful for target discovery and broad regression detection. It cannot establish parity for cap focus, an individual cap close, pane state, gutter layout, or command-palette usability until these postconditions exist.

## Execution note

Read-only source and existing diagnostics only. No activation, build, generator, server, browser, or artifact mutation was performed by this audit.
