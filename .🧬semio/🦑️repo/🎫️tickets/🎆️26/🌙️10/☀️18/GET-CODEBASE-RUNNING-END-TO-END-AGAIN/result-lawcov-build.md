# Result lawcov-build

Executor lawcov-build, wave 1021b. READY for integrate2: apply `lawcov-build-tools/staged/apply.py` (dry run first, then `--apply`).

Scope: units without test, corpus or oracle under `📚️library/🏗️build/` and `📚️library/⚡️caching/`, excluding what live agents edit (🕸️graph generations/inference, 📥️inputs/💾️storage, closures, cache-gc/leases of the process module). Method and risk list: `audit-lawcov.md`. Tools: `lawcov-build-tools/` (below).

## Outcome

- 31 new test sets, each with its own `🧪️tests/` carrying (1) a language-agnostic JSON corpus closed by a JSON schema in `🧬️schema/🔣️.json`, (2) a TS test running the real code over the corpus, (3) an independent oracle: a Python `🐍️oracle/🐍️.py` (standard library only) in all 31, and in addition ajv (every set closes its corpus with it and most compare validator verdicts), the `diff` npm library (patch), Python `zipfile`/`sqlite3`/`hashlib`/`posixpath`/`ntpath` as cross-engine references.
- 26 sets are live (new files only, under the non-excluded units); 5 are staged for the excluded units (cache-gc runner, cache-gc command, leases command, graph retry, graph generation-files).
- Every set was run with `nice -n 10 bun test`, one process at a time, load below 45; results in the verification table. No `repo`, Bazel, cargo, git-modifying command or sub-agent was used.
- Re-measured with the untracked files included (`lawcov-build-tools/analyze-untracked.py`): of the 35 not-OK units in scope at the start, only the 5 staged units (applied by integrate2) plus 📋️pristine (own function needs issued store leases) and 📥️acquisition (network fetch) remain below class 4; the whole library moved from 625 to 731 class-4 units (the rest of that delta is other executors).

## Units covered (live)

| Unit | Class before | Tests | Corpus | Oracle |
|---|---|---:|---|---|
| ⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun | 1-NOTEST | 4 | ZIP archives with byte mutations, 8 platform rows | Python zipfile, hashlib |
| …/🩹️patch | 1-NOTEST | 7 | 25 admission, 6 additions, 6 receipt, 10 apply cases + unit's own | `diff` library + Python |
| ⚡️caching/📦️artifacts/🟦️typescript/🔗️binding | 1-NOTEST | 4 | 20 plan cases, receipts, capacity sweep | Python posixpath/hashlib |
| 🏗️build/📥️inputs/🎯️selected/🦀️rust | 1-NOTEST | 4 | 31 + the unit's own | ajv agreement + Python |
| …/🔏️authorizers/📦️snapshot | 1-NOTEST | 6 | 23 snapshot faults, real workspace capture | Python builder |
| ⚡️caching/📦️bundle/🟦️bun/📤️stream | 1-NOTEST | 5 | frames, wire cuttings, 19 hostile streams | Python encoder/decoder |
| ⚡️caching/🧹️pruning/⏱️recency | 1-NOTEST | 5 | 19 summaries, 12 funding, 6 reports | Python big integers |
| …/📦️sdk/📥️source/📐️projection | 1-NOTEST | 4 | 4 pins, 15 projections + own | Python JSON writer |
| ⚡️caching/🦀️cargo/…/📍️workspace | 1-NOTEST | 3 | 20 + own | Python posixpath/ntpath |
| …/🧊️pristine/📋️plan | 1-NOTEST | 4 | 25 + own + 64-cell truth table | Python decision table |
| 🏗️build/🩺️preflight | 2-NOCORPUS | 4 | 3240-cell sweep, invalid facts/requirements | Python table + ajv |
| 🏗️build/🔧️tools/🏗️bazel/🚦️test-classes/🧾️witness | 2-NOCORPUS | 4 | 12 file sets | real witness program under Bun, Python hashlib |
| 🏗️build/🔧️tools/🏗️bazel/👁️evidence | 2-NOCORPUS | 3 | directories, closures | Python |
| 🏗️build/🚀️bootstrap/⌨️cli | 2-NOCORPUS | 2 | 30 argument vectors | Python |
| 🏗️build/📊️reporting | 2-NOCORPUS | 2 | names + length sweep | ajv + Python re |
| ⚡️caching/🚀️bootstrap/🛠️tools/🌡️host-capacity | 2-NOCORPUS | 2 | 5 plans | Python |
| …/📦️sdk/🪪️identity | 2-NOCORPUS | 5 | 9 + boundaries | Python hashlib |
| ⚡️caching/🕸️graph/🏁️startup/🚦️service | 2-NOCORPUS | 3 | 19 wires | Python classifier |
| ⚡️caching/🧹️pruning (+📋️orchestration, 🌐️workspace) | 3-NOORACLE | 5 + 4 | 13 + 150 seeded scenarios, byte counts; 3 layouts | Python plan, exact rounding, derivation |
| …/🛠️tools/⚙️operations | 3-NOORACLE | 6 | 738 argument vectors | Python + real wrapper run |
| 🏗️build/🌡️host-capacity (+⌨️command) | 3-NOORACLE | 6 | 1133 asks, 44 grants, 172 commands | Python |
| 🏗️build/🔧️tools/🏗️bazel/🌡️host-capacity | 3-NOORACLE | 5 | 1920 concurrency cells | Python closed form + Starlark run as Python |
| …/🛠️tools/👷️workers | 3-NOORACLE | 3 | 23 plans, 14 executors | Python |
| ⚡️caching/💾️storage/🔒️enforcement/🧵️session/🏊️pool | 3-NOORACLE | 5 | 16 scripted scenarios | Python model |
| ⚡️caching/🚦️ci/🧭️baseline/🏃️resolve | 3-NOORACLE | 4 | 43 scenarios | Python model |

## Staged (excluded units, applied by apply.py)

| Unit | Class before | Tests | Oracle |
|---|---|---:|---|
| 🏗️build/🧹️cache-gc/🏃️runner | 2-NOCORPUS | 12 | Python walker, hold probes, planner; real SQLite lock held by Python sqlite3 |
| 🏗️build/🧹️cache-gc/⌨️command | 3-NOORACLE | 4 | Python grammar over 172 vectors |
| 🏗️build/🔒️leases/⌨️command | 2-NOCORPUS | 3 | Python summary of the JSON status; ajv |
| 🏗️build/🕸️graph/🔁️retry | 1-NOTEST | 4 | Python projection |
| 🏗️build/🕸️graph/🗂️generation-files | 2-NOCORPUS | 4 | Python window model |

Not covered, with reason: 📋️pristine/🟦️.ts (`requireNxPristineSdkV1` needs issued store, lease and snapshot capabilities; its decision function is covered through 📋️plan), 📥️acquisition (fetches the registry; lock-row refusals sit behind the issued-control check), the compile path of 🔏️snapshot (needs an issued control; target and role refusals before it are tested), 🏪️budget scripts.

## Findings (real defects, file:line)

1. `⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/🪪️identity/🟦️.ts:4-5`: the install key hashes `name NUL bytes NUL` with no length framing, so a content that contains the separators collides with two files (`[a:"x", b:"y"]` and `[a:"x\0b\0y"]` give the same key). Test: `test.failing` in 🪪️identity/🧪️tests/🧮️keys.
2. `⚡️caching/💾️storage/🔒️enforcement/🧵️session/🏊️pool/🟦️.ts:12,24`: two concurrent requests that fail on one shared session each call `drop(entry)`, so the session is closed twice (`closes` 2 for 1 session). Test: `test.failing` in 🏊️pool/🧪️tests/🧮️lifecycle.
3. `⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts:23-26`: `readNxAuthorizerSnapshotV1` refuses any file above about 3.56 MB with "Closed captured authorizer bytes required" although `maximumFileBytes` is 4 MiB, because the repository validator's base64 pattern test fails beyond about 4.75 million characters; the `bytes.length>maximumFileBytes` branch is therefore unreachable. A manifest whose entry is not captured reports "snapshot capacity exceeded" (line 26), which names the wrong cause. Test: `test.failing` in 📦️snapshot/🧪️tests.
4. `🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts:155-161` (`undeclaredEntriesV1`): a plain file standing at a container path (for example `tools` in the repository cache root) is classified `container`, queued for descent and silently skipped, so it is never reported, measured or collected. Documented by the corpus case "a plain file standing where a container is expected is skipped instead of reported".
5. `🏗️build/🧹️cache-gc/⌨️command/🟦️.ts:21`: `--area=,` (or `--area=,,`) parses to an empty set, which selects no area at all instead of being refused like `--area` without a value.
6. `⚡️caching/🚀️bootstrap/🛠️tools/🩹️patch/🟦️.ts:24-26`: a truncated hunk that ends in a newline is reported as "unprefixed hunk line"; the counts message only fires for patches without a final newline. Both refuse; the message is imprecise. Pinned by the corpus.

Reading notes (not defects): `formatBytes` relies on JS `toFixed` round-half-up; ties such as 1152 B give `1.13 KiB`, which other languages must reproduce (the corpus pins it).

## Tools (all under `lawcov-build-tools/`)

- `lc.py`: `overlay|live|stage|check|adopt <slug> <unit>`: installs `work/<slug>/{test.ts,corpus.json,schema.json,oracle.py}` into a unit's own `🧪️tests/` (optionally a subdirectory via `names.json`), resolving the import macros `@LIB/ @UNIT/ @FW/ @CORPUS @SCHEMA @ORACLE` to exact relative paths with the real emoji variation selectors; `live` refuses to overwrite anything it did not write (`installed.json` records hashes).
- `run.sh <slug> <unit>`: runs a set from a symlink overlay under `/private/tmp/lawcov-build/` (used for staged sets); `live.sh <slug> <unit>`: installs and runs in place; `verify-all.sh`: runs everything one process at a time.
- `work/<slug>/`: authoring sets (`gen.py` writes the corpus from readable cases; `schema.json`, `oracle.py`, `test.ts`).
- `analyze-untracked.py`: the audit analyzer including untracked files.
- `staged/`: `manifest.json`, one directory per staged set, `registry.json` (every new test), `build-registry.py`, `apply.py`.

## Staged patch for integrate2

`python3 lawcov-build-tools/staged/apply.py` (dry run) then `--apply`:
1. copies the five staged sets into their `🧪️tests/` (refuses a destination that exists and differs);
2. registers every new 🏗️build test in the `workspace-build:contract` list of `🏗️build/📜️script.ts` (line 75, anchored on its last entry);
3. adds the lane `test law-coverage` to `⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts` (anchored before `testCacheContracts`) and the target `test-law-coverage` to `⚡️caching/📋️project.json` (anchored before `test-cache-command-source`).
Anchors are unique; the script refuses on drift and is idempotent. The 🏗️build lane runs in the existing 300 s budget; no taxonomy change is needed (new directories use existing emoji names).

## Verification

Final run (`lawcov-build-tools/verify-all.sh`, `nice -n 10 bun test`, one process at a time, 1-minute load below 45; the staged sets ran in their symlink overlay, which resolves the same relative imports as the final location):

- 26 live sets, 109 tests, 0 failures; 3 of the 109 are `test.failing` known-defect tests (identity, pool, snapshot) that pass while the defect exists and turn red when it is fixed, the signal to flip them to plain tests.
- 5 staged sets, 27 tests, 0 failures after the last edit.
- One transient failure was seen once in the leases command set under load 45 (not reproducible in 8 reruns): probable cause: the admission check asserted an upper bound for the host-derived floor, which depends on the live memory sample; only the authored minimum is asserted now (staged set re-run green).
- Earlier, the runner set exceeded bun's 5 s default timeout under load 44 because it spawned the oracle per case; it now batches the oracle calls and every set calls `setDefaultTimeout(60000)`.
- Docstrings: every test and oracle starts with its own emoji; no comments inside definitions. Longest new path 151 UTF-16 units (limit 256).
