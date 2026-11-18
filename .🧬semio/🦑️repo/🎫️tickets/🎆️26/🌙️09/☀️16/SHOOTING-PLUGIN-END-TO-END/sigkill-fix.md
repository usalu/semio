# SIGKILL fix: ownership by proof, liveness by life, kills with a name

Scope: suspects 1 to 3 of `sigkill-sources.md`. Everything is under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/` (`READY`) unless stated.

## Root causes

1. **Registry orphan reaper** (`readyReapOrphansV1`) attributed a victim by time: any lease holder of the checkout that started inside a stopped try plus 60 s was an "orphan". A graph service (`service host run`) or another agent's `repo cargo` build that started in that window was killed, with a straight SIGKILL (`readyStopV1`), so the service's own SIGTERM handler never wrote an exit row.
2. **Stage stall kill** counted only new log lines. A silent `rustc` codegen or a stage that waits quietly for 10 minutes was stopped (SIGTERM, then tree and process group SIGKILL). The group kill and the roster sweep could also take a resident service that a stage client had started.
3. **Sidecar restart** (`createSidecarRestartV1`): reviewed, see below. It only reaches processes it owns.

## Fix

### Ownership by token, never by start time
- A stage run gets the owner token `ready:<12 hex of SHA-256 of the checkout path>:<stage>:<start of the try>` (`readyOwnerTokenV1`, `READY/🌲️process-tree/🧹️containment/🟦️.ts`) in the environment variable `SEMIO_LEASE_OWNER` (policy `ownerEnvironment`). Every descendant inherits it.
- The lease registry writer puts it into the entry as `owner` (`🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts`, `LEASE_OWNER_ENVIRONMENT_V1`, test asserts it equals the policy name).
- Verdict `orphan` now needs: entry `owner` equals the token of a stopped try of this checkout and stage (windows now carry `stage`), identity (pid + start), not in the current stage tree, not a person's. Time windows, directory and command guessing are gone (`windowGraceMilliseconds`, `commandPattern`, `excludedDirectoryParts` removed). Foreign graph services, other agents' cargo builds and anything without a token are `not-ours`.
- Resident services are spared on top: verdict `shared` (`sharedCommandPattern` = `<hex> service host run`). `readySharedPidsV1` computes the spared pids; `readyKillPlanV1` got a `spare` set (the pid and everything below it are never in a plan), used by the reaper, the roster sweep, the settle stop and the stall stop. The process-group SIGKILL is skipped when a spared process is part of the tree.

### SIGTERM first, then SIGKILL, and a named kill record
- `readyStopV1` takes `grace`: pass 1 sends SIGTERM and polls the table (`terminatePollMilliseconds`) up to `terminateGraceMilliseconds` (10 s); only members still alive get SIGKILL. Used for orphans, rosters and the settle stop. (The stall stop already had TERM, 10 s, KILL.) Windows keeps `taskkill`.
- Every signal is recorded before it is sent (`readyKillRecorderV1`): one bilingual line (`<English> / <German>`) with victim pid and name, signal, killer runner pid, stage, owner token and reason (`orphan`, `holder`, `roster`, `stall`, `restart`, `timeout`, `cancel`, `settle`), appended to `<checkout>/.cache/f/ready/kills.log` and said into the stage log, so the log of a victim names the killer. The reap line now names the owner token too. All texts in the schema `Texts` (en and de, same placeholders, checked by test).

### Stall means no life, not no log lines
- The stage sampler (every 5 s, `ps -A -o pid=,ppid=,rss=,time=`, POSIX) reads the processor time of the stage tree and every recorded roster member (`readyTreeCpuV1`). `readyLivenessV1`: alive when the tree spent at least `livenessCpuMilliseconds` (500 ms) between two samples, or a process joined the tree. Alive restarts both the stall clock and the hard-cap progress clock. A silent rustc is therefore not stalled; the hard cap (4 times the soft cap) still bounds it. Windows has no CPU sample and keeps the output-only rule.

### Suspect 3 (sidecar restart)
- `createSidecarRestartV1` itself kills nothing; `abort()` ends the sidecar's own owned command trees (`terminateOwnedProcessTree`, identity by pid tree of its own child), the `ready` role's stages are detached and survive a restart by design, and `claim()` of the replica supervisor signals only the pid of a fresh own sidecar record (name, entry hash, wall time within `aliveMilliseconds`). No foreign victim path, so no change. Residual, not changed here: owned-execution aborts are immediate SIGKILL without SIGTERM, and an agent role abort after the 300 s drain still ends rustc jobs it serves for other agents; moving that to TERM+grace is a separate change in `🏃️process/🪓️termination`.

## Files
- `READY/🟦️.ts` (token in env, `startedMs` precomputed, recorder, spare, grace, liveness, kill reasons)
- `READY/🧬️schema/🔣️.json` unchanged
- `READY/🌲️process-tree/🟦️.ts` (`spare` in `readyKillPlanV1`, `readyKillTreeV1` record/spare, `readyCpuMillisecondsV1`, `readyTreeCpuV1`, `readyLivenessV1`) and its `🧬️schema/🔣️.json` (`livenessCpuMilliseconds`, `CpuCase`, `spare`)
- `READY/🌲️process-tree/🧹️containment/🟦️.ts` and `🧬️schema/🔣️.json` (owner token, verdicts, shared, grace, kill record, texts)
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🗂️registry/🟦️.ts` (`owner` field)

Graph service identity files (`HostPolicy.serviceRevisionFiles`): **none touched**, no service revision bump. The registry module is in the sidecar import closure, so the sidecars restart once for new code (debounced).

## Tests (all green: 64 pass, 0 fail, via `repo tools exec bun -- test`)
- Language-agnostic fixtures: process-tree `🧪️tests/🔣️.json` (`tree` case with `spare`, four `cpu` cases), containment `🧪️tests/🔣️.json` (windows with stage, `owner` token cases, orphan cases rewritten for ownership: other checkout's token, other stage's token, no token, shared service, token wins over time).
- Independent check: Python oracles (`🐍️.py`) recompute tokens (hashlib), the orphan plan with spare/shared, the CPU parse and liveness, the spare kill plan; compared to the TypeScript output.
- Real processes: only token owners reaped while an agent's process, a shared service, another stage's token, a reused pid and a person's window survive; kill record in both languages naming the killer; SIGTERM-only for a polite process, SIGKILL after grace for one that ignores SIGTERM; a spared child survives a tree stop with group kill; a busy silent stage is not stalled while a silent idle stage is, with its recorded kill; the real registry writer writes `owner` from the environment; end-to-end token propagation through the stage runner into detached grandchildren.
- Suites run: `ready/🌲️process-tree/🧪️tests`, `ready/🌲️process-tree/🧹️containment/🧪️tests`, `ready/🧪️tests` (3 files, 64 tests, 147 s under load).

## Not verified
- On this host the live stage still has the old code until the agent sidecar restarts; the next stall or orphan reap will show lines in `.cache/f/ready/kills.log`.
- Windows liveness (no CPU sample) and the Windows kill path were only covered by the existing taskkill unit tests.

## Follow-up: graceful owned-execution aborts and drain of the agent sidecar

Paths: `FW` = `🧰️framework/🔨️modules/🏃️process`, `SC` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution`.

### 1. Owned-execution aborts: SIGTERM, declared grace, SIGKILL, bilingual record
- `FW/🪓️termination/🧬️schema/🔣️.json` (new) declares the policy (`graceMilliseconds` 5000, `pollMilliseconds` 100) and the bilingual `kill-record` text.
- `FW/🪓️termination/🟦️.ts`: `terminateOwnedProcessTreeGracefullyV1` / `terminateOwnedChildTreeGracefullyV1` find the owned tree (groups and descendants) first, send SIGTERM, poll for up to the grace, then SIGKILL only the members still alive plus anything the tree started meanwhile. Windows and an unreadable process table fall back to the old immediate kill. The old sync functions stay for their other callers; the tree snapshot and signalling were factored out of them (`ownedRowsV1`, `signalOwnedRowsV1`) and gained a `record` hook. `ownedKillRecordLineV1` writes `<English> / <German>`: victim, signal, killing process, command label, reason.
- `FW/🎛️owned-execution/🟦️.ts`: `terminate` (cancel, timeout, progress deadline, output limit, ceiling) now uses the graceful variant and writes each record line to the owner's stderr, never into the command's own output stream (the output contract is compared byte for byte with independent Node execution), so the log of the killing process names the victim, the killer and the reason.
- Tests (`FW/🪓️termination/🧪️tests`): fixture case `owned-tree-graceful` (no survivors, compared with the existing `tree-kill` cross-check), a real-process test (polite tree gets SIGTERM only, a tree that ignores SIGTERM is killed after the grace, records in order), and `records` fixture checked against the Python oracle `🧪️tests/🐍️.py`.

### 2. Agent sidecar restart never aborts work it serves for other agents
- Before: after `drain` the agent was aborted unconditionally after the 300 s grace, ending in-flight rustc actions of other agents. Now `createSidecarRestartV1` (`SC/🛰️sidecar/🟦️.ts`) retires the agent at the grace only if it serves nothing; otherwise it looks again every `drainPollMilliseconds` (5 s) until the actions end by their own deadline (the worker already enforces each action's `timeoutSeconds`). `drainMaximumMilliseconds` (2 h, schema `Restart.agent`) only bounds a leaked busy flag; when it is hit the line `drainBounded` is written. `drainingServed` is written once. Both texts are English and German (`SC/🛰️sidecar/🧬️schema/🔣️.json`).
- The agent reports its load: `runRemoteExecutionAgentV1` got `onRunning` (`SC/🛰️agent/🟦️.ts`), `runAgent` (`SC/🛰️sidecar/👷️agent/🟦️.ts`) maps it into `SidecarWorkV1.busy` and the new `running`; `SC/🛰️sidecar/🧰️shared/🟦️.ts` has the type. Drain-when-idle (`drainWhenIdleV1`) already aborted early the moment nothing runs, so an idle agent still restarts promptly.
- Tests (`SC/🛰️sidecar/🧪️tests/🔁️restart`): `🧫️fixtures/🔣️.json` (five busy timelines with the expected abort time), a simulation through the real restart logic, and the independent Python oracle `🐍️.py`; the existing agent test (no work reported: abort at the grace) still passes.
- Not covered: the replica supervisor's `claim()` (replacing a sidecar of an older code hash) and its shutdown `end()` still send SIGTERM and, after `graceSeconds`, SIGKILL to the sidecar; that is a supervisor decision about its own sidecars, not the code-change restart.

Identity files: none of the 18 `HostPolicy.serviceRevisionFiles` were touched (`sidecar/🧮️closure` is untouched; termination is not in the list). Sidecars restart once for the new code.

### Boundary check of the owned-execution bundle
`🎛️owned-execution/🧪️tests/🟦️.ts` failed its bundle-closure assertion on five neutral files: `🎒️pack/🔤️json/` (members, decode) and `🧬️schema/📄️source/🔗️closure/`, pulled in because the allowed validator lazily imports the schema closure, which imports the JSON decoder. They depend on neutral code only, so the allowlist `neutralOwners` of that test gained those two prefixes. The same test then showed that injecting the kill record into the command's output breaks the output contract, so the record goes to stderr of the owner.
