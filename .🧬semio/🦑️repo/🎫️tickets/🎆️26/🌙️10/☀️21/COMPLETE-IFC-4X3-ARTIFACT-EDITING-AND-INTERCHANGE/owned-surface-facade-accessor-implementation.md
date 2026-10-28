# Acknowledged Ordinary Surface Lease Checkpoint

## Current Source Boundary

The actual renderer plugin handle now exposes `acquireOwnedUiSurface(instanceId, target)`. Only the exact ordinary refresh response object privately certified beside a successful issued native patch ACK can acquire a read lease. Certification binds the current real handle registry, actor, instance, activation/lifecycle, native UI owner, facade and exact acknowledged view object. A copied response, matching scalar hash, foreign handle/instance, pre-ACK view or stale target grants no reads. The normalized channel adapter without that native owner explicitly refuses acquisition.

The response remains unchanged on the wire. A private WeakMap records its certificate; there is no public scalar mint, recursive ViewModel, BuiltNode owner projection or JSON graph copy. Projection captures the exact source view before awaiting work and checks the same acknowledged view again before certification. Acquisition checks it again after creating the guarded lease. The implementation remains domain-neutral and read-only; staged action/window/session authorization is a separate consumer responsibility.

`handle-ui-intake/lease.ts` keeps the owner behind JavaScript private fields and a private constructor mint. The frozen lease exposes only `source`, `signal`, `current()` and retryable `close()`. The source offers the original facade's view, bounded node/view subscriptions, original read acknowledgement/unsubscription and notification retry. It never gives out the native instance owner. Limits are 64 leases per instance and 64 reads per lease, declared in `lease-limits.json` and independently validated by its JSON Schema.

## ACK And Retirement Integration

The actual intake calls `beginPatch` before advancing the replacement intake. This makes existing reads unavailable even though a stable facade's view may change before its next ACK. Only the exact pending intake can install a new ACK stamp. A compatible following ACK resumes existing leases over the same original source identity; the earlier response certificate cannot acquire another lease. A failed or unacknowledged intake remains unavailable.

Installation follows the existing actual issued ACK acceptance, ready result and `takeSurface`; current actor/lifecycle/owner checks precede indexing and certification. Publication notifications use bounded resident read snapshots and recheck currentness after reentrant callbacks. Source or owner replacement, disposing/closing, activation revocation and instance retirement refuse reads immediately.

Instance destruction marks closing synchronously, then revokes leases before clearing surface maps or closing the native owner. Consumer close retains failed unsubscriptions and paid native maintenance for retry. The same close promise is installed before dispatching abort callbacks. Original reads retire through the actual source's unsubscription and owner maintenance protocol; no new budget or wallet is fabricated. Promise tracking uses the existing `uiReadsByInstance` close ladder. Consumer close does not close sibling leases or the entire instance.

## Authored And Executed Checks

The language-neutral `lease-cases.json` matrix is validated by Ajv 2020 JSON Schema and exercised by `lease.test.ts`. The focused metadata/retirement harness passes **7 tests / 96 assertions / 0 failures**. Cases cover actual-object versus copied authority, getter noninvocation, pre-ACK replacement, exact intake matching, reentrant subscribe, retained failed unsubscription/maintenance, abort reentrancy, bounded publication and capacities, and reflection-resistant owner custody. This structural harness proves these source laws; its fake source is explicitly not native ownership or receipt evidence.

The real `UiDocumentStore/tests/owned-facade` fixture is registered through the existing typedwire companion. It authors an actual first-party activation/lifetime, issued native patch source, `OwnedUiPatchIntake`, genuine ACK submission receipt, `OwnedUiInstance`, original read snapshot and retirement protocol. It checks the changing-view-before-ACK gap, compatible subsequent ACK, old target refusal, sibling isolation and retirement before owner close. Its execution remains gated by the existing renderer Vitest policy. This checkpoint does not claim the fixture ran or that source registration is execution.

The exact 15-file repo-owned `framework test-staged-form` selection, including the new pure lease suite, passes **192 tests / 879 assertions / 0 failures** in 1.62 seconds through the documented direct-tool fallback. The fresh narrow strict TypeScript attempt exits 2 on existing unresolved UI/framework package aliases and their transitive implicit-any diagnostic. It reports no remaining independent local nullable error after its correction, but it is not a typecheck pass. The genuine renderer config invocation exits 1 before suite execution with `Explicit Vitest policy required`; no policy was fabricated. Logs are retained under `owned-facade-lease-tests.txt`, `owned-facade-staged-selection.txt`, `owned-facade-lease-types.txt` and `owned-facade-renderer-policy.txt` in this ticket's generated directory.

## Exact Source Roster

The adjacent generated `owned-facade-lease-source.json` records current SHA-256 hashes and absolute source paths for the lease implementation/contracts/tests, actual ACK installer/projection/close ladder, actual rich handle composition, declared handle API and refusing normalized adapter, registered native fixture and selected test entry. It is the source freeze for independent review; unrelated concurrent work is not attributed to this checkpoint.

## Remaining Gates

Independent read-only review is saved in `owned-facade-lease-independent-audit.md`: all 15 source hashes match and no new confirmed P1/P2 was found in the bounded source review. This is separate from the unexecuted genuine fixture and incomplete wider type graph.

The public staged sender/transport and auxiliary registration are still refused. The ordinary response lease is available only after authentic ordinary ACK installation. Connecting it to the staged pane requires the actual auxiliary source/ACK/context lifetime, current source revision/session and original control authority. No scalar response hash or graph projection substitutes for those prerequisites.

Native compiler ingress still refuses before rustc; native type, runtime, physical receipt, output, ABI mount and semantic admission flags remain false. Existing retained EXPRESS semantic publication stays refused. No compiler/test policy bypass, package export repair, general UI cap change or capability advertisement was made.
