CAUSE: the whole-inference settlement loop (`settleObservedComputationV1` around all 3,735 projects) restarts at project 0 on the first thrown failure of a round, discards the failure, and treats it as transient because peers keep editing consumed files, so a failure that returns unchanged is repeated up to 64 times; the failure text itself is unknown (never logged).

# result-loop-hunt: why `show projects` (pid 97996) restarted inference 9 times (wave 1020e, executor loop-hunt)

`L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `C` = this ticket. Read-only on the process and the real tree; the process was stopped by someone else at 22:07 (`Original Script process terminated`, SIGTERM path), I never signalled it. Staged, nothing applied: `C/loop-hunt-tools/` (`apply.py` dry run, `verify.sh`).

## 1. Mechanism (code, certain)

- The restart owner is the settle loop: `L/🏗️build/🕸️graph/📥️inference/🚦️invocation/🟨️.mjs` line 40 runs `settleObservedComputationV1(() => emojiProjectJsonNodes(...), settlementOptionsV1(ctx))`; the loop in `L/🏗️build/👁️observation/🟨️.mjs` (558-578) calls `compute` again from scratch whenever a round is not clean. Re-entering `emojiProjectJsonNodes` replays exactly the log sequence seen after every `build:settle`: `cargo-owners` (161 events), `commands`, `command-source-imports`, `playground-components` (316), `playground-variants/native`, `cargo-targets`, then `Inferring projects: 0 of 3735` (log lines 12925/13665, 25549/26255, 37806/38527, 49559/50241, 60646/61333, 72586/73254, 80743/81415, 85781/86448).
- A source conflict (a consumed file read at two states) is only detected after a round has completed (proved on the real module, overlay `settle-exp.mjs`: 30 of 30 projects run, then `conflict`, then the whole round again). The observed rounds ended at projects 678, 693, 661, 662, 675, 682, 659 and 937 of 3,735, so they ended by a thrown failure, not by a conflict.
- A failed round is rethrown only if all consumed facts still hold and the youngest consumed file is older than the round start (`youngest<started-RACY_MS`). Over a 13-25 minute round that reads about 10,000 files per project (the shared script/library closure), some file is always younger or changed: `L` saw 47 files edited in 2 h, 6 in the last 5 min, the Codex peers edit `⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts` every few minutes. So the failure is classified transient, `onRound` is called, the loop starts over, up to `limit = 64` rounds (14 h).
- `onRound` receives `{round, conflict | failure}` but `settlementOptionsV1` drops it: stderr has no error text in 2 h 30 (checked: only `[script]`, `[capacity]`, `[DATA]`, `[input-precision]` rows).
- Every project record observes the shared closure, so one peer edit of it stales all of them: record `a499a0ce…` (written 21:20) lists `📈️progress.json` with size 14,888, now 18,339 (edited 21:35:45) and is the only stale row (`chk.mjs`); round 9 therefore recomputed positions 13-314 that round 8 had just written.

## 2. Evidence from the persisted state (np records, `order.mjs`, `missing.mjs`)

- Job order = byte-sorted `📋️project.json` position; records written in a window map to a contiguous prefix (round 9: positions 13-314 in 21:36-21:50, 600 at 22:03).
- Persistence only worked from 21:10: 9 records written 19:37-21:09, 875 after (the peers' storage fix at 21:03). Before that every round recomputed about 680 projects without saving them.
- Round 8 died at 21:35:02-21:35:15: positions 936 and 938 written 21:35:00/:02, position 937 and 939-944 (the 7 in-flight) never written; a peer edited `🏃️process/…/📈️progress/🟦️.ts` at 21:35:06. Project 937 alone infers without error (`probe-projects.ts` plain mode on positions 936-938), so the failure is not content of that project.

## 3. Not proven (be explicit)

- The thrown message. Candidates the code allows (all retried as transient): `BUILD_SOURCE_UNPARSEABLE` rethrown for a source modified in the last 2 s, `MODULE_NOT_FOUND` from the command import walk of a half-moved module, `Inventory record changed while opening/reading` or `Single-link bounded regular inventory record required` from the lean leaf reader in the strict reuse check (a peer `repo` process republishing a leaf), a storage custody check (`Original optional ledger inode changed`) after a peer ledger reset. Leaves are healthy now (18,682 leaves: nlink 1, sha = name, 0 anomalies, `leafscan.mjs`). The staged `[graph-settle]` row names it on the next run.

## 4. Staged fix (`C/loop-hunt-tools/staged/ops.json`: 10 anchored edits in 4 files + 4 new files; `python3 C/loop-hunt-tools/apply.py check|diff|apply [--write]`, dry run default, 0 drift on the live tree)

| change | file |
|---|---|
| a failure that demands a restart (`BUILD_AUTHORITY_CHANGED`) or returns unchanged (same code + message) in the next round is rethrown at once with its facts retained; conflict rounds reset the memory; thrown `undefined` and frozen objects keep working (existing law: 6 of 6) | `L/🏗️build/👁️observation/🟨️.mjs` (settle, inline, no new binding) |
| one machine-readable row per repeated round: `[graph-settle] {"round":N,"failure":"…","code":"…","project":"<root>"}` or `{"conflict":"<path>"}` | `L/🏗️build/🕸️graph/⏱️phases/🟨️.mjs` + `.d.mts`, `settlementOptionsV1` in `L/…/📥️inference/📦️project-nodes/🟨️.mjs` |
| the failing project is named on the error (`semioProject`, property only, error identity and code unchanged) | `inferProject` in the same project-nodes file |
| new law `L/🏗️build/🧪️tests/🕸️graph/🔁️settle-repeat/` (14 language-agnostic scenarios, closed schema EN/DE, independent Python oracle `🐍️oracle.py`) | 4 new files |

Anchors are disjoint from conc-graph READY3 (it touches five other anchors of project-nodes and other parts of the observation file); apply order does not matter, drift refuses.

## 5. Verification (`sh C/loop-hunt-tools/verify.sh`, overlay `/private/tmp/loop-hunt/ov`, live tree untouched)

| check | result |
|---|---|
| repeat law on the live function (control) | fails at `repeated-young-failure-is-genuine` (3 rounds, accepted) |
| repeat law on the patched function | 14 scenarios pass, Python oracle equals fixture |
| existing settlement law (6 cases) on the patched function | pass |
| mutations (no repeat rule, no restart rule, signature without message / code, conflict keeps signature, signature never remembered, repeat ignores `failed`) | 7 of 7 killed |
| real observation module, 12 projects, one deterministic failure, a peer rewrites a shared source every 100 ms, `limit 8` | live: 8 rounds, 83 project inferences; patched: 2 rounds, 21 inferences, same error surfaced |
| row writer on thrown `undefined`, null-prototype object, conflict, code and project | prints well formed JSON |

## 6. What this does not fix

A pure conflict (an edit during one 3,735-project round) still repeats the round (bounded by 64); only conc-graph's settle-free snapshot publication removes it. Required follow-up after apply: run `show projects` once and read the first `[graph-settle]` row; if the cause is a transient reader error (lean leaf reader, storage custody), it needs its own project-level retry, which the row makes cheap to scope.

Tools: `C/loop-hunt-tools/`: `apply.py`, `ops_def.py`, `verify.sh`, `mutation-check.py`, `integration.ts`, `existing-law.ts`, `settle-exp.mjs`, `probe-projects.ts`, `order.mjs`, `missing.mjs`, `chk.mjs`, `young.mjs`, `leaves.mjs`, `leafscan.mjs`, `staged/`.
