# P9z Stdio Owned RFC 1950/RFC 1951

## Scope

This packet replaces stdio's final production `libz-sys` and `flate2` compression paths with owned deterministic RFC 1950/RFC 1951 implementations. It preserves the selected byte streams used by Office embedded members, Adobe Illustrator partial-flush PDF streams, and deterministic level-nine PDF output. Both the fixed-Huffman `DeflateEncodeJob` and tuned `TunedDeflateEncodeJob` are persistent, checkpointable, cancellation-aware boundaries; every synchronous encoder is a batch adapter over the same state machine.

## Implementation

- Added an owned classic zlib-compatible dynamic-Huffman encoder with deterministic hash-chain LZ77, lazy matching, stored/fixed/dynamic block selection, and raw/sync/partial finalization policies.
- Added an owned miniz-compatible level-nine encoder, including its match search, block boundary policy, Huffman construction, and RFC 1950 framing.
- Added `TunedDeflateEncodeJob` constructors for Office, Office high-search, Office compact high-search, Illustrator, and level-nine policies. Each step advances one bounded LZ/Huffman transition under `StepContext` fuel and preserves the full encoder state in its checkpoint.
- Routed tuned Office raw DEFLATE, Illustrator zlib partial flush, and deterministic PDF zlib materialization through those owned engines.
- Kept decompression in the existing owned RFC 1950/RFC 1951 decoder.
- Removed stdio's direct `libz-sys` and `flate2` manifest rows only after exact golden and differential parity was demonstrated. The retained `flate2` row in the ticket harness is test-only and is the reference implementation for the level-nine differential gate.

## Exact byte evidence

- `📝️p9z-owned-dynamic-iteration11.txt`: all three embedded Office members are byte-identical to the committed PPTX fixture: 768,040/768,040, 770,138/770,138, and 770,618/770,618 bytes.
- `📝️p9z-miniz-parity-iteration17.txt`: the 3,362-byte Illustrator stream is byte-identical; deterministic PDF level-nine output is byte-identical to the retained test-only `flate2`/miniz reference at 3,222 bytes, with identical token traces.
- `📝️p9z-linked-golden-resumability-watchdog-final-2.txt`: final-state linked production-code harness covers fixed fuel sizes 1/2/7/64/1024; tuned fuel sizes 1/7/64/1024; checkpoint/restore and pre-cancelled state preservation for both jobs; all three Office members; Illustrator; level-nine differential parity; empty/256-byte/64-KiB incompressible round trips; and both adversarial step watchdogs. The observed maxima were 1,909 µs fixed and 1,338 µs tuned, below the 8 ms ceiling.

## Dependency and warning ratchets

- Direct stdio manifest and executable-source uses of `flate2`, `libz_sys`, and `libz-sys`: zero. `📝️p9z-final-dependency-census.txt` contains one non-executable documentation sentence stating that the entropy codec has no `flate2` dependency.
- The compiler initially reported 17 `unused implementer of Future` warnings (`📝️p9z-unused-futures.tsv`). Schema registration calls are now deliberately resolved, the pure STEP/IFC preamble contract is synchronous, MP4 registration has the correct sync/async split, and the plugin bundle installer is synchronous. `📝️p9z-warning-free-debug-2.json` records exit 0, zero errors, and zero unused-Future diagnostics.

## Dependency-safe component routing

Stdio's manifest now emits an `rlib` by default, so downstream plugins do not component-link an intermediary stdio artifact. Stdio's own describe route explicitly asks Cargo for a root `cdylib` with `cargo rustc --lib --crate-type cdylib --target wasm32-wasip2`.

The shared SDK export seam was corrected for cross-crate generation: nested component paths are explicit, the WIT export macro is public with a stable bindings path, intermediary custom-section link helpers are disabled, and the link anchor is synchronous. `📝️p9z-stdio-root-cdylib-wasip2-final.txt` records the explicit stdio root component build at exit 0. The note-plugin owner independently confirmed that the rlib dependency route passes both former stdio and framework-OS missing-`poll` failures and reaches note-owned source diagnostics.

## Gates

- Linked golden/differential/resumability/watchdog harness: `cargo run --manifest-path <ticket>/🧪️p9v-stdio-compression-harness/Cargo.toml --bin p9v-stdio-compression-harness`, `📝️p9z-linked-golden-resumability-watchdog-final-2.txt`, exit 0 and `compression_harness=pass`.
- Native debug stdio library check: `cargo check -p semio-s-plugin-stdio --lib --message-format=json`, `📝️p9z-final-native-debug.json`, exit 0, zero errors, zero unused-Future diagnostics, and zero private-interface diagnostics.
- Native release stdio library check: `cargo check -p semio-s-plugin-stdio --lib --release --message-format=json`, `📝️p9z-final-native-release.json`, exit 0, zero errors and zero unused-Future diagnostics.
- Dependency-mode wasm32-unknown-unknown rlib check: `cargo check -p semio-s-plugin-stdio --lib --target wasm32-unknown-unknown --no-default-features --message-format=json`, `📝️p9z-final-wasm32-unknown.json`, exit 0 and zero errors.
- Dependency-mode wasm32-wasip2 rlib check: `cargo check -p semio-s-plugin-stdio --lib --target wasm32-wasip2 --no-default-features --message-format=json`, `📝️p9z-final-wasm32-wasip2.json`, exit 0 and zero errors.
- Explicit stdio root wasm32-wasip2 component: `📝️p9z-stdio-root-cdylib-wasip2-final.txt`, exit 0.
- Formatting and scoped diff checks: `cargo fmt -p semio-s-plugin-stdio --check` and the stdio/Puzzle-consumer `git diff --check` both exited 0; their logs are `📝️p9z-final-cargo-fmt-check.txt` and `📝️p9z-final-diff-check.txt`.

## External diagnostics

The broad stdio lib-test target still has the previously recorded test-only migration wall outside this compression packet. A Puzzle no-run boundary additionally exposed three Puzzle testkit modules missing explicit `EditorApp` imports and four Puzzle assertions using the removed `MutationOutcome::apply` call shape; those exact consumer sites were migrated to the exported adapter and inherent `apply_to` API, respectively. The prepared-render owner's authoritative `📝️r22-puzzle-test-no-run-2-errors.tsv` rerun confirms all 14 diagnostics are cleared and attributes all 125 remaining diagnostics to Puzzle-local code. No diagnostic was suppressed or converted into a weaker byte gate.
