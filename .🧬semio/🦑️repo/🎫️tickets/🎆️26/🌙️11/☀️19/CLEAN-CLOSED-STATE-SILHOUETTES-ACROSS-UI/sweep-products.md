# Sweep: Products and Apps onto Closed State Silhouettes

Scope: `inventory-state-lines.md` sections 4 (R1-R45) and 5 (S1-S22). Nothing under `🧰️framework/🔨️modules/🖱️ui/` was touched. Paths are relative to the repo root; `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `C` = `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react`, `ST` = `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`.

## Conventions used

- Borderless box: literal `data-shared-border-frame="" data-shared-border-edges=""`. `sharedBorderlessBoxProps` is not re-exported by `@semio-tech/ui-react/chrome` (open need 1), so products spell the two attributes out; the course has `BORDERLESS` and `FRAME` constants in `C/🔨️modules/🪟️chrome/🟦️.tsx`.
- Explicit state: `data-shared-border-state="active|hover"` on a box whose selection or hover comes from a store, not the DOM.
- Cell: `data-shared-border-cell`; a row of cells is a group (`ROW` in the course).

## Section 4 (R)

| Row | Done | Where |
|---|---|---|
| R1 node-tree row | ring removed, fills kept; row stays a frame (`edges=bottom`) | `E/🗣️Interpreter/🧱️node-tree/🟦️.tsx:451` |
| R2 surface view | focus ring removed; shell is now a borderless box (it is the focus target of the canvas hosts) | `E/🗣️Interpreter/🖼️surface-views/🟦️.tsx:117` |
| R3, R4 canvases | `outline-none focus:outline-none` removed; the canvases are not focusable (no tabindex, never `.focus()`ed), keyboard focus lives on the R2 shell, so the silhouette is the shell's | `E/🖥️Board2dHost/🟦️.tsx:66`, `E/🧭️TiledMapHost/🟦️.tsx:30` |
| R5 SelectionLabel | outline removed, borderless box; `aria-pressed` marks it | `E/🧩️BlockListHost/🟦️.tsx:71` |
| R6 local catalog file button | `focus-visible:outline` removed, borderless box | `E/🏛️ShellHost/🗂️local-catalog/📂️files/🟦️.tsx:48` |
| R7 spotlight input | kept (removal of a field's own ring, inside a frame) | unchanged |
| R8 paint tool cell | outline removed, compositor cell focus | `E/🖌️Paint2dHost/✍️editing/🟦️.tsx:34` |
| R9 paint application region | outline removed, `outline-none`, borderless box | `E/🖌️Paint2dHost/✍️editing/🟦️.tsx:384` |
| R10 control-port section | ring removed; `data-shared-border-state` (selected = active, hovered = hover) on the `SharedBorderRow` | `E/🗣️Interpreter/🎛️control-port/container.tsx:27,45` |
| R11 control-port record | ring and outline removed; same state attribute; `activityBorderClass` kept (loading and waiting go through the compositor marks); plain div becomes a frame (edges `""` unless it is a button) | `container.tsx:45-46` |
| R12 `ring` kind | excluded, type name | |
| R13 ink block | `ring-*` removed; frame, edges `""`, explicit state (hover over active) in `common` | `E/🖋️InkCanvasHost/🧱️ink-block-views/🟦️.tsx:91` |
| R14 ink block underline | excluded (run formatting) | |
| R15 ink overlay "U" | excluded (glyph of the underline command, static); buttons are already cells | |
| R16 time-travel buttons | static link typography kept, both buttons made borderless boxes | `E/🛠️ShellHelpers/⏪️time-travel/🟦️.tsx:419,516` |
| R17, R18 | excluded, static underline; the notice action buttons were boxed by the bare-button pass below | |
| R19, R20, R23-R29, R33, R34 | excluded, static | |
| R21 route chrome `ring` | excluded, a `kind` name, no state class | |
| R22 approval section | ring removed, borderless box with `data-shared-border-interactive` (a `tabIndex=-1` focus target) | `E/🤖️AgentApprovals/🟦️.tsx:168` |
| R30 diagram node `border-b`, R31 diff `border-l` | verified static, excluded | |
| R35 course card link | `focus-visible:underline` removed, `outline-none` added (the heading link lives in the title chip, the chip's closed silhouette takes the focus); hover underline kept | `C/🔨️modules/🪟️chrome/🟦️.tsx:96` |
| R36 energy clip fullscreen button | ring removed, borderless box | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx:35` |
| R37 playground stage item | dashed outline removed; borderless box; `aria-pressed` marks the selection | `ST/🔨️modules/🧪️playground/🖼️stage/🟦️.tsx:72` |
| R38, R39, R40, R42 | verified static (`border-t`, `border-l`, the clip cursor `border-l-2`), excluded | |
| R41 | excluded, static | |
| R43 staffage link title | `focus-visible:underline` removed, borderless box, hover underline kept | `ST/🔨️modules/🪟️chrome/🟦️.tsx:359` |
| R44 presentation reset | hover `border-color` removed (button is a cell in its chip group) | `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css:263` |
| R45 mit-bestand `h1` | kept (`focus:outline-none` on a programmatic focus target, no interactive state) | |

## Section 5 (S)

| Row | Done | Where |
|---|---|---|
| S1, S4 skip link | CSS rules removed; link is a borderless box | `C/🎨️styles/⚖️footer-and-forced-colors/🎨️.css`, `C/🔨️modules/📚️app/🟦️.tsx:450` |
| S2 forced-colours disabled border | `border-color: GrayText` removed (colour kept); the disabled stroke in forced colours is the compositor's (open need 2) | `🎨️styles/⚖️footer-and-forced-colors/🎨️.css` |
| S3 global `:focus-visible` ring | removed with the focus-visible fixture row in `🧪️tests/🧫️fixtures/🌗️contrast-states/🔣️.json` | same |
| S5 `.course-card-link:focus-visible` | removed; replaced by `outline-none` on the link (R35) | `🎨️styles/📊️plots-and-crowd/🎨️.css` |
| S6 field focus | rules removed; the input and unit select of a guess are cells of a group (`.course-guess-entry`), the identity input stays the single control of a cell | `🎨️styles/🏁️results-and-preferences/🎨️.css`, `C/🔨️modules/🧩️task/🟦️.tsx:238-263` |
| S7, S8 `.course-cell` focus rules, `--shared-border-flow` (5 places), `.course-cell` divider | removed. `.course-cell` keeps `align-self/margin/border:0/radius`. All its dividers (`.course-chip > select`, `.course-slot > select/button`, `.course-sort-up/down`, `.course-guess-unit`) removed. Rows (`li`) are now `ROW` (group + cell) and the select and the icon buttons carry `data-shared-border-cell` in classification, matching and sorting | `🎨️styles/🧩️task-layouts/🎨️.css`, `🔨️modules/🗂️classification`, `🃏️matching`, `↕️sorting` |
| S9 mockup scroller | outline removed; region is a borderless box | `🎨️styles/🖥️mockups/🎨️.css`, `🔨️modules/🖥️mockup/🟦️.tsx:102` |
| S12 relations summary | outline removed; borderless box | same, `:86` |
| S10, S11, S13 workbook mockup selection | kept deliberately: the workbook and the graph are a drawn illustration of another application (`td[data-selected]`, `[data-active]` sheet tab, `[data-highlight]`); nobody can hover, focus or select them, so they are content, not UI state | |
| S14 staffage global focus rule | removed; every interactive staffage control is a box (see below) | `ST/🎨️.css:3` |
| S15 pets story roster `aria-current` | out of scope: story-only page of the pets package, no `.semio-scope` and no compositor | |
| S16 | same as R44 | |
| S17 Bootstrap export | no app in scope renders it. It is a saved copy of the third-party BBSR Zukunft Bau page (`🌐️zukunft-bau-entwerfen-mit-bestand.html` and its `_files/`), served as static content and used only as a test mirror (`🪞️zukunft-bau-mirror`); the demonstrator never loads it. Vendored file untouched | |
| S18-S21 | same page copy (`main.css`, `Basic.css`): third-party content, not first-party theme CSS as the inventory said. Not edited | |
| S22 reveal region | UI module, other agent | |

## Additional controls made boxes (so no browser default ring is left)

- Course: `CardLink` (borderless), `BodyButton` (cell, tile = borderless, default = frame), every `summary`, leaderboard sort header, legal links, pets switch label, identity and challenge radio rows (a borderless wrapper of the single radio), `Records` was already a cell. `BORDERLESS`, `FRAME` are new constants next to `CELL`, `ROW`.
- Staffage: `Button` and `ButtonLink` (grouped = cell, quiet = borderless, otherwise frame; `PRESSED` lost `border-active-base`; `aria-current` links carry `data-shared-border-state="active"`), overview, gate, viewpoint focus, collections, clip event, provenance, timeline sliders and title button, brand link, skip link, all `summary`.
- OS, hub, demonstrator: 32 bare `<button>`, `<a>` and `<summary>` elements (close guard, bootstrap cancel buttons, artifact creation, notices, default-app toggle, canvas footer, locale picker, footer, media transport, spotlight, World3d status cancel, text editor completions, Paint cancel, staged args, ink overlay link) were made borderless boxes by script; elements whose parent is a single-control cell were skipped. mit-bestand footer partner links and the media-transport host element (`E/../🎬️media/🌐️browser/🎛️host/🟦️.ts`, a `tabIndex=0` host with `outline:none`) are boxes too. JSON tree toggle in the presentation too.

## Tests

Configs copied or created in this ticket: `vitest-course.config.ts`, `vitest-os-final.config.ts`, `vitest-interpreter.config.ts`, `vitest-teaching.config.ts`, `vitest-hub-admin.config.ts`, `vitest-demonstrator.config.ts`, `vitest-os-engine.config.ts`, `vitest-os-elements.config.ts`.

| Suite | Result |
|---|---|
| course | 1155 tests, all pass (one `load-failures` timeout appears only under full-suite load and passes alone, twice) |
| OS final (`vitest-os-final`) | 158 / 158 |
| interpreter in-source | 196 / 197. The failure is `loads all 77 corpus fixtures` finding 78: a corpus fixture added by someone else, not related |
| OS engine ink and renderer forms | 34 / 34 (both assertion sets updated to the new states) |
| OS element suites | 292 / 294, the 2 failures need Chromium (`MediaTransportHost` Chromium, empty-dock notice layout); 37 files are `bun:test` or helper files vitest cannot run, same as before |
| teaching (staffage and others) | 275 / 276, the failure is `catalog loading` (1 invalid asset, data under edit by someone else) |
| hub-admin | 27 / 28, `command-routing` (Nx test level) fails on the environment; `ownFrames` in the admin test now ignores borderless boxes (new Table row boxes) |
| demonstrator | 29 / 29 |

Updated tests: course `🧪️tests/🧱️shared-borders` (cell semantics, no flow variable, no `.course-cell` divider, no state-line rules, two new box tests), contrast-states fixture row removed, OS `ink-canvas-domain-interaction` and `renderer-types-and-forms` (explicit states instead of `ring-*`), hub-admin `ownFrames`.

## Open needs

1. `@semio-tech/ui-react/chrome` should re-export `sharedBorderlessBoxProps` so products stop spelling the two attributes.
2. Forced colours: the compositor should map the disabled stroke (`GrayText`); the course no longer declares it.
3. Native form fields (`<input>`, `<select>`) cannot hold an overlay: they rely on their cell, field frame or a wrapper box. Raw radios and checkboxes in the course sit in a borderless wrapper; the guess field and identity input are cells.
4. Static CSS layout dividers on non-cell course parts remain (`.course-guess`, `.course-tiles`, `.course-mockup`, table rows): they are layout, not state.
5. The wgpu browser host canvas (`E/../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts`, `tabIndex=0`, `outline:none`) draws its own focus natively; not a web compositor box. The staffage custom element and `semio-stage` shadow DOM styles keep their own `:focus-visible` outline: standalone web components with no compositor.
6. Canvas selection overlays (`border-2 border-accent` rectangles in NodeGraph, Paint navigator, World3d, ink handles) are content geometry, left as is.
7. Two UI-owned rules still compete in tests: the chip class from `overviewCardChipClass` carries its own focus utilities; the new course test strips those tokens and checks only course-owned classes.
