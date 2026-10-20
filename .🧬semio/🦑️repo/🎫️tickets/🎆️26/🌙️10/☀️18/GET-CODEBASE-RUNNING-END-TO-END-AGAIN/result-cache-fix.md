# result-cache-fix: complete cache keys before remote caching (executor cache-fix, 2026-10-20)

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Phase: write pause in force, so **everything is staged** under `C/cache-fix-tools/` (`apply.sh`, dry run clean). Nothing outside `C/` was written. Verified in overlay COPIES (`/private/tmp/cfov`, `/private/tmp/bzlcf*`, all deleted).

## Verdict per audit finding

| Audit item | Verdict | Action |
|---|---|---|
| A1 root `prepare` cached, `outputs: []` | `setup prepare` only prints; the target is a pure ordering task. The real hazard was the name evading the `setup` rule | manifest: `cache:false` + `aggregate/orders-prerequisites` |
| A2 dev `build` `inputs: []` | `completion complete` only prints; ordering task over `build-s-react` | `cache:false` + `aggregate`, `inputs: []` removed |
| A3 `browserBootSources` undeclared | **false positive**: declared through `metadata.sourceInputs` (project.json:1578), resolved by `declaredSourceInputs`, covered by test `wgpu-browser-boot-cache-inputs`. Law rule `named-input-undeclared` now counts `metadata.sourceInputs` keys; 0 undeclared words repo-wide | none needed |
| A4 seven `options.env` not hashed | all seven values are constants of the target definition; every one of those targets hashes `default` = `{projectRoot}/**/*`, which includes the declaring `📋️project.json`; Bazel hashes the manifest. Adding `{env}` inputs would hash the *ambient* variable, not the override, i.e. be wrong | law rule `env-unhashed` encodes the real condition (`{env}` input, or the declaring manifest is hashed); `SEMIO_BUILD_MODE` is always hashed (`targetBuildModes`) |
| A5 `mutating` rule miss | closed by A1 and by emoji-stripping in every name rule (see below) | |
| B producers with `outputs: []` | real: 18 producers without declared outputs and without a generator contract (ui-styling(-py):generate, plugin-registry:generate, print generate-viz/font-metrics, print-viz-inference:build, 2 Go builds, leutwiler build-lean, schema-mirror, channel-version-generate, logo, bericht/dev build as aggregates ...). Contract-injected generators (`generatorContracts`) and Rust `rustBuildIntent` builds are fine | `cache:false` + decision in manifests; production gate (below) as defence in depth |
| B fixture generators (~106 `🏭️generator*`, 140 targets counting `-manifests` etc.) | regenerate golden fixtures, `forwardAllArgs`, third-party tools, `outputs: []`: **poison risk, cached today** | class `side-effecting/mutates-workspace` (head `generator`, emoji-insensitive) + manifests `cache:false` |
| B outside-project outputs (35) | all stay inside the workspace and one module; no two replayed targets overlap | law rules `output-escapes`, `output-overlap`; **overlap found 2 real clusters** (below) |
| C fail-open `targetPolicy` (563 fallthrough) | fixed (below) | |
| C `sharedGlobals` lack lock/toolchain | inference adds command closure + `{externalDependencies}` + `package.json`/`bun.lock` fallback to every cacheable non-native target (`genericTargetCommandInputs`) | none |
| D six "uncached but cacheable" | wasm x3 are `managed-output` (receipted store, inferred), demonstrator/play `build` are release orchestrators (must be uncached), `test-bestest-regenerate` regenerates goldens | none enabled; `test-bestest-regenerate` gets explicit `side-effecting` decision |
| E Bazel restore | **verified** (below), with 2 new findings | |

## Systemic fix: fail-closed, schema-first classification

- `⚡️caching/🔣️policy.json` new `targetClasses` (schema `⚡️caching/🧬️schema/🔣️.json`, required): ordered rows `decision {class, reason}` + `exact | heads | tails | tokens`. An exact name beats any head/tail/token; then first row wins. Names are matched after stripping the leading symbol/emoji prefix (`normalizedTargetName`), for ALL name rules (continuous, uncached, mutating, live, classes).
- `target-policy/🟨️.mjs`: `targetPolicy` resolution order: continuous, authored `cache:false` (declaration validated), policy uncached, mutating/live (unless `cachedExact`), then the target's own `metadata.semio.cache` declaration (validated, closed; wins over name rows), then `targetClasses`, then the `verify` command. **No classification means `cache:false` with decision `unclassified/no-cache-classification`** (new closed class in `🏗️build/🏃️execution/🧬️schema/💾️cache.json`; Bazel schema, report schema and `BazelUncachedReasonV1` extended).
- Production gate `cachedOutputsAdmittedV1` (wired in `project-assembly` right before `targetBuildModes`, i.e. after contract/host-platform/receipt output injection): a replayed `pure-function-of-inputs` producer with no declared output (and no `rustBuildIntent`/`rustUnitAggregate`/`hostPlatformOutputsV1`/`repo:noop`) becomes `side-effecting/mutates-workspace`.
- All 3809 explicit targets classified by data: 2921 check-class, 328 producers, 372 explicitly uncached, 50 continuous, 0 unclassified after manifest ops. 135 targets flip cached to uncached (all reviewed: fixture generators 106+, producers without outputs, prepare/activate chain = `driver-owned`, managed-store = `managed-output`, acceptance gates = `live`).
- Manifest operations (141 ops, 62 manifests, span edits, idempotent, semantic-equality proof per file): `cache:false` + authored decision on every target the policy refuses; `infinite:fonts` gets the explicit `deterministic` declaration (receipt output). Second repo-wide finding: `repo-cli-go`, `repo-client`, `repo-mcp` all declare the SAME output `🗃️bin/<mode>/repo` (one Go binary, three owners) and `repo-vscode` `build`/`compile-extension`/`build-vsix` nest their outputs; `repo-mcp:build` and `compile-extension` stay the single cached owners, the others become uncached.

## Permanent law (language-agnostic fixture)

`⚖️laws/🔒️cache-correctness/` (new): `🟦️.ts` (rules, repository record builder, `policyCacheCorrectnessBreaches`), `🧪️tests/🔒️rules/{🟦️.ts, 🔣️.json (34 cases, 16 glob pairs), 🧬️schema/🔣️.json}`. Rules: `unclassified-target`, `authored-cache-overridden`, `empty-inputs`, `producer-without-outputs`, `env-unhashed`, `named-input-undeclared`, `output-escapes`, `output-overlap`, `command-mutating`, `command-network`. Cross-checks: strict Ajv on corpus; glob matcher vs `minimatch` on every pair. The repo test resolves every manifest target through the production `targetPolicy` (+ `targetDefaults`, generator-contract outputs) and requires 0 findings. Before the manifest ops it reports 151 findings (122 overridden authored caches, 14 producers without outputs, 7 unclassified, 6 overlaps, 1 empty inputs, 1 env), after: 0.
Existing tests updated: inference cache corpus (+ `outputsGate`, unclassified, emoji rows), cache-contracts assertion, Nx-contract vectors (+6 rows; 78 policy checks agree with Nx `isCacheableTask`), `source-demand` sandbox name list.

## Bazel `repo_task` restore: verified (scratch fixture, real checkout files, Bazel 8.4.2)

`C/cache-fix-tools/bz/verify-restore.sh` (about 8 min, all phases passed in the final run; `fail=0`):
- P1 cold: 2 actions run; declared outputs (nested file, directory, exec bit) appear.
- P4 `bazel clean` + deleted real outputs, disk cache only: **2 disk cache hits, outputs byte-identical, mode kept, undeclared file not restored** (contract: undeclared output = lost on a hit; that is what the law forbids statically).
- P7 remote cache (own HTTP cache `bz/http-cache.ts`), two fresh output bases and workspaces, no disk cache: **2 remote cache hits, real outputs restored byte-identical**; keys independent of output base/workspace path (they do depend on the captured source closure of the adapter, so a peer edit between two runs legitimately misses).
- Uncached task re-executes each time (P5).
New findings:
1. **`--remote_download_minimal` breaks the restore** (P8): the snapshot is not local, `restoreBazelSnapshotV1` throws ENOENT, no real output restored. rcache's contract must keep `--remote_download_outputs=toplevel|all` and the schema must forbid `minimal`.
2. **A declared output that is never produced is accepted** (P6): `captureBazelSnapshotV1` swallows ENOENT, the action succeeds and is cached. Optional patch (not in the default apply): `bz/optional-strict-outputs.json`, verified in the scratch host (task `missing` now fails with "Declared output of a replayed Bazel task was not produced"; normal tasks unaffected); enabling it repo-wide needs a run of all cached tasks, hence staged separately. Opt-out key `metadata.semio.optionalOutputs`.
3. Only `plan.selected` snapshots are restored into the checkout: with `consume` selected and a cached dependency `generate`, `generate`'s real outputs are not restored (Nx restores both). Not a poisoning risk (a dependent that executes receives dependency snapshots in its birth), listed for tool parity.

## Verification commands and outcomes

- `bun C/cache-fix-tools/apply.sh --dry-run`: library 20 writes, 62 manifests, clean.
- Overlay (`ov/build-overlay.sh`, applies the staged set on a copy of tracked `🧰️framework`, library and manifests): `bun test ./<overlay>/…/🧪️tests/💾️cache/🟦️.ts -t "decision|policy|replayed producer|emoji|native|corpus"` 9 pass (the playground test needs the missing package `@semio-tech/plugin-registry-publication`, so it was filtered out); `…/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts` 5 pass (including the repository scan = 0); `…/source-demand/🟦️.ts -t "normalized source contracts bind"` 1 pass.
- `verify-corpus.ts`, `verify-vectors.ts` (Ajv, Nx `isCacheableTask`): ok.
- Not run: `⚡️caching/🧪️tests/⚡️cache-contracts` full lane (needs Nx install), `bun repo verify taxonomy` for the new `⚖️laws/🔒️cache-correctness` tree names, any Nx or Bazel run on the real graph, inference-only (Cargo) projects beyond the production gate.

## Apply (after PAUSE LIFTED)

1. `C/cache-fix-tools/prep.sh` (regenerates manifest list/ops and the staged set from the live base; rerun if peers added targets).
2. `C/cache-fix-tools/apply.sh --dry-run`, then `apply.sh`.
3. `bun test ./$L/⚖️laws/🔒️cache-correctness/🧪️tests/🔒️rules/🟦️.ts` and `./$L/🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts`; `bun repo verify taxonomy`.
4. Coordinator: after `show projects`, the Bazel uncached report should list `unclassified` (expected 0) and fewer `unbounded`.

## Open items
- Take over finding 1 (rcache: forbid `minimal`) and decide on the optional strict-outputs patch and finding 3 (bzl-coverage / adapter owners).
- Inference-only projects have no manifest targets; they are covered by the production gate and classification, not by the static law.
- Files: `C/cache-fix-tools/` (edits/all.ts is the single source of the library edits; `rows.json` the classification; `law/` the law; `bz/` the Bazel harness; `handler/` a copy of the static write analysis).
