# First Real GIS Browser-Actor Readiness

## Result

There is no current real GIS component, extracted core, generated WIT, or closed actor ESM on disk. A read-only workspace scan for `semio_s_plugin_gis.wasm`, `semio_s_plugin_gis*.js`, `semio_s_plugin_gis*.wit`, and `closed-actor.mjs` returned no files. Consequently, neither the real GIS import manifest nor a real GIS browser instantiation is evidenced. The existing actor-factory and actor-import fixtures qualify the compiler/runtime seams only; they are not GIS execution.

The source is ready to make the first authoritative decision. It snapshots a fresh GIS component before derivation, runs the pinned JCO compiler over those bytes, and fails closed unless every *actual generated* interface belongs to the fixed browser policy. It is therefore safe to run the existing materialization gate once capacity is available; it is not safe to infer in advance that GIS has only the allowed imports.

No build, JCO invocation, or browser execution was run for this audit.

## Static closure versus unproved GIS closure

| Surface | Current source evidence | Real GIS result |
| --- | --- | --- |
| Component identity | GIS is `semio-s-plugin-gis`, package `semio:gis`, a `cdylib`, and enables the plugin crate's `component-guest` feature. [Cargo.toml](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:66) | No fresh `.wasm` was found. |
| Declared component world | `world actor` imports only `semio:framework/pure` and `semio:framework/host-async`; it exports `reactor`, `jobs`, `checkpoint`, and `describe`. [📜️.wit](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit:1319) | The producer will inspect required exports, but that check does **not** reveal transitive Preview2 imports. |
| First-party host import shape | The browser host supplies all currently declared `host-async` operations plus the three `pure` calls, with one per-activation pending/effect/stream owner. [🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️host/🟦️.ts:155) | Source-compatible for the declared WIT interface, but no production worker mounts a `BrowserHostPort` or returns actual Hub effect completions yet. |
| Preview2 host | The first-party WASI activation supplies exactly fourteen Preview2 interfaces and has no filesystem, socket, random, or wall-clock authority. [🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🌐️wasi/🟦️.ts:8) | Unknown. A GIS output requesting `wasi:filesystem/*`, `wasi:random/*`, `wasi:clocks/wall-clock`, or another interface must be rejected, not admitted by widening this list. |
| Pinned JCO policy | The allowed manifest universe is the two Semio interfaces plus those fourteen WASI interfaces. The fixed async imports are the eight blocking poll/stream methods. [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:22) | No real JCO manifest exists yet. |
| Generated factory closure | The build checks the JCO result's actual imports against that allowlist, validates emitted JSPI suspension when WASI is used, and requires the generated source's literal imported-interface accesses to match its manifest exactly. [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:299) [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts:403) | Ready to reject a mismatch before closed bytes are published; no GIS run demonstrates success. |

The `semio:framework/actor-import` fixture's 15 imports and the trusted-catalog JSON fixtures' two Semio imports are intentionally synthetic. They cannot establish the GIS component's Preview2 import set.

## Exact first real build boundary

The existing integrated gate—not an ad-hoc cargo/JCO pair—is the first useful execution boundary:

```sh
SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/first-real-gis-actor' \
  bun ./📜️script.ts trusted-stdio-gis-bundle-check --native
```

Run it with the working directory [Hub Rust package](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust). This is intentionally a later, expensive execution step; this audit did not run it.

The gate first builds `os-hub`, then materializes both prerequisite stdio and GIS packages. For GIS it invokes the canonical Wasip2 `cargo rustc -p semio-s-plugin-gis --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-release`, at an isolated ticket-owned target, snapshots the output, extracts its core and WIT, emits/verifies its descriptor, and derives the closed actor while the source component lease remains live. [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:429) [📜️script.ts](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts:444)

The result is admissible only if all of the following are observed from the resulting immutable generation:

1. `packages/gis/component.wasm` and `descriptor.semio` match the fresh receipt, and the WIT includes the four actor exports.
2. `packages/gis/browser/closed-actor.mjs` exists; its catalog `browserActor` is `closed-browser-actor`, names the pinned policy, and binds its SHA-256/length, source-component hash, source-descriptor-byte hash, policy hash, and sorted actual `importInterfaces`.
3. Those interfaces are the JCO-emitted set, not a caller-provided list; any unsupported interface or missing JSPI suspension stops the build before the ESM is staged. The materializer passes the raw leased GIS bytes directly to `buildClosedBrowserActorArtifactV1` and persists the derived bytes only after that succeeds. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5109)
4. The candidate Hub loads the generation and issues its authenticated selected GIS Map plan. This proves catalog/descriptor/plan readiness only. The gate explicitly does not claim client execution. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5974)

If the run rejects due to an import, retain the exact import list/error as ticket evidence and decide whether the dependency is genuinely required. Do not extend the browser policy based on a fixture or a generic Wasip2 assumption.

## Dependency order after the absent build output

1. **Materialize and load the real GIS actor record.** Run the native boundary above, then qualify the current mandatory actor record through trusted-catalog loading and the authenticated plan/lease field propagation. The materializer already requests GIS and stages the actor at the fixed `packages/gis/browser/closed-actor.mjs` path. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5096) This is a catalog result, not activation authority.
2. **Contain before serving actor bytes.** The current browser target gate still describes a verified lease followed by `renderer-unavailable`, rather than actor loading. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3852) A dedicated private Worker must own the exact closed bytes, aggregate argument/core-memory admission, deadline and non-cooperative forced termination, and close before the protected actor body route/broker handoff is enabled.
3. **Bind actor effects to the exact Hub scope.** The eventual worker host port must bind `{catalog generation, package, descriptor, browserActor hash, plan receipt/lease, space, document, surface}` before any `host-async.document-write` or proposal effect is sent. The source host has a bounded per-activation effect owner, but it is not wired to a Hub completion/outbox path.
4. **Make Map approval a real fixed-three durable commit.** The Hub proposal script currently documents that its two-user process is deliberately unrun: the prior materialized profile has never completed and approval remains fail-closed while no typed composition transaction is bound. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:5686) The Store-side `DurableOwnedThreeStoreMapAssemblyV1` is a useful pre-journal owner, but must be connected to the per-document WAL/journal and the Hub approval route before a GIS patch or inference approval can claim durability. [🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:929)
5. **Qualify multi-user revocation alongside—not after—a visual demo.** The source has independent native gates for normalized presence and SQLite-reopen admin removal/target revocation. [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7919) [📜️script.ts](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:7945) Their final composed process must include a real actor-bearing selected target: remove an author, ensure its actor/lease/presence becomes unusable, and ensure a retained peer cannot receive a cross-space effect or Map result.
6. **Only then run the user journey.** Author opens the authenticated GIS Map editor, actor issues one scoped patch/proposal, approval appends the same committed fixed-three group, spectator observes it only after commit, close/reopen reconstructs all three members, and author removal/restart blocks old lease, actor, presence, and approval continuation. Neither a synthetic actor fixture nor a native codec/provider receipt substitutes for this sequence.

## Narrow recommendation

Prioritize the single native materialization command above once the active fleet frees the required target. It produces the missing empirical import manifest and either proves the existing limited host/WASI policy sufficient for *this* GIS component or gives one concrete, fail-closed import incompatibility. Keep the worker/body route disabled regardless of a successful materialization until step 2 is qualified.
