# Process3d Owned Snapshot SQLite

Process3d is a persisted parent in its own right. The BREP and Flow child capabilities do not cover its retained workshop, independent stock presentation/payload fields, machine recipes, rules, step payloads, or tool addresses. Its actual Native `Process3dSnapshot` has nine root fields, and its Source Snapshot already existed. This work does not introduce a second SQLite DTO.

## Handwritten Contract

The authored SQL contains 32 domain tables: document; independent root/payload/measure poses; payload stock; six concrete solid variants and their ownership link; ordered machines, capabilities, parameters, and rules; six concrete recipes; ordered steps, optional origins, three concrete measures; and three distinct child-slot tables. The graph uses positive SQLite surrogate IDs and dense relationship ordinals. Literal machine, parameter, provenance, imported-solid/reference, child, and target identifiers remain strings; empty, repeated, unresolved, reserved-delimiter, NUL, and Unicode values are retained independently. No semantic identifier is incorrectly constrained by surrogate uniqueness/FKs.

Every pose, dimension, margin, parameter, and measure scalar has an individually named nullable REAL query column, a signed INTEGER exact binary64 word, and a numeric-class column. Companions follow the shared first-party IEEE layout and validate all three cells together. INTEGER storage reinterprets all 64 bits; NaNs expose NULL plus `nan`, infinities expose their query values/classes, and exact signed zero is retained by the word even when SQLite normalizes the REAL query value. This replaces the initial scalar-BLOB draft before implementation verification. There is no serialized snapshot, native stream, or JSON in these tables.

The neutral fixture contains all six recipes, six solids, three measures, five stock quantities, both rule variants, optional presence distinctions, independent root/payload identities, literal child-address vectors, eight float words, and explicit widths for all 32 tables. The complete executed Source fixture projects 173 domain rows.

## Executed Source Evidence

The initial registered uncached Source baseline genuinely ran one passing law and two failures: missing owned projection/reconstruction functions and the actual Artifact parser rejecting a literal target object. The independent initial SQL oracle executed successfully. Log: `🗑️generated/process3d-source-first-red.log`.

After mounting the explicitly authored Source provider, a local shadowed identifier caused actual runtime failures and was repaired. A subsequent run passed all eight complete word round trips; its two residual oracle expectations exposed SQLite's negative-zero query normalization and the UTF-16-unit text progress boundary, and were corrected while retaining exact words and actual interior cancellation.

Final registered Source gate:

`bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-source --skip-nx-cache`

**16/16 passed, 161 assertions, 973 ms assertions / 7.6 s Nx**, with explicit quick level. Each word exercises every native scalar position in all domain variants, physical first-party SQLite export, independent Bun SQLite reads/FK checks, independent SQL editing/re-serialization, and full reconstruction. Other laws check malformed one-of/cardinality/ordinal/boolean/IEEE rows, exact 173-row admission vs 172 refusal, schema/value bounds, initial and interior cancellation, unrestricted literal child identities, and one canonical model through Artifact/Diff/Mutation/SQLite. Exact report log: `🗑️generated/process3d-source-final.log`.

The authored strict consumer command walks the actual schema tree, including its Source tests, rather than a fabricated facade. Its first run caught existing text representation guards returning objects and fixed-width inference arrays typed as variable lists. Narrow owning fixes now return text and explicitly construct the three inference coordinates. SQL import has an adjacent declaration asset. Final registered consumer gate:

`bun nx run @semio-tech/process-process3d-rs:check-snapshot-sqlite-source --skip-nx-cache`

**Succeeded uncached**; strict tsc checked the actual Snapshot, Artifact, Diff, Mutation, inference, representation, SQLite, and test sources (external declaration checking uses the existing `skipLibCheck` convention). Log: `🗑️generated/process3d-source-consumers-final.log`.

## Native Baseline Readiness — Unexecuted

Eight `sqlite_snapshot_process3d_` laws are mounted in the actual Snapshot module, without a production Rust SQLite provider or capability hook. They cover actual bare parent capability; complete ordinary Text/Pack state and all exact float words; independent literal child addresses; genuine controlled schema/typed input/output; both erased encodings with named parent/payload SQL queries; independent Bun 32-table/word oracle; the actual Plugin/document registration; and real large-field interior controls. Rustfmt parsed/formatted the authored test source; this is syntax preparation, not Rust compilation or runtime evidence.

Root owns the sole Cargo lane. The next actual Native baseline command is:

`SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/process-process3d-rs:test-snapshot-sqlite-native --skip-nx-cache`

No Native command was launched by this executor. No Native provider completion, declaration capability, controlled bridge, or end-to-end I/O success is claimed. The actual handwritten Native tagged-field producers currently have ordinary conversion methods but lack genuine controlled construction/emission; this is an inspected pending obligation, not an executed RED. Existing whole Source JSON/native transport leaves are not claimed complete by this SQL suite, and Process has no separate artifact TypeScript publication package/facade to claim as passing.

## Registrations and Files

Owning Rust `📜️script.ts` registers combined/native/Source snapshot tests and the explicit Source-consumer check. Its existing project registers corresponding Nx calls. Both current and seed launch catalogs contain Process orders 408.702–704 and 408.718. All new executable logic is in the existing owning script; no standalone scripts or runtime third-party dependencies were introduced.

Changed/created under the Process3d artifact:

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/{🗄️.sql,🗄️.d.ts,🟦️.ts}` — complete handwritten SQL, asset declaration, Source mapping.
- `…/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json` — neutral full-domain contract.
- `…/📸️snapshot/🧪️tests/🪶️sqlite/{🦀️.rs,🟦️.ts}` — eight unexecuted Native baselines and sixteen executed Source laws.
- `…/📸️snapshot/{🦀️.rs,🟦️.ts}` — only Native test mount; complete canonical Source model/parser/provider exports.
- `…/🧬️schema/🟦️.ts`, `…/🔺️diff/🟦️.ts`, `…/🧬️mutations/🟦️.ts` — reuse canonical Snapshot and exact native-width mutation indices.
- `…/📸️snapshot/📝️text/🟦️.ts`, `…/🔺️diff/📝️text/🟦️.ts`, `…/🧬️mutations/📝️text/🟦️.ts`, `…/💡️inferences/📝️text/🟦️.ts`, `…/💡️inferences/🟦️.ts` — narrow actual strict-consumer repairs.
- `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}` — owned routes.
- Root `.vscode/{launch.json,🧩️launch.seed.jsonc}` — narrow Process launch entries.

The universal goal remains incomplete. The existing Block22 and Puzzle3d six Native baselines/providers remain in their prior staged/unmounted state until their own assertions execute.

## Unmounted Native Projection Draft

The adjacent `📸️snapshot/🪶️sqlite/🦀️.rs` now contains the explicit Native projection draft for all 32 authored tables. It projects every workshop/capability/recipe/rule, all six working-solid variants, every process measure/origin, seven individually named pose scalars, and the three independent literal child relationship families. Numeric queries retain the authored REAL/null plus classification and signed exact-word companions. Structural SQL keys are positive surrogates; persisted semantic identifiers remain literal fields. The checked forecast admits schema/table/column/row limits and publishes bounded progress before actual borrowed-cell projection.

This source is deliberately unmounted while the coordinator's eight-law Native baseline has not executed an authentic provider failure. Typed capability and genuine Native input/output hooks remain unfinished. Rustfmt syntax inspection alone gives no compiled/runtime claim. The prior final sentence about Block/Puzzle staging described the earlier epoch; those owners have since executed their genuine failures and mounted their own repairs, independently of Process3d.

The draft now also contains the corresponding field-by-field Native reconstruction. It validates exact declared SQL/width/positive primary keys; unique, contiguous ordering and singleton relationships; all six recipe and six solid alternatives; every measure, quantity and boolean; IEEE companion consistency; and distinct literal semantic IDs/Child fields. Every visited entity is owned exactly once; dangling, alternate-variant and orphan rows are rejected. Schema/database admission precedes reconstruction, bounded scans reach cancellation, and actual long-field copies use the existing first-party Reconstruction helper. This is still an unmounted, syntax-inspected draft, not compiled or executed production. The completed retained Snapshot retirement and genuine physical Native bridges must be bound explicitly after the baseline receipt.
