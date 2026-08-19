# terra · kernel-fanout-spr — report

## Result

**295 → 2 errors** in `📡️spr/**`, measured against `cargo check -p semio-framework-os-kernel --lib`
filtered to my module's files. The 2 remaining are one root cause, documented below with a
`lease-request`. Everything else in the packet's work-list is fixed.

Commands run (verbatim, foreground, `CARGO_TARGET_DIR` = scratchpad `target-fanout`):

```
$ CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-fanout \
  cargo check -p semio-framework-os-kernel --lib --message-format=short 2>&1 | grep -F "📡️spr" | wc -l
```
- check1 (after the dict.resolve revert only): 295 errors — matches sol's captured baseline exactly.
- check2 (after 📜️history bulk fixes): 212 errors.
- check3 (after 📜️history/🦀️component.rs/🧵️channel/⌨️cli/🎮️command hand-work): 165 errors.
- check4 (after 💎️materialize/🔌️io/🧪️testkit bulk fixes): 34 errors.
- check5 (final): **2 errors**, both in `📜️history/🦀️component.rs`, both the same root cause.

Final verification, re-run to capture the exit code (crate-wide, not just my filter — other packets'
sibling modules are still red, expected and not mine):
```
$ CARGO_TARGET_DIR=.../target-fanout cargo check -p semio-framework-os-kernel --lib --message-format=short
EXIT:101
$ ... | grep -F "📡️spr" | grep -c "error\["
2
```

## Starting state

`asyncify-universal.py --scan` on my paths reported `converted: 0, already: 602` — the module was
**already fully asyncified** before I started (sol's own earlier pass, per the packet brief).
`deasyncify-external-impls.py --scan` found 0 E1 damage. `insert-await.py --apply` reached fixpoint
at pass 1 with `await-edits=0` both before and after my hand-work — every fix in this packet was
genuine hand-work, exactly as the packet predicted ("your residue is the hard tail").

## The dominant residue shape (not in R10's four — a fifth, repo-wide pattern)

By far the largest class of errors (150+ of the original 295) was **not** one of R10's four listed
shapes. It was a systematic bug from an earlier automated pass: a constructor call was correctly
left un-awaited as a bound local (`let mut input = ByteReader::new(payload);`), then every
subsequent use of that local re-wrote `input.method()` as `input.await.method()` — awaiting the
**binding** on every single call instead of awaiting the constructor once. Since `.await` on a
non-`Copy` local moves it, only the *first* such use actually type-checked as a `Future`; every
later use was rustc reporting some derived shape of "this is a Future, not the resolved type"
against a local that had (in the buggy source) never actually been resolved at all. Symptom
patterns: `expected &mut ByteReader<'_>, found &mut impl Future<...>`, `X is not a future`
(re-awaiting an already-resolved value), `E0605` casts, `E0369` comparisons — all traceable to one
of:
- `let mut X = SomeType::new(...);` missing its own `.await` — X should have been the RESOLVED type
  everywhere after.
- `X.await.method()` — the wrong await, should be `X.method().await`.
- `X.await` on an ALREADY-resolved local reused a second time (`Ok((dict.await, edit_ids))` after
  `dict` had already been the concrete type for the whole function) — simply delete the stray
  `.await`.

I fixed this pattern with small, reviewable Python scripts (not name-keyed — keyed on a specific,
manually-verified async definition's exact call-site text, e.g. `DictBuilder::new()`,
`FrameCursor::new(`, `.next_range(`, `input.await.read_u8(` → `input.read_u8(`), applied file by
file, each verified by re-grepping for leftover un-awaited/double-awaited occurrences before moving
on. This is diagnostic-adjacent (the exact broken pattern was identified from a real compiler error
each time) but not literally `insert-await.py`'s span-keyed mechanism, so I want to be explicit
about it rather than let it pass as "the shared tool did it."

## R10's four shapes — all four occurred, each fixed by hand as the ticket prescribes

1. **`.await` inside a sync closure** (most common): `.ok_or_else(|| malformed(...))`,
   `.map_err(io_err)`, `Iterator::map`/`.filter_map` closures calling now-async pure helpers,
   `Result::and_then(|reader| reader.log())`. Fixed per-case: hoisted the await out of the closure
   (rewrote as a plain loop) wherever the callee had a real reason to be async, or applied **R9**
   (reverted the callee to sync + tagged) wherever the callee was a pure in-memory accessor whose
   *only* consumers were sync-closure-bound. R9 fns tagged this packet: `text_error_to_protocol`,
   `build_alternative_head`, `checkpoint_head_edit_ordinal`, `malformed` (🧵️channel, 💎️materialize,
   both independently-defined local copies), `io_err`, `fingerprint`, `kind_name`, `hex32`,
   `prefix_message` — each has a `// 🚫️async: R9 ...` comment with its consumer named.
2. **Awaiting one future repeatedly**: `field_authors`'s `filter_map` calling `field_text(rec, id)?`
   twice per item; several `Iterator::map(|_| async_fn(...))` chains — all hoisted into loops with
   one `.await` per element.
3. **Self/mutually-recursive async needing `Box::pin`**: none encountered in this module.
4. **Futures stored in structs / `map`/`and_then` chains**: the `open_and_log` helper (new,
   `⌨️cli/🦀️component.rs` and `🧪️testkit/🦀️component.rs`, one copy each — `reader.log()` sequenced
   explicitly since `Result::and_then`'s closure can't await) is this shape's fix, used at 5 call
   sites in cli and 2 in testkit.

## `Iterator::next` under E1 — the crate's one sanctioned `resolve_ready` bridge, reused (not redefined)

`📜️history::EditIter`/`RevEditIter` implement std `Iterator` (E1 — external trait, must stay sync),
but their bodies call now-async `next_frame`/`prev_frame`/`payload`/`apply_dict_record`/`decode_edit`
— all pure in-memory byte-parsing with no genuine suspension point. R4 bans a per-call-site
`block_on`, and R2 E5 caps "at most one [bridge] per crate." `🚪️io/🦀️component.rs` (off-limits to me,
same crate) already defines `pub fn resolve_ready<F: Future>(fut: F) -> F::Output`, built for exactly
this. I call it (`crate::os_io::resolve_ready(...)`) rather than defining a second bridge — a read
of an existing crate-shared helper, not an edit to `🚪️io`. Same fix applied to two sync test-fuzzer
closures in `🧪️testkit` (`fuzz_truncation`/`fuzz_bit_flips` take `impl Fn(&[u8]) -> Result<(), String>`,
from `pack::testkit`, sync by contract).

## `async-test-attr.py`

Ran `--scan` then `--apply` over my paths: **262 sites, 8 files**. Added the dev-dependency to
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` (a per-crate manifest, not the
registrar-only root `/Cargo.toml`) — the tool's own `--apply` does this idempotently; I did not
hand-edit it. `#[test] async fn` → `#[semio_framework_async_macros::async_test] async fn` throughout.

## The 2 remaining errors — `lease-request`

```
📜️history/🦀️component.rs:623:58  E0308  write_id_field: dict.intern(s) inside a sync closure
📜️history/🦀️component.rs:629:38  E0728  read_id_field:  dict.resolve(idx).await inside a sync closure
```

Root cause: `DictBuilder::intern`/`DictReader::resolve` (defined in
`🧰️framework/🔨️modules/📡️replication/📖️dictionary/🦀️component.rs`, **not mine**, a separate crate the
kernel merely speaks) were asyncified — correctly, under the universal-async decree — but their only
callers, `crate::os_spr::scalar::scalar::write_id`/`read_id`
(`🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️component.rs`, **also not mine**), still declare
their resolver/interner parameters as plain sync closures:

```rust
pub async fn read_id<'r>(input: &mut ByteReader<'_>, resolve: impl Fn(u32) -> Result<&'r str, PackError>, ...)
pub async fn write_id(out: &mut ByteWriter, id: &str, mut intern: impl FnMut(&str) -> u32, ...)
```

`read_id`/`write_id` call `resolve`/`intern` **from inside their own parsing logic** (mid-tag-decode,
not before it), so I cannot precompute the value outside and hand it in — the only fix is on the
`📡️replication` side. Two options for whoever owns that crate:
1. Change `resolve`/`intern`'s bounds to accept async closures (`AsyncFn`/`AsyncFnMut`, stable since
   the edition this repo targets — I did not verify which edition, that's part of the lease).
2. Simpler: change `read_id`/`write_id` to take `&DictReader`/`&mut DictBuilder` directly instead of
   closures, and call `.resolve()`/`.intern()` inline (both fns are already `async`, so this needs no
   further signature change beyond dropping the closure parameters) — my read is this is the
   cleaner fix, since the closure indirection's only remaining purpose (letting a caller substitute a
   different resolver) has no other user in the tree; grep confirms `read_id`/`write_id`'s only
   callers repo-wide are `📜️history`'s `read_id_field`/`write_id_field`.

**This is a genuine `lease-request`**, not deferred hand-work:

```lease-request
file: 🧰️framework/🔨️modules/📡️replication/🔢️scalar/🦀️component.rs
current (lines 234, 252):
    pub async fn write_id(out: &mut ByteWriter, id: &str, mut intern: impl FnMut(&str) -> u32, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<(), PackError> { ... }
    pub async fn read_id<'r>(input: &mut ByteReader<'_>, resolve: impl Fn(u32) -> Result<&'r str, PackError>, ordinal_to_id: impl Fn(u64) -> Result<&'r str, PackError>) -> Result<String, PackError> { ... }
requested: replace the `resolve`/`intern` closure parameters with a direct `&DictReader`/
`&mut DictBuilder` parameter (or, if you'd rather keep the closure shape, an `AsyncFn`/`AsyncFnMut`
bound) so 📜️history's read_id_field/write_id_field (🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/
📜️history/🦀️component.rs:611-630) can await DictBuilder::intern/DictReader::resolve inside them.
```

Until that lands, lines 623/629 stay exactly as reverted to their pre-fixpoint form (I tried the
`.await`-stripped variant first — that just trades E0728 for an equally-blocked E0599 "no method
named map_err found for opaque type impl Future" — so the `.await`-present form is the more honest
state to leave it in; either way it needs the lease).

## Files touched (all within owned paths)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs` — the bulk of the work:
  `input.await` pattern (14 sites), `write_timestamp`/`read_timestamp`/`FrameCursor`/`cursor.await`
  bugs, `author_spec` reverted to sync+E4-tagged (fn-pointer slot `Shape::Record(fn() -> RecordSpec)`
  in 🗣️dsl, not mine, requires a bare sync fn pointer), `field_authors`/`text_error_to_protocol` R9,
  102 `.await`-insertion fixes across the spec-building/print/encode/decode/append/scan regions,
  `EditIter`/`RevEditIter::next` rewritten around `os_io::resolve_ready` (E1), `write_id_field`/
  `read_id_field` left blocked (see lease-request above).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs` — `cursor.await` pattern (3 sites),
  `frame_len()`/`hasher.hash()` missing awaits, `build_alternative_head`/
  `checkpoint_head_edit_ordinal` reverted to sync + R9-tagged.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` — local `malformed` R9,
  `read_vec_bytes`/`read_vec_envelope` hoisted out of `Iterator::map` into loops (R10 shape 2).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️component.rs` — `kind_name`/`hex32`/
  `fingerprint` R9, new `open_and_log` helper (5 call sites), `build_history_file`/`FrameCursor`
  bulk-await sweep, one hand-rewritten `std::iter::from_fn` test helper (R10 shape 1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` — `MutationDescriptor::new`
  E0505 borrow fix, `prefix_message` R9, `plan_of`/`fold_plan_diff`/`fold_plan_inverse` fully
  rewritten around the `input.await`-family bug (`Planner::new`, `.into_parts`, `.diff`/`.apply`
  chains), `MutationMessage::fatal`/`.at` missing awaits.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/💎️materialize/🦀️component.rs` — local `malformed` R9,
  `input.await`/`cursor.await`/`ReverseFrameCursor::at_end` bulk sweep, `apply_dict_record`/
  `prescan_dict_and_edits` dict-future bug.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🔌️io/🦀️component.rs` — `io_err` R9, `flush_dict`
  dict-future bug (same shape as history's `flush_dict_delta`), `compact`'s whole encode sequence,
  132-site mechanical sweep over `HistoryFile`/`TailFollower`/test-helper calls.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` — the largest file: RNG
  helper awaits, `HistoryLogGen::generate`'s multiple already-resolved-then-re-awaited locals (`id`,
  `change_id`, `checkpoint_id`, `doc_id`, `schema`), new `open_and_log` helper +
  `os_io::resolve_ready` at the two sync-fuzzer-closure sites, and the whole `assert_mutation_*`/
  `assert_diff_algebra_*`/`assert_missing_target_is_error`/`assert_outcome_deterministic`/
  `assert_policy_matrix` law-assertion family rewritten around `MutationOutcome`/`MutationDiff`/
  `DiffAlgebra`'s now-async methods (`.diff()`, `.messages()`, `.apply()`, `.absorb()`,
  `.worst_level()`, `.is_empty()`, `D::between`).
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — `async-test-attr.py --apply`'s own
  idempotent dev-dependency insertion (not hand-edited).

Scratch files in the ticket folder: `terra-fanout-spr-check1.txt` … `terra-fanout-spr-check5.txt`
(the 5 `cargo check` snapshots), `terra-fanout-spr-history.txt`/`terra-fanout-spr-history2.txt`
(per-file error extracts), `terra-fanout-spr-await-report.json` (insert-await.py's own report).

## For siblings / the coordinator

- **The `input.await` / `X.await.method()` / re-awaiting-an-already-resolved-local family of bugs is
  almost certainly present in every OTHER module that went through the same earlier automated
  fixpoint pass sol described** ("I converted 606 fns and ran the await fixpoint"). It is NOT one of
  R10's four documented shapes and produces confusing derived errors (E0605 casts, E0369
  comparisons, "X is not a future") rather than a clean "missing await" diagnostic, so it's easy to
  mis-triage as something else. Worth adding to `📌️important.md` as a fifth named shape if other
  packets hit it.
- The `📡️replication` dependency crate (outside every packet's declared ownership in this ticket, as
  far as I can tell) is mid-conversion **right now**, live, concurrently with this packet — I caught
  it changing under me once (`DictReader::resolve` went from sync to async between my first `grep`
  and my first `Read` of the same file, seconds apart). Whoever owns that crate should know
  `read_id`/`write_id`'s closure-shaped parameters are the one remaining blocker their conversion
  leaves for kernel-side callers.
- `crate::os_io::resolve_ready` (in the off-limits `🚪️io` module, same crate) is the crate's
  sanctioned E5 bridge per its own doc comment. I read and called it from 📡️spr but never edited
  `🚪️io`. If another packet also needs it, it's already there — don't add a second one (R2 E5's "at
  most one per crate").
