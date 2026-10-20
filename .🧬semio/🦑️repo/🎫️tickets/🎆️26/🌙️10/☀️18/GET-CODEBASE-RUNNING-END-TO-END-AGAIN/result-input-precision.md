# result-input-precision: 243 whole-repository tasks get derived inputs; a law keeps the whole repository from coming back (staged; applied live 10-20 18:29 per the round 2 ledger)

READY: staged set complete 10-20 18:35 (`C/input-precision-tools/staged/` + `apply.py`; waits for "GO"; nothing written outside `C/`).

Executor `input-precision` (wave 1020d, resumed from the dead 1020c agent). `C` = this ticket. STAGE-ONLY: no `bun repo`, no Bazel, no cargo, no sub-agent, no repo-wide scan; every measurement ran in `/private/tmp/ip/` (rebuilt from `C/input-precision-tools/`) with one process at a time. Notes and rebuild commands: `C/input-precision-tools/NOTES.md`.

## Verdict

1. **Schema-first derivation instead of whole-repository globs.** A closed document (`🏗️build/📥️inputs/🎯️precision/🔣️.json`, schema `🧬️schema/🔣️.json`) names 26 ordered rules; each rule selects targets (project + target names or script command words) and says what the task reads: glob profiles, taxonomy file kinds, Cargo crate closures, dispatcher branches of the verify router, script literals plus import closure, owner routes, or a `tsconfig` program. The engine (`🟦️.ts`, 27 KB, no I/O of its own; the host is injected) turns that into the narrowest declared inputs and refuses anything it cannot bound.
2. **The law exists.** `⚖️laws/🔒️cache-correctness` gets the rule `whole-repository-input`: a cached target whose inputs select the whole repository (`{projectRoot}/**/*` at the root project, `{workspaceRoot}/**/*`, `**`) is a finding unless a precision rule narrowed it or records why it reads everything (`whole` by design, `assumed` = recorded debt). Real repository, 3,814 authored records: **239 findings without the rules, 0 with them** (the 3 `uncached-without-reason` findings are the known peer targets). Corpus 38 -> 47 cases, strict schema, language-agnostic.
3. **Declared inputs shrink where it counts** (static reconstruction of the 8,028-task graph, adapter's own expansion, 3,706 cached tasks): tasks declaring more than 100,000 files **243 -> 11**; more than 50,000 **254 -> 61**; 947 tasks changed, their declared input files **34.3 M -> 12.0 M (-65 %)**; all cached tasks 38.0 M -> 15.7 M (-59 %). The 11 that remain are recorded reasons, not defects of the derivation (below).
4. **No missed inputs, three independent ways:** (a) `tsc -p <tsconfig> --listFilesOnly` as the third-party oracle of the program derivation: `🌎️hub/📦️packages/🟦️typescript/tsconfig.json` 2,142 workspace files, **0 missing**; `✏️s/tsconfig.json` 12,876 files, **0 missing** (and no engine root that tsc does not list); the engine's closure uses the compiler's own module resolution under the tsconfig options. (b) The engine fixture (language-agnostic JSON) is answered identically by the TypeScript engine, a Python oracle (`🐍️oracle/🐍️.py`), `minimatch` (globs) and `Bun.TOML` (Cargo names); a tsc test over a generated fixture workspace runs inside the unit test. (c) Scanner and owner rules rest on the recorded read traces of 10-20 morning (`NOTES.md`: discovery scan 8.8 k reads, 99.9 % inside the `package-discovery` profile; pure oracle laws = discovery scan plus 620-700 code loads); evidence is stated per rule (`traced`, `static`, `bound`).
5. **Derivation runs on the real production analyses without refusing:** 250 targets over the static graph through the live import-closure, Cargo and TypeScript analyses, **0 failures** (`C/🗑️generated/input-precision/derivation-report-real-*.json`; owner delegates 90 targets 3.1 min, verify laws 71 + scanners 35 1.9 min, the rest 8.2 min).

## Defects found and fixed in the staged set (they would have been silent cache holes or load failures)

| Defect | Effect if applied | Fix |
|---|---|---|
| Replacing `default` dropped `sharedGlobals` (bun version, platform/arch, `SEMIO_BUILD_MODE`) and the project manifest | every narrowed task keyed without toolchain identity; `options.env` edits invisible | document field `retained` (`sharedGlobals`, `{projectRoot}/📋️project.json`) always kept; law models the same |
| A script literal resolving to `.` made the repo root a "crate" -> `{workspaceRoot}/./**/*` | the derived set was the whole repository again | literals resolving to the root are ignored; **final guard** refuses any derived pattern that selects the repository (also `./`-prefixed); document check normalizes `./` |
| Ticket drafts (`.🧬semio/**/Cargo.toml`) shadowed crate names in the crate index | 14 verify targets failed with "Native dependency has no Cargo manifest" | hidden trees never own a crate (fixture row with a shadowing `.ticket/draft/Cargo.toml`; the test fails without the fix) |
| Hook swallowed unbounded rules and left the whole-repository inherited inputs | the whole repository could return silently | fail closed: the graph refuses with `Input precision rule X cannot bound project:target`; `whole`/`assumed` rules keep inherited inputs and say so |
| Eager `typescript` load for every target | broke zero-touch boot without `node_modules` | loaded lazily on the first program rule |
| `assumed` allowed only by an env switch | hidden mode | removed; `assumed` is never applied (schema text) |

## What gets written on apply

`python3 C/input-precision-tools/apply.py` (dry run, default) prints 11 operations; `--apply` writes them all or none (temp file + rename, re-check of every file immediately before the first write, rollback of every file on any error, backup + manifest in `C/🗑️generated/input-precision/apply-backup-<hash>`); `--undo` restores if nothing moved since; `--strict` additionally refuses when an edited live file's hash moved since `mk-bases.py`. Anchors are matched against the CURRENT live text (each must occur exactly once; idempotent). Tested in a sandbox copy (`IP_LIBRARY`): dry run, apply, second run = 0 operations, undo, anchor drift = refused with nothing written, mid-write permission failure = everything rolled back.

| Operation | Target (under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`) |
|---|---|
| create x7 | `🏗️build/📥️inputs/🎯️precision/` `🟦️.ts`, `🧬️schema/🔣️.json`, `🔣️.json` (rules, regenerated from the live owner manifests at apply time), `🧪️tests/🟦️.ts`, `🧪️tests/🔣️.json` (+ `🧬️schema`), `🧪️tests/🐍️oracle/🐍️.py` |
| edit | `🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`: 4 anchored edits (imports, `readdirSync`, the hook block, one line before `normalized[name] = …`); the hook runs for cached targets that have inputs and records `metadata.semio.inputPrecision = {version, rule, evidence}` |
| edit x3 | `⚖️laws/🔒️cache-correctness/🟦️.ts` (rule, narrowing model, `repositoryCacheRecordsV1(…, precision?)`), `🧪️tests/🔒️rules/🧬️schema/🔣️.json` (enum), `🧪️tests/🔒️rules/🔣️.json` (9 cases + 1 marker on the workspace-root env case) |

Re-based 18:24 after the coordinator's integrate2 note: `apply.py --strict` dry run matches every anchor and file hash on the live tree; `round2-tools/touched.json` lists no other group on these 11 files (cache-law edits manifests only). Dry run against the live tree: all anchors match (project-assembly last committed 15:04; the law files carry the cache-fix and uncached-owners edits already).

## Rules by evidence (257 cached tasks decided in the authored manifests)

| Evidence | Tasks | Meaning |
|---|---:|---|
| `static` | 184 | derived from code/manifests (owner delegates 90, verify laws 71, typechecks 2, lint, test entrypoint, stdio native check, artifact ownership 2, …) |
| `bound` | 52 | kind-bounded superset (scanners 31, schema 12, repository-wide 5, generate, catalog) |
| `traced` | 3 | containment-checked against a recorded run (manifest scanners) |
| `whole` | 4 | reads everything by design, reason in the rule: `generate taxonomy census|duplicates` (semantic census of every component), `repo-lib` fixture placement x2 (walks all nine physical scopes) |
| `assumed` | 14 | debt, inherited inputs stay: `workspace-reentrant` x3 (`scale-fixture-check`, `stdio-long`, `stdio-exhaustive` re-enter `bun repo run`/`run-many`), `workspace-cpp` x2, `workspace-storybook`, `plugin-composition-test`, `hub-stdio-oracles` x7 (the stdio tree is 80-99 k files by itself) |

The 11 tasks still above 100,000 files: those 8 root tasks (3 re-entrant, 2 census, 2 cpp, storybook), the 2 `repo-lib` fixture-placement tasks and `s-plugin-composition:test` (117 k, assumed).

## Findings for other owners

- `generate-taxonomy-census|duplicates` are authored `cache: true` with `outputs: []` but write ticket files (`📊️semantic-census.json` …): they belong in the uncached-owners decision set (`side-effecting/mutates-workspace`). Same for the three re-entrant root commands (`aggregate/reenters-build-tool`): a cached result keyed on anything but the launched targets' inputs is wrong. Both are open in `uncached-owners` scope; the precision rules keep them conservative until then.
- The cache-correctness `globMatchesV1` mishandles braces (`a/{b,c}/d.ts`); the engine carries its own matcher (unit-tested against minimatch). Law owner should adopt `inputGlobRegExpV1`.
- The Bazel and Nx adapters treat `{projectRoot}/📋️project.json` and the named group `sharedGlobals` like any other input (checked through the adapter's own expansion).

## After "GO" (sequence in `C/input-precision-tools/go.sh`)

1. `apply.py` dry run (drift), `apply.py --apply`.
2. `SEMIO_INPUT_PRECISION_REPO=1 bun test` the precision tests (also asserts every owner route equals the manifests' `workspaceCommand` declarations and that `assumed` rules carry a reason) and the cache-correctness law test (repository records through the live policy and the live rules: expect 3 known peer findings, 0 whole-repository).
3. Live graph: `bun repo show projects`; then census on the live graph (`inputs-census.ts` with `IP_GRAPH`) and Nx hash cost with `measure-nx.cjs` (`expand-inputs.ts` builds `expanded.json`), one process at a time.
4. Watch the cold graph build time: deriving the `✏️s/tsconfig.json` program took 448 s under load 46 (`tsc` itself 145 s); the derivation runs inside the observed inference, so reads are recorded as dependencies and it is not repeated unless a read file changes. If the cold cost is unacceptable, flip that rule's `evidence` to `assumed` in `gen-rules.py` (one line) and re-apply.

## Not done / not proven

- No live apply, no live graph, no Nx/Bazel hash timing yet (waiting for GO); the census numbers are the static reconstruction (graph of 14:56, before integrate round 1), so cache-fix demotions that happened later are not subtracted.
- Containment of the scanner/owner rules is evidenced by the recorded traces and by static derivation, not by a fresh trace after the rules changed; a read-trace lane (run a task under a tracing preload, compare with its expanded inputs) was not built.
- `s-dev-typecheck` was derived on the real host once (17:09 run, same rule); after `retained` was added only the hub program was re-derived (rules text identical).
- `assumed` (14) and `whole` (4) tasks keep inherited inputs; they are the remaining whole/very large declarations and are recorded, not narrowed.

## Files

`C/input-precision-tools/`: `staged/` (`precision.ts`, `schema.json`, `rules.json`, `test.ts`, `fixture.json`, `fixture-schema.json`, `oracle.py`, `hook.mjs.txt`), `apply.py`, `hook-patch.py`, `law-patch.py`, `gen-rules.py`, `gen-fixture.py`, `layout.py`, `mk-bases.py`/`bases.json`, `prep-run.py`, `prep-law.py`, `law-check.ts`, `hook-harness.ts`, `apply-derivation.ts`, `merge-after.ts`, `inputs-census.ts`, `expand-inputs.ts`, `measure-nx.cjs`, `crosscheck-tsc.ts`, `real-host.ts`, `static-host.ts`, `graph.ts`, `go.sh`, `NOTES.md`. Generated (disposable): `C/🗑️generated/input-precision/` (graph, census, reports; 269 MB).

## Follow-up 10-20 19:25: program receipt (staged2, APPLIED live)

`C/input-precision-tools/staged2/` (`apply.py`, hash-checked against the first staged set, all or nothing; backup `🗑️generated/input-precision/apply2-backup-*`) is applied. The `✏️s` typecheck derivation no longer costs 448 s:
- **Cause found:** the time was the engine's exclude matching (17 globs x 2 regexes x 185 k walk entries, 280 s of 303 s with a no-op program host), not the compiler. `excludesV1` (literal, `prefix/**/name[/**]` by segment, regex fallback; equal to the glob semantics on 5 k generated paths) and a compiled-glob cache bring the engine to 26 s.
- **Receipt (schema-first, `definitions.ProgramReceipt`):** key = tsconfig + compiler version; per source file the content digest, the resolved imports and the facts (file/directory existence, manifest text digest, realpath) its resolution consulted, validated through the observed file system (so the enclosing inference still depends on exactly those facts) and reused while they hold; an edit re-derives only the files it touches; corrupt/foreign receipts are ignored; a cancelled run persists its partial receipt and resumes. Stored as the optional graph cache leaf `ip/<24 hex>.json` (brotli, ledger-admitted: storage schema `optionalPath` + graph fixture extended).
- **Progress and cancellation:** per-file `yieldInferenceV1` (200 ms throttled progress publication, abort honoured); EN/DE texts in the rules document (`messages`), language like the CLI help; printed only while something is derived.
- **Measured (real tree, live production analyses):** `✏️s`: cold 259 s -> 40 s, warm 20 s with 0 files derived (was 448 s every time its inference record was invalid); hub program cold 7-17 s, warm 0.5-1.1 s. `tsc --listFilesOnly` crosscheck: 0 missing on both programs (12,959 and 2,220 files).
- **Checks:** precision suite 12 pass (cold/warm/edit/new-import/shadow-resolution/remove/corrupt/tsconfig-change/cancel/resume scenario, tsc oracle, exclusion equivalence, EN/DE texts); cache-correctness law corpus passes and the repo run reports 0 whole-repository findings (2 unrelated `uncached-without-reason` peers: `framework-process:test-windows-conditional-retirement`, `repo-dashboard-rs:test-language`); optional-cache path test passes; `tsc --strict` clean on the engine. No `bun repo`, no graph build was run: the cold graph build with the new hook is the first live exercise.
