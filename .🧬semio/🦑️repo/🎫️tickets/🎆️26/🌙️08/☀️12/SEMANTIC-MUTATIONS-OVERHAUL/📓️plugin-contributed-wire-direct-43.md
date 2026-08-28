# Plugin Contributed Wire Direct Fixture

## Direct Owner

The cfg(test) contributed-mutation wire fixture now mounts the direct owner at:

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️contributed-mutation-wire`

Its transparent aggregate is `WireTestMutation`, with the sole direct `AddValue` leaf at `🧬️mutations/➕️add-value`. The leaf owns the mandatory descriptor, strict payload schema, semantic descriptor, direct diff/inverse, and composite plan. The aggregate owns only roster and the existing serde-JSON `OpBinary` round trip. It has no text opcode or numeric binary tag because this fixture has only the existing serde binary envelope.

`WireTestDiff` now stores ordered `Vec<i32>` deltas and applies them with `checked_add`; `absorb` appends. `AddValue(i32::MIN)` records `[1, i32::MAX]`, which restores when Store reverses stored inverse vectors. The local plan remains exactly one `AddValue`, with semantic kind `add-value` and label `Add <delta> to value`.

The Plugin cfg(test) module imports these owner types, calls `mutation::<WireTestSnapshot, WireTestMutation, AddValue>`, retains the mismatched-kind and unregistered-id tests, and decodes echoed owner bytes through `WireTestMutation`. The old `WireTestOp` and `WireTestMutationKind` declarations are removed; no alias or default descriptor remains.

## Schemas And Neutral Evidence

The direct owner includes:

- `🧬️mutations/➕️add-value/🔣️.json` and `🧬️schema/🔣️.json` for its mandatory descriptor and strict `i32` payload;
- `🧬️mutations/🔣️.json` for the closed serde aggregate envelope;
- `🧬️diff/🔣️.json` for ordered bounded deltas; and
- `🔣️cases.json` for valid/invalid envelopes, ordered overflow, cancellation, and the minimum inverse.

Ticket controller: `🧪️plugin-contributed-wire-43/📜️script.ts` uses Ajv 2020 plus jsonc-parser over actual files, validates the full descriptor through the authoritative descriptor schema, checks source provenance/mount/consumer replacement, and records SHA-256 fingerprints.

The retained meaningful pre-source red is `🧪️plugin-contributed-wire-43/🧫️run-csB3g2/🔣️result.json`: 55 assertions and 10 failures for the absent three direct Rust files and their required source contracts. The final Bun/Nx run is `🧪️plugin-contributed-wire-43/🧫️run-eNbeDb/🔣️result.json`: 61 assertions and zero failures.

Both runs explicitly record `nativeRustExecuted: false`. No Cargo, rustc, Plugin interaction/lifecycle, runtime wire implementation, shared DSL, launch, seed, or ledger source was changed. Native compilation and execution remain root-owned.
