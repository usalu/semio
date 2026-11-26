# GIS Plugin Conformance Audit (read-only)

Scope: `✏️s/🔌️plugins/🌍️gis` (PL) incl. artifacts `🗺️gismap` (GM) and `🏔️gisterrain` (GT), `🏭️bridge`, `📦️packages`; and `🌎️hub/🧩️compositions/🌍️gis` (HUB).
Method: file listing and grep only. No source edits, no builds, no cargo. Path prefixes are abbreviated as above; GM = `PL/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any`, GT = same with `🗿️artifacts/🏔️gisterrain`.

## Summary

| Check | Result |
|---|---|
| 1. i18n (EN+DE) | Mostly compliant. 5 English-only runtime strings and several English-only error faults found. |
| 2. Language-agnostic tests + third-party cross-check | Most features covered. Gaps: `local-file`, `shell`, `exaggeration`, `view`/`example` command dirs (no dedicated tests); no third-party cross-check for overlays and text/binary codecs. |
| 3. Path length > 230 chars | None. Longest PL path is 193 chars (298 bytes). Hub max is 119 bytes. |
| 4. `[DEBUG]` / debug output / TODO / legacy | No `[DEBUG]`, TODO, FIXME, XXX, HACK, deprecated, or compat shims. One `eprintln!("[TRACE] ...")` without the `[DEBUG] ` prefix. Plain `console.log` only in check scripts and tests (by design). |
| 5. Runtime third-party deps | `serde` and `serde_json` are runtime dependencies used directly (not behind an interface). `geo`, `geojson`, `gltf`, `ajv`, `mathjs`, `three`, `d3-geo` are test/dev only. `🏭️bridge` has only in-repo crates. |

## Prioritized actionable items

1. **Localize five English-only UI fallbacks** (`Label::data(format!("Unknown body: {body_key}"))`):
   - `GM/👁️viewer/🦀️.rs:293`
   - `GM/✏️editor/🧭️play-app-editor/🦀️.rs:58`
   - `GT/👁️viewer/🦀️.rs:92`
   - `GT/✏️editor/🦀️.rs:649`
   - Fix: build `LocalizedLabel::native("Unknown body: …", "Unbekannter Body: …")` and resolve it through the view locale (as `GT/🧬️schema/🧭️terrain-descriptor/🎞️scene/🦀️.rs:60` already does).
2. **Localize or classify English-only `Fault` messages** reaching command results:
   - `GT/✏️editor/🎮️commands/🗺️overlays/🦀️.rs:71,104,157,162,167,219`
   - `GT/✏️editor/🦀️.rs:563,564,567,607` (for example "Missing overlay identifier", "Invalid overlay label", "Missing WGS84 vertices").
   - If these surface in the UI, make them bilingual. If they are developer-only protocol faults, state that in the docstring.
3. **Remove or re-prefix the stray trace log**: `GM/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs:236` uses `eprintln!("[TRACE] …")`. Rename to `[DEBUG] ` or delete it. This is a temporary log by the rule.
4. **Replace direct `serde_json` runtime use with the in-house pack/JSON interface**: `GM/🗺️tiles/🌍️gis-policy/🦀️.rs:10` (`use serde_json::Value`). The same pattern appears in the GIS codec and mutation runtime files. The bridge already uses `semio_framework_pack_json` (`PL/🏭️bridge/🦀️.rs:86`), so route through that.
5. **Add manifest-level interface for runtime serde**: `serde` and `serde_json` appear under `[dependencies]` in roughly 25 of the 29 `Cargo.toml` files (for example `GT/📦️packages/🦀️rust/Cargo.toml`, `GM/🧬️schema/📦️packages/🦀️rust/Cargo.toml`). Wrap them behind the framework pack crate (or remove the direct dependency) so no runtime crate depends on them directly.
6. **Add language-agnostic tests for features with no dedicated test dir**:
   - `GT/✏️editor/🎮️commands/📂️local-file/` (only `🦀️.rs`, no tests)
   - `GM/✏️editor/🎮️commands/🌐️shell/` (only `🦀️.rs`, no tests)
   - `GT/✏️editor/🎮️commands/🏔️exaggeration/` (only `🦀️.rs`, no tests; covered only indirectly by `GT/🧬️schema/🧬️mutations/🎚️change-exaggeration/🧪️tests/⛰️raises/`)
   - `GT/✏️editor/🎮️commands/👁️view/` and `GM/✏️editor/🎮️commands/👁️view/` and `GM/✏️editor/🎮️commands/💡️inference/` have no own test dir (coverage through parents only).
   - Each needs a shared JSON fixture under `🧫️fixtures` or `🔮️oracles` plus a `.feature` or JSON oracle.
7. **Add third-party cross-checks where missing**:
   - Overlays: `GT/✏️editor/🎮️commands/🗺️overlays/🧪️tests/🦀️.rs` has a Rust test and no third-party oracle; the only language-agnostic input is the editor-level `GT/✏️editor/🧪️tests/🧫️fixtures/🗺️overlay-history` fixture. Use the `geojson` crate or `ajv` on the overlay-history fixture.
   - Text/binary codecs (`GM` and `GT` under `🚪️io/📝️text` and `🚪️io/💾️binary`): the `.ksy`, `.spicy`, `.abnf`, `.g4`, `.ebnf` grammars have no executable third-party parser cross-check in tests. Add one (for example a Kaitai or ANTLR run) or document that the grammars are specifications only.
8. **Localize the plugin label**: `HUB/🦀️.rs:41` `.label("GIS")` is English-only. Use `LocalizedLabel::native("GIS", "GIS")` or a German-specific term, so the plugin name follows the locale mechanism.
9. **Decide on the attribution label**: `GT/🧬️schema/🧭️terrain-descriptor/🎞️scene/🦀️.rs:46` `label: "Mapzen Terrain Tiles"` is English-only and user-visible. It is a provider name, so either make the surrounding text bilingual with the provider name kept as-is, or document it as a proper noun exception.
10. **No action needed, recorded for completeness**: `GM/📦️packages/🟦️typescript/📜️script.ts` and test files use `console.log` for structured check output (`gis-…-check:` prefixes). These are not temporary debug logs. `PL/🏭️bridge/🦀️.rs:48,53` `eprintln!` usage messages are English-only CLI text and are acceptable for a developer tool.

## Check 1: i18n detail

Compliant (both EN and DE present):
- Rust labels via `LocalizedLabel::native(en, de)`: `GM/🪪️artifact-declaration/🦀️.rs:14,94,96`, `GM` `🎭️modes`, `GM/✏️editor/🧾️manifest/🦀️.rs:48–112`, `GT/✏️editor/🦀️.rs:681–720`, `GT/✏️editor/🎮️commands/📐️edit-elevation/🦀️.rs:97–118`, `GT/…/🗺️overlays/🦀️.rs:42–275`, `GT/…/🏔️elevation-source/➕️create-grid/🦀️.rs:67–77`.
- Terrain data panel bilingual: `GT/✏️editor/📌️panels/🔍️terrain-data/🦀️.rs:45–70`.
- Terrain source notice bilingual: `GT/🧬️schema/🧭️terrain-descriptor/🎞️scene/🦀️.rs:54–60`.
- Inference port UI (TSX): `GM/💡️inference/🪟️presentation/🟦️.tsx:16–66` uses `locale: "en" | "de"` with EN+DE tables and a required locale prop (no default).
- Mutation labels test enforces `{ en, de }`: `PL/🧪️tests/🧪️mutation-inputs/🟦️.ts:31`.
- Localization artifact capabilities: `GM/🪪️artifact-declaration/🦀️.rs:94–96` (`localization.en` and `localization.de`).

Non-compliant (English only):
- Items 1, 2, 8, 9 above.
- Test-only strings (`console.log` in `*/🧪️tests/`) are excluded by design.

## Check 2: language-agnostic tests and third-party cross-check

Legend: L = language-agnostic fixture, oracle or `.feature` present. X = third-party cross-check present in tests.

| Feature | Location | L | X | Notes |
|---|---|---|---|---|
| GeoJSON import/export | GT `✏️editor/🎮️commands/🌍️geojson` | yes (`🔮️oracles/🐍️.py`, `🧫️fixtures/🔣️.geojson`, `🔣️large.json`) | yes (`shapely`, `geojson` in oracle) | Covered |
| Import elevation (ASC) | GT `…/📥️import-elevation` | yes (`🔮️oracles/🐍️.py`, `slope.asc`, `nodata.json`, `fractional.json`) | yes (`scipy`, `rasterio`, `numpy`) | Covered |
| Contours | GT `…/📉️contours` | yes (`🧪️tests/🔣️.json`, `🔣️.json`) | yes (`shapely`, `matplotlib` in `🧪️tests/🐍️.py`) | Covered |
| Grade region | GT `…/📏️grade-region` | no shared fixture | yes (`ajv`, `three`) | Partial: no JSON fixture |
| Edit elevation | GT `…/📐️edit-elevation` | no shared fixture | yes (`ajv`, `mathjs`) | Partial: no JSON fixture |
| Create grid (elevation source) | GT `…/🏔️elevation-source/➕️create-grid` | no shared fixture | yes (`ajv`, `mathjs`) | Partial: no JSON fixture |
| Overlays | GT `…/🗺️overlays` | yes (`🧫️overlay-history`) | no | Gap X (item 7) |
| Exaggeration | GT `…/🏔️exaggeration` | via mutation `.feature` and `🔮️oracles` | via `semio_repo_test` only | No dedicated tests (item 6) |
| Local file | GT `…/📂️local-file` | no | no | No tests (item 6) |
| Example | GT and GM `…/🎨️example` | via `🧩️example` tests | no | Smoke only |
| View command | GT and GM `…/👁️view` | via window oracles and `🧫️` fixtures | partial (`ajv` in window tests) | No own test dir (item 6) |
| Terrain window / mode | GT `…/👁️view/🪟️windows/🏔️terrain/` | yes (`🔮️oracles/🔣️.json`, `set-camera-applied`) | yes (`ajv`, `gltf`) | Covered |
| Map window / mode (edit, view) | GM `…/✏️edit/🪟️windows/🗺️map` | yes (many `🧫️fixtures/…/🟰️nop`, `🔮️oracles`, `.feature`) | yes (`ajv` in `🔬️window/🟦️.ts`) | Covered |
| Map features command | GM `…/🗺️features` | yes (`.feature` for mutate-gismap-1) | partial (`geojson` and `geo` dev deps) | Covered |
| Shell command | GM `…/🌐️shell` | no | no | No tests (item 6) |
| Inference command and service | GM `…/💡️inference` plus `🌉️mcp`, `🔌️client`, `👷️worker` | yes (`🧫️fixtures`, `🔬️oracle`) | yes (`ajv`) | Covered |
| Tiles (policy, style, prefetch) | GM `🗺️tiles` | yes (`🧫️fixtures/🔣️.json`, `🌍️gis-policy/🧫️fixtures`) | yes (`d3-geo`, `geojson`, `geo`) | Covered |
| SQLite codec (GM, GT) | GM and GT `🚪️io/🪶️sqlite` | yes (`🧫️fixtures/🔣️.json`) | yes (`ajv`; `bun:sqlite` is runtime, not oracle) | Covered |
| Text codec (GM, GT) | `🚪️io/📝️text` | yes (`snapshot`, `diff`, `mutations`) | no executable third-party parser | Gap X (item 7) |
| Binary codec (GM, GT) | `🚪️io/💾️binary` | yes (`snapshot`, `diff`, `mutations`) | no executable third-party parser | Gap X (item 7) |
| glTF export | GT `…/🧬️schema/…` | yes | yes (`gltf` dev dep) | Covered |
| Hub compositions | HUB `🧪️tests/*` | yes (`🧫️fixtures/*`) | yes (`ajv`) | Covered |

## Check 3: path length

- No file or directory under PL or HUB exceeds 230 characters. Longest PL path: 193 chars (298 bytes), `GM/…/🔬️window/…/📸️snapshot/⬅️before/🔣️.json`. Longest HUB path: 84 chars.
- Note: byte length exceeds character length because of emoji. If the 256 limit is in bytes, the PL max (298 bytes) exceeds it. Check the rule's intent before changing anything. (The file list is in `/private/tmp/claude-501/-Users-ueli-Documents-semio/dbe2eee4-52b9-4bb8-b567-a0f2a20320aa/scratchpad/plugin-files.txt`.)

## Check 4: debug, TODO, legacy

- `[DEBUG]` prefix: none found.
- TODO, FIXME, XXX, HACK: none found.
- deprecated, legacy, compat shims: none. The hits are false positives: `compatible_position` (geometry validation), `future_incompatible` (lint config), `backwards` (inverse mutation variable), `fallback` (value defaults).
- Debug output: `GM/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/🎚️config/🧪️tests/🔬️window/🦀️.rs:236` `eprintln!("[TRACE] …")` (item 3).
- Plain `console.log` in `🧪️tests` and `📜️script.ts` check scripts: structured output, not debug leftovers.

## Check 5: runtime third-party dependencies

- Rust runtime `[dependencies]`: `serde`, `serde_json` (workspace) in most runtime crates, used directly (`GM/🗺️tiles/🌍️gis-policy/🦀️.rs:10`). Violates "not behind an interface" (item 4, 5).
- Rust dev only: `geo`, `geojson`, `gltf`, `tokio` (`dev-dependencies` only). Compliant.
- `PL/🏭️bridge/Cargo.toml`: in-repo crates only. Compliant.
- TypeScript `package.json`:
  - `GM/📦️packages/🟦️typescript/package.json`: `d3-geo` and `fast-deep-equal` are `devDependencies`. `d3-geo` is used only in `GM/🗺️tiles/🧪️tests/🟦️.ts:3`. `fast-deep-equal` is only a dynamic import in `script.ts`. Compliant.
  - GT TypeScript package: only `@semio-tech/framework-os`. Compliant.
  - Test-only third-party imports (`ajv`, `mathjs`, `three`, `bun:sqlite`): in `*/🧪️tests/` only. Compliant.
- Hub Rust: `geo` is `dev-dependencies` only in `HUB/📦️packages/🦀️rust/Cargo.toml`. Compliant.

## Files read

- `PL/README.md`, `PL/AGENTS.md` (not edited)
- Plugin file list (1114 files), hub file list (30 files) from `…/scratchpad/plugin-files.txt` and `…/scratchpad/hub-files.txt`
- Cargo and package manifests (29 `Cargo.toml`, 2 `package.json` in PL, 2 in GM and GT)
- Presentation `GM/💡️inference/🪟️presentation/🟦️.tsx`, `PL/📦️packages/🟦️typescript/📜️script.ts`
- Representative test and oracle files for each feature in the coverage table
