# Fixed Operation Registry Language-Neutral Oracle

The permanent owned runner was executed with:

`bun ./📜️script.ts verify interactivity tool-jobs --fixed-operation-fixture-only`

The temporary ticket-only third-party oracle loaded Ajv 2020 from the existing development dependency, validated `fixed-operation-registry-law.json` against `fixed-operation-registry.schema.json` in strict/all-errors mode, and rendered the schema-defined expected results in fixture order. Ajv is not imported by production code or the permanent verifier.

Both commands exited 0 on 2026-08-26. Their byte-identical canonical outputs are retained in:

- `🧪️sol-fixed-operation-registry-owned-fixture-2026-08-26.txt`
- `🧪️sol-fixed-operation-registry-ajv-oracle-2026-08-26.txt`

The permanent runner independently simulates exact fixed-slot hashing, byte/capacity admission, rejected owner handback, take, cancel, stale-generation scan, interrupted/repeated one-owner close, O(1) occupied/credit accounting, and compares every emitted row to the schema-first expected result.

A separate ticket-only Rust oracle in `🧪️fixed-operation-arrayvec-oracle/` executes the same step stream through the third-party `arrayvec::ArrayVec<Option<Entry>, 64>` fixed-capacity container. It parses the language-neutral fixture with ticket-only Serde/serde_json dependencies, independently applies the operation/generation hash and retained-credit laws, and rejects any output mismatch before printing. It ran offline with:

`cargo run --offline --quiet --manifest-path .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️fixed-operation-arrayvec-oracle/Cargo.toml -- 🧰️framework/🔨️modules/🧵️job/🧪️fixtures/fixed-operation-registry-law.json`

Its exit-0 output is retained in `🧪️sol-fixed-operation-registry-arrayvec-oracle-2026-08-26.txt` and is byte-identical to both the owned model and the Ajv-validated schema output. These dependencies remain confined to the ticket-owned oracle workspace.

The production Rust test includes `fixed-operation-registry-cases.rs`, generated exactly from the same JSON step stream by the root verifier. The verifier rejects stale generated Rust before reporting the packet green.
