# Script Router Policy And Source Separation

Date: 2026-09-12

## Outcome

The `root-script` body policy now recognizes only closed task-routing forms whose effects resolve to runtime value imports or to values proven inside the same lexical scope. It does not admit a file because it is named `📜️script.ts`, extends `BundleScript`, or appears at a known path. The final live census of the 89 bounded candidates classifies 83 as `tool-metadata` and retains 6 as current `unresolved` reviews for separate ownership decisions.

Disk discovery now inspects source bodies under a package language root even when that ecosystem’s manifest is absent. Manifest parsing and package discovery still require the exact manifest, target owners still report a missing manifest, and catalog-backed discovery remains read-only.

The DSL fixture-sweep package router now delegates report parsing and semantic coverage assertions to its neutral verification owner. The executable route, native Cargo-law runner, budget/cancellation flow, and receipt handling remain in the mandatory package command leaf.

## Closed Router Grammar

The admitted forms are limited to:

- imports and imported binding provenance; one `run` method on each `BundleScript` subclass and no extra class body;
- imported command delegation, imported command construction, `process.exit` of a delegated result, and the established router/main terminal calls;
- argument guards, literal status logging, immutable task locals, and exact scalar environment assignment under an exact `process.argv[index] === literal` guard;
- finite literal target iteration; local expression-bodied closures whose only effect is imported delegation; finite path `map`/`flatMap` command-argument construction;
- dynamic imports whose module is a literal path inside a test owner.

The production recognizer uses an owned token, statement, and expression parser with parent-linked lexical scopes. It distinguishes runtime value imports from type-only clauses and specifiers, records original exported names through aliases, and creates scopes for classes, methods, blocks, loops, and closures. Ambient `console` and `process` are trusted only while no local binding shadows the intrinsic. Mutable bindings, nested functions/classes, non-routing callbacks, reducers, filters, accumulator mutation, filesystem reads, raw `Bun.spawnSync`, computed dynamic-import modules, computed environment values, and arithmetic or domain work inside status/path expressions are rejected. Every accepted call argument, condition, member expression, computed index/key, template interpolation, closure capture, parameter/default, and loop iterable is checked against that scope and provenance. Opaque results from a runtime imported delegate may be passed unchanged to another imported owner, including through an immutable receipt or delegation-only closure; harmless indexing such as `args[args.length]` remains valid.

The installed TypeScript oracle is structurally independent of the owned parser. It walks the compiler AST with its own scope and binding model and compiles the scope vectors together in one semantic program. Type-only runtime uses produce TypeScript diagnostic 1361; matching value imports compile without it.

## Corrective Scope And Fallback Evidence

An earlier revision of this report claimed recursive binding closure from a 91-test, 326-assertion run. That proof was insufficient: the recognizer still pooled identifiers across scopes, trusted intrinsic names after shadowing, and used regex effect guards, while the original TypeScript oracle did not validate several argument positions. The 91/326 result is historical and is superseded by the evidence below.

The portable corpus now includes 20 paired scope vectors covering runtime and type-only delegates, terminal functions, `BundleScript`, aliases, ambient and parameter/block-shadowed `console` and `process`, opaque delegate composition, immutable receipt forwarding, closure capture and parameter shadowing, and harmless computed indexing. Four type-only runtime forms are rejected by both classifiers and independently produce TS1361. Identical `log`/`exit` receiver shapes remain rejected when the receiver is a parameter or local binding rather than the unshadowed intrinsic.

The nine concrete argument adversaries and twelve fresh hostile variants are checked by both parsers. The end-to-end disk discovery regression additionally writes a package `📜️script.ts` three ways: a type-only terminal import, a type-only ordinary `execute()` delegate, and a runtime `runArtifactRustPackageMain` delegate. The first two remain `package-role-unresolved` even though the generic glue classifier would call them `thin-delegation`; only the runtime value import is admitted. `collectPackageRoles` therefore retains a specialized fixed/configurable disposition rejection unless generic classification independently proves an implementation body.

The regression corpus preserves the seven originally required hostile families: Python import plus hidden definitions, Vitest configuration plus hidden class, command script plus hidden class, unknown Rust registration computation, C arithmetic macro, known Rust registration plus hidden executable call, and C domain-call macro. It also preserves valid multiline configuration arrows and adds paired hostile cases for every newly admitted router form.

The argument-closure group adds one valid imported delegate, the nine independently observed false-admission sources from `📓️router-argument-adversaries-2026-09-12.md`, and twelve fresh hostile variants. These cover an imported delegate argument effect, console/member effects, array/object initializer effects, condition effects, unproven imported iteration, terminal and registration argument effects, closure-captured effects, nested member calls, arithmetic arguments, computed indexes and object keys, template interpolation, parameter and destructuring defaults, decorators, shadowed imports, and local aliases of imported iterables. Every hostile source is valid TypeScript and must be `unresolved` in both implementations.

## Manifestless Package Discovery

The portable fixture contains the same TypeScript domain body once with `package.json` and once without it; both must produce `package-implementation`. A declaration-only source without a manifest remains accepted as body glue. The direct live disk probe found the expected `package-implementation` diagnostic for each previously skipped body:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🐍️python/🐍️.py` — Python function or class declaration.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/📚️library/📦️packages/🟦️typescript/🟦️.ts` — runtime type or namespace declaration.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` — domain type declaration owned inside the package.

The earlier pass of `discoverCatalogPackages(process.cwd(), loadCatalogTaxonomy())` returned 222 packages with 222 unique package roots. The final correction pass, after concurrent package additions elsewhere in the shared checkout, returned 223 packages with 223 unique package roots. The catalog code path intentionally skips disk body inspection because its registry view is a declared input catalog; this proof is separate from the live disk classifier proof.

## Scope Boundary And Follow-Up

The 89-source decision map is the package-oriented candidate set assigned by this lane. The broader no-follow scan retained in `📓️script-ownership-coverage-expansion-2026-09-12.md` contains 455 mandatory scripts, including 318 paths outside the earlier package-oriented census. This change does not establish body ownership for those additional scripts.

The executed probes in `📓️nonpackage-script-enforcement-probe-2026-09-12.md` and `📓️fixed-script-enforcement-closure-packet-2026-09-12.md` establish a separate enforcement gap for fixed scripts both outside and inside packages. A mandatory `📜️script.ts` can receive the `root-script` fixed-filename contract while the general inventory path returns `not-package` or `configuration` before applying its source disposition. Direct source-disposition classification rejects the observed sources, yet scoped inventory reports no body-ownership diagnostic. A follow-up must apply semantic fixed-source disposition independently of package-role classification without weakening filename authority or requiring package metadata. This correction changes the package-role collection fallback only; it does not claim coverage of the broader 455-script inventory.

## Exact 89-Source Decision Map

### Admitted closed routers

| Source | Lines | Decision |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📜️script.ts` | 23 | `tool-metadata` |
| `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts` | 36 | `tool-metadata` |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts` | 36 | `tool-metadata` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📜️script.ts` | 37 | `tool-metadata` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts` | 35 | `tool-metadata` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts` | 28 | `tool-metadata` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/📜️script.ts` | 17 | `tool-metadata` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts` | 39 | `tool-metadata` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📜️script.ts` | 26 | `tool-metadata` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🐹️go/📜️script.ts` | 17 | `tool-metadata` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📜️script.ts` | 58 | `tool-metadata` |
| `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🔱️trinity/📦️packages/🟦️typescript/📜️script.ts` | 16 | `tool-metadata` |
| `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/📜️script.ts` | 35 | `tool-metadata` |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🖨️raster/📦️packages/🟦️typescript/📜️script.ts` | 19 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts` | 27 | `tool-metadata` |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts` | 19 | `tool-metadata` |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/✒️writer/📦️packages/🟦️typescript/📜️script.ts` | 12 | `tool-metadata` |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/🌿️vcs/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🟦️typescript/📜️script.ts` | 12 | `tool-metadata` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🦀️rust/📜️script.ts` | 5 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🟦️typescript/📜️script.ts` | 12 | `tool-metadata` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/📋️forms/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🏛️architect/📦️packages/🟦️typescript/📜️script.ts` | 12 | `tool-metadata` |
| `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/📏️layout/📦️packages/🟦️typescript/📜️script.ts` | 12 | `tool-metadata` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📜️script.ts` | 24 | `tool-metadata` |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🟦️typescript/📜️script.ts` | 8 | `tool-metadata` |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` | 26 | `tool-metadata` |
| `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `✏️s/🔌️plugins/🔋️energy/📦️packages/🦀️rust/📜️script.ts` | 25 | `tool-metadata` |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📜️script.ts` | 24 | `tool-metadata` |

### Retained unresolved source reviews

| Source | Lines | Reason and assignment |
| --- | ---: | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts` | 57 | Consumes imported receipts and performs a local arithmetic `reduce` for status output; assigned to the UI runtime source-extraction follow-up. |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts` | 64 | Builds a local forbidden-dependency table, mutates violation state, and evaluates branch-audit policy; assigned to the UI render verification/source-extraction follow-up. |
| `🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts` | 46 | Defines a nested Wasm `validate(files)` callback; assigned to the scale fixture verification-owner follow-up. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/📜️script.ts` | 29 | Reads directories and filters/maps artifact staging inputs; assigned to the repository test-support source-extraction follow-up. |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/📜️script.ts` | 25 | Calls raw `Bun.spawnSync`; assigned to the Jack shell task-runner source-extraction follow-up. |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | 22 | Computes an environment value with `String(128 * 1024 * 1024)`; assigned to the Puzzle 3D task-configuration follow-up. |

## DSL Fixture-Sweep Separation

The report contract has one valid portable report and six hostile reports: missing summary, duplicate summary, empty fleet, wrong registered-app count, empty asset coverage, and wrong native assertion count. The verification owner parses the exact native stdout/stderr receipts; the package command leaf only selects the law group, invokes `runExactCargoLaws`, and forwards receipts.

The registered native `source-check` currently stops after the portable contract at `ENOENT` for `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🦀️.rs`. The active Cargo manifest instead points at `../../🧪️tests/🧹️fixture-sweep/🦀️.rs`, whose current 120-line M5 mount test does not define the expected `repo_wide_dsl_fixture_law_sweep` or `repo_wide_semio_example_kind_coverage` laws. Coordinator probe `🗑️generated/coordinator/dsl-fixture-source-provenance.json` establishes that the current extraction fixture and Rust source are byte-identical to HEAD (source SHA-256 `ac94e9a722e069e5042c9dae4b51990926a6ffb1d9b21517c352064f1db6f7ed`; fixture SHA-256 `4d883250f32c5fd3e0b25189d6b7a43f67dff48829080aa34ff64278d3167d37`) and that neither law exists in active Rust. This is a retained source/fixture-contract inconsistency; no law semantics were reconstructed.

## Changed Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`: owned lexical router parser, scoped binding/effect proof, fixed-disposition fallback closure, and manifest-independent disk body inspection.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json`: portable router, adversarial argument, lexical-scope/type-only, and manifest-presence vectors.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/📦️package-boundary-classification/🔣️.json`: portable vector schema including the declared TypeScript semantic diagnostic set.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`: independent installed TypeScript AST and semantic compiler oracles, paired manifest fixture, actual disk fallback proof, and live route coverage.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📦️packages/🦀️rust/📜️script.ts`: retained task router delegating verification.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts`: neutral native receipt/report verification owner.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🧫️fixtures/🔣️.json`: portable valid/hostile report vectors.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🧬️schema/🔣️.json`: portable report schema.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-script-router-policy-2026-09-12.md`: retained this exact map and evidence.

## Verification

- Fresh live 89-source classification: **83 `tool-metadata`, 6 `unresolved`**.
- Direct portable/native body-policy suite after lexical correction: **113 tests, 431 assertions, 0 failures** in 12.87 seconds.
- Registered isolated Nx route with cache disabled: `@semio-tech/repo-lib:test-package-body-policy` passed **113 tests, 431 assertions, 0 failures**; Bun reported 5.29 seconds, and Nx reported 5.5 seconds total with a 5.4-second critical path.
- Independent TypeScript semantic oracle: all 20 paired scope vectors matched their declared diagnostic sets; all four type-only runtime uses produced TS1361 and all runtime value-import controls produced no semantic diagnostic.
- End-to-end package discovery fallback: type-only terminal and ordinary delegate forms both produced `package-role-unresolved`; the runtime `runArtifactRustPackageMain` wrapper produced no package problem.
- Manifestless live disk proof: all three exact sources produced `package-implementation`; catalog discovery remained 222/222 unique.
- Final strict shared authority: `loadTaxonomy()` succeeded and `validateTaxonomy()` returned zero diagnostics; final catalog discovery returned 223/223 unique package roots after concurrent package additions.
- DSL portable report contract: **1 valid and 6 hostile** vectors behaved as required.
- Native Bun parser/bundler oracle: the package router bundled **304 modules / 11.20 MB** in 259 ms; the neutral verification owner bundled **92 modules / 0.27 MB** in 53 ms.
- Registered isolated `@semio-tech/dsl-fixture-sweep-rs:source-check`: failed at the documented missing Rust source before native laws ran; Nx reported 1.2 seconds.

No blanket filename, class, package path, missing-manifest, or generated-directory exception was added. The six scripts remain current unresolved reviews with named follow-up assignments; this classification alone does not establish that each is a source defect. The three newly visible manifestless implementations remain explicit source-extraction work. The 455-source expanded census and fixed-script source-disposition gap remain separate bounded follow-ups.
