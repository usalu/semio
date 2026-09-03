# 🧪️ Runtime verification — `dev sourcing` on :6081

## Boot, before any of this ticket's fixes
`bun run dev:sourcing` exited 1. The Vite host itself was fine; the sourcing wasm in
`🔌️plugin-modules/sourcing/` was **five days stale** (Aug 26) because the crate had stopped compiling,
so the browser was loading a plugin built against a different framework.

## Boot, after the compile fixes + a fresh `plugin sourcing` build (wasm 12:30, descriptor 12:35)
The shell comes up and the app mounts: title `semio · sourcing · curation`, Editor mode, the `Demo`
example selector, and all four windows laid out — **Pool**, **Curated**, **Grid**, **Preview**.
That is already past every failure mode the plugin used to have.

Every window body is still EMPTY, and the console reports, per boot:
```
[DEBUG] PluginRuntime: turn failed for actor sourcing#1
[DEBUG] render failed
[DEBUG] readConflicts failed
[DEBUG] action failed setActiveExample {exampleId: demo}
[DEBUG] program load failed stdio   ← pre-existing, see below
```

## Diagnostics gap found and fixed on the way
Every one of those faults arrived as the string **`[object Object]`** — in the console, in the host,
and rendered verbatim into the on-screen error surface. `replyError`
(🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts:172)
did `error instanceof Error ? error.message : String(error)`, and a guest plugin rejects with a
**lifted fault record** — a plain object, never an `Error`. So the single most common failure in the
system reported nothing at all. It now serializes the record with `JSON.stringify`.

The shard worker is only republished as a side effect of a full `plugin <id>` build (a ~7-minute
cargo cycle), so `republish-shard-worker.ts` in this ticket folder rewrites
`🔌️plugin-modules/_shard/🟨️shard-worker.js` from the current `shardWorkerSource()` directly.

## Not sourcing's: `semio-s-plugin-stdio` does not link
```
error: linking with `wasm-component-ld` failed
  failed to encode component / module was not valid
  functions count exceeds limit of 1000000 (at offset 0xdd4)
```
`🔌️plugin-modules/stdio/` consequently has no `🔣️descriptor.json` at all (its last successful build
was Aug 18), and the OS reports `plugin.descriptor-invalid: /plugin-modules/stdio/🔣️descriptor.json
returned HTML` on every boot. This matters to sourcing because `CurationSnapshot::catalog` is an
`ArtifactChild<SemioKitSnapshot>` composed from `s.stdio.semio.kit` — the pool's stock is joined out of
that child plus sourcing's own `stock_extra`. It is a peer-owned crate and a peer-owned break.
