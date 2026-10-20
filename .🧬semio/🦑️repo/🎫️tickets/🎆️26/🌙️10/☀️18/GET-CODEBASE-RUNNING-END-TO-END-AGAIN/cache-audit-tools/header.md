# Cache Correctness Audit 1020 (remote-cache readiness)

Auditor: `cache-audit` (read-only; writes only this report and scratch under the ticket). Snapshot taken 2026-10-20 during an active fleet, so counts can drift by one or two.

## Scope and method

- Inventory: `git ls-files '*project.json'` minus ticket copies under `.🧬semio/`, giving 1471 files (`cache-audit-tools/pj-list.txt`).
- Scripts (Bun/TS, in `cache-audit-tools/`): `scan.ts` (per-target classification, mirrors `🎯️target-policy/🟨️.mjs` and `⚡️caching/🔣️policy.json`), `shapes.ts` (token shapes), `named.ts` (named-input resolution and env-bearing named inputs), `rustbuild.ts` (Rust build outputs), `report.ts` and `top.ts` (ranking). Raw scan: `🗑️generated/cache-audit-scan.json`. All 165 heuristic rows: `cache-audit-tools/rows-all.md`.
- Static reads of the inference plugin (`🕸️graph/📥️inference/`), `nx.json`, and the Bazel adapter (`🔧️tools/🏗️bazel/📜️script.ts`). No `bun repo show projects`, no dependency sync, no Nx or Bazel run (freeze in force).
- Limit: 835 projects declare no explicit targets; their targets come from the inference plugin and are not covered by the JSON scan. Only the 3810 explicit targets were classified.

## Counts

| Measure | Value |
|---|---|
| Tracked project.json (non-ticket) | 1471; 636 with explicit targets, 835 inference-only |
| Explicit targets | 3810 |
| `cache: false` (explicit) | 423 |
| Policy-uncached or mutating (no explicit cache) | 3 |
| Policy cachedExact | 3 |
| Cached by family regex | 2818 |
| Cached by fallthrough (name matches no family; `cache: target.cache !== false`) | 563 |
| Raw `cache: true` or unset | 3387 |
| Cacheable explicit targets with no or empty `outputs` | 3083 (mostly test/check/lint, which is expected) |
| Cacheable explicit targets without explicit `inputs` | 1023 (augmented by inference for script-backed targets) |
| `cache: false` targets that still declare outputs | 13 |
| Fallthrough targets with emoji-prefixed names (no family match possible) | 106 of 233 fallthrough targets lacking explicit inputs |
| Heuristic finding rows (raw) | 165: high 146, medium 12, low 7 |

Non-finding counts from the raw scan (not reported): 2363 "explicit inputs omit toolchain env" and 300 "explicit inputs exclude project sources". Inference augmentation (`🧱️project-assembly` `genericTargetCommandInputs`, `🧭️script-inputs` closure and env inputs) adds these for script-backed targets, so the raw counts are noise.

## Classification

- **A. Confirmed in project.json (fix now):** root `prepare` (setup side effect cached); dev `build` with `inputs: []`; wgpu `browserBootSources` undeclared (5 targets); `options.env` values not hashed (7 targets); fixed `mutating` name rule miss.
- **B. Probable, confirm on the generated graph:** cacheable producers with `outputs: []` (89 non-check raw hits); outputs outside the owning project (35, most inside the same module, one level up); emoji-prefixed generator targets that fall through to cache-on (106).
- **C. Systemic policy:** fail-open classification in `targetPolicy` (any unmatched target is cached); name-regex classification defeated by emoji prefixes; `sharedGlobals` and `default` carry only `SEMIO_BUILD_MODE` and no workspace lock or toolchain files (augmented for script targets only).
- **D. Uncached but could be cached:** 6 candidates. `test-bestest-regenerate` must stay uncached (regenerates goldens).
- **E. Bazel, unverified:** `repo_task` declares only `<label>.snapshot` as its output. Real repo outputs on a remote-cache hit depend on the birth runner (`📜️script.ts` lines 478 and 610) materialising them. Not verified in this audit.

Calibration (reviewed, not findings):

1. Rust explicit `build` with `outputs: []` and `rustBuildIntent` (24 raw hits): the selected-graph path replaces the parent with an uncached noop aggregate (`🕸️graph/🦀️rust/🟦️.ts`, `selectedRustAggregateTargetV1`), and per-unit compile targets carry the store outputs. Not a finding, but this path is only exercised in the selected graph and was not run here.
2. Print `build` and `build-viz`: explicit targets are replaced by `printDocumentTargets` with `cache: false` (`🧩️component-targets`). Not a finding.
3. `describe-rs` `build`: outputs come from `hostPlatformOutputsV1` (`withHostPlatformOutputsV1`). Not a finding.
4. `leutwiler/realparts-of-powers-z-n` `build`: `repo:noop` orchestrator. Not a finding.
5. Cached command text (static): no network fetch, wall-clock, or random source found in cached targets. This covers only the command strings; the scripts they run were not read.
6. Bazel fingerprint (`📜️script.ts` around line 436): includes platform, arch, bun version, `SEMIO_BUILD_MODE`, declared env values, runtime-command stdout, and a random UUID for uncached actions. `bazelCacheDecisionV1` refuses conflicting cache declarations, and an `unbounded` report already lists cache-without-outputs targets.

## Verification still needed (after the freeze lifts, read-only)

1. Dump the generated config for the listed targets to confirm `inputs` and `outputs` after inference augmentation.
2. Confirm `browserBootSources` resolves (or not) in the wgpu graph.
3. Read `setup` in the `prepare` script to confirm its writes.
4. Run a Bazel remote-cache hit in a fresh clone with deleted outputs to confirm restore.

## Recommendations

- Fail closed in `targetPolicy`: unclassified targets default to `cache: false` unless a metadata flag says otherwise.
- Classify by metadata flag, not by target name regex, so emoji prefixes cannot bypass classification.
- Set `cache: false` on `prepare` and any setup or generate target that writes into the workspace without declared outputs.
- Give each generated tree one owner and keep outputs under `{projectRoot}`; ban `../../` outputs in lint.
- Forbid `inputs: []` on cacheable targets and fail on undeclared named inputs.
- Add `{ "env": ... }` inputs for every `options.env` key on cached targets, or drop the override.

## Top 50 findings (category-balanced; full list in `cache-audit-tools/rows-all.md`)

Columns: file, target, issue, suggested fix, severity (heuristic).

