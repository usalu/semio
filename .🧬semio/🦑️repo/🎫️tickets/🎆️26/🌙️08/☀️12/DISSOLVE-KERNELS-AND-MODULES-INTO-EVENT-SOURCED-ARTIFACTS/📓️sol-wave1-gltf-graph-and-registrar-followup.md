# Sol Wave 1 glTF Graph and Registrar Follow-up

## Central Lease

The semantic graph now resolves relative Rust imports and TypeScript imports to individual terminal production components. Package glue is assembly only and cannot qualify as a production consumer. Module validation therefore uses reverse terminal closure rather than intermediary module or glue vertices.

The taxonomy recognizes `🚪️io/💡️inferences` as an I/O collection and rejects the former `🧬️schema/💡️inferences/{📝️text,💾️binary}` codec placement. Codec source leaves are not required inference members. A proposed disposition is no longer inferred from punctuation or conjunctions in a human-written responsibility; only graph-backed module rules determine it.

Changed central paths:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

The protected repository-library TypeScript package index was not edited.

## Registrar Work

The root Cargo registrar removed the retired `semio-s-mindmap` workspace member and workspace dependency. Cargo lock regeneration through the existing Nx target left no `semio-s-mindmap` or mindmap package path in `Cargo.lock`. The source owner then deleted the retired mindmap implementation while preserving its `AGENTS.md`.

The demonstrator registrar removes the requested stale topology, text shim, app, compatibility, and legacy mounts. Its canonical text and binary inference source leaves remain mounted exactly once. The local TypeScript package is an explicit empty production surface.

## glTF Census Result

`generate taxonomy census` after the glTF manifest correction proves the following module terminal closures and common owner:

| Module | Independent terminal production components | Lowest common owner | Disposition |
| --- | ---: | --- | --- |
| `vector-operations` | 5 | glTF `subsets/✳️any` | retain |
| `inference-measures` | 14 | glTF `subsets/✳️any` | retain |
| `mesh-topology` | 15 | glTF `subsets/✳️any` | retain |

The moved text and binary codec leaves are classified as I/O, not inferences. They are not subjected to the module consumer minimum.

The deterministic census path filter for the glTF owner is not release-clean. It has 84 structural findings: 37 `manifest-child-missing`, 34 `member-component-leaf-missing`, 8 `collection-authored-behavior`, and 5 `collection-manifest-missing`. The current `verify taxonomy report --scope s.stdio.gltf` command incorrectly omits these unregistered collection-path findings and prints zero errors; this is a central report-scope defect, not a clean owner. These are real owner-wide structural work; no baseline or path exception was added.

`🧬️schema/💡️inferences/🧾measure` is an additional semantic release blocker. Its source declares a shared inference vocabulary and contracts, not a derived result, and its resolved production consumers are the fourteen metric inferences, `geometric-analysis`, and `inference-measures`. It is currently typed as an inference, which is structurally accepted but semantically invalid. A new atomic glTF lease must promote that responsibility to a specifically named subset module, remove it from the inference collection manifest, and update its consumers and mounts. It must not be retained as a shared inference contract.

## Validation

Executed successfully:

```text
bun nx run @semio-tech/repo-lib:test-quick --skip-nx-cache -- --test-name-pattern 'semantic collection census'
```

Result: 8 passing tests, 21 assertions. The genuine-module fixture describes an `exact and stable` responsibility, proving that names or prose conjunctions cannot cause a split disposition.

Executed for deterministic ticket artifacts and scoped evidence:

```text
bun ./📜️script.ts generate taxonomy census --ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
bun ./📜️script.ts generate taxonomy duplicates --ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf
```

The census and duplicate artifacts are updated in this ticket. Enforce mode is correctly deferred: the active owner path has unresolved findings, regardless of the current scoped-report filtering defect.

`git diff --check` is clean for the central taxonomy, discovery, script, test, and Cargo registrar paths.

## Queued Measurement-contract Registrar

The glTF source owner promoted `🧾measure` to `ROOT/🔨️modules/🧾️measurement-contracts` with semantic ID `s.stdio.gltf.module.measurement-contracts` and sixteen direct production consumers. After its manifest/import pass, the central registrar removed the retired `schema::inferences::measure` mount and added exactly one `modules::measurement_contracts` Rust mount in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`. The stale-mount and duplicate-mount sweep is clean; the three existing glTF module mounts are unchanged.

`bun nx run @semio-tech/stdio-plugin:test-quick --skip-nx-cache` reached the shared Cargo build lock held by a concurrent workspace check before compiling this package. The queued local Cargo process was released without changing source; this is a validation contention, not a reported build or test result. The source owner has been notified to validate through the available Cargo lane.

A later captured rerun of the same target reached `cargo nextest run --no-tests warn --profile quick -p semio-s-plugin-stdio -- --skip long:: --skip exhaustive::`, then the package script terminated it at its configured 30,000 ms budget. The target therefore exits nonzero for `[budget] ... exceeded 30000ms — killed`; no compiler diagnostic or failing test assertion was emitted before the budget termination. This is recorded as an unresolved runtime-budget validation failure, not a passing test.

The post-registrar census resolves `measurement-contracts` to fifteen independent terminal inference consumers at the glTF subset owner. It correctly retains the module, but exposes three source-owner follow-ups: the manifest also declares intermediary `s.stdio.gltf.module.inference-measures` and must remove that non-terminal ID; the empty retired `🧬️schema/💡️inferences/🧾️measure` directory must be deleted. The latter causes one `manifest-child-missing` and one `member-component-leaf-missing`; the former causes one `module-consumer-graph-mismatch`. No central graph change is warranted.
