# October 19 Source Discovery Continuation

## Bounded Review and Concrete Defect

The current launcher, source picker, form acceptance, registry adoption, terminal input projection and session-start path were read before editing. No course README, launch catalog, course fixture, AGENTS, Git state, native store or build receipt was changed.

The concrete integration defect was in `LauncherState::set_registry`: it rebuilt tree nodes and replaced the registry Arc while retaining an open form unchanged. That form still carried the previous entry index, parameter declarations/defaults, mutation-confirmation flag and derived source owner/path/directory. Reordering commands therefore allowed a form to render another entry's title; removing or changing parameters retained stale controls; changing an entry from nonmutating to mutating left the old flag in the configure/activate path. The old source subscription also remained registered until some later cleanup or old-registry drop. Session start already resolves the stable command ID against the current registry, so this finding does not claim that an old numeric index executed a different command.

The terminal's existing registry refresh calls this method for every launcher. A locale change calls the same method with the same immutable registry Arc. Those two cases need different lifecycle behavior.

## Implemented Behavior

A different registry releases this launcher's subscription from the old registry, finds its command by stable ID and rebuilds its form from current parameters. Fields are associated by stable ID and kind. Compatible explicitly edited values survive, including an empty working directory; removed choice values and automatically derived source values are cleared. Newly offered fields receive current preference/default behavior. Extra arguments/environment and the logical focused field or extra-input slot survive. Current entry position, parameter metadata and mutation flag replace the old values. A form returns to configuration so the current declaration receives a fresh confirmation; a removed command closes the form and returns to browsing.

Adopting the same registry Arc for a locale change retains the form, confirmation state and subscription. The tree is still rebuilt for the selected language. No persistent cache format or compatibility adapter was introduced.

The bounded action/lifecycle review also followed launcher outcomes into `carry_out`, `Dashboard::start`, current `Registry::resolve` and the typed repository-action selection/projection. Those paths already resolve current stable IDs and retain their existing admission/pending-process gates. No further concrete defect was established there, so they were not changed.

## Schema-First Validation and Executed Evidence

Four language-neutral registry-change cases were added to the source-discovery schema/fixture before the production fix: reordered commands and fields with derived values; compatible manual edits and a removed choice; command removal; and same-registry locale adoption. The fixture stages distinguish a nonmutating configure form from a mutating confirmation form. Ajv validates the closed fixture contract. An independent lodash `keyBy` model derives the expected stable-ID field projection and compares it to the handwritten expected results.

The native regression compiles and executes the exact production `set_registry` and `refreshed_form` bodies with small typed model records. Before the fix, the harness failed because the old form state remained instead of adopting the refreshed declaration. After the fix it checks current entry index, mutation flag, focus, field declarations/values/provenance, extra input preservation, command removal, old-session release and same-Arc preservation. The harness's registry/preference/tree shell is a test model; it does not compile or execute the complete dashboard, its renderer or daemon.

Executed results:

- Source suite: **three passed, zero failed, 30 Bun assertions**, with actual Rust runtime `[DEBUG]` evidence. Existing physical Windows junction, owner/search, cancellation, refresh and form-normalization assertions still execute.
- Final focused authority entry: **four passed, zero failed, 32 filtered, 77 assertions, 4.44 seconds**. This includes the new registry-refresh test, source schema/AST/filesystem oracle, actual Rust source discovery/form regression and existing launcher DOM/clamp/input-batch oracle.
- Read-only installed stable `rustfmt --emit stdout --config skip_children=true` accepted the current launcher file. Output was discarded; source was not rewritten. This proves syntax parsing only.

The final focused command from the repository root, with `SEMIO_TEST_ARTIFACT_DIR` set to this ticket's `🗑️generated`, was:

```powershell
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts' --test-name-pattern '^(registry refresh|source suggestions|actual Rust source discovery|launcher input batches)'
```

Generated Rust harness/compiler outputs were created under the caller-owned generated directory and removed after each test through the existing bounded cleanup guard. No permanent helper script or runtime dependency was added; lodash is an existing test dependency.

## Limits and Unchanged Blockers

Whole-dashboard compilation, real terminal pointer/keyboard behavior, installed-native dry-runs and all-platform native integration remain unverified because the admitted native build is blocked. The prior authority failures from the independent writer's restored README/catalog and the policy-rejected directory cleanup are unchanged by this focused fix; this continuation did not rerun or modify those areas. The earlier reports retain the actual failing point-in-time results.

Generic source suggestions still do not reconstruct every helper's purpose, command modes, required domain arguments or historical working directory. Direct runtime `bun:test` import detection is a bounded lexical classifier validated against the shared TypeScript AST cases, not a full parser or proof that a file registers nonzero tests. Large global discovery latency has not been benchmarked. These limits remain explicit rather than being presented as native runtime success.

## Exact Changed Files

Relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`:

```text
🖥️terminal/🚀️launcher/🦀️.rs
🧬️schema/🔎️source-discovery/🔣️.json
🧪️tests/🧫️fixtures/🔎️source-discovery/🔣️.json
🧪️tests/🔎️source-discovery/🟦️.ts
🧪️tests/🔎️source-discovery/🥒️.feature
```

This retained report is the only new ticket file in this continuation. Prior authored inputs and reports remain preserved.

## Additional Current Installation/Execution Caller Audit

The parent subsequently requested a bounded audit of current installation/execution helper signatures after repairing the package TestScript's missing options argument. The current installation implementation, package script, build-owner native dispatch, bootstrap native dispatch, installed-native test readers and execution fixtures were read. Course files, the parent's dispatch test/TestScript and historical ticket helpers were not edited.

A completed TypeScript AST census used the authority fixture's exact inventory policy, independently admitted by Ajv, and fast-glob with only its declared exact excluded roots and dependency boundaries. It read **17,444 active JavaScript/TypeScript-family source files**. Twelve files contained helper references; parsing their declarations/import aliases/direct/namespace/constant-property calls found **52 calls to twelve current helpers**, with **zero arity violations and zero unresolved argument spreads**. Required/maximum arities were derived from the actual current function declarations rather than copied from a historical signature. Literal options objects on the build-owner dispatch, bootstrap dispatch and execution bridge fixture explicitly contain all five required fields: `mode`, `network`, `control`, `invocation`, `workspace`.

| Helper | Current required arguments | Observed calls |
| --- | ---: | ---: |
| `installedDashboard` | 3 | 6 |
| `withInstalledDashboardV1` | 3 | 3 |
| `launchDashboard` | 3 | 6 |
| `dashboardExecutable` | 3 | 6 |
| `buildAndInstallDashboard` | 3 | 2 |
| `installDashboard` | 3, optional fourth source snapshot | 7 |
| `staleDashboard` | 2 | 8 |
| `dashboardInstalled` | 2 | 3 |
| `captureDashboardSources` | 2 | 3 |
| `digestDashboardSources` | 2 | 3 |
| `dashboardInvocation` | 1 | 3 |
| `mayRunStale` | 1 | 2 |

Manual argument-flow review confirmed the current package callers distinguish the crate/package directory from the installation-record root and the Original workspace. `installedDashboard` readers pass record root, selected mode and executable-owning workspace. The package options constructor supplies all required authority fields; build/bootstrap inline options and leased-reader callbacks forward the selected Original control/invocation. The execution fixture deliberately permits an isolated installation-record root while the immutable executable remains owned by the Original workspace. Treating those two roots as identical would change that current contract; no such edit was made. The leased installation reader continues to verify tools custody, immutable bytes/permissions/digest and reader lifetime. No bypass was introduced.

The AST census completed successfully and its generated output is `🗑️generated/dashboard-helper-call-audit.json`. No additional concrete missing/wrong-argument caller defect was established. Accordingly, this audit added no production changes or permanent tests and did not repeat the full authority suite. It is a static current-source/argument-flow audit, not a whole-program typecheck, installed-native runtime proof or proof of arbitrary dynamically constructed calls. The previously recorded native store/runtime and authority conflicts remain separate requirements.

## Final Post-Retirement Full Authority Run

After the parent reported course retirement green and the independent writer finished, the requested final complete non-native authority entry ran once against the converged source, parser, metadata and documentation. Actual result: **34 passed, one failed, one installed-native test filtered, 1,614 assertions, 35 executed tests, 25.25 seconds; exit code 1**.

The sole failure is the unchanged strict active-directory assertion, whose complete refusal list is exactly `[".vscode"]`. The root catalog and seed absence assertion passes. No exemption was added and no retry of the automatic-approval-rejected empty-directory removal was attempted.

Runtime debug evidence confirms:

- All **134 current developer documents** and **23 instruction vectors** passed, including the repaired course README.
- All **1,447 owners, 3,792 targets and 3,791 existing canonical script invocations** passed current metadata/declaration checks.
- Current source-discovery schema/AST/filesystem oracles, actual Rust discovery/form normalization, registry refresh lifecycle and source-subscription regression passed.
- The received dashboard test context passed its seven shared dispatch vectors. Actual Rust reuse/policy bodies, launcher DOM/clamp/input vectors, localized metadata, console-width oracles and typed Bazel phases passed.
- The traversal inspected over **120,000 physical source directories** without broadening exclusions.

The complete command output is retained temporarily at `🗑️generated/final-post-retirement-authority.log`. The exact command, with the caller-owned ticket `SEMIO_TEST_ARTIFACT_DIR`, was:

```powershell
bun test './🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️authority/🟦️.ts' --test-name-pattern '^(?!the native dashboard resolves).*'
```

This is the latest point-in-time authority result and supersedes the earlier README/catalog failures for the areas exercised. It remains a failing exclusivity gate because the directory exists. The installed-native test was explicitly excluded; normal native provisioning and real terminal integration remain unverified. No production changes were made during this verification.
