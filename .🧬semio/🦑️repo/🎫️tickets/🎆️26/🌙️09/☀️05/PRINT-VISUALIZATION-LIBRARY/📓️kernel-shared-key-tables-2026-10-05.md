# Kernel Shared Key Tables

Seven existing kernel API sections now bind to their actual owner scopes. Six existing tables retain their human bilingual titles and gain canonical table IDs; Text gains its six-key bilingual table. The prior comprehensive RED 64318 established the missing scoped rows before this edit.

Source: `🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex`. Before SHA-256: `AF11421FF426B6B2C476428B12F0203A933993834DE24A9F9E60440B2AFBB154`; after SHA-256: `CD4ACEDB9036C9E20D6B975539B708DBFF86A42BBBDCD3356937A78ABD8C47F6`. Authored source slices and manifest are retained in `📥️authored-inputs/kernel-shared-key-tables-before`.

| ID | Native Scope | Required Keys |
|---|---|---|
| api-path-scale | semio / viz / scale | 17 |
| api-path-transform | semio / viz / transform | 23 |
| api-path-mark | semio / viz / mark | 40 |
| api-path-axis | semio / viz / axis | 43 |
| api-path-legend | semio / viz / legend | 30 |
| api-path-annotation | semio / viz / annotation | 24 |
| api-path-text | semio / viz / text | 6 |

## Source Semantics

The edit adds the 11 omitted axis, 17 omitted legend, five omitted mark and one annotation controls. It removes the phantom legend anchor key and corrects the legend scale default to x, documents actual side placement and size area conversion, restores actual mark theme colour and elliptic/padding defaults, and documents text kind-specific anchors and fonts. Guide widths are millimetres, guide font sizes are points, mark angles are radians and axis rotations are degrees. Guide grid overrides are described conditionally because legend drawing itself has no grid path.

Authority was read directly from existing `semio-viz-guide.sty`, `semio-viz-mark.sty` and `semio-viz-annotation.sty` key declarations, resets and consumers. Table occurrence counts and the independent inherited semantic controls remain pending on this candidate; no runtime PASS is claimed.
