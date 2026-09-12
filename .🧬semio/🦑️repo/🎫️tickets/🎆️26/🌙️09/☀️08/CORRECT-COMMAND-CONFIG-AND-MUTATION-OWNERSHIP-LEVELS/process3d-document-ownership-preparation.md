# Process3D Ownership Preparation

Read-only inspection found three independently redeclared child-handle families across artifact/snapshot/diff TypeScript. The artifact parser expects a string target despite its exported type declaring an object; nested workshop, stock, pose and step parsers return arbitrary objects. JSON repeats the same string-target mismatch and permits arbitrary nested payloads. The diff exposes obsolete Process3dStepsDelta helpers and lacks its own full parser. Native parent schemas still own extensive text/binary codecs. Pure diff application was found below the text leaf and is now moved as described below. These require a complete document-contract migration with real committed fixtures, independently validated nested domain shapes and exact BRep/Flow child identities. Full child payload persistence remains part of the SDK closure work.

The canonical child-kind metadata correction is complete and recorded separately. This report does not claim those parser/codec/runtime changes are implemented.

## Retained Route Test Schema Ownership

The artifact JSON Schema also embedded Process3dRetainedRouteLaws: route partition, test byte grants, source-code proof labels and hostile-input test flags. The registered TypeScript test was its only consumer. These are test contracts, so the schema and its neutral fixture now live together under the artifact's `🧪️tests/⚖️retained-route-laws` owner. The artifact schema retains only document types; the existing registered test loads the test-owned schema directly. No compatibility definition remains.

Changed files: Process3D artifact JSON Schema; moved `🧫️fixtures/⚖️retained-route-laws.json` into the exact test's `🧫️fixtures/🔣️.json`; new exact test `🧬️schema/🔣️.json`; Process TypeScript package `📜️script.ts`. The existing test target supplies independent Ajv plus extent/partition oracle verification.

The registered `@semio-tech/process-js:test` run (`process3d-test-contract-owner-1.log`) passed both example asset tests and independently validated the relocated test-owned schema against all 31 retained routes: 25 bounded and six resumable. The hostile source/fixture checks also passed.

## Pure Diff Behavior

`Process3dDiff::apply_to_artifact`, `MutationDiff::apply/absorb`, and `diff_set_snapshot` now live alongside the diff schema. The two existing native unit laws moved with them from the text leaf into `🧬️schema/🔺️diff/🧪️tests/🔬️unit`. The text leaf retains only its grammar and string carrier. There were no external helper imports to retarget. Native validation for this pure behavior relocation is still pending.

The focused permanent native route is now `@semio-tech/process-process3d-rs:test-diff`, implemented by the existing owning artifact script with a Nextest filter for the two moved diff tests. Both launch sources register it at 311.190. Native attempt1 never reached Cargo: Nx project discovery encountered concurrently relocated Equation generator Cargo metadata and a missing npm graph project. Attempt2 uses an isolated ticket-owned Nx discovery directory with the same registered owner target. No foreign source or cache was deleted.

Native attempt2 also stopped before Cargo on unrelated workspace-discovery metadata. A ticket-only `process3d-diff-laws` Nx facade now invokes the same canonical Process3D package script with the identical filter. It avoids repeated whole-workspace Nx discovery while preserving the owning native runner, normal 2MiB test stack and ticket-owned generated artifacts. The permanent artifact target and launch remain the reviewable developer entry.

## Native Diff Ownership Verification

The registered ticket `process3d-diff-laws` facade reached the owning Process3D Rust runner and completed on the explicit `RUST_MIN_STACK=2097152` test setting. Nextest ran both selected schema diff laws: **2 passed, 330 unrelated tests skipped**, in 0.058 seconds after a 2m05s build and invocation. Evidence: `🗑️generated/process3d-diff-owner-native-3.log`. This proves the moved pure diff tests compile and pass at their schema owner; it does not close the separately documented parent-child contract or codec ownership work.
