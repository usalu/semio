# R226: lease timeouts per class (declared wait, heartbeat, stale-holder rule, typed refusal)

STATUS: STAGED, NOT LANDED (the read gate of the coordinator is not lifted; the lease layer is loaded by every read). Script `T/🧰️scripts/R226/land.sh` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`), uses R224's `land-lib.sh` (window, lock with owner, exit trap, bases under the lock, new file first, delivery wait, exact restore). State (base/, new/, land.tsv) in the agent scratch `…/scratchpad/R226`. Times UTC, 2026-11-19.

## Built (verified against the code first)
Cause in code: `timeoutMs === undefined` is `Infinity` in `remaining()` (`🔒️leases/🟦️.ts`), so every caller without a timeout waited for ever, and a wait with no `control` and no `onWait` printed only a bare `Waiting for ...` line without the holder. R218's table existed only as prose.

* **Schema first** (`🧭️order/🧬️schema/🔣️.json`): new closed definition `Wait` (bound, ceiling, heartbeat, stale-holder silence, hint EN/DE), required on every `Class` and as `Policy.unclassifiedWait`; messages `waiting*`, `timeoutDeclared`, `verdict*` (EN/DE). Type `LeaseWaitV1` in the contract.
* **Policy** (`🧭️order/🔣️policy.json`): the 17 classes of the R218 table each declare their wait (short coordination locks 120 to 1800 s bound, ceiling 600 to 21600 s; graph, namespace, artifact, whole-retirement, image 1800 s / 21600 s = the dependency store sync bound; image-mapping equals the ground bound 1800 s); silence rule 300000 ms for all, heartbeat 30 s; unclassified resources get a wait too. Bounds are scaled by the machine speed factor (`registerLeaseWaitScaleV1`, at most 16, like the ground bound) so a 4-core 8 GB worker waits longer, not forever.
* **Mechanism** (one chokepoint, `traceLeaseV1` of both public entries and `upgradeQueuedResourceLease`): `declareLeaseWaitV1` gives a request without timeout the class bound, keeps a caller's timeout up to the class ceiling, leaves a try-lock (0) alone; the self-deadlock rule still sees a declared bound as unbounded (`waitDeclared`). `leaseHeartbeatV1` says, every class heartbeat and until the wait ends in any way, a two-language line naming the holder (pid, command, since, seconds of at most N s, "has not renewed its registration for N s, so it may be blocked"), another task of this process, or nobody registered. A nested acquisition does not say it twice (`waitBeating`).
* **Stale-holder rule**: the registry `holderOf` now reads entries of live pids of any age and returns `renewedSeconds`; `leaseHolderVerdictV1` is `progressing` (renewed within the class silence), `silent` (alive, not renewing: blocked or a reused pid after a restart) or `unregistered`; the verdict is in the heartbeat line and in the refusal text.
* **Typed refusal**: the existing `ResourceLeaseTimeoutErrorV1` (message unchanged) texts now add the class, the declared limit (declared seconds, factor), the holder verdict and the class hint (EN/DE). No class declares an automatic fallback: every one refuses typed, the hint says what to do; callers that have a fallback catch the typed class.
* **Ready supervisor agreement** (process-tree schema): `holderNamedPatterns` and `holderWaitPatterns` accept the lease wait line next to the dependency-store line, so a declared wait with its heartbeat lines is judged by `readyHolderCustodyV1` (progressing holder: no stall; silent: named; bound spent: the lease itself refuses at that bound), and the stale rule is one number (class `silentMilliseconds` = ready `holderSilentMilliseconds` 300000, a law).
* Fixture `🧭️order/🧪️tests/🧫️fixtures/🔣️.json` gained `wait` per class and `unclassifiedWait` (the closed schema requires it).

## Proof
New law `🔒️leases/🧪️tests/⏱️class-waits` (10 tests). Red before: with the staged policy but the old lease layer 6 of 10 fail (never-releasing holder: "still waiting" after 15 s; dead holder, stale holder, cancel, clamp, source shape); base without the schema fails at import. Green staged: 10/10 (16 s).
* policy table: 17 classes = R218 ids, every wait valid, ordering invariants, ground = image-mapping bound, a policy without `wait`/`unclassifiedWait` is refused;
* speed factor 4 and 1000 (capped 16), effective timeout matrix, verdict table;
* heartbeat texts EN/DE for named, silent, self, nobody; declared refusal text;
* **real child process holder that never releases**, plain and queued: typed refusal at the bound (2 s test policy), holder pid in `holder`, class + hint EN/DE in `texts`, at least 4 heartbeat lines in both languages, lines differ per beat;
* **dead holder** (SIGKILL while waiting): waiter acquires, no refusal, heartbeat stops with the wait, dead entry not named;
* **stale holder**: entry rewritten 10 minutes old (older than the registry's own staleness): default readers drop it, the lease layer still names it, silent line and refusal text say "may be blocked or its process number was reused";
* **cancelled waiter**: abort reason, no timeout error, no further lines, lease acquirable after the holder dies;
* caller timeout below ceiling kept, above cut, try-lock misses at once with no line;
* **agreement with ready**: every class silence = ready silence, heartbeat x4 <= stall window; heartbeat lines parse with `readyHolderWaitOfV1` (pid, command, seconds, limit, silent), tracker counts the wait, `readyHolderCustodyV1` gives progressing / silent / bound;
* source shape of both entries and the upgrade.
Neighbours staged = base (same pass/fail on both): leases wait 1/14, retirement dirs, fairness 5/12, physical, standing 26/0, bounded, admission 17/0, timeout-refusal 4/0, nested-shared, writer, status, order 76/0, ground 5/0, processes 28/1, sites 4/1, wait-for 6/0, component-driver 24/0, registry-root, windows 5/0, stale-queue 0/1; process-tree 16/0, containment 16/0, holder-custody 6/0, step-overdue 17/0; typed-refusals 12/0; capacity units 14, capacity 5, adapters 7, bazel sessions 45, cargo writer 3, native-build whole 5/1, graph startup 22/1, cargo order 0/1, bundle concurrency 0/1 (the failures are identical on the base; worker helper dirs "fail" as tests on both). Landing script rehearsed on a scratch root: forced red restores all 9 files and removes the new directory byte-exact; green path writes the staged bytes.

## NOT verified
* No landing, no live run; running reads and workers keep the old code. No Windows run (the heartbeat and verdict use the registry and `process.kill(pid,0)`, no `ps`).
* A reused pid is judged only by its missing renewal (silence rule), not by its start time; a holder that renews but is blocked is "progressing" (R183 rule, by design).
* Order-of-magnitude of the numbers (bounds, ceilings) is derived from R218 and the stated bounds, not measured on a worker; the heartbeat adds two stderr lines per 30 s per waiting acquisition.
* The ready test `🤝️holder-custody` (ready/🧪️tests) is flaky at load 40 on the base as well (it fails 1 of 6 in about half of the runs, staged and base).

## What others must know
* **R223 edits the same ready process-tree schema file**: whoever lands second must rebuild its base. For R226: refresh `base/` for that file and run `patch4.py` (three anchored replacements) again; the landing script writes nothing when a base differs.
* Behaviour change: a lease request that brought no timeout now refuses (typed, names the holder) after its class bound (graph, namespace, artifact, image 1800 s x speed factor) instead of waiting for ever; explicit callers (dependency store 21600 s) are unchanged because the ceiling equals their bound.
* Tests that build their own lease policy must declare `wait` and `unclassifiedWait` (the order fixture did).
* Landing: window hh:00 to hh:05 / hh:30 to hh:35 UTC, after the coordinator lifts the read gate: `T/🧰️scripts/R226/land.sh` (lock plus the 1113 lock, ~50 s of tests under the lock).
