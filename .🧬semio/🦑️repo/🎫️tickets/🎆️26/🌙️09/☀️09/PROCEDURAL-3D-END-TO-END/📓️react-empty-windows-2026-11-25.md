# React empty Flow canvas and parked preview (2026-11-25)

Probe: `react-empty-probe.mjs` (one headless chromium, capped). Outputs `🗑️generated/react-empty/{a..j}-*.{png,txt,json}`.

## Findings
1. Document loaded: `__semioFlowGraphProbe["window:procedural-main"]` holds 7 nodes; `data-status-json` shows profile queued, extrusion-axis computing, extrude queued.
2. Flow canvas empty: `createFlowSession` rejects with `Flow Wasm source is required`. `flowBrowserDeliveryV1` returns kind `source` for `serve`, so `@semio-tech/flow-core` is the stub (`playground-stubs`). No `flow_core*.wasm` exists under `.cache`. The rejection was swallowed; `useFlowSessionAttach` now logs `[flow] session unavailable` (verified in `j-console.txt`).
3. The 21 Nov node-graph wasm is not the cause: the Flow window never requests it; a manual import creates a session.
4. Preview parked: phase faulted `flow.extension-not-contributed` (brep). Manual `semioReactActivateOnDemand` answers active, manifests fetch, no contribution lands in 40 s (`h-*`, `i-*`).
5. One `[DEBUG] receiving quote lane=Config` per boot: scope items 10 > phase items 8, quote 0, publication waits (`typed-operation-unit/🦀️.rs:165`). Likely blocker for the contributions push; owner: escrow lane.

## Needed
- Build `semio-framework-os-flow-browser-package:wasm -c prototype`, then make serve choose the publication delivery when `flow_core.js` is published (contract change, not yet made).
- Escrow headroom vs phase grant fix.

## Follow-up: serve delivery and contribution route (same day)
- Serve delivery: `flowBrowserDeliveryV1` (flow/wasm/browser/publication/contract) selects the publication when `flow_core.js`, `🌐️browser/🟨️.js` and `🖥️host/🟨️.js` exist for the selected mode, otherwise the Source stubs stay (and `[flow] session unavailable` shows). Law: `🧰️framework/.../🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` "neutral Flow serve delivery ..." (passes under `bun test ./🟦️.ts -t "Flow serve delivery"`). Restart the 6018 host after the wasm is published.
- Quote tail: refuted as the blocker. Slots 1 -> 0 is followed by no fault; the boot completes (fault gone since 10:05).
- Route after `semioReactActivateOnDemand` (temporary [DEBUG] hooks, removed again; runs `🗑️generated/react-empty/{m,n,q,t,u,v}-*`):
  1. door answers requested/active: yes.
  2. extension manifest import: yes (`🧩️extension-modules/<id>/🔣️.json`, 739 KB for brep).
  3. extension component birth: NO. No request for `semio_s_plugin_flow_brep_component*.core.wasm` and no actor activation, ever.
  4. loadedPlugins change, registry key, refresh: yes. `setContributions` runs twice (363351 and 425209 bytes) and settles with no fault and zero effects; the installed pack holds 159 brep operators but brep appears twice (duplicate row) and `__semioOsInstalledContributions(1)` shows no math row.
  5. preview re-owe: effects=0 at settle; phase goes faulted -> computing ~45 s later by itself (faulted is not terminal), then stays `computing units=0/1` for 110 s.
  6. First hop that never happens: any `invokeExtension` dispatch (`dispatchInvokeExtensionEffect` never entered), hence no extension instance birth.
- Not proven: whether the stale 24 Nov extension components or 04:18 shard would fault once invoked, because nothing invokes them.

## Run-7 flow product (14:18), host restarted 14:19
- Published set complete; `flow_core.js` now exports all seven `flow_bridge_*` (react-flow-core-probe.mjs). Session creation succeeds (no `[flow] session unavailable`).
- Next error: `[flow] canvas attach refused on window:procedural-main RuntimeError: unreachable` (`🗑️generated/react-empty/{x,z}-console.txt`, screenshot `x.png`: canvas still empty). Top frames: `rust_panic` -> `panic_fmt` -> `drop_glue<Option<DagInputFacts>>` -> `drop_glue<FlowDagAction>` (`🌊️flow/🕸️wasm/🧵️dag-input/🦀️.rs`, `FlowDagAction.application: Option<DagInputApplication>` holding `DagInputFacts`). Reading: a FlowDagAction input action (ops 2514/2515/2525/2528) is dropped with unretired facts, which the retained-allocation discipline turns into a panic; the original refusal that triggered the early return is masked. The panic message is not printed (no panic hook); needs a guest-side message or a rebuilt product with the hook.
