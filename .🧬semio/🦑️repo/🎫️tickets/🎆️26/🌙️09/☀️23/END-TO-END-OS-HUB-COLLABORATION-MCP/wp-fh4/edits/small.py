"""📋️🪐️📐️ forms, space, cad (P2-X): the refusals whose producer computed a sentence (`validate()`'s message, the child
URI's reason, the directory page's fault) keep only the fact — the binding goes, the frozen code carries the meaning; the
space and forms laws assert the code and target instead of the Debug text. draw, gis and layout take the defaults."""
FORMS = "📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/"
HOME = "🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/"
CAD = "📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
PAGE = HOME + "🫧️transient/🧬️schema/🧬️mutations/📬️apply-directory-page/🦀️.rs"
CAD_LEAVES = ["⚡create-energy-model", "🏛️create-structure-classic-model", "🏢create-building-model", "📐️create-drawing", "🧱create-shape-model"]

OVERRIDES = {
    (FORMS + "🧬️schema/🧬️mutations/📨️commit-response/🔺️diff/🦀️.rs", 6): {"skip": True},
    (PAGE, 22): {"skip": True},
    **{(CAD + leaf + "/🔺️diff/🦀️.rs", 11): {"skip": True} for leaf in CAD_LEAVES},
}

EDITS = [
    (FORMS + "🧬️schema/🧬️mutations/📨️commit-response/🔺️diff/🦀️.rs",
     'if let Err(message) = payload.response.validate() { return protocol::MutationOutcome::error("forms.invalid-response", message, [payload.response.id.clone()]); }',
     'if payload.response.validate().is_err() { return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, [payload.response.id.clone()]); }'),
    (PAGE,
     '''        let refused = |code: semio_framework_plugin::FaultCode, text: &str| protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::error(code, text).at(["directory"])]);
        let Ok(page) = store::os_directory::DirectoryEventPageV1::parse_canonical_json(&self.page_json) else {
            return refused(semio_framework_plugin::app_fault("s.home.directory-event-page-invalid").code, "The directory page is not one canonical, receipt-sealed page.");
        };''',
     '''        let refused = || protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::error(protocol::MutationCode::Invariant).at(["directory"])]);
        let Ok(page) = store::os_directory::DirectoryEventPageV1::parse_canonical_json(&self.page_json) else {
            return refused();
        };'''),
    (PAGE, '''            Err(fault) => refused(fault.code, "The directory page does not continue the frontier the projection holds."),''',
     '''            Err(_) => refused(),'''),
    (HOME + "🎚️config/🧪️tests/🔬️unit/🦀️.rs",
     '''        assert!(outcome.messages().iter().any(|message| format!("{message:?}").contains("s.home.local-studio-tombstone-refused")));''',
     '''        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.target == ["retiredLocalStudioIds"]));'''),
    (HOME + "🫧️transient/🧪️tests/🔬️unit/🦀️.rs",
     '''    assert!(refused.messages().iter().any(|message| format!("{message:?}").contains("s.home.directory-event-page-invalid")));''',
     '''    assert!(refused.messages().iter().any(|message| message.code.0 == "mutation.invariant" && message.target == ["directory"]));'''),
] + [
    (CAD + leaf + "/🔺️diff/🦀️.rs",
     'Err(reason) => return protocol::MutationOutcome::fatal("mutation.child-identity", reason, [payload.child_id.clone()]),',
     'Err(_) => return protocol::MutationOutcome::fatal(protocol::MutationCode::Invariant, [payload.child_id.clone()]),')
    for leaf in CAD_LEAVES
]

RENAMES = [(FORMS, {"forms.missing-response": "TargetMissing", "forms.invalid-response": "Invariant", "forms.duplicate-response": "DuplicateId"})]

EDITS.append((HOME + "🎚️config/🧪️tests/🔬️unit/🦀️.rs",
              '''    assert!(again.messages().iter().any(|message| format!("{message:?}").contains("mutation.no-op")));''',
              '''    assert!(again.messages().iter().any(|message| message.code.0 == "mutation.no-op"));'''))
