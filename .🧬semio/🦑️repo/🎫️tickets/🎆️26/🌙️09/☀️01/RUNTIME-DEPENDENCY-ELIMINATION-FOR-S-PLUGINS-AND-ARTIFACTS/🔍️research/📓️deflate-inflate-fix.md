# 🛠️ `inflate` fix report — root cause, fix, and verification

File: `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs`. Defect and reproduction: `📓️deflate-inflate-defect.md`
(ticket root). This report covers only the fix and its verification.

## Root cause

`Inflater::advance` and the shared `BitReader`/`Huffman` state machine were already correct — the
compressor was already known correct. The entire defect was in the one-shot convenience wrapper
`pub fn inflate(...)`, which admits input one byte at a time and drives `Inflater::advance` in a
loop. On **any** `InflateOutcome::NeedInput` — which just means "the state machine wants the next
admitted byte, and one may still be coming" — the driver gave up immediately instead of feeding the
next byte and continuing:

```rust
// before (buggy)
match inflater.advance(&mut pending, input_complete)? {
    InflateOutcome::NeedInput => return Err(DeflateError::UnexpectedEnd),
    ...
```

Trace for the canonical empty fixed-Huffman block `[0x03, 0x00]`: admitting byte 0 supplies exactly
the 3-bit block header (`BFINAL=1`, `BTYPE=01`), leaving 5 spare bits in the reader. The next phase
(`DecodeSymbol`) conservatively wants 15 bits before decoding any Huffman symbol (max code length),
so it returns `NeedInput` — correctly, since `index = 1 < stored.len() = 2`, i.e. there genuinely
is one more byte. The buggy driver treated that `NeedInput` as fatal and returned
`Err(UnexpectedEnd)` without ever admitting byte 1. This reproduces for every input, including the
empty one, which is why the failure set was contiguous from length 0 — `inflate` had never worked.

## Fix

```rust
// after (fixed)
match inflater.advance(&mut pending, input_complete)? {
    InflateOutcome::NeedInput => {}
    InflateOutcome::Wrote(byte) => { ... }
    InflateOutcome::Done => return Ok(output),
```

`NeedInput` now falls through to the top of the loop, which admits `stored[index]` (if any) and
calls `advance` again. `Inflater::advance` and `BitReader::ensure` were not changed — no changes to
the public API, the compressor, or `[features]` wiring.

### A second issue found, tried, and deliberately reverted

While extending coverage I found that a genuinely truncated/corrupted input can silently decode to
plausible-looking *wrong* bytes instead of erroring (e.g. `inflate(&compressed[..1], len)` returned
`Ok([112])` for a real multi-block sample). I traced this to `BitReader::ensure` returning `true`
(fabricated success) whenever `input_complete` is set and bits are still short — the remaining
"phantom" bits beyond real input read as zero. I tried making `ensure` strict (error the moment
bits run out with `input_complete` set), which broke 7 of the module's own already-green tests: a
legitimate final block can validly end with fewer than the conservative 15-bit lookahead reserve
(e.g. a 7-bit EOB code with nothing after it), and that zero-padding is exactly what makes such
streams decode correctly today. Distinguishing "legitimate short tail" from "truncated before EOB"
needs real-vs-phantom bit tracking through `Huffman::decode`, which is a redesign, not a decoder
fix, and is out of this ticket's scope — reverted, not shipped. `BitReader::ensure` and
`Inflater::advance` are otherwise byte-for-byte unchanged from before this ticket.

## `Inflater` (resumable path) — verified explicitly, not assumed

`Inflater`/`advance` were never the bug (only the one-shot wrapper was) — confirmed with a
byte-granular test (`streamed_inflate`, admits one byte per `advance` call, exactly the granularity
`DeflateRetainedCursor` in `📡️replication/⚙️codec/🦀️.rs` uses) that agrees with one-shot `inflate`
across empty/small, dynamic-Huffman, stored-block, and multi-block-spanning inputs. See
`stream_produces_the_same_bytes_as_one_shot_inflate`, `resumable_inflater_agrees_with_one_shot_across_block_types`
in the test output below.

## Before / after — `scope.rs` (lengths 0..=512)

Before (from the original defect report):
```
lens 0..=512: our inflate OK on 0, FAILED on 513
first 20 failing lengths: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]
sample errors: [(0,"UnexpectedEnd"),(1,"UnexpectedEnd"),(2,"UnexpectedEnd"),…]
failing set contiguous from 0? true
```

After (this fix, re-run in the standalone verification crate):
```
lens 0..=512: our inflate OK on 513, FAILED on 0
```

## Verification method

Workspace build is contended (twelve live sessions, `os-kernel` red from an unrelated refactor), so
everything below ran in the standalone crate
`/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/dfverify`
(`miniz_oxide`/`serde`/`serde_json` as `[dev-dependencies]` only), with the real module file copied
in verbatim before every run (`diff` confirmed byte-identical each time) — never edited only in the
scratch copy. `RUSTC_WRAPPER="" cargo test --release -- --nocapture`, no `CARGO_TARGET_DIR`
override, all in the foreground.

## Definition-of-done checklist

1. `scope.rs` — OK on 513, FAILED on 0. ✅ (above)
2. `parity.rs` — all three tests (ours→miniz, miniz→ours × 3 levels, ours→ours). ✅
3. Resumable `Inflater`/`advance` covered by a dedicated byte-granular test agreeing with one-shot
   `inflate`. ✅ — confirmed the resumable path was **not** broken; only the one-shot wrapper was.
4. Coverage beyond the original corpus: dynamic-Huffman (`BTYPE=10`, confirmed via bit inspection,
   not assumed), stored blocks (`BTYPE=00`), multi-block inputs, inputs >64 KiB spanning the 32 KiB
   window across block boundaries. ✅ — all added as tests in the module's own `mod tests`, plus
   scratch-crate reproductions.
5. The module's own in-repo test suite is real: `🧪️tests/🔣️deflate-corpus.json` already exists (11
   cases, seeds 1–11, lengths 0 through 70000 — the 70000 case already exceeds 64 KiB) and its
   fixture-driven test now passes. The earlier report that the fixture was "never generated" was
   stale by the time I checked — it was present and valid.

## Verbatim tails — every passing test, this fix, final run

Unit tests inside `🦀️.rs`'s own `mod tests` (run via the scratch crate with the real file copied in
verbatim, `diff` confirmed identical):

```
running 8 tests
test tests::reads_a_stored_block ... ok
test tests::stream_produces_the_same_bytes_as_one_shot_inflate ... ok
test tests::round_trips_repetitive_input_that_forces_long_matches ... ok
test tests::round_trips_empty_and_small_inputs ... ok
test tests::round_trips_pseudo_random_lcg_input_across_sizes ... ok
test tests::reads_miniz_oxide_dynamic_huffman_blocks ... ok
test tests::ours_inflates_miniz_oxide_output_and_vice_versa ... ok
test tests::round_trips_multi_block_input_spanning_the_window ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

`coverage.rs` (dynamic-Huffman BTYPE check, stored-block BTYPE check + hand-built stored block,
multi-block, >64 KiB window-spanning, resumable-vs-one-shot across all block types):

```
running 5 tests
level 0: btype(first block)=0 saw_stored_first_block=true
level 1: btype(first block)=0 saw_stored_first_block=true
test miniz_emits_stored_blocks_for_incompressible_input_and_we_read_it ... ok
test miniz_emits_dynamic_huffman_for_structured_input_and_we_read_it ... ok
test inputs_larger_than_window_round_trip_across_block_boundaries ... ok
resumable Inflater agrees with one-shot inflate across 5 cases incl. dynamic/stored/multi-block
test resumable_inflater_agrees_with_one_shot_across_block_types ... ok
test multi_block_input_round_trips ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

`empty.rs` (minimal repro, now succeeding):

```
running 2 tests
our deflate(empty) = [03, 00] (2 bytes)
miniz inflate of ours: Ok([])
miniz deflate(empty, lvl 1) = [03, 00]
   our inflate(theirs, 0) = Ok([])
miniz deflate(empty, lvl 6) = [03, 00]
   our inflate(theirs, 0) = Ok([])
miniz deflate(empty, lvl 9) = [03, 00]
   our inflate(theirs, 0) = Ok([])
our inflate(ours, 0) = Ok([])
our inflate(ours, 1) = Ok([])
test empty_input_diagnosis ... ok
non-empty parity OK across 33 cases (self, miniz->ours x3 levels, ours->miniz)
test nonempty_only_parity_still_holds ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

`oracle.rs` (the real in-repo `🧪️tests/🔣️deflate-corpus.json` fixture, read from its actual repo
path, not a copy):

```
running 1 test
real in-repo fixture: OK across 11 cases
test ours_inflates_miniz_oxide_output_and_vice_versa_real_fixture ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

`parity.rs`:

```
running 3 tests
our deflate -> miniz inflate: OK across 34 cases
test our_deflate_roundtrips_through_miniz_oxide ... ok
our deflate -> our inflate: OK across 34 cases
test our_own_roundtrip_is_exact ... ok
miniz deflate -> our inflate: OK across 34 cases x 3 levels
test our_inflate_reads_miniz_oxide_output ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

`scope.rs`:

```
running 1 test
lens 0..=512: our inflate OK on 513, FAILED on 0
test characterise_inflate_failures ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

20/20 tests passing across all six test binaries, run against the real module file verified
byte-identical to what is now committed in the working tree.

## Files touched

- `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` — the actual fix (one line of behavior change in
  `inflate()`), plus four new tests (`reads_miniz_oxide_dynamic_huffman_blocks`,
  `reads_a_stored_block`, `round_trips_multi_block_input_spanning_the_window`, and a
  `streamed_inflate` helper factored out of the existing streaming test) added to the module's own
  `mod tests`.
- No other repo file was touched by this fix. `🧪️tests/🔣️deflate-corpus.json` and the
  `Cargo.toml`s were already correct and untouched.

## Not done, and why

Truncation/corruption of persisted data can currently decode to wrong bytes without an error (see
"a second issue found, tried, and deliberately reverted" above). This is a pre-existing
characteristic of the zero-pad-at-EOF design, not a regression from this fix, and is not in this
ticket's Definition of Done. Flagging separately rather than attempting a risky redesign under an
urgent-fix ticket.
