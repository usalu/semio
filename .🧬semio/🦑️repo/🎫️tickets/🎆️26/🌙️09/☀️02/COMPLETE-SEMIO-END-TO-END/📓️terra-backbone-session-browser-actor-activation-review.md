# Backbone Session-Bound Browser Actor Activation Review

## Verdict

The reviewed private activation shape is coherent: a child is reserved only from the current private lease, started only after the accepted socket `Session`, and every asynchronous body/load/describe boundary revalidates the current socket, actor, lease, grant, scope, selected target, and expiry. The explicit `sourceSocket` argument to `handleHubFrame` is an important fence: a delayed frame from a replaced socket exits before it can establish Session authority.

The `renderer-unavailable` terminal status remains an honest describe-only milestone. It must not be interpreted as actor rendering, checkpoint hydration, or a permission to enable host effects; the cold-render bridge is recorded separately in `📓️terra-gis-cold-checkpoint-browser-actor-render-current-frontier.md`.

This is a source audit only; no native/build result is claimed here.

## Verified Ownership Chain

| Boundary | Source | Current protection |
| --- | --- | --- |
| Grant admission | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:649-656,1274-1284` | A module-private lease retains the open-plan owner callback and binds the socket-grant actor/expiry before publication. |
| Session selection | `backbone-worker.ts:2011-2013,2129-2156` | Actual websocket dispatch supplies `sourceSocket`; old sockets are ignored. The exact pending actor must match before `hubActorReady` and the activation kick-off. |
| Reservation | `backbone-worker.ts:995-1027,1084-1102` | Reservation is installed before capacity allocation; reserve completion validates it is still the exact state/lease/grant and closes its child on any stale result. |
| Body, byte and describe validation | `backbone-worker.ts:664-692` | The private lease performs timeout/abort/current checks before and after fetch, body read, hash, child load and child invoke. Source/result buffers are zeroed in `finally`. |
| Terminal cleanup | `backbone-worker.ts:982-988,1053-1063,1103-1124,1720-1729,3336-3352` | Lease drop closes reservation, aborts the reader/child, clears state only if it still owns it, and connection close/document close route through the same drop. |

The new neutral corpus has twelve meaningful rows at `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️browser-actor-session-v1.json`: before/wrong Session, body and load replacement, same document in another space, body/descriptor faults, and scope/client changes. Its socket-path test drives the real `onmessage → handleHubFrame(..., sourceSocket)` route at `backbone-worker.ts:5827-6017`, rather than calling reservation activation as a standalone helper.

## Root-Identified In-Flight Corrections

The source snapshot initially had two narrow races which the coordinator has already identified and is correcting; they should stay covered in the final reviewed source rather than be reintroduced.

1. Header-time owner invalidation must enter the bounded reader before the next stale assertion, so the response stream's cancellation/release `finally` owns an unread body. The `scope-change-at-headers` vector is the required regression (cancel count one and unlocked stream).
2. Reservation assertions after lease awaits must invoke the original captured open-owner assertion, not only compare state/lease/grant/socket fields. This fences a replacement of client/attempt/binding in the final microtask between a successful `describe` and publication.

## Remaining Focused Acceptance Rows

No further source-proven P0 was found beyond those two active corrections. Add these compact rows before treating the boundary as stable:

1. Duplicate valid `Session` on the same socket after the first accepted one must close/drop and cause no second body fetch or status publication. Current `pendingSocketActorId === null` correctly routes it to the mismatch branch; make that explicit.
2. Socket replacement or `closeArtifactRuntime` after `describe` resolves but before the final status assertion must terminate the child once, retain no reservation/lease, and publish neither `renderer-unavailable` nor a stale `integrity-failed` status for the replacement owner.
3. A status observer that synchronously changes the document owner while receiving `verifying` must be caught by the immediate post-callback assertion, with reader/child capacity returning to zero. This proves status notification is not a re-entrancy publication bypass.
4. A second Session during pending body/read must not transfer the pending body to a new reservation. The stale child must observe one close, the replacement must use a distinct generation and a newly authenticated body.

## Boundary for the Next Renderer Slice

Keep `DocumentBrowserActorReservation` as the sole parent owner. Extend it with the separately reviewed checkpoint-pair and typed turn driver only after `activateBrowserActor` returns a verified descriptor. Do not publish raw child/lease/body state to Shell, construct a second reservation from `sourceSocket`, or let generic `BrowserHostPort` effects bypass the reservation. Child render results must remain parent-owned until they pass the exact generation/scope/surface/receipt check and `UiDocumentStore.applyPatch`.
