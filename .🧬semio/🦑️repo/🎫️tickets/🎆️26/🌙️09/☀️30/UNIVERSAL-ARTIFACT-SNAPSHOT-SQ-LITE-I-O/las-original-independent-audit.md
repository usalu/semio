# LAS Original Independent Audit

Read-only source audit, 2026-10-13. Read root AGENTS.md, ✏️s/AGENTS.md, scope plan and paid float refinement. No runtime tests or native dispatch. Repo MCP unavailable; no lifecycle claim. Paths below use LAS base `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header` and SQLite owner suffix `🚪️io/🪶️sqlite/📸️snapshot`.

## Signed Identity

- SQL `🗄️.sql:25–40` permits every signed INTEGER child identity, including zero and i64 extrema. Fixed document/header identity and parent are 1. GPS/RGB primary keys reference point identities directly.
- TS `🟦️.ts:16` currently rejects `id < 1n`. Remove positivity only; retain integer-cell reading and exact rowid/id equality. Apply uniqueness per table to histogram and octets as well as VLR/point/GPS/RGB. Current `ids:38` checks VLR/point uniqueness and `components:39` checks optional uniqueness/ownership; histogram `44` and octet `47` only invoke identity, leaving duplicated IDs untested by local reconstruction logic.
- Retain fixed document/header checks through `artifactSqliteDocument` at `43`; histogram/header and VLR/point/document references remain exactly 1. Octet owner membership at `47` and optional matching at `39` must accept signed parents. Ordinals remain nonnegative contiguous per semantic sequence; signed identity acceptance does not change ordinal rules. A signed remap must update octet parents and both optional IDs with point IDs.
- Old Rust `🦀️.rs:15` also rejects `id < 1`; the new original receiving path must avoid inheriting that policy. No SQL positivity change is required.

## Float Contract

Framework `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs:186,209–223` defines exact classes `finite`, `positiveInfinity`, `negativeInfinity`, `nan`. Both zero signs are `finite`. Companions hold signed INTEGER bits reinterpreted as u64. NaN query cell must be NULL; non-NaN query cell may be matching REAL or exactly matching INTEGER. Query comparison deliberately ignores zero sign; companion bits retain it. Do not require REAL-only cells or introduce `negative_zero`/underscore infinity classes. LAS companion NOT NULL declarations mean an absent optional GPS is an absent row, never an all-NULL present row.

Framework receiving `🫴️receiving/🦀️.rs:43–51` offers `Projection`, `Scalar`, `cell`, `text_cell`, `row`, `push_row`; no float convenience helper. Use admitted original rows to append query scalar and signed bits/class directly. Older `insert_ieee754` at artifact `201–206` allocates intermediate cells through `float_cells:194`; it is unsuitable for cold original receiving custody. Complete validation must compare `to_bits()` and option presence: model derives PartialEq (`🧬️schema/📸️snapshot/🦀️.rs:24,119,143`) do not preserve zero-sign distinction and reject equal-payload NaNs.

## Smallest Wrong-Field Witnesses

Rust Host `🦀️.rs:39` and TS `lasSnapshotValidateSqliteSubset`, `🟦️.ts:55–57`, compare only dialect/document/schema. With those unchanged, mutating header `creation_year` (column7), histogram counter (column3), VLR `description` (column5), point `intensity` (column6), present GPS bits/query/class, or RGB red (column1) escapes their comparison. Removing header/point/options likewise escapes because these validators never consult those tables. A valid full eight-table database with just intensity changed is the smallest scalar mismatch witness; a zero-sign-only point float change proves why complete source comparison must use bits. Full borrowed validation must check schema/row widths/aliases/uniqueness/parents/order/options and all native fields before returning clean diagnostics.

No literal `defaultHost` identifier was found in current ✏️s or framework TS source; the actual exposed TS validation hook above and Rust `ArtifactSqliteSnapshotHost` implementation are the concrete audited hooks. Preserve accepted dialect `s.stdio.las`, standard `1.0`, subset `*` and exact dialect refusal.
