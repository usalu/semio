# 🔎 Audit 2: Print, Docs, Tokens and Fixtures After the Frame Model

Read-only audit. No source file was edited, no git write was run, no build was started.
Model under audit (from `design.md`): one axis-aligned frame, content edge to edge, chips inside the frame
on top of the content, chip glass one level above the content and tinted (`--chip-tint`), no gaps, no cutouts.

Reference values (from `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`, `levels`): `glassAlphaStep` 0.12,
`glassBlurStepPx` 8, `glassSaturate` 1.45, `chipTintPercent` 10. The web and wgpu chip formula
(`🎭️theme/🧩️component/🦀️.rs` `chip_glass`) gives chip alpha `1 - (1 - (1 - k * 0.12)) * 0.9 = 1 - 0.108 k`
where `k` is the index of the level one above the content. So the derivable chip alphas are 0.892 (k=1) and
0.784 (k=2). No value below is derived from this.

Abbreviations: `🧰` = `🧰️framework`, `🖱️ui` = `🧰️framework/🔨️modules/🖱️ui`, `PRINT` = `🧰️framework/🛍️products/📓️print`.

## Answers to the four questions

- **(a) LaTeX.** No chips outside frames, no title tabs above boxes, no `tcolorbox attach`, no shadows, no rounded
  corners remain in `PRINT/🖋️latex` (`semio-window.sty` uses `arc=0mm`, `boxrule=\semio@stroke@hairline`). The
  `mit-bestand`, `teaching`, `leutwiler` and `semio-tech` trees have no chrome of their own; they inherit
  `semio-window`, `semio-table`, `zukunftbau.cls` and `assignment.cls`, so they need no separate fix. Chips on
  figures and photos are translucent in print (`semio-window.sty:2315-2318`, `\pgfsetfillopacity`). But the opacity
  is a hand-typed `0.82` and there is no `--chip-tint` veil, so print does not match the web chip.
- **(b) Other outputs.** No Typst files exist in the repo. No PPTX or DOCX theme exists (only `.docx` fixtures for
  document operations). SVG brand assets carry no chrome identifiers. README (EN and DE, `PRINT/README.md:56-66`
  and `:307-318`) already describes the new model. No `SKILL.md`, `AGENTS.md`, `CLAUDE.md`, MDX or storybook doc
  describes the old silhouette model. The only old-model wording outside tickets is the user prompt log
  (`.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:3066`), which must not be edited, and the pets story CSS (see table).
- **(c) Tokens.** `chipTintPercent` exists in the JSON source, the generated TypeScript and the Rust tokens. It is
  missing from LaTeX, Python and C#, and it is hand-copied in CSS. Details in the table.
- **(d) Fixtures and snapshots.** No snapshot, `.snap`, golden PNG or fixture pins `window-silhouette`, `clip-path`
  or the removed gap markup. The only hits are negative "must not contain" guards. Leftover contract fixtures
  (`junctions.*`, `snap.*`) do remain and conflict with the design text. Details in the table.

## Findings, sorted by severity

Severity: **H** = contradicts the model or drifts from the single source of truth; **M** = missing target or
hand-copied value; **L** = stale wording, dead token or test guard.

| # | Sev | Lane | File:line | Verdict | Recommended fix |
|---|-----|------|-----------|---------|-----------------|
| 1 | H | P (print) | `PRINT/🖋️latex/semio-window.sty:2311` `\newcommand{\semio@chip@opacity}{0.82}` and `:2315-2319` `\semio@chip@fill` | Hand-typed opacity, no chip-tint veil, not derived from `glassAlphaStep`/`chipTintPercent`. 0.82 matches no derivable value (0.892 or 0.784). The print chip is the window surface at 0.82 only, so the glass is not "tinted differently from the body" the way the web chip is. | Define `\semio@chip@opacity` and a chip veil from the token generator (`\semio@chip@tint`, foreground at `chipTintPercent` over the chip fill). Keep `\SemioChipTransparency{false}`. Update `PRINT/README.md:61`. |
| 2 | H | D (tokens: CSS) | `🖱️ui/🎨️styling/🖌️ui/🎨️.css:1026-1032` `--glass-alpha-step: 0.12`, `--glass-blur-step: 0.5rem`, `--chip-tint: 10%` | Hand-copied from `🔣️.json` `levels` (lines 472-478). The comment at 1017-1019 only asks for "lockstep". No CSS adapter emits these values. Values agree today, so this is drift risk, not a current bug. | Emit a `levels` knob block from `📽️projection/🟦️.ts` into `🤖️generated/` and `@import` it. Remove the hand copy. |
| 3 | M | D (tokens: theme) | `🖱️ui/🎨️styling/🌓️theme/🟦️.ts:167-172` | Theme overrides apply `glassSaturate`, `glassAlphaStep`, `glassBlurStepPx` as CSS vars but not `chipTintPercent`, so a themed chip tint cannot be set. That breaks the "customizable UIs" rule for the new chip tint. | Add `--chip-tint` from `chrome.chipTintPercent` to the same setter block, and add it to the theme schema. |
| 4 | M | D (tokens: LaTeX) | `PRINT/🖋️latex/semio-tokens.sty` (whole file, 143 lines); generator `PRINT/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts:44-47` (type `PrintStylingLevels` has no chip field) and `:65-71` (`DEFAULT_LEVELS` copy), `:209` (silent merge) | No glass, chip or tint token in the LaTeX target. Levels are read from JSON only when present. `DEFAULT_LEVELS` is a silent fallback copy. The file header at line 1 names a stale path (`framework/ui/styling/🔣️.json`; the real path is `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`). | Add `chipTintPercent` (and `veilAlpha`, `veilBlurPx`) to the type and emit `\semio@chip@tint`, `\semio@glass@alpha@step`. Delete `DEFAULT_LEVELS` or make it throw when a key is missing. Fix the header path. |
| 5 | M | D (tokens: Python) | `🖱️ui/🎨️styling/📽️projection/🟦️.ts:554-600` (`emitPython`) and output `🖱️ui/🎨️styling/🔤️tokens/🐍️.py` (504 lines) | No levels, glass or chip constants at all. Consumers therefore hard-code them (see row 7). | Add a `LEVELS`/`GLASS`/`CHIP_TINT_PERCENT` section to `emitPython`, mirroring `emitRust` at `projection/🟦️.ts:502-508`. |
| 6 | M | D (tokens: C#) | `🖱️ui/🎨️styling/🎨️palette/🔷️.cs` (186 lines), `emitCSharp` at `projection/🟦️.ts:368` | No levels, glass or chip constants. Acceptable only while no C# renderer draws chrome. | Add the same section as row 5, or record in `🛂️adapters.manifest.json` that the C# target carries palette only. |
| 7 | M | Python / manim | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py:39-40` (`CHROME_CHIP_FILL = color("dark-6-7")`, `CHROME_CHIP_OPACITY = 0.76`), `:126` (`fill_opacity: float = 0.88`), `:81-121` (`window_frame_outline`, `window_frame_chip_regions`) | Hand-picked glass values, not derived. 0.76 and 0.88 match no derivable web or wgpu value. The geometry is a third port of `WindowFrame` (the file says it "matches" `frame.cases.json`, which is good) but the chip alpha is not shared. | Read the chip alpha from the Python token module (row 5) and use `1 - (1 - glass.alpha) * (1 - chipTint)`. Keep the geometry twin, since `frame.cases.json` already checks it. |
| 8 | M | Python / manim tests | `🎓️teaching/.../🎨️visuals/🧪️tests/🐍️reference_chip.py:42-44` | Asserts fill and stroke opacity against the same module constants it tests, so it cannot catch drift. | Assert against the token value (row 5), not the module constant. |
| 9 | M | Pets story CSS | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css:426-438` (`.card .tab`) | Chip glass is hand-typed: `color-mix(... 10% ...)` (should be `--chip-tint`) and `backdrop-filter: blur(8px)` (should be the glass utility, `--surface-blur`). The chip has `border-radius: 0 9px 0 0`, which breaks the sharp-rectangle rule. Its fallbacks at `:441-457` are right. | Replace with `[data-glass-chip]` (or the `ui-glass` utility) and `var(--chip-tint)`. Drop the radius. |
| 10 | M | Pets story CSS | same file, `.card .note` (around line 457) | A tab above the card (`bottom: 100%`) with rounded top corners. This is the old "tab above box" pattern. | Move it inside the card as a chip, sharp corners, or remove the story element. |
| 11 | M | Contract fixtures | `🖱️ui/🧱️elements/🔲️WindowFrame/junctions.cases.json`, `junctions.schema.json`, `snap.cases.json`, `snap.schema.json`, `snap.test.ts`; `shared-border.ts:85-137` (`createSharedBorderJunctionResolver`, `sharedBorderJunction`, `createSharedBorderSnap`) | The design says "junction cases" and "full-span snapping" are removed without replacement, yet the frozen API keeps "junction resolvers" and the fixtures still run. This is a contradiction, not yet a bug. | Decide once. If junctions and snap stay, say so in `design.md` and keep the fixtures. If not, delete the fixtures, the resolvers and their tests in one change. |
| 12 | M | Lane conflict | `🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts` (61,537 bytes) and `🖱️ui/🧱️elements/🔲️WindowFrame/shared-border.ts` (12,540 bytes), plus two `shared-border.test.ts` | Two files share one name and one responsibility. The design says `WindowFrame/shared-border.ts` keeps the export names of the former silhouette file and the compositor lives in `WindowChrome`. Parallel edits will collide. | Move the compositor into `WindowChrome` only, keep geometry in `WindowFrame`, and rename by role (for example `outline-compositor.ts`). |
| 13 | L | React doc comments | `🖱️ui/🔨️modules/🛟️chrome-safe-area/🟦️.tsx:40-43` ("a flex row of `[tab strip \| gap \| controls]`") | Describes the removed gap element. | Rewrite to "chip row over content". |
| 14 | L | React doc comments | `🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx:220,258,260,372`; `🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx:28,73` | "cap row" wording survives. Slots are still `window-chrome-cap`, so this is terminology only. | Rename the prose to "chip row" with the next slot rename. |
| 15 | L | Dead tokens | `🖱️ui/🎨️styling/🌓️theme/🔣️.json:134-136` (`glassPanelAlpha`, `glassMenuAlpha`, `glassWindowOptionsAlpha`), `:287-288` (`glassPanelBlurPx`, `glassWindowOptionsBlurPx`) | Old four-tier glass knobs. No consumer found in TS, Rust, CSS or Python. Only the print type mentions `opacities.glassPanelAlpha`. | Delete them from the JSON and from the print type. |
| 16 | L | Rust comment | `🖱️ui/🎨️styling/🎭️theme/🧩️component/🦀️.rs:53` | Mentions "panel/ribbon/menu/windowOptions" glass tiers that the 6-level model replaced. | Reword to the 6-level model. |
| 17 | L | Negative guards | `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🌳️owned-locale-detector-retirement-and-more/🟦️.tsx:415-417`; `.../🧊️shell-components-3/🟦️.tsx:303`; `🖱️ui/🎨️styling/🧪️tests/🧩️suite/🎨️styling-resolve/🟦️.ts:72,80`; `🖱️ui/🧱️elements/🗂️WindowChrome/frame-structure.test.tsx:47`; `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/📐️shell-components-4/🟦️.tsx` (`mode-dock-tab-cap`, `mode-dock-tabbar`) | These pin removed names as "must not appear". They are retirement guards, so keep them. The `mode-dock-tab*` slot assertions are current behaviour. | No change. Once the old names are gone for good, the guards can be trimmed. |
| 18 | L | Docs / prompts | `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:3066` | The user's own prompt says "rectangle with chips around it" (the old model). It is a log of intent. | Leave it. Do not edit prompts. |
| 19 | L | Pets story CSS | `.../📖️stories/🎨️.css` `.prop` (`border-radius: 6px`, line 337) and `.shelf` (`10px`, line 474) | Rounded corners on non-chip story frames. Not chrome, but it breaks the rectangle rule for story-built frames. | Set `border-radius: 0` if these stories are meant to show the frame model. |

Not a finding (checked and clean):

- `PRINT/🖋️latex/semio-window.sty`: no `semio@window@cap*`, `semio@tab`, `semio@heading@cap*`, `titlebar@height` or
  silhouette macros remain (grep of tracked and untracked files, outside tickets). `\semio@window@gap` is
  `-\semio@stroke@hairline` (abutting frames, correct).
- `PRINT/🖋️latex/semio-table.sty:504`: the chip raise stays inside the band (`ht+dp-hairline-chip@h`).
- `zukunftbau.cls`, `assignment.cls`, `semio.cls`, `semio-components.sty`, `semio-thesis.sty`: no chrome of their own.
- `semio-viz-*.sty`: `silhouette`, `rounded corners` and `opacity` hits are data marks (chart kinds, map layers),
  not chrome. Left alone, as the print handoff notes.
- `clip-path` uses left in UI code are the checkbox tick (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:7454`) and the
  LayeredOverview veil (`🖱️ui/🔨️modules/🥞️layered-overview-geometry`). Neither is a chrome silhouette. The wgpu
  stencil hits are the 3D cap pass, not silhouette masks.
- `🖱️ui/🧱️elements/🔲️WindowFrame/🦀️.rs` and `🟦️.tsx`: `frame.schema.json` and `frame.cases.json` contain no
  `outline`, `gap`, `junction` or `silhouette` keys.
- TUI (`🖱️ui/⌨️tui`): no glass by design; no chip-tint consumer needed.
- Old-name grep of tracked and untracked files (outside tickets, `.cache`, `node_modules`): zero matches for
  `WindowSilhouette`, `window-silhouette`, `silhouette-geometry`, `SilhouetteMask`, `silhouetteSlot`,
  `gapProps`, `footerGapProps`, `window-chrome-gap`, `mode-dock-tab-gap`, `boxSilhouetteSegments`,
  `begin_silhouette_clip`, `contentClipPath`, `semio@window@cap`, `semio@tab`, `titlebar@height`.

## Lanes (no file is in two lanes)

- **Lane P, print LaTeX:** rows 1, 4 and the README line `PRINT/README.md:61`. Files: `PRINT/🖋️latex/semio-window.sty`,
  `PRINT/🖋️latex/semio-tokens.sty`, `PRINT/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`, `PRINT/README.md`.
- **Lane T, token pipeline:** rows 2, 3, 5, 6, 15, 16. Files: `🖱️ui/🎨️styling/📽️projection/🟦️.ts`,
  `🖱️ui/🎨️styling/🖌️ui/🎨️.css` (generated import), `🖱️ui/🎨️styling/🌓️theme/🟦️.ts`, `🖱️ui/🎨️styling/🌓️theme/🔣️.json`,
  `🖱️ui/🎨️styling/🎭️theme/🧩️component/🦀️.rs` (comment only), `🖱️ui/🎨️styling/🔤️tokens/🐍️.py` and `🎨️palette/🔷️.cs` (generated).
- **Lane M, manim:** rows 7 and 8. Files: `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py` and its `🧪️tests/🐍️reference_chip.py`.
- **Lane S, pets story:** rows 9, 10, 19. File: `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css`.
- **Lane F, WindowFrame contract and shared border:** rows 11 and 12. Files: `🖱️ui/🧱️elements/🔲️WindowFrame/*` and
  `🖱️ui/🧱️elements/🗂️WindowChrome/shared-border*`. Decide the junction and snap question first (design owner).
- **Lane R, React prose:** rows 13, 14. Files: `🖱️ui/🔨️modules/🛟️chrome-safe-area/🟦️.tsx`, `🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx`,
  `🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx`. Comments only, no behaviour change.
- **Lane G, guards:** row 17. Trim only after lanes F and R have merged.
- **No action:** rows 18 (prompt log). `AGENTS.md` files were not touched and no chrome wording was found in them.

## Gaps this audit could not close

- Print: the handoff's open items still stand (full `repo test` quick run and `test-assignment` not run through
  `repo`, gallery and `viz-api` not recompiled). This audit did not recompile anything.
- The untracked-file grep with `--untracked` takes about four minutes on this repo. The stored result above is from
  that run. Doc wording was checked with a tracked grep plus a directory-limited recursive grep, not a full
  untracked sweep of every `.md` file.
- SVG brand assets were checked by identifier grep, not by rendering.
