# Forms And DAG Native Regression Findings

Both full libraries now compile after shared capability and local test-context repairs. Forms runs 209 tests but app creation fails because its manifest advertises updatePlaybook even though no such Forms command, factory or classification exists. This is a foreign domain action, not a missing Forms classification. The stale Playbook manifest entry was removed; updateForm remains the existing Forms action. Existing manifest-to-command laws cover the correction.

Both artifact demo inference tests also expose historical fixture expectations. DAG live command tests report further runtime failures; these need isolated diagnosis. Full-suite success is not claimed. Targeted parent contract tests will run independently so ownership evidence is not hidden by an unrelated abort.

## DAG Example Ownership

DAG's example definition and inference test still imported the framework DAG example, whose document marker is dag.fixture. The plugin requires dag.dag. The plugin example now owns its own asset include, and codecs/default/tests consume examples::demo::PRIMARY_TEXT. Removed the IO-owned duplicate constant and retargeted all consumers without compatibility aliases. This source correction addresses the native demo parse failure; rerun pending.

- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🧪️tests/🔬️semio-protocol-conformance/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

DAG config's neutral mutation law had representation-sensitive serde_json integer-vs-float equality. It now decodes expected state with the independent serde deserializer and compares the typed record against the native DSL mutation result. The fixture values and expected behavior are unchanged.
