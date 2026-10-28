# Native Font Performance Audit

Inspected the installed ManimPango Cython and Python sources in the ticket generated environment; no processes or sources were modified.

## Confirmed Native Paths

- manimpango/_register_font.pyx lines 54–69: Windows register_font calls AddFontResourceExW with FR_PRIVATE and adds a record to registered_fonts. No explicit font-map rebuild occurs there.
- Same file lines 131–146: list_fonts creates a new Pango Cairo font map and adds every registered Windows font file to it before enumeration.
- manimpango/cmanimpango.pyx lines 94–96 and 262–264: every plain/markup text layout adds every registered Windows font file to its font map. Twenty-one registrations therefore multiply each render attachment workload, even when text uses only Anta.
- manimpango/register_font.py list_fonts constructs a fresh lru_cache wrapper for each invocation. Direct calls are not persistently cached.
- manim/mobject/text/text_mobject.py lines 442–446: Text.font_list itself is cached, so only the first normal Text warning check pays that enumeration cost per process. Lines 466, 476–477 expose warn_missing_font=False to bypass enumeration entirely.

## Correlated Test Evidence

The clip_tests agent reports the first UI chip render spent more than two minutes after registering twenty-one fonts; four subsequent UI tests passed quickly. A later direct list_fonts call in font-suite setup stalled until that agent canceled its own test. This aligns with cold font enumeration/attachment rather than a repeated per-file registration rebuild. Exact timings for native operations were not independently benchmarked during this audit.

## Recommendations

1. After validating exact bundled files and cmap coverage, set warn_missing_font=False in the repo-owned Text wrapper/defaults. This avoids expensive host font enumeration and avoids using host availability as font admission policy.
2. Register only fonts actually used by teaching output: required Anta coverage faces plus Libertinus Math; add emoji only when needed. Do not register Kelly Slab, mono and thirteen overlapping Noto faces merely because canonical CSS declares them.
3. Reduce redundant same-family subsets where coverage proves unnecessary. Preserve required Latin coverage and German glyphs; use the catalog/TTF cmap as authority rather than filename assumptions.
4. No public batch-registration API exists in the inspected ManimPango wrapper. Register all required files once before first layout; avoid repeated list_fonts probes. Do not mutate third-party registered_fonts or patch native internals.
5. Keep deterministic registration selection/order in the repo interface. The third-party store is a set, and each native layout iterates it; multiple overlapping faces with the same family/style can therefore have attachment-order ambiguity. Fewer exact faces reduces this risk.

Source inspection establishes the hot-path mechanics; the operating system/Pango internals may cache attachments, so remaining wall-clock cost must be measured after selection and warning fixes.
