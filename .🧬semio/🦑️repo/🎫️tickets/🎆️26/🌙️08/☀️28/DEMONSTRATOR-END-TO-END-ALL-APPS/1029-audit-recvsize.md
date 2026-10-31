# 1029 Audit: RetainedRecordReceiving birth quote vs fixture copyBytes

Auditor: `recvsize`. Read-only. No cargo, no edits (only this file was written).
Receiver dir: `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/` (abbrev. `R/`).

## Verdict

Mostly accidental bloat, with one intended growth. The `copyBytes` 4096 grant was made impossible by commit 700 (`b7b9f105a82`), which inlined a BLAKE3 hasher (about 2.2 KB) into the receiver's `RecordCursor::Document` variant. Commit 705 (`2aaf5b279b8`) then added `birth_demand()` quoting `2 * size_of::<Self>()`, so 4096 can no longer fund a birth. The 65536 bump is a round number chosen to pass, not a derived value. Recommended: box the Document cursor (or the hasher) so `Self` shrinks, then derive the grant from `birth_demand()` rather than hard-coding it.

## 1. Receiver struct, HEAD vs working tree

- `R/🦀️.rs:21-25` `pub struct RetainedRecordReceiving`. Working tree is identical to HEAD (`git diff HEAD -- R/🦀️.rs` is empty). No field-type edits in the working tree.
- Field types, HEAD (same in WT):
  - `source: OriginalWireSourceIdentity` (was `(usize,usize)` before commit 705, `2aaf5b279b8`).
  - `cursor: RecordCursor` (`R/📦️document/🦀️.rs:3`: `enum RecordCursor{Body(RetainedRecordBodyCursor),Document(RetainedDocumentDriver)}`). Not boxed.
  - `owned: ManuallyDrop<Owned>`, `retiring: Option<ControlledRetirement<Owned>>`, `pending: Option<Token>`, `copy: CopyPhase`, plus scalars.
- Working-tree edits in sibling files do not change any size:
  - `📜️record-body-cursor/🦀️.rs:532` only changes a `released_items` literal.
  - `🎮️driver/🦀️.rs` only adds an import.
  - The receiver test diff is a new test, not a struct change.
- So the growth is from HEAD commits, not from the current uncommitted work.

## 2. Which fields grew, and by how much

Lower bound estimates from type definitions (not compiled):

| Field / type | Before | Now | Growth | Source |
|---|---|---|---|---|
| `source` | `(usize,usize)` 16 B | `OriginalWireSourceIdentity` 48 B (address, extent, range_start, `Option<(u64,u64)>`) | about +32 B | `🛬️retained/🦀️.rs:10`, commit 705 |
| `cursor` | `RecordCursor` with Body variant only (no Document variant before 700) | Enum with inline `RetainedDocumentDriver` | at least +2.2 KB | `📦️document/🦀️.rs:21-25`, commit 700 |
| Inline hasher in Document driver | none | `schema_hash: semio_framework_hash::Hasher` | about 2.17 KB | `📦️document/🦀️.rs:25`, `🔏️hash/🦀️.rs:512-516` |

Hasher breakdown: `cv_stack: [[u32;8]; BLAKE3_MAX_STACK_DEPTH]` with depth 64 gives 2048 B (`🔏️hash/🦀️.rs:335`). `Blake3ChunkState` is about 112 B (`🔏️hash/🦀️.rs:437-443`). Total about 2.17 KB.

The Document driver also inlines `Anchor` (header 32 B, footer ring 84 B, ordered footer 84 B, `🎮️retained/⚓️catalog-and-anchor-cursors/🦀️.rs:51-57`), two `Segment` cursors, a `Catalog` cursor, a `Value` cursor, and a `PagedList` (`🌱️value/📋️list/🦀️.rs:169`). These add more on top, but I did not size them.

So `size_of::<RetainedRecordReceiving>()` is at least about 2.2 KB, and `birth_demand().copy_bytes = 2 * size_of::<Self>()` is at least about 4.3 KB, which is above 4096.

## 3. Intended or accidental?

- Intended: the 32 B growth in `source` (commit 705) is deliberate identity hardening. Small.
- Accidental: the inline 2 KB BLAKE3 stack and the other Document driver state sit directly in the enum variant. Only Document-mode receivers use them, and the enum is not boxed. Body-mode receivers pay the same cost. This is the `clippy::large_enum_variant` pattern, and boxing the variant is the standard fix.
- A 64-level stack is also more than the usual BLAKE3 bound (54 in the reference crate). That is a secondary point.

## 4. Is 65536 principled?

No. It is a round number about 15x the current birth quote (about 4.3 KB), chosen to pass. It is not derived from the quote.

- The fixture carries it at `R/🧪️tests/🧫️fixtures/🔣️.json:10` (`"copyBytes": 65536`). It was 4096 at HEAD `git show HEAD:...`.
- The test reads it at `R/🧪️tests/🦀️.rs:31`. `admit` returns `Ok(None)` when unfunded (`R/🦀️.rs` admit, `if !funded(grant,Self::birth_demand())`), so the `.unwrap().unwrap()` at `R/🧪️tests/🦀️.rs:13` panics. This is why the executor's fix is needed.
- The birth-refusal test at `R/🧪️tests/🦀️.rs:101-112` already uses `birth_demand()` directly, so the fixture number is only needed for the turn grant.

Recommended, in order:
1. Box the Document variant: `Document(Box<RetainedDocumentDriver>)` in `R/📦️document/🦀️.rs:3`, or box the hasher. Re-measure `size_of::<Self>()`. This removes the inline 2 KB from Body-mode receivers.
2. Derive the fixture grant from the quote, or assert it in the test: `assert!(grant.maximum_copy_bytes >= RetainedRecordReceiving::birth_demand().copy_bytes)`. Set the fixture to a value justified by the chosen size (for example `2 * size_of::<Self>()` plus per-turn headroom).
3. Keep 65536 only if it is documented as a per-turn budget and still passes after step 1.

## Caveats

- Sizes are estimates from type definitions, not `size_of` output. No cargo was run, per the audit rules.
- I did not size `RetainedRecordBodyCursor`, `Value`, `Catalog`, or `Segment`, so the total is a lower bound.

## Evidence index

- `R/🦀️.rs:21-25` receiver struct, `R/🦀️.rs:35` `birth_demand`, `R/🦀️.rs` admit `funded` check.
- `R/📦️document/🦀️.rs:3` `RecordCursor` enum, `:25` inline `Hasher`.
- `R/🧪️tests/🧫️fixtures/🔣️.json:10` grant `copyBytes`.
- `R/🧪️tests/🦀️.rs:13, 31, 101-112`.
- `🔏️hash/🦀️.rs:335, 437-443, 512-516`.
- `🛬️retained/🦀️.rs:10` `OriginalWireSourceIdentity`.
- Commits: `bd9e4e02bd0` (696, fixture 4096 added), `b7b9f105a82` (700, Document driver and inline hasher), `2aaf5b279b8` (705, source identity and `birth_demand`).
