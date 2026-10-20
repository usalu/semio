# handoff-graph-closure: persisted owner records and an admission receipt for `repo run <target>`

From x-graph (`T/result-x-graph.md` has the evidence). For the peer fleet of ticket 26/10/18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN. `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL`, tools `T/x-graph-tools/`. Nothing is applied live (peer freeze: `🕸️graph/**` admission and the admission schema).

## What it fixes in one paragraph

`repo run <target>` already admits only the transitive owner closure of the selected tasks, but derives that closure from scratch in every process, twice on a cold run (`inventory()` before and after inference), and publishes the selected graph only at the end of a successful attempt. For `@teaching/architecture-course:test` that is 258 owner observations (162 owners, 367 CPU-s) per run, and the run currently never succeeds (typegen own-output refusal), so no run is ever warm. The staged set persists every owner observation as a content-keyed record (resumable, shared across runs of the same selection), remembers the closure for the second `inventory()` call, persists it as `selected/<scope>.admission.json` beside the graph, names the requester chain of a failing owner, and shows per-owner progress. Both Nx and Bazel go through the same `loadBuildGraphV1`, so the neutral graph is one object.

## Apply (independent of every other staged set; order does not matter)

```
python3 T/x-graph-tools/apply.py --check     # 21 operations pending, exit 1
python3 T/x-graph-tools/apply.py --live      # refuses with exit 2 and writes NOTHING if any anchor is not unique
python3 T/x-graph-tools/apply.py --check     # exit 0
sh T/x-graph-tools/check.sh                  # mirror proof, about 60 s: oracle, 14 tests, 13 mutations, tsc
```

Targets and anchors (all verified unique on the live tree at 17:15, files unchanged since 03:16 / 11:24):

| file | edits |
|---|---|
| `🏗️build/🕸️graph/🚪️admission/🟦️.ts` | 2 import edits, `requestedBy`/`origin` tracking in `closeBuildOwnerAdmissionV1` (4 one-line edits), annotated observation failures, append of the cache vocabulary |
| `🏗️build/🧬️schema/🚪️admission.json` | insert `$defs.admissionCache`, `observationRow`, `admissionReceipt` (this file is already in the graph module authority list, so receipts bind to it) |
| `🏗️build/🕸️graph/🟦️.ts` | 2 imports, inventory head (reuse / memo / lazy ports), closure port, memo, persist-with-graph, staging sweep regex |
| new | `🕸️graph/🚪️admission/🧪️tests/🗃️closure-cache/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🐍️oracle.py,🏃️step/🟦️.ts}` |

Applying invalidates every existing graph and authority receipt once (the module authority hash includes `G/🟦️.ts` and the admission files). Expected and harmless.

Interaction with other sets: x-ingress (`🚀️bootstrap`, generation receipt), graph-perf (`script-inputs`, `inputs`, `observation`, `seal`, `bootstrap`), bzl-lazy and x-release/x-paths (Bazel adapter) touch none of these files. graph-perf makes each record cheaper (shared command-input closures, async brotli); apply it too, the two multiply.

## Validate (after apply, when dry-runs are allowed again)

1. Existing suites through the normal runner (a scratch mirror lacks the process envelope, so `🛂️admission`, `🎯️source-demand`, `🔭️observed-facts`, `🧊️optional-cache`, `📌️pinned-publication`, `🔄️candidate` could only be compared control-vs-patched: identical failing sets). `🔄️candidate` is most likely stale already (its `Function` model cannot resolve the dynamic `import()` of the current attempt); the "graph attempt" tests of the new suite show how to model it.
2. `sh T/x-graph-tools/measure.sh` (two consecutive `bun repo --tool=bazel run @teaching/architecture-course:test --dry-run` under `graphtime.ts`, per-phase table, then the census). Acceptance: pass 2 has zero `build:selected-declaration` events and zero closure derivation; pass 1 at most one cycle per distinct `(owner,targets,inputs)`. Record the numbers in `T/result-x-graph.md` section 5.
3. Cancel a cold run after about 60 s (Ctrl-C or the invocation signal) and rerun: only the unfinished owners show `selected-declaration`.

## Things the peer should know

- **typegen own-output refusal (yours per coordination 16:39).** It is a cycle INSIDE the closure, not an over-approximation: a consumer's compile import of `🧰️framework/🔨️modules/🐚️shell/🟦️.ts` (which re-exports `./🤖️generated`) makes `@semio-tech/framework-os-shell-rs:typegen` a required producer task via `generatedDependencies`, and typegen's own input group `schemaModule` contains `{projectRoot}/📜️script.ts` whose import closure reaches the same module. `generated()` in `🔎️owner-discovery/🟨️.mjs:213` rejects it for every SELECTED target of the observed owner and only in the selected path (the all-graph path has no such rule, which is why `show projects` never failed). With this set the next failure prints `Build owner admission chain: @semio-tech/framework-os-shell-rs <- <consumer>:<target> <- selection`, and the 161 finished owners stay persisted, so the retry after your fix costs only the remaining owners.
- **Selections are cold once each.** Records live in the selection's isolated writer directory (`inference/selections/<scope>/np/<24 hex>.json`, same grammar as project records, no storage schema change). `course:test`, `course:build`, `course:typecheck` ... each pay one cold closure. Sharing the selection-independent `([],[])` observations (162 of 258) across selections needs a non-isolated writer; not staged (F1 in the result).
- **Optional cache quota.** 1 GiB max, 512 MiB low water, shared with the all-graph `np` store (about 855 MB today, retired oldest-first). The new records add about 30-60 MB per selection. If writes are silently refused (quota), pass 2 of `measure.sh` stays slow: look at `owner records consulted` and the optional ledger `used_bytes`.
- **Receipt trust model** is the one the graph authority receipt already uses: `observationsHold` over every consumed fact (stamp fast path, sha256 on a stamp mismatch) plus the loaded module authority plus the registry-bound scope key plus a fresh membership digest. A receipt is withheld when any consumed file is younger than 2 s at closure start (racy rule), and it is written only after graph and authority receipt were renamed, best effort.
- **Cancellation.** `check()` + `advanceScriptInvocation("build:selected-records")` per owner; abort and cancel errors are never annotated or wrapped. Everything else about `settleObservedComputationV1` (signal, rounds) is unchanged.
- **Windows.** `check.sh` and the suite call `python3` and `bun` by name; the oracle falls back to the frozen receipt verdicts when Python `jsonschema` is absent (Ajv still judges them).

## Rollback

`git`-free: the three live files are plain edits; revert by reversing the replacements listed in `apply.py edits()` (each `replace` carries both strings) or restore from the peer's snapshot. Deleting `selected/*.admission.json` and `inference/selections/*/np/*` (optional, recomputable) is always safe.
