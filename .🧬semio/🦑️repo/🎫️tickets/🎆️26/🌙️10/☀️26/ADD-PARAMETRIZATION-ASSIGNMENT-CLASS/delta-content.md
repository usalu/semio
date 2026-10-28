# Named Parameter Changes Content Audit

The new content scope is limited to the six worked-example ledger arguments in each language. Existing categorical shape names and arrows, assignment metadata, headings, empty table-title wrappers, table values, diagrams, row spacing, exercise matrices, and blank task ledger must remain unchanged.

The two zero tuples describe no numerical change and will be omitted entirely, leaving only the existing categorical transition. Each positive tuple will become the three named changes L+1; B+1; H+1. Each negative tuple will become L-1; B-1; H-1. The original reverse-pair ordering remains unchanged.

Baseline German content SHA-256: e27a8e3e3f08829b66572608520249fbffdebe450b487aa4817eb9c3a7da4cdd.

Baseline English content SHA-256: d81636004fd518abe2e23b58c8abe581326e5ee0524eaaaf56dad7ba46cbb8fd.

Root PDF edit marker succeeded, and independent schema-first v16 regression failed on the original tuple notation before production edits. Plain native Anta parameter text was selected after independent fontTools and ReportLab width checks: longest German 60.28 mm and English 58.20 mm within the unchanged 64.6 mm rules at 9 pt. No font or layout change is required.

## Source Authoring and Freeze

Exactly six ledger arguments per language were replaced using full-file atomic writes. Reversing only these exact replacements reproduced every original byte. Final readback matched the authored buffers; original categorical phrases and arrows were preserved. No other production files were edited by the content owner.

- de: before e27a8e3e3f08829b66572608520249fbffdebe450b487aa4817eb9c3a7da4cdd; frozen after 7419ce4530b9bb33df48ae05f4a24afd6cae4edc65bf5464b330cee6bab1e702; 1074 to 1064 bytes.
- en: before d81636004fd518abe2e23b58c8abe581326e5ee0524eaaaf56dad7ba46cbb8fd; frozen after df440cd167a4088481df56c27622bb714a2e3547db16c78a838e3a848a09d998; 1073 to 1063 bytes.
