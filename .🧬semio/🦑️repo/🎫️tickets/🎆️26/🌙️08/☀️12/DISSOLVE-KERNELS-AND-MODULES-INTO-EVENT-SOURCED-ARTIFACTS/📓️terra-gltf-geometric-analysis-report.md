# Terra glTF Geometric Analysis Lease

## Baseline

- Ticket: `2026/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`
- Goal and ticket were confirmed through the parent task's repo MCP session before this lease began.
- Applicable instructions read in full: repository `AGENTS.md` and `✏️s/AGENTS.md`. No nested `AGENTS.md` exists below the glTF 2.0 any-subset root.
- `git status --short` was checked before edits. The worktree already contained unrelated changes, but none below `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`; this lease begins with that scope clean.

### Contract Baseline Hashes (SHA-256)

| File | SHA-256 |
| --- | --- |
| `.../💡️inferences/🦀️component.rs` | `3fc7cff8386e22f17da61163d6f6dcc8b68beee25ebfe70e86440046cce28817` |
| `.../💡️inferences/🟦️component.ts` | `346fcb8e517d54f5c41a02bb2c867655ee0dae38e5950bde9de2f35ba55e5be9` |
| `.../💡️inferences/🔗️component.graphql` | `67558b54ee490e4101e6d464b0ddc9d9d6d72ccbc144143f687a9b071b2e7bbe` |
| `.../💡️inferences/🔣️component.json` | `d257ae99b540622d7328bf1bce72194a894c67082de2114752dab3290f69477b` |
| `.../💡️inferences/🛰️component.proto` | `8f9744228688623e8e56809624ac751782aeae5ff43c799df818ab443eab8f10` |
| `.../💡️inferences/📐️geometry/🦀️component.rs` | `87ee415f11a22a2a8f6066ca08fd3e0845c381ad553bfb75ebb9718a2291cc62` |
| `.../💡️inferences/📐️geometry/🟦️component.ts` | `df0f2e875ac3e649f69ac8eea2975c02074b34084de55270124c81cc417bd7be` |
| `.../💡️inferences/📝️text/🦀️component.rs` | `67558b54ee490e4101e6d464b0ddc9d9d6d72ccbc144143f687a9b071b2e7bbe` |
| `.../💡️inferences/💾️binary/🦀️component.rs` | `66a90cdc867a86715160dec65f42d27269932d3729b049e535779d270e262b3a` |
| `.../💡️inferences/📝️text/📖️component.grammar.semio` | `dbcaa1f2a7ce880948e029303c58e8a87dfd78085e1d778c6f8a34b7a5dd69e6` |
| `.../💡️inferences/📝️text/🅰️component.g4` | `e72c3216a0f7d331fe513f3e43241f7500caa467968a5aa36e75fbd10153ab24` |
| `.../💡️inferences/📝️text/🔤️component.ebnf` | `4329ba6966e9491d5126936e67a2fb9c177611a90d4a2f16c7da435490fcfd2b` |
| `.../💡️inferences/💾️binary/📡️component.protocol.semio` | `4c644059ffa4eb1dbae99eecf30f743f8034c90a8147e8d8786972df35dc5aae` |
| `.../💡️inferences/💾️binary/🥋️component.ksy` | `b64ba3a0cb5af266cc9c54c8568634b841d76855712e30ddb77641e66865dc4e` |
| `.../💡️inferences/💾️binary/🔠️component.abnf` | `155a73927d3657ca6856aa8c9c288571ee6dc021df4554e8a131972af21fb860` |
| `.../💡️inferences/💾️binary/🌶️component.spicy` | `f5287e18eba5d339d29f23a4892d206f223022864fc33910bfe5059cfa74ede6` |

### Baseline Referrers

- The aggregate Rust and TypeScript contracts expose `GltfInference.geometry`; GraphQL, JSON Schema, and protobuf represent the same root field.
- The text and binary codec specifications currently encode the `geometry` root and are source inputs, not generated outputs.
- Fourteen independent metric inference leaves consume the umbrella's shared geometric context: `size`, `area-volume`, `compactness`, `proportion`, `mass-distribution`, `curvature`, `thickness`, `concavity`, `clearance`, `adjacency`, `orientation`, `symmetry`, `roughness`, and `topology`.
- The only direct glTF consumers are the standard 2.0 any-subset schema, its `🚪️io` glue, its local examples/tests, and the glTF artifact facade. External matches found by broad search were unrelated standards and remain outside this lease.
- The direct stdio Rust glue mounted the old umbrella through `pub use geometry as bounds`; this lease removes it without a replacement alias.

## Registrar Requests

- No shared registrar, root taxonomy/discovery surface, project script, or launch configuration is in this lease. If global registration of the renamed field/module becomes necessary, the parent registrar owner must add it after this scoped migration.

## Implementation

- Replaced the semantic umbrella directory `📐️geometry` with `🧮️geometric-analysis`; all fourteen existing metric leaves remain mounted.
- Renamed the source-contract root from `geometry` to `geometricAnalysis` in Rust, TypeScript, GraphQL, JSON Schema, protobuf, dependency field IDs, and text/binary codec inputs. No compatibility field or forwarding module was retained.
- Relocated text and binary codecs (Rust, TypeScript, grammar, protocol, and codec schema inputs) beneath `🚪️io/💡️inferences`; direct stdio glue and the glTF facade now consume `io::inferences::binary`.
- Extracted the multiply-consumed measurement primitives (`Topology`, availability/exactness measures, and vector arithmetic) to `💡️inferences/🔨️modules/🧮️geometric-measurement`; the fourteen metric leaves consume that module where applicable. Remaining decoding, context construction, and analysis orchestration are private to `🧮️geometric-analysis`.
- Removed the `pub use geometry as bounds` glue alias and old `schema::inferences::{text,binary}` mounts.

## Validation

- Structural search: no old `📐️geometry` path, `pub use geometry as bounds`, public `geometry` field, or `schema::inferences::{binary,text}` consumer remains in the glTF lease.
- `bun nx show projects` confirmed the target project is `@semio-tech/stdio-plugin`; the initially assumed `semio-s-plugin-stdio` project does not exist.
- `bun nx run @semio-tech/stdio-plugin:test` was run after migration. Its first run found seven missing `unavailable` imports in the size leaf introduced by the module extraction; those imports were corrected.
- A repeat scoped Nx test still exits non-zero after a full build stream. The only terminal output is the workspace runner's generic non-zero wrapper amid 800 pre-existing workspace warnings; filtering found no Rust compiler error, panic, failed-test summary, or glTF-specific failure. This remains the validation blocker and needs the stdio test-owner's verbose log/runner diagnosis.

## Registrar Requests

- No registrar change is needed for this scoped source migration. If a global taxonomy/discovery catalog consumes the renamed `geometricAnalysis` field or new I/O paths, its owner must make that independent registration.

## Ownership-Correction Amendment

- Removed the noncompliant nested `🧬️schema/💡️inferences/🔨️modules/🧮️geometric-measurement` component and its glue mount.
- Added three coherent components under the glTF 2.0 any-subset LCA, `✳️any/🔨️modules`: `🧮️vector-operations` for seven vector primitives (used by size, concavity, curvature, roughness, and orientation); `💡️inference-measures` for exact/estimated/unavailable measure construction (used by all fourteen leaves); and `🕸️mesh-topology` for the topology summary consumed by clearance, adjacency, area-volume, orientation, symmetry, and the aggregate analysis.
- Rewired all metric leaves and `🧮️geometric-analysis` directly to those LCA components. Aggregate-only policy construction and its private orchestration remain in `🧮️geometric-analysis`; no forwarding alias was introduced.
- Corrected an initially misplaced glue mount that briefly targeted the unrelated raw-binary subset before the next test run; the final mount is exclusively at the glTF `✳️any` scope.
- Re-ran `bun nx run @semio-tech/stdio-plugin:test --output-style=static`. The first correction-run reported missing `super::modules` paths, fixed by mounting the three components at the actual glTF any-subset LCA. The repeat reached a non-zero runner exit with no Rust compiler error, panic, failure summary, or test-result marker in filtered output; the existing noisy test-runner diagnosis remains the blocker.

## Manifest Amendment

- Added canonical manifests at each leased semantic collection root:
  - `✳️any/🔨️modules/🔣️component.json`: `s.stdio.gltf.module.vector-operations`, `s.stdio.gltf.module.inference-measures`, and `s.stdio.gltf.module.mesh-topology`.
  - `🧬️schema/💡️inferences/🔣️component.json`: exact direct-child membership for the measure vocabulary, all fourteen metric leaves, and aggregate `s.stdio.gltf.inference.geometric-analysis`, whose inputs are `schema/snapshot` and `buffers` and whose derived target is `geometricAnalysis`.
  - `🚪️io/💡️inferences/🔣️component.json`: `s.stdio.gltf.inference.io.binary` and `s.stdio.gltf.inference.io.text`, declared as deterministic export codecs with their respective formats.
- Declared module production terminals: vector operations → `size`, `concavity`, `curvature`, `roughness`, `orientation`; inference measures → all fourteen metric leaves; mesh topology → `area-volume`, `clearance`, `adjacency`, `orientation`, `symmetry` (all IDs use the `s.stdio.gltf.inference.*` namespace).
- After exact mount/referrer scans, removed empty retired `📐️geometry`, `💾️binary`, `📝️text`, and nested `🔨️modules/🧮️geometric-measurement` directories. No contents were recoverable because all were empty after their prior source moves.
- `bun nx run workspace:verify-taxonomy-report -- --scope gltf` completed in report mode: 79 components and 135 findings. It confirms the manifests are parsed; remaining glTF-specific semantic findings are owned by the central graph model (it currently classifies `🚪️io/💡️inferences` as an inference collection and collapses resolved module consumers to the stdio plugin). Sol has been notified to rerun after that fix.
