# Retained Deflate Physical Ceiling Origin Repair

## Authentic native failure

Root's actual JSON20 runs `018bafd3-94d5-4434-b503-69eba14f0fa1` and diagnostic `47c94fde-4fc3-4e3f-b820-1730466602b9` each executed twenty selected laws: nineteen passed, one failed, ninety outside the selection. The diagnostic completed in 3.531 seconds. The existing assertion `sqlite_snapshot_json_native_decode_owner_restores_typed_state_with_interior_control` at JSON Native test line 173 received Binary `ValueError { kind: WorkLimit, message: "limit exceeded: retained deflate physical ceiling" }` while requiring `OwnershipLimit`. The JSON assertion and its limit witness are unchanged.

Diagnostic log: `🗑️generated/root-authentic-json20-controlled-owned-input-refusal-context.log`. Initial corrected-budget run: `🗑️generated/root-authentic-json20-paid-input-corrected-controlled-owned-budget.log`.

## Exact source authority

The mounted controlled Pack payload reader supplies `DeflateRetainedCursor::try_new` with the remaining Native ownership allowance: the smaller of the Native ceiling and Pack total-allocation ceiling, minus already admitted Native bytes. The cursor calls `Inflater::try_new_retained(maximum_history_bytes, maximum_allocation_bytes)`.

The actual retained history constructor fails with its intrinsic `OutputLimitExceeded` only when the history target exceeds that allocation allowance or the allowance exceeds the signed pointer address space. This constructor performs no allocation and does not read compressed syntax. The target is the semantic history limit capped at the 32768-byte retained window. These actual preconditions establish an owned physical-admission refusal at this producer.

Only the originating `map_err` now returns `PackError::ValueRefusal(ValueError::new(OwnershipLimit, "retained deflate physical ceiling"))`. The preceding declared raw-length comparison, history address-space conversion, output-work overflow and global `LimitExceeded -> WorkLimit` mapping remain unchanged. No classifier examines message text, and no String conversion or compatibility API was introduced.

## Schema-first Source evidence

The closed neutral Deflate cause schema now requires named `retainedAdmission` cases:

- `physicalHistory`: declared raw length 1, semantic segment limit 65536, retained history target 32768, maximum allocation 32767, origin `ownedPhysicalCeiling`, kind `ownershipLimit`.
- `semanticLength`: declared raw length 65537, semantic segment limit 65536, maximum allocation 32768, origin `semanticSegmentLength`, kind `workLimit`.

The existing grammar corpus and its authored invalid/empty DEFLATE vectors are preserved. The new language-neutral Source law rejects exchanged categories and extra contract fields. Independent system zlib compresses and inflates both literal byte vectors. Independent SQLite projects the authored origin-category mapping. Actual canonical TypeScript ValueError/PackError cause ownership preserves each explicit kind despite identical misleading cancellation/depth prose. This verifies the neutral contract and error transport; it does not execute the Rust inflater constructor. Root's authentic existing Native assertion remains the production gate.

The first registered Pack error `test-ownership` attempt stopped before assertions at sixteen pre-existing TypeScript producer-authority API mismatches (`refusalKind`, `causeKind`, `RetainedAllocation`, paged factories and transport metadata). Receipt: `🗑️generated/retained-deflate-physical-category-source-red.log`. It is a prerequisite failure and is not claimed as feature RED. Those broad staged APIs were not modified.

The proven kernel portable route then executed the new Source law before fixture correction and producer mount:

```
SEMIO_TEST_LEVEL=quick NX_DAEMON=false NX_ISOLATE_PLUGINS=false \
NX_WORKSPACE_DATA_DIRECTORY=<unique-ticket-generated-directory> \
bun nx run @semio-tech/framework-os-kernel:test-snapshot-native-admission \
  --skip-nx-cache --args='portable'
```

Authentic RED: thirteen laws, twelve passes, one failure, 209 expectations, 297 milliseconds Bun, 2.7 seconds Nx critical path / 3.0 seconds run. The newly required fixture member was absent and Ajv rejected the unchanged fixture. Receipt: `🗑️generated/retained-deflate-physical-category-portable-source-red.log`.

After the literal fixture and exact originating Rust refusal were authored: thirteen laws, thirteen passes, zero failures, 223 expectations, 482 milliseconds Bun, 10.9 seconds Nx critical path / 11.3 seconds run. Receipt: `🗑️generated/retained-deflate-physical-category-portable-source-green.log`. This route returns after Source laws and invokes no Cargo tests.

## Changed files and parser

1. `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`: one physical constructor refusal mapping.
2. `🧰️framework/🔨️modules/🎒️pack/⚠️error/🧬️schema/🧭️cause/📡️codec/🔣️.json`: explicit closed admission origins and categories.
3. `🧰️framework/🔨️modules/🎒️pack/⚠️error/🧫️fixtures/🧭️cause/📡️codec/🔣️.json`: two authored admission witnesses.
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🧪️tests/💰️allocation/🟦️.ts`: one portable neutral law in the already registered suite.

`rustfmt --edition 2021 --emit stdout` on the actual protocol codec exited 0; parser log `🗑️generated/retained-deflate-physical-category-parser.log`. Syntax only is confirmed. No Native feature mount, default change, control-budget relaxation, lifecycle change or JSON test edit was made. Root owns the fresh JSON20 regression.

## Root's subsequent Native receipt

Root reported the fresh whole JSON20 regression after the exact origin repair: Nextest `c1923b58-8b8c-40b8-b336-3b3b3644f492`, twenty executed, twenty passed, ninety outside the selection, 3.560 seconds test runtime / 47.7 seconds Nx. The existing 4096-byte ownership-category refusal and paid-input law passed. This is Root's actual Native execution, distinct from this lane's thirteen portable Source laws.
