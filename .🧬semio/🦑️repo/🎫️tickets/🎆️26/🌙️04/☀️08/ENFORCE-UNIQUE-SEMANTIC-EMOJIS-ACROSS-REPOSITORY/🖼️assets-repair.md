# Framework Asset Repair

This scope is still in progress. Catalog icons, lists, cursors, badges, logos and introduction artwork are repaired; fonts, metabolism and other asset families still require individual review. This is not a whole-assets or whole-workspace completion claim.

## Handpicked Icon Tree

All 249 source SVGs were moved individually with explicit commands. No script selected names, generated rename commands, or replaced text in bulk. Each public icon ID is unchanged. All 249 source SHA-256 hashes match the current-byte snapshot taken immediately before these moves.

The domain groups were chosen manually. Emojis describe the icon subject, not its SVG format. The projection family uses single keycap graphemes for one-, two-, and three-point perspectives; cabinet, cavalier, military and curvilinear projections have distinct subject-specific symbols. A second manual review separated time from people, used a brain for CPU, and gave puzzle and component their direct piece/brick symbols.

| Group | Icons |
| --- | ---: |
| 🏗️construction | 4 |
| 🛠️applications | 19 |
| 📊️data | 13 |
| 🪟️layout | 16 |
| 📚️documents | 12 |
| ✍️editing | 14 |
| 🗂️filing | 4 |
| 🎬️media | 11 |
| 🔗️connections | 4 |
| 👥️people | 4 |
| 🕸️graphs | 18 |
| 🔐️security | 3 |
| 🧭️navigation | 14 |
| 🕰️time | 5 |
| 🖱️interaction | 7 |
| 🧑️‍💻️development | 7 |
| 📝️notes | 3 |
| 🎛️controls | 8 |
| 🔷️shapes | 11 |
| 💻️devices | 7 |
| 🎨️drawing | 10 |
| 📐️projection | 15 |
| 🔄️transforms | 9 |
| 🔎️viewing | 12 |
| 🚦️feedback | 13 |
| 🌍️geography | 6 |

The canonical asset generator now reads source groups, keeps public IDs separate from paths, rejects duplicate IDs and linked source entries, and carries the exact handpicked source path into generated Rust SVG copies. It does not assign emojis. The generated mirror is `🖼️icon_svgs`; the shortcode implementation is `🔤️shortcodes.ts`. Incoming exports and current generator ownership fields were updated exactly. The source input pattern includes grouped SVGs but not generated mirrors.

The WGPU native icon builder now reads those same source groups, preserves relative paths and emits bare public IDs. Its generated Rust include is `🧩️icons.rs`, its logo is `🪧️semio_logo.svg`, and IconRenderHost's include was updated. The actual builder was compiled and executed in ticket-generated output, not merely source-inspected.

## Reserved README and Recovery

The existing asset `README.md` and redundant `📃️readme/📝️.md` had identical SHA-256 `a4b6beb0fe5824af8f79f38e1978c2118a5b48ae33a53ccacf10dd638418b09b`. The literal reserved README remains canonical. The duplicate was moved intact to `📖️duplicate-assets-readme.md` in this ticket, and its now-empty directory was removed. The generator and current output ownership now retain `README.md`; its instructions identify the real grouped source directory and Nx build route.

The old 249-file generated icon mirror was moved intact to `🗃️prior-icon-svgs` in this ticket before canonical regeneration. It is recovery input, not disposable compiler output. The final generator preview reported 286 files and no stale removals before the build ran. The manually reviewed nine follow-up path changes were also applied directly to their generated copies before rebuilding, so that rebuild deleted nothing.

The digest-locked historical README ownership catalog was not edited. Its 40 rows remain readable. Its internal historical generator claims remain checked; only the effective current output comparison follows the reserved-document statute, so the old catalog cannot force a live generator to recreate a relocated README.

## Verification

- Final icon audit: 558 entries, zero naming findings, zero independent emoji-regex discrepancies, and zero unresolved directory roles. This includes generated copies; it is not an exemption for generated names.
- Source/native parity: 249 source SVGs and 249 compiled native builder entries; zero changed source hashes, missing IDs, path mismatches or raw/normalized copy mismatches. The public IDs still match the external shortcode catalog.
- Canonical assets build and check-generated Nx routes passed; all 286 deterministic outputs are fresh.
- UI React Nx long route: 692 tests passed. UI native Nx long route: 235 tests passed. These consumer runs followed the main grouping and generated-path change; later refinements preserved IDs and SVG bytes and were checked through the compiled builder and canonical generator.
- Added a schema-first 15-case language-neutral SVG path fixture. The initial regression failed because the source identity function was absent. It now runs through Bun and independent TypeScript transpilation, with emoji-regex and picomatch as independent path oracles. Generated Rust path/ID separation is separately checked through both compilers.
- Added a reserved generated-README regression. It failed before the output contract was corrected; it checks the literal output, the unchanged frozen catalog bytes, and both compiled generator implementations.
- A temporary output-root ordering error was fixed immediately. An intermediate neutral run later had 25 passes and one failure from another agent's in-flight JCO/scale live output paths; those references were not suppressed. A fresh full focused run is recorded separately once the shared catalog settles.
- The settled full focused run passed all 26 tests with 627 assertions (`assets-neutral-final.txt`).

Detailed current-byte, native-output and audit evidence is under this ticket's `🗑️generated/assets-*` outputs. Retain the Markdown report and recovery inputs; only actual generated diagnostics are disposable at final ticket cleanup.

## Asset Lists

Seventeen list files were individually renamed without changing contents: licenses `⚖️licenses.csv`; adjectives `💬️`; animals `🦊️`; global-life-cycle-cost taxonomy `💶️`; qualities `🧭️`; area qualities `📐️`; performance assessment `📈️`; life-cycle-cost records `🏦️`; warming-potential records `🌡️`; original taxonomy `📜️`; structured tags `🔖️`; tag phrases `🏷️`; individual tag tokens `🧩️`; grayscale image `🩶️`; palette image `🎨️`; color Grasshopper document `🖍️`; and MIME table `🪪️mimes.csv` inside its existing `📋️mimes` owner. The source data and historical fixture contents were not rewritten.

The direct list audit covers 18 entries with zero statute/Unicode-oracle findings and confirms the unchanged 17-file SHA-256 multiset. Actual MIME directory ancestry resolves through the existing `asset-table-subject` role. Targeted authored-code searches found no incoming literal references to these previous paths; historical README fixture links remain unchanged as fixture data.

## Cursor Artwork

All 53 cursor SVGs were moved individually into 14 explicitly named interaction groups: default `🖱️`, pointer `👆️`, grab `✋️`, grabbing `✊️`, crosshair `🎯️`, move `🧭️`, not-allowed `🚫️`, east/west resize `↔️`, north/south resize `↕️`, northeast/southwest resize `↗️`, northwest/southeast resize `↘️`, selectable `☑️`, foldable `🪭️`, and pointing-source `👈️`. Each group's actual variants are `☀️light.svg`, `🌙️dark.svg`, `✏️light-source.svg`, or `🖋️dark-source.svg`; absent variants were not fabricated. The single fitting `📐️cursors.3dm` authoring document was left intact.

The exact 42 CSS URLs were updated without changing hotspot coordinates, fallback cursor values, or light/dark artwork selections. That includes existing selections of editable-source SVGs and the existing light grabbing artwork in the dark theme; no unrelated appearance change was introduced. The direct audit covers 68 entries and all 54 original file hashes, with zero naming findings, Unicode discrepancies, unresolved roles or missing CSS targets. The existing styling Nx quick suite passes all 30 tests (129 assertions).

The one introduction mouse SVG was also individually renamed to `🖱️mouse.svg`, preserving its bytes. Targeted authored-reader searches found no incoming literal path to its former generic name.

## Badge Artwork

All 37 badge files were individually moved into 20 handpicked subject groups: Windows `🪟️`, macOS `🍎️`, browser `🌐️`, Grasshopper `🦗️`, JavaScript development `🟨️`, TypeScript `🟦️`, C# `🔷️`, Python `🐍️`, release `📦️`, research `🔬️`, semio repository `🧬️`, play site `🎮️`, documentation `📖️`, license `⚖️`, citation `🔖️`, the two conference presentations `🎤️`/`🎙️`, community `👥️`, online chat `💬️`, and Discord `👾️`. Within each subject, the original Shields source is `🛡️source.shields` and its artwork is `🏷️badge.svg`; missing counterparts were not fabricated or downloaded.

The direct audit covers 57 entries, with zero statute findings, independent Unicode discrepancies or unresolved directory roles. The complete 37-file SHA-256 multiset is unchanged, including the distinct community and chat source files whose contents happen to match. The live root README's badge paths were updated together with the logo paths below.

## Logo Artwork

All 78 logo files were moved individually into handpicked families: emblem `🛡️`, favicon `🌐️`, logo `🪧️`, animation `🎞️`, semio `🧬️`, kit `🧰️`, elements `🧩️`, code icon `🧑️‍💻️`, and QR code `🔳️`. The actual light/dark/round variants have separate semantic owners; vector, raster, editable source, icon and explicit-resolution variants have distinct sibling identities. The six animation keyframes use the single-grapheme keys `1️⃣` through `6️⃣`, preserving their original order. No artwork, model or movie bytes were changed: the complete 78-file SHA-256 multiset is unchanged.

The 109-entry physical audit has zero statute findings, independent Unicode discrepancies or unresolved semantic roles. The Unicode oracle folds optional presentation selectors before recognizing joined emoji; this avoids treating the single developer grapheme as two symbols.

The literal root `README.md` retains its name. Its 20 logo/badge HTML source references now point to current files, including the previously broken Discord assets-owner spelling. An independent JSDOM parse resolves all 20 targets. This does not alter frozen imported README evidence.

The asset logo generator previously looked for nonexistent bare `logo_1.svg` through `logo_6.svg`. It now uses six explicit current paths and fails if any required keyframe is absent, instead of silently generating an incomplete animation. Animation output and MP4 export paths follow the new animation owner. Four exact styling favicon source/test references and the MP4 fixture's provenance link were updated; historical path fixtures remain intact.

A schema-first neutral regression failed on the absent keyframe resolver, then passed through Bun and independent TypeScript compilation. The entire focused naming suite passes 27 tests and 631 assertions. The actual logo parser and renderer were executed against all six current SVGs into ticket-generated output: each input has 23 groups, and the output contains the same 23 groups and 23 transform animations. Existing generated artwork was not overwritten. The styling Nx quick run reached 29 passes and one `src.split` font failure during the separate font catalog cutover; its final rerun remains assigned to that agent. MP4 export was not run.

## Metabolism Icons

Seventy-three SVG/CAD source files were moved individually into eight handpicked families: capsules `💊️`, ellipsoids `🥚️`, trapezoids `📐️`, balconies `🏞️`, bases `🧱️`, capitals `🏛️`, tambours `🥁️`, and cylindrical tambours `🔋️`. Letter/shape variants have distinct hook-J, angle-L, letter-P, magnifier-Q, snake-S, lightning-Z, slash and backslash identities. Storey variants distinguish the standard body, first, last and single-storey forms. Vector and CAD source files are siblings with different identities; public icon IDs, including uppercase J/L, remain unchanged.

The complete 73-source-file SHA-256 multiset is unchanged. The final 192-entry icon audit includes generated copies and has zero naming findings, independent Unicode discrepancies or unresolved semantic roles. The two Metabolism preview images were also renamed individually to `🌱️metabolism.png` and `🧱️base.png`; their bytes remain intact.

The old 29-file generated Metabolism mirror was moved intact to this ticket's `🌿️prior-metabolism-svgs` before rebuilding. The generator preview reported no stale removals, and the canonical build succeeded. Current SVG source paths, including semantic groups and case-sensitive IDs, are carried into the Rust mirror instead of producing generic flattened names. Source path regression tests first failed for uppercase identity rejection and flattened Metabolism output, then passed; the full focused suite now passes 27 tests and 674 assertions.

Puzzle's real native builder was compiled and executed against the new tree. All 29 ID/path pairs and raw SVG copies match; its generated `🧩️metabolism.rs` also compiles standalone. Infinite's builder now reads both grouped catalogs instead of silently skipping the old flat paths. It was compiled in a ticket-only diagnostic Cargo manifest and executed: all 249 catalog and 29 Metabolism paths and SVG bytes match, and generated `🔎️shortcodes.rs` compiles standalone. Initial attempts to reuse unrelated workspace cache libraries failed due to metadata-stub/mismatched-cache dependencies; those were diagnostic cache issues, not suppressed source failures. No whole Infinite or Puzzle application-test pass is claimed.

## Metabolism CAD Models

All 156 representation CAD files were moved individually into the same eight shape families plus `🌉️bridge`. Each actual shape distinguishes its authoring model `📐️`, collision model `🛡️`, 1:200 detail `🔬️`, and 1:500 detail `🔭️`; missing source variants were not fabricated. The 209-entry CAD/directory audit has zero naming, Unicode-oracle or semantic-role findings. All 246 original representation file hashes still match, including the 90 GLBs deliberately left at their exact original paths while the live URL publication cutover is reviewed separately. Those remaining GLB siblings are not claimed clean.
