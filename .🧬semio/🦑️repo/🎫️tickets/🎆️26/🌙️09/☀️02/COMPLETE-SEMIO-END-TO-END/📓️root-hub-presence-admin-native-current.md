# Hub Presence and Administrator Recovery: Current Native Evidence

Current accepted mount boundary: session61554 is GREEN17, exact evidence `🗑️generated/exact-cargo-laws-gfGodw/00`, executable SHA256 `fc3caa3df81b904c9cabdc6d6da02baff4997b2c7d5c9bbf85cb0fb6a6fa9157`. All registered laws were listed and executed exactly once, including ownership before first storage probe, release before terminal ACK, retained hard refusal, terminal-authority latch, wake-gated close and returned external pending-close cursor. This supersedes the older mount reds at this exact boundary, not the broader backend/inference/Hub journey.

Writer29219/tnTE0b advanced to native execution: first9 laws passed, law10 `wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks` failed at storage8353. WG identified its old eager-release assumption and changed the law to require Conflict before the first release poll, Closed after explicit first-poll signaling, and continuation for the originally pinned operation. Production dormant-release behavior was preserved. The writer36 retry remains separate acceptance work.

Latest September6 continuation: mount94265/oanQof fails its first law with post-ACK Busy1; WG moves the artifact handoff use release before terminal ACK. Mount46637/bw5a5F then passes the first eleven laws and law12 fails a mandatory ordinary fixture submission on transient Contended; its probe now uses retained timer admission. Writer55699/q7gnQt stops on an in-flight typed registration change, and65318/WIlYF3 stops on prepared rollback helper imports/visibility before any native law. WG has repaired those exact module boundaries; writer29219 is active on the root-exclusive cache. Registered writer scope is now36 laws and mount scope17, including exact external pending-close cursor retention. Source AJV6/backend-pool-use8 is green, but no new complete native scope is accepted yet. Existing Hub presence/admin results below are not superseded by these build attempts.

Session55262 is terminal with a mixed result. It ran the registered headless SQLite targets sequentially on the root-owned warm target.

- Presence normalization/reconnect/expiry/roster/restart: GREEN6 in `🗑️generated/exact-cargo-laws-kOFmLK/00`, executable SHA-256 `a30f3ccc0252d28767998617829ea3eb90635ff48d518f6acaa441a799626dbd`. This is actual Hub binary/socket execution, including authenticated identity overwrite and no-refresh rejections, stale-lease reconnect rejection, server-clocked expiry, roster bounds and restart-empty/member-only presence.
- GIS proposal group: build RED in `exact-cargo-laws-nRxzs1/00`. Two concurrent source integration errors were repaired: the ordered directory publication fixture moved under `🧫️fixtures/🧪️fixtures`, and a newly added real GIS native-catalog fixture/test needed its existing `native-artifact-execution` feature guard for headless compilation. No native GIS proposal verdict followed from that run.
- Composed admin removal/target/presence/SQLite reopen: compiled, but RED in `exact-cargo-laws-CWdReu/00` at the first member socket Welcome assertion, before membership removal. No administrator recovery acceptance is claimed. The assertion now retains the actual unexpected frame in its diagnostic; a warm rerun is active to distinguish a handshake fixture defect from a real concurrent-open fault.

The new retained Database shutdown caller migration was included in the current Hub source. Successful presence does not qualify the composed administrator law, GIS approval publication, or all new DB shutdown interruption states.

Diagnostic retries93632 and58634 both stopped before native discovery on the in-progress typed WAL-open API. The latter's exact receipts are `exact-cargo-laws-ah48d6/00` (admin) and `exact-cargo-laws-rYullN/00` (GIS): an artifact snapshot rejection before WAL acquisition still returned plain DbError, and compact_document returned the inner actor Result without the new error mapping. The execution lane repaired both; admin-only51669 is the next warm diagnostic.

The independent source audit `📓️terra-hub-concurrent-first-document-open-race.md` proves a real catalog-published/not-mounted and dual-open race in Database's ready-only map. It does not yet identify the unexpected frame observed by55262. The DB execution lane now owns a bounded, generation-stamped single-flight mount lifetime, retained rejection cleanup and Hub API migration. The existing two-member concurrent Hello order remains intact; no serialized-test acceptance is permitted.

Further warm compile receipts51669,96862,20836,6056 and88853 exposed the coordinated typed-open/single-flight migration before test discovery. In particular W7Modt/00 reports two Ready-reply projections and a testkit helper trying to format/drop ArtifactEngineOpenRejected; the source now unwraps the authority projection correctly and explicitly retires acquired rejection owners. The Emit API now promises Send futures and publication stays inside the retained owner, before Ready/fanout, after the Terra audit rejected arbitrary waiter-owned emission.

Current93860 is still compiling the coherent seven-law snapshot in `exact-cargo-laws-U4wlLS/00`. Its eight-case neutral oracle is GREEN. The execution lane subsequently added an eighth native law and parked exact rejection cleanup, resumed only by controlled join/uncancelled shutdown; that later source is not included in this in-flight binary and must get a fresh sequential run. No new administrator, GIS, writer30, shutdown3 or compaction4 assertion verdict is claimed.

93860 is now terminal native RED, with successful compile/discovery and exact passes for the first five laws: concurrent ensure, catalog-published join, cancelled waiter, shared terminal rejection/retry, and owner-emits-before-Ready. The capacity law then fails at shutdown with one shared authority despite all returned handles having been dropped. Binary SHA256 is `aada518bb4bee808a073698a7acdb884d8008baac0545905d50e2f68dada279b`. The internal completion clone survived while waiter replies were woken. Root rejected an initial lock-across-wake repair because ReplySender::send directly calls arbitrary Waker::wake: the next source must prepare bounded outcomes under the lock, drop internal temporaries, unlock, then send; an inline reentrant-waker law guards the ordering. Neither sleeps nor weakened shared-authority checks qualify as a correction. The later parked cleanup law remains unexecuted.

Current70086 compiles the coherent nine-law mount snapshot in `exact-cargo-laws-RNAs9H/00`; its nine-case neutral oracle is GREEN. The current source builds fixed-capacity reply outcomes, hands off internal ownership under lock and performs every arbitrary wake after unlocking. It adds both controlled parked-cleanup retry and an inline reentrant wake test. No current native nine-law verdict exists yet.

70086 is terminal: build/discovery and concurrent-ensure law GREEN; the catalog-published join law timed out. A direct reproduction and sampled stack localized the test caller inside `drive` waiting on the work mutex while the worker polled the deliberately gated mount future under that mutex. The execution lane changed only drive admission to `try_lock`, treating an active poll as already-owned work and resuming parked cleanup only when available. Current15440/WFy5o0 reruns the same nine native laws; neutral traces now total ten, with the nonblocking-join invariant. No timeout-budget or gate weakening was used.

15440 is terminal GREEN9: all exact native selectors in `WFy5o0/00` pass, with executable SHA256 `07ebac781d2f276c2c88b64563f31ba403ffb05ec5ef1fdcf8fbfe76e27d7956`. This qualifies the retained single-flight mount states and all observed regressions above; it does not substitute for composed Hub administration. Root proceeds with retained shutdown47661 on the same warm cache before Hub/GIS follow-ups.

Shutdown47661/EY8Qjg and84891/p7L913 both stop before native discovery in the execution lane's in-progress typed journal bridge: first the definitive pre-write Rejected arm, then standalone DB-crate Store import paths. The owning lane is repairing those semantics and imports; root holds the warm cache until its full coherent source boundary.

The fully read audits `📓️terra-db-document-mount-interleavings-current-audit.md` and `📓️terra-db-worker-pool-liveness-current-frontier.md` identify additional states not covered by GREEN9: multiple queued pollers can strand worker threads on the same work mutex; a controlled cleanup-resume request can be lost while a cleanup poll parks; public shared pool shutdown has no retained-use guard. The execution lane is assigned coalesced sole-poller/wake/resume state, weak driver closures and truthful hard scheduler state, followed by a narrow native/cooperative pool-use lifetime guard. Existing successful completion fanout is not implicated. Native9 remains valid for its traces but is not an exhaustive scheduler/lifetime claim.

24225/Gll8tm reaches a borrow-lifetime error in the new weak retry callback; the owner lane now takes the retry job into a local so its mutex guard is released before submission. 76432/vfJMzf reaches the new cleanup-race law's completed future retaining an immutable Database borrow; explicit retirement after await fixes the mutable shutdown boundary. Neither run reaches native discovery. Current42812/ze6X3B compiles shutdown3 after the full source boundary: coalesced driver states, weak jobs, controlled resume flags, pool-use lifecycle guard and hard non-runnable executor witness. Mount registration now has fourteen selectors, still awaiting runtime qualification. An independent current-code audit has found additional post-terminal direct Database operations that need the same admission fence; that source repair belongs to the execution lane.

42812/ze6X3B is compile RED: dropping the future after consuming it did not repair the earlier shutdown borrow. The execution lane now signals the retained driver's controlled resume directly during the cleanup race and keeps mutable shutdown after the awaited future. It also fences terminal catalog creation, sync hello and checkpoint through one `require_open_use` admission.

5576 completes GREEN3 in `exact-cargo-laws-ZhuNVU/00`, executable SHA256 `5ae5a9dd65f2539adcbd16247ca49a017c0afbbb0ab793082b1734f99aeee357`. All three exact retained shutdown laws pass. This is the coherent pre-backend-guard boundary, not current mount14 or global pool-lifetime acceptance. The fully read updated `📓️terra-worker-pool-use-database-mount-current-review.md` verifies the facade fence and identifies lower-level backend registry lifetime, caller-selected raw task pool, forged backend kind and standalone compaction guard gaps. The execution lane owns that central correction; root defers the next native compilation until its API migration is coherent.

The backend registry/use and atomic pool-affinity migration is source GREEN with five additional neutral laws; writer native registration is now35. Mount97442/kwrDft stops before discovery on two stale internal test-call arities and a task-slot WorkerPool versus Arc<WorkerPool> mismatch. The execution lane repaired all three; mount82097/RIqL0s is compiling the current fourteen selectors. Terra's fully read addendum identifies registration rollback dropping its nonterminal owner if all lost-owner parking tiers are full. That retained rejection/saturation repair remains required; no global backend lifecycle claim follows from the current guards or future writer35 alone.

## September6 Continuation

Latest journal qualification is64870 GREEN3, `exact-cargo-laws-ZJRrWu/00`,
executable SHA256
`0349c09335867ba4d64bb04258c7483bdafc674a302616822c6877a789bd64a5`.
It proves exact Fsync decision Event plus a separate committed ordinary command
produce two physical replay transactions and one opaque decision witness,
pre-handoff cancellation is absent, and hash rejection precedes mailbox handoff.
The retained shutdown helper now drives its exact terminal cursor. This is not
three-Store recovery or approval/publication. The immediately preceding29484/
LxSP1R build RED was the borrowed Arc consumed by run_turn, now cloned explicitly.

New source boundary after the earlier GREEN17 mount: typed storage-open failures
retain incoming/registered backend cleanup; external terminal `resume` returns
its exact cursor on refusal; a preadmitted authority retirement registry owns
bare-authority Drop. The registered mount suite is now19, native-pending.

Writer98692/Be9XY4 reaches the cross-key wake law's old eager-release expectation.
6243/eBCsE9 passes its first11 laws and finds that the first release poll already
returns the intended retained missing-controller rejection, not Pending. WG now
captures that exact returned rejection at first poll before checking B's wake.

Journal93700/bUcnaf's added control transaction was buffered Memory durability;
explicit Fsync preserves the intended two physical transactions/one witness.
54176/7gwJw6 then exposes a test shutdown helper that never drove the retained
terminal job.63610/6indWO stops at compile during typed constructor/retirement
edits (FS import and authority literal), both since repaired by the owner lane.
These are executed REDs, not new journal or composed-admin acceptance.

The preceding logical turn made substantive source/runtime progress; this is continued implementation, not external waiting or a blocked goal. After an interrupted turn,82097 was no longer attached, but its retainedRIqL0s receipt proves compilation/discovery and the first eight laws passed, then law9 failed at engine13074: the temporary unlock fault did not park the retained mount owner. BinarySHA256 is `f0a58d83f9579b4d05e9e8ac20056efd349a9100c6cfbf6b312c2bc5576d642e`. No terminal was inferred from absence of its process.

The first repair separates ordinary new/join callers' wake-only requests from controlled shutdown cleanup-resume requests, consuming resume only for parked work. Root53820/uv5IaM still reproduces the same law9 failure after eight passes. That is not mount14 acceptance; execution and Terra audit are tracing the remaining deferred release/wake behavior without weakening the assertion. Current coherent DB/storage source is independently dispatched through writer35 while mount diagnosis continues. The older shutdown3 receipt remains scoped to its earlier bytes.

The Store three-edit audit was read fully. Its new verified projection is read-only canonical decoding, not committed transaction authority or a live three-Store seal. Hub approval remains unavailable until a DB-owned committed single-Event witness and live `recover_store_owned` publication/recovery complete; event bytes, a caller receipt, or the parent edit alone cannot authorize partial Map application. See `📓️terra-store-three-edit-wal-witness-current-audit.md`.

The concrete writer28565/yDnbws failure is premature release activation before an `ArtifactWalOpenRejected` reaches its caller. The corrected writer release is dormant until first cleanup poll; Drop still signals nonblocking cleanup. Mount15475/Z35ppx then exposes a retained pool use from an obsolete strong queued ArtifactRunner job. Normal queued jobs now carry weak ownership; hard-refusal terminal Jobs retain the exact runner strongly. The close callback is invoked outside its mutex. Terra's follow-up identifies a remaining terminal-authority race between retry/wake and taking the terminal job; WG owns its latch correction and the separate registration rollback reservation P0.

Mount26475/GaiJEI advances through the earlier ownership laws, then rejects a coalescing fixture's one-shot pool submission. That fixture now submits the same ordinary gate-release job through the real pool's retained scheduled retry for transient contention, not inline execution or a sleep. Mount97224/0Rcb2U proves the first eleven exact laws, including parked cleanup, shared-pool coalescing and racing controlled resume. Law12 fails because its early-shutdown expectation is still `Busy { retained_uses: 1 }` while the central backend guard now yields2. This expectation must be reconciled with the exact lifecycle ledger; no mount14 pass is inferred. The root cache is idle after this terminal result, pending coherent owner-lane source.
