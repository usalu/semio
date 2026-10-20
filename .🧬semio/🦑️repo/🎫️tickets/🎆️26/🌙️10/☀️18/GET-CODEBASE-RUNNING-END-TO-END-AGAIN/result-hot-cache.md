# result-hot-cache: resident repo service + native file watcher (blazingly fast hot caches)

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`, new member `R` = `B/🏡️resident`. Tools, staged set, results: `C/hot-cache-tools/`.

## Status

READY for GO as a staged set (nothing applied, nothing under `L/` or `🧰️framework/🔨️modules/🏃️process/` touched; chain17 freeze respected). Today: `python3 C/hot-cache-tools/staged/apply.py` (dry run) = 53 files, 8 anchored edits, 0 problems, longest path 151 UTF-16 units. `sh C/hot-cache-tools/staged/verify.sh` (builds the natives, applies to a scratch root, runs every suite there, touches nothing else) = **96 tests pass**, Rust unit tests 8, `mutation-check.py` 19 of 19 mutations killed.

What is proven and what is not (details in sections 6-8): the resident service, the native watcher, the fence, the memo, capability tokens, governor, crash safety and the thin clients are implemented and measured in an overlay on a synthetic 33k-file / 1,500-project workspace. The real `repo` is not yet wired live: the composition (`repo service start`) and the real execution port are staged but never executed; reaching "tool starts within 500 ms" on the real CLI needs phase 2 (section 7).

## 1. Result in numbers

Host: macOS arm64, 10 cores, load average 26-48 during every run (3-5x oversubscribed by the fleet), so absolute tails are inflated; the ratios are reliable. Raw lines: `C/hot-cache-tools/results/prototype-1021.jsonl`, `warm-pool-real-closure.json`.

| Goal | Today's path (cited, `result-cli-latency.md`, re-measured where marked) | Resident path (prototype, measured) |
|---|---|---|
| warm no-op `run`/`test` | ~1.0-2.3 s module load (re-measured today on the real `run`+nx+bazel closure: 0.73 / 1.26 / 2.27 s) + ~1.2 s image/lease custody + 0.2-0.5 s membership proof and closure digest (re-measured: 199-535 ms steady, 3.2 s on a cold page cache) | native client: **p50 11.6 ms at load 26** (CPU 3.1 ms per command), p50 28.6 / p90 78 / p99 166 ms at load 46 (n=100, process spawn included). Bun client: p50 54 ms at load 26 (CPU 33.6 ms), p50 170 ms at load 44 |
| service-side answer (no client spawn) | - | min 1.1, p50 8-12, p90 12-34 ms; 11 ms of it is the fence (FSEvents flush); 0 stat calls and 0 hashes for a clean task (law test) |
| one-file edit, rebuild starts | whole path above, then the tool | edit -> first tool process stamp, `run` started right after the edit: **p50 37 ms, p90 136, p99 150** (n=40, load 48; 1-18 stale tasks, p50 7); `watch` mode: p50 62 / p90 90 / p99 136 ms (load 38), p50 15.6 ms at load 16 |
| edit -> dirty in the service | stat pass over all members | native FSEvents: p50 11.5-12.2 ms, p99 16-72 ms (n=200). Bun `fs.watch` fallback: p50 45 ms and a lost event in 200 |
| 16 agents at once | serialised behind exclusive locks | 16 clients x 40 requests: 181-362 requests/s, p50 46-74 ms, p99 59-219 ms, 0 refused |
| service cost | - | RSS 120 MB (31.7k files, 1,500 owners, 3,000 tasks, 172 results), watcher process 6.4 MB; ready 0.55-1.7 s after start (scan 0.4-1.5 s) |

Warm worker pool on the REAL closure (18 modules of the `run`+nx+bazel closure, `warm-pool-real-closure.json`): cold imports 728 / 2,273 / 1,257 ms; in a pre-warmed worker 0.3-0.5 ms; request to exit 26-68 ms for a trivial main; warm worker RSS 105 MB. This is the module-load share of finding 4 of cli-latency, removed.

Watcher storm: 20,000 file writes inside an excluded `node_modules`: 0 rows reach the service, watcher CPU 0.04-0.05 s. Restart replay: 20 of 20 offline edits recovered from the persisted FSEvents cursor, no rescan, no loss frame.

## 2. Architecture

```
agent / human                     per worktree + user                          per OS
launcher (native, 3 ms)  -- NDJSON over unix socket / named pipe -->  resident service (bun, engine .mjs)
  `semio-repo-forward`            capability token per request                  |  state (event-sourced projections)
  falls back (exit 75) when       languages, progress, cancel                   |  governor (bounded, fair per lane)
  nothing answers                                                              |  warm worker pool (tier 2)
                                                                               +-- semio-repo-watch (Rust, std only)
                                                                                    FSEvents | inotify | ReadDirectoryChangesW
```
- **Tier 1, in the service (re-entrant, no process globals):** `ping status cursor dirty affected run test run-many watch adopt issue drop-caches stop`. `run`/`test`: fence -> plan from the graph -> answer from the memo or execute only the stale tasks -> record. Results of tasks are an accelerator on top of the tool caches: the tool keeps the last word whenever the service has any doubt.
- **Tier 2, delegated:** the real CLI (`repo run <id> --service=off`) executes in a pre-warmed one-shot worker: `bun --preload warm-<hash>.mjs 📜️script.ts` imports the closure once, announces `ready` on fd 3, waits for one request line (argv, cwd, env patch), rewrites `process.argv/cwd/env` and lets the main script run as if started with that command. No change to the CLI, no shared globals (the findings of cli-latency item 3: mutated `process.env`, cwd and `console.log`). Cold fallback when no worker is idle. Worker count/idle/age bounded by policy.
- **State (`R/🗃️state`)**: append-only journal of watcher batches (bounded ring, cursor = instance + sequence) and projections: member/directory map (the conc-graph frontier), per owner file digests (only dirty files are rehashed), environment digest, task keys (command, environment, owner digest, dependency keys), result memo (`replay` text, outputs). Path -> owner by a segment trie (nested owners carve out their subtree). Output paths of tasks are routed to the producing task, never to inputs.
- **Watcher (`R/🔭️watch/🦀️rust/🦀️.rs`, one std-only file, no crate, no cargo)**: `watch` (FSEvents via CoreServices FFI with kernel exclusion paths, event ids, device UUID, replay from a cursor; inotify with recursive watches that never descend into excluded directories and directory move tracking; ReadDirectoryChangesW), `scan` (parallel directory walk emitting the conc-graph frontier shape: equal to Bun's `lstat` mtime floats, tested). Frames are NDJSON (`ready batch lost history-done error bye`), coalescing in Rust, exclusions before delivery, EOF on stdin ends the process (no orphan, tested with SIGKILL of the parent).
- **Fence (the correctness core)**: a request first writes a unique file into the fence directory and waits for its event; all earlier events on the same stream are applied before it resolves (stream order), so an agent that edits and immediately runs sees its edit. Concurrent requests share one fence per cycle; a request that arrives during a cycle waits for the next, never joins one that started before it. The same fence is the start canary: no event within the timeout means trust `verify`, never `exact`.
- **Trust levels** (reported by `status`, `welcome`): `exact` (events + fence), `verify` (every answer re-proved by stat of the closure's files, racy-window rehash), `none`. Falls to `verify` on: failed canary, a fence miss, watcher death (until the restart replays or reconciles), `watch-limit` (inotify; permanent for the scope), Bun portable backend failure. Devcontainer bind mounts that do not forward host edits are caught by the canary the same way.
- **Gaps** (`lost`: overflow, kernel/user dropped, must-scan, mount, root change, watcher exit): owners under the scope get stat proof, `reconcile(scope)` re-walks the scope and feeds the differences through the normal path, the journal refuses to vouch across the gap, remembered outputs re-verify by bounded signature instead of being rebuilt blindly.
- **Graph changes**: a changed `project.json`, `Cargo.toml`, `package.json`, `go.mod`, `BUILD.bazel`, `MODULE.bazel`, `pyproject.toml` or environment file marks the graph dirty; the next request re-reads the graph (progress phase `plan`, EN/DE text) and `reconfigure`s owners/tasks while keeping membership; unchanged results stay valid (law test: only the task with the changed command rebuilds).
- **Environment binding**: `SEMIO_BUILD_TOOL/MODE/WORKERS/GRAPH_POLICY` of a request must equal the service's, otherwise `refused(environment, retry=fallback)` and the CLI runs in-process.
- **Early cutoff / dirty set for conc-graph (`R/🕰️graph-hooks`)**: `membershipFromJournalV1` turns a `dirty --since instance:seq` reply into the host's `membership(previous)` result (patched frontier, changes, digest) - verified equal to a fresh capture, equal to the live `diffFrontiersV1`, and accepted by the live `frontierHoldsV1`. `createResidentHoldsV1` trusts fact rows whose path (or directory) the journal proves untouched and asks the old checker about the rest. Not wired into `G/🟦️.ts` (conc-graph owns it, chain17 freeze): the two seams are `membership` (line ~299) and `holds` (line ~290) of the host; the service itself already benefits through reload-only-when-dirty.

## 3. Protocol (schema first): `R/🧬️schema/🔣️.json`

One closed JSON Schema (draft 7, `additionalProperties:false`) is the authority for client frames, service frames, watcher frames, the discovery descriptor, capability payloads and the policy (limits, timers, command table, EN/DE texts). Runtime validation is a 100-line schema interpreter (no per-frame codegen); it is cross-checked against Python `jsonschema` on 1,239 mutated frames (130 valid, every definition has both verdicts) and against Ajv when installed.
- Framing: one JSON object per line, 1 MiB bound, UTF-8 strict, a violation is refused once and the connection hung up.
- Client: `hello{client,lane,languages[1-2],cap?}`, `request{id,command,args,cwd,env,cap?,deadlineMs?}`, `cancel{id}`, `ping`. Service: `welcome{service{pid,instance,startedAt,implementation,generation,watcher{backend,trust,cursor}},limits}`, `accepted{id,queue}` (the point after which a non-idempotent request must not be retried), `progress{id,phase,done,total,message{en|de}}`, `output{id,stream,text|b64}` (chunked at 64 KiB, surrogate safe), `result{id,exit,signal,served,timing,summary}`, `refused{id,code,message{en,de},retry}`, `notice`, `pong`. `retry=fallback` means run in-process.
- **Languages**: no default. The client sends an ordered list; the service renders only those (a request with none it can render is refused in both languages); refusals and notices before the handshake carry both. `repo service` resolves the language with the existing `resolveLanguageV1`.
- **Capabilities**: macaroon-style HMAC-SHA256 chain (`v1.<payload>.<mac>`), root key in a 0600 file inside a 0700 user-owned runtime directory (checked by service and client before any token is sent). Caveats `scope` (status/query/run/watch/admin), `projects` (names, `prefix*`), `notAfter`, `cwd` are conjunctive; a holder can narrow without the key and nobody can widen (stripped or reordered chains fail; tested). The launcher mints a 60 s token without `admin` from the key file; `repo service issue --classes run --for lib-a --ttl 3600` hands an agent exactly that. Rust and JS mint byte-identical tokens (tested), checked against Python `hmac` and RFC 4231 / FIPS vectors.
- **Cancel/deadline/disconnect**: abort signal per request; the tool process is interrupted (tested: tool saw the abort, result exit 130; a disconnect aborts too; a deadline refuses with `deadline`).
- **Endpoint**: `$XDG_RUNTIME_DIR` or tmp `/semio-repo-<uid>/<sha256("semio-repo-service-v1\0"+realpath)[:12]>/s` (<96 bytes, falls back to `/tmp`), Windows `\\.\pipe\semio-repo-<uid>-<id>`; one descriptor file `service.json` (pid, instance, endpoint, implementation digest, `stopping`).

## 4. Capacity governance, idle, crash safety

- **Governor**: at most `maximumInFlight` (8) requests run, `maximumQueued` (64) wait in per-lane FIFOs served round-robin (one lane per agent/terminal), the queue position streams as `progress(queued)`, a full queue is `refused(busy, retry=later)` (the CLI exits 75, it does NOT fall back and add load). Tested: bounds, queue, busy, fairness (a lane arriving later is served before the earlier lane's last request; FIFO fails the test).
- **Memory**: resident bytes checked every 5 s against `maximumResidentBytes` (512 MiB): drop memo/digests first, restart (`exit 75`, reason `memory`) when still 25 % over. Warm workers (105 MB each, default 1; 0 on Windows until fd-3 pipes are verified there) are claimed through `ports.capacity` (host capacity gate; unwired, see 7).
- **Idle**: no clients and nothing in flight for 15 min -> graceful shutdown (tested with a short timer); custody pins are released after 20 s idle so `dependencies sync`/exclusive maintenance are never starved, and re-acquired on demand (tested with counters). `custodyPortFromPoolV1` adapts the existing `createRepositoryCustodyPoolV1` (idle/age limits already there).
- **Crash safety**: single instance by listening on the endpoint (`EADDRINUSE` + connect probe: live -> `ServiceRunningErrorV1`, stale socket -> unlink and take over); `kill -9` of the service: clients fall back at once (exit 75 in ~1-10 ms), the watcher exits by itself (EOF), the next service replays from the saved cursor + device UUID (state file written atomically, debounced). A client that loses the service after `accepted` returns 70 and never silently re-runs. Own source changes (`implementationPaths`, digest over the member's files) make the service refuse with fallback, drain and exit 75 (stale reader, like `revision-fix`).
- **Mid-session faults tested**: watcher process SIGKILLed mid-session (answers stay correct through `verify`, trust returns to `exact`), no watcher at all (`none`/`poll`: silent edit and deleted outputs still found), oversize/garbage/invalid UTF-8 frames, id reuse, double hello, unknown command.

## 5. Soundness laws (all executable)

1. Read-your-writes: 60 and 100 edit-then-run iterations; the executed set equals the reverse-closure oracle each time and a second run executes nothing.
2. A result is filed under the key computed BEFORE the run (an edit during the run can only make it stale, never wrongly current); failed tasks and outputs over 1 MiB are never remembered.
3. Outputs: deleting or changing a produced file rebuilds exactly its task; an unrelated output directory does not; events during the task's own run are ignored by construction (fence, then `endTask`).
4. Membership and digests equal naive oracles after every step of 4 x 70 random operations (create, modify, delete, rename across owners, mkdir -p, rm -rf, atomic save, chmod, env edits, excluded and output writes, spurious/duplicate/reversed event rows); owner digests also equal Python `hashlib`; net journal changes equal a before/after snapshot diff; task keys change iff the closure fingerprint changes.
5. A clean task touches the disk zero times (stat/hash/refresh counters), 100 repeated keys.
6. Capability narrowing, forging, expiry, cwd and project scoping; bilingual refusals; `issue` needs `admin`.
7. Environment and graph binding, stale implementation, bounded journal (too old or foreign cursor proves nothing).

## 6. Cross-platform

- macOS: everything above runs. Linux and Windows backends are type-checked on macOS with forced cfgs (`rustc --emit=metadata --cfg semio_check_linux|windows`, no warnings of mine) but NOT executed (no Linux/Windows host, docker down). Node-level code uses `node:net`, `node:fs`, `node:crypto`, `node:path` only; the Windows pipe name, the fd-3 handshake of the warm pool, `windowsHide` detach and `LOCALAPPDATA` paths are untested there.
- Bun quirk found and fixed: a `node:net` client without an `end` handler never gets `close` after the server hangs up; both clients now destroy on `end`. `process.exit()` after piped writes truncated large output; the entry now sets `exitCode` and drains.
- No new runtime dependency: Rust std only (FFI declared by hand, no `libc`/`windows-sys`), JS `node:` builtins only. Ajv and Python `jsonschema` appear only as test oracles.

## 7. Integration and what remains

Staged edits (all anchors resolve on the live tree today): `B/📜️script.ts` (`repo service` branch next to `concurrency`; `RepoScript.run` tries the service first for plain `run/test/run-many`, flags that can change a task disqualify, `--service=auto|off|require` or `SEMIO_BUILD_SERVICE`), `B/🧭️invocation/🟦️.ts` (`--service` is consumed like `--workers`), `B/⚙️operations/🔣️catalog.json` + its schema (operation `service`, flags, `OperationName`; schema errors unchanged at the 4 pre-existing ones), `.gitignore` (the fence directory). After the apply: regenerate the help snapshots (`C/conc-graph-tools/regen-help.ts`).
- **Launcher (repo-launcher)**: embed the six steps of `R/🚪️client/🦀️rust/🦀️.rs` (endpoint from the realpath, trust check of the directory, read key, mint token, send `hello+request` in one write, stream frames, exit 75 on `retry=fallback`/no service); `--print-capability` and `--print-endpoint` make parity testable. The bun path (`RepoScript.run`) still pays the door (~0.2-0.5 s) first: the ≤ 300 ms goal on the real CLI is the native launcher + service (measured: 11-30 ms).
- **Provisioning**: `build-native.py` compiles the watcher and forwarder with plain rustc (ticket tooling; it is a direct tool invocation and belongs behind tool-iface's registry as `tools setup watcher`; the service finds the binary by `SEMIO_REPO_WATCH_BINARY`, else runs the portable backend in `verify` trust and says so).
- **Phase 2 (not done, needed for "rebuild starts ≤ 500 ms" on the real CLI)**: the worker child still runs the door, capacity gate and lease/image custody itself (~1.2 s cited; cli-latency `command-custody` removes part of it). Two ways, both with existing seams: (a) the service runs inside an owned invocation (it is started through the door) and hands each worker the issued envelope of `scriptInvocationEnvironment(invocation, env)` so children skip the door like nested scripts do; (b) the execution port calls the tool adapters directly in the service (Bazel slot clients and the dependency generation pinned in the service, the pin protocol `custody.acquire/release` is already in place). The prototype shows the service share of that budget is ~40 ms.
- **conc-graph seams** (`membership`/`holds`) and **adopt** (`adopt <generation>` is implemented; the cursor<->generation side file is not).
- Not done: `ports.capacity` (host capacity claim for the service and workers), the real `loadGraph` wiring was never executed (`createForegroundRunnerV1` and `repo service start --foreground` exist and are typed, not run), `tsc` not run (declarations are hand-written `.d.mts`), memo for tasks without declared outputs trusts the tool's own output store (Bazel/Nx outputs must be declared in the neutral graph to be validated), Linux/Windows execution.
- Bun `fs.watch` is NOT a substitute: in 200 edits it lost one (2 s timeout) and once failed the canary; the design degrades to `verify` instead of trusting it.

## 8. Verification

| Suite (staged path under `R/🧪️tests/`) | Tests | Notable oracles |
|---|---|---|
| `📨️protocol` | 24 | Python `jsonschema` (1,239 frames), Ajv, Python `hmac`/`hashlib`, RFC vectors, byte-identical Rust tokens and endpoints |
| `🗃️state` | 13 | naive snapshots after every step, Python digest, closure fingerprints |
| `🔭️watch` | 13 | real FSEvents (fence ordering 25 x 40 files, replay 30/30, orphan on SIGKILL, scan equal to `lstat`), scripted watcher (lost, escapes, canary, restart/backoff, watch limit) |
| `🏛️service` | 25 | end to end over the socket: memo/plan/outputs/env/graph change, capabilities, abuse, cancel/deadline, governor fairness, lifecycle, degraded trust, large output, native forwarder, `kill -9` |
| `🕰️graph-hooks` | 9 | live conc-graph `diffFrontiersV1` and `frontierHoldsV1`; neutral graph mapping |
| `🏊️delegate` | 5 | real `bun --preload` workers: argv/cwd/env/streams/exit/cancel/idle |
| `⌨️command`, `🚪️client` | 3 + 4 | `repo service` subcommands EN/DE, eligibility and mode rules |
| Rust (`rustc --test`) | 8 | FIPS 180-4, RFC 4231, JSON, base64url, exclusion boundaries |

`mutation-check.py` (staged): 19 mutations (events not marking dirty, key ignored, gap without stat proof, journal across a gap, parents not re-stat'ed, output events, no fence, token not checked, caveats not enforced, result filed under the key after the run, environment binding, unbounded queue, FIFO, instant fence, tamperable tokens, open schemas, dropped parent directories, env not applied in workers, eligible-any-flag): 19 killed (one first equivalent mutant - recording the old key although inputs moved - was replaced because it is semantically harmless).

## 9. Apply (after GO)

```
cd /Users/ueli/Documents/semio; C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN; S=$C/hot-cache-tools/staged
python3 $S/apply.py                       # dry run, refuses on drift or ambiguous anchors
sh $S/verify.sh                           # optional: natives + scratch root + 96 tests
python3 $S/apply.py --write               # 53 new files, 8 edits
python3 $S/build-native.py <dir>          # watcher + forwarder; export SEMIO_REPO_WATCH_BINARY=<dir>/semio-repo-watch
bun C/conc-graph-tools/regen-help.ts      # help snapshots after the catalogue edit
bun repo service start; bun repo service status
```
Rollback: delete `R`, reverse the 8 edits (each carries both strings in `apply.py`), remove `.🧬semio/🦑️repo/🏡️resident-fence/` from `.gitignore`. Nothing persistent beyond the per-user runtime directory (removed by `service stop`) and `state.json` inside it.

## 10. Reproduce the measurements

`C/hot-cache-tools/staged/dev/bench/` (`workspace.mjs` builds the synthetic workspace, `serve.mjs` starts a service on it, `bench.mjs noop|direct|edit|watch|concurrency`, `legacy.mjs`, `events.mjs`, `replay.mjs`, `all.sh`); `C/hot-cache-tools/warm-bench.mjs` + `warm-main.ts` + `mods.json` for the real-closure worker numbers (they use `/private/tmp/hot-cache` paths; the scratch tree was deleted).
