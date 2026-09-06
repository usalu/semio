# Bounded Execution-Target Body Ownership

## Current Verdict

The shared reader and sixteen production ownership cases pass in exact registered Nx target `os-hub:browser-actor-document-reservation-check`. This does not establish real GIS actor loading, rendering, Map mutation, undo, or peer propagation.

## Test-Driven Receipts

- Session47521: expected RED, successful Response remained locked.
- Session84713: GREEN, seven neutral stream cases and existing lease/reservation suites (five selected Vitest tests,273 skipped). Both neutral corpora validated with AJV.
- Session58050: expected RED, changing the current document's Hub scope during component delivery still published a live lease.
- Session12911: GREEN, seven stream plus nine production ownership rows, six selected Vitest tests,273 skipped. All sixteen reservation rows and seven lifecycle laws remain included.
- Session17784: GREEN after final failure-cleanup refinement; it clears only this attempt's published lease on a post-publication error. Session26737 GREEN with sixteen ownership cases, including all five stalled plan/manifest/component/descriptor/grant bodies, superseded attempt, and synchronous observer reentrancy.

## Implementation

A module-private `ExecutionTargetReadControl` spans the absolute header/body deadline and document cancellation. The common bounded reader serves plan/grant JSON, the lease manifest, component, descriptor and the future actor body. Exact assets require matching Content-Length; unknown-length JSON uses bounded backing, copies only the accepted prefix, then wipes backing.

Each read owns a pending/claimed/abandoned slot. Abort/deadline rejects a pending read without waiting for an underlying cancel promise. Every claimed or abandoned chunk is wiped, failures wipe aggregate bytes, cancellation is issued once, and finally removes listeners/timer and releases the reader. The shared framework header-only fetch contract is unchanged.

The production attempt captures a private symbol, runtime-key/map identity, document schema, client instance, Hub origin/scope, selected surface and full installed target. These are checked across body reads, hashes, EOF and grant publication. A superseding attempt or changed owner cannot mint authority. Unpublished local leases are dropped even when grant fetch itself rejects.

## Independent Test Basis

Language-neutral JSON/schema: `os/🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json` and its sibling schema. AJV independently checks its contract. Vitest executes real native Response/ReadableStream semantics, including never-settling underlying cancellation; no handcrafted stream reader is used.

## Remaining Frontier

The protected actor route and lower Worker are separate qualified slices. Production Session-gated actor fetch/load and the real GIS normalized describe oracle are not connected yet. Native trusted generation/candidate verification is still compiling in root's exclusive cache (session27152), not a passing result.

## Final Regression Pass

Scoped format97577/check7468 GREEN. Body/reservation71956 and87087 GREEN6 (7 stream +16 ownership +16 reservation +7 lifecycle rows); GIS source11217 GREEN9. Broader browser-document-open20007 exposed its old test-only empty Hub binding; fixture now installs the actual binding/runtime key. Plan-read cancellation retains the public cancelled diagnostic. Retry42724 GREEN6. Execution-target-lease source50979 GREEN (four routes). Trusted bootstrap source40421 found one formatting-sensitive cleanup marker; whitespace-tolerant exact finalizer syntax fixed it and20917 GREEN. No native or whole-product completion is implied.
