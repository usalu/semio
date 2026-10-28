# Assignment Table Title Chips

Only the three explicit AssignmentTable title arguments in each language were changed to empty arguments. All other source bytes were verified unchanged by reversing only those replacements and comparing the exact byte buffers. Atomic full-file replacement was followed by exact readback verification.

- de: before 819b7978938a859e63f13d562e2c72ab877c7e8f3f325ac145980517a1ae11a4; after e27a8e3e3f08829b66572608520249fbffdebe450b487aa4817eb9c3a7da4cdd; 1100 to 1074 bytes. Removed explicit labels: Beispiel, Tabelle A, Tabelle B.
- en: before 0e862445c5e8d0e3e441ee5d64762741a472db18574b6b17c3ca70a05061d5d8; after d81636004fd518abe2e23b58c8abe581326e5ee0524eaaaf56dad7ba46cbb8fd; 1094 to 1073 bytes. Removed explicit labels: Example, Table A, Table B.

Visual verification completed against the final frozen sources.

## Final English Native Verification

Pinned Tectonic 0.16.9 compiled the original English main file successfully with the registered offline bundle and shared compiler-input search directory. The PDF has three pages, 127079 bytes, title Finger Exercise: Parametrization, author Architecture, and en-US catalog language. The compile log contains no overfull or underfull boxes.

The focused English validator passed after correcting its expected label punctuation to Table:. It found exactly one Example subsection heading, no Table A or Table B title text, and exactly three automatic right chips at x = 513.64886, 510.04226, and 510.04226 PDF points, numbered 1.1.a, 1.2.a, and 1.2.b. No extracted character extends outside a page.

The independent native geometry validator passed all three empty left name zones, the absence of ghost left tabs, the presence and connection of the automatic right chips, the eight-row worked example with original filled values, the bare directed K3 and six separate ruled numeric/category changes, and both compact 9 by 9 exercise matrices with centered canonical artwork. The graph implementation and source content outside the six title arguments remain byte-identical.

All three full-resolution rendered pages were visually inspected. Each table starts with a plain connected top-left border and retains only its automatic right chip. Page one preserves the Example heading, table, diagram, and six filled ledger entries. Page two has two aligned matrices and no Table A/B chips or outlines. Page three preserves the full graph, arrowheads, sixteen icons, and twenty blank ledger rules. Headers, footers, page numbers, and all page transitions remain clean, with no clipping or overlap.

Retained verification input: chip-content-native.py. Temporary PDF, logs, renderings, and machine-readable metadata remain under the ticket generated folder for final parent cleanup.

Final English PDF SHA-256: 00791676dbb1cb1fdba72d1a2c6c864abd1f3e0194209b56099c59250657c23d
