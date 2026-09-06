# Terra Browser Child Worker Current Audit

Status: read-only current-source review on 2026-09-06. I did not run the Chromium gate. Root reports `browser-actor-child-worker-containment-check` GREEN for its fourteen fixture laws; this report treats that as externally reported execution evidence, not a result reproduced by this audit.

## Scoped verdict

The new [`child`](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child) boundary is a coherent **isolated containment foundation**:

- its worker URL is first-party and static ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:57)); the caller cannot select one;
- it transfers and rehashes the actor bytes inside the child before `Blob` import/activation ([child worker](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:27));
- it binds a private port with fresh nonce plus generation, keeps one invocation in flight, bounds transferred values before crossing either direction, and terminates the worker on every parent terminal path;
- the child has a deny-all host bridge. No Hub URL, plan/lease, receipt, credential, or real host callback crosses it.

The registered Hub target is a real Vite + Chromium gate, not Vitest/Node worker emulation ([target](../../../../../../../🌎️hub/📦️packages/🦀️rust/📋️project.json:398), [gate](../../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts:3941)). Its transfer and tight-loop rows are appropriate evidence for this narrow foundation.

It is deliberately not mounted to Backbone, does not fetch actor bytes, and accepts only a caller-supplied admission record. It therefore must not yet be called a verified execution route or a browser security sandbox.

## P0 before production mounting: bind the admission to the retained execution target

`reserveBrowserActorChild` is public and its `actorId`, generation, digest, and length are caller values ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:15)). The source correctly documents that this is not catalog/execution authorisation. Two callers can reserve the same actor/generation, and there is no document scope, verified target lease, policy digest, or closed-bundle graph assertion at this API.

That is safe only while the API is fixture-only. The forthcoming Backbone owner must be the sole constructor, take a private verified body owner (not a URL or a public `ArrayBuffer`), and derive all of these from the retained `DocumentExecutionTargetLease`/actor catalog row:

`runtimeKey, exact document scope, lease generation, actor record digest/length, closed-bundle policy digest, and one fresh activation generation`.

It must hold one exact owner per document runtime key so a duplicate reserve becomes a local conflict, and it must release it only after child terminal retirement. Keep the raw reservation API unexported from a production-facing bundle surface or move it behind a constructor capability at that point. Do not add a body route or activate a bundle merely to exercise this class.

## Byte-identity race: no activation bypass, but add the specific law

There are intentionally two hashes:

1. the parent hashes before transfer ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:76));
2. the child hashes the transferred buffer before it makes the blob URL ([worker](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:31)).

Because an `await crypto.subtle.digest` in the parent leaves a microtask opportunity for another same-thread holder to mutate the original buffer before `postMessage`, the first check alone would be TOCTOU-prone. The second check is the actual activation fence: mutated bytes can reach only the deny-all child, are rejected before import, then the child faults/terminates. I found no path from this race to actor activation or parent authority.

Add one focused adversarial law before catalog mounting: mutate the caller buffer after parent digest initiation but before transfer (a controlled digest seam or deterministic source-level second-hash assertion is acceptable where browser scheduling cannot make the race deterministic). Require no `loaded`, no `activate`, terminal phase, and restored capacity. The law must specifically preserve the child-side rehash; testing only the parent hash would miss the real fence.

## P1: pre-transfer cancellation currently consumes and zeroes the caller buffer

`load` zeros `bytes` in its catch path ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:72)). If abort/deadline occurs while the parent’s asynchronous digest is still pending, no successful transfer has occurred, but the catch still overwrites the caller-owned attached buffer. The existing tests cover successful detachment and bad-digest wiping, not this abort-before-transfer ownership boundary.

Pick and document one exact contract before public use:

- **consuming call:** ownership transfers to `load()` immediately, even while hashing; add an abort-during-digest law that expects the caller buffer to be zeroed; or
- **transfer-point ownership:** do not wipe until `postMessage` actually detached the buffer; add a law that cancellation before transfer leaves it intact, while hash mismatch after a defined consuming checkpoint follows the same rule.

The first fits private verified-byte admission more naturally, but must be explicit in the type/API docs. This is an ownership/API correctness issue, not an actor activation bypass.

## P1: schema fixture is top-level only; hostile wire vocabulary has no declarative corpus

The JSON schema fixes limits and fixture module text, while `binding`, value grammar, and wire protocol are descriptions only ([schema JSON](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🔣️.json:22)). Runtime validation is considerably stronger (`childRecord`, `measureChildValue`, and nonce/generation checks), but no fixture rows currently exercise malformed child replies or stale/duplicate packets.

Before the parent has a real actor body, extend the schema-first corpus with exact expected terminal behaviour for:

1. wrong nonce and wrong generation;
2. duplicate `ready`, `loaded`, `result`, and `transferred`;
3. a result with an extra key, wrong sequence, non-finite/aliased value, oversized buffer, and mismatched detached count;
4. parent `close` racing load and one pending invocation;
5. malformed child `init`/load/invoke shape.

The current state machine is safely fail-closed on such messages: it either ignores an unmatched binding or closes for a protocol violation ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:125)). The missing item is independent executable coverage, not a requested relaxation of the exact parser.

## Other checked lifecycle points

- Constructor failure, boot deadline, abort, message decode failure, worker error, load error, result-bound error, and invoke deadline converge through idempotent `close`; it rejects the retained pending/operation and releases the reservation exactly once ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:100)).
- A result is retained until the explicit child transfer-detachment acknowledgement. Closing in that interval wipes retained received buffers before rejecting, which prevents a partially accepted result escaping ([owner](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:136)).
- The child revokes the blob URL and wipes its received source in a `finally` before activation ([worker](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:34)). Parent termination remains the non-cooperative fallback when guest JavaScript blocks the child event loop.
- The only visible child host methods are deny-all dispatch/log/trace, closed cancellation, and a clock ([worker](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts:20)). This validates isolation for the present fixture but does not satisfy GIS/WASI actor imports.

## Acceptance boundary

The reported Chromium fixture is suitable to qualify: static child provenance, ordinary `ArrayBuffer` detachment, pre-transfer input rejection, output bound, exclusive single flight, forced termination of load and invoke tight loops, and capacity restoration. It cannot qualify real actor byte admission, document/space scope, component policy, host effects, browser heap/cpu quota, or MCP Map execution. Those require the retained Backbone execution-target owner and the pending authenticated catalog/lease handoff.
