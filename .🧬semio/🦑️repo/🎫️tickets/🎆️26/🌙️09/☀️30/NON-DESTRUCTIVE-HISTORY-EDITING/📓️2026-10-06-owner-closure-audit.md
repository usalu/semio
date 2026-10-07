# PPTX and Semio Owner Closure Audit

Read-only source audit on 2026-10-06 of the sixteen PPTX owners recorded in `📓️2026-10-06-pptx-semio-owner-repairs.md` and the three Semio PNG consumers described there. Applicable instructions read: root `AGENTS.md`, `✏️s/AGENTS.md`, and `✏️s/🔌️plugins/🗄️stdio/AGENTS.md`. No source edits, builds, tests, modifying Git commands, or ticket/goal operations were performed.

## Findings

No concrete visibility, relocated-helper dependency, or ownership defect was found in the inspected repairs. This is a source census, not compilation or runtime proof. Native runs and browser assertions remain necessary.

## Declaration Closure

- PPTX schema snapshot declares `native` and `subset` with `pub(crate)` at lines 294 and 297. Native declares its backing child `pub(crate)` at line 11. Binary snapshot imports that backing at line 36 and forwards encoding; text snapshot imports it at line 48 and forwards input. Backing exposes both functions `pub(crate)` at lines 32 and 55. Its `pub(super)` preflight at line 17 is consumed by its actual parent at native line 48. Private native read/write helpers remain accessible to their backing descendant.
- Snapshot SQLite owns its private backing child. That child declares projection/reconstruction `pub(super)` at lines 4 and 6. Their functions use `pub(in super::super)` at projection line 19 and reconstruction line 25, matching the grandparent snapshot module calling them at lines 95 and 99. Parent-private constants, helper functions, and ownership guards are accessible from these descendants.
- Canonical XML snapshot declares `pub mod ownership` at line 560. Canonical XML SQLite snapshot declares public `XmlSqliteTables` at line 10 and public measurement, append, and reconstruction helpers at lines 73, 77, and 92. PPTX imports these declared owners rather than private assembly children.
- Relocated text mutation carrier is local to its `mutations_codec` consumer at lines 19–24. Its `PptxSnapshotRecord` dependency and conversion helpers remain `pub(crate)` in schema snapshot at lines 239, 251, and 261. Mutation child modules are public and imported by the schema mutation glob. `OpText` is implemented once in the text consumer; no duplicate implementation was observed in the inspected schema mutation owner.
- Strict/transitional IO callers resolve conformance through their IO owners, whose public analysis child exports expose `check_strict_conformance` and `check_transitional_conformance` at lines 218 and 212 respectively.
- Semio drawing PNG exporter line 22, image PNG exporter line 5, and image import fixture line 2 import `PngProjection` from PNG 1.2 any IO. That owner declares the public structure at line 20, authoring at line 2141, and projection at line 2267. No removed schema projection type is required by these imports.

## Ownership Guards

Native read retains decoded XML documents in `DecodedValue` until the closing field delimiter is accepted, then transfers them into a `Parts` guard. Native backing retains the completed snapshot in `DecodedValue` through its final checkpoint. SQLite reconstruction guards XML documents and transferred parts through subsequent failure paths and guards the completed snapshot through its final checkpoint. These inspected transfers do not introduce an unguarded recursive XML drop path.
