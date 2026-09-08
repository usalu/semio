# Flow Native Serialization and Retirement

## Scope

This lane repairs the Flow native-test closure after the first-party `DslValue` transition, preserves the browser session admission owner through uncertain open failures, and moves byte-bounded retirement to the retained DAG/Scene owners beneath `FlowHostRetirement`.

## Browser session ownership

- Flow artifact, extension, playbook, registry, VCS, and host tests use first-party `ToValue` / `FromValue`. `serde_json` remains only an independent RFC 8259 test oracle.
- Only an exact failed `Open` reply is a verified pre-admission rejection that may release its JS session entry.
- Transport, decode, malformed reply, and Wasm exceptions remain uncertain admitted owners keyed by exact operation and generation. Global close clears them only after the exact host-terminal proof.
- Shared runtime close is a single retained promise. A failed per-session close cannot prevent whole-host drain, and a failed whole-host terminal proof cannot claim the owner empty.

The controlled browser suite capture `70750` proves two sibling sessions, late-open sibling preservation, receipt/control backpressure, verified rejection, uncertain admission, terminal refusal, and cooperative close. It then reaches the known stale generated-Wasm trap in `flow_bridge_send`; this is not a fresh Wasm runtime claim.

## Deep retirement contract

`DagRetirementStep`, graph backing, icon backing, and opaque Scene retirement now distinguish:

- `credited_bytes`: credit consumed from the current grant; always at most that grant.
- `released_bytes`: physical backing freed on this turn; it may use credit retained from earlier turns but never exceeds cumulative outstanding credit.

Large contiguous allocations cannot be physically deallocated in fragments. Their exact cursor therefore retains the owner and its accumulated credit; only once the full backing is covered does it drop that backing and emit the physical release. Every terminal law requires cumulative credited bytes to equal cumulative physical releases. `released_bytes` is therefore charged against the cursor's cumulative outstanding credit, while the current turn's `credited_bytes` remains bounded by the current grant.

The retained DAG cursor owns nested strings/DSL/property/port/node/edge/event payloads, vector backing, empty hash-table backing, graph-engine vector backing, icon key/exclusive-raster backing, and a transferred ghost node. A 4,800-byte UTF-8 note and a Scene command vector larger than 4,096 bytes both converge under grants `1`, `64`, and `4096`; zero grant remains blocked.

The opaque Scene registry is now generation-fenced and reusable. `IconPaintCache` retains the exact Scene token, drives one Scene command per item turn, accumulates byte credit for the empty Scene vector backing, and cannot report terminal before that token releases. The native law exercises 1,025 sequential reservations against the 1,024-slot registry, proving reuse only after terminal release.

The normal `BoardHost` world-cache path also retains its exact registry token instead of discarding it. Each Board holds at most one such cursor, advances it by one item and 4,096 bytes per render/close turn, defers another cache replacement while it is live, and cannot leave `WorldScene` or satisfy its terminal predicate until the token is released. Its cache law combines 128 commands with a 1,600-element path, crosses the 4,096-byte backing boundary, and observes the retained token through more than 1,600 turns before terminal release.

`EngineCanvasPacket` uses the same owner discipline: it no longer reports close after merely publishing a Scene to the process registry. The packet retains the token, advances it with the same bounded turn, and requires exact token terminal before its own terminal predicate. Its 128-command unit law covers the prior publish-and-forget path.

Scene retirement now reaches beneath the command vector. Each command is transferred into a retained command cursor, its full currently-owned backing is captured and credited before any nested owner can be released, then Bézier path elements, stroke dashes, exclusive raster ownership, and exclusive Vello encoding vectors are drained one item at a time. Only after that drain may the command destructor run and report its physical release. The source/native fixture adds both a 1,600-element first-party path and a 256-rectangle SVG/Vello fragment, independently forcing command payload backing past the normal 4,096-byte turn.

Nonterminal `DagHostRetirement` drop still refuses recursive release at the process boundary. Ordinary cancellation is represented by retaining the same cursor after `Pending`/`Blocked`; no dummy replacement or whole-DAG destructor is used.

## Validation

The direct permanent Flow source command is GREEN:

```text
[DEBUG] Flow session-retirement source fixtures=1 hostileRejections=5 bytes=42405 dagBytes=4800 sceneCapacity=1024 sceneCommands=128 scenePathElements=1600 sceneVelloRects=256 sceneConsumers=3 grants=1,64,4096 oracle=fast-json-stable-stringify runtimeClaims=0
[DEBUG] Flow retained-session close fixture=1 hostileRejections=4 oracle=fast-json-stable-stringify runtimeClaims=0
[DEBUG] Flow browser runtime lifetime fixture=1 hostileRejections=5 oracle=fast-json-stable-stringify runtimeClaims=0
```

Launch seed and generated launch both contain the registered native command with `--skip-nx-cache`; registry generation completed and the subsequent generated check was dispatched.

The registered runner resolves to one `semio-framework-os-flow` lib-test binary and applies the libtest substring `session_close`. Its current selected source corpus is ten laws: three deep retirement laws, five ABI session-close ownership/cancellation/receipt laws, and two compiled-session sibling/terminal laws. `--skip-nx-cache` prevents Nx replay; Cargo must re-fingerprint the changed Flow and Infinite inputs.

The prior Flow test fingerprint diagnostic is timestamped `2026-09-08T02:26:07+0200`; current DAG and retirement-test sources are timestamped after `04:49`, so the earlier mixed-interface build is not admissible evidence. Current source-frontier SHA-256 anchors are:

```text
1ee2313ed875d34afafcaca85d3f752027308fdb25ca25eb39e3f67a507a99a2  directed/dag/🦀️.rs
45adc7b1528445613eb8403bba0d279752919baef6bb85ef7cf7ad4a953f4085  host/retirement/tests/🦀️.rs
fd526771a41dde8c2a474a6dde5809fc692ebdeecf9773d31a4bc93d84948c01  canvas/🦀️.rs
cb89943c421d5f344a1892f402f042482cca79cf562124f75f0a489793a6717c  geometry/engine/🦀️.rs
9eaab376025b9def2c5cd3b95c3600b8882068c24afb101e8121ffef8cb64639  directed/normal/🦀️.rs
5879388bea685d25afa02e1c830e7399a620ca54923279ce977e3725431d0d1e  EngineCanvas/wgpu/🦀️.rs
```

## Native status

Native session `61322` terminated BUILD RED before laws with four stale-interface errors in `FlowHostRetirement`: the already-built Infinite dependency still exposed the prior zero-argument DAG close and no `DagRetirementStep`. It is not runtime qualification.

Native session `13627` compiled the current Infinite/Vello retirement source and then terminated BUILD RED before list/laws with 25 Flow lib-test closure errors. The retained diagnostic is `🗑️generated/flow-session-close-native-target/debug/.fingerprint/semio-framework-os-flow-0c94a2807b6bd994/output-test-lib-semio_framework_os_flow`. The errors were test-only stale serde derives over first-party-only `Widget`/`FlowFixture`, two `?Sized` helpers passed to the sized first-party printer, one component serde roundtrip, one `UnwindSafe` boundary, and two inferred JSON object keys. No deep-retirement compiler error was emitted.

Those 25 errors are repaired without restoring serde compatibility: all ten direct mutation leaves, their aggregate, and Flow diff now use only first-party `ToValue`/`FromValue`; the component roundtrip uses the first-party JSON parser with an independent serde JSON oracle retained elsewhere. The source gate is GREEN with the unchanged 4,800-byte DAG and large Scene/Vello vectors.

Native retry `60416` completed BUILD RED before list/laws with five test-closure errors: three missing explicit DAG test imports, one stale immutable `FlowHost` binding, and one unnecessary mutable binding. The current retirement implementation compiled through that frontier; no native law ran. The exact retained diagnostic remains `🗑️generated/flow-session-close-native-target/debug/.fingerprint/semio-framework-os-flow-0c94a2807b6bd994/output-test-lib-semio_framework_os_flow`. The imports and binding mutability are repaired source-side; native GREEN is still unclaimed pending a fresh exact retry.

Native retry `25499` built and listed the current binary, then ran 8 of the selected 10 laws before fail-fast. Seven laws passed: all three deep DAG/Scene retirement laws, both compiled-session sibling/terminal laws, and two ABI close/cancellation laws. Law 8, `session_close_receipt_retries_a_colliding_event_slot_and_preserves_sibling`, failed before its intended collision assertion completed. Its fixture uses request id `65`, but `FLOW_MAX_REQUESTS` is now the ABI-wide `256`; the injected event occupies slot 65 while the first session's terminal receipt occupies slot 1. The third close poll can therefore publish the receipt instead of remaining `Pending`. This is a stale test collision, not evidence of a production retirement fault. The production-owned protocol test should derive the colliding id as `FLOW_MAX_REQUESTS as u64 + 1`. The last two laws were fail-fast skipped by Nextest; both passed when invoked directly by exact name against the same emitted binary. The emitted test binary SHA-256 is `f21541e78fcc144a2506fb65c0d3b54c37df3b544c908c5d546f34e59bba6538`; a full native GREEN was not claimed from that receipt.

Native retry `11578` used the collision derived from `FLOW_MAX_REQUESTS`, built and listed one current Flow lib-test binary, and completed GREEN10. Nextest run `667b1b96-0716-4993-978d-11b3e1c117f1` passed all ten selected laws with 241 tests skipped: three deep DAG/Scene retirement laws, two compiled real-adapter sibling/terminal laws, and five ABI ownership/cancellation/receipt laws. The current emitted binary is `🗑️generated/flow-session-close-native-target/debug/deps/semio_framework_os_flow-0c94a2807b6bd994`; its SHA-256 is `2125a6380aec5163e8baf9a3dc8dcb6a2687337a151053d07da957387d7096d4`. The DEBUG witnesses include A-only retirement while B completes selection, real VCS/host terminal emptiness, and terminal receipt retention across event-slot backpressure without disturbing the sibling.

## Residual boundary

Scene commands and their first-party nested vectors are retained through byte accounting before release. An exclusive host-only Vello fragment is similarly drained through the public encoding vectors, with each allocation capacity credited before its final drop. Heap storage shared through `Arc` is not reported as released unless the retiring owner is exclusive. Vello font/image blobs remain a deliberately narrow item-owned residual: they are third-party shared blobs whose public logical byte length is not an exact physical allocation-capacity witness, so this lane does not falsely claim that backing as physically byte-accounted.
