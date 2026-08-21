# P9b — Owned Serialization

Status: **COMPLETE for the bounded P9b packet.** Phase 9 remains open because serialization
dependencies still enter the wider runtime graph through crates outside this packet.

## Outcome

The framework pack crate now owns the JSON value model, lexer, parser, and writer used by the
bounded schema crate. The schema crate now owns the JSON Schema subset its actual descriptors use.
Production declarations of `serde`, `serde_json`, `schemars`, and `jsonschema` were removed where
this packet proved them unnecessary. Third-party JSON and schema implementations remain only as
development-time differential oracles.

The abandoned `🧾️json` attempt was reconciled without touching shared work: its incomplete
`🦀️component.rs`, `component2.rs`, and `probe.txt` were removed after confirming that nothing
referenced them. The complete implementation lives only at
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`.

## Owned JSON implementation

The canonical module is wired through pack's Rust glue and domain facade. It provides:

- `Value`, `Number`, insertion-ordered `Object`, `Lexer`, `Token`, and structured `JsonError`.
- Whole-input `parse`/`parse_bytes` and compact `to_string` serialization.
- RFC 8259 number grammar, including leading-zero and non-finite overflow rejection.
- Distinct integer and floating wire forms, including `42` versus `42.0`.
- JSON escapes, UTF-16 surrogate pairs, raw UTF-8, raw-control rejection, and lone-surrogate
  rejection.
- A 128-level nesting ceiling.
- Last-value-wins duplicate keys while preserving the first insertion position.
- `null` serialization for non-finite in-memory floats, matching the reference writer.
- Pack-level aliases and an owned `content_hash(&[u8]) -> ContentHash` primitive backed by pack's
  existing BLAKE3 dependency.

The parser is intentionally whole-input because every proven consumer provides a small in-memory
schema leaf. Pretty printing, arbitrary-precision numbers, and speculative JSON features remain
out of scope.

### Differential evidence

The pack module contains 22 JSON-focused tests. Its deterministic SplitMix64 generator avoids a
new random-testing dependency. The differential corpus covers 6,013 documents:

- 3,000 owned-writer documents parsed by both implementations and structurally compared.
- 3,000 reference-writer documents parsed by the owned implementation and structurally compared.
- 13 curated documents compared byte-for-byte with the reference writer.

There are also 3,000 owned writer/parser round trips. The reference parser is enabled with its
`float_roundtrip` feature in dev dependencies; without it, one generated 16-digit float parsed one
ULP differently even though the owned spelling was valid. The reference feature gives the
structural differential test the exact-round-trip semantics it asserts.

Runtime console evidence from the focused differential command:

```text
[DEBUG] [differential] cross-parse: 3000 serde_json-written documents matched
[DEBUG] [differential] parse: 3000 generated documents matched serde_json
```

## Bounded schema conversion

`semio-framework-schema` now stores and parses schemas with `semio_framework_pack::json::Value`.
Its runtime no longer directly depends on `jsonschema`, `schemars`, `serde`, `serde_json`, or
`thiserror`.

The owned validator supports the keyword surface evidenced by repository schemas:

- `type`: `null`, `boolean`, `object`, `array`, `string`, `integer`, and `number`.
- `properties`, `required`, boolean `additionalProperties`, and `enum`.
- Annotation keys `$id`, `$schema`, `title`, `description`, and `x-*`.

Unsupported validation keywords are rejected at registration, so accepted schemas are never
silently under-validated. Recursive property checks, required fields, additional-property checks,
and enums are covered. JSON Schema numeric semantics are preserved: `1.0` satisfies `integer`,
and numeric enum values compare mathematically (`1` equals `1.0`) without erasing the owned JSON
value model's wire distinction.

The unused generic `SchemaCatalog::register<T: JsonSchema>` path was deleted after a repository-wide
caller check found none. Existing schema-first `register_json`/`load_json` paths remain.

The schema tests compare the owned validator with dev-only `jsonschema` across accepted and
rejected object, property, required, additional-property, scalar-type, enum, and mathematical
integer cases.

### Explicit schema versioning

`SchemaVersion(ContentHash)` and `schema_version(&str)` provide content-addressed schema drift
detection. Artifact and app descriptors expose their schema versions. Canonicalization recursively
sorts object keys before hashing, so whitespace and object-key order do not change a version while
semantic mutations do. Tests prove both invariance and drift detection. This is deliberately drift
detection, not a speculative migration executor.

## Dependency declaration changes

- Pack production: removed direct `serde` and `serde_json`; retained existing runtime `thiserror`
  because pack's established error types still use it outside this serialization packet.
- Pack development: added `serde_json` with `float_roundtrip` solely as the differential oracle.
- Schema production: removed direct `jsonschema`, `schemars`, `serde`, `serde_json`, and
  `thiserror`; added the internal `semio-framework-pack` dependency.
- Schema development: retained `jsonschema` and `serde_json` solely for differential tests.
- `Cargo.lock` reflects those direct dependency-list changes; no new package was introduced.

`cargo tree --edges normal` still shows serde-family crates transitively. For pack they enter via
`semio-framework-replication`; schema also depends on the broader OS-kernel graph. Removing those
transitive paths belongs to later Phase 9 packets, not this bounded pack/schema conversion.

## Files changed

- `Cargo.lock`
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️component.rs`
- `🧰️framework/🔨️modules/🎒️pack/🦀️component.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs`
- This evidence report.

The pack `📜️script.ts` correction swaps the stale `runCargo(this.repoRoot, args)` argument order
to the helper's current `runCargo(args, cwd)` contract for both build and test. The first Nx test
attempt exposed this pre-Cargo failure (`Array` supplied as the working directory); subsequent Nx
pack surfaces passed.

## Verification evidence

Commands were run from the repository root on 2026-08-21.

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/framework-pack-rs:test -- json::tests` | PASS, 22/22 focused JSON tests |
| `bun nx run @semio-tech/framework-pack-rs:test -- differential -- --nocapture` | PASS, 2/2 tests; 6,000 generated cross-parser documents logged |
| `bun nx run @semio-tech/framework-pack-rs:test` | PASS, 66/66 debug tests plus doc tests |
| `bun nx run @semio-tech/framework-pack-rs:test -- --release` | PASS, 66/66 release tests plus doc tests |
| `bun nx run @semio-tech/framework-pack-rs:build -- --target wasm32-unknown-unknown` | PASS |
| `bun nx run @semio-tech/framework-pack-rs:build -- --target wasm32-wasip2` | PASS |
| `cargo test -q -p semio-framework-schema` | PASS, 13/13 debug tests plus doc tests |
| `cargo test -q -p semio-framework-schema --release` | PASS, 13/13 release tests plus doc tests |
| `cargo check -q -p semio-framework-schema` | PASS; pre-existing warnings only |
| `cargo clippy --all-targets -p semio-framework-pack -p semio-framework-schema` | PASS (exit 0); pre-existing warnings remain |
| `cargo clippy -q --all-targets -p semio-framework-pack` | PASS (exit 0) after JSON lint cleanup; only pre-existing replication/async warnings |
| `bun ./📜️script.ts verify dependencies` | PASS, baseline 238, current 238, no new dependencies |
| scoped `git diff --check` | PASS |

Direct Cargo commands were used for schema because its existing `📜️script.ts`/Nx project exposes
only code-generation `generate` and `check` targets, while the Phase 9 verification matrix
explicitly requires Rust check/test/clippy for the crate. Pack validation used its existing Nx
surface.

### Encountered and resolved

- The initial Nx pack test did not reach Cargo because the pack router passed `runCargo` arguments
  in the obsolete order. The router was corrected and all relevant Nx targets then passed.
- The first full pack test had 64/65 passing: the reference parser rounded one generated float by
  one ULP. Enabling its exact `float_roundtrip` development feature resolved the oracle mismatch;
  the final suite has 66 tests after adding overflow rejection coverage.
- The first schema test had 11/12 passing because an existing assertion still expected retired
  state vocabulary `INFERRED`. It now asserts current `TRANSIENT`, consistent with the descriptor
  preamble and the existing retired-vocabulary test.
- The first clippy pass reported three new owned-JSON style findings. They were corrected; the
  final focused pack clippy exits zero without owned-JSON findings.

## Remaining blockers outside P9b

- `cargo fmt --check -p semio-framework-pack -p semio-framework-schema` is not green because both
  crates contain broad pre-existing formatting drift in untouched pack HTTP/format/IO/testkit and
  schema sections. The touched suggestions were applied manually. Running workspace formatting
  would rewrite shared concurrent work, so it was not done. Scoped `git diff --check` is green.
- The pack runtime graph still reaches serde/serde_json through `semio-framework-replication`.
- The schema graph still reaches serialization libraries through OS-kernel and pack transitive
  dependencies.
- OS-product pack/value serialization and other Phase 9 dependency-removal packets are outside this
  bounded framework pack/schema packet.
- Existing warnings include replication's unknown `typegen` cfg and unrelated clippy findings;
  none originated in the owned JSON module after cleanup.

These items prevent claiming Phase 9 completion, but they do not block the bounded P9b deliverable.
