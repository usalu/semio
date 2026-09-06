# `fetchWithTimeout` Header Contract and Response-Body Ownership

## Verdict

Keep `fetchWithTimeout` as a **header-acquisition** primitive. Its lifetime
must not silently include every returned response body. The clean boundary is
an explicit, consumer-owned bounded body reader immediately after a successful
header fetch; for the actor path, root's private execution-target reader is the
right owner. There is no current first-party generic reader safe enough to
reuse unchanged.

No build or runtime test was run for this audit.

## Current contract is deliberately header-scoped

`fetchWithTimeout` returns the native `fetch` response immediately when the
fetch promise resolves and clears both timer and external abort listener in its
`finally` ([`framework/🟦️.ts:347-364`](../../../../../../🧰️framework/📦️packages/🟦️typescript/🟦️.ts#L347-L364)).
The structural public return type intentionally has only headers, status,
`json()`, and `text()`—not a body stream or binary method
([lines 322-337](../../../../../../🧰️framework/📦️packages/🟦️typescript/🟦️.ts#L322-L337)).
The existing success law specifically asserts the timer/listener cleanup at
that point ([lines 1711-1731](../../../../../../🧰️framework/📦️packages/🟦️typescript/🟦️.ts#L1711-L1731)).

That matches native `fetch`: resolving a `Response` means headers are
available, not that its body has been consumed. Extending this function's
lifetime through an arbitrary later `response.text()`, `json()`,
`arrayBuffer()`, or `getReader()` is not coherent:

* the function cannot know which body method a caller will select, its correct
  size limit, or whether it will ever be called;
* keeping the timer/listener alive would leak them when a caller only inspects
  status (all successful writes do that); and
* consuming the body before return would make the returned real `Response`
  unusable. Cloning/rewrapping would create unbounded duplicate buffering and
  alter stream semantics.

The appropriate API distinction is therefore already present: header fetch
transfers the still-open response body to the immediate caller. Document that
transfer explicitly; do not alter `FetchTimeoutResponse` into an implicit
long-lived response-body owner and do not make all callers inherit an actor
component's 64 MiB policy.

## Exact production consumer census

There are thirteen lexical references: one implementation, three framework
tests, and these nine production call sites.

| Consumer | Body ownership after headers | Consequence |
| --- | --- | --- |
| [`readBackboneEnvelopeOnce`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:242) (two GET branches) | `arrayBuffer()` under a retry/document signal | Separate bounded reader needed before its documentation can call the retry window a complete read deadline. |
| [`writeBackboneEnvelope`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts:308) (two PUT branches) | status only | Header ownership is exactly right; no body reader. |
| [`browserBrokerFetch`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:387) | returns the live native response after proof rotation | Must remain header-only; its callers have incompatible JSON/asset policies. |
| [`pollFolderOnce`](/Users/ueli/Documents/semio/🛍️products/💻️os/🧵️backbone-worker.ts:1207) | `arrayBuffer()` under `docAbort` | Separate P1: close after headers can still leave the folder body pending. |
| [`writeFolder`](/Users/ueli/Documents/semio/🛍️products/💻️os/🧵️backbone-worker.ts:1306) | status only | Header-only is correct. |
| [`getCachedBlob`](/Users/ueli/Documents/semio/🛍️products/💻️os/🧵️backbone-worker.ts:2889) | `arrayBuffer()` in global cache | Separate P1: its comment that the fixed timeout prevents a stalled server is only true through headers. It has no document cancellation and is not currently exposed. |
| [`putCachedBlob`](/Users/ueli/Documents/semio/🛍️products/💻️os/🧵️backbone-worker.ts:2909) | `json()` reply | Separate global-cache body policy, currently unexposed. |
| execution target manifest/component/descriptor through `browserBrokerFetch` | streamed private bytes | P0 for the upcoming actor body; repair locally now. |
| GIS inference JSON through `browserBrokerFetch` | `text()` under operation abort | Separate P1 for cancellable inference: an abort after headers does not presently interrupt `text()` ([`readInferenceJson:2566-2573`](../../../../../../🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts#L2566-L2573)). |

The first-party codebase therefore does not have one uniform body lifetime or
bound to move into `fetchWithTimeout`. Header-only writes, bounded trusted
component reads, document envelopes, global cache blobs, and JSON error/status
responses need distinct ownership and cancellation behavior.

## Existing reader candidates are not reusable as-is

`readLocalRelayBody` in Hub's Bun relay does attach abort cancellation and
releases its reader ([`Hub script:380-414`](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L380-L414)). It is nonetheless not a drop-in:
it consumes an inbound `Request`, has no total deadline or post-EOF state
fence, retains source chunks without wiping them, and belongs to the local
relay runner rather than the browser worker. Its sibling response reader omits
a final release entirely ([lines 416-440](../../../../../../🌎️hub/📦️packages/🦀️rust/📜️script.ts#L416-L440)).

The WGPU frame worker's BYOB reader is deliberately bound to renderer page
credits and runtime `pushAssetResponsePage` ownership
([`frame-worker:275-293`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts#L275-L293)); importing it into Backbone would either duplicate its renderer admission or discard the invariant that makes it safe. It is not generic infrastructure.

## Minimal scoped action

1. Preserve the public `fetchWithTimeout` implementation and its current
   timeout/listener-release law. Amend its docstring to say the returned
   response body is now caller-owned, rather than implying a full response
   lifetime.
2. In `backbone-worker.ts`, add one private bounded stream owner for the
   execution-target manifest/component/descriptor and future actor body, as
   specified in
   [`terra-browser-execution-target-body-read-ownership.md`](./📓️terra-browser-execution-target-body-read-ownership.md).
   It owns the deadline, abort race, lock release, source-chunk wipe, and
   exact state/lease revalidation; it does not change framework behavior.
3. Keep the folder/backbone/blob/inference body gaps as independently scoped
   follow-ups. Do not claim their existing request timeout covers body bytes.

One small framework law is useful but does not change runtime behavior:
`fetchWithTimeout_returns_open_response_at_header_completion`. Use a real
deferred `Response` body; require `fetchWithTimeout` to resolve and remove its
external listener at headers, then prove the caller—not the helper—can choose
to consume/cancel the body. This locks the intended ownership transfer and
prevents a future global “fix” from silently consuming bodies.
