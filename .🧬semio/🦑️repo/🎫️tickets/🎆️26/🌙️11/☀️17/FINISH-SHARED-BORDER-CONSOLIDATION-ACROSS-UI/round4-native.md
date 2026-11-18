# Round 4 native verification and interpreter wiring

Crate paths under `🧰️framework/🔨️modules/🖱️ui/`. Command: `repo cargo --store=isolated --configuration=prototype -- test --offline --manifest-path <crate>/Cargo.toml --lib` (script `🗑️generated/r4-run.sh`).

## 1. Build and test (all green, no source changes needed)

- No caller of the removed `resolve_card_bands_inset` remains: repo-wide grep finds only `bands.rs`, `bands.test.rs`, the CardStack wgpu target and the paint `text-and-structure-nodes` caller, all on `resolve_card_bands(cards, inset)`.
- semio-ui-wgpu-light 26/26, semio-ui-border-presentation 6/6, semio-ui-draw-contract 23/23 (builds its tests, so it is under the 4000 active-line boundary), semio-ui-wgpu-layout 37/37, semio-ui-wgpu-mounted-layout 24/24, semio-ui-wgpu-widgets 8/8.
- semio-ui-wgpu-paint compiles (0 lib tests); its paint-laws companion crate 69/69.
- semio-ui-wgpu-hosting 44/44 with `--features semio-ui-wgpu-mounted-layout/testkit` (arena test fixed).
- semio-ui-tui-laws 101/101 (the 4 previous failures are fixed).

## 2. Native card-stack inset

The native outline (`card_outline`, `paint_card_stack`) is painted at the card box edge: lines lie on the first pixel row or column inside the stack bounds, a hairline per shared-border segment. Inset is therefore 0 and both native callers correctly pass `0.0`. The web uses `WINDOW_SILHOUETTE_PATH_INSET = 1` only because the SVG stroke is centred on a path inset by one pixel; `resolve_card_bands(cards, inset)` carries both through the same `bands.cases.json` (cases with `inset` 0 and 1), which stays the single source of truth. No code change.

## 3. React Interpreter mapping for `cardStack` and `card`

The interpreter is the OS renderer's UI Interpreter (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/`, container frame in `🎛️control-port/container.tsx`). Changed `UiContainerFrame`:
- role `cardStack` renders the React `CardStack` (data/aria attrs, node id and key, children).
- role `card` renders `OverviewCard as="section"` (flush body so child cells share borders), title = container `label`, footer chip = container `description` (rendered in the right footer with `overviewCardChipClass`), heading id `<id>.title`.

Test: `🧪️tests/🪪️container-node-ids/🟦️.tsx`, new `interpreted card containers` case (stack id and `data-card-stack`, two `section[data-overview-card]`, headings, footer text present for the first and absent for the second, `aria-labelledby`). Interpreter suite: 196/196 (was 195).

## Files
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎛️control-port/container.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪪️container-node-ids/🟦️.tsx`
- `🗑️generated/r4-run.sh` (kept as input script)

## Open items
- The generated TS ui-contract type was not inspected for `card`/`cardStack` in `Component` roles (the typed wire and Rust contract already carry them); the interpreter test runs against the wire fixtures and passes.
- Footer chip is a plain chip (no action); an actionable footer would need a button role in the schema.
- Rust source-line boundary: none hit, no splits needed.
