# R138: identity and lifetime of the resident service

Scripts: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R138/` (`land1.py`, `land2.py`, `window.sh`, `one.sh`, `*.tests`, `frames-test.ts`).

## State at the time of writing

See the bottom section "Landed" for what is in the live tree; everything else in this file is design and open work.

## What exists already (read, not changed)

- `🏛️service/🌐️host`: a reload state (`current`, `pending`, `draining`) retires a service whose code digest no longer matches the tree after a quiet period (`🔄️reload`), and `idleShutdownMilliseconds` (15 to 30 min by policy) ends an idle service.
- `🏛️service/🧯️failure`: every unexpected failure gets a reference, a block with name, code, message and stack in the origin folder of the shared store (`service-failure-<reference>.txt`), and a refusal text with the reference and log path.

## Defect 1: identity (step 1)

Before: `hostEndpointsV1(implementation)` keyed a service by the implementation digest only; the runtime directory silently moved to the shared `/tmp/semio-repo/<uid>` when `<base>/semio-repo/<uid>/h-<tag>.sock` was longer than 96 bytes.

After: the tag is `sha256("semio-repo-host-v2" NUL implementation NUL checkout-root NUL machine-state-directory)[0..12]`; the checkout root is the nearest ancestor with `.git` or `.🧬semio` of the given directory (canonical spelling); the machine state root is `SEMIO_MACHINE_STATE_DIRECTORY` (R133 seam) or empty. The socket directory is `<base>/semio-repo/<uid>`, else `<base>/sr<uid>` (same base, shorter), else `HostRuntimePathErrorV1` (one plain text with both languages) naming the base and the bytes; there is no fallback to a shared folder. `service-status` carries `identity` (implementation, root, state, tag, directory, endpoint, log); `repo service host status` prints a `host-identity` line (code, checkout, machine state, start time, running and waiting counts, socket, log) in the language of the caller.

Proof: new law test `🏡️resident/🧪️tests/🆔️host-identity` (7 tests): two state roots on one checkout give three distinct endpoints/logs (none, one, two); two checkouts and two digests differ; the tag equals an independent Python hashlib computation; an overlong base uses the compact spelling below the same base or fails in both languages and never names `/tmp/semio-repo`; every endpoint of a usable base fits 96 bytes; the Windows pipe carries the tag; a subdirectory of a checkout reaches the same service as the checkout. Red on the old module (export missing), green on the new one.

## Defect (d): failure texts (step 2)

`failureFramesV1` (first six `at` lines, 200 characters each, 1500 in all) is a new field `frames` of the `internal` refusal, and both texts print `{frames}` after the message, so the client prints code, message and frames itself; the origin block already held the full stack. Test `🥞️failure-frames` (2), red on the old module.

## Lifetime (step 3, partly)

Every service records why it lives: `owner {route, pid, idleSeconds}` in its descriptor and in `service-status` (route `launchd`: kept by its job; `foreground`: `repo service host run`; `direct`: started implicitly by a repo command, pid = that command). A `direct` service ends after `implicitIdleShutdownMilliseconds` (120 s) without client or run in flight; the others keep 900 s. `repo service host status` prints the owner line (EN and DE). Test `🪪️owner-idle` (3): implicit service retires in about 0.3 s with a short override and says so in descriptor and status; launchd and foreground services outlive 1.2 s; two state roots on one checkout run two services side by side, each answers with its own identity (this is the runtime isolation proof; red before on the old tree).

Already present and unchanged: a hello of other code is refused with `host-stale` (client starts a fresh service), and the reload state machine retires an idle superseded service.

## Landed

2026-11-17 16:34:49 to 16:35:48 UTC under `landing.lock` (one landing, steps 1 to 3 stacked, `window.sh all`). Tests under the lock, 18 files, 4 in parallel: all pass except the known `🌐️host` slice-pool law (1 fail before and after); both standing preflights pass. A first try at 16:32 was restored on one red: the real-process failure law bounded the whole refusal text at 700 characters, now 2400.

Side effect to know: the plan test of the invocation module relied on the silent /tmp fallback with long runtime directories; it now uses a short base.

## Not done

- (b) lifetime: owner records for processes other than the service (farm children, reads), a running read on old code told to finish its unit and hand over, and the stale-code retirement of reads (not only idle services).
- (c) front door: holder identity and progress in the wait text, bounded loss of the door, disjoint scopes not queueing.
- Two real lab machines (slice lab) were not started for the isolation proof; the law test is the proof.
