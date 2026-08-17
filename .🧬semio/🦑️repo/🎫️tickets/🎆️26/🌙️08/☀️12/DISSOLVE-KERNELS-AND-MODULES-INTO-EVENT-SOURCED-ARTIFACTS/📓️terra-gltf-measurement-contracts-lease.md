# Terra glTF Measurement Contracts Lease

## Baseline And Conflicts

- Owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any`.
- Applicable repository, `✏️s`, and `🗄️stdio` instructions were read. No deeper `AGENTS.md` exists below this owner.
- The parent ticket's central checkpoint is [Sol Wave 1 glTF Graph and Registrar Follow-up](./sol-wave1-gltf-graph-and-registrar-followup.md). It establishes terminal component referrers, subset-LCA module ownership, and prohibits generated glue edits.
- The working tree is intentionally dirty from the concurrent glTF geometric-analysis/codec/module lease. Its renamed aggregate, codec relocation, existing `vector-operations`, `inference-measures`, `mesh-topology`, and manifest changes are protected baseline work. This lease edits only the existing measure import/export lines, the two local manifests, the new sibling module, and no generated file.
- Baseline SHA-256: measure Rust `cc477b107b528dc4e80b239d1fc6b237c1b73a852ea6709a05be9853c1daaa63`; measure TypeScript `7fabb2be58f6d4ca5cc46803255223d073746c602ccb3910c9c9eef949c0198d`; inference manifest `952a826fdfa1def8eea276e13c4f490951048bc0cf072a2cb002a440360a74d5`; module manifest `2a2d75cb8b1acdb15e5c0b00465416bc232f6479384866d1267ba2c76993d1ee`; protected generated Rust glue `c13eb6492ddf8256c7a6e84aae6756f2072923ac0bec9a16915c364ac0bf1719`.

## Semantic Decision

`🧾️measure` declares value units, availability and validity states, numerical forms, quality, provenance, policy, diagnostics, entity addresses, and the local inference-stage trait. It does not derive a result. Its independent terminal production consumers are the fourteen metric inferences (`size`, `area-volume`, `compactness`, `proportion`, `mass-distribution`, `curvature`, `thickness`, `concavity`, `clearance`, `adjacency`, `orientation`, `symmetry`, `roughness`, `topology`), `geometric-analysis`, and `inference-measures`.

It therefore moves unchanged to the precise subset-LCA module `🔨️modules/🧾️measurement-contracts`, semantic id `s.stdio.gltf.module.measurement-contracts`. Rust and TypeScript consumers use that canonical module directly. The old inference leaf, inference-manifest member, TypeScript re-export, and generated inference mount are removed; no copy, alias, forwarding export, or migration path is retained.

## Central Registrar Change

The central registrar atomically removed the retired inference mount and added exactly one sibling module mount in generated `📦️packages/🦀️rust/📦️glue.rs`:

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧾️measurement-contracts/🦀️component.rs"]
pub mod measurement_contracts;
```

This lease did not edit the generated file. The old `schema::inferences::measure` branch is absent and the canonical `modules::measurement_contracts` branch is unique.

## Applied Source Change

- Moved the Rust and TypeScript contracts without semantic modification to `🔨️modules/🧾️measurement-contracts/{🦀️component.rs,🟦️component.ts}` and structured their declarations with named regions.
- Retargeted fourteen metric inference leaves and `geometric-analysis` to the canonical module in Rust and TypeScript. Retargeted `inference-measures` in Rust.
- Removed the former TypeScript inference re-export, the retired inference manifest member, and the now-empty `🧬️schema/💡️inferences/🧾️measure` directory.
- Declared exactly fifteen terminal inference consumers in the module manifest: the fourteen metrics plus `geometric-analysis`. The intermediary `s.stdio.gltf.module.inference-measures` is intentionally not declared as a terminal consumer.

## Validation

| Check | Result |
|---|---|
| JSON parse of the module and inference manifests | Passed |
| Static referrer and retirement sweep | Passed: 16 direct Rust consumers (including the non-terminal construction module), 15 direct TypeScript consumers, and 15 declared terminal inference consumers; no old source path, import, inference identity, or retired directory remains |
| Registrar mount sweep | Passed: exactly one `measurement_contracts` mount; no retired measure mount |
| `git diff --check` (worktree and index, scoped to glTF and glue) | Passed |
| `cargo check -p semio-s-plugin-stdio` | Passed in 0.54s, with 941 pre-existing workspace warnings |
| `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` | Completed. The census retains `s.stdio.gltf.module.measurement-contracts` with 15 consumers and records no problem for that module or the retired measure identity/path. It still reports 84 unrelated pre-existing glTF taxonomy problems. |
| `bun nx run @semio-tech/stdio-plugin:test` | Did not complete: its configured `fundamental` nextest subgate exceeded the 15-second budget and was killed; no assertion or compilation failure was reported. |

## Release Scope

The contract lease is source-clean and compile-validated. The remaining release gate is the coordinator-owned scoped Nx quick validation timeout; the 84 remaining census findings are outside this lease and do not involve `measurement-contracts` or the retired measure leaf.

## Standards And Subset Manifest Lease

### Scope And Baseline

- Read the repository, `✏️s`, and `🗄️stdio` instructions; no deeper glTF instruction file exists.
- Followed the exact five-finding queue in [Sol glTF Scoped Problem Lease Map](./sol-gltf-scoped-problem-lease-map.md): one missing standards manifest, then a missing membership and immediate leaf for each of `🔖️2.0` and `✳️any`.
- Kept the concurrently dirty `🔨️modules/🔣️component.json` outside this lease. No leaf implementation, I/O, schema, mutation, glue, taxonomy, root, or generator file was changed.

### Applied Structure

- Added `🏅️standards/🔣️component.json` as an `x-semio` collection with the exact `🔖️2.0` member: `s.stdio.gltf.standard.2.0`, kind `standard`, and a specific assembly responsibility.
- Added immediate `🦀️component.rs` and `🟦️component.ts` leaves at `🏅️standards/🔖️2.0`; both contain only named regions and therefore remain mechanical assembly.
- Preserved the existing legacy `🪆️subsets/🔣️component.json` fields and added its `x-semio` collection extension with the exact `✳️any` member: `s.stdio.gltf.subset.any`, kind `subset`, and a specific assembly responsibility.
- Added immediate `🦀️component.rs` and `🟦️component.ts` leaves at `🪆️subsets/✳️any`; both are assembly-only named regions.

### Validation

| Check | Result |
|---|---|
| Manifest JSON parse and exact member-field assertion | Passed |
| Immediate canonical Rust and TypeScript leaf presence and assembly-only regional shape | Passed |
| `git diff --check` for the standards scope, worktree and index | Passed |
| `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` | Invoked after the edits and completed. This report mode streams its full census only to stdout; the terminal yield did not retain the final stream. The five targeted conditions were independently checked against the same collection rules and are absent. |

The five owned findings are resolved by exact structural membership and leaves. The remaining scoped taxonomy findings belong to the schema-assembly, artifact-I/O, and mutation leases in the central map.

## Lease B — Inference Contract To Geometric Analysis

### Baseline And Scope

- Lease owner: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences`.
- The collection-root contract baselines were recorded before the source move: Rust `597d2e3f0b2f635c364912f083580eff966ec2223876141b63276d8c5a67ffac`, TypeScript `a2ef524638d689d316f2a5184343d6de3733baf74b9a4bbd1cbf7c4db4ecac1f`, GraphQL `d055dae8e4d29126b3173000a854d8b33f6521ac09c881a4e75fd0ed631a9294`, JSON Schema `86cc57c4edaa8522d085c3a0d5c087434c18727ab46d8ba75f72d577cf6440e9`, and protobuf `7df119bd6a723927a05372f4bed3f35500dbee17d46e0cb0a32d37a3ac128717`.
- This source lease excludes generated `📦️glue.rs`, taxonomy/root/script/index/lock files, and all unrelated dirty paths.

### Applied Move

- Moved the root GraphQL, protobuf, and complete JSON Schema contracts to `💡️inferences/🧮️geometric-analysis`; their destination SHA-256 values exactly equal the recorded root-source values.
- Moved `GltfInference`, its protocol/default/spec/field invalidation behavior, descriptor, and contract tests into the geometric-analysis Rust facet. Moved the TypeScript `GltfInference` contract into its TypeScript facet.
- Repointed the glTF artifact descriptor and local text/binary Rust codecs to `schema::inferences::geometric_analysis`; repointed the local text TypeScript and protobuf codec imports to the corresponding geometric-analysis facets.
- Replaced root Rust and TypeScript with named-region mechanical assembly only, removed root GraphQL and protobuf facets, and replaced the root JSON contract with only its canonical `x-semio` collection manifest. The collection has exactly its existing fifteen inference members.
- No forwarding export, alias, compatibility facade, or copied contract remains at the inference root.

### Referrers And Central Registrar

- Pre-move direct consumers were the glTF artifact descriptor, text and binary Rust codecs, text TypeScript codec, and text protobuf codec. The post-move sweep finds no direct Rust, TypeScript, or protobuf import targeting the retired inference-root contracts.
- The required generated registrar action was the removal of the former glTF `schema::inferences` root `component` mount/re-export immediately after the `geometric_analysis` mount. Central applied it: current `📦️glue.rs` retains exactly the `geometric_analysis` mount and has no glTF inference-root `🦀️component.rs` mount. This lease did not edit generated glue.

### Validation

| Check | Result |
|---|---|
| Root/destination contract topology and exact 15-member manifest assertion | Passed |
| GraphQL, protobuf, and JSON Schema source-to-destination SHA-256 equivalence | Passed |
| Retired-root direct Rust/TypeScript/protobuf referrer sweep | Passed |
| `cargo check -p semio-s-plugin-stdio` after central registrar update | Passed in 23.41s; 941 pre-existing workspace warnings |
| `bun ./📜️script.ts verify taxonomy report --scope s.stdio.gltf` | Completed: 23 components, 76 errors, 0 warnings. The displayed errors concern the separately owned artifact/I/O collection-manifest paths. |
| `git diff --check` worktree and index, scoped to glTF and glue | Passed |

Lease B is source-complete and has no remaining source-side or registrar blocker.
