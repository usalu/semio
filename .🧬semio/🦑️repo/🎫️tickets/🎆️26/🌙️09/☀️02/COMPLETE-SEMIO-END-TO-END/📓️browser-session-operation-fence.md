# Browser Session Operation Fence

## Scope

Inference submission, reconciliation, polling, cancellation, approval, and approval Undo must remain owned by the exact accepted browser session authority under which they began. A browser-broker proof is a ratcheting transport credential, so an ordinary successful proof advance must not rotate operation ownership. Proof invalidation, proof replacement, broker-port detachment, or an accepted `/me` response with a different binding or authorization generation must rotate ownership even when no prior `/me` authority had been accepted.

## Production Boundary

- A worker-private opaque token identifies the browser session-operation epoch.
- Opening an inference port requires a live proof and an unexpired accepted `/me` authority.
- Each inference operation and approval Undo owner captures the opaque token, session binding digest, and authorization generation.
- Every broker admission and every post-await application validates that immutable capture.
- Ordinary proof ratcheting retains the token.
- Authority retirement preserves a sealed inference request as indeterminate and capacity-blocking; it cannot reconcile, poll, cancel, approve, or Undo under a successor proof.
- A submitting approval Undo becomes non-retryable on authority retirement so a successor authority cannot reuse its idempotency owner.
- An unchanged `/me` authority may refresh without rotating the token. Because expiry participates in the binding digest, a response that changes expiry under the same digest remains a contradiction and is refused.

## Neutral And Independent Oracles

The GIS inference neutral fixture contains `absent-authority-refused` and `sealed-request-proof-replacement`. Its schema is compiled by Ajv, and the fixture shape is independently compared with `fast-deep-equal`. The worker law verifies that pre-`/me` inference is refused without a request, that null-authority proof replacement rotates the token, and that a sealed indeterminate request produces no successor request or reconciliation and blocks a new opening.

## Execution Evidence

- Focused new law: session `79763`, one passed and 330 skipped, exit 0. Log: `🗑️generated/pre-me-authority-current.log`.
- Broad inference baseline after the first implementation: session `71784`, 10 failed, 12 passed, and 309 skipped. The token comparator incorrectly required the current proof bytes while `browserBrokerFetch` had temporarily consumed them during a legitimate ratchet, converting ordinary results to `indeterminate`. Log: `🗑️generated/pre-me-authority-inference-cohort.log`.
- Corrected broad inference cohort: session `33916`, 22 passed and 309 skipped, exit 0. The comparator now validates the immutable session token and accepted authority while proof possession remains an opening-only requirement. Log: `🗑️generated/pre-me-authority-inference-cohort-green.log`.
- Direct browser-actor/Undo verification remains separately recorded after its current-source run completes.

## Claim Boundary

This qualifies the TypeScript worker inference and approval-Undo authority fence. It does not qualify a native/browser process or any native WGPU inference owner. Directory bootstrap-page authority correlation and Shell periodic `/me` refresh are separate root-owned boundaries.
