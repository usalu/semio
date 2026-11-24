# 🌐️ Sourcing browser journey (2026-11-23)

## Status: BLOCKED at boot, journey not yet exercised against the real DOM

Polled up to the 60 min budget. Port 6081 came up (vite pid 11215, 200) but the sourcing plugin never became loadable.

## Probe evidence (`🗑️generated/probe/*-en-US.*`)
- beacon `error:sourcing`; body shows "Plugin Recovery / This program crashed." with Restart App / Disable Plugin; zero `[data-window-id]`, tables, canvases.
- Console: `http 404 /🔌️plugin-modules/🪵️sourcing/🔣️.json`, then `program module unavailable sourcing ... descriptor answered HTTP 404; ... extensions: no registry entry`.
- Root cause: the sourcing wasm and descriptor were never staged. `.cache/b/p/m/browser/*/modules/` still holds only `wfc`, `shard`, `vendor` (as of 06:2x). Activation (cargo wasm32-wasip2 build of `semio-hub-sourcing-component` plus transpile) had not finished; cargo/rustc were still compiling flow/stdio crates. This is an activation/build gap, not a Rust or TS app bug in sourcing.

## Files
- `probe-sourcing.ts`: loads page, waits for beacon, dumps console, page errors, failed requests, aria snapshot, DOM summary and screenshot.
- `sourcing-journey.ts`: 16 steps per language (en, de): boot faults, windows, ten kinds, grid canvas, add, set count, undo/redo, preview, remove, drag both ways, query/module/typology/min-availability filters, sort, Demo-Empty-Demo, reload persistence. Locators are role/label based (labels from `🗣️terminology/🦀️.rs`) but are UNVERIFIED guesses (stepper as spinbutton, module toggles as checkbox/button, selector as combobox/button, `data-window-id="sourcing-*"`). Adapt after the first boot with the probe's aria/dom dumps. The app exposes no known language switch; locales are driven by the browser locale (en-US / de-DE).
- `poll-6081.sh`: 30 s poller.

## Resume
Once `curl http://127.0.0.1:6081/🔌️plugin-modules/🪵️sourcing/🔣️.json` returns 200: run `bun probe-sourcing.ts`, fix locators, run `bun sourcing-journey.ts`.

## Results 2026-11-24

Native server `http://127.0.0.1:6081/`, `sourcing-journey.ts en de` (locators now role/name/id based: `tabpanel` per window, `window:sourcing-pool/...` filter ids, `data-drag-role="transfer"` handles, `playground.navbar.fixture`; German labels from `✏️s/🔌️plugins/🪵️sourcing/.../✏️editor/🗣️terminology/🦀️.rs`; peer `[DEBUG]` console lines (they arrive as console `error`) are excluded from faults; polling `eventually` replaces fixed sleeps).

| Step | en | de | Note |
|---|---|---|---|
| boot without faults | pass | pass | |
| four windows render | pass | pass | |
| Pool lists ten kinds and reuse rows | pass | pass | 12 rows |
| Grid 3D canvas | pass | pass | |
| curation add | pass | pass | |
| curation set count (stepper 1 to 3) | pass | pass | Count input is readonly; steppers only |
| undo / redo | pass | pass | |
| Preview renders selected kind | FAIL | FAIL | defect A |
| curation remove | pass | pass | |
| drag pool to curated and back | pass | pass | |
| filter by query | pass | pass | |
| filter by module | pass | pass | |
| filter by typology | pass | pass | |
| filter by minimum availability | FAIL | FAIL | defect B |
| sort by availability | pass | pass | asc then desc verified on the column values |
| example selector Demo, empty, Demo | FAIL | FAIL | defect C |
| archive save and reload persistence | FAIL | FAIL | defect D |

### App defects

- **A. Preview does not follow a row selection until some other render happens.** A row click dispatches `interactionSelect` (the command turn runs) but the Preview stays on "No selection"; after any unrelated edit (for example another Curated increment) it renders the selected kind with its canvas. So the selection is recorded and the Preview surface is not re-dirtied. The plugin side is correct (`🧩️sourcing-app/🦀️.rs` `render_with_request_context` reads `interaction.selection("rows")`). The fix is in the wasm reactor: an `interactionSelect` turn must dirty the surfaces that read the `rows` domain (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🎯️dirty-poll-surfaces/🦀️.rs`, `🎨️dirty-surface-render/🦀️.rs`; the doc comments there describe the same "selection lane never re-published after a pick" family). Needs a wasm rebuild; not done.
- **B. `setFilterMinAvailability` faults and wedges the plugin instance.** Typing a value or stepping with the arrow key logs `original-turn.semantic plugin.internal.prior-outcome: runtime live cleanup faulted for instance 1`, the filter does not apply, and every later action is refused (`sortTable refused: dispatch-failed`, further adds). Query, module, typology and sort edits do not do this, so it is specific to this Config mutation, the only one whose payload is a scalar `u32` (`SetFilterMinAvailabilityEdit`, `✏️editor/🎚️config/🦀️.rs:237`; its retained accounting is `(1, 0)` at `📬️config-store-preparation/🦀️.rs:77`; command at `🎮️commands/📉️set-filter-min-availability/🦀️.rs`). Suspects: the zero-byte item in the one-item preparation/footprint (`ArtifactStoreOneItemFootprint::for_leaf`) or the scalar leaf's publication; the first faulting step is not logged. Needs a wasm rebuild plus a Rust unit test through the real preparation factory; not done. The journey reloads the page after this step so later steps are judged independently.
- **C. The example selector offers only "Demo" (no empty example).** `NavbarExampleSelect` supports a "No example" row but `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx` never passes `includeNoExample`, and the selector normalises it to id `""` while the plugin command expects `EMPTY_EXAMPLE_ID` (`🎮️commands/🎬️set-active-example/🦀️.rs`). Not fixed (shared shell used by every app and the wgpu shell; id mapping needs a decision). Demo reselect works.
- **D. No archive save and no persistence in the web host.** The page has no save/archive control (Command palette, History panel, More menu checked), no IndexedDB and only `semio.os.*` localStorage keys; a reload returns to the Demo state. Archive round trip is proven at guest level only (`a_saved_curation_archive_carries_its_catalog_member_and_reloads_with_its_edits`). The step stays strict and fails with that evidence.

### Other observations

- React console error "two children with the same key" on the first Curated add (benign for the journey, not classified as a fault).
- The Curated row shows a duplicate "+"/trash pair beside the stepper (the "Curate" and "Remove" buttons of the More cell render inline).
- Not a defect: `aria-sort` is not set on column headers; sorting verified through the column values.

### Files

- `sourcing-journey.ts` (rewritten locators, German labels, polling, `[DEBUG]` filter, wedge isolation)
- this report; screenshots and `journey.json` in `🗑️generated/journey/`
