# Forms Unsupported Kind Localization

The Try window now resolves its unsupported-kind message through FormsLabels for the explicitly selected locale and terminology. English is “Unsupported kind”; German is “Nicht unterstützte Art”. Both native and reuse terminology are defined.

The existing language-neutral controls fixture now includes four unsupported-kind cases. The existing Rust control-vector test renders the fallback for every case and validates its serialized message with the existing serde_json dependency. It also compares the repository JSON parser against serde_json for the fixture. No runtime dependency was added.

Test-first evidence: the fixture and law were written before implementation; the original English literal disagrees with both German vectors. A bounded source/fixture check verified all four final label-axis combinations and absence of the original renderer literal. Native compilation was intentionally left to the parent task’s pending native selection.

Native law owner: semio-s-artifact-forms-forms-questions; law: unsupported_question_messages_follow_language_neutral_vectors.

Changed files (relative to Forms any subset):
- ✏️editor/🗣️terminology/🦀️.rs
- ✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs
- ✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🧫️fixtures/🔣️controls.json
- ✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🧪️tests/🔬️control-vectors/🦀️.rs

No temporary output files or generated folder were created.

Root ownership correction: the questions crate entry mounts control_vectors at lines52–53 under cfg(test). The editor reexports production Try code but does not compile this test module. The questions package is already in the selected10-owner native command. Fresh canonical source inventories under all three platform cfgs still fit all limits. Native assertion execution remains pending.
