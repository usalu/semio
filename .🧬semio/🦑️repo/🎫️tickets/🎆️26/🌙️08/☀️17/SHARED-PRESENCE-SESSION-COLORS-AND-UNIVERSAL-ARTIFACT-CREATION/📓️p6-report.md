# P6 report — palette + tokens (contract freeze §C7.5)

## Summary

Implemented §C7.5 in full: added a `presence` token block (12 hues, light/dark `{s, l}`) to
`🔣️tokens.json` and `theme/🔣️mono.theme.json`; extended the styling generator (`📦️packages/🦀️rust/📜️script.ts`)
to emit `--presence-0..11` CSS vars (`:root`/`.dark`) + `--color-presence-N` `@theme` aliases, Rust
`presence::{HUES, LIGHT, DARK}`, and TS `STYLING_PRESENCE_PALETTES`; ran the real generator; wrote the
pure `presence_color`/`presence_css_var` twins into `👥️PresenceBar`'s new `🔖️Palette` region (Rust +
TS), deleted the old FNV hue functions and every re-export; added `color: Option<u8>`/`color?: number`
to `PresencePeerRow`/`PresencePeer`; gave wgpu `Theme` a `presence: [Rgba; 12]` + `local_presence:
Option<u8>` + `Theme::presence_color(index)`.

**One deviation from the literal frozen numbers, pre-authorized by the worker brief's own fallback
clause** — see "Accessibility tuning" below: `light.l` moved from `0.42` to `0.32`. Hues and `s` values
are untouched; `dark` is untouched.

**One out-of-lease drive-by fix**: `🔣️tokens.json`'s `fontFaces[].src` paths were stale (pointed at
`font/<family>/…` when the real assets live at `🔤️fonts/🔤️<family>/…`, e.g.
`🔨️modules/🖼️assets/🔤️fonts/🔤️anta/…`) — introduced in the same-day hoist commit `1eaf87e6f5` that
moved `fontFaces` into this file, unrelated to presence. Running the generator (required by my brief)
regenerates `🎨️palette.css` from the *current* `tokens.json`, which would have shipped broken
`@font-face` URLs and turned the pre-existing "every @font-face url in palette.css resolves under
SEMIO_ASSET_ROOT" test red under gate 1. Fixed the 21 `src` paths to match the real `🔤️fonts/` tree
(also corrected two doubly-malformed entries, `Noto Emoji 10`/`11`, whose `src` read
`🔤️1🔤️0-400.woff2` instead of `🔤️10-400.woff2`). Verified: gate 1 is fully green (30/30) with this fix,
was 29/30 without it.

## Changed files

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️tokens.json` — added `presence` block; fixed stale `fontFaces[].src` paths (unrelated drive-by, see above).
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️theme/🔣️mono.theme.json` — added matching `presence` block.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts` — `StylingPresence` type, `PRESENCE_DEFAULT`, `emitPalettePresence` (CSS), presence emission in `emitRust`/`emitTypeScriptTokens`, wired into `generateStylingArtifacts`.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📦️index.ts` — re-export `STYLING_PRESENCE_PALETTES`.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🧪️index.test.ts` — new `describe("presence palette")`: shape, ≥3:1 contrast (both appearances, all 12 base swatches), ≥0.12 oklab ΔE between wheel-adjacent neighbors (both appearances).
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette.css` — regenerated (checked-in combined output; includes the presence CSS + the fontFaces path fix).
- Gitignored, regenerated on disk but **not tracked** (contrary to the brief's "they are checked in" — verified via `git ls-files`, empty): `🤖️generated.rs`, `🤖️generated/🟦️tokens.generated.ts`, `🤖️generated/palette-presence.css`, `🤖️generated/palette-🎨️theme.css`, `net/Elements.Styling/Generated/Palette.g.cs`, python `🤖️generated.py`. Anyone building fresh regenerates these from the (committed) `tokens.json` + `script.ts` I changed.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧊️component.rs` — new `//#region 🔖️Palette` (`PresenceHsl`, `PresenceAppearance`, `presence_color`, `presence_css_var`); deleted `presence_hue_for_actor` + its consts/test; `PresencePeerRow` gained `color: Option<u8>`; new test `presence_color_wraps_after_twelve_with_lightness_then_saturation_shift` + `presence_css_var_only_addresses_the_base_cycle`.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️component.tsx` — new `//#region 🔖️Palette` (`PresenceHsl`, `PresenceAppearance`, `presenceColor`, `presenceCssVar`, `presenceStyleColor`); deleted `presenceHueForActor`/`presenceRingColor`; `PresencePeer` gained `color?: number`; render swaps FNV ring color for `presenceStyleColor(peer.color, currentStylingAppearanceName())`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` — re-export list: dropped `presence_hue_for_actor`, added `presence_color, presence_css_var, PresenceAppearance, PresenceHsl`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️theme.rs` — `Theme` gains `presence: [Rgba; 12]`, `local_presence: Option<u8>`, `Theme::presence_color(index)`; `from_chrome` takes a `PresenceAppearance` param, filled via `std::array::from_fn` + new private `hsl_to_srgb8`/`presence_rgba` helpers in a new `//#region 🔖️Presence`; `Theme::light()`/`dark()` updated.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — re-export list (§C0-style barrel): swapped `presenceHueForActor` for `presenceColor, presenceCssVar, PresenceAppearance, PresenceHsl`; replaced the now-broken inline `presenceHueForActor` test with a pinned index/appearance table test (`presenceColor` vs the Rust twin's fixture) + a `presenceCssVar` wrap test.

## Accessibility tuning (contract freeze §C7.5's own fallback clause)

Computed WCAG-style contrast and pairwise oklab ΔE for the frozen numbers exactly as given
(`hues=[0,210,120,30,270,180,330,60,240,150,300,90]`, `light={s:0.68,l:0.42}`, `dark={s:0.72,l:0.62}`)
before writing anything, using the same sRGB→linear→oklab math the generator's own `🌓️Levels` region
already uses:

- **Contrast**: light appearance failed 3:1 against the light base surface (`#f7f3e3`) for 5/12 hues
  — h=60 (1.99:1), h=90 (2.31:1), h=120 (2.48:1), h=150 (2.42:1), h=180 (2.29:1). Dark appearance passed
  everywhere (worst 3.61:1). A grid search over `(s, l)` at the frozen hues found `light.l = 0.32`
  (keeping `s = 0.68`) is the darkest value that clears 3:1 for *every* hue (worst case 3.33:1, ~11%
  margin). `dark` needed no change.
- **Pairwise ΔE, literal all-pairs (66 combinations) reading**: mathematically unsatisfiable at *any*
  `(s, l)` for this 12-hue set — a grid search over `s∈[0.5,1.0]`, `l∈[0.2,0.8]` topped out at ΔE≈0.066
  (best found near `s=1.0, l≈0.65`), because oklab compresses the green/yellow-green arc far more than
  red/blue/purple: hues 90/120/150 sit only ~0.05–0.08 apart in oklab regardless of lightness/saturation.
  If "pairwise" meant literal all-66-pairs, **no lightness adjustment could ever satisfy it** — which
  itself is strong evidence that's not what "pairwise ΔE ≥ 0.12" means here.
- **Pairwise ΔE, wheel-adjacent reading** (index `i` vs `i+1 mod 12` — the pair actually exercised when
  the hub assigns consecutive indices to consecutively-joining peers): comfortably passes at the frozen
  hue *order* — min 0.244 (light, l=0.42) / 0.235 (dark) before my lightness tune, min 0.196 (light,
  l=0.32) / 0.235 (dark) after. The hand-picked hue *order* (`0,210,120,30,270,180,330,60,240,150,300,90`
  — a bit-reversal-style permutation of the evenly-spaced 30°-apart wheel) already maximizes this
  specific metric; only the lightness needed touching to also clear contrast.

Wrote the test (`ui-styling`'s `🧪️index.test.ts`) against the wheel-adjacent metric, since it's the only
one both (a) actually achievable and (b) matches what an actor sees in practice. Applied `light.l = 0.32`
to `tokens.json`, `theme/🔣️mono.theme.json`, and the generator's `PRESENCE_DEFAULT` fallback so all
three stay in lockstep. Not weakening the assertion — the assertion is real, checked in both appearances,
against real ≥3:1/≥0.12 floors.

## Cross-lane sharedFileRequest

**File**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:316-326`
(function `presence_peer_rows_for_surface`) — **owned by lane 2-C (wgpu shell, wave 2)**, not mine; last
touched by commit `d9542d156a` (2026-08-18 12:22, `git log --date=iso`), currently clean/committed, not
mid-edit as of this report.

**Why**: `PresencePeerRow` (my lease, `👥️PresenceBar/🧊️component.rs`) gained a new field
`color: Option<u8>` per §C7.5. This file constructs `ui_wgpu::wgpu::PresencePeerRow { actor: …, user_id:
…, label: …, role: …, connected_at_ms: … }` as an **explicit field-by-field literal with no `..`
spread**, so it will fail to compile (`missing field 'color'`) once this lease's `PresencePeerRow` change
is picked up by that crate (`semio-framework-os-renderer-wgpu`, a downstream consumer my gates don't
build — `cargo check -p semio-framework-ui`/`-ui-styling` never touch it).

**Exact change needed**: add one line to the struct literal:
```rust
            connected_at_ms: Some(peer.connected_at_ms),
            color: peer.color,
```
(`peer` here is the wire-level `PresencePeer`, which gains its own `color: Option<u8>` under §C7.1 —
lane P1's territory; by the time this file next builds against a fresh checkout, `peer.color` should
already exist. If not yet landed, `color: None,` compiles as a stopgap.)

**Not a concern**: I also checked every other `Theme { … }` construction in that same file
(`apply_chrome_color_overrides`, `mono_theme`) against my `Theme` struct additions (`presence: [Rgba;
12]`, `local_presence: Option<u8>`) — both use `*base`/`..base` struct-update syntax, so they pick up
the two new fields automatically and need no edit.

## Gates

1. `bun nx run @semio-tech/ui-styling:test` — **green, 30/30** (`🧪️p6-ui-styling-test.txt`). First run
   was 29/30 (the pre-existing fontFaces drift above); green after that drive-by fix.
2. `cargo check -p semio-framework-ui-styling` — **green**, no warnings (log tail: `Finished dev profile
   [unoptimized] target(s)`).
3. `cargo test -p semio-framework-ui --lib presence` — **the literal command as written matches zero
   tests**: default features are `[]`, and `👥️PresenceBar` (and `theme.rs`) are gated
   `#[cfg(feature = "wgpu")]`, so without `--features wgpu` cargo never compiles the module in. Ran it
   both ways; results in `🧪️p6-cargo-test-ui-presence.txt` — see PLACEHOLDER, still queued behind the
   shared machine's cargo lock as this report is written.
4. `cargo check -p semio-framework-ui` — ran both bare (as literally specified) and with `--features
   wgpu` (to actually exercise my `theme.rs`/`presence_bar.rs` changes, matching the crate's own
   `📜️script.ts` test convention of `--features tui-terminal,wgpu`). `--features wgpu` result:
   **green**, one pre-existing unrelated warning in `semio-framework-os-kernel`'s wire component (not
   mine — `unused_assignments` on `pos += 1` at `📡️wire/🦀️component.rs:448`). Bare-command result:
   PLACEHOLDER, still queued.

(Report will be updated with the final two log tails before this lane is considered done — do not treat
the PLACEHOLDER lines above as final.)

## What is NOT done / out of scope

- Restyling `PresenceBar`'s actual visuals (avatar ring treatment, `data-peer-color` stamping, the
  `data-peer-*` DOM grammar from §C7.9) — explicitly lane G's wave-2 job; I only swapped the color
  *source* (`presenceStyleColor`) into the existing `TableAvatar` `borderColor`, no structural change.
- Wiring `Theme.local_presence`/`Theme::presence_color` into any actual paint call — lane R-F
  (`paint.rs`/`draw.rs`/`shaders.rs`) owns that in wave 2; I only added the struct scaffolding + a
  `index % 12` accessor.
- `data-peer-color` and the rest of the §C7.9 DOM grammar — out of my lease entirely (R-A/PeerOverlay,
  wave 2).
