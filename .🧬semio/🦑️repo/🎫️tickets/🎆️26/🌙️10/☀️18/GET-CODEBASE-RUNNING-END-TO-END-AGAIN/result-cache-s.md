# result-cache-s: cache coverage of the `✏️s/**` manifests (2026-10-19)

Executor `cache-s`. Static only: no `bun repo`, no nx graph, no cargo, no builds. Scope: the 325 tracked, nx-included `📋️project.json` under `✏️s/`. Tools: `C/cache-s-tools/` (`apply.ts`, `decide.ts`, `rules.ts`, `extras.ts`, `reach.ts`, `jsonspan.ts`, `lib.ts`, `validate.ts`, `ajv.ts`, `census.ts`). Edits are text-surgical (JSON offsets), so formatting of every manifest is preserved; each file was re-read right before it was written; originals are in `🗑️generated/cache-s-backup.json`.

## Before / after (census replica, owner `✏️s`)

| Class | Before | After |
|---|---|---|
| targets | 1425 | 1423 (two projects were deleted by a peer meanwhile) |
| deterministic-cacheable | 1003 | 1362 |
| deterministic-but-cache-off | 307 | 9 |
| unclassified-cache-off | 110 | 47 |
| side-effecting / live | 2 / 3 | 2 / 3 |
| raw `cache: true` / `false` / absent | 385 / 416 / 624 | 749 / 55 / 619 |
| targets without `executor` | 112 | 0 |

The 9 remaining deterministic-but-cache-off are deliberate or plugin-forced (see below). The 47 unclassified are all pinned off with a reason.

## What was changed (211 manifests)

1. **366 targets flipped to `cache: true`** (all deterministic cache-off `test*/check*/verify*/build` and the deterministic unclassified checks: `graph-wire-check`, `preview-generated`, `mutation-inventory(-check)`, `source-check`, `native-schema-check`, `retained-opc-check`, gis `inference-*`, `typecheck`, `catalog-check`, `jack-graph-check`, ...). Five Rust/TS `build` targets got `outputs: ["{projectRoot}/dist/build"]` (the pinned artifact contract of `📇️package-inventory`).
2. **112 targets converted to the `repo:script` form** (coordinator request): `"executor": "repo:script"` and an explicit `options.cwd` (project root) added; the command was already `bun ./📜️script.ts <cmd>` in every case, so no handler had to move. 95 of them are stdio, the rest are generators, `deps`, `carrier-reproduce`, `🧪️test*`. Zero unconverted. The one entry of group "other" (`👴️leutwiler/💤️realparts-of-powers-z-n` `build`) is `repo:noop` (an aggregate with `dependsOn`, no command); left as is, it calls nothing.
3. **Four effectively cached targets pinned `cache: false`** (name family defaulted to cached, but they write checked-in files or use the network): `fixture`, `regenerate-example-assets`, `test-bestest-regenerate`, `tiles-prefetch`. cad `generate` stays off (regenerates a checked-in example through the cad engine closure).
4. **Inputs**
   - Rust owners: the plugin replaces the inputs of `build|wasm|native|test|lint|check|verify*` targets by `nativeSources`/`nativeTestSources` (cargo module walk of the crate only; manifest `inputs` of those targets are discarded). Reads outside that walk (TS oracle tests, fixtures, framework test suites that the `verify-*`/`test-*` handlers run) were therefore declared at project level in `namedInputs.nativeSources` for 130 Rust projects (derived by `reach.ts` from the path literals of the owning `📜️script.ts`, import specifiers excluded; family granular globs plus the exact test directories; projects with unresolvable paths get the owner's typed `*.{ts,tsx,json,semio,wit,sql}`). 17 script-owner projects got the same under `namedInputs.default` (only reads outside the owner tree).
   - 7 generic (non-native-named) cached targets of Cargo projects (`*-check`, `io-descriptor-parity*`, `source-check`, ...) and every flipped generic one got `inputs: [default, ^default, <owning Cargo.lock>]` (`Cargo.lock` is not in the extension set of `default`).
   - Explicit inputs where the handler provably reaches outside the owner: `typecheck` (whole `✏️s` and `🧰️framework` TS/JSON, `tsconfig.json`, `typescript`), six gis checks (gis plugin + gis dev variants + `vitest`), `carrier-reproduce` (the artifact standards JSON + `ajv`).
5. **Outputs**: generator targets keep `outputs: []` because the plugin fills their outputs from the taxonomy generator contracts; output-less families (`test/check/verify/...`) keep absent outputs.

## Kept `cache: false` (reason; the schema has no reason field, none was invented)

| Targets | Reason |
|---|---|
| `canonical-architecture` (20) | repo-wide architecture gate, run with `--skip-cache`; policy `targetDefaults` pins it off |
| `hub-agent-participant-check`, `plugin-coverage-check`, `user-path-check`, `inference-quartet-check`, `security-check`, `agent-reply-check`, `live-agent-loop-check` | need live hub/serve endpoints, hour-scale budgets |
| `fixture-ownership-check`, `canonical-pair-composition-check`, `rebuild-all` | orchestrate other tasks with `--skip-cache` / full rebuild |
| `guest-framework-check` | plan-defined multi-workspace cargo check; input closure not declarable statically |
| `browser-test` (2), `mcp-client-e2e`, `verify-fem-live-visual-publication` | real browser / live end-to-end |
| `tiles-prefetch`, `lock`, `run`, `deps`, `oracle-*` (5), `oracle-setup` | network, `cargo update`, interactive shell, external EnergyPlus/uv toolchain |
| `composition-prepare` (2), `generate-register-catalog`, `regenerate-example-assets`, `test-bestest-regenerate`, `fixture`, cad `generate`, `🧪️witness` | write checked-in files / witness output at undeclared paths |
| `deletion-proof`, `sqlite-oracle` | need an externally provided `SEMIO_TEST_ARTIFACT_DIR` / argument-and-stdin helper |

## Plugin-forced (manifest `cache` has no effect; owner: inference plugin)

- `project-assembly` sets `cache=false` and `outputs=[📤️dist/mode, dist]` for **`build` and `test` of every TS project tagged `role:artifact` + `language:typescript`**. I left their manifests unchanged (the 110+ `cache: true` builds are dead until that rule changes). Same for cargo `wasm`/`wasm-release` (forced off, outputs `dist/wasm/mode`).
- `nativeTarget` inputs ignore manifest `inputs`; `verify-*` use `nativeSources` (no integration tests) although some run cargo integration tests (`kind:"test"`, for example jack `verify jack-child-selection`, generation3d widget-output laws). Spec: any target that runs tests should use `nativeTestSources`.
- Bazel adapter `taskInputs`: `^group` expands over direct edges only (nx: transitive) and drops `dependentTasksOutputFiles`.

## Spec for the synthesized playground targets (plugin `🎮️playground-targets`, 28 per variant, 156 variants)

| Family per variant `V` | Deterministic? | Inputs it would need | Outputs it would need | Note |
|---|---|---|---|---|
| `prepare-V-{react,wgpu}-{prototype,release}`, `prepare-V-{react,wgpu}` | yes (builds the closure through the driver) | `nativeSources` of the component closure (runtime component closure of the row, appScoped for boot), toolchain fingerprint, `SEMIO_BUILD_MODE`, prerequisite outputs | `{workspaceRoot}/<modules>/<deployment-directory>` for every closure member (what the plugin already declares) | pinned `cache: false` by `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` ("answers to Cargo freshness"): enabling cache needs that test changed together with the generator |
| `prepare-V-native-{prototype,release}` | yes | as above, full closure (`appScoped` per row) | module dirs of the full closure (`outputs.slice(1)`) | same pin |
| `activate-V-react-{p,r}` | yes (already `cache: true`) | receipts/hashes of the prepared modules (`dependentTasksOutputFiles`), `activationSources` | activation receipt dir | keep |
| `activate-V-wgpu-{p,r}` | no, publishes live extensions | n/a | n/a | keep off |
| `serve|dev-V-*`, `mcp-V-*` | continuous | n/a | n/a | keep off |
| `run-V-native-*`, `smoke-V-native-*` | live runs | n/a | n/a | keep off |
| `build-V-react-release`, `build-V-react` | yes in principle | prepare outputs, `@semio-tech/assets:build` output, vite config and site sources | `{projectRoot}/dist/build-V-react/release` or `{workspaceRoot}/<distDir>/release` (declared, but vite writes the leased products store, see `result-bzl-man.md` finding 2) | cache only after the store ingests a declared output tree; pinned off by the same test |
| `build-V-site`, per-crate `build` | aggregates (`repo:noop`) | none | none | not cache-relevant |
| `session-V` (plugin-registry) | yes (already `cache: true`) | row metadata, Cargo manifests | session file | keep |
| `build-all-playground-cdn-sites` | no (CDN publish) | n/a | n/a | keep off |

## Verification

| Check | Command | Result |
|---|---|---|
| Repo validator (`validateJsonSchemaSubset`) on all 325 manifests against `🏗️build/🧬️schema/📋️project/🔣️.json` | `bun C/cache-s-tools/validate.ts` | 0 invalid |
| Third-party (Ajv, draft-07, allErrors) | `bun C/cache-s-tools/ajv.ts` | 325 checked, 0 bad |
| Only intended properties changed (parsed diff against backup of the 211 changed files: only `cache/outputs/inputs/executor`, `options.cwd`, `namedInputs`) | `validate.ts` | 0 unintended |
| Idempotence | `bun C/cache-s-tools/apply.ts` (dry run after write) | 0 files to change |
| Census re-run | `cd C/cache-s-tools && bun census.ts cache-s-after.json` | numbers above (`🗑️generated/cache-s-before.json`, `cache-s-after.json`) |
| Not run | nx graph load, `bun repo show projects`, any Bazel/nx task run, cargo, existing bun suites (`⚡️caching/🧪️tests`), so the effective plugin output (inputs after `project-assembly`) is derived by reading `project-assembly`/`📥️inputs`, not observed |

## Open items

1. Run the nx graph once and check `namedInputs.nativeSources` (130 projects) does not trip `declaredSourceInputs`/native watch plan (`Unresolved native watch source coordinate` is only raised for `{workspaceRoot}`/`{projectRoot}` left inside a path; the brace globs are fine by reading). The extras also widen the dev source-watch plan of plugin crates.
2. The reach scan sees only string literals in the owning script; dynamic paths (fixture-driven `app.source`, template strings) fall back to the owner-wide typed glob. Transitive TS imports of oracle tests into other modules are not tracked.
3. Pinned test `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` asserts `cache === false` for synthesized prepare/native/build targets; it moves together with the plugin change.
4. `generator` target `outputs: []` placeholders are filled by the plugin from taxonomy contracts; emoji-prefixed generator names stay invisible to name defaults (policy owner).
5. Peer deleted `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/...` during the run; tools skip missing files.
