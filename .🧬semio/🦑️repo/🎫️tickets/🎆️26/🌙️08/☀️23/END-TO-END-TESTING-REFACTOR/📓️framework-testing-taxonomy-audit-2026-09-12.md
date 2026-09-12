# Framework Testing Taxonomy Audit

Date: 2026-09-12  
Scope: read-only inventory of authored testing taxonomy in `🧰️framework`, excluding `node_modules`, Git internals, and generated package trees. This report does not change source or run tests.

## Required target taxonomy

The target has four testing categories only:

- `🧪️tests`: each executable test lives at `<semantic-owner>/🧪️tests/<test-name>/<implementation>`.
- `🧫️fixtures`: testing-only static vectors/examples live at the semantic owner, outside case folders.
- `📚️examples`: product or user-facing examples. A test-only example belongs in `🧫️fixtures`, not here.
- `🔮️oracle`: a real independent reference/comparator contribution only. Helpers, fakes, harnesses, and shared assertions are not oracles.

Shared executable testing infrastructure must be a semantic module of the repository test owner (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`), with an explicitly test-only exposure. Case-specific executable support belongs beside the consuming case implementation.

## Inventory summary

There are **15 authored `testkit` roots**, with **239 files / 3,278,714 bytes**. Two are core framework roots and thirteen are under the OS product. Four conventional Rust `tests/` directories and the legacy mirror-fixture root are empty, so file-driven scanners currently miss them.

| Scope | Roots | Files | Bytes | Classification |
| --- | ---: | ---: | ---: | --- |
| Core framework | 2 | 2 | 8,637 | reusable testing algorithms/oracles; not fixtures |
| OS product | 13 | 237 | 3,270,077 | mixture of fixture corpora, case code, shared harnesses, packages, generated artifacts |
| Conventional Rust `tests/` | 4 | 0 | 0 | empty prohibited legacy roots |
| `🪞️fixtures` | 1 root, 3 empty children | 0 | 0 | empty prohibited legacy root |

## Exact noncanonical roots and intended scopes

| Existing root | Contents observed | Consumers observed | Correct destination/classification |
| --- | --- | --- | --- |
| `🧰️framework/🔨️modules/🎒️pack/🧨️testkit` | 1 Rust file, 5,254 B: generic truncation/bit-flip corruption-fuzz algorithm and report types | `📦️packages/🦀️rust/🦀️.rs:38-39` unconditionally mounts `pub mod testkit`; OS pack re-exports it | Shared executable test infrastructure. Put it in a semantic repository-test module and expose it only to tests/test support; it is neither a fixture nor an oracle. |
| `🧰️framework/🔨️modules/🧬️schema/🧪️testkit` | 1 TypeScript file, 3,383 B: Ajv independent document-contract comparator | 11 current plugin test imports per the owner audit | A genuine executable comparator helper. Put it in the schema owner's oracle test implementation or a semantic repository-test oracle module, with test-only dependency exposure. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️testkit` | 4 files, 4,279 B: one authored mutation vocabulary (`🦀️.rs`, descriptor, schema) | `🧪️tests/🧬️job-test-mutations/🦀️.rs:39`; `🧪️tests/🧬️job-test-mutations-mutations-add-value-unit/🦀️.rs:16` | Move static vocabulary to the mutation-plan owner's `🧫️fixtures`; co-locate any executable adapter with the consuming test case. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️testkit` | 4 files, 5,933 B: dependency-contribution mutation vocabulary | `🧪️tests/🔗️dependency-contribution/🦀️.rs:42`; `🧪️tests/➕️dependency-contribution-add-value-unit/🦀️.rs:3`; builder admission tests | Move descriptors/schema/source vectors to builder `🧫️fixtures`; keep only case implementation under each `🧪️tests/<case>`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testkit` | 53 files, 65,467 B: 28 JSON and 25 Rust mutation/test-app/publication corpora | Numerous `🔌️plugin/🧪️tests/**` cases consume `crate::app::testkit` and its mutation vectors | Split: static mutation/test-app data to plugin `🧫️fixtures`; reusable executable app harness to a named semantic module under the repository test owner. Do not retain it as a plugin child named testkit. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testkit` | 43 files, 1,721,333 B: actor-import test project, 19 TS, 9 JSON, 7 WASM, 3 JS, test outputs and script/project configuration | Browser-bundle tests and its own `📋️project.json` / `📜️script.ts` | Split the actor-import case implementation/configuration into `🧪️tests/<case>`; put expected bundle/WASM/output inputs in browser-bundle `🧫️fixtures`. It cannot remain a nested test project category. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️testkit` | 1 Rust file, 1,921 B: owner/test helper | `🧪️tests/🔁️document-replacement/🦀️.rs:38`; `🧪️tests/🪟️retained-window-input/🦀️.rs:46`; plugin retirement test | Case-specific helper: co-locate under the affected test case(s), or move only genuinely shared executable logic to a semantic repository-test module. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit` | 2 files, 40,076 B: compatibility re-export plus nested unit tests | OS Rust barrel `📦️packages/🦀️rust/🦀️.rs:150-151` mounts it publicly | Remove compatibility layer. Move the nested test into canonical owner tests and consume core shared fuzz support through the test-only semantic module. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit` | 2 files, 78,475 B: fault storage, deterministic simulation, workload/law helpers, plus nested tests | Package barrel `📦️packages/🦀️rust/🦀️.rs:83-84`; DB engine/WAL/artifact tests; **production** `🗄️storage/🦀️.rs` declares `Fault(Box<db_testkit::FaultStorage>)` and related references | High-risk split. Move fault/simulation test infrastructure to a semantic repository-test DB module, make DB test imports test-only, and remove production storage types' direct dependency on `db_testkit`. Migrate nested unit tests to `🛢️db/🧪️tests`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit` | 27 files, 122,859 B: Rust law helpers, 12 JSON vectors, nested `🧪️tests`, `🧩️support`, `🧫️fixtures`, and `🏃️benches` | OS barrel `📦️packages/🦀️rust/🦀️.rs:203-204`; store tests call `crate::os_spr::testkit::*`; nested SPR tests | Split all four concerns: vectors to SPR `🧫️fixtures`; executable assertions/law helpers to repository-test semantic module; nested tests to SPR owner `🧪️tests`; benchmark executable as a named test case implementation. The nested `🧩️support` is prohibited. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🧪️testkit` | 20 files, 22,386 B: registry and mutation-law descriptors/Rust sources | Command registry and mutation-law tests use `#[path]` and `include_str!` from this root | Move static mutation trees to command owner `🧫️fixtures`; place any module source beside the specific test implementation. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testkit` | 48 files, 41,597 B: 28 JSON and 20 Rust mutation vocabulary leaves for demo, timestamped, severity, validated, lossy cases | 16+ store fixture test cases use `include_str!("../../🧪️testkit/…")`; `🏪️store/🦀️.rs` participates in testkit wiring | Move the full static corpus to store `🧫️fixtures`; update every `include_str!` test consumer. No runtime/store source may depend on that fixture corpus. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/♻️retirement/🧪️testkit` | 4 files, 2,944 B: mutation vocabulary fixtures | `…/♻️retirement/🧪️tests/🧪️fixture-mutations-set-value/🦀️.rs` | Move to retirement owner `🧫️fixtures`; keep adapter in the consuming case. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️testkit` | 1 Rust file, 27,185 B: reusable note/CAD source builders and evaluation cases | MCP `📦️packages/🦀️rust/🦀️.rs:52-53` mounts it; `🦀️.rs:21` re-exports it; quick tests under conformance, context, catalog, search, workspace, dispatch consume `crate::testkit` | Shared executable test infrastructure. Extract into a semantic repository-test MCP module and make imports test-only. Put any fixed source vectors in MCP `🧫️fixtures`. |
| `🧰️framework/🛍️products/💻️os/🧪️testkit` | 28 files, 1,135,622 B: `⚖️scale` test package/profile plus `🧩️jcoprobe` guest package, browser harness, vendored shim and build data | `🧪️tests/🧩️jcoprobe-callback/🟦️.ts`; OS Rust package/scripts; OS schema; repo taxonomy, discovery, projection asset, and workspace-contract test all hard-code this path | Split by behaviour: JCO probe sources/harness belong beside its canonical OS test case; produced browser/WASM data belongs in OS `🧫️fixtures/🧩️jcoprobe`; scale test code belongs in its own OS test case. Move the vendored preview2 shim with the JCO test implementation and update every metadata path. |

## Other prohibited authored layouts

### Empty legacy roots

These are empty but authored and prohibited. They are invisible to current file-only scanning and should be removed, not retained as compatibility locations.

- `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/tests`
- `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/tests`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/tests`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/tests`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures` and its empty `🫴️owners`, `🔮️oracle`, and `🔄️lifecycle` children.

### Helper/harness categories

These contain executable support rather than test cases or fixtures. They must not be relabelled as oracles.

| Existing path | Actual purpose | Direct consumers / destination |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🔄️machine/🧪️tests/🔬️testing-support/🦀️.rs` | Unit toggle machine model/support | `🔬️persist-unit`, `🔬️testing-unit`, `🔬️runtime-unit`, `🔬️inspect-unit` import `testing::support`. Co-locate the model with an owning test case or make it a semantic repository-test machine module. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️test-support/🦀️.rs` | `FakeTransport` / `FakeWs` network stand-in | `🧪️tests/🔬️unit/🦀️.rs` imports it extensively; the production client has a doc reference. Co-locate with `🔬️unit` or use a named repository-test transport module. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️test-support/🦀️.rs` | reactor state/test drivers | `🚪️lifetime/🧪️tests/🧵️runtime/🦀️.rs` calls it. Co-locate with that case or extract a semantic reactor test module. |
| `…/💻️os/🧪️testkit/🧩️jcoprobe/🌐️harness/🧩️support` | browser test harness and vendored preview2 shim | JCO callback test and metadata rules. Move with the JCO case implementation; do not retain `harness`/`support` categories. |
| `…/💻️os/🔨️modules/📡️spr/🧪️testkit/🧩️support` | mutation-law support source/data | Split executable support to semantic test module and data to SPR fixtures. |

Names such as `🧪️tests/🔬️mock-guest-runtime`, `🧪️tests/🧪️multi-shell-harness`, and test names containing `fixture` are not separately classified as directory categories when they are a single behaviour case with its implementation.

## Examples and oracles: scope findings

### Mis-scoped testing-only examples

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📚️examples/🧪️conformance` is a **147-file / 111,991-byte test corpus**, not a product example. It is consumed only in test-gated paths:

- `🧬️contract/📦️packages/🦀️rust/🦀️.rs:33-35` mounts the corpus under `#[cfg(test)]`.
- `…/📜️script.ts:50-52`, `🧬️schema/🦀️.rs:59-61`, and both local conformance test cases load it.
- `💻️os/…/🗣️Interpreter/🧪️tests/🧪️unknown-component-placeholder/🟦️tsx:92-109` consumes the same cross-module corpus.

Move it to `🧬️contract/🧫️fixtures/🧪️conformance` and update those six direct consumers. The other discovered `📚️examples` roots are product/DSL example data or empty; no evidence currently shows they are testing-only:

- `…/🌊️flow/📚️examples`
- `…/🪐️space/📚️examples`
- `…/🪐️space/🗿️artifacts/{🪐️space,🗂️collection}/📚️examples`
- `…/🗣️dsl/📖️grammar/📚️examples` (empty)
- `…/💻️os/📚️examples`

### Genuine oracle contributions

The following `🔮️oracle/🔣️.json` files describe independent references or explicit no-oracle decisions and should remain an oracle category:

- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🔮️oracle` — `mercantile` reference.
- `🧰️framework/🔨️modules/🎠️kernel/🔮️oracle` — `semver` reference.
- `🧰️framework/🔨️modules/🖱️ui/🔮️oracle` — `clsx` and `class-variance-authority` references.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔮️oracle` — documented no-external-oracle decision.

`🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json` is a genuine test contribution manifest, but it uses a stale directory name. Move it to the configured canonical `🔮️oracle`; its direct feature/test references are under plugin host mutation tests. The test-domain fixture `🧭️contribution-directory-ownership` intentionally contains `🧪️oracle` strings as regression input; preserve its test purpose while updating the expected canonical policy.

## Consumer and enforcement gaps

### Runtime / package exposure

1. Core pack exposes `pub mod testkit` from its normal Rust barrel.
2. OS pack and SPR barrels mount public `testkit` modules in `💻️os/📦️packages/🦀️rust/🦀️.rs:150-151` and `:203-204`.
3. DB mounts `pub mod db_testkit` in its normal package barrel; `🗄️storage/🦀️.rs` has actual production-type references to it. This is the clearest production dependency on testing infrastructure and must be broken architecturally.
4. MCP mounts `pub(crate) mod testkit` in its package and re-exports it in `🌉️mcp/🦀️.rs:21`.
5. OS Cargo declares a `testkit = []` feature, while DB requests testkit features in normal package manifests. The refactor must use explicit test/dev-only wiring rather than a renamed production feature.

### Metadata and generator consumers

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` actively permits both `🧨️testkit` and `🧪️testkit` as module member names and permits package role `testkit`. It also pins JCO harness vendored-file rules and generated-package paths under the OS testkit root. `📚️library/🔍️discovery/🟦️.ts` hard-codes the JCO testkit semantic owner. `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json`, `💻️os/🧬️schema/🔣️.json`, and `📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts` also contain exact JCO testkit paths. All require an atomic path update.

### Policy implementation gaps

The repository test implementation already has useful partial checks:

- `🧪️test/📦️packages/🟦️typescript/🟦️.ts:1698-1718` validates canonical test depth and flags named legacy fixture directories.
- `:2101-2105` flags configured legacy `test`, `tests`, and `__tests__` directories.
- `:1869-1943` detects canonical-fixture imports from production sources.

It does not enforce the requested result:

1. The scanner is file-driven, so the four empty `tests/` roots and empty `🪞️fixtures` root are not reported.
2. `testkit`, `harness`, `support`, `helper`, mock/stub/fake categories, and nested test roots are not taxonomy-forbidden segments. They can pass as ordinary production-like directories.
3. `SEMANTIC_NON_PRODUCTION_SEGMENTS` in `📚️library/🔍️discovery/🟦️.ts:10814` lists tests/examples/fixtures but omits testkit and oracle, allowing testkit packages and dependencies to be treated as normal production structures.
4. The production-fixture-dependency check only recognises canonical `🧫️fixtures`; it will not catch a production import of a `testkit` corpus.
5. `testFixtureLegacyDirectoryNames` still includes `🪞️fixtures`, while the broad taxonomy member registry admits it. Legacy names must be rejected by directory existence, not merely recognised when a source file is traversed.
6. The canonical configured oracle contribution directory is `🔮️oracle`, yet current fixture/test vectors and OS config still model `🧪️oracle`; this creates contradictory policy and fixtures.

## Execution order

1. Establish the directory-level taxonomy prohibition and test-only dependency checks first. Include empty-directory detection and a test that rejects every legacy helper/testkit name.
2. Extract shared executable algorithms (pack fuzzing, schema comparator, DB/SRP/MCP/plugin harnesses) into semantic repository-test modules with test-only package wiring. Remove all normal-barrel `testkit` exports.
3. Relocate static corpora to their nearest semantic-owner `🧫️fixtures`, then update `include_str!`, `#[path]`, script, project, Cargo, and metadata consumers together.
4. Move each nested executable test or harness into its canonical test case; delete empty legacy roots rather than preserving adapters.
5. Rename config's `🧪️oracle` contribution to `🔮️oracle` and update all intentional regression vectors to assert the new policy.
6. Run the repository test-layout / workspace-contract checks, then the affected Rust/TypeScript test targets. No execution was performed for this read-only audit.

## Unresolved items for implementation audit

- The OS plugin root testkit has many shared helper entry points. Its exact semantic module partition must be drawn from the import graph before edits; it should not be duplicated mechanically into every case.
- Browser-bundle's 1.64 MB testkit contains compiled artifacts. Confirm which are reproducible test outputs versus committed input fixtures before moving them; either way they must be below the owner fixture/test case, never under testkit.
- DB `FaultStorage` currently appears in production storage enum/type definitions. Determine whether the production abstraction should become an injected generic test double or whether the fault type can be completely test-gated; a mere file move will not sever the dependency.
- JCO source/harness and existing `🧫️fixtures/🧩️jcoprobe` outputs must be reconciled as one test case so no copy or alternate fixture tree remains.
