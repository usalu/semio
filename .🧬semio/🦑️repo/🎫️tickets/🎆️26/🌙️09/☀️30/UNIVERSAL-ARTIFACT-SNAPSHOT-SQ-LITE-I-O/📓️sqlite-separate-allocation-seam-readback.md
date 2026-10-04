# SQLite Separate Allocation Seam Readback

Read-only source/design audit, 2026-10-03. No implementation, Cargo execution or runtime claims.

## Actual First-Party Owners

- Rust limits and transfer control: `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs:104–156`. Seven current limits are file, semantic value bytes, schema bytes, rows, columns, tables and pages. `check_database` totals semantic cell lengths (scalar INTEGER/REAL8, textUTF8, bloblength). `check_value_bytes` must retain that meaning.
- Rust typed projection/reconstruction: same directory `🧩️artifact/🦀️.rs`. `NativeEncodingBound::add` currently compares allocation estimates against both semantic value and file limits. `Reconstruction::reserve` charges copied strings/blobs and scalars into control reconstruction_bytes/reconstruction_scalar_bytes, sharing max_value_bytes. `ordered_row_refs` builds a BTreeMap and final Vec without a separate backing ledger.
- TS physical options: same directory `🟦️.ts:12–44`: `SqliteDatabaseOptions`, normalized `limits(options)`. There is no TS SqliteSnapshotControl class under that name. `sqliteValueByteLength` is semantic accounting and must stay semantic.
- TS typed owner: same directory `🧩️artifact/🟦️.ts`: `ArtifactSqliteOptions`, `ArtifactSqliteProjection`, reconstruction and ordered relationship helpers. These expose per-call options, row/value ceilings and cancellation, but no shared transfer backing ledger.
- First-party cumulative native allocation/work controls: `🧰️framework/🔨️modules/🌱️value/🛬️decode/{🦀️.rs,🟦️.ts}` and `🛫️encode/{🦀️.rs,🟦️.ts}`. Charge/allocate/copy APIs already track cumulative admitted ownership, scoped stages restore only work cursors, and refusal categories distinguish ownershipLimit/workLimit/canceled. Their existing schemas/fixtures under each control directory are the language-neutral allocation/work contract seam.

## Recommended Clean Shared Seam

Add `max_allocation_bytes` to Rust SqliteDatabaseLimits and `maxAllocationBytes` to TS SqliteDatabaseOptions. Specify it as cumulative admitted backing storage created by one snapshot transfer. Keep max_value_bytes solely the exact semantic cell-byte ceiling and max_file_bytes solely the physical carrier-byte ceiling. Never charge Vec/String/node overhead into semantic payload accounting. Default allocation ceiling should be independently documented and fixture-backed rather than secretly multiplied from max_value_bytes; caller adjustment of one must not change the other.

Put the persistent admitted-allocation byte count on Rust SqliteSnapshotControl. Provide checked remaining/admit methods there and a common first-party bridge for native encode/decode stages: pass remaining allocation to Native*Control, then settle actual `owned_bytes` back onto the parent, including failure/cancellation paths. A stage must not obtain a fresh full budget on every nested owner. Forecasts and actual admission must be separate operations, so the same allocation is never charged twice. Phase-local progress can reset while the backing ledger never resets. Existing reconstruction semantic/scalar accounting should remain independent where it proves reconstructed literal budget; it cannot stand in for physical backing admission.

For TS introduce the analogous explicit operation-owned control in the typed/physical first-party owner, with normalized options, cancellation/progress and cumulative allocation. Pass that authority through helpers rather than reconstructing one from options in every method or hiding a ledger in global/WeakMap state. Existing public options may construct the operation once at entry. Charge explicit modeled backing sizes before buffers, slot arrays, relationship frontiers, page/schema temporary storage, or native histories allocate. Rust can use checked size_of<T> multiplications; TS should specify deterministic logical backing charges because JS engine heap overhead is not portable. Neutral tests should describe admitted bytes and rejection boundaries per implementation model, not pretend heap allocation is identical cross-runtime.

The seam covers every artifact and physical database owner. TIFF must consume it rather than inventing a TIFF budget multiplier. Native encoding traversal/frontier/output backing, erased DSL record construction, typed reconstruction slots, retained compressed history, CSV/TSV fields and BMP pixel/palette buffers all use the same operation authority. Progress/cancellation remains the existing phase callback. Retirement does not refund cumulative admitted bytes unless a separately authored peak-live contract replaces this definition; this avoids reuse-based budget bypasses.

## Schema-First Surface

Current `sqlite-snapshot/🧬️schema/🔣️.json` defines SQLite metadata only; it is not the limit schema. `🧫️fixtures/🧮️database-control/🔣️.json` specifies row checkpoints, not backing admission. Add the new bound to the actual transport contract at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🔣️.json`, its neutral `🧫️fixtures/🔣️.json`, and `🧬️schema/📜️.wit:1395` snapshot-limits record. Extend neutral first-party control law fixtures to prove independent semantic-vs-backing refusal, cumulative cross-stage admission, failed-stage settlement, preallocation rejection and interior cancellation. Existing native ownership/control schemas are suitable reference conventions; metadata must not acquire unrelated resource knobs.

## Bounded Constructor Inventory

A balanced-brace source inventory under only `🧰️framework` and `✏️s` found806 `SqliteDatabaseLimits { ... .. ... }` update-form candidates across134 files. These accept an added field through their base/default and do not require field-list edits. Counts are lexical source candidates, not compiler-confirmed callsite counts. Filtered false positives include return-type function bodies and the Default impl heading.

The full-literal SqliteDatabaseLimits sites requiring explicit new-field authorship are:

1. Central `Default::default` Self literal in `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs:115`.
2. Guest `SnapshotLimits::native` literal in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🪶️sqlite/🦀️.rs:25`.

Additional full transported SnapshotLimits constructors must mirror the field:

- Same guest schema file `From<SqliteDatabaseLimits>::from:19`.
- Plugin host `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:2891` actor-binding record.
- Plugin glue `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:75` guest record.
- SnapshotLimits struct, strict JSON required/properties, strict fixture and WIT record listed above.

Default-mutation idioms such as `let mut limits=SqliteDatabaseLimits::default(); limits.max_value_bytes=...` keep compiling but tests must set the new bound explicitly when their intended refusal is backing allocation. Shared native entry points currently use NativeDecodeControl::new(limits.max_value_bytes, ...) and NativeEncodeControl equivalents, including snapshot-capability native encode/decode and unmounted stdio drafts. Those compile without field-list edits yet remain semantically wrong until switched to the common remaining-allocation bridge. Merely adding the limit field leaves fresh per-stage budget reset and duplicate charging risks unresolved.
