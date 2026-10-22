# Result agent-session (executor agent-session, wave 1021a)

READY: staged, verified on overlays, NOT applied (waiting for the coordinator's GO). Nothing outside `C/` and `/private/tmp/agent-session/` was written (one cleanup of test records in the user cache, section 9).

`C` = this ticket folder. `F` = `🧰️framework/🔨️modules/🏃️process`. `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`. `L` = `M/📚️library`. Tools: `C/agent-session-tools/` (`stage.py`, `apply.py`, `progedits.py`, `overlay.py`, `battery.py`, `compare.py`, `mutation-check.py`, `verify.sh`, `go-run.sh`, `tsc-check.sh`, `gen/` = generators of the schema and the fixture, `staged/edits.json` + `staged/files/`).

## 1. What was wrong (evidence)

| # | finding | evidence |
|---|---|---|
| A1 | Identity was derived five times, each with its own rule: `SEMIO_CARGO_LANE` or the nearest client ancestor (cargo lane), `SEMIO_SESSION` (TypeScript compiler), `SEMIO_AGENT_ID` (run leases, output cache, MCP, resident client), `agent`/`lane` strings (MCP result, service token), nothing at all in Bazel slots, test sandboxes, the lease registry and capacity grants. | `rg` over `F`, `L`, `M` before the change: 6 files read the three variables, 4 more derived identity by themselves. |
| A2 | **The old client-process rule matched the whole command line.** `clients=/(claude\|codex\|…)/iu` was tested against every ancestor's command, including Claude Code's own shell wrapper `/bin/zsh -c source ~/.claude/shell-snapshots/…`. That wrapper is a new process per Bash call, so every call of a Claude Code agent would have received its own lane and never reused a warm one. | vector `the nearest client process, not the shell that mentions the client in a path` in `F/🪪️agent-sessions/🧪️tests/🧫️fixtures/🔣️.json`; the new rule looks at the executable (or the script argument of node/bun/python), and one `claude` ancestor now names this very session across all the Bash calls of this task (one session identifier in the registry for dozens of calls). |
| A3 | The macOS ancestry walk spawned `ps` once per ancestor (up to 16 spawns). | now one `ps -axo pid=,ppid=,lstart=,command=` listing (measured 75 ms at load 25). |
| A4 | Nothing ordered the machine admission queue between agents: one agent with many requests served first-come-first-served kept the others waiting. | `orderHostWaitersV1` had priority, arrival, id only. |
| A5 | No place showed who holds a lease, a capacity grant, a Bazel slot or a test sandbox. | `repo leases status` rows had pid and command only. |

## 2. Design: one concept, one schema, one API

Module `F/🪪️agent-sessions/` (schema first: `🧬️schema/🔣️.json` holds the closed `AgentSession`, the authored `Policy` and the EN/DE `Messages`; code in `🪪️identity`, `🧭️ancestry`, `🗄️store`, `🌿️current`, `🗣️messages`). Facade `F/📦️packages/🟦️typescript/🪪️agent-sessions/🟦️.ts`, package export `./agent-sessions`.

A **session** (record `<registry>/<worktree key>/<id>.json`, per user, registry next to the lease registry, override `SEMIO_AGENT_SESSION_DIRECTORY`):

| field | derived from | read by |
|---|---|---|
| `agent` `{id, client, origin}` | explicit name, else the client's own variable (`CLAUDE_CODE_SESSION_ID`, `CODEX_THREAD_ID`), else the MCP connection, else the nearest agent-client ancestor, else the default agent | everything |
| `id` | `a` + 10 base-36 characters of a domain-separated digest of the agent id (`d` for the default session) | everything |
| `resources.cargo.lane` | the id | Cargo launcher (`withRepositoryCargoLaneV1`), lane metadata records session and agent |
| `resources.bazel.preference` | digest, 31 bits | `rankBazelSlotsV1`: tie-break for the first choice (`preference mod slots`), so concurrent agents start on different slots; the slot state records session and agent |
| `resources.nx.namespace` | digest, 12 hex | optional group `nx-watchers` (section 7) |
| `resources.tests.root` | first 6 characters of the id | test sandboxes live in `<tmp>/st/<root>/<8 hex>`, owner row names the session; temp root `<tmp>/st/<root>/t` is created by `session open` |
| `lease.owner` | `<agent>/<id>` | lease registry rows carry the session address; `leases status` shows `session <id> (<agent>)` |
| `budget.weight` (1-16) | `--weight`, default 1 | `agentSessionSharesV1` (largest-remainder split of 10000 basis points between live sessions); admission queue order (A4) |
| `capability` `{scopes, projects, expiresAtMs}` + private token file | `repo service issue` (the service holds the key); the record never carries the token | resident client and service |
| `anchors` (≤ 8 `{pid, started}`) | the client ancestor, `--owner-pid` | liveness and garbage collection |

The only thing that travels between processes is the **address** `<worktree key>.<id>` in `SEMIO_AGENT_SESSION` (`agentSessionEnvironmentV1`). `SEMIO_AGENT_ID`, `SEMIO_SESSION`, `SEMIO_CARGO_LANE` and `SEMIO_SERVICE_TOKEN` are gone, no fallback reads them (source law in two test suites fails when one reappears). `currentAgentSessionV1()` is the one read API: environment address, else derivation, opened on first use (same identity, same session), degraded in memory when the store cannot be written (a read-only home never stops a build); the CLI door uses the strict form.

State of a session: `live` (seen within 10 minutes or the default session), `idle` (owner alive within a day, or seen within an hour), `gone` otherwise. `collectAgentSessionsIfDueV1` runs at most once a minute from the door: gone sessions are reclaimed (test sandboxes with images detached first, temp root) and removed; a session whose sandbox still runs is kept; the default session is never collected.

## 3. `repo session open|status|close|list` (EN/DE)

`L/🏗️build/🪪️agent-sessions/⌨️command/` (+ `🟦️.ts` door helper `attachAgentSessionV1`). Catalogue operation `session` with four subcommands and eight flags (`--agent --client --client-pid --owner-pid --weight --export --dead --force`, `--json` and `--all` extended), added by `progedits.catalog` at apply time (it locates JSON structure, survives other sets that change the catalogue layout: checked against the round-6 overlay where rows no longer carry `implementations`); help snapshots regenerated by the existing `regen-help.ts`.

- `open [--agent=<n>] [--client=<n> --client-pid=<p> --owner-pid=<p>] [--weight=<w>] [--export] [--json]` opens or attaches (idempotent); `--export` prints `SEMIO_AGENT_SESSION=<address>`.
- `status [<id>] [--json]`: identity, state, owner, every derived name, budget share, capability, token present, and what it holds right now (leases of the checkout, capacity grants, test sandboxes).
- `close <id> [--force]` refuses a live session; `close --dead` collects every gone one.
- `list [--all] [--json]`.
- Door: every heavy `repo` command attaches before the capacity gate and puts the address into its environment; light commands (`leases`, `capacity`, `session`) do not pay for it. `repo capacity status` prints a session block (grants, reserved MB, waiters, weight, share) and `--json` gains `sessions`.

## 4. Where the adapters changed (all through the one API)

Cargo: `🦀️cargo/🚪️launcher` takes lane and owner from the session; `unit-cache/🧵️lane` lost `resolveLaneIdV1`, the ancestry readers and `DEFAULT_LANE_ID_V1` (moved to `F/🪪️agent-sessions/🧭️ancestry`, vectors moved with them). Bazel: `🧵️sessions` (`preference`, `owner` in request and slot state, schema extended), `📜️script.ts` passes both. Test sandbox: `📁️root`, `♻️reclaim` (`reclaimAgentSessionSandboxesV1`, `listSessionSandboxesV1`, session-aware sweep). TypeScript units: `session = currentAgentSessionV1().id`. Run leases / output cache / blob storage / retention: `agentId()` is the agent of the session. Leases: registry entry field `session`, status holders/waiters `session`+`agent`, message `sessionTag`. Capacity: waiter `session`+`weight`, grant `session`, `orderHostWaitersV1(waiters, ledger)` orders by priority, then reserved memory per weight, then arrival, `withBuildCapacityV1` and `capacity run` pass the session. MCP: the Go server asks `repo session open --json` once per agent (or per connection: client name, client pid, own pid), caches the answer, passes only `SEMIO_AGENT_SESSION`; failure leaves the call running with a hint; policy `identity` shrank to the argument and one variable name, four tools `session_open|status|list|close` were added with their golden vectors. Resident service (group `service`): below.

## 5. Staged set

`python3 C/agent-session-tools/apply.py --root <checkout> [--group core|routing|nx-watchers|service|all]` (dry run unless `--write`; `--backup DIR`; `--undo DIR`; refuses while a dependency sync or `show projects` runs unless `--ignore-running`). Edits are data (`staged/edits.json`): anchored hunks (must match once; an applied hunk is recognised, a moved anchor is refused, never guessed), whole-file replacements guarded by the base hash, programmatic edits for JSON structure, new files (16), deletions (2: the lane suite whose vectors moved). Longest path 114 characters.

| group | edits | notes |
|---|---|---|
| core | 58 (16 new files, 31 hunk files, 8 programmatic, 1 replace, 2 delete) | module, CLI, all adapters, MCP surface and tests, README (EN/DE), package export, two F suites registered as `test-agent-sessions[-adapters]` |
| routing | 2 | the hand-written verb lists of the Go and Rust launchers; skipped by itself when the launcher derives routes (repo-launcher set, round 6) |
| nx-watchers | 2 | optional, section 7 |
| service | 6 | needs the hot-cache set applied |

Order for round 6: after repo-launcher and tool-iface (routing then skips itself), `service` immediately after hot-cache, `regen-help` (round 6 step) afterwards; then `bun repo run repo-mcp:build && repo-mcp:install` and `/mcp` reconnect (the running binary has neither `repo session` nor the four tools); `bun repo tools setup nx` if the frozen authorizer closure includes the edited leases/bootstrap files (conc-nx needed it for its enforcement files; not provable here). Apply when no build library code is being loaded by a running chain (the set edits `L/🏗️build/📜️script.ts`, `F/🔒️leases`, `F/🏗️build` and the sandbox; chain17 freeze applies).

## 6. Verification (overlays under `/private/tmp/agent-session`, `nice -n 10`, one heavy process at a time)

- **Apply**: dry run on the live checkout now: 62 pending, 0 problems. Fresh clone of the live tree (`ov-v0`), apply core+routing+nx-watchers: 62 files, second run 62 applied / 0 changes; the result equals the finished overlay file for file wherever the live file had not moved (2 files had moved under the peers meanwhile and the hunks still apply); the pristine pre-change tree and the tree with the hot-cache set apply too (62 and 68 pending, 0 problems). **Round-6 snapshot** (integrate2's overlay with repo-launcher, tool-iface, command-form codemod, cli-latency, pm-seam, runtime-seam, … applied, cloned at 10:50): 60 files written, the two routing files skipped by themselves, 0 problems; there the new suites pass (14 + 8), sandbox 17, Bazel sessions 21, and the failures of the operations and build-surface oracle suites are the snapshot's own (identical before and after the set). The command suite's source law correctly reports the hot-cache resident client, which still reads `SEMIO_AGENT_ID`: **apply group `service` in the same step as the hot-cache set**.
- **Battery** (`battery.py`, base vs applied clone, same machine, load 28-42): no test fails that did not fail before. Agent-session suites 14 + 8 + 9 pass (core, adapters with real child processes, command), sandbox 17 (was 16), host capacity 18 + 16 + 9, lease order 58, bazel sessions 21, TS units 21, build-surface oracle 5, unit-cache suite with real Cargo **22 pass** (66 s). Pre-existing failures are identical before and after (operations 4 with byte-identical messages, bootstrap dependencies 9, artifact-root-source 8, discovery-contract 1, lease processes 1-2 that flip with load).
- **Language-agnostic vectors** (`F/🪪️agent-sessions/🧪️tests/🧫️fixtures/🔣️.json`): 16 identity rows (hand-stated fields + pinned full rows), 15 client-process rows, 9 state rows, 7 share rows, the moved `/proc`, Windows and `ps` ancestry vectors; an independent Python derivation (`🐍️oracle.py`, hashlib, no shared code) must equal the TypeScript result on every row; Ajv (strict) validates the whole schema, a record, the ledger rows, the JSON of the CLI; messages exist in EN and DE with equal placeholders and every message id the code uses is authored.
- **Mutation check** (`mutation-check.py`): 18 mutations (precedence, default session, client matching by argument, idle boundary, share tie-break, anchor bound, rewrite of unchanged records, default collection, kept resources, token mode, ignored address, admission weight and held memory, Bazel preference and owner, sandbox root, lease registry session, sandbox reclaim): **18 of 18 killed**; two survivors in the first run led to two more assertions.
- **Go**: the repo MCP module with the canonical Go overlay: `go vet` clean, `go test -short` ok (new: one open per agent, failing open leaves the call running with a hint, request argv, reply parsing; golden vectors incl. 8 for the new tools; TS/Ajv oracle 5 pass).
- **Types**: `tsc --strict --noUncheckedIndexedAccess` over the 23 new and edited TypeScript files: 0 errors in them (9 pre-existing errors in untouched files, identical on the live tree).
- **Composition with hot-cache** (group `service` on a tree with the hot-cache set applied): protocol 23, executor 7, delegate 7, graph hooks 12, state 13, client 8, command 3 pass; service 30 pass (28 before + 2 new: issue opens the session, records the capability, keeps the token in the private file and the client then runs without a token variable; the executor environment carries the address of the connection's agent and replaces a forged one) with the same 3 deterministic + 2 load-flaky failures as before.
- Sandbox concurrency law (`⚖️law`) : `fresh: []` before and after (stale rows come from directories outside the framework-only overlay).

## 7. Decisions and limits (said plainly)

1. **Nx**: the namespace is issued and visible; only the optional group `nx-watchers` uses it (it is part of the watcher identity, so each agent gets its own watcher graph store and daemon socket, test updated). Not in `core` because detached supervisors (nohup) lose the process ancestry and must carry the address first, otherwise a supervisor and the commands that probe it would compute different watcher identities. Finite Nx runs keep sharing the digest namespace and daemon by design (conc-nx).
2. **Budget** is enforced as order in the admission queue (reserved memory per weight), not as a hard cap per session; a lone agent is never throttled.
3. **Token**: minted only by the resident service (it holds the key); `repo session open` issues none. `status` says whether a token file exists.
4. **Native launcher (hot-cache, repo-launcher round 6) is NOT covered**: `dev/go/resident.go` reads `SEMIO_AGENT_ID` and `SEMIO_SERVICE_TOKEN` (lines 293-296 of the staged file). It must read `SEMIO_AGENT_SESSION`, the record `<registry>/<worktree key>/<id>.json` (agent id) and `<id>.token`, using the same registry path rule (`agentSessionRegistryRootV1`, `agentSessionWorktreeKeyV1`: sha256 of `semio-agent-session/v1\0worktree\0<realpath, case-folded on macOS and Windows, backslashes as slashes>`, first 12 hex). Without an address a native call announces the default agent, as it announced `launcher` before. Deriving the agent natively (no address) needs the identity rules ported to Go; the vectors for it are the identity rows of the fixture. Not done because that Go code lives in two unapplied sets and could not be built here.
5. **Not run**: Windows and Linux hosts (readers use `node:` APIs and injected ports; the Linux and Windows ancestry vectors are replayed), the taxonomy/leaf gates for the new member directories (names reuse admitted emojis; registry shows no per-member entry for sibling modules), a real `repo session` through the built launcher (needs the rebuilt binary), Rust compile of the one-token edit in the Rust entrypoint list (skipped when the launcher derives routes).
6. Tests of the whole repository that create a sandbox now resolve a session once per process; without an address and outside a client they derive from the ancestry (one `ps` listing) and write one small record per worktree and agent into the per-user registry (bounded, collected after an hour).
7. The temp root is issued, created by `session open` and collected with the session; no tool writes into it yet (the per-run test `TMPDIR` stays inside the sandbox on purpose: socket path budget).

## 8. After GO

1. `python3 C/agent-session-tools/apply.py --root . --group core,routing,nx-watchers` (dry run), then `--write`; `service` after hot-cache; `bun C/mem-gov-tools/regen-help.ts .`.
2. `bun repo run repo-mcp:build && bun repo run repo-mcp:install`, reconnect `/mcp`.
3. Smoke: `repo session status`, `repo session list --all`, `repo capacity status` (session block), `repo leases status` while two agents run `repo cargo`; two concurrent `repo --tool=bazel run` of different targets should print `slot s0` and `slot s1` without a waiting line; the MCP tool `session_status`.
4. Tell the hot-cache/repo-launcher owners about item 4 of section 7.
5. Rollback: `apply.py --root . --undo <backup dir>`.

## 8b. READY2 (re-audit r3)

Fixed the two docstrings without an emoji lead (the Go test `reportScript` comment and the module docstring of `🐍️oracle.py`); a scan of every new and edited docstring of the set (TypeScript `/**`, Go declaration comments, Python module docstrings) finds no other. The 8 `prog` edits carry their text in `agent-session-tools/progedits.py` (each function is idempotent and its result hash is `sha` in `staged/edits.json`). While re-running the Go suite I also fixed a flake that my change had made visible: the test peer dropped a response that arrived before the one it awaited, and the extra `session open` before the slot gate made the arrival order random; the peer now stashes responses by id (6 of 6 runs green, was 1 of 4). Re-staged: 62 core/routing/nx-watchers edits + 6 service, dry runs clean on the live tree, the pristine tree and the hot-cache tree.

## 9. Housekeeping

Test runs of overlays wrote session records for the overlay paths into the real per-user registry (`~/Library/Caches/semio/agent-sessions`, a directory that did not exist before this set); they and the empty sandbox homes under `$TMPDIR/st/` that the runs left were removed (the sandbox now removes its empty session home itself and tolerates a concurrent removal). What stays: `/private/tmp/agent-session/{ov-a (finished sources), ov-base, ov-hc2c, ov-hc2 (staging bases), ov-v0, ov-v (verification), gocache, results}`; APFS clones, delete when the set is applied.
