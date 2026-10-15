# Issuance Scope Independent Review

Read-only source review on 2026-10-15. No tests, builds, Originals, images, leases, or runtime probes were started. Findings describe code paths, not observed runtime behavior.

## P1: Accepted detached resource loans are not drained

Owner: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️issuance/🟦️.ts` lines 10–14, 44–49, 66–78, 82–91.

The private scopes contain only liveness and parent metadata; there is no pending-child collection, child cancellation controller, or drain. `loan` synchronously admits a control and invokes its callback before awaiting the returned promise (76–77). A callback can therefore start an accepted `withRequestedIssuedResourceControlV1` child and return without awaiting it. The parent immediately retires at 78 while the child callback is still suspended. The child's later authority check rejects through the parent liveness check at 26, but its already-running callback is neither canceled nor awaited. The same issue occurs when root `run` starts `withIssuedResourceControlV1` without awaiting it: process issuance retires at 49 as soon as root `run` resolves.

The concrete authored test already demonstrates the first state: ticket `lease-namespace/📜️script.ts` lines 49–51 holds a child callback behind `finishChild`, awaits parent completion, checks escaped-child rejection, and only then manually releases/awaits the child. It validates rejection after parent retirement, not preservation of custody until accepted child settlement. Root-level detached-child success/error cases are absent from this owner.

Underlying `🧭️routing/📥️invocation/🏃️process/🟦️.ts` lines 33–36 and 47–48 clear the deadline timeout and remove SIGINT/SIGTERM handlers immediately after dispatch settles. Thus detached accepted work can also remain active after process cancellation ownership has ended; rejection on its next explicit check does not restore that lifetime.

Required invariant: register accepted child work with its parent before callback execution; stop accepting children when parent dispatch settles; cancel/drain every accepted child on success, failure, deadline, and interruption; retire authority and release process handlers only after that drain. A closed/closing distinction may be needed so cleanup and accepted work can settle without admitting new work.

## P1: Physical custody is not enrolled in the control lifetime

Owner: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📥️issuance/🟦️.ts` lines 32–43.

`withIssuedNativeImageV1` checks the control, then opens a backend asynchronously (33–34); after opening it awaits `run` and closes only its own image in `finally` (42–43). There is no enrollment of this promise into the issuing control scope. If called without awaiting while the control is live, its open or callback may remain pending after its parent loan and process return. If it reaches an image callback before parent return, the image stays physically open until that detached callback settles, while image reads reject because the control has retired (19). If parent returns during open, cleanup eventually runs after open settles, but the parent does not wait for it. Neither case preserves Original authority through actual physical cleanup.

The converse nested-image case also has no child registration: a callback can start an accepted second image operation without awaiting it, and the first wrapper can close while the second remains pending. Source review does not establish whether any particular backend prevents an unsafe physical release; it establishes that these wrappers provide no lifetime guarantee for accepted detached work.

Ticket test owner `lease-namespace/📜️script.ts` lines 34–42 awaits every genuine physical image wrapper and checks escaped-token refusal only at 37. No accepted-but-detached image open/callback/close drain case is covered there.

## Narrow checks that are present

Private WeakMap identity checks reject structural invocation/control/image copies (`issuance` 35–37, 54–57; physical issuance 16–21). The process issuer receives exactly `process.env` and uses a module-local once flag before dispatch (41–44), so a second call through this module instance is refused (test owner 55–56). These are valuable authenticity/retirement checks; they do not address pending work. This review makes no claim that schema validation of environment text cryptographically authenticates its author.

## Validation limitation

Only `rg` and source reads were used. Actual issuer behavioral tests are currently ticket-owned in `lease-namespace/📜️script.ts` (27–64); the framework search found declarations, not permanent dedicated issuance test owners. The existing nested-child test explicitly expects early retirement. Update that expectation and add detached root, nested child, image-open, image-callback, and error/interruption drain cases before claiming accepted-child lifetime is preserved.
