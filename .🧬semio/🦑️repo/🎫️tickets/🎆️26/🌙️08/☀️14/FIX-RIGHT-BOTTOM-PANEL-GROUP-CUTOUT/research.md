# Research & Fix Plan: Right Bottom Panel Group Cutout Bug

## Root Cause Analysis
1. In `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️component.tsx`, `chromeHostedTrailingEndReserveStyle` used `horizontal === "right"` to determine whether to apply the top-right navbar trailing end reserve style to chrome-hosted panels. Since `anchorHorizontal("bottom-right")` evaluates to `"right"`, the `bottom-right` panel group wrongly received the top-right cutout reserve style.
2. In `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`, `shellNavbarTrailingEndReserveStyle(widthPx)` returned a fallback cutout (`paddingInlineStart: shellNavbarTrailingEndReserveCss`) when `widthPx <= 0`. As a result, even when there was no UI element (such as fullscreen toggle) present in the top-right navbar, a cutout was still applied.

## Solution Plan
1. Update `shellNavbarTrailingEndReserveStyle` in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` to return `undefined` when `widthPx <= 0`, so no cutout reserve style is generated when no top-right element is measured.
2. Update `chromeHostedTrailingEndReserveStyle` in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️component.tsx` to check `anchor === "top-right"` instead of `horizontal === "right"`, ensuring only the `top-right` anchor can receive the navbar trailing reserve cutout.
3. Add unit tests in `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` covering `bottom-right` and `top-right` with zero width.
