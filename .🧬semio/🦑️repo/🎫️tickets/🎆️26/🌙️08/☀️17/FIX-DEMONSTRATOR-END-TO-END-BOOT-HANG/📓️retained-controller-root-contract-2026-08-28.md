# Retained Shard Controller Root Contract

## Current UI264 Checkpoint

The UI owner source-mounted its serialized child slot and exported pool envelope264/5/5. The original controller208/1/1 and its retained768/10/10 reservation are unchanged. UI-only pool ownership is now824/14/14; combined admission is1592/24/24. The runtime still imports the actual exported metadata rather than a hardcoded combined total. No capacity changed.

The old fixture genuinely failed9PASS/2FAIL/137skip148,1.80s,start00:59:15: observed1592 versus expected1520, and3184 versus3040 for two pools. After updating only the actor fixture/schema prices, the same eleven-group test passed11/137skip148,3.77s,start00:59:55. Logs: `🧪️actor-pool-metadata-red-1.log` and `🧪️actor-pool-metadata-green-1.log`. Six selected inputs stayed identical before/after:

```text
3b7bd406195eb3ba360d0b31f4bc14ba4cccf3f282fdd9e78219380a5af65b22  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
9e8a686e21034b192cb2d2d3b36d3d8d5b625740efdf5fdc9913d75b8c29f74e  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
e0a03d04bad6133c974e7cd462c4872c482d00db9637585200da7b373f6a099c  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
ee47747e81eea96a43924962145137388d30c6d45a253b0a7119401731688b94  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
a9e103be4635c52244ef12de057b47005e97b4a81e50cf9001a9959b8af79ca0  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🪪️metadata/🟦️component.ts
6957021837e6e5c731cec0530bb109694cff4ef6b25a9ecb0a9f02f37c1c8840  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
```

This refresh verifies the imported envelope's controller/empty-pool conservation and the existing focused ownership laws. The UI's positive child-admission gate and payload integration are not certified by it. The following192-byte checkpoint and hashes are retained historical evidence.

## Scope and Executed Red

The pool record must not fund the controller that continues running after that record's intrinsic retirement. The language-neutral controller-charge law genuinely failed against the previous implementation: 10 passed / 1 failed / 136 skipped / 147 total, 1.12 seconds, start00:41:51. The observed304 bytes were below even the former controller-plus-cell minimum456. Log: `🧪️actor-controller-charge-red-1.log`.

The separate controller record/cell is now implemented. The first focused run passed11/136skip147 in1.76s at00:45:48. The strengthened final run passed11/136skip147 in1.68s at00:46:38; it checks the actual original record's private shell match before any pool admission and again after per-pool close and `disposeAll`. Logs: `🧪️actor-controller-root-green-1.log` and `🧪️actor-controller-root-green-2.log`. Strict Ajv validates the language-neutral declarations; TypeScript's AST independently checks the real nine-field census, and Immer computes the conservation oracle. Thirty wrapper-after-original-return combinations and all fifteen preparation cancellation frontiers execute. This is focused controller/child accounting and ownership proof, not full controller retirement, a full actor/UI pass, guest execution or six-app acceptance.

## Original Parent Ownership

The existing Shard owns a separate controller admission cell and controller record in the same injected ledger. It installs the exact original Shard shell before beginning any UI-pool admission. That controller record is never detached or refunded by per-pool close, ordinary dispose posting, worker-route deletion or loss. Its genuine final captured Shard/controller terminal join remains future work; this packet exposes no surrogate terminal witness or release API. The original ledger's actual record roster strongly owns the installed shell.

The pool record then funds only the UI pool's exported metadata envelope. The controller remains charged while the pool's intrinsic record retires, its result aliases detach, its cell retires, and the controller performs final per-pool bookkeeping. It also remains charged indefinitely while an arbitrary original fault is held. A fault-cell's own metadata never substitutes for the controller charge.

## Implemented State and Initial UI192 Prices

The actual controller has nine private fields: controller cell, controller record, pool cell, pool record, pool shell, phase, pool witness, first fault, and a closing bit. The closing bit prevents new pool construction while bounded fixed controller-bootstrap completion is driven for an already-started close. Existing actor logical metadata convention is 64 bytes per record plus 16 per field: 208 bytes / one slot / one owner. This is logical retained accounting, not measured physical heap.

Controller ownership is therefore 208 + 264 + 296 = 768 bytes / 10 slots / 10 owners. Pool ownership is exported UI192/4/4 + neutral record264/3/3 + cell296/6/6 = 752 bytes / 13 slots / 13 owners. Both admitted together use 1520 bytes / 23 slots / 23 owners. Healthy per-pool completion leaves the actual controller768/10/10 charged. Intrinsic cleanup with a pool fault leaves controller768 plus fault cell296 = 1064 bytes / 16 slots / 16 owners. No capacity is increased.

## Granted Phases

Controller admission: bootstrap296, recover original cell64, claim64, observe claim64, reserve controller record264, recover typed record64, install original shell64, observe exact installation64. Only then may pool bootstrap begin. Pool admission retains its six transitions296/64/64/64/264/64. Each transition consumes at most one child grant and returns pending; a later ready0 result admits the UI constructor.

Every wrapper-loss law applies separately to controller and pool bootstrap, claim and record calls. Before claim clears the bootstrap pointer, the original private slot already holds the cell. Typed recovery never repeats resource admission. Original faults are captured without inspection and handed to the appropriate original cell on a separate64-byte turn. Constructor/installation failure keeps the original rejected facade and charge; no replacement controller is created.

Closing before any controller admission can complete an empty per-pool scope. Closing an already-started healthy controller bootstrap may finish only that fixed root registration, under the same declared phases, before completing the empty pool; it never admits UI content or a second root. Faulted partial bootstrap remains retained and is not declared terminal. A closed-ledger refusal with no admitted cell is separately observed and closes without inventing a cell.

The executed tests check source-derived field inventory, actual controller registration before UI allocation, both admission scopes' wrapper failures, partial cancellation, independent peers, and retained charges at every intrinsic/alias/cell/parent frontier. Final whole-controller retirement, live response admission, native InputAck, guest memory, and all-app behavior remain unproved.

## Stable Source and Strict Boundary

Seven captured inputs were identical before and after the strengthened gate and parallel strict run:

```text
ca48e83b2a028d345968d9653fc18777ae7e7b8fffc58847b3edec5f8d241508  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts
2f5cbe23c19f32de649d09682e726e275cd8030e154d40d38826cf64eabcda19  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧬️schema.json
c92b506325e99087ed81df16d55e2cfbd8d1946c551bc2f2c02c19d7adc6d810  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎭️actor/🏘️composition/🧪️fixture.json
6957021837e6e5c731cec0530bb109694cff4ef6b25a9ecb0a9f02f37c1c8840  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️component.ts
fce17eadc8339fbd5bdad8feaed33361d8c853b955fc0ef67a902b6d036a3b99  /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️component.ts
eab0e5365916a82d78fcf89cb4dcb996e0b6dc14a32a550270365871439dc2f0  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx
448cd4f9a53ebfb2161b24acbd99908e130743316ef6b57f2930f653f56c88d1  /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/💾️resident/🧪️fixture.json
```

The actual renderer typecheck exited1 with53 diagnostics: seven tutorial joins, four actor-owned old private-pool fixture constructors,22 UI pool child/counter joins,19 UI fixture private-pool constructors, and UiDocumentStore829 implicit-any. No new Shard controller/cell diagnostic appeared. Exact output: `🧪️actor-controller-root-strict-1.log`. UI child/payload and fixture adoption remain in progress; no full-current strict/actor/React pass is claimed.

The UI owner has proposed an additional72/1/1 serialized child slot, changing pool metadata192/4/4 to264/5/5. That price is not released at this checkpoint. Production uses the exported metadata function, not the1520 fixture total. The exact schema/test boundary will be refreshed when the actual UI envelope is source-released. No capacity increase, neutral edit, Cargo/native build, publication, cleanup or source restoration occurred.
