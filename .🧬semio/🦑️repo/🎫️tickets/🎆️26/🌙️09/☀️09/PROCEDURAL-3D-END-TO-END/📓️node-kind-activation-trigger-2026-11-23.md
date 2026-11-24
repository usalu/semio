# Node-kind activation trigger (2026-11-23)

Closes the open item of `📓️lazy-extension-activation-2026-11-23.md`: a flow document that uses a node kind of an inactive extension now activates that extension by itself. Host side is complete and tested (`bun test`, 32 tests); the Rust side was written but **not compile-checked** (swap free was 810 MB, below the 1.5 GB gate; no cargo ran). The only Rust that executed is the pure request helper, compiled standalone with `rustc +nightly-2026-07-20 --test` (4 of 4 laws pass).

## Where a document becomes host-visible

| Surface | Earliest host-visible point | Trigger |
|---|---|---|
| React, boot example and picker | `dispatchActiveExample` in `ShellHost/🔎️command-bar/🔶️scope` (the initial example goes through the same function, `useInitialExampleReadiness`) | manifest example body (`documentJson`, the inline DSL of the bundled example) |
| WGPU, boot example (`?example=` or default) | frame Worker `boot`, after `mountPluginHandles` and before `semioWgpuWorkerBootstrap` starts the shell (`descriptor.appExample` plus the mounted manifests) | same body |
| WGPU, example picked inside the wasm shell | none: the pick never leaves wasm | guest request (below) |
| Any document the guest loads or edits | none | guest request (below) |

A bundled generation3d example is `include_str!` of its `.dsl.semio`; that text is the manifest's inline `documentJson`, so it is host-visible without any wasm change.

## Host design (shared, one implementation)

New kernel module `🧰️framework/🔨️modules/🎠️kernel/🧭️node-kind-trigger/🟦️.ts`, exported from the kernel entry:
- `scanNodeKinds(text)`: `neuron-kind=` (DSL) and `neuronKind` / `neuron-kind` / `neuron_kind` (JSON) values, distinct, in order.
- `nodeKindRequests(kinds)`: one `{kind:"node-kind", nodeKind}` per leading namespace, ascending. `documentExtensionOwners(kinds, candidates)`: kind to namespace to `on-node-kind:<ns>` event to owner.
- `activateDocumentNodeKinds`: requests run concurrently through the shared `OnDemandActivator`, progress per namespace, never rejects. Report states `ready | partial | cancelled`; `unavailable` lists each node kind with its plugin and reason; `unowned` are kinds no deferred plugin claims (served by an active plugin, or the guest reports them unknown).
- `openDocumentAfterActivation` / `openExampleAfterActivation`: scan, activate, notify, then call `handoff` exactly once after everything settled. Cancel (caller abort or `activator.cancel()`) hands nothing off. An unavailable extension still hands the document off, with the report so the guest can mark the nodes.
- `documentActivationLabel` / `documentActivationNotice`: en and de texts (unknown locale tells English), ARIA `alert/assertive` for unavailable, `status/polite` otherwise.

Wiring:
- WGPU: `🎞️frame-worker/🔌️boot-and-plugins/🟦️.ts` runs `openExampleAfterActivation` with `bootAbort.signal` and posts progress and the notice as boot-progress stages; a cancel throws into the existing boot fault path.
- React: `dispatchActiveExample` runs `openExampleAfterActivation` and calls `onAction(buildActiveExampleAction(...))` as the handoff; a newer pick aborts the previous activation; the notice uses the shell's transient notice (`error` tone when unavailable).

## Guest design (Rust, edit only)

- Request: `Effect::PublishEvent { topic: "os.on-demand.request", payload: UTF-8 JSON of $defs.onDemandRequest }`. No ABI or WIT change, no new Effect variant. Schema: new `$defs.onDemandRequestTopic` in `🧬️schema/🧩️plugin-modules/🔣️.json`.
- `flow/🖥️host/🚦️eval-status/🦀️.rs`: `ON_DEMAND_REQUEST_TOPIC`, `on_demand_node_kind_requests(unserved)` (the Rust twin of `nodeKindRequests`, same namespace rule), exported through `semio_os_flow`.
- `flowEvalTick` (`✏️s/🔌️plugins/🌊️flow/.../⏱️flow-eval-tick/🦀️.rs`): the tick that finds unserved kinds (the existing `unserved_flow_operator_kinds`) still abandons its window (the document stays parked, no spin) and now emits `on_demand_effects(&unserved)`: one request per namespace.
- Resume: unchanged existing mechanism. The activated extension's contributions push moves the flow registry generation and `invalidate_for_flow_extension_registry` re-arms every attached window. An extension that stays unavailable leaves its nodes faulted as unknown kind (the "marked unavailable" state).
- React consumes the request in `host-effects/🧭️on-demand-request/🟦️.ts` (door `semioReactActivateOnDemand`, fire and forget, invalid payload dropped with a `[DEBUG]` line). WGPU consumes it in the shell `dispatch-action` effect loop (cfg wasm32 arm, topic guard) through the new bridge function `request_on_demand_activation`, which calls the existing `semioWgpuActivateOnDemand` door without awaiting.

## Tests

- TypeScript (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧭️node-kind-trigger/🟦️.ts`, fixture `🧫️fixtures/🧭️node-kind-trigger/🔣️.json`, 32 pass, `tsc --strict` clean): all nine 3D examples resolve to exactly their extensions (brep alone for six, brep plus math for three) against the committed catalog; the scan equals an independent oracle (whitespace tokenizer, no shared regex) on the real `.dsl.semio` files; JSON form equals a structural walk of the parsed document; requests validate against the schema with `ajv`; ordering (handoff once, last, after both mounts); cancellation (caller abort, early abort, `activator.cancel()`); unavailable extension (opens, nodes reported, texts equal the fixture in en and de, alert role) through both the React-shaped and the WGPU-shaped activator; React and WGPU activators give identical reports and mounts for every example; source laws pin the call order in both hosts and the topic in the Rust files.
- Rust: `🧪️tests/🔬️unit/🧭️on-demand-request-laws` (flow host) and a law in `⏱️flow-eval-tick/🧪️tests/🔬️unit`. Not run through cargo; the helper laws were run standalone.
- Unchanged and still green: `⏳️on-demand-activation` (22).

## Open and rebuild needs

1. Rust compile check pending: flow host session crate, flow editor crate, program-bridge, shell. Run `cargo check -p` of those when swap allows.
2. Rebuild: TypeScript browser bundle (frame Worker, shell host, kernel); `flow` and `procedural` components (the flow host session crate changed); the renderer wasm (shell arm plus bridge function). Nothing else restages.
3. The generation3d preview chain (`🧵️preview-eval/⏱️tick-and-resolve`) has its own tick whose outcome struct (`FlowEvalTickOutcome`) has no effect slot; it does not emit the request yet. Boot and React picks are covered host-side; an in-wasm WGPU pick of a 3D example needs that outcome to carry `on_demand_effects`, a small change across the viewer job layer, deliberately not made blind.
4. To verify live: that the WGPU frame Worker's on-demand mount (`onDemandModules`) is followed by the shell's `setContributions` push so the registry generation moves; the React lifecycle is believed to do so through its contributions refresh. If the WGPU push is missing, it is the one remaining link of the resume chain.
5. The WGPU boot notice is a boot-progress stage text, not a shell notice; the accessible announcement for WGPU needs a shell notice once the renderer is rebuilt.

## Compile check (2026-11-23)

- `semio-os-flow-host` `cargo check` (prototype, shared store, -j 2): clean, 0 errors (24 min cold). Unit laws `unserved_kinds_become_one_sorted_request_per_namespace`, `the_wire_text_is_valid_json_even_for_a_hostile_kind`, `the_topic_is_the_one_the_hosts_listen_on` pass. `a_kind_without_a_usable_namespace_asks_nothing...` also passes on rerun (all four on-demand laws green). The unfiltered `test --lib` aborts (SIGABRT, non-unwinding panic in `a_coalesced_tick_parks_a_whole_wave...`, other host tests FAILED) while `semio-framework-job-native-decode` was under concurrent edit by another lane; not related to this change.
- `semio-s-artifact-flow-flow-editor` `cargo check` (flow-eval-tick): clean. Two earlier attempts failed on other lanes' in-flight edits (`debug_state` in store publication digest), unrelated.
- `semio-renderer-wgpu` (ProgramBridge wgpu + Shell dispatch-action arm) `cargo check --lib`: clean natively and with `--target wasm32-unknown-unknown` (the cfg wasm32 arm is compiled); only style warnings.
- flow-eval-tick law `unserved_kinds_publish_one_typed_activation_request_per_namespace` NOT run: the `flow-unit-tests` crate fails to compile with 9 errors in other files (`connect-media-ports` tests, `native-resolve`, `evaluate` tests: `RetainedAllocationEffect` API drift from another lane). None in eval-tick.
- Hub staging: first attempt failed on others' transient syntax and compile errors in the graph; staging was then handed to the lane that fixed the offload `#[path]` bug, which owns the hub rebuild. Logs in `🗑️generated/check-*.txt` and `test-*.txt`.
