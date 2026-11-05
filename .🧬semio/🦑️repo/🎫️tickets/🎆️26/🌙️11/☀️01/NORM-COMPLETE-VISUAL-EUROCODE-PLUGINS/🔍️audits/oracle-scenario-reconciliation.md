# Oracle Scenario Reconciliation

Read-only audit, 2026-11-05. Inspected all twelve declarations, their actual fixture directories and native diff sources. No production edits or test execution.

All twelve are real missing duplicate/refusal witnesses. None has an existing equivalent case directory to substitute: each mutation directory contains only ✅apply, whose outcome explicitly says applied. The real member duplicate fixture uses InsertMember, so redirecting these declarations to it would test a different mutation and conceal missing coverage. Preserve all existing apply and member/connection duplicate witnesses.

Declaration file: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json

Fixture root: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations

| Mutation | Declaration Line | Proposed Scenario ID | Required Case Directory | Native Duplicate Branch |
| --- | --- | --- | --- | --- |
| insert-placement | 1141 | insert-placement-dupe | ➕️insert-placement/⛔dupe | line 6–7: duplicate in own root collection |
| insert-taper | 1167 | insert-taper-dupe | ➕️insert-taper/⛔dupe | line 6–7: duplicate in own root collection |
| insert-cutout | 1303 | insert-cutout-dupe | ➕️insert-cutout/⛔dupe | line 6–7: duplicate in own root collection |
| insert-fastener | 1439 | insert-fastener-dupe | ➕️insert-fastener/⛔dupe | line 6–7: duplicate in own root collection |
| insert-panel | 1586 | insert-panel-dupe | ➕️insert-panel/⛔dupe | line 6–7: duplicate in own root collection |
| insert-floor | 1667 | insert-floor-dupe | ➕️insert-floor/⛔dupe | line 6–7: duplicate in own root collection |
| insert-panel-layer | 1814 | insert-panel-layer-dupe | ➕️insert-panel-layer/⛔dupe | line 9–10: duplicate within selected parent |
| insert-fastener-position | 1873 | insert-fastener-position-dupe | ➕️insert-fastener-position/⛔dupe | line 9–10: duplicate within selected parent |
| insert-diaphragm | 1921 | insert-diaphragm-dupe | ➕️insert-diaphragm/⛔dupe | line 6–7: duplicate in own root collection |
| insert-truss | 2079 | insert-truss-dupe | ➕️insert-truss/⛔dupe | line 6–7: duplicate in own root collection |
| insert-bracing | 2237 | insert-bracing-dupe | ➕️insert-bracing/⛔dupe | line 6–7: duplicate in own root collection |
| insert-deck | 2373 | insert-deck-dupe | ➕️insert-deck/⛔dupe | line 6–7: duplicate in own root collection |

Each source is exactly ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️<mutation>/🔺️diff/🦀️.rs. Every inspected duplicate branch returns MutationOutcome::fatal with code mutation.duplicate-id and the duplicate entity ID as target. Successful insertion clamps the requested index to the existing list length. The nested panel-layer and fastener-position branches first require their actual panelId/fastenerId parent; an absent parent is mutation.invariant, a distinct witness and not a duplicate substitute.

Author each missing ⛔dupe fixture as a real four-file case (🔣️.before.json, 🔣️.mutation.json, 🔣️.after.json, 🔣️.outcome.json). Use that mutation's committed ✅apply after snapshot as the duplicate before, reuse its actual typed insertion mutation (same ID and actual existing parent), and set after equal to before. This guarantees the target already exists while preserving valid concrete data/references and native mutation variant. Expected outcome matches the existing member duplicate convention: status rejected, code mutation.duplicate-id, messages containing fatal mutation.duplicate-id. Verify real native execution before treating these authored expectations as accepted.

The twelve declarations currently all misuse ID insert-member-dupe with directory ⛔dupe. Replace each ID with the table's mutation-specific ID while retaining its own new duplicate directory. Do not replace the duplicate declaration with ✅apply: that success witness is already separately declared and has opposite semantics.

The committed native fixture roster ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🪵️mutate-en1995-1/🦀️.rs currently includes each of the twelve apply cases, plus the actual member and connection duplicate cases, but not these twelve missing duplicate cases. Add the real new cases to that native roster and the relevant scenario list when authored; correcting only the catalogue path would leave native witness coverage incomplete.
