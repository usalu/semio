# 🧭 Round 2: Audits, Lanes and Decisions

Second pass over the complete monorepo. The frame model itself was in place after round 1 (`summary.md`); round 2 removes what the first audits missed and what drifted since.

## Audits (read-only, small model; every row is verified by the lane before a change)

| Audit | Area |
| --- | --- |
| `audit2-web.md` | React UI module, products, sites |
| `audit2-native.md` | Rust wgpu, OS shell, TUI, C#, Go |
| `audit2-plugins.md` | Plugin drawing code |
| `audit2-python.md` | Manim clips |
| `audit2-print-docs-tokens.md` | LaTeX, docs, tokens, fixtures |

## Lanes

| Lane | Handoff |
| --- | --- |
| wgpu chip level, bands, square controls, Select rows | `wgpu-laws-handoff.md` |
| React drag law | `react-drag-handoff.md` |
| React UI module | `web-ui-handoff.md` |
| Products and sites | `web-products-handoff.md` |
| OS React renderer and plugins | `web-os-handoff.md` |
| Python clips | `python-handoff.md` |
| CardStack gap rule, TUI schema dialect, tint projection | `bands-dialect-tint-handoff.md` |

## Decisions

- Depictions of other programs (Grasshopper, Excel, course mockups) are content and keep their shapes (laws ticket, section 7). This rejects `audit2-plugins.md` F13 and the Grasshopper and Excel rows of `audit2-python.md`.
- The circle node shape of the graph and board engines is drawn geometry chosen by the document schema, not a frame with chips. `audit2-plugins.md` F1 to F5 are rejected; changing the schema would change documents, not chrome.
- `audit2-plugins.md` F6 to F12 are the rows of the React UI lane.
- Pictures, drawings and videos run edge to edge beneath chips. Text keeps the chip-row clearance at rest.
- The wgpu Select follows the React `Select` contract: hovered row filled with emphasised text, chosen row active fill, no tick.
