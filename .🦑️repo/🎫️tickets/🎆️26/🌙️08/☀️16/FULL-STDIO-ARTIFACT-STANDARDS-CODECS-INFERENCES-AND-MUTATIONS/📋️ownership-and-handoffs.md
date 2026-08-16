# Ownership and Handoffs

## Program identity

- Goal: `🎯aioptimizedrepo` (`AI-OPTIMIZED-REPO`)
- Ticket: `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`
- GitHub issue: `https://github.com/usalu/semio/issues/2557`
- Catalog scope: the 36 entries in `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`
- Non-artifact cleanup: the extra generic `🧬️schema` root is infrastructure to relocate, never artifact 37.

## Dependency handoffs

| Dependency | This program consumes | This program owns afterward |
|---|---|---|
| `26/08/15/FULL-GLTF-GEOMETRIC-INFERENCES-AND-SEMANTIC-MUTATIONS` | The current glTF schema, geometry algorithms, codecs, runtime tests, and in-flight taxonomy refactor | Atomic inference leaves, command-local mutation triads, plural declarations, strict registries, and program-wide gates; the dependency's dirty paths remain preserved until its owner releases them. |
| `SEMANTIC-MUTATIONS-OVERHAUL` | Semantic command traits, mutation laws, event reset/checkpoint lane, and vocabulary policy | Stdio artifact command inventories and leaf implementations; no generic `Set*`, `SetSnapshot`, `CollectionMutation`, `NoMutation`, or whole-document semantic command remains. |
| `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` | Artifact/event-store boundaries and the rule that reusable kernels do not own authoritative state | Pure codec/math/topology/signal kernels plus artifact-owned schemas, diffs, commands, inferences, and IO; no artifact-private stateful engine is introduced. |

## Wave 0 exclusive write leases

| Lease | Owner | Paths | Deliverable |
|---|---|---|---|
| W0-A | Terra A | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` and its directly co-located tests only | Plural schema-owned artifact definition/declaration registry, deterministic conflict rejection, localized descriptor and capability identities. |
| W0-B | Terra B | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`, and their directly co-located tests only | Codec/resource/policy contracts, conflict-rejecting IO/document registries, typed projection and strict apply seams. |
| W0-C | Terra C | Stdio catalog/definition/manifest/package assembly excluding `🧊️gltf/**`; root `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` | Definition-derived catalog ledger and counts, false schema-root relocation plan/implementation where safe, TypeScript package completeness, Wave 0 Nx/launch gates. |
| W0-I | Coordinator | Ticket documents, ownership integration, review, cross-lane compilation, and barrier evidence | Contract freeze, combined-tree verification, audit dispatch, and remediation allocation. |

No lane may edit another lane's lease. The current glTF changes are treated as externally owned and are not reformatted, restored, deleted, or folded into Wave 0. Each worker records exact touched paths and evidence in this ticket. Heavy Cargo/Nx gates run serially at the barrier.

## Audit barriers

After every writer pass, the tree is frozen for three read-only Luna audits:

1. standards, profile, representation, and transitive codec coverage;
2. schema/taxonomy/open-closed/multi-implementation parity;
3. runtime, CQRS, security, test, performance, and evidence honesty.

Findings are remediated by new non-overlapping Terra leases before the next wave begins.
