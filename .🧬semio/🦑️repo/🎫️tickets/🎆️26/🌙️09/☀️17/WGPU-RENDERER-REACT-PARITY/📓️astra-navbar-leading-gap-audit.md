# Navbar Leading Gap Audit

Checkpoint12's actual WGPU chrome places Artifact at x3.2 with width62.454487 and Catalogue at x128.10898. The intervening gap is exactly one additional Artifact width. React's corresponding controls are adjacent at approximately x3 and x79; its differing individual widths also include the still-missing trailing drag handle.

The source contains a separate deterministic placement defect: `navbar_leading_tab_row_rect` initializes x from its passed `lead_x`, then adds every earlier sibling's width. `render_navbar_step` passes its advancing `cursor.x`; after completing each chip it updates that cursor to the previous chip's right edge. The next chip therefore counts preceding width twice. This affects actual paint and hit geometry together, so clicking the visible Catalogue can work while its placement remains wrong.

Sol Chrome should repair this alongside chrome geometry using one authority: a fixed band origin plus preceding sibling widths, or an advancing cursor with no repeated prefix. The chosen law should cover at least three leading tabs, multiple anchor bands if supported, and a resumed glyph cursor; checking only the first tab or total band width misses the duplicate prefix. Include the separate drag-handle width in the React DOM oracle rather than compensating for it with arbitrary spacing.

No production file was changed by this audit. Source seams: Shell `navbar_leading_tab_row_rect` around22535 and `render_navbar_step` phase10 around23366–23392. Evidence: checkpoint12 `steps.json` boot/dismiss-tour chrome and `wgpu/02-dismiss-tour.png`.
