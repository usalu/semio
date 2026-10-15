# Canonical Process Integration Independent Review

Read-only Source audit, 2026-10-15. No production edits, gates, Originals or runtime operations. Exact seams sent to Storage. This is an actionable callgraph inventory, not proof all supported calls have been runtime qualified.

Process root: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`. Library root below: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`.

## Actual births and same-process nested receivers

The current process owner constructs a fresh controller/deadline/handlers on every `withScriptProcessEnvelope` (31–48); receive parses its supplied environment then calls that owner (52–55). It currently has no once-only private root identity. A direct replacement that rejects every second receiver breaks existing real helper callgraphs, not just neutral tests.

Supported authored/carried root convergence points are library `🟦️.ts` 908–912, Native entrypoint `🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts` 16–18, build CLI `🏗️build/📜️script.ts` 186–192, bootstrap `⚡️caching/🚀️bootstrap/📜️script.ts` 606–621 and OS browser bundle `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` 911. Bootstrap dashboard and Nx branches are alternative actual dispatches, not two births executed sequentially there.

Concrete real nested receivers requiring exact-original reuse/derivation rather than another root:

- Compiler store `⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts` 42 reparses an envelope inside callers already owning an invocation, then opens tools/toolchain/Rust image.
- Tools provision `⚡️caching/🚀️bootstrap/🛠️tools/📜️script.ts` 163 receives again while already provisioning under a parent invocation, to derive clone control.
- Selected Native snapshot `🏗️build/📥️inputs/🎯️selected/📦️snapshot/🟦️.ts` 47–49 receives inside installed Native owner execution, then opens graph image and snapshot reader.
- Graph publisher `⚡️caching/🕸️graph/📤️publication/🟦️.ts` 58 receives for an already owned publisher body; same file 34 is an actual exported installed boundary.
- Library `🟦️.ts` 1175 Cargo dispatch and bootstrap dependency execution `⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts` 24 receive inside tool execution and then open preparation/compiler storage.
- Frozen tooling use `⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🟦️.ts` 48–53 is an installed actual module boundary which receives, opens tooling and retains its SDK reader. Supported installed hooks must converge with other same-process helper scopes rather than register a copied object.

Do not classify these as neutral schema tests: their callback bodies perform real Native opens. Conversely isolated schema/envelope constructors can remain pure validation APIs without issuing canonical namespace authority. Actual OS child process/isolate entry may create its own privately authenticated child birth; invoking an in-process helper with carried text is not a new OS child.

## Daemon is a concrete promotion seam

`⚡️caching/🕸️graph/🏁️startup/🚦️service/🟦️.ts` 60–66 currently executes **two sequential roots in one daemon OS process**: finite parent-witness authentication under tools at 62; then a new authored cache-only service envelope at 64–66. Service opens tools, Rust/Nx and exact SDK/namespace custody at 66–71 and acknowledges acquisition only after another live-parent check. Service is intentionally a distinct startup-to-long-lived policy, not a renewed finite task loan.

Smallest honest seam: one private daemon startup birth, one concrete typed service promotion request carrying the existing closed birth/handoff/witness and authored ServicePolicy/ServiceCapabilities, authorized only after the exact physical witness + fresh endpoint response at 31–43. Promotion must bind repository versus watched workspace, toolkit/generation, directory, watcher/graph/implementation identities, finite cache scopes, and the actual startup ownership phase. One successful transition only; no generic capability callback or serialized receipt may bless arbitrary service objects. Preserve all pre-acquisition failures and full actual resource drainage before root retirement. Ordinary capability narrowing must never offer this service-duration transition.

Journal reader at 125–131 is a **worker isolate**: same OS PID but independent JS module registry, actual threadId and exact live service reader token. It validates ServicePolicy/Capabilities and deployment equality before envelope at 128, then performs fresh token response verification before Nx/SDK opens at 129–130. Scope uniqueness per isolate must not confuse this supported worker loan with a second main-thread service root. Later requests use ownedDaemon.request (74), not new service envelopes; preserve that distinction.

## Concrete capability projection seams

Bootstrap `📜️script.ts` 589–600 creates `{...parent,capabilities}` at 597 after rebuilding Nx capabilities from args/environment and optionally carrying original native grant at 592–595. Build `🏗️build/📜️script.ts` 80 does another parent copy with authored build/ancestor capabilities. Neither structural copy preserves future private invocation identity.

Add concrete repository-owned projection schemas/validators at those exact sites: parent and projected root/buildMode, selected executable/arguments, tool/native grants, network, transport and storage must be checked as an explicit subset of the same parent authorization. Bind a privately derived child record to that exact live root/control and no-later deadline. Keep capability semantics in repository owners; process core validates a closed accepted derivation result and provenance, never arbitrary object registration or a user-supplied “trusted validator” callback. New command selection cannot broaden merely because createNxBootstrapCapabilities rebuilt a well-formed object.

## Generic freeze and boundedness

Current envelope schema `🏃️process/🧬️schema/🔣️.json` requires only `capabilities: object`; Script invocation check merely requires a record (`📥️invocation/🟦️.ts` 33–36). It provides no aggregate bytes, object entries, depth, cycle or accessor/prototype contract. Staged issuer recursive freeze uses Object.values recursively without a visited set or depth/entry limits. Direct authored capabilities with a cycle can overflow recursion; repeated shared subgraphs can multiply traversal; accessors/non-JSON values are not authenticated immutable data merely by Object.freeze. JSON carried envelopes cannot contain cycles, but can still be excessively deep/large.

Before applying generic deep freeze at every real root, admit a finite plain JSON-data capability tree with closed entry/depth/byte ceilings (or existing concrete stricter schema), reject cycles/accessors/unsupported values, and freeze a privately captured bounded representation. Do not silently normalize capability semantics or call getters while blessing authority. Pure readScriptProcessEnvelope acceptance must remain distinct from actual process birth issuance.

## Integration qualification

Required concrete laws include one root plus compiler-store/snapshot/installed hook nested usage, duplicate-root refusal, structural projection refusal, valid closed Nx/build narrowing, widened grant refusal, one authenticated daemon promotion, stale/replayed handoff refusal, and worker-isolate loan versus main-root rebirth. No runtime result is claimed by this inventory.
