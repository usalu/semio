# Other Testing Taxonomy Audit — 2026-09-12

## Scope and method

This is a read-only inventory for the end-to-end testing refactor. It covers authored `✏️s`, `🌎️hub`, and root testing/tooling content, and excludes `🧰️framework`, dependencies, build products, repository-ticket output, and `♻️mit-bestand` from change candidates. The legacy tree was sampled only to distinguish it from authored scope.

The inventory used `rg --files` for the authored file index, then enumerated directory roots and read representative Rust and JSON consumers. It made no source edits and ran no tests.

The target convention used for classification is:

```text
<semantic-owner>/🧪️tests/<test-name>/<implementation>
<semantic-owner>/🧫️fixtures/<fixture-name>/<payload>
<semantic-owner>/📚️examples/<example-name>/<implementation-or-payload>
<semantic-owner>/🔮️oracles/<oracle-name>/<implementation-or-reference-payload>
```

`🖼️assets` owns production static data. A directory is not treated as a test category merely because its name contains a testing word; its owner, contents, and consumers decide that classification.

## Inventory at a glance

| Root or family | Authored count | Contents and consumers | Classification |
| --- | ---: | --- | --- |
| `🧪️tests` | 5,264 roots: 5,231 plugins, 32 hub, 1 root | Test-case roots and implementations | Canonical collection |
| `🧫️fixtures` | 409 roots: 388 plugins, 20 hub, 1 root | Serialized documents, mutation inputs, expected projections, static binary/text samples | Canonical collection; payload shape still needs targeted normalization |
| `📚️examples` | 179 plugin roots | 1,333 descendant files: 420 Rust, 342 TypeScript, 274 `.semio`, plus media/data. 210 nested test roots validate examples. | Canonical collection, but not an opaque data exemption: executable examples need owner-level review |
| `🔮️oracle` | 198 plugin roots | 322 files: 196 JSON, 125 Rust, 1 Python. 59 nested test roots exercise oracle implementations. | Noncanonical singular alias; structurally migrate to plural `🔮️oracles` after consumer rewrites |
| `🧪️oracle` | 20 roots: 19 plugins, 1 hub | 41 files: registry JSON, Rust oracle crate/modules, tests, and package metadata | Noncanonical icon/name alias for the same semantic role; migrate to `🔮️oracles`, not under `🧪️tests` |
| Helper/support families below | 79 named roots | See the next section | Noncanonical testing categories except the one production conformance module noted below |
| `📸️snapshot` | 3,081 plugin roots | 2,485 under fixtures, 387 under tests, 209 in domain source | Do not bulk rename: most are domain snapshot payload or domain implementation, not a parallel test collection |
| `🧱️baseline` | 4 plugin roots | Standard/subset names and baseline mutation cases | Needs individual semantic review; not evidence of a generic testing category |
| `golden` | 0 authored roots | 2 roots only in `♻️mit-bestand` | Legacy-only; no authored migration target |

The previously observed 412 fixture, 184 example, 223 singular-oracle, 3,082 snapshot, 5 baseline, and 2 golden totals include legacy or excluded roots. The table isolates authored scope.

## Test helper and support roots

There are 79 named candidate roots. Seventy-eight are test-only by placement or consumer; one is production code whose name happens to contain `support`.

| Exact root name | Roots | Scope partition | Actual contents and consumer type | Action classification |
| --- | ---: | --- | --- | --- |
| `🔬️testkit` | 56 | All are immediate children of `🧪️tests`; 30 plugins. Partition: `📕️norm` 15; `🧩️puzzle` 5; `🌀️procedural` 3; `🧱️block` 3; `🌍️gis`, `🏗️fem`, `📐️cad`, `🪐️space` 2 each; 20 plugins 1 each. | Rust setup/dispatch/render helpers shared by adjacent test cases. Representative [`writer testkit`](../../../../../../../../../../✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs) constructs a real app, loads a document pack, dispatches commands, drains operations, and renders views. | Test-only support category; dissolve into the semantic test implementations that use it, splitting shared setup by actual case ownership. |
| `🔬️test-support` | 2 | `🌀️procedural` editor tests; `🖨️raster` binary-mutation tests | The procedural root serializes a global evaluator cache. The raster root is also test-only support. | Test-only support; put the small support implementation with the actual consuming case(s). |
| `🏗️test-support` | 1 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog` | Feature-gated builder creates a synthetic catalog bundle consumed by hub binary-target and integration tests; it writes synthetic files and calls the real loader. | Test-only support despite cross-target compilation; move under the semantic hub test owner, preserving the feature boundary. |
| `🔬️testing-support` | 1 | `🖍️draw` canvas-pointer FSM tests | Test-only helper next to an FSM test suite. | Test-only support; dissolve into the case implementation. |
| `🔬️compliance-helpers` | 15 | One per `📕️norm` standard artifact, all immediately under schema tests | The files contain actual `async_test` assertions and numeric compliance cases, rather than reusable helper-only APIs. | Misnamed test suite, not a shared support library. Rename/re-home as semantic test cases while preserving each standard owner. |
| `🔬️document-helpers` | 2 | `🕸️dag` and `📖️playbook` schema tests | Test-only document-construction helpers interleaved with test assertions. | Split case-local construction from assertions; do not retain a helper category. |
| `🔬️testgen` | 1 | WFC oracle tests in `🌀️procedural` assembly | Builds tiny compiled models, arcs, domains, and random graphs for oracle tests. | Test-only generator; distribute to the named oracle test cases or retain as private functions in their implementation file. |
| `🏅️conformance-support` | 1 | `🗄️stdio` PDF 1.7 base schema | Pure object-graph edit primitives reused by six production conformance subsets; not nested under tests and not a test dependency. | Production domain module. Do not move it as part of the testing taxonomy work; a separate domain naming decision may rename it. |

The `testkit` partition is exact: `✒️writer`, `➗️mathematical`, `🌊️flow`, `🌿️vcs`, `🎞️animate`, `🎥️shooting`, `🎪️demonstrator`, `🎬️sequence`, `🏛️architect`, `🏭️process`, `💠️lowpoly`, `💡️reasoning`, `📋️forms`, `📏️layout`, `📖️playbook`, `📜️imperative`, `📸️remodel`, `🕸️dag`, `🖍️draw`, `🖨️raster`, `🗒️note`, and `🪵️sourcing` each have one root; the multi-root plugins are enumerated in the table above.

## Oracle aliases and consumers

The two aliases are semantically one reference/comparator system, not test-suite folders.

- The 198 `🔮️oracle` roots mirror artifact/standard/subset ownership. Their JSON records reference decisions or comparison profiles; their Rust files implement independent readers, writers, semantic projections, and comparators. These are therefore legitimate oracle contents, but the singular collection spelling is noncanonical.
- The 20 `🧪️oracle` roots are also oracle owners. The key example is [`🗄️stdio/🧪️oracle`](../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🧪️oracle): its Rust crate documents itself as reference implementations, is enabled only by the `oracles` feature, and re-exports format-specific oracle modules. Its `🔣️.json` registry is consumed by per-artifact test manifests through `oracles`, `comparisonProfiles`, and `oracleHostPackages` entries. Other `🧪️oracle` JSON manifests similarly declare no-oracle decisions or host-package wiring.
- This makes `🧪️oracle` a structural alias, not a test name. The canonical destination is `🔮️oracles` beside the relevant semantic owner. Moving it below `🧪️tests` would break its registry and feature consumers.
- Five further oracle-containing names require individual classification before a broad rename: `⚖️oracle` (two architect domain roots), `🔮️protocol-oracle` (a sequence test case), `🔬️oracle-standalone`, `🔬️oracle-unit`, and `🔬️oracles-unit` (test-case names), plus four `🦀️oracle-probe` developer probes. Their names alone do not establish a collection role.

## Examples, fixtures, snapshots, and baseline data

Examples and fixtures are not empty payload buckets.

- `📚️examples` includes executable Rust and TypeScript as well as `.semio` and media assets. It also contains 210 nested `🧪️tests` roots, normally a test named `🧩️example` that validates a specific example. An example with executable behavior should remain under its semantic owner only when it is an actual example; testing-only sample input belongs in that owner’s `🧫️fixtures` outside the test-case folder. Production static media belongs in `🖼️assets`.
- `🧫️fixtures` primarily holds mutation scenario inputs and expected retained document state. The 2,485 `📸️snapshot` roots inside fixtures are not automatically a fifth test collection: their surrounding paths identify a named fixture/mutation scenario and the snapshot is the event-sourced document payload for that scenario. Their placement is intentional test data, but a later move must preserve owner and scenario identity instead of flattening every snapshot.
- The 387 snapshots under tests are usually the semantic `snapshot` component being tested; the 209 outside all testing collections are domain snapshot implementation or I/O state. Neither group can be mechanically reclassified as fixtures.
- `🧱️baseline` occurs in four authored standard/subset owners, including JPEG and TIFF mutation fixtures. Its names describe a standard baseline/subset and mutation case, not an established generic expected-output collection. Review them with the owning standard before renaming. `golden` exists only in excluded legacy research fixtures.
- Nine `🧫️fixtures` roots sit below `🧪️tests`; they are suspected local test-data aliases. They need case-by-case relocation to the owning `🧫️fixtures` root, after checking whether they are data payloads or a test name that merely contains “fixture”.

## Handoff order and unresolved checks

1. Canonicalize `🔮️oracle` and `🧪️oracle` to `🔮️oracles`, updating registry paths, Rust module paths, feature wiring, and manifest consumers together.
2. Remove the 78 test-only `testkit`/support/helper/testgen category roots by moving code to the named consuming test case implementations. Do not move `🏅️conformance-support` in that change.
3. Review the 179 example roots by consumer: executable demonstration code may remain an example; test-only inputs move to owner fixtures; production static media moves to assets.
4. Review the nine fixtures nested below tests and the artifact-specific snapshot/baseline roots without flattening domain snapshot or standards vocabulary.
5. Keep `♻️mit-bestand`, external/dependency trees, generated `test-results`, and `🧰️framework` out of mechanical authored-tree moves until separately scoped.

No runtime behavior was asserted in this audit. The counts and classifications are filesystem and source-consumer evidence only.
