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

## Pending Central Registrar Change

Generated `📦️packages/🦀️rust/📦️glue.rs` currently mounts `schema::inferences::measure` at lines 2217-2218 and has no `modules::measurement_contracts` mount. The central registrar must delete that former branch and add exactly one sibling module branch:

```rust
#[path = "../../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧾️measurement-contracts/🦀️component.rs"]
pub mod measurement_contracts;
```

This lease will not edit the generated file. Cargo validation follows the central correction; scoped taxonomy verification follows the central owner's availability signal.
