# Verify Log — Print Heading Color Scheme (revision 2)

## Changes

- Body/window/panel text now uses `semio-chrome-text-normal` (UI gray) instead of emphasized foreground.
- Heading box padding uses `\semio@spacing@single` (one spacing unit).
- Swapped Part ↔ Section tier colors: Section → primary, Part → tertiary.

## Build

- `bun ./script.ts build report paper` — OK (light + dark)

## Visual check

### Report page 3

| Level | Fill | Body text |
|---|---|---|
| Chapter | secondary teal | gray (normal) |
| Section `1.1 Background` | **primary red** | gray (normal) |
| Subsection | secondary teal | gray (normal) |
| Subsubsection | gray | gray (normal) |
| Paragraph | canvas + gray border | gray (normal) |

### Paper page 2

- Sections use **primary red** fill with emphasized heading text; body paragraphs gray.

### Dark theme

- Heading tiers unchanged; body text muted gray, not bright foreground.
