# 🧮️ Cache Coverage Census For Bazel (Read-Only Audit, 2026-10-19)

Auditor: `aud-cachecov`. Read-only. No `bun repo` run, no nx graph run, no build. Only writes: this report, `aud-cachecov-tools/census.ts`, `🗑️generated/aud-cachecov.json`.

Sources: `git ls-files` (tracked manifests), `nx.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` (the workspace cache policy and `targetDefaults`), `🏗️build/🕸️graph/📥️inference/` (plugin source), `C/audit-bazel-targets.md` (Bazel adapter).

Reproduce: `bun aud-cachecov-tools/census.ts` (from the ticket folder). Output: `🗑️generated/aud-cachecov.json` (per-target rows with `classProposal` and `flags`, plus totals).

## 1. Method And Important Caveat

- Explicit targets: read from every tracked nx-included manifest, then classified with a replica of the workspace's own policy, in the same order as `🎯️target-policy/🟨️.mjs` `targetPolicy` and with `POLICY.targetDefaults[name]` merged first, as `🧱️project-assembly/🟨️.mjs` l.138 does.
- Synthesized targets: the nx inference plugin is not run here (that needs the graph, which the brief forbids). Section 5 is a static reconstruction of the playground grammar from the Cargo manifests. It is an upper bound, not an enumeration.
- The `cache` policy replica is the only logic reimplemented. The `targetDefaults` lookup is the one in project-assembly.

## 2. The Nx Inference Plugin

`nx.json` `plugins`: one entry, `./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (name `@repo/emoji-project-json`), `include: ["📋️project.json"]`, `options: {}`. Its `createNodesV2` glob is `**/{📋️project.json,Cargo.toml,bun.lock,*.patch}`, so Cargo, bun.lock and patch files also synthesize projects and targets.

| Inference component (in `🏗️build/🕸️graph/📥️inference/`) | Synthesizes |
|---|---|
| `🎮️playground-targets` | Per playground row (`[[package.metadata.semio.playground]]` with `variant`): `prepare|activate|serve|dev-<variant>-{react,wgpu}-{prototype,release}`, `prepare-<variant>-{react,wgpu}` (merged alias), `prepare|run|smoke-<variant>-native-*`, `build-<variant>-react(-release)`, `build-<variant>-site`, `mcp-<variant>-{stdio,http}-*` (only with `mcpHost`), `runtime-input-admission-<variant>` (refusal path only). Also `build` per crate and `build-all-playground-cdn-sites` (any `distDir`). `session-<variant>` is added to `@semio-tech/plugin-registry`. |
| `🎯️target-policy` | Not targets. Sets `cache`/`continuous` per name from `⚡️caching/🔣️policy.json` (section 6). |
| `🧱️project-assembly` | Applies `targetDefaults` by name, then the policy. For Cargo-owning projects it adds native targets whose names match `^(build|wasm|native|test|lint|check|verify)`. Not enumerated here (needs graph). |
| `🦀️native-manifests`, `🧶️bun-lock`, `🏭️generator-outputs`, `🧩️component-targets`, `📦️project-nodes` | Cargo, bun-lock, generator and component targets. Not enumerated here. |
| `🕸️` `framework-os-dev` wiring (`📥️inference/🟨️.mjs` l.1645, per the Bazel audit) | Attaches the playground grammar to `@semio-tech/framework-os-dev`. That project has 53 tracked targets and 0 tracked `*prototype*` keys (confirmed). |

## 3. Census Scope

| Status | Files |
|---|---|
| `nx-project` (basename `📋️project.json`, outside fixtures/examples/pkg/tickets) | 1447 |
| `ticket-excluded` (`.🧬semio/` tickets, `.nxignore` `/.🧬semio/**`) | 142 |
| `nxignore-fixture-or-example` | 4 |
| `not-plugin-include-basename` (plain or `🎯️project.json`) | 1 |
| Total tracked candidates (`*project.json`) | 1594 |

Parse errors: 0. Nx projects with at least one explicit target: 637 of 1447 (810 are fully plugin-synthesized or empty). Explicit targets: 3792.

## 4. Per-Target Facts (Explicit Targets, 3792)

| Field | Values |
|---|---|
| executor | `repo:script` 3791 (explicit or implicit default), `repo:noop` 1 |
| command form | `options.command` 3791, none 1 |
| inputs (raw) | absent 2705, named-only 251, explicit-only 142, explicit+named 652, object-only 2, explicit+object 21, named+object 12, explicit+named+object 6, empty 1 |
| outputs (raw) | absent 2313, declared 302, empty `[]` 1177 |
| cache (raw) | `true` 843, `false` 1718, absent 1231 |
| continuous (raw) | 39 (none has `cache:true`) |
| dependsOn | 204 targets with at least one entry; 49 use object entries |
| configurations | 62 targets carry `configurations` (for example `native-build`, `build-storybook`, `test-contract`, `test-native`) |

## 5. Class Totals (Proposed)

Rule order (first match wins): continuous (raw `continuous` or policy continuous name) → live (`e2e|live` in name) → side-effecting (policy `uncached`/`uncachedExact` except diagnostic names, or `clean|gc|prune|setup|fuzz`) → diagnostic (`audit|bench|cache-report|cache-verify|ci-baseline|disk-report|doctor|perf-baseline|test-inventory|test-doctor|parity`) → deterministic-cacheable (effective cache true) → aggregate (noop or no command) → deterministic-but-cache-off (name in cacheable families, cache forced false) → unclassified-cache-off (decision needed).

| Class | Targets | Share |
|---|---|---|
| deterministic-cacheable | 2056 | 54.2% |
| deterministic-but-cache-off | 1177 | 31.0% |
| continuous (dev/serve/watch) | 56 | 1.5% |
| side-effecting (publish/deploy/install/clean/release/deps/setup) | 65 | 1.7% |
| test-against-live (e2e/live) | 18 | 0.5% |
| diagnostic-uncached | 12 | 0.3% |
| aggregate | 0 | 0% |
| unclassified-cache-off | 408 | 10.8% |
| Total | 3792 | 100% |

Top names in deterministic-but-cache-off (all explicit `cache: false`): `test` 56, `test-native` 37, `build` 34, `test-snapshot-sqlite*` 87 across three variants, `check` 20, `verify-*` 40, `test-ownership` 14, `test-contract` 10.

Top names in unclassified-cache-off: `canonical-architecture` 34 (policy itself declares `cache:false`), `preview-generated` 14, `graph-wire-check` 9, `mutation-inventory(-check)` 15, `source-check` 6, `composition-prepare` 3, `native-schema-check` 2, `rebuild-all` 2, `guest-framework-check` 2, `docker-image-build|check` 4, `cargo-prepare` 2.

## 6. Per Top-Level Owner

Projects and targets by owner, with class counts.

| Owner | nx projects | Targets | Det. | Det. cache-off | Det. no inputs (raw) | Det. no outputs (raw) | Cont. | Side | Live | Diag | Unclass. | cache true | cache false | cache absent |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 🧰️framework | 998 | 1672 | 1495 | 793 | 806 | 787 | 16 | 19 | 4 | 11 | 127 | 237 | 963 | 472 |
| ✏️s | 326 | 1425 | 1310 | 307 | 390 | 759 | 0 | 2 | 3 | 0 | 110 | 385 | 416 | 624 |
| 🌎️hub | 115 | 301 | 231 | 16 | 168 | 90 | 11 | 3 | 5 | 0 | 51 | 114 | 84 | 103 |
| root | 1 | 318 | 162 | 51 | 144 | 65 | 15 | 37 | 0 | 1 | 103 | 84 | 205 | 29 |
| ♻️mit-bestand | 3 | 30 | 17 | 3 | 1 | 1 | 8 | 0 | 2 | 0 | 3 | 13 | 15 | 2 |
| 🎓️teaching | 2 | 27 | 7 | 3 | 0 | 0 | 3 | 3 | 1 | 0 | 13 | 4 | 23 | 0 |
| 🏢️semio-tech | 1 | 13 | 6 | 4 | 2 | 0 | 3 | 0 | 3 | 0 | 1 | 2 | 11 | 0 |
| 👴️leutwiler | 1 | 6 | 5 | 0 | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 4 | 1 | 1 |
| Total | 1447 | 3792 | 3233 | 1177 | 1511 | 1703 | 56 | 65 | 18 | 12 | 408 | 843 | 1718 | 1231 |

Column "Det. no inputs (raw)" is after merging `targetDefaults`. Of the 1511, 1110 are covered by a name family in the policy at runtime, 401 are not.

## 7. Output And Input Gaps (Deterministic Targets)

| Gap | Count | Note |
|---|---|---|
| Deterministic total | 3233 | 2056 cacheable + 1177 cache-off |
| Cache forced off by explicit `cache: false` on a cacheable name | 1177 | All 1177 are explicit in project.json. Biggest fix. |
| Cache absent (policy decides) | 1219 | Make explicit in schema. |
| No raw inputs (after targetDefaults) | 1511 | 1110 covered by family default at runtime, 401 not. |
| No raw outputs (absent) | 1703 | 1596 are output-less families (`test|check|lint|verify|typecheck|format-check|*-check|*-contract|stdio`), so `outputs: []` is valid for them. 107 are not (build 41, generate 12, wasm 9, others). |
| `outputs: []` on a non-test name | 448 | Suspect: must declare outputs or justify. |
| Deterministic targets with declared outputs (for context) | 290 of 3233 | Rest are absent or empty. |

## 8. Targets Hit By Workspace Target Defaults

`nx.json` has no `targetDefaults` (asserted by `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` l.71). The workspace set lives in `🔣️policy.json` `targetDefaults` (38 keys) and is applied by name in project-assembly. Keys that hit tracked targets:

| Key | Targets | Key | Targets |
|---|---|---|---|
| test | 516 | build | 252 |
| check | 232 | canonical-architecture | 34 |
| package | 26 | generate | 13 |
| dev | 13 | typecheck | 12 |
| lint | 12 | test-contract | 12 |
| wasm | 11 | publish | 5 |
| deps | 5 | serve, watch | 2 each |
| setup, verify, bench, native-build | 2 each | others | 1 each |

## 9. Residual Names Not Covered By A Name Default

Name coverage today: 2801 of 3233 deterministic targets (86.7%), via `targetDefaults` key or the `cacheableFamily` regex. Stripping a leading emoji prefix before matching raises it to 2903 (89.8%); 330 deterministic targets remain.

| Residual name | Targets |
|---|---|
| preview-generated | 17 |
| graph-generate | 9 |
| publication-authority-audit, fonts, typegen, run (2 each), activate/prepare-prototype (2 each), child-identity-check, child-edit-check, add-widget-retained-check, canonical-pair-check, native-catalog-selection-check, native-codec-check, native-codec-oracle, scale-fixture-check, tiles-check(-native), tiles-prefetch, package-graph, metadata-(source|native|oracle), io-descriptor-parity(-oracle), akteursnetz-validate | 1-3 each |

Emoji-prefixed names bypass name defaults: 103 deterministic targets (for example `🏭️generator`, `🏭️generator-manifests`, `⚙️generator`) plus 5 unclassified. They are invisible to `^generator`.

## 10. Playground Grammar (Synthesized, Static Estimate)

Playground rows: 156 across 44 crate manifests (71 plugin/extension component Cargo manifests). Rows with `mcpHost`: 1. Rows with `distDir`: 1. The plugin's catalog-admission filter cannot be replicated statically, so these counts are an upper bound.

Per variant (admitted path): 28 targets. Plus 4 `mcp-*` only for variants with `mcpHost`. Plus 1 `build` per crate (44), plus `build-all-playground-cdn-sites` (1). Estimated total: 156 x 28 + 1 x 4 + 44 + 1 = 4417 (upper bound, attached to `@semio-tech/framework-os-dev` and `plugin-registry`).

| Family (per variant V) | Count | Cache (as authored) | Proposed class |
|---|---|---|---|
| `prepare-V-{react,wgpu,native}-{prototype,release}` | 6 | false (orchestrator) | deterministic-but-cache-off |
| `prepare-V-{react,wgpu}` (merged alias) | 2 | false | deterministic-but-cache-off |
| `activate-V-react-{p,r}` | 2 | true | deterministic-cacheable |
| `activate-V-wgpu-{p,r}` | 2 | false (comment: publishes live extensions) | side-effecting (deliberate) |
| `serve|dev-V-{react,wgpu}-{p,r}` | 8 | false, continuous | continuous |
| `run-V-native-{p,r}` | 2 | false, non-continuous | test-against-live (live run) |
| `smoke-V-native-{p,r}` | 2 | false | test-against-live |
| `build-V-react-release`, `build-V-react` | 2 | false | deterministic-but-cache-off |
| `build-V-site` | 1 | false, repo:noop | aggregate |
| `session-V` (plugin-registry) | 1 | true | deterministic-cacheable |
| `mcp-V-{stdio,http}-{p,r}` (mcpHost only) | 4 | false, continuous | continuous |
| `build` (per crate) | 44 total | false, repo:noop | aggregate |
| `build-all-playground-cdn-sites` | 1 total | false | side-effecting (CDN publish) |

Bazel implication (from `C/audit-bazel-targets.md` section 3): the adapter maps `cache:false` to a `no-cache` requirement. Every cache-off deterministic target therefore re-executes on each Bazel build. The generator owns 11 cache-off families per variant (the site aggregate included) (about 1716 targets at N=156), so one change in `🎮️playground-targets/🟨️.mjs` fixes most of them.

## 11. Dependencies (`dependsOn`)

395 references: 295 resolve to tracked targets, 94 match the playground grammar (synthesized, expected), 3 are unresolved (`semio-hub-gis-component:component`, a missing target), 3 use `^` (dependency-scoped).

## 12. Mechanical, Schema-First Proposal

Defaults by name (in `🔣️policy.json` `targetDefaults` / `cacheableFamily`, one place):

1. Emoji-tolerant name matching: strip a leading non-alphanumeric prefix before family and default lookup. Brings 103 deterministic targets under defaults. Coverage 86.7% to 89.8%.
2. Forbid explicit `cache: false` on cacheable families unless the target carries a `cacheExemption` reason. Expect about 1177 targets to become deterministic-cacheable. Live names stay off via the existing `e2e|live` rule.
3. Outputs: keep `outputs: []` valid only for output-less families (`test|check|lint|verify|typecheck|format-check|*-check|*-contract|stdio`). Require declared outputs for `build|wasm|native-build|package|extension-package|generate|cpp-build`. Residual 107 (plus 448 suspect `[]`) need per-target declarations.
4. Inputs: name defaults already cover 1110 of the 1511 raw-missing. Add families for `*-check`, `*-inventory`, `*-oracle`, `*-generate`, `*-validate`, then declare the remaining 330 per target.
5. Unclassified 408: decide per target. Most `*-check|*-inventory|*-oracle|*-validate` are deterministic and should become cacheable. `preview-generated` and `rebuild-all` need review.
6. Keep cache off, by design, for continuous (56), side-effecting (65), live (18), diagnostic (12). The Bazel adapter reads `cache` directly, so this list is the Bazel exemption list.
7. Playground grammar: set `cache: true` on `prepare-V-*`, `build-V-react(-release)`, `build-V-site` where the outputs are deterministic. Keep `activate-V-wgpu` off (live publish). Changing the generator changes about 1716 targets at once. Requires a re-verified closure (outside this read-only audit).
8. `nx.json` `targetDefaults`: stays empty (asserted by test). All defaults live in the policy file, so the schema stays single-sourced.

## 13. Open Items And Limits

- Not verified at runtime: nx graph loader, the playground catalog-admission filter (affects the 4417 upper bound), native-manifest targets for Cargo projects (not enumerated).
- Not run: `bun repo`, nx graph, cargo builds, activations (per brief).
- Per-target rows, flags and classes: `🗑️generated/aud-cachecov.json` (`targets[]`, `projects[]`, `synthesized`).
