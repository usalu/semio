# Course Staffage Coverage

The shared course-coverage vectors and both language gates now check catalog identity, authored scale and view geometry, pose or clip state, thermal/structural scenarios, direct source references and dynamic cast families across every course series.

## Changes

- Added the missing TypeScript Vitest twin in staffage tests, with the same JSON vectors as Python and an independent Ajv oracle for valid and invalid cast syntax.
- Added asset ambiguity and authored-denominator negative vectors; completed the manifest-version diagnostic inventory and direct version assertion.
- Added malformed and empty timeline, fractional and oversized frame-rate, and nonpositive transport-dimension vectors. Python now handles these inputs without crashing and respects the cast schema numeric limits.
- Synchronized the office-swivel-chair and curtain-wall-bay catalog-index pose ordering to their authored documents.
- Finished the seasonal table and desk references in the cooling heating-comparison clip. The scene now selects the proper winter or summer table and desk, and measures the same selected table for toaster placement.

## Validation

- Python 3.12 course coverage: all 5 tests passed, including the complete live course source scan and catalog validation.
- TypeScript course coverage: all 52 tests passed, including the independent schema-oracle test, actual-manifest Ajv validation, catalog resolution, and complete course-source scan.
- Focused runtime configuration: codex-coverage.vitest.mjs. All verification commands use the repo tool interface. Generated dependencies remain in the ticket generated folder for the parent task to clean after its broader checks.

## Files

- 🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🎓️course-coverage/🔣️.json
- 🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎓️course-coverage/🟦️.ts
- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️course_staffage.py
- 🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json
- 🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py
- Ticket codex-coverage.vitest.mjs and this report.

## Additional Parity and Source Audit

Parsed all 219 scanned Python sources with the standard-library AST parser. No missing series, syntax errors, or staffage import aliases outside the scanner vocabulary were found. The numeric predicate rejects booleans, NaN and both infinities. Decimal JSON scale 50.0 resolves to the authored 50 representation. Added shared regressions for boolean version/FPS/scale/time, unknown timeline-key fields, malformed environment, duplicate transport layers and missing transport age. Both semantic twins now enforce these schema constraints.

Final parity verification: Python 5/5 passed and TypeScript 59/59 passed. No current source imports a staffage API under an unrecognized alias. The scanner validates direct literal calls/keywords and recognizes declared dynamic families; arbitrary runtime-computed strings remain subject to loader validation rather than general static Python evaluation.

## Final Full Guard Verification

The complete live staffage-only guard passed all 39 tests in 427.489 seconds (zero failures, zero skips), using the provisioned Manim virtual environment through repo tools exec and unittest module invocation. This includes actual course-source primitive-object/composition checks, precise diagram allowances, thermal/nonthermal policy, projection/mirror/scale/cast checks and scene thermal spans. The authoring test command now registers both course-coverage and media-freshness gates. No further verification sessions are running for this work.
