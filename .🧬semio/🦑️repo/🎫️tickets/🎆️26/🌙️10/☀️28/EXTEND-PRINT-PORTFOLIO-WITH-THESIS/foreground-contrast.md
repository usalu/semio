# Print Semantic Foreground Contrast

The actual preview review found body/cover text using a border paint and dark solid headings using page foreground. This change consumes existing schema-authored framework mutedForeground and accentForeground roles. Palette values are unchanged. Hyperlinks, citations, URLs and file links use the theme foreground. Outlined headings and the neutral filled panel rung retain theme foreground; brand-filled rungs and nested brand headings use accent foreground.

Test-driven evidence: the independent neutral vectors first produced10failures/7passes because the required semantic paints were missing from generated output. After emitting and consuming the proper roles, all pair vectors agree with existing colord a11y as a third-party oracle. The current fixture also covers neutral filled panels, and explicitly rejects the three prior body/accent/citation pairs.

| Theme | Role | Text | Surface | Owned ratio | colord ratio |
|---|---|---|---|---:|---:|
| light | body | #3e494a | #f7f3e3 | 8.3680 | 8.36 |
| light | cover | #3e494a | #e9e6d7 | 7.4288 | 7.42 |
| light | link | #001117 | #f7f3e3 | 17.2846 | 17.28 |
| light | outline | #001117 | #f7f3e3 | 17.2846 | 17.28 |
| light | filled-primary | #001117 | #ff344f | 5.3600 | 5.36 |
| light | filled-secondary | #001117 | #34d1bf | 10.0851 | 10.08 |
| light | filled-tertiary | #001117 | #fa9500 | 8.5706 | 8.57 |
| light | neutral-filled | #001117 | #cccdc1 | 11.9629 | 11.96 |
| dark | body | #91968f | #001117 | 6.3733 | 6.37 |
| dark | cover | #91968f | #061a1f | 5.9246 | 5.92 |
| dark | link | #f7f3e3 | #001117 | 17.2846 | 17.28 |
| dark | outline | #f7f3e3 | #001117 | 17.2846 | 17.28 |
| dark | filled-primary | #001117 | #ff344f | 5.3600 | 5.36 |
| dark | filled-secondary | #001117 | #34d1bf | 10.0851 | 10.08 |
| dark | filled-tertiary | #001117 | #fa9500 | 8.5706 | 8.57 |
| dark | neutral-filled | #f7f3e3 | #1c2d31 | 12.8545 | 12.85 |

The full rendering harness records actual xcolor-resolved body, outlined chapter, filled section and citation colors from the compiled document and compares them with neutral expected theme hex values. No compiler result is claimed yet for the new foregrounds. Previous PDFs are stale visual evidence; rebuild and independent inspection must complete before acceptance. Tests are wired into test-thesis and the permanent Print CheckScript.

Final independent audit passed all16new text/surface pairs, exact owned/colord agreement with minimum5.36. It also confirmed hyperref defers named-color evaluation until link output after theme aliases exist, and the actual matrix role markers cover runtime choice. Final strict47330 passed zero diagnostics. New compiled visual proof remains pending.

## Final Actual Canonical Confirmation

Final inspector16985 terminal0 and fresh thesis visual review pass. Actual operators confirm semantic body8.367996light/6.373263dark, brand-filled title and complete hierarchy number10.085119, inline citation17.28456; colord difference below0.011. Earlier title-only proof missed prefix/leaf numbering; permanent shared validator now checks every contiguous numeric run and final previews visibly confirm both1.1/1.2 parts use dark ink. Actual final SHA records are retained in visual-review.md for cleanup-safe evidence.
