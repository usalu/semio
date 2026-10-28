# Semio Visual Style Audit

Canonical tokens: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`.

- Base colors: light #f7f3e3; dark #001117; primary #ff344f; secondary #34d1bf; tertiary #fa9500; gray #7b827d.
- Typography: Anta sans; Kelly Slab serif; Share Tech Mono mono, with Noto Emoji fallbacks.
- Compact spacing .2rem, touch .275rem. Chrome radius 0; border widths hairline/default/focus 1/2/3px.
- UI scale: 9.6, 11.2, 12.8, 14.4, 16, 21.6px type sizes. Scale proportionally for video.

## Silhouettes and Chips

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/🟦️.tsx` defines axis-aligned stepped outlines, top/bottom chip intervals, body/content/glass regions, SVG paths and CSS clip polygons.

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx` lines 392 onward specifies left title chips, open U-gap, optional controls and a continuous body outline. Chip rows are transparent except painted chip cells. Title chips are borderless; the silhouette is the sole border. Normal uses border-normal-color, active uses active-base, introduced uses secondary.

Print corroboration: `🧰️framework/🛍️products/📓️print/🖋️latex/semio-tokens.sty` uses the same palette, .2em/.4em spacing, .75/1.5/2.25pt strokes, 7.2pt chip and 9.6pt body fonts.

## Font Assets and Native Registration

The font catalog is source-first and contains TTF, WOFF and WOFF2 identities: `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/📇️catalog.json`. The existing TypeScript interface `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️fonts/🟦️.ts` exposes FontCatalog, fontCatalogSources and resolveFontSource. Prefer deriving exact TTF asset identities from this catalog over WOFF2 conversion or basename fallback.

Vendored TrueType sources are present and have valid 00010000 SFNT signatures:

- Anta: `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf` (26772 bytes), plus latin-ext, symbols and math subsets.
- Share Tech Mono: `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf` (19540 bytes).
- Kelly Slab: `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/🔤️outline.ttf` (27608 bytes), plus latin-ext and Cyrillic.
- Noto Emoji includes complete regular TTF (888960 bytes) and other subsets/weights.

Recommended video implementation: retain a repo-owned font-registration interface and register vendored TTF paths through the existing Manim/Pango registration context for the render lifetime. This avoids fontTools conversion, network access, system installation and new runtime dependencies. Validate registration against the installed Manim/Pango implementation; no existing Python register_font/manimpango/fontTools usage was found in the searched framework and shooting plugin trees.

## Applicable Instructions

UI AGENTS.md is metadata only: pure consistent business-logic-free UI components. Assets AGENTS.md instructs shipping icons/logos/lists/builtin kits and keeping test JSON outside assets. No instructions were modified.

## Scope

Read-only audit; no source code changes. Broad filename/content discovery did not identify an existing energy clip before the owner task located its target. The owner should reuse canonical tokens and TTF assets in the target clip implementation.
