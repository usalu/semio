# Semio Text Preflight Bound Separation

Read-only current source review. No owning runtime qualification or production changes.

The proposed Text decode join also needs a coherent encode preflight: its current `Bound::new` immediately forecasts 1024 bytes through `NativeEncodingBound::new`, which checks both max_value_bytes and max_file_bytes. Therefore SQL semantic max_value_bytes=257 cannot pass current preflight even before any run is visited. Fixing only decode cannot establish the new exact copied-cell law.

## Exact Authorities

- [Text preflight and authored field scan](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:20): preflight constructs `Bound::new("", control)`, then visits schema, run language/content, marks and mark href. Mark scalars are forecast at 2048 each.
- [Shared Semio Bound](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🪶️sqlite/📏️native/🦀️.rs:11): starts 1024, text length×16, entity count×128, bytes×8, scalars×2048; these are native forecast constants, not authored SQL cells.
- [NativeEncodingBound modes](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs:20): existing `file_only` disables only check_value_bytes; checked arithmetic, max_file_bytes, checkpoints and actual paid frontier methods remain.
- [Actual Text encoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🛫️native/🦀️.rs:5): original Writer emits actual strings/lists and owns cumulative encoding enforcement. Keep this path and its caller control.

## Narrow Coherent Proposal

First admit Text's complete authored schema/three tables/five-column maximum and all actual typed rows through the domain RowWriter visitor. This separately enforces copied semantic bytes (full 257, empty 26), rows and cancellation. Then retain the existing field forecast constants as a file-only forecast. A distinct Semio Bound constructor selecting NativeEncodingBound::file_only can be used by Text only; leave ordinary Bound::new unchanged for owners whose original qualification is not yet repaired. Do not change forecast constants or existing physical file ceilings merely to make the semantic law pass.

Decode should use the already documented Text-specific original outer allocation_stage join: one NativeDecodeControl funded by actual remaining allocation, with scoped borrowed full semantic admission before construction, same callbacks and final cumulative owned_bytes settlement. Neither encode forecast nor decode allocations should consume max_value_bytes as if it were a physical backing budget. Original file, allocation, refusal, cancellation and retirement laws still need genuine owning replays; this static recommendation provides no runtime credit.

## Additional Actual Encode Coupling

[Shared native Writer](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🛫️native/🦀️.rs:8) also checks its actual encoded count against both max_file_bytes and max_value_bytes. The shared encode helper at line34 creates NativeEncodeControl with remaining.min(max_value_bytes). Thus selecting a file-only preflight does not by itself separate the encoding backing/output from SQL semantic cells. This is a concrete shared-helper seam, not approval of a complete Text repair.

Provide an explicit owner-selected semantic-admitted encoding entrypoint/policy, keeping existing encode callers unchanged. It must reuse the same outer allocation_stage, fund one native controller from remaining actual allocation, retain actual encoded max_file_bytes checks, exact two-pass count equality, original row checks, bounded 256-byte advances, and return cumulative owned_bytes to the original caller. Only the already-complete Text SQL semantic visitor may authorize this policy. Do not mask caller max_value_bytes by mutating copied limits or fund a second controller. All other custom owner routes retain their current behavior until their own coherent semantic admission and original laws are qualified.

## Existing Projection Ownership Scope

Current Text project_sqlite_database at SQL Rust line62 manually tallies the correct relational cells, then constructs SqliteDatabase::from_schema and raw rows.push/vec!/String copies. It is not currently a paid RowWriter visitor. Extracting the actual row body into one authored RowWriter visitor must cover both owned SQL projection and borrowed typed admission. Existing reconstruction at line36 uses BTreeSet/BTreeMap, ordered_rows and ordinary Vec pushes alongside reconstruct_text; its traversal/frontier backing is a separate old ownership gap. A new native census does not qualify those reconstruction allocations.
