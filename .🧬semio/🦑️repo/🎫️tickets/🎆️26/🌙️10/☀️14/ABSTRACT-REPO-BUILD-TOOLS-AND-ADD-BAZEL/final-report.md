# Repository Build Framework and Bazel

The repository owns one neutral build interface, with Nx configured as the main tool and Bazel available explicitly. The default remains `nx` in the build policy while broader native platform qualification remains outstanding.

## Developer Interface

```text
bun repo run <project>:<task>[:<configuration>]
bun repo --tool=bazel run <project>:<task>[:<configuration>]
bun repo tools setup all
bun repo graph
bun repo show projects
```

The same interface supports run-many, affected selection, execution, project/task inspection, watch, reset, strict portable flags, argument boundaries, progress and cancellation. `SEMIO_BUILD_TOOL` carries the chosen implementation through nested orchestration. English and German editor entries expose the infrastructure commands and tool selection.

Configurations override execution options. Executor kind, native ownership, dependencies, inputs, outputs and cache policy belong to the target; a configuration preserves its command form. Both backends enforce this contract and preserve base environment values when a configuration supplies partial overrides.

Task arguments use literal string arrays. All eight prior scalar defaults were converted by hand; authoring and graph admission reject scalar arguments. A private Nx bridge preserves array and caller boundaries through Bun's portable shell, using a digest-addressed source file bound to the original invocation. Configured command and directory alternatives contribute their full script closures and required conservative fallbacks to shared task inputs.

## Implementation

- Schema-owned project declarations, task graph, selection, execution recipes, inputs, configurations and resource grants live in the repo framework. Shared inference, input closures, observation, mode policy and catalog build contributions have neutral ownership. Private adapters translate to backend protocols; the Nx entry does not reexport shared inference APIs.
- Public package commands, dashboard, MCP agent instructions, setup, devcontainer, CI, native bootstrap, native producers, application orchestration and editor controls use the neutral interface. Package authoring no longer contains Nx project metadata.
- Bazel uses pinned 8.4.2 distributions with checksums for six platform/architecture combinations. Setup seeds a shared fixed-module cache and lock; new task namespaces run with downloads disabled. Uncached offline tool/module setup fails before network access. Task servers have a 60-second idle limit. Its backend implements task dependency closure, configurations, arguments, source/lock/environment hashing, action products and restoration, nested orchestration, uncached actions, nonparallel operations, continuous service readiness and cancellation.
- Canonical graphs publish atomically with source authority and immutable digest pins. Nx graph/socket caches use that digest. Native Cargo discovery follows actual local owner dependencies and rejects stable missing owners. Concurrent unrelated crate additions do not invalidate each native owner's assembly key. Graph receipts preserve physical membership/type/mode plus consumed raw facts and registered structural analyses. Parser revision, current source SHA and nested observations guard semantic replay; execution caches continue hashing full input bytes.
- Artifact publication uses `.semio-artifact.json` across current publishers, consumers and schemas. Existing products with proven current ownership were corrected under their original publication leases, preserving marker and payload hashes. Unproven historical products and backend cache snapshots were preserved.

## Executed Validation

| Scope | Result |
| --- | --- |
| First fresh canonical graph | Published in 173.25 s after one correctly detected concurrent edit: 1,179 projects, 33,853 edges, 9,848 base targets and 131,150 authority observations. The actual Nx run exposed six generated policy facets sharing roots with declared owners; their common ownership correction is implemented and regression-tested. Fresh current-store publication remains pending |
| Dashboard launch and public integration laws | Earlier combined launch/surface gate passed 14 tests, 27,057 assertions, including independent Ajv/RFC6902/string-argv/TOML oracles and actual producer witnesses. The latest registered build-surfaces preflight passed 10 tests, 27,385 assertions under an original 120-second invocation |
| Neutral graph and configuration contract | Latest combined checkpoint passed 51 tests, 888 assertions in 16.31 s, including strict Ajv, actual Nx options/task graph/hash, configured source authority, literal arrays, policy ownership, provider revision, shorthand/input extraction, current artifact store, controlled reset, GUI authority, mandatory pins, loaded-module authority and Rust producer graft planning; earlier core/model/scheduling gate passed 28 tests, 696 assertions |
| Real pinned Bazel backend and bootstrap | 8 tests, 123 assertions passed in 35.80 s under broken proxy settings; real cache restoration, invalidation, nested calls, release configuration, cold offline namespace setup, service/user/parallel command-tree termination and bounded policy. Current public `bun repo tools setup bazel` passed in 0.919 s through the enforcing tooling store, verifying the warm pinned tool |
| Literal argument contract and backend parity | Arrays-only common gate passed 2 tests, 251 assertions. Actual installed Nx/Bazel runtime parity passed 1 test, 24 assertions across eight fixture configurations using the converted authored values, preserving spaces, empty values, quotes, `$()` and `--` with original matching-tool capabilities |
| Private Nx bridge controls | 1 test, 21 assertions passed: malformed/corrupt/digest/wire/expired inputs refused and the actual child terminated at its unchanged two-second deadline. Native route rerun passed 4 tests, 73 assertions; bridge strict TypeScript passed and installed Nx resolved its actual registry/schema/implementation |
| Private Nx engine cache identity | Actual native Nx HashPlanner/TaskHasher regression passed 1 test, 5 assertions: engine source and external package changes alter task identity; invocation staging does not. Projection binds the real local engine/helper import closure and Node/Bun versions while preserving effective default inputs |
| Current production stages and artifact store | Read-only configured source inference passed 1 test, 189 assertions through the exact production normalizer, retaining independent schema, Nx, TypeScript and esbuild checks. Three artifact tests, 19 assertions passed through the real enforcing store with original invocations. Separate owner and manifest cancellation laws remain intact; full producer admission is the fresh publication gate |
| Controlled graph reset | 1 test, 24 assertions passed with a real filesystem inventory, independent fast-glob selection, closed ownership refusals and genuine original cancellation before storage admission. Load/reset share enforcing custody and the graph-state lease; canonical.json is distinct from the charge receipt. A live mounted-partition reset was not executed |
| Current dashboard publication | Three source/oracle tests, 31 assertions passed; a freshly compiled standalone Rust helper derived the same schema-owned current path as TypeScript. Registry, inventory, fixtures and coverage oracles use the shared declaration. A full dashboard binary containing this new source was not executed |
| Loaded implementation and pinned reuse | Focused guard/pin/receipt/bootstrap gate passed 4 tests, 69 assertions. Required authority version 3 binds loaded implementation/declaration hashes; production admission rejects old or mismatched receipts. Actual discovery subsequently refused a concurrent loaded-module edit before publication |
| Cross-language workspace identity and native Bash bootstrap | 4 tests, 187 assertions passed across actual TypeScript, Rust, Python, Go and C# hosts; 22 Bash identity/JSON vectors |
| Native DSL library through Bazel | 22 tests passed |
| MCP through Bazel | Protocol tests passed; live executable build passed; independent SDK checked neutral server instructions |
| Native caller/private adapter boundary | 17 tests, 317 assertions plus 4 tests, 73 assertions passed |
| Installed dashboard bridge with authored grants | Real CLI subprocess passed 1 test, 16 assertions; preserved original limits, tool and deadline; no graph acquired |
| Shared observation/input contracts | 44 observation tests, 185 assertions and 3 input tests, 25 assertions passed; independent schema/glob/graph/hash/Nx-expansion oracles |
| Compact input graph expansion | 5 tests, 26 assertions passed through actual pin/load; nested arrays preserved and reads bounded at 16 |
| Native watch source authority | Actual Bun/Node plan equality, six required source paths, seven minimatch vectors, live changes/deletion/restoration, identical-byte suppression and cancellation passed |
| Native script admission | Actual filesystem rename, invalid UTF-8, ASCII/multibyte boundaries, semantic body rewrite, oversize and absence vectors passed; the affected owner surface test and targeted strict API checks passed |
| Neutral artifact ownership | Publisher/browser-consumer regression and independent Python/fast-glob checks passed; 97 validated marker assets renamed with unchanged complete payload hashes |
| Actual Node inference admission | 2 tests, 25 assertions passed; current recomputation after an import race, reverse command ownership and unrelated-owner reuse checked against the TypeScript parser oracle |
| Fresh dependency bootstrap | Earlier isolated missing-node_modules frozen installation succeeded. After the managed-layout refactor, real bounded lock/synchronization admitted 1,495 packages and 96 bindings in 74 s under the original 120-second policy; the declaring package owns its test-only argv oracle |
| Authored project declarations | Strict Ajv admitted 633 declarations at its snapshot and resolved all 480 present local schema references. A later production execution-contract preflight admitted all 662 discovered project files and 76 execution declarations, with native Cargo owner checks. The package audit inspected 275 manifests without Nx metadata or direct Nx script callers |
| Scope and syntax audit | Scoped Git whitespace check passed; no recorded existing source path exceeds 256 UTF-16 units; Bash syntax passed |

The first diagnostic snapshot has immutable digest `c9d6df4f84aa51f053fc206e4bacd592b24d631b41830232536bca05a9419845`. Corrected graph ownership, exhaustive Bazel planning and final default Nx/Bazel execution are being qualified. Their final results are recorded below when complete.

## Qualification Limits

The host executed macOS checks. Native Windows, native Linux and devcontainer runtime execution were not performed; PowerShell is unavailable here. Nx remains the default. Full TypeScript dependency-closure checks encountered concurrent native-build errors (`sourceBoundary` missing from `CargoArtifactBuildOptionsV1`, and undefined `plan`/`options` plus an `await` in a synchronous component-pass function). The owned graph/helper check and the launch helper/test strict check passed. The dashboard's retained compiled test binary passed 41 relevant registry/inventory tests; a fresh whole-dashboard Rust compile encountered an unrelated concurrent UI scene type error. The dashboard report records binary/source digests and the source-only assertion correction.

All task plans can be checked without executing tasks that clean, commit, deploy or otherwise change external state. Only the representative safe runtime tasks documented here were executed; no claim is made that every monorepo action was run.

## Audit Trail

[Core](build-core.md), [Bazel](bazel-validation.md), [Dashboard](📝️dashboard.md), [Current dashboard publication](dashboard-publication.md), [Workspace identity](workspace-identity.md), [Native bootstrap](bootstrap-identity.md), [Artifact ownership](artifact-marker.md), [Asset witnesses](artifact-marker-assets.md), [Graph reset](graph-reset.md), [Configured inputs](configured-inputs.md), [Integration](integration.md), [Final interface audit](final-audit.md), [Source inventory](source-changes.md). Exact created, edited and removed paths are preserved in `📥️files.json`. Ticket input harnesses and Markdown reports are retained; generated logs and compiler output are removed before closure. No modifying Git command, worktree or AGENTS.md edit was used.
