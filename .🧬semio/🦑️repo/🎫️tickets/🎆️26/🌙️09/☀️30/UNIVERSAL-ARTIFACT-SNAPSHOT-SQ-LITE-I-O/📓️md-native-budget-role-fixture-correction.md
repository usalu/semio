# MD Native Ownership Budget Fixture Correction

Root’s authentic MD25 receipt `🗑️generated/root-authentic-md25-paid-native-bridge-canonical-typed-current.log` reached assertions: Nextest `c9ee8e64-7f89-42c9-9635-425f8ee31666`, 25 run, 24 pass, 1 fail, 47 outside, 0.541 s Native / 16.5 s Nx. The sole failure was `sqlite_snapshot_md_whole_native_control_keeps_deep_fields_and_interior_cancellation`, line 122:197: its native refusal fixture still supplied semantic `max_value_bytes=4096` after the mounted native bridge began using the independent `max_allocation_bytes` allowance. This fixture correction changes no production code.

Low’s complete artifact consumer census found exactly one Native consumer of controlled.json’s maxValueBytes property, plus its include. No Source/schema consumer existed. The independent SQL Source `maxValueBytes:0` refusal remains semantic and unchanged.

## Schema-First Source Contract

The new closed draft-07 schema requires the authored native ownership budget `maxAllocationBytes=4096` and explicit `allocationLimitRole=nativeOwnedBacking`. It retains all existing exact witness values. A registered Source law first rejected the old fixture, then independently checked malformed old-budget, wrong-role, and unknown-property variants with Ajv. Bun SQLite measures the actual schema owner using `length(CAST(schema AS BLOB))`; Buffer UTF-8 byte length independently agrees. The 270000-byte literal schema exceeds the 4096 ownership allowance while default semantic SQL projection/reconstruction succeeds. This Source law validates the neutral budget-role contract and SQL literal facts; it does not execute the Rust Native decoder.

Registered command:

```sh
SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_WORKSPACE_DATA_DIRECTORY=<ticket-generated-unique-directory> bun nx run @semio-tech/stdio-md-rs:test-snapshot-sqlite-source --skip-nx-cache
```

Before fixture/caller correction, `🗑️generated/md-native-budget-role-source-red.log`: 13 laws, 12 pass / 1 fail, 91 assertions, 2.45 s Bun / 10.8 s critical path / 11.5 s Nx. The new fixture contract failed; all 12 prior Source laws passed. The registered wrapper’s owner-command name includes cargo infrastructure, but its selected Source command skips Native and did not run Cargo.

## Exact Changes

All paths below are beneath `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot`.

- `🧫️fixtures/🪶️sqlite/🛬️controlled/🧬️schema/🔣️.json`: new handcrafted closed ownership-role contract.
- `🧫️fixtures/🪶️sqlite/🛬️controlled.json`: rename the single 4096 property and add its explicit ownership role.
- `🧪️tests/🪶️sqlite/🟦️.ts`: one registered independent Source law.
- `🧪️tests/🪶️sqlite/🦀️.rs`: exactly one Native control field/property pair changes from max_value_bytes/maxValueBytes to max_allocation_bytes/maxAllocationBytes. Existing is_err assertions, payloads, and callbacks remain intact.

Preserved the actual controlled depth600, maxRows10, cancelAfter256, schemaRepeat10000, orderedStart4294967295, the independent cohort partialDepth2048 and lateSqlCancellationDepth8192, denied8192/4096 retirement witnesses, and all prior Native25 laws. No cold/depth guard implementation, native paid bridge, retirement logic, semantic preflight, default limit, or provider opt-in changed. Root’s four canonical cancellation Source assertions are preserved.

Parser-only `rustfmt --edition 2021 --emit stdout` returned zero for the changed Native test; receipt `🗑️generated/md-native-budget-role-parser-current.log`. This is syntax validation, not Rust type or runtime verification. Root remains the sole Cargo lane; fresh actual MD25 Native verification is pending.

After the handcrafted fixture/caller correction, the same registered route returned GREEN: `🗑️generated/md-native-budget-role-source-green.log`, 13/13 laws, 98 assertions, 2.58 s Bun / 8.6 s critical path / 9.1 s Nx, exit zero. The Native suite retains its original 25-law selector/count; only Root can provide the fresh Native receipt.
