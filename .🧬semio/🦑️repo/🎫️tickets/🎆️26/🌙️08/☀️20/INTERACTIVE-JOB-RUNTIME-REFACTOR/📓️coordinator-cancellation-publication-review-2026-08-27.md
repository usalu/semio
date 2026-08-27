# Cancellation At Publication

## Latest Native Attempt

The next r6 attempt reaches normal mounted startup but **aborts** at a process-pool configuration mismatch: the fixture initialized two cores while the real subsystem requested ten. The production one-pool guard is correct and remains unchanged. The exact fixture configuration is repaired in source, and the executor is iterating the actual registered path under the exclusive compiler lease. A second Store Drop panic during unwind explains the process abort; r6 is not a completed nine-pass or ten-pass result. Log: `🧪️coordinator-latest-wins-native-r6-2026-08-27.txt`.

Expanded r5 is **RED: 9 passed, 1 failed, 414 filtered**, 0.39 s runtime after 1m53s compilation. The new exact-rebase, capacity-plus-one registry reclamation, reserved-slot/publisher fairness, lock-held deferred finish, and raw-capacity retirement laws all execute and pass alongside the original four laws. The registered-dispatch test fails before dispatch because its fixture manifest omits a window kind. The normal app validator is unchanged; the executor added the fixture's edit mode and main Canvas2d window, and the ten-test rerun is queued. Log: `🧪️coordinator-latest-wins-native-r5-2026-08-27.txt`.

The contended finish repair stores a preadmitted atomic finished marker and scans one registry slot per maintenance turn. A held registry mutex yields without advancing the cursor or faulting; the subsequent turn releases only the matching operation/claim. The native fixture proves an independently replaced claim survives. The full registered command path remains unverified until its constructor and later assertions actually execute.

The repaired r4 cohort passes **4/4**, 414 filtered, 0.06 s runtime after 44.55 s compilation. Runtime debug records confirm all six exact 8,192-byte scopes, all five Presence cancellation boundaries, and ten Document cancellation/ACK-exhaustion cases, with exact roots/counts/generations preserved and explicit terminal cleanup. This is a pass for these four native laws, not the still-pending registered dispatch/admission/rebase/fairness integration. Log: `🧪️coordinator-latest-wins-native-r4-2026-08-27.txt`.

The source repair adds an explicit default-denied draft Store owner hook and constructor forwarding. TestApp supplies its exact NoDraft owners/disposer plus zero-payload Presence/Transient shell disposers; the Presence fixture rejects nonempty peer rosters. The coordinator reviewed these helpers. Real app close coverage must adopt and validate exact owners separately; empty fixtures do not certify populated multi-user stores.

The namespace repair compiled. The second attempt ran the latest-wins cohort but aborted during the real Store cancellation test. An exact third reproduction retained its full output: the first failure is the app close at `component.rs:17803`, where the synthetic TestApp has no bounded draft-store disposer (`interactive-job.close-owned-disposer-missing`). ArtifactStore's strict final-owner Drop then panics during unwind. Neither this aborted cohort nor the standalone reproduction is a pass. The executor is repairing the exact fixture ownership while retaining all app-close assertions and production guards. Evidence: `🧪️coordinator-latest-wins-native-r3-2026-08-27.txt`.

The separate root-owned source-only `tool-jobs --self-test` gate passes 762 checks, including latest-wins schema/oracle and hostile-source cases. Discovery finds 33 exact factory owners, 254 custom rows and 25 generic rows. This does not substitute for the failing native test or the full command census. Evidence: `🧪️coordinator-tool-job-self-tests-r9-2026-08-27.txt`.

The first root-owned `retained_latest_wins_` native attempt failed during plugin compilation on 15 E0433 namespace errors: the plugin's local protocol module does not expose `protocol::value::ordered`. No runtime test ran. The executor is selecting the existing explicit owned ordered-map import. The warning/full-rendered stream was tool-truncated; every diagnostic's code, message and primary location was recovered without truncation in `📊️coordinator-latest-wins-native-r1-errors-2026-08-27.json`. Console record: `🧪️coordinator-latest-wins-native-r1-2026-08-27.txt`.

## Expanded Source Boundary And Follow-Ups

Additional source review found a contention cleanup hole: release_current returns immediately when the cancellation registry mutex is busy, but finish/finish_in_place mark the lease finished anyway. Mounted retirement takes and drops that lease. If both release attempts encounter the held lock, the registry entry and active counters remain without a retained retry; a finished Drop does not even cancel its publication claim. The executor now owns a lock-held adversarial fixture and bounded deferred/retried release. Keyed removal must also confirm the exact scope/claim, not merely an operation ID. This is separate from the four already-passing uncontended native laws.

The coordinator reviewed the private atomic publication claim, exact operation/document/app cancellation scopes, immutable full-domain key encoding, bytewise owned key registry, pending-command FIFO admission, and real Store publisher fixtures. The key is a length-framed instance/document/controller/tool/target encoding, not a hash or truncated target. Fixtures include six distinct 8,192-byte target scopes, actual Presence publication boundaries, and actual Document mutation/ACK-failure boundaries. They still need native execution.

Required follow-ups before full registered-command integration can pass:

- Rebase changes pending operation revision/generation; the reserved cancellation lease and its registry entry must be rebound to that same exact new authority before the worker starts.
- A free destination slot observed at initial enqueue is not a reservation across many pending turns. Own an actual slot reservation or explicitly retain/reject before final insertion; test a colliding later admission.
- Maintenance stage 17 already advances dead-key reclamation; the coordinator's initial suspicion that no maintenance path existed was incorrect. However, a full map can still take the immediate admission branch before maintenance reclaims dead entries. Test capacity-plus-one completed targets and retained reclaim-before-admit behavior.
- Publication currently selects the first registry owner each time and returns if it is not in Publishing. Add bounded round-robin selection and prove a long-running first operation does not starve a ready independent target.
- Exercise the actual registered factory path through dispatch, pending key admission, fresh-snapshot capture, worker result and Store publication. Separate key and publisher fixtures do not establish that whole path.

The current first-stage acknowledgement is pending admission, not proof that target supersession has already linearized. Any UI or acceptance claims must distinguish that boundary. Final-input freshness still requires the new accepted key to deny old publication, or a previously claimed commit to determine the new base/rebase.

## Earlier Review

The coordinator reviewed the new mounted pre-turn cancellation guard and its five-boundary PresenceStore fixture. Before a publication turn, cancellation retains the completion/pending Store owner, begins explicit close, suppresses later UI publication, and queues one cancellation result. Already committed pages remain owned until their exact acknowledgement. Native execution is pending.

This is a useful backstop, not final latest-wins proof. Entry-check followed by commit has a race unless cancellation and publication have an explicit serialized decision or atomic claim. The executor is adding exact instance/document/target-key admission, full-domain byte-resumable key handling, and a private lease publication claim. Supersession accepted first must prevent the old claim; an old claim accepted first is a prior commit and must inform the new operation's base/rebase. A real document mutation test and cancellation/ACK-failure cleanup are required in addition to the current NoPresence fixture.

No fixed widget-ID cap, hash-only identity, producer-local generation, or UI timer is accepted as a substitute for this authority.
