# Hub Presence Normalization

## Current Evidence

The new `os-hub:presence-normalization-source-check` ran RED first: all 15 independent neutral vectors passed, then the source fence rejected the absent Hub reconstruction. After implementation it ran GREEN with all 15 vectors. The oracle validates the neutral schema with AJV, reconstructs exact wire bytes independently with third-party LEB128 and native double encoding, and compares the shared TypeScript codec byte-for-byte.

Native laws are implemented and registered but have **not yet run**. Root has reserved the space cache for a fresh execution-target native run followed by this selected SQLite gate, after the current WAL regression group. No duplicate lock wait was queued. No socket/runtime acceptance is claimed from the source gate.

The adversarial audit expansion is also source GREEN: two exact positive timestamps (2^32 and 2^53−1) first reproduced the independent U32 oracle bug, then passed after switching that oracle to the third-party unsigned U64 encoder. The corpus now has 17 exact vectors. The existing lease source regression also ran GREEN (18 checks).

Native tests now use a FIFO preview marker to prove all rejected and identical refresh frames were processed before inspecting TTL/publication. They also inject the two over-limit admitted-label cases into the server-local test lease, and the reconnect law now uses three plan-backed sockets, including an independent observer and a ledger-confirmed old-socket close. These are source-ready tests, not executed native receipts.

The later preflight audit identified an unsynchronized negative expiry assertion. Its test-only clock now gates the actual WebSocket server tick with admission/release/evaluated semaphores. The test must observe a completed tick at TTL−1 with unchanged roster and no fanout, then a completed tick at TTL with an empty roster frame, followed by refresh on the same live socket. No wall-clock sleep substitutes for server evaluation. A separate canonical-ingress extension of the capacity law installs 64 real normalized peers and rejects actor 65 without changing its deadline, roster or fanout; existing raw-byte aggregate arithmetic checks remain narrowly labeled. Both changes await native execution. The 17-vector source gate was rerun successfully after the tick change.

## Implemented Boundary

- The existing private lease retains Hub-captured connection time, directory display label, current admitted role, and plan-only document surface alongside its server actor/user/color.
- The existing per-frame socket authority gate still checks session generation, exact role, scope, plan and live ownership before presence ingress.
- Ingress bounded-decodes the existing peer format, creates a fresh existing `PresencePeer`, copies only presence pack, drag ghost, interaction, views and UI, then re-encodes. The encoded result must satisfy the same fixed decoder/entry limits.
- Malformed or over-limit input cannot refresh TTL or publish. A replaced live owner cannot overwrite its successor. Identical normalized bytes refresh TTL without republishing.
- One bounded user lookup supplies both display name and the already-existing sync-session email recording. The peer never contains email; missing lookup never falls back to a client label.
- Existing reconnect/expiry socket laws now send real encoded peers, not opaque text. New native laws cover the 15 admission vectors and an actual plan-backed socket with all seven authority fields forged.
- Directory presence uses the plan-bound surface as well; non-plan test rows remain in the document roster but do not mint a directory surface. The redundant URL surface was removed from the lease.
- New native fixture scratch roots honor `SEMIO_TEST_ARTIFACT_DIR` through the shared Hub test helper.

## Files

- `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`
- `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧬️schema/🔣️.json`
- `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json`
- `.vscode/🧩️launch.seed.jsonc` (411.065/411.066; Home agent regenerated launch and checked exact freshness)

## Remaining

The test-only tick gate now disarms before releasing its final admitted tick, then waits for evaluation and asserts no stale release permit remains. This resolves both the stale-token issue and the queued-next-tick race; production has no barrier code. Fresh registered source execution (session 71704) passed all 17 vectors after this correction. Native execution remains pending behind the root selected-SQLite execution-target build; it has not been claimed as passing.

Run native laws after the owned shared cache is free. The Home agent now owns browser projection plus the prerequisite exact `(space, document)` session/ready/heartbeat/close bookkeeping; a document-id-only lookup is insufficient. The codec/WGPU agent owns shared Rust/TypeScript hostile decoding and native roster projection. This slice does not qualify a complete Home, mounted browser/WGPU document, two-user collaboration, or AI approval journey.
