# Native Transport Integration Review

Read-only review of the landed WGPU native renderer stream, its component-close bridge, and every production HttpBody implementation. No build was run. The known pre-head and local ReadPage mid-await gaps are excluded.

## Result

The plain HTTP path has exact request ownership and a close of A does not cancel B. Two new source-proven issues remain:

| Finding | Result |
| --- | --- |
| Exact A transport cancellation does not affect a different B request | Verified |
| Plain-socket cancellation avoids the blocked body-state mutex | Verified |
| Busy owner handback restores the exact fetch | Verified |
| HTTPS stalled body keeps A close and B’s fetch lane blocked until its read deadline | New defect |
| Any pending component close stops new frame building for every window | New control-flow defect |

## Verified ownership boundary

The lease records the World surface token and World asset request token. The guard removes a lease only when both still match, then wakes the host. Close returns Terminal without invoking the body cancellation handle when its requested surface or request differs.

- [Lease fields](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11168)
- [Guard’s exact-match drop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11193)
- [Exact close filter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12424)
- [Busy-safe handoff](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12357)

A guard release before handoff cannot lose the external fetch: World asset close refuses an in-flight claim, and the later handoff returns the exact owner or restores it on a busy runtime.

- [In-flight World close guard](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15159)
- [Exact World owner return](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:15034)

The stream checks cancellation immediately before and after every HTTP next-chunk await, including EOF before success handoff. A cancellation between those checks can retain at most one bounded page; seal then detects noncurrent ownership, begins close, and drains it rather than publishing it.

The existing native plain-HTTP regression already exercises a real stalled A followed by B and requires zero terminal leases: [renderer law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:399). The underlying socket test independently proves interruption of the blocked read and a usable sibling request: [service law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs:1179).

## HttpBody census

| Body | Renderer route | Cancellation | Result |
| --- | --- | --- | --- |
| BufferedHttpBody | No; legacy blocking transport only | Idle; immediate buffered chunk | No stream-close issue |
| SocketHttpBody | HTTP | Interrupt via cloned TcpStream shutdown | Correct |
| UreqStreamingHttpBody | HTTPS | Read deadline only | Defect below |
| LocalSocketBody | Test-only | cloned-socket interrupt | Test-only oracle |

RuntimeMailbox selects SocketHttpTransport for HTTP and UreqStreamingHttpTransport for HTTPS: [construction](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:11384).

SocketHttpBody’s blocked read owns its state mutex, while the interrupt closure owns a separately cloned TcpStream. Component close therefore does not contend for the blocked read lock: [body read](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:1163), [independent shutdown handle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:1239).

## New defect: HTTPS body cancellation is deadline-bound

UreqStreamingHttpBody is the selected HTTPS body. It exposes a ReadDeadline cancellation handle, whose cancellation result is AwaitingReadDeadline rather than an interrupt. Its owned reader remains locked while the blocking Read operation runs.

- [Ureq body](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1576)
- [Deadline-only cancellation handle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1619)
- [Blocking page read](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs:1675)
- [Cancellation result semantics](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:973)

For a World A HTTPS response whose head is received but whose body stalls, component close cancels the child token and retains AwaitingReadDeadline. The native fetch remains active until the Ureq read returns, and B cannot enter the renderer’s capacity-one fetch lane. This is distinct from the acknowledged pre-head gap and the local ReadPage gap: the body lease exists and it is the selected production HTTPS implementation.

Fail-first law: make an owned Ureq-style reader withhold its next page, close A, and require B’s request admission plus zero A publication within the existing native asset observation deadline. The source will wait for the read timeout. The service test can own the deterministic stalled-reader portion; the renderer test should consume the existing native-asset-response fixture and prove A’s exact World token never seals.

Repair boundary: give the HTTPS body an exact operation that can unblock a started read, or preserve that external read owner outside the component-close barrier while retaining its exact handback. AwaitingReadDeadline must remain nonterminal because the reader and pool admission are still live.

## New defect: component close globally blocks new frame construction

Every native redraw first pumps asset work. If advance_component_surface_close reports incomplete work, Winit invalidates RESOURCE_READY and returns before input draining, worker-frame admission, and snapshot publication.

- [Global return before event and frame work](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:239)
- [Close result meaning](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694)

A pending A close therefore blocks new B interactions and new B snapshots until A finishes. redraw_core still calls present_snapshot after this early return, so the last accepted B snapshot remains visible. It cannot reflect a new B event or render result. This is source-proven control flow; this audit has no measured OsHost receipt.

Fail-first law: retain an A component close in an external-wait state, enqueue B input, run one native redraw, and require B’s event admission and snapshot revision advance while A’s close token remains nonterminal. Also require no access or publication through A’s host token. Current code returns at the close gate, so this should fail.

Repair boundary: retain and advance one close unit with RESOURCE_READY wake, but do not make a component-local external wait a global frame-build barrier. Frame admission needs a target-aware participation exclusion for the closing host. It must preserve B work while preventing any new A resource/presentation use; a generic skip of close would violate the ownership boundary.

## Known exclusions

- Cancellation before HttpPool fetch has produced a body/lease.
- Native local ReadPage cancellation during an already-started platform read.
- The plain HTTP post-head socket law, which is covered and sound.


