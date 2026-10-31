# Original Reserved Host Gate Readiness

The selected Bun test body is independent of Cargo, but the exact declared frontdoor `repo --service=off --tool=bazel run @semio-tech/repo-lib:test-original-reserved-host --output=stream` is **not established as safe from overlapping dependency synchronization**. The outer `run` readiness path can install and publish JavaScript dependencies before the graph or test executor is reached. The earlier body-level suitability conclusion does not establish frontdoor readiness. This is a source review, not an execution or a passing result. No test, graph, setup or Cargo command was executed by this audit.

## Outer Frontdoor Correction

In `🏗️build/⌨️command/⚙️runtime/🟦️.ts`, lines 49–52 provide an early return for the separate canonical `cargo` operation. The requested command is `run`, so it continues to lines 53–54. `buildOperationRequiresV1` reads the operation catalogue, whose `run` row requires `dependencies`. That check takes the operation name, not the selected target; there is no original-reserved-host target exception here. With no graph-plan-output request, line 54 computes `buildEnvironmentPlanV1`, optionally checks lock synchronization, and always calls `ensureBuildEnvironmentV1`. `--service=off`, `--tool=bazel` and `--output=stream` do not supply an exception to this predicate.

In `🏗️build/🚀️bootstrap/🟦️.ts`, lines 61–63 select a parser-only scoped installer for Bazel closure mode; other modes retain the complete installer. Both still enter readiness. Lines 145–148 inspect actual readiness, return without installation only when ready, invoke the installer when material is required, and publish/check the receipt. Runtime line 54 raises dependency readmission after material publication. Thus current readiness might avoid installation, but no current supported receipt in this audit establishes that outcome. Closure mode narrows publication scope; it does not remove publication authority.

The outer path also contains later graph, Rust and products regrow/collector calls at runtime lines 85–89 before graph execution. These are further reasons not to describe the whole declared route as containing only Bun sandbox writes. This audit does not claim they necessarily mutate storage for this particular invocation.

Recommendation: do not release this declared frontdoor beside Native8 on the basis of the source-test body alone. Preserve the normal declared route and schedule it when overlapping own dependency synchronization is authorized or excluded by supported current readiness evidence. Do not manufacture an alternate entry or skip flag to bypass readiness.

Current defining frontdoor hashes:

| Library-relative path | SHA-256 |
| --- | --- |
| `🏗️build/⌨️command/⚙️runtime/🟦️.ts` | `6db5a1603f9af7b9a3d4e34ac315d416a02c62edae2a4fc026890eb250cd63e8` |
| `🏗️build/⚙️operations/🟦️.ts` | `635750341293158c1310daa2d0e7ab59c2d36414cb8425bb533d10356f04880e` |
| `🏗️build/⚙️operations/🔣️catalog.json` | `1da8c2a2529c9ac18fa111d66ec3cbd874182b91445fd6bbc659035e52820065` |
| `🏗️build/🚀️bootstrap/🟦️.ts` | `ca171c64748e414681adeadddeebaf8be23a9dd208abc1e69dcf2c1baec793ad` |

The project target uses `repo:script`, `repo ./📜️script.ts test original-reserved-host`, source execution metadata, freshness observation, no declared outputs and three explicit runtime source entries. The ownership route accepts exactly that argument and calls the bounded repository test interface with `process.execPath`, `test` and all three entries together, repository-root cwd, the `original-reserved-host` test artifact environment and a 60,000 ms work budget. The Bun branch does not enter the repository-command interface's separate `command === "cargo"` dynamic admission path.

The host, input and worker entries read current production Rust source text and perform in-memory hostile mutations. Ajv/Ajv2020 validates the neutral schemas and fast-json-patch supplies the independent omission/mutation expectation. The host additionally runs framework and puzzle route classifiers. These are source-contract tests; they do not compile or exercise the native worker implementation. The production instance loader reads six existing families: instance, maintenance-ladder, cancellation, retained-command, job and command. No tested route invokes setup, Cargo, dependency publication or native-storage maintenance. Normal test infrastructure does create bounded sandbox/protocol artifacts and a child process, so this is not an effect-free operation.

Prerequisites are the canonical repo route, its Bun executable, resolvable Ajv and fast-json-patch, current neutral fixtures/schemas, current first-party classifier imports and readable production source families. Their successful runtime resolution has not been proved here. The declared target also includes default/transitive/default, test environment, reads and the six Rust-family input globs. A future execution should preserve its input receipt; the hashes below are a bounded current checkpoint, not an exhaustive transitive seal.

All paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

| Input | SHA-256 |
| --- | --- |
| `📦️packages/🟦️typescript/📋️project.json` | `6ac01c31a34979c52f155b29f5c9b16c955ac91d660d844082490473ef0ce614` |
| `📦️packages/🟦️typescript/📜️script.ts` | `623dc3bec4c285a6e430beb99007e8f1f8ba0136682d670567a8c1462919dc76` |
| `🧪️tests/🔬️typescript-package-commands/🟦️.ts` | `0a5f3ffa5ab772e42ecf989b44884d0d62608a5318bd427f9127fd0f42df8ad9` |
| `🧪️tests/🔬️typescript-package-commands/⚖️ownership-law-routes/🟦️.ts` | `7d9f8b9ec8adb625a6dbdb8168a992a6fe4e3e5ecd7133f057ec4a00dc950756` |
| `🧪️testing/🎛️repository-command/🟦️.ts` | `614972072fcfbdcc67000876956b1af3d2aa6ed235a649a0aa211e3f039d6c3b` |
| Host test | `2c2379b57828795f64c36fb086ca2f7eddd09476f07e9d0954d202deac381f3d` |
| Host schema / fixture | `48ab98fd9e2594be5c25921941676e125c37532c8e926766165b31809b90cd85` / `d9b675119ad62fed2ce30c9683db0eabc2108d6d6bd377eef15d2b7cdea74019` |
| Input test | `6deb8deff70f8142ab9a211c9c88c4492aa07f4da99bf93e10e815aa02708304` |
| Input schema / fixture | `3fd3d9b39854439dfda2fa3ecf8a4d65e84e50403a16790b9d42e5b63d7dffe0` / `f5c9c63f3cea0575299b98e4c3ef2907d3bc04f2c8aebd18acb9a7ef33e0c95b` |
| Worker test | `6b6d7db463de32da66a968b3572164ea9119bc6e62045d6d0b704c758fa5d40c` |
| Worker schema / fixture | `350a4ecdd51e04bfc3ed7003255d6d11051fb01a21b51b60258f5d607538597b` / `8c27a10c774002932b36abe696421ab78386cdc7e6e773b5977a7900fe1d8d35` |

The three tests live respectively under `⚖️laws/🎯️tool-job-coverage/🛣️routes/🧬️original-host`, `⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🧬️original-input` and `⚖️laws/🎯️tool-job-coverage/🔒️lifecycle/🧬️original-worker`, with `🧪️tests/🟦️.ts` and sibling `📐️.json` / `🔣️.json`.
