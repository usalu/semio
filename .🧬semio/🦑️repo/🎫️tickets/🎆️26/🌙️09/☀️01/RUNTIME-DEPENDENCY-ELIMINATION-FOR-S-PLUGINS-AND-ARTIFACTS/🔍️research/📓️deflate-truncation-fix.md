# DEFLATE Truncation Fix

## Result

`BitReader::ensure` now fails with `DeflateError::UnexpectedEnd` when a fixed-width field needs bytes that cannot arrive. Huffman decoding is the single exception: it may begin with fewer than the conservative 15-bit reserve at final input, but `Huffman::decode` now consumes only real buffered bits through `BitReader::take_bit`. It returns `UnexpectedEnd` instead of treating exhausted input as zero bits.

This preserves valid short endings while rejecting a stream that reaches the end before a complete Huffman symbol. The rule is used for the code-length, literal/length, and distance Huffman tables; all length, distance, stored-block, and header fields still require their full real-bit width.

## Regression Coverage

- `rejects_truncation_before_the_final_huffman_symbol` compresses the reported repeated-text sample, keeps one byte, and asserts `UnexpectedEnd` from both `inflate` and byte-granular `Inflater::advance` driving.
- `accepts_a_short_final_huffman_tail` consumes the language-neutral `🧪️tests/🔣️deflate-tail-cases.json` vector `[0x03, 0x00]`: a final fixed-Huffman EOB has only 13 available bits after its three-bit block header. The first-party one-shot and resumable decoders agree with the `miniz_oxide` oracle on its empty output.
- Existing dynamic-Huffman, stored-block, corpus-oracle, and large-input tests all remain green. The dynamic-Huffman case verifies that the checked-tail rule remains compatible with dynamic tables.

## Pack Integrity Context

Pack standard verification validates each segment's CRC-32C over the stored bytes before decompression. Full verification additionally checks raw chunk/document hashes with BLAKE3. The retained cursor also enforces declared decompressed length. These outer checks provide important persistence integrity, but the raw DEFLATE decoder now rejects missing bits independently so callers using `Inflater` directly cannot silently obtain a successful result from fabricated zero bits.

## Verification

```text
bun x cargo fmt --manifest-path 🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml --check
exit 0

CARGO_TARGET_DIR=<isolated temporary target> bun x cargo test --manifest-path 🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml
10 passed; 0 failed; doc-tests: 0 passed; 0 failed
```
