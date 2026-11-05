# Native Test Ownership

Forms subset paths below start at 🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any. These are source/module mounts, not compiler test-list evidence.

| Cargo Package | Laws and Actual Mount |
| --- | --- |
| semio-s-artifact-forms-forms-config-tests | SetTry window lease/default/chunk/invalid JSON laws: ✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config/🧪️tests/🔬️window/🦀️.rs, mounted by its own 📦️packages/🦀️rust root. |
| semio-s-artifact-forms-forms-tests | Vector validation: 🧬️schema/✅️validation/🧪️tests/🦀️.rs. Shared Forms config unit/contracts: 🎚️config/🧪️tests/🔬️unit and 🔬️contract-vectors, now explicit root test mounts. |
| semio-s-artifact-forms-forms-tests-2 | Response events: 🧬️schema/📨️response/🧪️tests/🔬️events/🦀️.rs. Responses row law: ✏️editor/🎭️modes/📨️responses/🪟️windows/📊️results/🧪️tests/🦀️.rs. |
| semio-s-artifact-forms-forms-viewer | ALL Viewer entry/window/integration native laws: canonical source owner at 👁️viewer/📦️packages/🦀️rust. This replaces the former tests2 implementation mount and questions-owned Viewer unit tests. |
| semio-s-artifact-forms-forms-forms | Large extension envelope/editor unit: ✏️editor/🧪️tests/🔬️unit/🦀️.rs, through the editor component UnitTests mount. |
| semio-s-artifact-forms-forms-schema | Shared extension exact render vectors: 🧩️extensions/🧪️tests/🦀️.rs, explicit schema crate root mount. |
| semio-s-artifact-forms-forms-questions | Existing Try control vectors. All Viewer laws now belong to the dedicated Viewer crate. |

The entire Try/🧪️tests/🔬️unit/🦀️.rs file, including long_text_answers_render_paged_scenes_for_exact_fill_windows and its initial empty/full cases, is already cfg(test) mounted by the main editor mirror in semio-s-artifact-forms-forms-forms. Its unit harness imports match that owner. An initial partial search missed this mount and incorrectly described the file as dormant; full mirror review corrected that conclusion before native retry. No extra mount or duplicate law was added.

The shared UI native UTF8 text chunk law is in semio-framework-ui-contract-values (the values library itself), not semio-framework-ui-contract-values-laws. Its root is framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/📦️packages/🦀️rust/🦀️.rs. It is a dependency of Forms through ui-contract. Selecting it under the Forms manifest has not been verified; no duplicate metadata/check command was started.

The provider geometry owner additionally registers module_manifest_contributes_building_component, which projects the actual three Forms topic contributions into ProgramContributionEntry records and asserts public invocation character cost is within 4,096. Actual output size remains pending its native execution.

A comprehensive root invocation failed source admission before compiler because tests2 exceeded its 32-source owner cap after the Viewer retained files were added. The clean correction is a dedicated Viewer Cargo member, not a larger cap. Its normal dependency closure is acyclic and contains no tests2 dependency. Main editor/root facade directly consume the new Viewer package. tests2 now consumes the child projection from shared Snapshot and has no Viewer dependency. Manifest filesystem audit found no missing dependency paths. Root must add semio-s-artifact-forms-forms-viewer to comprehensive package selection.
