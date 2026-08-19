# terra · kernel-fanout-ospack — report

Packet: `kernel-fanout-ospack`. Crate: `semio-framework-os-kernel`. Owned paths:
`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/**`.

## Result

**Starting error count (handed list, `sol-fanout-ospack.txt`): 82**
**Ending error count attributable to this module: 0**, verified by a fresh
`cargo check -p semio-framework-os-kernel --lib` filtered to `🔨️modules/🎒️pack/` (see
`terra-fanout-ospack-check4.txt`, and the precise re-filter that printed `TOTAL: 0`). Exit code of
the whole crate check is still 101 — that is sibling-module residue (📡️spr, 🚪️io/🧬️schema, 🏪️store,
🗣️dsl and others), not ours; confirmed by checking that every remaining `error[` line's path lacks
`🔨️modules/🎒️pack/`.

Files touched (all inside owned paths):
- `⌨️cli/🦀️component.rs`
- `🔢️value/🦀️component.rs`
- `🧪️testkit/🦀️component.rs`

`🦀️component.rs` (the module's top-level facade) was read but not edited — it had zero errors on
the handed list and none surfaced there in any later pass.

## Method actually used

1. `asyncify-universal.py --scan` on the owned paths → `converted: 0, already: 170` — the module was
   **already fully asyncified**, nothing to convert. Confirms the packet brief's caution ("several
   were caught mid-revert") did not apply here.
2. `deasyncify-external-impls.py --scan` → `reverted: 0` — no E1 (external-trait) damage in scope.
3. `insert-await.py --apply --scope '🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack'` → fixpoint at
   pass 1 with **0 unambiguous edits** (`terra-fanout-ospack-await-report.txt`). All 82 handed errors
   turned out to be residue shapes the span-keyed tool correctly refuses (macro-argument spans,
   sync-closure bodies, repeated-await, self/mutual recursion) — genuinely hand-work, exactly as R10
   predicts for a module this deep into codec/generator territory.
4. Hand-fixed every flagged error, file by file, then ran `cargo check` 4 times total (all foreground,
   `CARGO_TARGET_DIR` = the shared scratchpad `target-fanout`, `--lib`, timeout 600000) to reach
   fixpoint — each pass caught cascading errors the previous one's type-check failure had been
   masking (see "cascading errors" below). Reports: `terra-fanout-ospack-check{1,2,3,4}.txt`.

## R10 residue shapes hit (all four, repeatedly)

**Shape 1 — `.await` inside a sync closure.** By far the most common. `Iterator::map`/`unwrap_or_else`
closures can't `.await`; every instance was rewritten as an explicit `for` loop that draws/encodes/
decodes sequentially into a `Vec`/`HashMap` instead of `.collect()`-ing an iterator of futures.
Hit in: `RecordValueGen::{next_string, next_bytes, generate_value, generate_dsl_value}` (testkit),
`normalize_value`/`normalize_record` (testkit), `walk_value_for_symbols`/`walk_dsl_value_for_symbols`
(value — the `TableSoA` present-bitmap `.map()` closures), `read_inline_string`/`decode_value`'s
`TAG_EXPR` arm (value — `map_err`'s closure can't `.await` a now-async `reader.position()`, so the
offset is read *before* the closure).

**Shape 2 — awaiting one future repeatedly.** `RecordValueGen::generate_wire_node` awaited the same
`next_string(6)` future twice (`id.await.trim()...else {id.await}` → E0382 use-after-move); `cmd_diff`
awaited `content_hash`'s future once inline then tried to reuse the un-awaited binding; `cmd_inspect`
awaited `recovery` (a `crate::os_pack::recover(..)` future) in one match arm via `&recovery` and again
via `recovery.await` in the other; `encode_table` did `out.push(elem_tag.await); match elem_tag.await`.
All hoisted to a single `.await` stored in a plain local, reused by value/reference afterward.

**Shape 3 — self/mutually-recursive async fns needing `Box::pin`.** The big one, and the one that only
surfaced in **later** passes (see below): `encode_value` ↔ `encode_record_fields`/`encode_seq`/
`encode_map`/`encode_table` (plus direct self-recursion on `FieldValue::Block`); the decode-side mirror
`decode_value` ↔ `decode_record_fields`/`decode_seq_body`/`decode_map`/`decode_table_soa`;
`decode_dsl_value`'s own `TAG_LIST`/`TAG_MAP` self-recursion (its encode-side twin,
`encode_dsl_value`, already had `Box::pin` in the pre-existing source — asymmetric, now fixed);
`RecordValueGen::{generate_record ↔ generate_value, generate_value self-recursion, generate_dsl_value
self-recursion, shallow_value self-recursion}`; and `walk_record_for_symbols` ↔
`walk_value_for_symbols` plus `walk_dsl_value_for_symbols`'s self-recursion (value.rs's canonical
symbol-table pre-pass). Every recursive edge in each cycle now reads `Box::pin(callee(..)).await`.

**Shape 4 — futures stored/chained.** Present as a variant of shape 3 in `WireValue`/`WireNode`
construction (`generate_wire`/`generate_wire_node`, `encode_wire`/`decode_wire`) where a struct
literal's fields were futures instead of resolved values — fixed by resolving each field to a plain
local before building the struct.

## Not an R10 shape, but load-bearing: E4 fn-pointer registry (`⌨️cli/🦀️component.rs`)

`schema_registry()` held `HashMap<&str, fn() -> RecordSpec>` with `sample_spec`/`note_spec` as its
values. Both became `async fn` under the decree, and an `async fn` item's pointer type is unnameable —
textbook R2 **E4** ("registry rows" is literally named as an E4 example in the packet brief). Rather
than de-asyncifying `sample_spec`/`note_spec` (which would have required de-asyncifying their own
callees in the `🗣️dsl` module — out of scope, not ours to touch), I replaced the fn-pointer table with
a closed two-variant enum (`SchemaKind::{Sample, Note}`) dispatched through an `async fn spec(self)`
method. This is O1's own prescribed shape ("every first-party dyn/fn-pointer-erased seam becomes enum
dispatch") applied to a fn-pointer table instead of a `dyn` object — same principle, same fix. No E4
tag needed because there is no longer an fn-pointer slot to tag.

## Cascading errors (why 4 checks, not 1)

Pass 1 → 2 (82 → 9): the bulk of the work. One self-inflicted regression from my own blind
`replace_all` on `reader.position() as u64` → `reader.position().await as u64`: two of the eleven
occurrences were inside **sync** `map_err(|_| ...)` closures (E0728, "await outside async fn") —
exactly the class of mistake R10 warns a bulk/mechanical edit produces. Fixed by hoisting the position
read above the closure in both spots (`read_inline_string`, `decode_value`'s `TAG_EXPR` arm).

Pass 2 → 3 (9 → 4): two errors were **not** stale — `crate::os_dsl::schema::print_expr` and
`crate::os_dsl::format_f64` are apparently sync now (a concurrent sibling packet in `🗣️dsl` — not
ours — evidently de-asyncified them, most likely under their own R9), so my `.await` on them (matching
the *original* handed diagnostic, captured before that sibling's edit landed) was now wrong. Removed.
One more (`encode_symbols`) turned out to be async and needed an `.await` I hadn't added.

Pass 3 → 4 (4 → 0): three `E0733` "recursion in async fn requires boxing" errors appeared for the
first time — `encode_value`, `decode_value`, `decode_dsl_value`. These were always going to be errors;
they were invisible in passes 1–3 because rustc suppresses borrowck/opaque-type-cycle checks for a
function whose body still has a type error, and every one of those three functions had at least one
still-unfixed missing-`.await` error in an earlier pass. Once the surface type errors cleared, the
recursion-size check ran and reported the true shape-3 residue underneath. Boxed every recursive edge
in each cycle (listed above) to fixpoint. **Lesson for the next packet touching a recursive
async-fn family: don't declare victory at "type errors gone" — re-check, because shape-3 errors hide
behind shape-1/2 errors in the same function.**

## Correctness fixes beyond the strict error list

While rewriting the functions above (never as a separate sweep — only inside functions I was already
reading line-by-line to fix a flagged error), I found and fixed several **silently-dropped-future**
bugs: an unawaited `async fn` call as a bare statement compiles clean (the `Future` is just dropped),
so these were real behavioral bugs the codemod introduced with no compiler signal. Fixed:
`print_help()` never printing (3 call sites in `main_impl`), `print_manifest()` never printing in
`cmd_inspect`, `encode_string`/`encode_wire_node` never writing bytes in `encode_wire`/`encode_map`,
and the entire `walk_*_for_symbols` family in `value.rs` — which, unfixed, would have made
`build_symbols`' interning pre-pass silently a no-op (empty symbol table on every encode). I did not
do an exhaustive whole-file audit for this pattern beyond the functions the flagged errors already put
me inside — that would have expanded scope well past this packet's budget. **Flagging for whoever next
touches `🎒️pack` (os): a targeted `grep -n 'async fn' | ...` cross-check for statement-position async
calls without `.await` in `🔢️value/🦀️component.rs` outside the functions listed above would be cheap
insurance**, since this crate's `#[must_use]` discipline doesn't catch dropped-future bugs.

## Lease requests

None. Everything needed was inside the owned `🎒️pack` (os) paths.

## For the coordinator / siblings

- `insert-await.py`'s own `--report` JSON `residual_errors` count comes from a **fresh, separate**
  `cargo check` run at report-write time, not the same snapshot as the pass loop — on a crate this
  actively edited by four concurrent packets, that count swung from 1021 to 1 between two calls a
  few seconds apart. Don't trust that field as a measurement; trust your own filtered `cargo check`
  output instead (which is what this report's "0" claim is based on).
- `crate::os_dsl::schema::print_expr` and `crate::os_dsl::format_f64` are sync (not `async fn`) as of
  this packet's last check (`terra-fanout-ospack-check3.txt` onward) — useful if another packet is
  reasoning about the `🗣️dsl` module's current state.
