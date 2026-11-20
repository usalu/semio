# R218: readiness and real work on one machine: the `tools` stage of g1-m05 (living report)

Scripts: `T/🧰️scripts/R218/land.sh` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`); staging, patches, preload and test runners in the agent scratch `R218` (`patchA.py`, `patchB.py`, `mkrun.py`, `preload.ts`, `run.sh`, `runlive.sh`, `land.tsv`). Times UTC, 2026-11-18.

STATUS: LANDED 20:30:00 to 20:32:24 UTC, green (entry in landings.md). Section 7 has the g1-m05 reading.

## 1. What the `tools` stage needs, and why it asked for exclusive access on a machine whose tools exist

The stage is `repo tools setup all` (`🚦️ready` stage table, `tools`, arguments `tools setup all`). In `⌨️command/⚙️runtime/🟦️.ts` the command does, in this order:

1. `setupRepositoryDependencyPublicationV1` (`⚙️setup/🟦️.ts`): the dependency store (`native-fixed-image:<.cache/b/d image>`) is reconciled: funding observation, producer preparation, retirement of obsolete or unready generations, publication, funding reconciliation. Every one of these goes through `withFundingParentV1` / `retryWhileDependencyStoreBusyV1` and takes the **maintenance parent of the store exclusively** (`withNativePlatformProtectedMaintenanceParentV1`, `wait:false`, retried with the holder line you see). The wait bound is the store sync queue bound, 21600 s.
2. `withRepositoryCacheStoreV1(root,"dependencies",...)`: opens the store as a **reader** (shared image hold) plus one native-storage reservation (`native-storage:<identity>`: shared for journal reads, exclusive for one journal `transaction`, normally milliseconds).
3. `runToolchainsSetupV1`: `setupToolsV1` verifies every tool first and skips a tool that verifies (`skipped` line); only a missing or changed tool reaches `provision`, which is the only place that enters `inStore` (`withRepositoryToolingStoreV1`) and publishes (bun, binaryen, native tools, python by one rename, dotnet into a private directory and one rename, rustup). A machine whose tools exist therefore needs **nothing exclusive from step 3**.

So the exclusive demand of a verify-only stage is step 1, and it is demanded **unconditionally**: the only early exit was "an ANCESTOR holds the store shared and the funding is current" (R-earlier shelter rule). Any other shared holder (a developer's `run @semio-tech/fem-3d-rs:check`, a `script.ts component` driver stage) made the stage queue for the exclusive parent, although the very first question of the function ("is the funding current, is the envelope current, is nothing pending") is answerable by a lock-free read (`readRepositoryDependencyFundingCurrentV1`, already in the module). That is the cause: the verdict "nothing to do" was only honoured for ancestors, and everything else (including retirement of garbage generations, an optional step) was run as if it were an installation.

What the foreign commands hold, from the registry and from R183: the dependency image **shared** for their whole life (readers, hours); `native-storage:` shared while reading the journal and exclusive per transaction. The stage's own process after the fix holds nothing exclusive on a verified store.

Also observed on g1-m05 now (19:54 UTC, read-only): the current run is not waiting at all; it is walking `dependencies:funded-ownership:frontier|list:N` (64 per step, 100 k per 25 s at this moment, the counter ran to 2.4 M in the R187 run) and passed `native-artifact-funding-measurement` into `native-artifact-input-verification` within about 90 s of its restart. The funded-ownership walk is the long part of the stage once nothing blocks it; it is an exclusive-parent activity (published as `mode: exclusive` in the activity lines), so it also is what a foreign shared holder starves.

## 2. Fix by construction (built, task 1)

`⚙️setup/💰️funding/🟦️.ts`:
* `dependencyStoreVerdictV1({envelope,pending,funding,holders})` is the pure verdict: `unverified` (not provably current: the full exclusive path, unchanged), `occupied` (provably current AND every live holder holds shared: nothing needs exclusive access and none is possible), `verified` (provably current, no shared-only holder: maintenance is optional).
* `withDependencyStoreWaitBoundV1(ms, body)`: an async-scope bound on every `retryWhileDependencyStoreBusyV1` wait inside the scope (the smaller of it and the ports' bound). No signature changed; the existing source-shape laws of the funding wait stay true.
* `DEPENDENCY_STORE_WAIT_V1.verifiedMaintenanceMilliseconds` 20000.

`⚙️setup/🟦️.ts`: after the existing ancestor shelter, the verdict is computed from the lock-free evidence and the registry. `occupied`: say `dependency-store.verified-shared` (EN/DE, names count, first holder pid, command, since) and return before any exclusive step. `verified`: the remaining body runs under the 20 s bound; on `dependency-store.busy-timeout` it says `dependency-store.maintenance-deferred` (warn) and returns, the maintenance (retirement of obsolete generations) is left to the next setup. `unverified`: unchanged, full wait (the R183 rule: waiting behind a live progressing holder is waiting).

Consequences: a verify-only stage never waits for a developer's shared build; it never takes the exclusive parent while someone reads; it never blocks a developer's command beyond the instant of real maintenance, and a command that arrives during that maintenance waits under the same rule as before. A missing or changed tool still stages privately and publishes by one rename (unchanged code), and the funding is `unverified` or the envelope changed only in a genuine dependency change, where the exclusive path is right.

Not changed (stated honestly): garbage retirement happens only in gaps; on a machine that always has a shared reader it never runs. That is acceptable for readiness; a human `repo cache` command or a quiet moment collects it. The funded-ownership walk itself (2.4 M entries) is untouched.

Laws (`⚙️setup/🧪️tests/⏳️busy-funding`, 4 new tests; 22 of 23 pass staged, the 23rd `the real process ancestry equals an independent walk with ps` fails identically on the base, an environment fault of this sandbox's `ps`):
* verdict table (unverified for each missing evidence, occupied for shared-only including ancestors, verified for exclusive or no holder);
* wait scope: inside 5 s a forever-busy store ends with the bilingual `busy-timeout` after 5 to 10 virtual seconds; outside the scope the same busy store keeps the full bound (a 4 s wait succeeds, a 40-poll wait passes 5 s); the scope is not inherited by a sibling call;
* real process + real registry + real marker file: a `sleep` process registered as a developer build (`bun script.ts repo run fem-3d-rs:check`, shared, started 120 s ago): verdict `occupied` and the holder's command is named; the same process with an exclusive hold: `verified`; the marker changed to a different envelope: `unverified`; marker restored: `occupied`; the process killed: its entry is removed by the observation and the verdict is `verified`;
* source shape: verdict before the first exclusive observation, `occupied` returns before it, `unverified` runs the full path, `verified` runs under the bound and defers on `busy-timeout`.
Neighbours staged = base: setup tests 8/0, moving-tree 7/0, busy-driver 12/0, retirement 5/0.

NOT proved (no Windows machine, and the full `setupRepositoryDependencyPublicationV1` needs a real native image and an issued invocation): the end-to-end run of the setup with a real long shared holder. The two-process law covers the registry, the marker and the verdict; the wiring is covered by the source-shape law and the pure scope law. A real run of `repo tools setup all` on a machine with a running `run ...:check` is the open check (g1-m05 itself, section 6).

## 3. `Resource lease timed out: native-storage:[...]` as "Internal error" (built, task 2)

Cause: `acquireResourceLease` / `acquireQueuedResourceLease` threw a bare `Error("Resource lease timed out: <resource>")` in nine places; the CLI boundary renders anything that is not a typed refusal as `Internal error: ... (reference ...)`; the resident service rendered it as an unexpected failure. R170 typed only the lease ORDER violation (`SEMIO_LEASE_ORDER`), R190 only the admission naming.

Built:
* One function, `leaseTimeoutErrorV1(options, started, cause?)` in `🔒️leases/🟦️.ts`, builds the one class `ResourceLeaseTimeoutErrorV1` for all nine sites (the eight plain sites and the re-mapped admission timeout of the queue path, which keeps its `cause`). The message is unchanged (`Resource lease timed out: <resource>`), because callers match it (`startsWith`, `===`, `includes`, in more than twenty places); the class adds `code SEMIO_LEASE_TIMEOUT`, `resource`, `mode`, `directory`, `waitedMilliseconds`, a lazy `holder` (registry lookup: pid, command, age; another task of this process; nobody) and lazy bilingual `texts` (EN/DE: resource, mode, seconds waited, holder, "run the command again once the holder has finished; a holder that renews its claim is working, not stuck"). Lazy, because try-locks that are meant to miss (timeout 0) throw this class thousands of times and must not read the registry.
* Order module: `leaseHolderOfV1`, `formatLeaseTimeoutV1`, message `timeout` in the schema (EN/DE), `blockerText` narrowed to the fields it reads.
* Boundary: typed refusal class `lease-timeout` (`ResourceLeaseTimeoutErrorV1`, exit 1) in the progress schema `TypedRefusals`; resident service: `leaseTimeoutRefusalOfV1` and the typed refusals `host-lease-timeout` / `host-lease-timeout-unregistered` (EN/DE) next to R170's `host-lease-refused`.
* Law `🔒️leases/🧪️tests/⌛️timeout-refusal` (4): census (the only construction of the message in the process and build libraries is the one `super(...)` of the class: no other `Error("Resource lease timed out` anywhere, found by the walk of R170's census); message, name, code, lazy texts; rendering of holder pid/command/age/self/nobody in both languages; **two real processes, every lease class**: one child holds one exclusive lease of each of the 17 declared classes, the parent waits for each in four ways (plain/queued x shared/exclusive) with a 250 ms budget: all 68 timeouts are the typed class with the exact message, name the child's pid, name the resource, and render at `reportBoundaryErrorV1` in EN and DE identical to `texts`, exit 1, no internal-error text. Typed-refusals law extended by the new class (10/0); resident failure law +1 (19/0).
* Neighbours staged: order 76/0, processes 28/0, wait-for 6/0, ground 5/0, wait 15/0, fairness 17/0, stale-queue 5/0, standing 26/0, admission 17/0, windows 5/0, nested-shared, terminal-release, status all green; the order `sites` law has one failure that is identical on the base (`the census finds the acquisition sites...`).

Decision per class (what a timeout means; NOT built, staged proposal for the policy):

| class (rank) | resources | timeout right? | rule |
|---|---|---|---|
| pin 0, toolchain 1, tool-singleton 2, descriptor-custody 2, cargo-coordination 2, cargo-custody 2, compiler-capacity 3, daemon-slot 0, daemon-startup 8, registry-fetch 8, bazel-picker 9 | short coordination locks | yes: caller's own budget; a holder that does not release in that time is wrong | typed refusal with holder (built) |
| graph 4, namespace 5 | graph shards, rust code namespaces | caller's budget, but a reader of hours exists (graph read holds shards exclusive for its life) | typed refusal (built); the graph server already yields (R183) |
| artifact 6, native-whole-retirement 6 | publication / retirement | yes for try-locks; a queued wait behind a live renewing holder is waiting | R183 rule proposed: wait while the registered holder is alive and renewed within 300 s, bound = stated limit x speed factor, else typed refusal |
| image 7 (`native-fixed-image:`, `native-storage:`) | dependency store, journal | `native-fixed-image:` is shared by every reader for hours and exclusive only for maintenance: the exclusive taker must wait under the R183 rule (already true for the dependency store, 21600 s, holder named); `native-storage:` is a per-transaction mutex: a holder that keeps it past a command-phase budget (62 s on g1-m05) is a stuck or starved transaction, a timeout with the holder named is right, and the next improvement is to not enter the transaction while a shared reader is expected to block it | typed refusal (built); R183 rule for the image class proposed |
| image-mapping 7 (ground) | native image mapping | never a hold-and-wait; stated bound 1800 s x speed (R170) | unchanged |

What the 18:24 failure was (`native-storage:`, run 39, 62 s): the stage needed the journal exclusively inside its funding observation while a foreign command's transaction or queue held it. The new text names the holder (pid, command, age) the next time. Which holder it was cannot be known from the store (the log was overwritten).

## 4. What `ready: true` should require on a machine with resident work (proposal, task 3)

Facts: g1-m05 has run builds for a day (real command 36528 `run fem-3d-rs:check`, `component` driver 12936, `show projects` etc. in the containment log), so its native toolchains, dependency store, and a graph generation exist; yet readiness re-runs `native, repo-mcp, dependencies, tools, workspace` as if the machine were new, against a busy machine, and each stage is serialised behind work the machine is already doing.

Rule (precise; nothing weakened, each stage keeps its claim, only the source of the verdict changes):

| stage | claim | evidence that equals the stage | where it lives today |
|---|---|---|---|
| `native` | the pinned native bootstrap tools run | the real `repo` binary and the bootstrap marker exist and the stage digest is current (this is what `adopted` already does for a whole first run) | `ready.json` stage digests, `marker`, `repoPresent` |
| `repo-mcp` | `repo-mcp:install` was built | the build output of that target is present and its input digest equals the stage digest | stage digest + `.cache` output |
| `dependencies` | `dependencies sync --all` is current | the funding marker is current (`readRepositoryDependencyFundingCurrentV1`: lock-free) AND `bun.lock`/`package.json` digests equal the stage digest | lock-free read exists (built in this ticket) |
| `tools` | every pinned tool verifies | `repo tools verify` exit 0 (verify-only, takes nothing), or the preflight of any real command that passed `setupToolsV1`'s `skipped`/`ok` verdict for the same tool table digest | `runToolsVerifyV1` exists; evidence row not yet written |
| `workspace` | the three probes build | the probes' selected generation fetched from the fleet (R171), or a green real build of the same targets on this machine with the same digest | R171 generation fetch |

Evidence ledger (staged proposal): `origins/<machine>/ready-evidence.json`, schema first, one row per `{stage, stageDigest, source: "real-command"|"fleet-generation"|"verify", observedMs, command, exit}`; written by the real command's own preflight (tools verify result, probe build), read by the ready loop as `done[stage] = digest` only when `source` is in the declared list, `stageDigest` equals the current digest and the evidence is younger than the declared maximum (the stage digest already changes with every input). A stage with evidence is recorded `doneMs` with reason `evidence`, never re-run; a stage without evidence runs as today. `ready: true` still needs all five stage rows, the marker and the repo binary.

Implemented in this ticket (the evidence that exists and is pure): the `dependencies`/`tools` dependency-store claim is answered from the lock-free funding read and the registry instead of the exclusive observation (section 2). Not implemented: the ledger, its writer in the real commands, and its reader in the loop (a change of the ready loop and of every real command's preflight across the build tooling, proposal only; the loop is an 87 KB module that all workers run).

Why not simply `adopted`-style for tools: `adopted` marks ALL stages done from a marker on a first run; per-stage evidence is the generalisation and needs the ledger.

## 5. Unverified

* Everything Windows: lease registry on Windows paths, `ps`-less liveness of holders (the CIM start time path is R183's), real `repo tools setup all` on m05.
* The whole `setupRepositoryDependencyPublicationV1` end to end with a real shared holder (needs a real native image).
* That the verified path is sufficient for the dependency generation on m05: reasoning: the `dependencies` stage precedes `tools` with the same inputs, so a current funding and a current envelope mean the generation was produced; a funding that is current while the generation is incomplete would be skipped (the old path would have run the producer preparation). The producer preparation was already skipped under an ancestor's hold on the same evidence.
* The cause of the 18:24 `native-storage:` timeout (holder unknown).

## 7. Landing and what g1-m05 did next

LANDED 2026-11-18 20:30:00 to 20:32:24 UTC (hold 144 s). A first try at 20:00:05 was restored by the script on red: the law `actual setup frontdoor returns a proved current empty image...` extracts the setup function and runs it with a fixed port list, so the new identifiers were undefined (ports added in the law); staged runs read live sources and hid it (lesson: laws that read sources run only under the lock or from a full mirror).

g1-m05 at 20:32 UTC (read-only, store `origins/g1-m05/`): `ready.json` state running, stage tools, attempts 20, reason `adopted` (pid 29612: a stage already in flight is adopted, so it runs the OLD code; the new code reaches m05 only with the next stage start after the ready role restarts). Verbatim from `ready-log-tools.txt`: "Process 11736 (C:\semio\...\driver-stages\script.ts component --manifest hub/compositions/fem/compon...) holds the dependency store since 2026-11-18T20:05:55.260Z and has not renewed its registration for 1342 s, so it may be blocked; waiting for it (1357 s of at most 21600 s)". So: the previous 33-minute run (frontier walk, funding measurement, input verification, all progressing at 19:54 to 20:03) was replaced; the new attempt waits for a `component` driver that has been silent 22 minutes (R183 marks a silent holder as no progress: the stall rule will stop the try after 10 min of identical lines and name it `stage-holder-silent`). Next to watch, after the first stage start on the new code: `dependency-store.verified-shared` (EN/DE line naming the holder) or `maintenance-deferred` in `ready-log-tools.txt`, and a lease timeout rendered as the typed text instead of `Internal error: Resource lease timed out`.
