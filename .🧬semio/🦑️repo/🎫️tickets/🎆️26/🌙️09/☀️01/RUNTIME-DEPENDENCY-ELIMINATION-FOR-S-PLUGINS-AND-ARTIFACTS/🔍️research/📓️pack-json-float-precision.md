# 🔬️ `pack::json` Float Precision Verification

## Verdict

`pack::json` is correct for `8322951083873004.0`. The text denotes an exactly representable
`f64` below `2^53`; `pack::json` and Rust's standard decimal parser both produce that exact
value. The observed one-ULP result comes from an oracle crate that enabled `serde_json` without
its `float_roundtrip` feature, not from `pack::json`.

Changing `pack::json` to match that value would introduce a real parser defect.

## Independent reproduction

An isolated crate under `🗑️generated/json-float-oracle` received a byte-identical copy of
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` (`cmp -s` exited `0`). Its minimal local
`protocol::value` shim exists only to compile that copied source; the JSON parser and its tests are
otherwise verbatim.

With `serde_json = "1"`, the requested command reproduced the failure:

```text
RUSTC_WRAPPER= CARGO_TARGET_DIR='../cargo-target-json-float' \
  cargo test differential_parse_matches_serde_json_on_arbitrary_values -- --nocapture

case 37: structural mismatch; text=8322951083873004.0
mine=Number(Float(8322951083873004.0))
theirs=Number(8322951083873005.0)
```

The bit-level probe establishes the correct result without relying on the differential assertion:

```text
literal       0x433d91ae0ed652ec
str::parse    0x433d91ae0ed652ec
pack::json    0x433d91ae0ed652ec
serde default 0x433d91ae0ed652ed
```

The first three are the exact integer `8_322_951_083_873_004`. `serde_json` default is one ULP
higher.

## Root cause

The relevant `serde_json` default path is its own `f64_from_parts`: it casts the decimal
significand `83229510838730040` to `f64`, then divides by `10`. The initial cast rounds; the
division rounds again. This double rounding produces the adjacent representable integer. The
crate provides the optional `float_roundtrip` feature specifically for its correctly rounded
decimal conversion path.

The real pack manifest already uses that configuration:

```toml
# 🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml
serde_json = { version = "1.0.140", features = ["float_roundtrip"] }
```

`pack::json` invokes Rust's `str::parse::<f64>()`, which returned the correctly rounded result;
there is no hand-written decimal-to-binary conversion to repair. The defect was the standalone
oracle manifest's missing feature, and the differential test itself is valid only under the
pack crate's established precise-oracle configuration.

## Durable regression coverage

`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs` now makes the exact oracle configuration
explicit in `Lexer::read_number`'s contract and adds two cases, each checked against the precise
third-party oracle:

- `8322951083873004.0`
- `83229510838730040e-1`

Both must parse to exactly `8_322_951_083_873_004.0`.

## Final verification

After changing the isolated crate to the same `float_roundtrip` dependency configuration as the
real pack crate, the requested differential test passed for all 3,000 generated documents:

```text
test json::tests::differential_parse_matches_serde_json_on_arbitrary_values ... ok
```

The complete copied component suite then passed:

```text
running 34 tests
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
