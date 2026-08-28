# Store Fixture Canonical Text Codecs

## Scope

Only the direct Store fixture text-codec contract changed. GIS, Store history, lifecycle, shared derives, launch, seed, and ledger sources were not edited.

The affected direct leaves now advertise and parse only their semantic kind:

| Family | Variant | Canonical opcode | Rejected former spelling | Binary tag |
| --- | --- | --- | --- | --- |
| demo | `AddN` | `add-n` | `bump-n` | 2 |
| severity | `SetN` | `set-n` | `clean-n` | 0 |
| severity | `SetWarningN` | `set-warning-n` | `warn-n` | 1 |
| severity | `SetErrorN` | `set-error-n` | `error-n` | 2 |
| severity | `SetFatalN` | `set-fatal-n` | `fatal-n` | 3 |
| validated | `SetN` | `set-n` | `set-n-validated` | 0 |
| validated | `RestoreN` | `restore-n` | `restore-n-validated` | 1 |

The six matching Rust leaves and descriptors are under `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/{🚦️severity,🛂️validated}/🧬️mutations`. Each leaf now has regions and an exact `dsl` keyword equal to its descriptor `textOpcode` and `semanticKind`; no alias was retained.

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️fixtures/🔣️mutations.json` supplies all seven rows above. The main Store fixture test now uses the generic `assert_fixture_text_codecs<Op>` helper for Demo, Severity, and Validated aggregates, checking canonical parse/print, exact rejection, and existing `u32` tags without changing behavior tests.

## Evidence

Pre-change source-only red executed with:

```sh
bun ./node_modules/nx/bin/nx.js exec --projects=workspace --skipNxCache -- bun '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-fixtures-39/📜️script.ts'
```

Retained result: `🧪️store-fixtures-39/🧫️run-TQ6H9K/🔣️result.json`. It recorded 442 assertions and the six descriptor canonicality failures plus the missing seven-family roster assertion.

The same command after the fix retained `🧪️store-fixtures-39/🧫️run-LtzHSA/🔣️result.json`: 473 assertions, zero failures, and full SHA-256 fingerprints of the controller and every inspected actual file. The controller requires every advertised Store text opcode to equal both the descriptor semantic kind and the leaf key, asserts all seven vector fields, confirms the exact rejected former spelling, verifies the generic native test helper calls all three aggregates, and self-hashes/re-reads its inputs.

This is a Bun/Nx schema-and-source gate only. `nativeRustExecuted` is explicitly `false` in both retained results; no Cargo, rustc, or native test was run. The next native gate remains root-owned.
