# WGPU Shell Layout Metric Audit

## Scope and Evidence

This bounded source audit checked the current logical viewport ingress, default chrome sizing, dock canvas inset and customizable theme metric authority. The root agent owns physical React/WGPU comparison after the fresh Wasm build. No new runtime conclusion is asserted here.

The default token values agree: canonical compact spacing is `0.2rem` at a 16 px root (`3.2 px`); navbar/footer are nine spacing units (`28.8 px`), controls seven (`22.4 px`), and padding/inset/gap one (`3.2 px`). React `Navbar` and `Footer` use `h-large`; CSS defines `--size-large` as nine `--ui-spacing` units. WGPU `Theme` metrics read the same canonical multipliers. React Mode body uses `MODE_CANVAS_INSET_CLASS = p-single`; WGPU applies its theme spacing inset before planning the dock.

Current host metrics pass logical sizes to shell geometry and physical sizes/density only to the GPU/atlas. Existing `📐️wgpu-dpi-logical-units` laws exercise the production shell/dock geometry at 1, 1.5, 2 and 3 density. The neutral `📐️dock-axis-geometry` fixture already serves both implementations. Those source paths provide no new confirmed default-density mismatch.

## Confirmed Custom Theme Divergence

React `🖱️ui/🎨️styling/🌓️theme/🟦️.ts:676` applies each authored `theme.spacing` value to CSS. Compact CSS derives `--ui-spacing` from `--spacing-compact` (`🖌️ui/🎨️.css:755`). In contrast, `Shell/🎯️targets/🧊️wgpu/🦀️.rs:32521` resolves all live chrome/DOM dimensions from `metrics.chrome.uiSpacingCompactPx`, ignoring `ThemeDocument.spacing.compact`. A custom compact spacing of `0.25rem` therefore makes React spacing four logical px, navbar 36 px and controls 28 px, while WGPU remains at 3.2/28.8/22.4 px unless the separate numeric metric is also edited. The theme editor already admits `setThemeSpacing("compact", "5px")`, and its Rust test asserts persistence only, not resulting geometry.

A second direction diverges: editing `metrics.chrome.navbarHeightUiSpacing` changes WGPU navbar geometry; React `applyUiThemeToRoot` currently applies only chrome glass/shade metrics, so Navbar's `h-large` continues to use nine spacing units. Its analogous footer/control/gap/inset multipliers and DOM panel limits have the same split authority. Current Rust theme tests intentionally assert a navbar metric edit (20 units) changes rendering, but there is no matching React CSS projection for that edit.

## Required Shared Authority

The clean fix is one schema-owned resolved geometry projection consumed by CSS and WGPU: compact spacing is the authored length resolved at the explicit root-rem metric, while navbar/footer/control/inset/gap and DOM layout limits each use their named authored multiplier. The numeric compact reference metric must be a derived reference, not a second editable live authority. Both theme implementations should consume the same neutral custom-spacing and custom-multiplier cases. The shared DOM projection must override the corresponding named CSS size variables; fixed utility class aliases cannot represent independent edited navbar/footer/control metrics.

This audit was handed to the root and windows lanes before any metric source mutation. A bounded report was selected under the parent task's “implement/test a confirmed mismatch or produce a bounded audit report” option; no speculative geometry patch has been added while the fresh full source gates are underway.


## Shared Custom Theme Geometry Implementation

The shared geometry bindings now project 18 named chrome/DOM/typography fields to both native Theme scalars and React CSS variables. Authored `spacing.compact` plus `metrics.dom.rootRemPx` is the compact-length authority. The duplicate editable `metrics.chrome.uiSpacingCompactPx` was removed from canonical styling and theme documents; generated Rust `UI_SPACING_COMPACT_PX` remains a derived reference. The Shell fixture now expresses its original four-pixel compact spacing as `0.25rem` at root 16.

The first-party TypeScript math twin lives beside the JSON contract in `styling/🌓️theme/📐️geometry/🟦️.ts`. The Rust twin is owned by the windows lane in the same directory. The bounded contract accepts CSS numbers (leading dots, optional signs, exponents, ASCII whitespace, case-insensitive units), px/rem and absolute in/cm/mm/q/pt/pc units, and calc/min/max/clamp with dimensional arithmetic. Relative rem resolves against the authored root. Addition/subtraction require matching dimensions and CSS whitespace; multiplication/division obey scalar/length dimensions. Unknown variables and context-dependent units, invalid dimension arithmetic, division by zero, excessive nesting/tokens/arguments, and negative or unrepresentable final geometry are refused. The final pixel bound is f32 representability, shared by both targets. Other authored spacing entries retain their existing CSS string scope.

React Navbar/Footer/panel tabs now consume the named height variables; independently edited control sizes, standard gap/padding, tree pitch/indent/toggle width, layout width limits, and typography project through generated palette CSS and root-specific runtime overrides. Dock body inset uses `panel_inset`; navbar/general Canvas control padding uses `padding_standard`. Chrome-hosted panel offsets use each actual navbar/footer height and the named panel header height. Invalid staged spacing/metric edits preserve the retained theme. Compact authoring guidance and invalid-input feedback are localized English/German, with `aria-invalid` and an associated description.

Neutral fixture `styling/🧫️fixtures/📐️theme-geometry/🔣️.json` has 15 positive geometry cases, invalid arithmetic/grammar/resource bounds, and six invalid metric cases. The independent test oracle uses CSSTools CSS tokenizer to resolve rem dimensions at the fixture root, then CSSTools CSS calc arithmetic and canonical absolute-unit conversion. Ajv validates the input schema with the independent semantic-resolution keyword. Both libraries are development-only dependencies; runtime parsing is first-party.

## Validation Witnesses

- `bun nx run @semio-tech/ui-styling:test` initially executed 68 passes and three stale source guards (native cursor inventory 25 vs current 27, pre-palette emphasis selector, pre-silhouette tab divider). The guards were aligned to the actual current semantics without relaxing metric or media limits. Fresh full run `theme-styling-full-green.log` passed 71/71 with 2,634 assertions.
- The geometry focus passed the 15 vectors, independent schema/CSS oracle, invalid staged edits, and core shell metric bindings. The later representability bound adds two refusals and awaits the final rerun.
- The new mounted-root test applies every neutral geometry vector to an isolated DOM root, checks computed CSS variables, and verifies another mounted root retains its geometry. The default UI React gate exceeded its 15-second startup budget before assertions; the registered long gate is running.
- Windows owns the native fail-first geometry/editor laws and the Rust projection repair. Root owns fresh native/WASM and physical browser validation. These witnesses remain pending; the packet is not a claim of whole-renderer parity.

## Final Custom Geometry Source Witness

The current uncached styling geometry acceptance route passed three neutral/oracle laws (941 assertions) and two mounted DOM laws. The already running full styling suite then passed all 71 laws (2646 assertions). These executions include the final first-party bounded CSS length/math resolver, the independent CSSTools calculator oracle, mapped f32 representability validation, and localized metric editor descriptions. Logs: `🗑️generated/sol-2026-09-29/theme-geometry-acceptance-current-2.log` and `theme-styling-full-current.log`.

The first retry failed before any test because Nx inferred the new package script as its own target and recursively invoked `test-geometry`. The root integration lane repaired inference with the repository's existing `nx.includedScripts: []` convention. The effective target again runs `bun ./📜️script.ts test-geometry`; no test contract changed.

## CSS Token Refusal Refinement

A final bounded-parser review found two concrete token mismatches: TypeScript accepted a function name separated from `(` and accepted a trailing-dot dimension (`1.px`). Five shared invalid vectors now cover spaced `calc`/`min`/`clamp` and literal/nested trailing-dot dimensions. The existing Rust numeric scanner already rejects trailing dots; its function adjacency gap was coordinated with the Rust lane. The first focused TypeScript run failed two laws with `calc (1px + 2px)` resolving to 3 px and being admitted by the staged editor. The shared contract now explicitly requires function-name adjacency and a digit after a decimal point; the TypeScript twin enforces both.

The resulting uncached focused route passed all three neutral/oracle laws (956 assertions) and both mounted DOM laws. Logs: `🗑️generated/sol-2026-09-29/theme-function-adjacency-red.log` and `theme-function-adjacency-green.log`. Valid leading dots, signed/exponent numbers, authored rem roots, absolute units, nested arithmetic, and calc/min/max/clamp capabilities are retained.
