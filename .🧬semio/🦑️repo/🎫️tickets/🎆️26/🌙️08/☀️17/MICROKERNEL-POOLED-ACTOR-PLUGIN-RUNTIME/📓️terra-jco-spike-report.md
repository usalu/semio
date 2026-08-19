# 🧪️ terra-jco-spike — does jco 1.27.0 actually drive an async-lifted-everywhere component from JS?

Executor: `terra-jco-spike`. Owned path: `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/**`.
Everything below was actually built, transpiled, and run — never inferred from jco's docs alone
(this ticket's rule 9). Toolchain actually used: rustc `nightly-2026-07-07`, `wit-bindgen 0.57.1`,
`wasm-tools 1.256.0` (reused the already cargo-installed binary from `terra-probe-spikes`'
`🎯️target-s1/cargo-install/bin/wasm-tools`, read-only, nothing written there), `jco 1.27.0`
(repo-pinned), `bun 1.3.14`, `node v24.15.0`, and the in-app Chromium Browser pane (`Chrome/148.0.7778.280`
inside Electron `42.9.2`).

## VERDICT: GO-jspi

jco 1.27.0 transpiles and correctly drives a wasip2 `wit_bindgen::generate!({ async: true })`
component whose **every** WIT function is `async func` — exports, host-implemented imports, and a
`stream<u8>` — across bun, Node (with JSPI enabled), and a real Chromium Web Worker. **But this is
GO-jspi, not GO-callback: JSPI is not optional.** jco's generated JS unconditionally calls `new
WebAssembly.Suspending(...)` and `WebAssembly.promising(...)` for every component-model-async
export/import, with **no flag that removes it** — `--async-mode jspi` produces a **byte-identical**
`jcoprobe.js` to the default (`--async-mode` omitted, "sync" preset). Confirmed both ways: (a) a
byte-for-byte `diff` between the default transpile and an explicit `--async-mode jspi` transpile of
the identical component is empty; (b) running the SAME unmodified `jcoprobe.js` under plain
`node` (JSPI off by default in Node 24) fails at **module top-level evaluation**, before any
instantiation or call, with `TypeError: WebAssembly.Suspending is not a constructor` — and the
identical file under `node --experimental-wasm-jspi` runs clean, all 4 checks PASS. This directly
contradicts this ticket's "Facts established" assumption that the P3 callback ABI should not
require JSPI — the callback ABI describes how the *guest wasm* avoids stack-switching; jco's *JS
glue* for bridging component-model-async's suspend/resume semantics to JS `Promise`s still needs
JSPI regardless of what ABI the guest itself uses. Practically: **Chrome/Chromium-embedded targets
work today (JSPI on by default); any target without JSPI hard-fails the whole module import, not a
graceful per-call degradation** — see "what must change" below.

## S1-S5 verdict table

| # | Verdict | Evidence |
|---|---|---|
| S1 | **PASS** | `probe.poll(21)` returns a real `Promise` resolving to `42`, confirmed in bun, Node+JSPI, and Chromium Worker. |
| S2 | **PASS** | `await-echo(50, 777)` (a dedicated export that directly `.await`s the host import) took ~72-93ms; a concurrent `setInterval(5ms)` fired **13-17 times** while it was pending, in every environment — the worker/event loop was never blocked. Tick counts pasted below, not asserted. |
| S3 | **PASS** | `spawn-detached(80)`'s own export Promise resolved in **~0.9-1.75ms**, while the detached background `slow-echo(80, ..)` call it spawned (via `wit_bindgen::spawn`, not awaited before return) took **~50-85ms** to actually complete — the export resolved roughly 50-90x faster than the detached work it triggered, which is only possible if `task.return` fired independently of the spawned sibling task draining. Matches `wit-bindgen-rust`'s own codegen: `AsyncTaskReturn` fires as soon as the root future resolves (`bindgen.rs`'s `CallInterface` comment), not after `spawn`'s `FuturesUnordered` drains. |
| S4 | **PASS** | `read-body()` read a JS-supplied 5-chunk async-generator `stream<u8>` one byte at a time via the guest's `StreamReader::next().await` loop, returning `5`; host-shim logs show all 5 "yielding chunk" lines interleaved with the guest's polls, not one bulk transfer. |
| S5 | **PASS (JSPI on) / clean hard-FAIL (JSPI off)** | See verdict above — this is the one criterion that flips the overall verdict from GO-callback to GO-jspi. Both states were reproduced with a real engine, not a docs claim: Node 24 default = fail at import; Node 24 `--experimental-wasm-jspi` = all pass; Bun (JavaScriptCore) = JSPI on natively, all pass; Chromium Browser pane = JSPI on natively, all pass in a real Web Worker. |

## the WIT world actually built

`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/🧬️schema/📜️world.wit` (own copy, real
schema never touched):

```wit
package semio:jcoprobe@0.1.0;

interface pure {
  log: async func(level: string, message: string);
  now-ms: async func() -> s64;
  trace-span: async func(name: string);
}

interface probe-host {
  slow-echo: async func(ms: u32, v: u32) -> u32;
  fetch-body: async func() -> stream<u8>;
}

interface probe {
  poll: async func(n: u32) -> u32;
  await-echo: async func(ms: u32, v: u32) -> u32;       // added for S2 — see below
  spawn-detached: async func(ms: u32) -> u32;
  read-body: async func() -> u32;
}

world jcoprobe {
  import pure;
  import probe-host;
  export probe;
}
```

`await-echo` was added beyond the packet's original sketch: none of `poll`/`spawn-detached`/
`read-body` directly `.await`s a host import before returning, which S2 specifically requires
("the guest `.await`s the host import `slow-echo(50ms)`... the export's own Promise stays pending
for the full delay"). `spawn-detached` deliberately does NOT await it (that's the whole point of
S3), so a fourth export was the honest way to test S2 as specified rather than reusing S3's export
and getting an ambiguous result.

## guest Rust — the two load-bearing regions

`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/🦀️component.rs`:

```rust
wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "jcoprobe",
    async: true,
});

// #region S3
async fn spawn_detached(ms: u32) -> u32 {
    wit_bindgen::spawn(async move {
        let _ = probe_host::slow_echo(ms, 0xDE7AC4ED).await;
    });
    1
}
// #endregion

// #region S4
async fn read_body() -> u32 {
    let mut reader = probe_host::fetch_body().await;
    let mut total: u32 = 0;
    while let Some(_byte) = reader.next().await {
        total = total.wrapping_add(1);
    }
    total
}
// #endregion
```

`wit_bindgen::spawn` requires the `async-spawn` Cargo feature (NOT in wit-bindgen's `default`
feature set — `Cargo.toml`: `wit-bindgen = { version = "0.57.1", features = ["macros",
"async-spawn"] }`). Read `wit-bindgen-0.57.1`'s own source before trusting this would work:
`~/.cargo/registry/.../wit-bindgen-0.57.1/src/rt/async_support/spawn.rs`'s doc comment says
verbatim "this can be used to express execution-after-returning in the component model", and
`wit-bindgen-rust-0.57.1/src/bindgen.rs`'s `CallInterface` instruction handler confirms
`AsyncTaskReturn` (the `task.return` call) happens immediately once the *root* future resolves —
independent of whatever `wit_bindgen::spawn` queued into the same `FutureState::tasks`
`FuturesUnordered`. The runtime evidence (S3 row above) matches this source-level reading exactly.

## exact commands that worked

```
$ CARGO_TARGET_DIR=<scratchpad>/target-jcoprobe cargo build --release --target wasm32-wasip2 \
    --manifest-path 🧰️framework/…/🔌️jcoprobe/👽️guest/Cargo.toml
Finished `release` profile [optimized] target(s) in 19.75s (first pull), 0.11s (cached re-run)
exit 0 (terra-jco-guest-build1.txt, terra-jco-guest-build2.txt, terra-jco-guest-build-final.txt)

$ <cargo-install wasm-tools 1.256.0> validate <wasm>
exit 0, "VALIDATE_OK" — confirms wasm32-wasip2 emits a real, directly-valid component; no
`wasm-tools component new` step needed (Rust's wasip2 target is component-native already).

$ <cargo-install wasm-tools 1.256.0> component wit <wasm>
exit 0 (terra-jco-component-wit.txt) — dumped the component's own declared world; NOTE the `pure`
import is ABSENT from this dump even though it's declared in the source WIT — wasm32-wasip2's
release build with opt-level=s dead-code-eliminated it because the guest never actually calls
log/now-ms/trace-span. Real plugins that do call `pure.log` will not hit this; flagged here only
because it's a real, reproducible surprise for anyone diffing "declared imports" vs "component's
imports" during debugging.

$ bunx jco transpile <wasm> -o <out> --name jcoprobe \
    --map "semio:jcoprobe/probe-host=./host-shim.js"
exit 0 (terra-jco-transpile-callback2.txt) — THE command that worked, no --async-mode flag at all
(jco's own default/"sync" preset).

$ bunx jco transpile <wasm> -o <out2> --name jcoprobe \
    --map "semio:jcoprobe/probe-host=./host-shim.js" --async-mode jspi
exit 0 (terra-jco-transpile-jspi-explicit.txt) — `diff <out>/jcoprobe.js <out2>/jcoprobe.js` = 0
lines, byte-identical to the default. --async-mode is a no-op for a component that's already fully
component-model-async lifted; it only matters for the LEGACY path (sync-ABI exports opted into a
JSPI-based async JS surface via --async-imports/--async-exports), which this component doesn't use
because every function in the WIT source is already `async func`.

$ bun run-node.mjs                              # bun 1.3.14, JavaScriptCore, JSPI native
exit 0, all 4 PASS (terra-jco-bun-run1.txt)      # one harmless non-fatal Bun quirk, see "gaps" below

$ node run-node.mjs                              # node v24.15.0, JSPI OFF by default
exit 1 — TypeError: WebAssembly.Suspending is not a constructor, thrown at jcoprobe.js:9393,
BEFORE any instantiation — the whole ES module import fails (terra-jco-node-nojspi-run1.txt)

$ node --experimental-wasm-jspi run-node.mjs     # node v24.15.0, JSPI explicitly enabled
exit 0, all 4 PASS, identical shape to the bun run (terra-jco-node-jspi-flagged-run1.txt)
```

**Browser (Chromium Browser pane, via `preview_start` — never Bash for the server):**

```
$ bun 🧰️framework/…/🔌️jcoprobe/🌐️harness/serve.ts   # launched via .claude/launch.json's
                                                        # "terra-jco-spike-static" entry, port 8846
```

`index.html` spawns `new Worker("./worker.js", { type: "module" })`; `worker.js` imports the
transpiled `jcoprobe.js` directly and runs the identical S1-S4 sequence, posting results back via
`postMessage`. Page text after load (`get_page_text`):

```
S1: PASS — probe.poll(21) -> 42 (Promise=true) in 93.60ms
S2: PASS — awaitEcho(50,777) took 89.40ms, result=777, setInterval(5ms) ticks=17 while pending
S3: PASS — spawnDetached(80) export resolved at t=0.90ms value=1 (see host-shim slowEcho DONE log for actual detached completion time)
S4: PASS — readBody() -> 5 (expected 5)
==== DONE, overall=true ====
```

Console (`read_console_messages`) independently confirms the S3 timing claim — `slow-echo(80,..)`'s
own host-shim `START`/`DONE` lines show `elapsed=84.20ms` for the detached call, while the export's
own resolution (`exportDoneAt`) was `0.90ms` — roughly 90x faster, impossible unless `task.return`
genuinely fired before the spawned sibling drained. Confirmed the browser's JSPI support directly:
`typeof WebAssembly.Suspending === "function"` and `typeof WebAssembly.promising === "function"`,
user agent `Chrome/148.0.7778.280 Electron/42.9.2` — Chromium, JSPI on by default, matching the
ticket's own prediction ("JSPI is default-on in Chrome").

### a real, non-obvious fix required to get the browser run working at all

First browser load failed with an opaque `WORKER ERROR: undefined undefined undefined` (browsers
redact cross-context Worker `ErrorEvent` details) and zero captured network requests. Root cause,
found by re-reading `jcoprobe.js`'s own import lines: jco emits **bare npm-style specifiers** —
`import { ... } from '@bytecodealliance/preview2-shim/cli'` — for the WASI CLI/clocks/io shims a
wasip2 component always pulls in (even one with zero real file/socket/stdio usage, since
`wit-bindgen`'s wasip2 adapter still imports these unconditionally). Bare specifiers resolve fine
under bun/Node (their module resolvers walk `node_modules/`), but a browser's native ES module
loader has no such fallback and fails outright. **Fix**: vendor
`node_modules/@bytecodealliance/preview2-shim/dist/browser/*.js` (a browser-specific build the
package already ships) into the output directory and rewrite the bare specifiers to relative paths
— exactly what `🌉️plugin-web-materialize.ts`'s existing `rewritePreview2ShimImports` function
already does in production. **This spike independently validates that step is load-bearing, not
optional** — without it, the plugin Worker cannot even import the transpiled module in a real
browser. No change needed to that production function; this is confirmation it's doing the right
thing, not a gap found in it.

## what `🌉️plugin-web-materialize.ts` will need to change

Read `transpilePluginComponent` (~line 363) and `pluginComponentBridgeSource` (~line 295) before
writing this section, against what was actually observed transpiling a fully-`async func` world:

1. **JSPI must be feature-detected BEFORE spawning the plugin Worker, not discovered by a failed
   call.** The failure mode is not "a call rejects" — it's the entire `import()` of the transpiled
   module throwing synchronously at top-level evaluation (`new WebAssembly.Suspending(...)` inside
   unconditional trampoline setup code, confirmed at `jcoprobe.js:9393` under plain Node 24). Any
   engine without JSPI kills the whole Worker before a single plugin function is ever called. Given
   the real schema is moving to `async func` everywhere, this is no longer a hypothetical for some
   future plugin — it will be true of every plugin the moment this migration lands. The
   materialize/activation pipeline needs an explicit `typeof WebAssembly.Suspending === "function"`
   gate (or equivalent capability probe) with a real user-facing failure path for browsers without
   it, not a bare uncaught exception surfacing from inside a Worker.
2. **`transpilePluginComponent`'s current bare `jco transpile` call (no `--async-mode` flag) is
   already the right one to keep** — confirmed the flag is a byte-identical no-op for a component
   that's already fully component-model-async lifted, so there's nothing to add or change there for
   the async-only schema migration itself.
3. **`rewritePreview2ShimImports` stays required and is already implemented correctly** — this
   spike is independent confirmation of that, not a new requirement.
4. **`pluginComponentBridgeSource`'s destructure shape is confirmed correct for jco 1.27**: a world
   exporting a single interface `probe` produces `export { probe010 as probe, ... }` — the
   interface's own (camelCased-if-multi-word) name, matching the doc comment's assumption
   `const { reactor, jobs, checkpoint, describe } = await import(...)`. Function names inside each
   interface object are camelCased from kebab-case (`spawn-detached` → `spawnDetached`,
   `read-body` → `readBody`), also matching what the existing bridge code already assumes
   (`stepJob`, `cancelJob`, etc.).
5. **Once the real schema lands `jobs-async`/`checkpoint-async`** (per `terra-probe-spikes-report.md`'s
   S9 finding, already flagged there for the coordinator to re-specify), `pluginComponentBridgeSource`'s
   destructure and `createActorApi()` field names need updating to match
   (`jobsAsync`/`checkpointAsync` instead of `jobs`/`checkpoint`) — cross-referencing, not
   re-deriving that finding here.
6. **Every exported/imported function returns a `Promise`** now (confirmed via the emitted
   `.d.ts`: `poll(n: number): Promise<number>` etc.) — `pluginComponentBridgeSource`'s existing
   plain (non-`async`) wrapper methods (`poll: (events, budget) => reactor.poll(events, budget)`)
   already work correctly with this because a plain function returning a `Promise` propagates it
   transparently to any `await`ing caller; no change needed there either.

## honest gaps

- **Firefox was never tested — no Firefox available in this environment (Browser pane is
  Chromium-based only).** Per the ticket's own fallback instructions, tried to get a Firefox-shaped
  data point by verifying the generated JS is JSPI-free under a plain transpile — **that
  fallback does not work**: the generated JS is NEVER JSPI-free for a fully async-lifted component,
  regardless of `--async-mode`. The Node-without-`--experimental-wasm-jspi` run is offered instead
  as the actual JSPI-off data point — a different JS engine (V8, not Firefox's SpiderMonkey) but a
  genuinely JSPI-unavailable one, with a real, reproduced, non-fabricated failure. This is weaker
  than an actual Firefox test (SpiderMonkey's exact failure mode for a JSPI-dependent script was not
  observed) but is real evidence, not a docs claim.
- **`--async-wasi-imports`/`--async-wasi-exports` and `--async-imports`/`--async-exports` flags were
  not explored** — irrelevant to this component (no WASI-interface function needed async treatment;
  none of the pulled-in `wasi:cli`/`wasi:io`/`wasi:clocks` functions this component uses are async in
  its WIT), but a plugin using real file/socket I/O might exercise a different code path not tested
  here.
- **`--minify`/`--optimize` (the ship-mode `wasm-opt` pass `plugin-web-materialize.ts` runs) were not
  tested against this component** — no reason to expect JSPI-dependence to change under
  binaryen optimization (it's JS-side glue, not touched by `wasm-opt`, which only rewrites the core
  wasm modules), but not empirically confirmed here.
- **A harmless Bun-specific console error was observed and not root-caused**: `error:
  process.binding("tcp_wrap") is not implemented in Bun` appeared mid-run in the bun harness
  (`terra-jco-bun-run1.txt`) but did not stop execution — S2 through S4 still ran and the process
  exited 0. Likely `@bytecodealliance/preview2-shim`'s `wasi:clocks`/`wasi:io` polyfill probing for
  a Node-only API Bun doesn't implement. Did not chase further since it's non-fatal and orthogonal
  to the JSPI question; flagged for whoever picks up the real Worker runtime to be aware of.
- **Multi-interface export worlds were not tested** — this probe's `world jcoprobe` exports exactly
  one interface (`probe`), unlike the real `world actor`, which exports four
  (`reactor`/`jobs`/`checkpoint`/`describe`). The destructure-shape claim above (point 4) is
  extrapolated from jco's naming convention observed here plus the single-interface case, not
  directly confirmed for a 4-interface export world.
- **The `pure` import's disappearance from the componentized wasm** (dead-code-eliminated because
  never called) was observed but not chased into whether it causes any transpile-time or run-time
  difference — since real plugins DO call `pure.log`, this is very unlikely to matter in practice,
  flagged only as a debugging-time surprise.

## files touched (all within owned paths)

- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.toml` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/🧬️schema/📜️world.wit` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/🦀️component.rs` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/🌐️harness/serve.ts` (new — static file server for the browser test, launched only via `preview_start`)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/🌐️harness/out-callback/` (new — jco transpile output, `host-shim.js`, `run-node.mjs`, `index.html`, `worker.js`, vendored `preview2-shim/`)
- `.claude/launch.json` — appended ONE new entry, `"terra-jco-spike-static"` (port 8846); the three
  pre-existing entries (`cad-react`/`s-react`/`dag-react`) were not touched.
- Ticket-folder logs: `terra-jco-guest-build1.txt`, `terra-jco-guest-build2.txt`,
  `terra-jco-guest-build-final.txt`, `terra-jco-component-wit.txt`, `terra-jco-transpile-callback1.txt`,
  `terra-jco-transpile-callback2.txt`, `terra-jco-transpile-jspi-explicit.txt`, `terra-jco-bun-run1.txt`,
  `terra-jco-node-nojspi-run1.txt`, `terra-jco-node-jspi-flagged-run1.txt`.

No files outside these paths were modified. `🎯️target-s1/cargo-install/bin/wasm-tools` (another
packet's build artifact) was invoked read-only for `validate`/`component wit`, never written to.
