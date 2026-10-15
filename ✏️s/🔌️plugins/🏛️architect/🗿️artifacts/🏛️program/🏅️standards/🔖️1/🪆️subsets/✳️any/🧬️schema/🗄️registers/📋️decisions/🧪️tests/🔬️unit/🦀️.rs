//! ⚖️ Patch round-trip tests of the decision registers: requirements, priorities, scenarios, option evaluations, decisions, validations, performance, quality, artifacts, changes, collaboration, analyses, reports and search filters.

use super::*;

macro_rules! patch_through_program {
    ($item:ident, $patch:expr, $field:ident, $delta:ty, $id:expr) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$field.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($id, $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff(&diff, &program).expect("the patch commits through the central applier").$field.remove(0);
    }};
}

#[semio_framework_async_macros::async_test]
async fn requirement_patch_round_trips() {
    let mut item = Requirement {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("requirement", "Base Requirement"), "Base Requirement") },
        code: String::new(),
        kind: RequirementKind::Functional,
        statement: TextField::default(),
        rationale: Some(TextField::default()),
        source: Some(String::new()),
        stakeholder_ids: Vec::new(),
        element_ids: Vec::new(),
        function_ids: Vec::new(),
        parent_requirement_id: Some(EntityId::new_serial("base38", "base38")),
        child_requirement_ids: Vec::new(),
        acceptance_criteria: Vec::new(),
        verification_method: Some(String::new()),
        validation_status: ValidationStatus::Pending,
        conflict_ids: Vec::new(),
        risk_ids: Vec::new(),
        cost_estimate: Some(0.0),
        schedule_constraint: Some(String::new()),
        regulatory_refs: Vec::new(),
        trace_links: Vec::new(),
        superseded_by: Some(EntityId::new_serial("base38", "base38")),
    };
    let original = item.clone();
    let patch = RequirementPatch {
        name: Some("Patched Requirement".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-38".to_string()),
        kind: Some(RequirementKind::Spatial),
        statement: Some(TextField::plain("patched-38")),
        rationale: Some(TextField::plain("patched-38")),
        source: Some("patched-38".to_string()),
        stakeholder_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        element_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        function_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        parent_requirement_id: Some(EntityId::new_serial("new38", "new38")),
        child_requirement_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        acceptance_criteria: Some(vec!["patched-38".to_string()]),
        verification_method: Some("patched-38".to_string()),
        validation_status: Some(ValidationStatus::Passed),
        conflict_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        risk_ids: Some(vec![EntityId::new_serial("new38", "new38")]),
        cost_estimate: Some(42.0),
        schedule_constraint: Some("patched-38".to_string()),
        regulatory_refs: Some(vec!["patched-38".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom38n", "tfrom38n"), EntityId::new_serial("tto38n", "tto38n"), TraceKind::FullAuditTrail)]),
        superseded_by: Some(EntityId::new_serial("new38", "new38")),
    };
    patch_through_program!(item, patch, requirements, crate::standards::v1::subsets::any::schema::diff::ProgramRequirementsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Requirement");
    patch_through_program!(item, inverse, requirements, crate::standards::v1::subsets::any::schema::diff::ProgramRequirementsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn priority_record_patch_round_trips() {
    let mut item = PriorityRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("priorityrecord", "Base PriorityRecord"), "Base PriorityRecord") },
        subject_id: EntityId::new_serial("base39", "base39"),
        subject_kind: String::new(),
        ranked_priority: Priority::Mandatory,
        rank: Some(0),
        weight: Some(0.0),
        rationale: Some(TextField::default()),
        decision_id: Some(EntityId::new_serial("base39", "base39")),
        stakeholder_ids: Vec::new(),
        effective_from: Some(String::new()),
        effective_until: Some(String::new()),
        review_cycle: Some(String::new()),
        dependencies: Vec::new(),
        conflicts: Vec::new(),
        scoring_method: Some(String::new()),
        score: Some(0.0),
        criteria: Vec::new(),
        approved_by: Some(EntityId::new_serial("base39", "base39")),
        approval_date: Some(String::new()),
        ranking_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = PriorityRecordPatch {
        name: Some("Patched PriorityRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new39", "new39")),
        subject_kind: Some("patched-39".to_string()),
        ranked_priority: Some(Priority::Essential),
        rank: Some(7),
        weight: Some(42.0),
        rationale: Some(TextField::plain("patched-39")),
        decision_id: Some(EntityId::new_serial("new39", "new39")),
        stakeholder_ids: Some(vec![EntityId::new_serial("new39", "new39")]),
        effective_from: Some("patched-39".to_string()),
        effective_until: Some("patched-39".to_string()),
        review_cycle: Some("patched-39".to_string()),
        dependencies: Some(vec![EntityId::new_serial("new39", "new39")]),
        conflicts: Some(vec![EntityId::new_serial("new39", "new39")]),
        scoring_method: Some("patched-39".to_string()),
        score: Some(42.0),
        criteria: Some(vec!["patched-39".to_string()]),
        approved_by: Some(EntityId::new_serial("new39", "new39")),
        approval_date: Some("patched-39".to_string()),
        ranking_notes: Some(vec![TaggedNote { tag: "new39".into(), text: "new-note39".into() }]),
    };
    patch_through_program!(item, patch, priorities, crate::standards::v1::subsets::any::schema::diff::ProgramPrioritiesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PriorityRecord");
    patch_through_program!(item, inverse, priorities, crate::standards::v1::subsets::any::schema::diff::ProgramPrioritiesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn scenario_patch_round_trips() {
    let mut item = Scenario {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("scenario", "Base Scenario"), "Base Scenario") },
        code: String::new(),
        hypothesis: TextField::default(),
        assumptions: Vec::new(),
        variables: Vec::new(),
        element_ids: Vec::new(),
        requirement_ids: Vec::new(),
        growth_plan_id: Some(EntityId::new_serial("base40", "base40")),
        probability: Some(0.0),
        impact_summary: Some(TextField::default()),
        cost_delta: Some(0.0),
        area_delta: Some(0.0),
        headcount_delta: Some(0.0),
        schedule_delta: Some(String::new()),
        risk_ids: Vec::new(),
        option_ids: Vec::new(),
        baseline: false,
        preferred: false,
        analysis_ids: Vec::new(),
        owner_id: Some(EntityId::new_serial("base40", "base40")),
    };
    let original = item.clone();
    let patch = ScenarioPatch {
        name: Some("Patched Scenario".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        code: Some("patched-40".to_string()),
        hypothesis: Some(TextField::plain("patched-40")),
        assumptions: Some(vec!["patched-40".to_string()]),
        variables: Some(vec!["patched-40".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        requirement_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        growth_plan_id: Some(EntityId::new_serial("new40", "new40")),
        probability: Some(42.0),
        impact_summary: Some(TextField::plain("patched-40")),
        cost_delta: Some(42.0),
        area_delta: Some(42.0),
        headcount_delta: Some(42.0),
        schedule_delta: Some("patched-40".to_string()),
        risk_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        option_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        baseline: Some(true),
        preferred: Some(true),
        analysis_ids: Some(vec![EntityId::new_serial("new40", "new40")]),
        owner_id: Some(EntityId::new_serial("new40", "new40")),
    };
    patch_through_program!(item, patch, scenarios, crate::standards::v1::subsets::any::schema::diff::ProgramScenariosDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Scenario");
    patch_through_program!(item, inverse, scenarios, crate::standards::v1::subsets::any::schema::diff::ProgramScenariosDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn option_evaluation_patch_round_trips() {
    let mut item = OptionEvaluation {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("optionevaluation", "Base OptionEvaluation"), "Base OptionEvaluation") },
        option_name: String::new(),
        option_description: TextField::default(),
        scenario_id: Some(EntityId::new_serial("base41", "base41")),
        criteria_ids: Vec::new(),
        scores: Vec::new(),
        weighted_score: Some(0.0),
        cost_estimate: Some(0.0),
        schedule_estimate: Some(String::new()),
        risk_summary: Vec::new(),
        benefits: Vec::new(),
        drawbacks: Vec::new(),
        assumptions: Vec::new(),
        dependencies: Vec::new(),
        stakeholder_feedback: Vec::new(),
        recommendation: Some(String::new()),
        decision_id: Some(EntityId::new_serial("base41", "base41")),
        evaluation_status: ValidationStatus::Pending,
        evaluator_ids: Vec::new(),
        evaluation_date: Some(String::new()),
    };
    let original = item.clone();
    let patch = OptionEvaluationPatch {
        name: Some("Patched OptionEvaluation".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        option_name: Some("patched-41".to_string()),
        option_description: Some(TextField::plain("patched-41")),
        scenario_id: Some(EntityId::new_serial("new41", "new41")),
        criteria_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
        scores: Some(vec![42.0]),
        weighted_score: Some(42.0),
        cost_estimate: Some(42.0),
        schedule_estimate: Some("patched-41".to_string()),
        risk_summary: Some(vec!["patched-41".to_string()]),
        benefits: Some(vec!["patched-41".to_string()]),
        drawbacks: Some(vec!["patched-41".to_string()]),
        assumptions: Some(vec!["patched-41".to_string()]),
        dependencies: Some(vec![EntityId::new_serial("new41", "new41")]),
        stakeholder_feedback: Some(vec![TaggedNote { tag: "new41".into(), text: "new-note41".into() }]),
        recommendation: Some("patched-41".to_string()),
        decision_id: Some(EntityId::new_serial("new41", "new41")),
        evaluation_status: Some(ValidationStatus::Passed),
        evaluator_ids: Some(vec![EntityId::new_serial("new41", "new41")]),
        evaluation_date: Some("patched-41".to_string()),
    };
    patch_through_program!(item, patch, options, crate::standards::v1::subsets::any::schema::diff::ProgramOptionsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched OptionEvaluation");
    patch_through_program!(item, inverse, options, crate::standards::v1::subsets::any::schema::diff::ProgramOptionsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn decision_patch_round_trips() {
    let mut item = Decision {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("decision", "Base Decision"), "Base Decision") },
        decision_statement: TextField::default(),
        context: TextField::default(),
        options_considered: Vec::new(),
        selected_option_id: Some(EntityId::new_serial("base42", "base42")),
        rationale: TextField::default(),
        decision_maker_ids: Vec::new(),
        consulted_ids: Vec::new(),
        informed_ids: Vec::new(),
        decision_date: Some(String::new()),
        effective_date: Some(String::new()),
        reversal_conditions: Vec::new(),
        impacted_requirement_ids: Vec::new(),
        impacted_element_ids: Vec::new(),
        cost_impact: Some(0.0),
        schedule_impact: Some(String::new()),
        risk_impact: Vec::new(),
        approval_status: ValidationStatus::Pending,
        meeting_ref: Some(EntityId::new_serial("base42", "base42")),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = DecisionPatch {
        name: Some("Patched Decision".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        decision_statement: Some(TextField::plain("patched-42")),
        context: Some(TextField::plain("patched-42")),
        options_considered: Some(vec![EntityId::new_serial("new42", "new42")]),
        selected_option_id: Some(EntityId::new_serial("new42", "new42")),
        rationale: Some(TextField::plain("patched-42")),
        decision_maker_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        consulted_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        informed_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        decision_date: Some("patched-42".to_string()),
        effective_date: Some("patched-42".to_string()),
        reversal_conditions: Some(vec!["patched-42".to_string()]),
        impacted_requirement_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        impacted_element_ids: Some(vec![EntityId::new_serial("new42", "new42")]),
        cost_impact: Some(42.0),
        schedule_impact: Some("patched-42".to_string()),
        risk_impact: Some(vec!["patched-42".to_string()]),
        approval_status: Some(ValidationStatus::Passed),
        meeting_ref: Some(EntityId::new_serial("new42", "new42")),
        artifact_refs: Some(vec![EntityId::new_serial("new42", "new42")]),
    };
    patch_through_program!(item, patch, decisions, crate::standards::v1::subsets::any::schema::diff::ProgramDecisionsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Decision");
    patch_through_program!(item, inverse, decisions, crate::standards::v1::subsets::any::schema::diff::ProgramDecisionsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn validation_record_patch_round_trips() {
    let mut item = ValidationRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("validationrecord", "Base ValidationRecord"), "Base ValidationRecord") },
        subject_id: EntityId::new_serial("base43", "base43"),
        subject_kind: String::new(),
        validation_type: String::new(),
        method: Some(String::new()),
        criteria: Vec::new(),
        result: ValidationStatus::Pending,
        evidence: Vec::new(),
        validator_ids: Vec::new(),
        validation_date: Some(String::new()),
        next_review_date: Some(String::new()),
        findings: Vec::new(),
        non_conformities: Vec::new(),
        corrective_actions: Vec::new(),
        waivers: Vec::new(),
        standards: Vec::new(),
        trace_links: Vec::new(),
        report_id: Some(EntityId::new_serial("base43", "base43")),
        confidence_level: Some(String::new()),
        validation_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = ValidationRecordPatch {
        name: Some("Patched ValidationRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new43", "new43")),
        subject_kind: Some("patched-43".to_string()),
        validation_type: Some("patched-43".to_string()),
        method: Some("patched-43".to_string()),
        criteria: Some(vec!["patched-43".to_string()]),
        result: Some(ValidationStatus::Passed),
        evidence: Some(vec!["patched-43".to_string()]),
        validator_ids: Some(vec![EntityId::new_serial("new43", "new43")]),
        validation_date: Some("patched-43".to_string()),
        next_review_date: Some("patched-43".to_string()),
        findings: Some(vec!["patched-43".to_string()]),
        non_conformities: Some(vec!["patched-43".to_string()]),
        corrective_actions: Some(vec!["patched-43".to_string()]),
        waivers: Some(vec!["patched-43".to_string()]),
        standards: Some(vec!["patched-43".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom43n", "tfrom43n"), EntityId::new_serial("tto43n", "tto43n"), TraceKind::FullAuditTrail)]),
        report_id: Some(EntityId::new_serial("new43", "new43")),
        confidence_level: Some("patched-43".to_string()),
        validation_notes: Some(vec![TaggedNote { tag: "new43".into(), text: "new-note43".into() }]),
    };
    patch_through_program!(item, patch, validations, crate::standards::v1::subsets::any::schema::diff::ProgramValidationsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ValidationRecord");
    patch_through_program!(item, inverse, validations, crate::standards::v1::subsets::any::schema::diff::ProgramValidationsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn performance_criterion_patch_round_trips() {
    let mut item = PerformanceCriterion {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("performancecriterion", "Base PerformanceCriterion"), "Base PerformanceCriterion") },
        criterion: String::new(),
        metric: String::new(),
        target: Some(0.0),
        unit: Some(String::new()),
        minimum: Some(0.0),
        maximum: Some(0.0),
        measurement_method: Some(String::new()),
        frequency: Some(String::new()),
        requirement_ids: Vec::new(),
        element_ids: Vec::new(),
        baseline: Some(0.0),
        benchmark_ref: Some(EntityId::new_serial("base44", "base44")),
        weight: Some(0.0),
        data_source: Some(String::new()),
        reporting_cadence: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base44", "base44")),
        verification_plan: Some(String::new()),
        penalty_threshold: Some(0.0),
        incentive_threshold: Some(0.0),
    };
    let original = item.clone();
    let patch = PerformanceCriterionPatch {
        name: Some("Patched PerformanceCriterion".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        criterion: Some("patched-44".to_string()),
        metric: Some("patched-44".to_string()),
        target: Some(42.0),
        unit: Some("patched-44".to_string()),
        minimum: Some(42.0),
        maximum: Some(42.0),
        measurement_method: Some("patched-44".to_string()),
        frequency: Some("patched-44".to_string()),
        requirement_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
        element_ids: Some(vec![EntityId::new_serial("new44", "new44")]),
        baseline: Some(42.0),
        benchmark_ref: Some(EntityId::new_serial("new44", "new44")),
        weight: Some(42.0),
        data_source: Some("patched-44".to_string()),
        reporting_cadence: Some("patched-44".to_string()),
        owner_id: Some(EntityId::new_serial("new44", "new44")),
        verification_plan: Some("patched-44".to_string()),
        penalty_threshold: Some(42.0),
        incentive_threshold: Some(42.0),
    };
    patch_through_program!(item, patch, performance, crate::standards::v1::subsets::any::schema::diff::ProgramPerformanceDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched PerformanceCriterion");
    patch_through_program!(item, inverse, performance, crate::standards::v1::subsets::any::schema::diff::ProgramPerformanceDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn quality_record_patch_round_trips() {
    let mut item = QualityRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("qualityrecord", "Base QualityRecord"), "Base QualityRecord") },
        quality_topic: String::new(),
        standard: Some(String::new()),
        target_level: Some(String::new()),
        inspection_points: Vec::new(),
        acceptance_criteria: Vec::new(),
        testing_requirements: Vec::new(),
        sample_rate: Some(String::new()),
        defect_categories: Vec::new(),
        corrective_action_process: Vec::new(),
        element_ids: Vec::new(),
        requirement_ids: Vec::new(),
        supplier_requirements: Vec::new(),
        documentation_requirements: Vec::new(),
        training_requirements: Vec::new(),
        audit_schedule: Some(String::new()),
        kpis: Vec::new(),
        owner_id: Some(EntityId::new_serial("base45", "base45")),
        certification_targets: Vec::new(),
        continuous_improvement: Vec::new(),
    };
    let original = item.clone();
    let patch = QualityRecordPatch {
        name: Some("Patched QualityRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        quality_topic: Some("patched-45".to_string()),
        standard: Some("patched-45".to_string()),
        target_level: Some("patched-45".to_string()),
        inspection_points: Some(vec!["patched-45".to_string()]),
        acceptance_criteria: Some(vec!["patched-45".to_string()]),
        testing_requirements: Some(vec!["patched-45".to_string()]),
        sample_rate: Some("patched-45".to_string()),
        defect_categories: Some(vec!["patched-45".to_string()]),
        corrective_action_process: Some(vec!["patched-45".to_string()]),
        element_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
        requirement_ids: Some(vec![EntityId::new_serial("new45", "new45")]),
        supplier_requirements: Some(vec!["patched-45".to_string()]),
        documentation_requirements: Some(vec!["patched-45".to_string()]),
        training_requirements: Some(vec!["patched-45".to_string()]),
        audit_schedule: Some("patched-45".to_string()),
        kpis: Some(vec!["patched-45".to_string()]),
        owner_id: Some(EntityId::new_serial("new45", "new45")),
        certification_targets: Some(vec!["patched-45".to_string()]),
        continuous_improvement: Some(vec!["patched-45".to_string()]),
    };
    patch_through_program!(item, patch, quality, crate::standards::v1::subsets::any::schema::diff::ProgramQualityDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched QualityRecord");
    patch_through_program!(item, inverse, quality, crate::standards::v1::subsets::any::schema::diff::ProgramQualityDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn document_record_patch_round_trips() {
    let mut item = ArtifactRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("documentrecord", "Base ArtifactRecord"), "Base ArtifactRecord") },
        document_type: String::new(),
        title: String::new(),
        version: String::new(),
        file_ref: Some(String::new()),
        format: Some(String::new()),
        author_ids: Vec::new(),
        reviewer_ids: Vec::new(),
        approver_ids: Vec::new(),
        issue_date: Some(String::new()),
        revision_date: Some(String::new()),
        distribution_list: Vec::new(),
        related_entity_ids: Vec::new(),
        classification: Some(String::new()),
        retention_period: Some(String::new()),
        access_controls: Vec::new(),
        supersedes: Some(EntityId::new_serial("base46", "base46")),
        document_status: LifecycleStatus::Draft,
        checksum: Some(String::new()),
        source_system: Some(String::new()),
    };
    let original = item.clone();
    let patch = ArtifactRecordPatch {
        name: Some("Patched ArtifactRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        document_type: Some("patched-46".to_string()),
        title: Some("patched-46".to_string()),
        version: Some("patched-46".to_string()),
        file_ref: Some("patched-46".to_string()),
        format: Some("patched-46".to_string()),
        author_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        reviewer_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        approver_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        issue_date: Some("patched-46".to_string()),
        revision_date: Some("patched-46".to_string()),
        distribution_list: Some(vec![EntityId::new_serial("new46", "new46")]),
        related_entity_ids: Some(vec![EntityId::new_serial("new46", "new46")]),
        classification: Some("patched-46".to_string()),
        retention_period: Some("patched-46".to_string()),
        access_controls: Some(vec!["patched-46".to_string()]),
        supersedes: Some(EntityId::new_serial("new46", "new46")),
        document_status: Some(LifecycleStatus::Proposed),
        checksum: Some("patched-46".to_string()),
        source_system: Some("patched-46".to_string()),
    };
    patch_through_program!(item, patch, artifacts, crate::standards::v1::subsets::any::schema::diff::ProgramArtifactsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ArtifactRecord");
    patch_through_program!(item, inverse, artifacts, crate::standards::v1::subsets::any::schema::diff::ProgramArtifactsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn change_record_patch_round_trips() {
    let mut item = ChangeRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("changerecord", "Base ChangeRecord"), "Base ChangeRecord") },
        change_type: String::new(),
        summary: TextField::default(),
        reason: TextField::default(),
        requested_by: Some(EntityId::new_serial("base47", "base47")),
        approved_by: Some(EntityId::new_serial("base47", "base47")),
        change_date: Some(String::new()),
        effective_date: Some(String::new()),
        impacted_entity_ids: Vec::new(),
        before_snapshot: Some(String::new()),
        after_snapshot: Some(String::new()),
        cost_impact: Some(0.0),
        schedule_impact: Some(String::new()),
        risk_impact: Vec::new(),
        approval_status: ValidationStatus::Pending,
        rollback_plan: Vec::new(),
        communication_plan: Vec::new(),
        version_from: Some(String::new()),
        version_to: Some(String::new()),
        audit_event_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = ChangeRecordPatch {
        name: Some("Patched ChangeRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        change_type: Some("patched-47".to_string()),
        summary: Some(TextField::plain("patched-47")),
        reason: Some(TextField::plain("patched-47")),
        requested_by: Some(EntityId::new_serial("new47", "new47")),
        approved_by: Some(EntityId::new_serial("new47", "new47")),
        change_date: Some("patched-47".to_string()),
        effective_date: Some("patched-47".to_string()),
        impacted_entity_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
        before_snapshot: Some("patched-47".to_string()),
        after_snapshot: Some("patched-47".to_string()),
        cost_impact: Some(42.0),
        schedule_impact: Some("patched-47".to_string()),
        risk_impact: Some(vec!["patched-47".to_string()]),
        approval_status: Some(ValidationStatus::Passed),
        rollback_plan: Some(vec!["patched-47".to_string()]),
        communication_plan: Some(vec!["patched-47".to_string()]),
        version_from: Some("patched-47".to_string()),
        version_to: Some("patched-47".to_string()),
        audit_event_ids: Some(vec![EntityId::new_serial("new47", "new47")]),
    };
    patch_through_program!(item, patch, changes, crate::standards::v1::subsets::any::schema::diff::ProgramChangesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ChangeRecord");
    patch_through_program!(item, inverse, changes, crate::standards::v1::subsets::any::schema::diff::ProgramChangesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn collaboration_record_patch_round_trips() {
    let mut item = CollaborationRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("collaborationrecord", "Base CollaborationRecord"), "Base CollaborationRecord") },
        session_type: String::new(),
        title: String::new(),
        participants: Vec::new(),
        facilitator_id: Some(EntityId::new_serial("base48", "base48")),
        start_time: Some(String::new()),
        end_time: Some(String::new()),
        location: Some(String::new()),
        agenda: Vec::new(),
        outcomes: Vec::new(),
        action_items: Vec::new(),
        decision_ids: Vec::new(),
        issue_ids: Vec::new(),
        document_ids: Vec::new(),
        recording_ref: Some(String::new()),
        feedback: Vec::new(),
        follow_up_date: Some(String::new()),
        workshop_id: Some(EntityId::new_serial("base48", "base48")),
        survey_id: Some(EntityId::new_serial("base48", "base48")),
    };
    let original = item.clone();
    let patch = CollaborationRecordPatch {
        name: Some("Patched CollaborationRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        session_type: Some("patched-48".to_string()),
        title: Some("patched-48".to_string()),
        participants: Some(vec![EntityId::new_serial("new48", "new48")]),
        facilitator_id: Some(EntityId::new_serial("new48", "new48")),
        start_time: Some("patched-48".to_string()),
        end_time: Some("patched-48".to_string()),
        location: Some("patched-48".to_string()),
        agenda: Some(vec!["patched-48".to_string()]),
        outcomes: Some(vec!["patched-48".to_string()]),
        action_items: Some(vec!["patched-48".to_string()]),
        decision_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        issue_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        document_ids: Some(vec![EntityId::new_serial("new48", "new48")]),
        recording_ref: Some("patched-48".to_string()),
        feedback: Some(vec![TaggedNote { tag: "new48".into(), text: "new-note48".into() }]),
        follow_up_date: Some("patched-48".to_string()),
        workshop_id: Some(EntityId::new_serial("new48", "new48")),
        survey_id: Some(EntityId::new_serial("new48", "new48")),
    };
    patch_through_program!(item, patch, collaboration, crate::standards::v1::subsets::any::schema::diff::ProgramCollaborationDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched CollaborationRecord");
    patch_through_program!(item, inverse, collaboration, crate::standards::v1::subsets::any::schema::diff::ProgramCollaborationDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn analysis_record_patch_round_trips() {
    let mut item = AnalysisRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("analysisrecord", "Base AnalysisRecord"), "Base AnalysisRecord") },
        kind: AnalysisKind::Gap,
        title: String::new(),
        parameters: Vec::new(),
        input_entity_ids: Vec::new(),
        output_summary: TextField::default(),
        findings: Vec::new(),
        metrics: Vec::new(),
        charts: Vec::new(),
        run_by: Some(EntityId::new_serial("base49", "base49")),
        run_at: Some(String::new()),
        duration_ms: Some(0),
        tool_version: Some(String::new()),
        scenario_id: Some(EntityId::new_serial("base49", "base49")),
        report_id: Some(EntityId::new_serial("base49", "base49")),
        confidence: Some(String::new()),
        limitations: Vec::new(),
        recommendations: Vec::new(),
        raw_result_ref: Some(String::new()),
    };
    let original = item.clone();
    let patch = AnalysisRecordPatch {
        name: Some("Patched AnalysisRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        kind: Some(AnalysisKind::Conflict),
        title: Some("patched-49".to_string()),
        parameters: Some(vec!["patched-49".to_string()]),
        input_entity_ids: Some(vec![EntityId::new_serial("new49", "new49")]),
        output_summary: Some(TextField::plain("patched-49")),
        findings: Some(vec!["patched-49".to_string()]),
        metrics: Some(vec!["patched-49".to_string()]),
        charts: Some(vec!["patched-49".to_string()]),
        run_by: Some(EntityId::new_serial("new49", "new49")),
        run_at: Some("patched-49".to_string()),
        duration_ms: Some(7),
        tool_version: Some("patched-49".to_string()),
        scenario_id: Some(EntityId::new_serial("new49", "new49")),
        report_id: Some(EntityId::new_serial("new49", "new49")),
        confidence: Some("patched-49".to_string()),
        limitations: Some(vec!["patched-49".to_string()]),
        recommendations: Some(vec!["patched-49".to_string()]),
        raw_result_ref: Some("patched-49".to_string()),
    };
    patch_through_program!(item, patch, analyses, crate::standards::v1::subsets::any::schema::diff::ProgramAnalysesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AnalysisRecord");
    patch_through_program!(item, inverse, analyses, crate::standards::v1::subsets::any::schema::diff::ProgramAnalysesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn report_record_patch_round_trips() {
    let mut item = ReportRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("reportrecord", "Base ReportRecord"), "Base ReportRecord") },
        kind: ReportKind::ExecutiveSummary,
        title: String::new(),
        audience: Vec::new(),
        sections: Vec::new(),
        generated_at: Some(String::new()),
        generated_by: Some(EntityId::new_serial("base50", "base50")),
        analysis_ids: Vec::new(),
        format: Some(String::new()),
        file_ref: Some(String::new()),
        distribution_list: Vec::new(),
        approval_status: ValidationStatus::Pending,
        approver_id: Some(EntityId::new_serial("base50", "base50")),
        version: String::new(),
        template_id: Some(EntityId::new_serial("base50", "base50")),
        parameters: Vec::new(),
        confidentiality: Some(String::new()),
        expiry_date: Some(String::new()),
        related_decision_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = ReportRecordPatch {
        name: Some("Patched ReportRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        kind: Some(ReportKind::ProgramOverview),
        title: Some("patched-50".to_string()),
        audience: Some(vec!["patched-50".to_string()]),
        sections: Some(vec!["patched-50".to_string()]),
        generated_at: Some("patched-50".to_string()),
        generated_by: Some(EntityId::new_serial("new50", "new50")),
        analysis_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
        format: Some("patched-50".to_string()),
        file_ref: Some("patched-50".to_string()),
        distribution_list: Some(vec![EntityId::new_serial("new50", "new50")]),
        approval_status: Some(ValidationStatus::Passed),
        approver_id: Some(EntityId::new_serial("new50", "new50")),
        version: Some("patched-50".to_string()),
        template_id: Some(EntityId::new_serial("new50", "new50")),
        parameters: Some(vec!["patched-50".to_string()]),
        confidentiality: Some("patched-50".to_string()),
        expiry_date: Some("patched-50".to_string()),
        related_decision_ids: Some(vec![EntityId::new_serial("new50", "new50")]),
    };
    patch_through_program!(item, patch, reports, crate::standards::v1::subsets::any::schema::diff::ProgramReportsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ReportRecord");
    patch_through_program!(item, inverse, reports, crate::standards::v1::subsets::any::schema::diff::ProgramReportsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn search_filter_patch_round_trips() {
    let mut item = SearchFilter {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("searchfilter", "Base SearchFilter"), "Base SearchFilter") },
        filter_name: String::new(),
        filter_description: Some(TextField::default()),
        keywords: Vec::new(),
        categories: Vec::new(),
        owner_ids: Vec::new(),
        statuses: Vec::new(),
        priorities: Vec::new(),
        sources: Vec::new(),
        date_from: Some(String::new()),
        date_to: Some(String::new()),
        entity_kinds: Vec::new(),
        tag_filters: Vec::new(),
        sort_field: Some(String::new()),
        sort_direction: Some(String::new()),
        is_public: false,
        created_by: Some(EntityId::new_serial("base51", "base51")),
        last_used: Some(String::new()),
        use_count: 0,
        pinned: false,
    };
    let original = item.clone();
    let patch = SearchFilterPatch {
        name: Some("Patched SearchFilter".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        filter_name: Some("patched-51".to_string()),
        filter_description: Some(TextField::plain("patched-51")),
        keywords: Some(vec!["patched-51".to_string()]),
        categories: Some(vec!["patched-51".to_string()]),
        owner_ids: Some(vec![EntityId::new_serial("new51", "new51")]),
        statuses: Some(vec![LifecycleStatus::Proposed]),
        priorities: Some(vec![Priority::Essential]),
        sources: Some(vec!["patched-51".to_string()]),
        date_from: Some("patched-51".to_string()),
        date_to: Some("patched-51".to_string()),
        entity_kinds: Some(vec!["patched-51".to_string()]),
        tag_filters: Some(vec!["patched-51".to_string()]),
        sort_field: Some("patched-51".to_string()),
        sort_direction: Some("patched-51".to_string()),
        is_public: Some(true),
        created_by: Some(EntityId::new_serial("new51", "new51")),
        last_used: Some("patched-51".to_string()),
        use_count: Some(7),
        pinned: Some(true),
    };
    patch_through_program!(item, patch, search_filters, crate::standards::v1::subsets::any::schema::diff::ProgramSearchFiltersDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched SearchFilter");
    patch_through_program!(item, inverse, search_filters, crate::standards::v1::subsets::any::schema::diff::ProgramSearchFiltersDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
