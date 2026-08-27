# Exact App Lifetime Close Audit

## Outcome

Source inspection confirms that the existing host `destroyApp` completion is not a native cleanup witness. A query whose original outcome iterator fails cannot safely infer cleanup from that promise or from a generic Idle turn. Dag owns the replacement exact-lifetime close protocol; Fem owns the retained renderer descendants joining it. Implementation and runtime proof remain open.

## Inspected Ownership Chain

| Boundary | Current source behavior | Missing guarantee |
| --- | --- | --- |
| React PluginRuntime adapter, `adaptPluginHandle` | Disposes the AppChannelClient subscription and removes its map entry before awaiting the underlying destroy | The only observer disappears before cleanup is acknowledged |
| React kernel handle, `destroyApp` | Deletes instance/retained/effect mappings, tears down the actor and invokes `shardClient.dispose(actorId)` | No InstanceClose event or joined cleanup completion |
| Native renderer TypeScript plugin bridge | Performs the same subscription/map removal and actor disposal | Same lifetime hole |
| Actor ShardClient `dispose` | Returns void; rejects pending requests, posts an unacknowledged dispose and removes the actor route | Late callbacks can only carry an ID, not the original activation authority |
| Generated plugin web worker dispose | Deletes actor, in-flight-turn and granted-budget entries | No guest close request or final acknowledgment |
| Native plugin reactor InstanceClose processing | Starts runtime/reactor/patch close, advances one bounded cleanup turn | Several UI descendant cleanup return values are discarded; generic MoreWork/Idle is not exact per-instance emptiness |

The coordinator read the actual code at these paths (under `/Users/ueli/Documents/semio`):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧵️shard-client.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`

PluginRuntime contains a literal NUL; source searches must use `rg -a` to avoid treating the file as an opaque binary.

## Assigned Completion Contract

Capture a generation-bound actor route/lease at activation. Retain one idempotent close job on that exact lifetime, with an original route rather than a fresh lookup by actor/instance number. The worker verifies the lease before dispatching InstanceClose. RuntimeCloseWorkerState, ReactorCloseState, patch owners and every per-instance UI descendant must join the terminal witness. Only that exact acknowledgment permits host observer/route/worker release. A reused numeric ID must remain untouched by stale close callbacks.

Ordinary malformed frames and consumer exceptions continue using the original outcome subscription and exact query cancellation token; the independently rerun OS suite passes 216 tests at that checkpoint. Fatal iterator failure/end/turn error requires the close authority above, not a second subscriber or polling loop. Strict language-neutral lifecycle fixtures, delayed/duplicate/stale close receipts and real native/worker execution are required before admission.

No all-app, hard timing, browser or complete lifecycle pass is claimed by this source audit.
