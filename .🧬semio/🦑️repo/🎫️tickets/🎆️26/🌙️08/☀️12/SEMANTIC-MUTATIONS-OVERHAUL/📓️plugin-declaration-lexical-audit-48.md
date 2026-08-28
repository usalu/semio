# Plugin Declaration Raw Serde Lexical Audit 48

## Scope

Read-only inspection only. The Plugin declaration fixture/controller/vectors were left unchanged during the released Plugin source freeze. No Cargo, rustc, or native test was run.

`Cargo.lock:6830-6865` pins `serde_core 1.0.228` and `serde_json 1.0.149`. The local registry source inspected was:

- `/Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.149/src/de.rs`
- `/Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs`
- `/Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_derive-1.0.229/src/de/{struct_.rs,enum_externally.rs}`

## Confirmed correction

The current raw vector/controller/report assertion that `-0` is accepted as `i32(0)` is false.

`serde_json 1.0.149` parses a nonpositive integral token in `parse_number` at `src/de.rs:509-523`. The explicit `-0` branch has `neg >= 0` and produces `ParserNumber::F64(-(significand as f64))`, not `ParserNumber::I64(0)`. `ParserNumber::visit` at `src/de.rs:112-123` sends this form to `Visitor::visit_f64`.

The `i32` request uses serde_json's `deserialize_number` macro (`src/de.rs:1341-1503`, including `deserialize_i32` at line 1502). `serde_core`'s default `Visitor::visit_f64` returns `Error::invalid_type(Unexpected::Float(..), ...)` at `src/de/mod.rs:1491-1502`. Therefore both `OpText::parse_op` and UTF-8 JSON `OpBinary::decode_op` must reject the current `-0` raw vector; no coercion is warranted.

## Other lexical assumptions inspected

- `1e0` is also `ParserNumber::F64` via `parse_number` at `serde_json/src/de.rs:509-512`, so rejection for `i32` is correctly expected.
- Plain integral tokens in range reach `I64` or `U64`; out-of-range i32 tokens are rejected by the `i32` visitor rather than coerced. The current integer range check remains directionally correct once it excludes `-0`.
- `serde_json::from_str` calls `from_trait`, and `from_trait` calls `Deserializer::end` (`src/de.rs:2497-2508`); `end` rejects non-whitespace residual input as `TrailingCharacters` (`src/de.rs:143-150`). Thus the trailing-second-JSON vector correctly expects rejection.
- A direct struct duplicate is explicitly rejected by generated serde derive code: `serde_derive/src/de/struct_.rs:266-272` returns `duplicate_field` after the first field. This supports the repeated nested `value` expectation.
- The external enum duplicate remains a native-runtime assertion to retain after the freeze. The external-tag derive source uses one enum access (`serde_derive/src/de/enum_externally.rs:55-98`), while `from_trait(...).end()` must consume the whole input. The raw vector and native test are appropriate, but no direct runtime outcome is claimed in this audit.

## Post-freeze correction plan

1. Change the `i32 negative zero` raw vector expectation from accepted/decoded zero to rejection.
2. Amend the controller's lexical model so `-0` is excluded before its `BigInt` range test; it must not derive acceptance from JavaScript numeric equivalence.
3. Keep the existing native text/binary vector assertion, but remove its accepted decoded-value branch for `-0`.
4. Correct the earlier follow-up report's `-0` statement and retain the original source-only GREEN as superseded evidence, not a native pass.
5. After runtime releases the Plugin freeze, rerun the Bun/Nx controller, then execute the mounted native tests to establish the two codec boundaries independently.
