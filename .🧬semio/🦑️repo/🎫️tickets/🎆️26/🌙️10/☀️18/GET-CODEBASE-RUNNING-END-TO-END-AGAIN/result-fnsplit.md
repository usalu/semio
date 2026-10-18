# Result fnsplit (tjc, srs, flow brep `register`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no cargo, no `bun repo` build. `C` = this ticket folder. Tools: `C/fnsplit-tools/`. Snapshots of the originals: `C/fnsplit-tools/orig/` (`tjc.ts`, `srs.ts`, `register-operators.rs`). Scratch (outside the repo): `/Users/ueli/.cache/fnsplit/` (rustc sandboxes), the c6fin2 scratch `nm/node_modules` for `bun test`.

## 0. Verdict

| tag | file | before | root after | children | largest child | status |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| tjc | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-coverage/🟦️.ts` | 1,863 (one function, 1,836 lines) | 61 | 11 | 406 (`🖼️raster-envelope`) | DONE |
| srs | `…/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts` | 3,692 (one function, 3,672 lines) | 48 | 20 | 430 (`💓️heartbeat-and-liveness-watchdogs`) | DONE |
| fb | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧾️register-operators/🦀️.rs` (`pub fn register`, 1,088 lines) | 1,108 | 63 | 6 | 277 (`🧮️booleans-transforms-and-solid-features`) | DONE |

Every new file is at most 430 lines, every function is a named phase, group or part. No behaviour change, no new dependency, no compatibility layer. Exported API unchanged: `toolJobCoverageSelfTests()` (imported lazily by the repo root `📜️script.ts` and typed in `self-test-lanes`), `registerTests1(vitest, dependencies, testSource)` (imported by `🎭️actor-transport`), `register`, `module_registry`, `extension_manifest_json`.

## 1. tjc (`toolJobCoverageSelfTests`, 731 depth-1 statements)

Method: `fnsplit-tools/psplit.cjs` (TypeScript checker based phase splitter). The statements stay verbatim and in order; the function is cut into 28 contiguous phase functions in 10 semantic child files plus 1 helper file; locals that live across phases (`fixtures`, the retained writer/jack/raster/drawing sources) are returned by the producing phase and passed by the root as one typed object. The root keeps the five one-liners that feed the return expression (`checkpointChecks`, `memberProducerChecks`, `scalarConfig`, `sealer`) and the original return statement (`fixtures.length + 394 + …`). Two pure closures are hoisted to module level and renamed: `evaluate` becomes `evaluateToolJobSources`, `exactProof` becomes `exactBoundedFirstStepProof` (`🧮️source-evaluation`).

Children under `…/🔬️tool-job-coverage/` (lines): `🧾️proof-catalog-and-admission` 173, `🧯️payload-store-progress-and-mounted-session` 211, `📤️publication-contracts` 182, `🛑️cancellation-close-and-output-ownership` 123, `🗄️artifact-store-and-envelope-codec` 47, `✍️caller-retained-document-envelopes` 249, `🖼️raster-envelope` 406, `🎨️drawing-envelope` 405, `🤝️peer-presence-and-command-ingress` 70, `🔭️universal-and-layout-retained-ownership` 125, `🧮️source-evaluation` 30.

Oracles (all run here):
- `verify-seq.cjs tjc.json orig/tjc.ts`: expands the new root phase by phase (AST) and compares with the 731 original statements: `statements original=731 (helpers 2) expanded=729 phases=28 root-kept=5 LOSSLESS OK` (729 plus the 2 hoisted helpers, which are compared separately after dedent and rename).
- Runtime differential `run-tjc.sh old|new` (bun test, instrumented: every `if` guarding a `throw` is recorded as T/F, every `throw` is recorded and does not abort, every `*SelfTests()` call is isolated with try/catch): old (snapshot) and new produce the identical 564-event trace, the identical result (`NaN`, because of environment-only failures of imported lanes) and no error. Negative control: mutating one fixture string in the new tree (`TJC_MUTATE`) changed the trace at event 512, so the harness is sensitive. The old function already fails `mounted-fem2d-valid` and some library lanes in this checkout (library drift, missing `immer`, `fast-json-stable-stringify`); identical in old and new.
- Type check (`tscfiles.py`, repo tsconfig, cached TypeScript): no diagnostic in any new file except the environment ones that the original has (missing `node_modules`, `@types/node`). One real issue found and fixed on the way: literal types of `const` strings must be widened in the cross-phase parameter types (`getBaseTypeOfLiteralType`).
- `newcheck.py` (folder shape, docstring emoji = folder emoji, taxonomy prefix, unique among siblings, path length): 11 files, 0 problems, longest path 198 bytes.

## 2. srs (`registerTests1`, 3,672 lines)

Method: `fnsplit-tools/ksplit.cjs` (test-registration splitter). Structure:
- Kit: `🧰️settlement-test-kit` (241 lines) exports `createShardClientSettlementTestKit(vitest, dependencies, testSource)` and `ShardClientSettlementTestKit = ReturnType<…>`. It holds the dependency destructure, the vitest destructure, the closure-bound fixtures (`harness`, `fixtureResident*`, `BUDGET`, `fixtureHosts`, `bindFixtureHost`, `fixtureRetirement`, `answerLifecycle`, `captureFixtureInstance`, `retireFixtureInstance`, `activateActor`, `prepareResidentFixture`) and returns exactly the names that children use. Module-level exports of the kit file: `spyGetter`, `afterSettle`, `TestSource`, the 38 type aliases, and the pure helpers `FakeShardWorker`, `flushAdmission`, `makeEffectRequestFrame`, `findEffectReply`, `flushMicrotasks`. Declarations used by one group moved into that group (`workerPreparationFixture`, `prepareWorkerFixture`, `fixtureOutputReservation`, `makeEnvelope`).
- Root `registerTests1` creates the kit and calls 14 groups in the original order; the six `//#region` pairs and `void vi;` stay in the root around the calls.
- Groups: `🚀️worker-bootstrap` 250, `💾️resident-composition` 224, `🧾️reserved-response-settlement` 51, `🔒️captured-instance-lifecycle` 269, `🎟️activation-lease` 236, `🚪️instance-close-and-round-trip` 136, `📥️segmented-download-and-multiplexing` 215, `💓️heartbeat-and-liveness-watchdogs` 430, `🛰️lease-watchdog-and-shard-loss` 173, `📨️envelope-ordering-and-frames` 187, `🌉️host-effect-bridge` 102, `🛂️inbound-activation-authority` 116, `💼️job-wire` 43.
- The 1,050-line `describe("ShardClient captured return authority")` stays one describe node: `🪪️captured-return-authority` (19) opens it and calls `🛠️captured-return-helpers` (64, the five shared fixtures) and four part functions that register their `it`s inside the same describe: `🔓️captured-return-admission` 280, `🔗️kernel-return-input-binding` 270, `🧵️kernel-return-input-settlement` 313, `🔀️captured-return-routing` 134. The test hierarchy and full test names are therefore unchanged.

Oracles:
- `run-srs.sh old|new`: registers the suite against a recording vitest (nesting, order, names, sha256 of the whitespace and import-depth normalised source of every test callback) and a proxy dependency bag. Old and new: identical 216-event trace, 158 tests, no error. Negative controls: renaming a test and altering one assertion inside a test body both change the trace.
- `verify-text.cjs orig/srs.ts registerTests1 srs-files.txt`: all 265 original statements (depth 1, depth 1 of each top-level describe, module-level) occur verbatim exactly once in the new files (modulo indentation, relative import depth, `export `): `TEXT LOSSLESS OK`.
- Type check: error-code multiset of the new tree equals the one of the original (placed temporarily next to the tree, removed again) except one extra `TS2339 ImportMeta.vitest`, which comes from the signature type being repeated in the kit factory and is environment-only (no `vitest` types here).
- `newcheck.py`: 20 files, 0 problems, longest path 214 bytes.
- Not possible here: a real run of the suite. `🔨️modules/🎭️actor/📮️shard-client` does not load under bun in this checkout (`Cannot assign to import "submitCapturedReturn"` in its `static {}` block, a peer's file), and `vitest` is missing. The coordinator should run `🎭️actor-transport` with the full `node_modules`.

## 3. fb (`pub fn register`, flow brep extension)

Method: `fnsplit-tools/rsplit.py` cuts the statement run of `register` into 14 group functions in 6 child modules mounted by `#[path]` from `🧾️register-operators/🦀️.rs` (children `use super::*;`, functions `pub(super)`, called in the original order from `register`, which keeps `mesh::register_mesh` and `registry.finalize()`):
`🧊️schemas-primitives-and-curves` (`register_schemas_and_deconstruct`, `register_primitive_solids`, `register_curves`) 219, `🪡️surfaces-and-sweeps` (`register_surfaces`, `register_sweeps`) 175, `🧮️booleans-transforms-and-solid-features` (`register_booleans`, `register_transforms_and_patterns`, `register_solid_features`) 277, `🔍️intersection-and-evaluation-operators` (`register_intersections`, `register_evaluations`) 191, `📐️measurement-utility-and-topology-operators` (`register_measurements`, `register_utilities`, `register_topology`) 186, `📂️io-operators` (`register_io`) 95. Root file: 1,108 to 63 lines.

Oracles:
- Lossless: the 1,071 non-blank statement lines of the old body equal, in order, the concatenation of the group bodies plus `mesh::register_mesh` and `registry.finalize();` (checked inside `rsplit.py` before writing).
- `rs-oracle.py` (oracle5 machinery, sandbox with the original file and without the new folders against the live tree, `rustc +nightly-2026-07-20 --emit=metadata`, lib and `--test`): errors orig 1,878, new 1,892; the only difference is 14 x `cannot find type Session` = exactly the 14 new function signatures (`Session` comes from an external glob that the oracle neutralises identically on both sides). No unresolved in-crate name (`reg_geo`, `register_typed`, `q`, the operator structs). A toy crate in the scratchpad proves that a grandchild module resolves names through two chained private glob imports and `pub(super)` functions.
- `rgi.mjs`: 6 of 6 folder names RGI; `tax.ts`: 6 of 6 resolve (`crate-module-children`); `rustfmt --edition 2021 --check`: all 7 files parse (formatting diffs only, no errors).
- Source-boundary gate (`census-brep.ts`, canonical `readRustSourceClosureV1`): `semio-s-plugin-flow-brep` closure 16 files, 208,236 bytes, 3,048 active lines (ratio 0.794), inside the gate (lib and test closure equal).

## 4. Crates the coordinator must compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`:
1. `semio-s-plugin-flow-brep` (also with `--features testkit`)
2. `semio-s-plugin-flow-brep-tests` (unit tests of the extension)
Dependents worth a check afterwards: `semio-s-plugin-flow-brep-component`, the flow dev package (`✏️s/🧑‍💻dev/🌊️flow`) and the procedural playbook extension that depend on `semio-s-plugin-flow-brep`.

TS tests to run with a full `node_modules`: `bun test` of `🔌️plugin/🧪️tests/🎭️actor-transport/🟦️.ts` (srs), and the library lane `toolJobCoverageSelfTests` through `bun run 📜️script.ts` self-test lanes (tjc; note it currently fails `mounted-fem2d-valid` for a reason that predates this split).

## 5. Open risks

- tjc and the library: the single failing assertion `mounted-fem2d-valid` and the failing `memberStoreProducerSourceSelfTests` ("portable guard mismatch") exist in the unsplit function as well (library drift by peers). Not touched.
- srs: `describe` blocks are registered by functions in different files; the hierarchy is preserved (verified by trace), but a future edit that adds `beforeEach` state to the captured return authority describe must go into the wrapper or the helpers factory, not a part.
- Peers edit `📜️script.ts` and the library `self-test-lanes`; both only import the unchanged root paths.
- The splitters refuse to write when the root changed during the run (all three applied cleanly). Nothing was committed or staged.
- Tool files: `C/fnsplit-tools/` (`psplit.cjs`, `ksplit.cjs`, `rsplit.py`, specs `*-spec.cjs`/`rs-spec.py`, `verify-seq.cjs`, `verify-text.cjs`, `rs-oracle.py`, `census-brep.ts`, `run-tjc.sh`, `run-srs.sh`, `tjc-run.test.ts`, `srs-run.test.ts`, `revert-split.cjs`). The recorded traces under `C/🗑️generated/fnsplit-*.json` were removed after the comparison.
