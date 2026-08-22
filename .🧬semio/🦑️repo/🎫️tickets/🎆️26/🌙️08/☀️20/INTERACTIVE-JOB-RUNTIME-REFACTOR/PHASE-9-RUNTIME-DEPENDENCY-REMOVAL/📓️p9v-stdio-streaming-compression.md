# P9v Stdio Streaming Compression

## Scope

This packet owns stdio's consumed RFC1950/RFC1951 and ZIP compression boundary. It preserves the existing synchronous artifact-codec entry points while moving the owned fixed-Huffman encoder behind a persistent `InteractiveJob` and retiring comparison/benchmark dependencies only after retaining direct golden-byte gates.

## Consumed behavior census

- Generic raw DEFLATE and RFC1950 zlib framing are owned implementations used by stdio DEFLATE, PNG, PDF, and ZIP codecs.
- ZIP parsing/materialization is already owned; method 0 and method 8 are the only consumed member methods.
- Native `libz-sys` remains load-bearing for tuned Office raw members and Illustrator's partial-flush RFC1950 stream.
- `flate2` remains load-bearing for the wasm tuned fallback and the existing level-nine deterministic PDF path.
- `miniz_oxide`, `zlib-rs`, and `zopfli` were used only by non-asserting exploratory backend matrices. Their comparison work is superseded by direct fixture equality against the selected production policies.
- `criterion` was used only by `benches/brep_kernel.rs`.

## Implementation

- `DeflateEncodeJob` persists input, bit cursor/output, the deterministic hash-chain index, input cursor, lazy pending match, and checkpoint schedule.
- Each transition is bounded to one longest-match decision plus at most one 258-byte match insertion. `StepContext` cancellation is checked before work and after every transition; fuel and deadline yielding are honored after every transition.
- Checkpoints use a versioned, explicit little-endian owned format and restore without replaying completed match search.
- `deflate_raw` is now the non-interactive batch adapter over the same state machine, so interactive and batch paths cannot diverge.
- Tests cover byte identity across fuel sizes, checkpoint/restore, pre-cancelled state preservation, adversarial step timing, raw round trips, exact embedded-OLE members, and the Illustrator partial-flush fixture. The linked debug harness measured a 1,939 µs maximum adversarial step.
- The Criterion BREP suite is now an owned fixed-warmup/fixed-sample executable using `std::hint::black_box` and `Instant`; the existing Nx/Cargo bench route is unchanged.

## Dependency ratchet

Removed direct stdio rows and all stdio source references:

- `criterion`
- `miniz_oxide`
- `zlib-rs`
- `zopfli`

Retained deliberately:

- `libz-sys`: exact Office/Illustrator bytes are selected production behavior, not merely round-trip behavior.
- `flate2`: wasm tuned fallback and deterministic PDF level-nine materialization remain consumed.

No direct `zip` dependency existed in stdio. `miniz_oxide` and `zopfli` remain transitive through retained `flate2`/framework ZIP consumers; the stdio-owned source and manifest census is zero.

## Gates

- Native: `cargo check -p semio-s-plugin-stdio --lib --message-format=json` — exit 0, zero diagnostics (`📝️p9v-stdio-compression-native-final.json`).
- Release: same command with `--release` — exit 0, zero diagnostics (`📝️p9v-stdio-compression-release-final.json`).
- Wasm: same command with `--target wasm32-unknown-unknown` — exit 0, zero diagnostics (`📝️p9v-stdio-compression-wasm-final.json`).
- Linked production-code consumer harness — exit 0 (`📝️p9v-stdio-compression-harness3.txt`): fuel sizes 1/2/7/64/1024 are byte-identical; checkpoint/restore is byte-identical; pre-cancellation preserves state; all three embedded OLE members match their PPTX golden streams; the 3,362-byte Illustrator partial-flush stream matches its PDF golden; maximum adversarial step is 1,939 µs.
- Owned BREP benchmark: `cargo check -p semio-s-plugin-stdio --bench brep_kernel` — exit 0 (`📝️p9v-stdio-owned-bench-check.txt`).
- Rustfmt and scoped `git diff --check` — exit 0 with empty logs.
- Dependency graph: `📝️p9v-stdio-compression-tree.txt`; direct source/manifest retired-dependency census is empty.
- Stdio's monolithic test target remains independently blocked before execution: the completed structured `cargo check -p semio-s-plugin-stdio --tests --message-format=json` reports exactly 898 primary test-only diagnostics across 241 file/code groups, while this RFC1950 IO file has zero. Exact ownership is in `📝️p9v-stdio-lib-tests-final-errors.tsv`; no errors were suppressed or masked.
