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
