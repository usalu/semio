# Fixed Worker Maintenance Hooks

## Current Status

Latest exact qualification is terminal GREEN11: session 35401 exit 0, `worker-maintenance-exact/exact-cargo-laws-N6tja5/00`, executable SHA256 `5ed2649ed330b0b9737db00688b961d42f9fc18b6ab576560fb6e3ef6eb678bb`. All seven new hook laws and four existing DRR/cooperative host regressions passed, including in-flight close/shutdown and identical actual native/cooperative job/two-hook ordering. Earlier paragraphs below preserve the test-development sequence. The projection target is root-owned and idle after this receipt. DB writer signal/backend integration remains required.

In progress. The strict neutral AJV/Map lifecycle oracle (20 transitions, capacity 64) passed before the expected missing-module failure in session 59127. The first four-law native run is terminal GREEN: session 78367 exit 0, `worker-maintenance-exact/exact-cargo-laws-PDOhF8/00`, executable SHA256 `89153ce3ee2043113322c18a95f5cf8579bfb346baefcc409fe009e504cb7727`. It directly exercised actual native idle callbacks and cooperative host-pump callbacks. This is not a DB/WAL writer integration receipt.

The exact selector is now eleven laws: the original four, actual native in-flight removal/shutdown, native and cooperative job/two-hook interleaving, and four pre-existing DRR/cooperative host scheduling regressions. Native session 69070 uses the same exclusively owned projection cache. Source assertions now require both platform APIs, registry lifecycle markers, and both permit-bearing scheduler call sites, in addition to the independent neutral model. No eleven-law verdict is claimed in this update.

Session 69070 / `zJcr77` passed the first ten exact laws, including every new hook law, then failed the pre-existing live-host source assertion. That assertion still searched for removed raw maintenance constants and an `_ => Err` fallback, while the actual plugin runtime now uses an exhaustive typed `RuntimeMaintenanceStatus::Queued | RuntimeMaintenanceStatus::Running` branch. The production branch still pumps exactly once. The checker now locates the current typed branch and its balanced body; its hostile missing/doubled pump, looping pump, and fake clock cases remain intact. A warm eleven-law retry is running. No production plugin runtime file was changed for this test repair.

## Integration Reason

The WAL writer lifetime needs an idle wake that neither submits a DB task from the locked lost-owner path nor allocates a one-shot job in permit Drop. Existing DB maintenance runs only on task ingress/poll/close. An idle dropped writer would otherwise retain its file guard indefinitely. The chosen framework primitive is a fixed reusable WorkerPool hook, with a plain function and two inline context words, one exact pool/slot/generation ticket, and coalesced requests. The WAL/backend owns all durable guards and credits; the hook owns no database resource.

## Implemented Boundary

Each native or cooperative pool pre-admits 64 registry entries. Native workers and cooperative host pumps select hooks through their existing deficit-round-robin lanes and worker permit accounting. Hooks alternate with ordinary jobs in the same lane and rotate among requested hook slots. A callback gets one bounded turn; More rearms, Idle/Fault wait for another request, and a request concurrent with an active callback is preserved. Callback panic is caught before retiring its invocation state.

Removal is explicitly retained: false fences new requests but keeps the exact running callback/ticket; the owning close state must call again in a later bounded opportunity. Only true removes the slot. The registry's closed state linearizes install/request with pool shutdown, clears pending flags, and prevents an in-flight More result from rearming after shutdown. It never invokes domain cleanup from ordinary registry removal.

The initial exact laws cover the neutral lifecycle including stale slot reuse and concurrent wake, capacity/foreign pool/generation exhaustion, actual native idle wake without queued jobs or DB ingress, and actual cooperative host-pump/Io-lane behavior. Additional in-flight close/shutdown and competing job/hook cases are being added from the read-only audit. This primitive is not yet wired into WAL storage.

## Registration

The domain lives in `🧰️framework/🔨️modules/⏳️async/🔔️maintenance`. Its package task router has `worker-maintenance-check` and `worker-maintenance-native-check`; launch seed orders 411.079/.080 use the ticket-local output and the owned warm projection cache. Home ran normal registry generation and immediate freshness verification after these seeds: 59 plugins, 60 playgrounds, 45 framework packages.
