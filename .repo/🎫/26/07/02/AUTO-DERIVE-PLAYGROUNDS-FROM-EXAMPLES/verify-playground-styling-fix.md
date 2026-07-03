# Playground styling fix

## Root cause

`framework/product/playground/dev/globals.css` imported Tailwind theme tokens but lacked Tailwind v4 `@source` directives. Without them, utility classes used in `ui/react`, playground renderer, and app react packages were never scanned — only `@theme` variables applied (cream background) while `.flex`, `.size-workbench`, etc. were missing.

## Fix

Aligned `globals.css` with `cad/renderer/react/globals.css` and `.storybook/globals.css`:

```css
@import "../../../../ui/react/globals.css";
@source "../core";
@source "../renderer/react";
@source "../../platform/renderer/react";
@source "../../../../ui/react";
@source "../../../../**/react";
```

## Verified (dev port 6185)

| Check | Before | After |
|-------|--------|-------|
| `.flex` rule | false | true |
| `.size-workbench` rule | false | true |
| Navbar logo width | 1280px | 24px |
| `#root > div` display | block | flex |

## Verified (preview E2E)

- `lowpoly`: build + boot ok, nav + 40 buttons
