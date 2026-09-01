# 🚨️ Defect — first-party `inflate` is completely non-functional, on the DEFAULT production path

Found by central verification 2026-09-01. The authoring agent reported this module as
**WRITTEN BUT UNVERIFIED** (no test run ever completed under machine saturation). It does not work.

## Severity: high, and it is NOT behind an off-by-default flag

```
🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml:   default = ["deflate"]
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:    default = ["deflate"]
```

`semio-framework-pack` and `semio-framework-os-kernel` both enable `deflate` BY DEFAULT, chaining
to `semio-framework-replication/deflate` → `dep:semio-framework-deflate`. Every s plugin depends on
`os-kernel`. So the broken decompressor sits on the default build of essentially everything, and it
replaced `miniz_oxide`, which worked.

## What is broken

`🧰️framework/🔨️modules/🗜️deflate/🦀️.rs`:

- ✅ `deflate()` (compressor) is **correct**. Its output is byte-identical to `miniz_oxide` for the
  empty input (`[03, 00]`) and round-trips through `miniz_oxide::inflate::decompress_to_vec`
  for all 34 corpus cases.
- ❌ `inflate()` (decompressor) is **totally broken**. Across input lengths `0..=512` — 513 cases,
  fixed-Huffman blocks produced by our own correct compressor — it succeeded **0 times** and failed
  **513 times**, every one with `DeflateError::UnexpectedEnd`. The failing set is contiguous from
  length 0, i.e. it has never worked for any input.

```
lens 0..=512: our inflate OK on 0, FAILED on 513
first 20 failing lengths: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]
sample errors: [(0,"UnexpectedEnd"),(1,"UnexpectedEnd"),(2,"UnexpectedEnd"),…]
failing set contiguous from 0? true
```

Minimal reproduction — the canonical empty fixed-Huffman block, which every DEFLATE decoder must
accept:

```
our deflate(empty)        = [03, 00]      (byte-identical to miniz at levels 1/6/9)
miniz inflate of ours     = Ok([])        ← our encoder is fine
our inflate([03,00], 0)   = Err(UnexpectedEnd)
our inflate([03,00], 1)   = Err(UnexpectedEnd)
```

`inflate` also cannot read `miniz_oxide`'s output at levels 1, 6 or 9. Since `🎒️pack` containers
were previously written with `miniz_oxide`, **existing persisted pack data would fail to
decompress.**

`Inflater` (the resumable byte-granular path used by `DeflateRetainedCursor` in
`📡️replication/⚙️codec/🦀️.rs`) shares the same bit-reader and is very likely broken the same way —
verify it explicitly rather than assuming.

## How this was found, and how to verify a fix

The workspace was too contended to build, so the module was copied verbatim into a standalone crate
outside the repo with `miniz_oxide = "0.8"` as the only dependency. Test sources are preserved in
`🔬️verification-deflate/` beside this file — `parity.rs` (three-way round-trip), `empty.rs`
(minimal repro + diagnosis), `scope.rs` (the 0..=512 sweep). Copy
`🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` to `src/lib.rs`, strip its `#[cfg(test)]` module (it needs
serde and a fixture file), and run. This takes seconds and needs no workspace lock.

## Root-cause hint

Since the encoder is correct and the decoder fails at length 0 with `UnexpectedEnd`, suspect the
bit-reader's end-of-input handling and the `BFINAL`/`BTYPE` header read, not the Huffman tables:
a decoder that mis-tracks how many bits remain will report `UnexpectedEnd` before ever decoding a
symbol. The empty fixed-Huffman block `[03, 00]` is BFINAL=1, BTYPE=01, then the 7-bit
end-of-block symbol 256 and padding — decoding it exercises only the header and the EOB symbol.

## The process lesson

A first-party replacement for a working third-party crate was wired onto the DEFAULT path of the
whole framework without a single passing test. The differential oracle test was written but never
executed. `miniz_oxide` is correctly retained as a `[dev-dependencies]` oracle — running it once
would have caught this immediately.

---

# ✅️ RESOLVED — and independently re-verified by the coordinating session

## Root cause (one line, in the driver — not the decoder)

The one-shot `inflate()` driver treated **any** `InflateOutcome::NeedInput` from
`Inflater::advance` as fatal (`return Err(UnexpectedEnd)`), even when input bytes remained to be
admitted. For `[0x03, 0x00]`, byte 0 supplies only the 3-bit block header; the next phase needs 15
more bits and legitimately asks for byte 1 — the driver bailed instead of feeding it. Fix:
`NeedInput => {}`, falling through so the loop admits the next byte.

`Inflater`, the bit reader, the Huffman tables and `deflate()` were **never broken**. The resumable
byte-granular `Inflater`/`advance` path that production's `DeflateRetainedCursor` actually uses was
verified explicitly and was correct all along — so the blast radius was narrower than first feared:
the streaming path in `📡️replication` was fine; only the one-shot entry point failed.

## Verification

Fixing agent: `scope.rs` before **OK 0 / FAILED 513** → after **OK 513 / FAILED 0**; `parity.rs`
3/3. It added four tests to the module's own suite — dynamic-Huffman (BTYPE=10 *confirmed by bit
inspection*, not assumed), a hand-built stored block (BTYPE=00), a multi-block input >96 KiB
exercising the 32 KiB window across block boundaries, and a byte-granular resumable-vs-one-shot
agreement test. It also found the in-repo fixture `🧪️tests/🔣️deflate-corpus.json` already existed
(11 cases, up to 70000 bytes) and its oracle test now passes — the earlier "never generated
fixture" claim was stale.

Coordinator re-verification, from a fresh byte-identical copy of the repo file:

```
our deflate -> our inflate: OK across 34 cases
miniz deflate -> our inflate: OK across 34 cases x 3 levels
our deflate -> miniz inflate: OK across 34 cases
test result: ok. 3 passed; 0 failed

lens 0..=512: our inflate OK on 513, FAILED on 0
test result: ok. 1 passed; 0 failed
```

## Good judgement worth recording

The fixing agent found a second, adjacent gap — truncated input can silently decode to wrong bytes
rather than erroring — attempted a fix, saw it regress 7 legitimate-stream tests, and **reverted it
and filed a follow-up** rather than push a rushed redesign into an urgent fix. That is the right
call: the shipped defect is repaired with a minimal, well-understood change, and the separate
hardening question is tracked instead of being bundled in.
