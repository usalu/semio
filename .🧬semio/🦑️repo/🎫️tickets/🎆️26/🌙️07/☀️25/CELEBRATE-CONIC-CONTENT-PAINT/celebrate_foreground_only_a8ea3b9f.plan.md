---
name: Celebrate Foreground Only
overview: "Replace the broken `mix-blend-mode: destination-in` celebrate icon recipe with a real alpha mask driven by a new `--icon-mask` data URI emitted by `Icon`, so the spinning conic paints icon/grip ink instead of a rectangular fill behind it."
todos:
  - id: icon-mask
    content: Add memoized iconMaskImage() to the 🖼️IconCodec region and stamp --icon-mask on Icon's SVG-backed wrapper
    status: completed
  - id: celebrate-css
    content: "Replace the ::before + mix-blend-mode recipe in CelebrateContent with mask-image: var(--icon-mask, transparent) + hidden svg; add themed kind; drop drag-handle color:#000 overrides"
    status: completed
  - id: glyph-kinds
    content: "Give maskless glyph icon kinds (text/typst/shortcode/missing) the background-clip: text conic"
    status: completed
  - id: unit-tests
    content: Update the celebrate CSS contract test and Icon markup tests in 📦️index.tsx
    status: completed
  - id: browser-test
    content: Add a real-browser celebrate paint assertion to .storybook/ui-new-stories.spec.ts
    status: completed
  - id: verify
    content: Run ui-react + ui-styling tests, storybook playwright, and capture a screenshot proof into the ticket folder
    status: completed
  - id: ticket
    content: Reopen 2026/07/25/CELEBRATE-CONIC-CONTENT-PAINT, keep artifacts in the folder, close with summary and files
    status: completed
isProject: false
---

### Root cause

`🎨️ui.css` `CelebrateContent` paints the conic on an oversized `::before` (`inset: -100%`) behind every `[data-icon]` inside a celebrated control, then tries to knock it back out with `mix-blend-mode: destination-in` on the inner `<svg>`:

```7028:7053:🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript/🎨️ui.css
) :is([data-icon], [data-icon-kind="catalog"], [data-icon-kind="svg"])::before {
  content: "";
  position: absolute;
  inset: -100%;
  background-image: var(--celebrate-conic);
  z-index: 0;
}
/* ... */
) :is([data-icon], [data-icon-kind="catalog"], [data-icon-kind="svg"]) > svg {
  mix-blend-mode: destination-in;
}
```

`destination-in` is a Porter-Duff compositing operator (canvas `globalCompositeOperation`, SVG `feComposite`) and is not a valid `mix-blend-mode` value, so that declaration is dropped at parse time. What survives is the conic rectangle (bleeding a full box beyond the icon on every side) plus `color: #000` on the wrapper — exactly the screenshots. Drag handles are hit because `DragHandle` renders `GripVerticalIcon`, i.e. an `Icon`, so it carries `data-icon="grip-vertical"`.

The label (`background-clip: text`) and tree guide/elbow/stem strokes (conic as the 1px line's own background) already work. Only the icon recipe is broken.

### 1. `Icon` emits an alpha mask — [🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx](🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx)

In the `🖼️IconCodec` region add a memoized helper next to `iconSvgMarkup`:

```ts
const ICON_MASK_CACHE = new Map<string, string>();

/** @emoji 🩻️ Alpha-mask image for an icon's own resolved SVG — lets CSS paint gradients (e.g. the celebrate conic) through the glyph instead of behind it. `currentColor` is baked to opaque black because a mask image renders in its own context and only its alpha channel is read. */
export function iconMaskImage(svgMarkup: string): string { ... }
```

It replaces `currentColor` with `#000`, then returns `url("data:image/svg+xml,${encodeURIComponent(markup)}")`, caching by markup so all instances of one icon share a single string. All 246 vendored icons carry `xmlns` and a `viewBox`, so they load standalone.

`Icon`'s SVG-backed return (the `data-icon` / `data-icon-kind="catalog"|"themed"|"svg"` branch at ~1477) gains the property on its existing `style`:

```tsx
style={{ ...boxStyle, ["--icon-mask" as string]: iconMaskImage(svgMarkup) }}
```

Derived from the already theme-resolved `svgMarkup`, so `UiTheme.icons` variants mask correctly. Do **not** register `--icon-mask` with `@property`: it must stay unset on non-SVG kinds so the `var()` fallback in the CSS below can guarantee a transparent mask rather than `none`.

### 2. Rewrite the icon recipe — [🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript/🎨️ui.css](🧰️framework/🔨️modules/🖱️ui/🎨️styling/⚡️implementations/🟦️typescript/🎨️ui.css)

In `CelebrateContent`, delete the three-rule blend recipe (the `isolation: isolate; color: #000` block, the `::before`, and the `> svg { mix-blend-mode }`) and replace with mask-based ink on the wrapper, adding the currently missing `themed` kind:

```css
) :is([data-icon], [data-icon-kind="catalog"], [data-icon-kind="themed"], [data-icon-kind="svg"]) {
  background-image: var(--celebrate-conic);
  -webkit-mask-image: var(--icon-mask, linear-gradient(#0000 0 0));
  mask-image: var(--icon-mask, linear-gradient(#0000 0 0));
  -webkit-mask-size: 100% 100%;
  mask-size: 100% 100%;
  mask-repeat: no-repeat;
}
) :is(...) > svg { visibility: hidden; }
```

The fully transparent `var()` fallback is the structural guarantee the user asked for: if a mask is ever missing, nothing paints at all instead of a fill. `mask-size: 100% 100%` matches the inner svg's `size-full` stretch, and hover transforms on `[data-icon]` (`:where([data-icon], [data-icon-kind]):hover { animation: var(--icon-animation) }`) still work because the mask travels with the element box.

Glyph-based kinds that have no mask (`text`, `typst`, `shortcode`, `missing`) get the same `background-clip: text` treatment as labels; `emoji`, `image`, and `node` keep their own paint. Remove the now-pointless celebrated drag-handle `color: #000` overrides (the last ~35 lines of the region) since the grip's svg is hidden. Update the region docstring so it describes masking, not blending.

### 3. Tests

In-source, [📦️index.tsx](🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx):
- `"celebrate content paint shares --celebrate-conic..."` (~28148): drop the `mix-blend-mode: destination-in` and drag-handle `color: #000` assertions; assert the region has no `mix-blend-mode` and no `inset: -100%`, and does contain `mask-image: var(--icon-mask, linear-gradient(#0000 0 0))` plus `visibility: hidden` for `> svg`.
- `"Icon hover animation attributes"` (~30518): catalog markup contains `--icon-mask:url(&quot;data:image/svg+xml,`; `emoji` / `text` markup does not.

Real-browser regression in [.storybook/ui-new-stories.spec.ts](.storybook/ui-new-stories.spec.ts) — this bug class (a silently dropped CSS declaration) is invisible to string tests. Load an existing tab/tree story, stamp `data-celebrated="true"` via `page.evaluate`, then assert on `getComputedStyle` of a nested `[data-icon]` that `maskImage` starts with `url("data:image/svg+xml` (proving the browser accepted it) and that `::before` no longer yields a conic layer.

### 4. Verification

- `bun nx run @semio-tech/ui-react:test` and `bun nx run @semio-tech/ui-styling:test`.
- `bun run build:storybook` then `bun run test:storybook` for the Playwright check.
- Manual runtime proof stored in the ticket folder: a temporary Playwright script that stamps `data-celebrated`, logs the computed `mask-image` / `background-image` with a `[DEBUG]` prefix, and writes a cropped PNG of a celebrated tab and tree row for visual confirmation that no fill remains.

### Ticket

Reopen `2026/07/25/CELEBRATE-CONIC-CONTENT-PAINT` (it owns this exact recipe) via `ticket_reopen`, goal `R26-02/RUNNING-SKETCHPAD`; keep notes, script, and screenshots in its folder and close with a summary plus touched files.
