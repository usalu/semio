# Bands, Dialect, Tint Handoff (Lanes 5, 6, 7)

Paths below are relative to `🧰️framework/🔨️modules/🖱️ui/`.

## Lane 5: card stack gap rule

Audit verified: the sliver rule (`chipGapReadsAsSliver`, `CARD_BAND_MIN_GAP`) had real consumers that all existed only to close gaps between chips, which the rectangle model no longer has (a gap is content). Removed everything that served it; `snapCardHeight` / `snap_card_height` stay (CardStack layout uses them).

- `🧱️elements/🃏️CardStack/bands.{ts,rs}`, `bands.test.{ts,rs}`, `bands.schema.json`, `bands.cases.json`: only `snaps` remain; `gaps` removed from schema and cases; TS and Rust read the same cases.
- Deleted: `🧱️elements/🗂️WindowChrome/chip-abut.{ts,test.ts}`, `chip-abut.cases.json`, `chip-abut.schema.json`; `🧱️elements/🧭️PanelTabBar/trailing-space.cases.json` and its test in `shared-border.test.tsx`.
- Consumers updated: `🗂️WindowChrome/🟦️.tsx` (observers and cap/footer element state removed), `🪟️Window/🟦️.tsx` (pane abut observer removed), `🧭️PanelTabBar/🟦️.tsx` (last-tab stretch effect removed), `🎨️styling/🖌️ui/🎨️.css` (`[data-chip-abut]` rules removed), `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (chip-abut test entry removed).

## Lane 6: dialect

The four emphasis schemas were already 2020-12. The only draft-07 file of the TUI shared schemas was `⌨️tui/🧬️schema/📏️text-width/🪟️console/🔣️.json`; moved to 2020-12. Its only validator (`🛍️products/🦑️repo/.../🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts`) already uses Ajv2020; no code depends on draft-07 of these files. The `⏯️tool-run` and `📁️filesystem` draft-07 schemas are unrelated and untouched.

## Lane 7: chip tint

`emitPaletteTheme` in `🎨️styling/📽️projection/🟦️.ts` now emits `--chip-tint: <levels.chipTintPercent>%` (one added block in the TS part only); hand-written `--chip-tint: 10%` removed from `🎨️.css`. Regenerated only the CSS artifacts `🎨️styling/🎨️palette/🎨️.css` and `🤖️generated/🌓️palette-theme/🎨️.css` (diff is exactly the new line). New test in `🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts`.

## Verified

- `bands-dialect-tint-vitest.config.ts` (this folder, bun:test aliased to vitest) with `lockf ... node node_modules/vitest/vitest.mjs run`: styling-resolve and TUI emphasis, 22 passed. Ajv2020 check of all five TUI schemas against their fixtures: all valid (bun is not installed here).
- `compositor-vitest.sh bands chip-row WindowChrome PanelTabBar Window`: 206 passed, 1 failed.
- Open, not mine: `CardStack/chip-row.test.ts` "reads the token in WindowChrome/🟦️.tsx" fails because WindowChrome contains `min-h-medium` twice (pre-existing, WindowChrome lane).
- Rust: the prescribed `repo cargo ... test -p semio-ui-wgpu-light --lib` failed at planning ("requires dev-dependencies and is not a member of the workspace"), so it did not run; the Rust change is a pure deletion of `chip_gap_reads_as_sliver`/`CARD_BAND_MIN_GAP` and its test (no other Rust user, `light/🦀️.rs` only mounts the module).
- Python tokens/projection untouched; Rust `levels` output unchanged.
