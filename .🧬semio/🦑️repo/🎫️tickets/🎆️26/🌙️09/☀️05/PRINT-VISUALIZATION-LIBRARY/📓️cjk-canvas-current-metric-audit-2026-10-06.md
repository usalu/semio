# Independent Registered Canvas Metric Audit

Observed 2026-10-06, actual terminal session 83662 exit 0. The existing print project was selected through Bun 1.3.14 and Nx exec, with daemon disabled and ticket-local Nx data/cache. No product files or native compilers were changed or invoked. Two earlier evaluator attempts exited 1 before registration (static import inside eval, then require unavailable in ESM); they are infrastructure failures, not font evidence.

The successful console contains `[DEBUG] primary exact registered=true` and `[DEBUG] fallback exact registered=true`. Exact tracked Anta TTF and NotoSansCJKsc OTF were registered as separate unique aliases. Each Han run explicitly selected SemioAuditCJK; Latin runs explicitly selected SemioAuditPrimary. Mixed text was divided into those runs without trimming spaces. Font size conversion was TeX points × 96/72.27 pixels; Canvas advances converted by 25.4/96 to millimetres.

| Text | TeX pt | Canvas mm | Current measurePrintSans mm | Difference mm |
|---|---:|---:|---:|---:|
| 根 | 8 | 2.809875 | 1.405839 | -1.404036 |
| 葉 | 8 | 2.809875 | 1.405839 | -1.404036 |
| 根葉 | 8 | 5.622396 | 2.811678 | -2.810717 |
| Root 根 / Leaf 葉 | 8 | 23.076958 | 20.272092 | -2.804866 |
| 根 | 12 | 4.217458 | 2.108759 | -2.108699 |
| 葉 | 12 | 4.217458 | 2.108759 | -2.108699 |
| 根葉 | 12 | 8.434916 | 4.217518 | -4.217399 |
| Root 根 / Leaf 葉 | 12 | 34.620729 | 30.408137 | -4.212592 |

All eight CJK observations exceed a 0.03 mm comparison tolerance: a genuine current metric RED. Eight Latin controls (`office affine fi fl`, `Straße Größe`, two leading/trailing spaces around First Child, and `a,b`, at both sizes) differ by only 0.001395–0.005525 mm. These controls support the unit conversion and retain current primary shaping and whitespace semantics. The fallback correction should therefore preserve primary runs and replace only unsupported scalar runs.

The existing metric owner line 14 maps absent codepoints to glyph 0; line 51 converts its summed advance using TeX points. The current measured Han advance is roughly half the registered fallback width. The source and font byte hashes, full numerical run/ink observations and exact exit receipt are retained in `📥️authored-inputs/cjk-canvas-current-metric-source-capsule-2026-10-06.json`. The numerical observation is in `📥️authored-inputs/cjk-canvas-current-metric-observation-2026-10-06.json`; generated console remains `🗑️generated/cjk-canvas-current-metric-audit3.log`.

Canvas interface bounds: GlobalFonts.registerFromPath returns FontKey or null; has(name) checks family registration, not scalar coverage. Canvas exposes measureText and actualBoundingBox fields but no glyph coverage or selected glyph ID method in the inspected declaration. Explicit alias runs avoid silently asking Anta to select a system fallback; successful registration and nonempty ink are observed, but do not prove engine fallback was disabled or identify actual glyph IDs. Exact cmap coverage remains a separately required source parser/native observation. No native glyph GREEN is claimed.

A neutral regression can reuse these 16 text/size records, compare each deterministic selected run with the registered third-party Canvas advance within 0.03 mm, verify exact font SHA-256, and assert preserved spaces and primary-run agreement. Existing schema-first fallback proposal describes bounded cmap format 4/12 and source binding. This audit establishes the pre-fix numerical failure without introducing an external runtime dependency.
