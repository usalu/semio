# Procedural Unit Composition Ownership

Two semantic development declarations remain in generation3d: actual Flow BREP and Math extensions. The unit-test serial lock installs those packs and contributes their manifests before app laws run. Removing the declarations without moving these actual laws would erase their concrete coverage. External geometry and incremental suites now belong to `✏️s/🧑‍💻dev/🧩️composition-laws`; fixture authorities and Parry3d/FNV remain unchanged.

The following authored modules directly use the concrete composition or its serial helper. This is an ownership inventory for extraction, not an accepted baseline or exception. Helper-only rows are retained separately from actual laws. Other leaves consume the editor/viewer shared contexts and must also be checked before moving those contexts.

| Source | Authored Native Laws | Direct Composition Uses |
| --- | ---: | --- |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🧪️tests/🔬️unit/🦀️.rs` | 3 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs` | 5 | test_serial::lock |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔪️knife-mesh-selection/🧪️tests/🔬️unit/🦀️.rs` | 4 | test_serial::lock |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🧪️tests/🔬️unit/🦀️.rs` | 5 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🔬️unit/🦀️.rs` | 7 | test_serial::lock |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 4 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 79 | flow_operators, test_serial::lock, brep_extension |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️eval-chain/🦀️.rs` | 5 | flow_operators, brep_extension |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️status-contract/🦀️.rs` | 8 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs` | 13 | test_serial::lock, brep_extension |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` | 15 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs` | 0 | flow_operators, brep_extension |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔒️serial-lock-discipline/🦀️.rs` | 5 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs` | 0 | flow_operators |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️publication-authority/🦀️.rs` | 0 | test_serial::lock |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️serial/🦀️.rs` | 0 | flow_operators |

Extraction must consume existing production public APIs and preserve every actual law. Pure artifact laws remain generic. The new integration owner must explicitly own the real extension session, registry initialization, contributions, mesh delivery and teardown. It must not compile a duplicate artifact model, expose a test-only runtime facade, or change role labels to mask ownership.
