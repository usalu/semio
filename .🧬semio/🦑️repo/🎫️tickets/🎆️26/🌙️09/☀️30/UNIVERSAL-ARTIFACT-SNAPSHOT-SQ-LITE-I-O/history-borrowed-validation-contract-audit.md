# History Borrowed Validation Contract Audit

Read-only source/fixture audit, 2026-10-13. No execution or production changes.

Owner root: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot`. Use its existing `🗄️.sql`, `🧪️tests/🧫️fixtures/🔣️.json`, semantic/preflight/grant fixture siblings and mounted Source/Native tests; no duplicated whole corpus needed.

## Legal Surrogates And Semantic Values

All seven INTEGER PRIMARY KEY columns have no positivity predicate. Legal database surrogate IDs include zero and negative signed i64. Require each rowid equals its id cell and uniqueness within its table, not id>0. Document has exactly one row; its actual id is the parent authority. Timestamp id is a one-to-one foreign key to checkpoint id. Parent relations use actual signed surrogates, never vector offsets or semantic string IDs.

Actual checks constrain ordinals to nonnegative values and six timestamp words to 0..4294967295. Preserve u64 actor/physical_ms/logical by comparing high and low words independently or checked recombination; avoid conversion through f64/i64. Timestamp fixture clocks include u64::MAX and 9007199254740993.

Root nativeSnapshot deliberately permits duplicate checkpoint semantic IDs, duplicate author IDs, unresolved parent checkpoint strings/member pins/checkpointIds, unmatched activeAlternativeId, empty strings, absent parent, nullable avatar versus empty avatar, and empty child lists. These are equality fields, not referential integrity rules. Preserve vector order and repeated pins. Literal root fixture values containing `\\x00` are backslash characters; true NUL witness is the semantic fixture's `\u0000` text.

## Complete Borrowed Equality Predicates

| Table | Complete comparison and structure |
|---|---|
| document | Exactly one row, 2 cells, id=rowid; activeAlternativeId Null versus exact text |
| checkpoint | Exact source checkpoint count, 6 cells; document parent; contiguous unique ordinal; checkpoint id, optional parentId, message |
| timestamp | Exact checkpoint count, 7 cells; exactly one matching checkpoint surrogate; actor high/low, physical_ms high/low, logical high/low |
| author | Exact sum authors, 6 cells; checkpoint parent; per-parent contiguous unique ordinal; id/name/avatar presence and exact content |
| member_pin | Exact sum members, 6 cells; checkpoint parent; per-parent contiguous unique ordinal; documentId/checkpointId/alternativeId |
| alternative | Exact source alternative count, 5 cells; document parent; contiguous unique ordinal; id/name |
| alternative_checkpoint | Exact sum checkpointIds, 4 cells; alternative parent; per-parent contiguous unique ordinal; exact checkpointId including duplicates |

Paid id indexes for seven tables and ordered indexes for the five ordinal tables cover this relational shape. Sort only retained integer indexes, preserve original source order, reject duplicate surrogates/parent-ordinal pairs, orphan children and extra rows. Clock index joins by surrogate. Per-parent exact source counts prevent omitted empty-owner groups, extra child rows and mistaken global ordinal continuity. Checked sums/conversions avoid overflow. Validate declared schema plus exact cell arity/type independently of value comparison.

Text equality must first compare byte lengths, then compare bounded chunks with original Port work/checkpoints at least every 256 bytes. Avoid whole String equality before bounded traversal, temporary candidate P, copied text, formatted diagnostics or BTreeMap construction. Empty text still needs explicit work. Long same-prefix mismatches and exact NUL/Unicode content should reach the real original cancellation callback.

## Existing Fixture Binding And New Law Rows

Root fixture already independently authors all seven table rows (14 total): document1/checkpoint2/timestamp2/author2/member2/alternative2/pins3. Semantic fixture's 13 role cases cover individual text ownership and true NUL/Unicode text (45056 bytes). Extend existing fixture with a closed directional-validation sub-contract and malformed edits; validate that input independently using the canonical Grant schema, rather than making the whole historical corpus a new schema authority.

Minimum new malformed rows: signed surrogate renumbering accepted; duplicate rowid/id, id-cell mismatch, orphan parent in each child table, duplicate/gapped ordinal in each of five tables, missing/extra timestamp, one timestamp word outside u32, optional Null↔empty mismatch, one scalar/string mismatch for every listed field, missing/extra table/column/row. Source independent SQLite applies each edit and checks declared constraints or exact query rows; Native directional laws additionally assert typed source unchanged and actual retained frame cleanup.

Source binding should verify both directional trait overrides mount the borrowed validator, seven authored schemas and explicit field predicates, paid frame indexes, literal causes and bounded text comparison. Reject cold `Self::from_sqlite_database` candidate validation. Native should test all five grant axes, prior cumulative progress, SQL cancellation, original native cancellation and same original recipient zero/funded close. No Native qualification claimed here; parent Debug24557 is a separate live run.
