//! 📒️ Patch round-trip tests of the record registers: status, workshops, surveys, issues, audit events, templates, knowledge, benchmarks, assumptions, constraints, compliance, approvals, meetings and governance.

use super::*;

/// 🧪️ Commits `$patch` onto `$item` the only way a patch is ever committed: as a `modification` of a one-row program through the central applier.
macro_rules! patch_through_table {
    ($item:ident, $patch:expr, $payload:ident, $field:ident, $delta:ty) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$payload.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($item.header.id.0.clone(), $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff(&diff, &program).expect("the patch commits through the central applier").$payload.remove(0);
    }};
}

macro_rules! patch_through_program {
    ($item:ident, $patch:expr, $field:ident, $delta:ty, $id:expr) => {{
        let mut program = crate::standards::v1::subsets::any::schema::snapshot::empty_plugin();
        program.$field.push($item.clone());
        let diff = crate::standards::v1::subsets::any::schema::diff::ProgramDiff { $field: Some(<$delta>::modification($id, $patch.clone())), ..Default::default() };
        $item = protocol::apply_diff(&diff, &program).expect("the patch commits through the central applier").$field.remove(0);
    }};
}

#[semio_framework_async_macros::async_test]
async fn status_record_patch_round_trips() {
    let mut item = StatusRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("statusrecord", "Base StatusRecord"), "Base StatusRecord") },
        subject_id: EntityId::new_serial("base52", "base52"),
        subject_kind: String::new(),
        record_status: LifecycleStatus::Draft,
        previous_status: Some(LifecycleStatus::Draft),
        changed_by: Some(EntityId::new_serial("base52", "base52")),
        changed_at: Some(String::new()),
        reason: Some(TextField::default()),
        blockers: Vec::new(),
        next_actions: Vec::new(),
        due_date: Some(String::new()),
        progress_percent: Some(0.0),
        health: Some(String::new()),
        escalation_level: Some(String::new()),
        related_issue_ids: Vec::new(),
        related_risk_ids: Vec::new(),
        milestone_id: Some(EntityId::new_serial("base52", "base52")),
        reporting_period: Some(String::new()),
        status_notes: Vec::new(),
    };
    let original = item.clone();
    let patch = StatusRecordPatch {
        name: Some("Patched StatusRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        subject_id: Some(EntityId::new_serial("new52", "new52")),
        subject_kind: Some("patched-52".to_string()),
        record_status: Some(LifecycleStatus::Proposed),
        previous_status: Some(LifecycleStatus::Proposed),
        changed_by: Some(EntityId::new_serial("new52", "new52")),
        changed_at: Some("patched-52".to_string()),
        reason: Some(TextField::plain("patched-52")),
        blockers: Some(vec!["patched-52".to_string()]),
        next_actions: Some(vec!["patched-52".to_string()]),
        due_date: Some("patched-52".to_string()),
        progress_percent: Some(42.0),
        health: Some("patched-52".to_string()),
        escalation_level: Some("patched-52".to_string()),
        related_issue_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
        related_risk_ids: Some(vec![EntityId::new_serial("new52", "new52")]),
        milestone_id: Some(EntityId::new_serial("new52", "new52")),
        reporting_period: Some("patched-52".to_string()),
        status_notes: Some(vec![TaggedNote { tag: "new52".into(), text: "new-note52".into() }]),
    };
    patch_through_program!(item, patch, status_records, crate::standards::v1::subsets::any::schema::diff::ProgramStatusRecordsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched StatusRecord");
    patch_through_program!(item, inverse, status_records, crate::standards::v1::subsets::any::schema::diff::ProgramStatusRecordsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn workshop_patch_round_trips() {
    let mut item = Workshop {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("workshop", "Base Workshop"), "Base Workshop") },
        workshop_type: String::new(),
        objectives: Vec::new(),
        agenda: Vec::new(),
        facilitator_id: Some(EntityId::new_serial("base53", "base53")),
        participants: Vec::new(),
        scheduled_start: Some(String::new()),
        scheduled_end: Some(String::new()),
        location: Some(String::new()),
        materials: Vec::new(),
        methods: Vec::new(),
        outputs: Vec::new(),
        decisions: Vec::new(),
        issues: Vec::new(),
        follow_up_actions: Vec::new(),
        feedback: Vec::new(),
        recording_ref: Some(String::new()),
        budget: Some(0.0),
        workshop_status: LifecycleStatus::Draft,
        survey_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = WorkshopPatch {
        name: Some("Patched Workshop".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        workshop_type: Some("patched-53".to_string()),
        objectives: Some(vec!["patched-53".to_string()]),
        agenda: Some(vec!["patched-53".to_string()]),
        facilitator_id: Some(EntityId::new_serial("new53", "new53")),
        participants: Some(vec![EntityId::new_serial("new53", "new53")]),
        scheduled_start: Some("patched-53".to_string()),
        scheduled_end: Some("patched-53".to_string()),
        location: Some("patched-53".to_string()),
        materials: Some(vec!["patched-53".to_string()]),
        methods: Some(vec!["patched-53".to_string()]),
        outputs: Some(vec!["patched-53".to_string()]),
        decisions: Some(vec![EntityId::new_serial("new53", "new53")]),
        issues: Some(vec![EntityId::new_serial("new53", "new53")]),
        follow_up_actions: Some(vec!["patched-53".to_string()]),
        feedback: Some(vec![TaggedNote { tag: "new53".into(), text: "new-note53".into() }]),
        recording_ref: Some("patched-53".to_string()),
        budget: Some(42.0),
        workshop_status: Some(LifecycleStatus::Proposed),
        survey_ids: Some(vec![EntityId::new_serial("new53", "new53")]),
    };
    patch_through_program!(item, patch, workshops, crate::standards::v1::subsets::any::schema::diff::ProgramWorkshopsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Workshop");
    patch_through_program!(item, inverse, workshops, crate::standards::v1::subsets::any::schema::diff::ProgramWorkshopsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn survey_patch_round_trips() {
    let mut item = Survey {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("survey", "Base Survey"), "Base Survey") },
        survey_type: String::new(),
        title: String::new(),
        objectives: Vec::new(),
        questions: Vec::new(),
        target_audience: Vec::new(),
        distribution_channels: Vec::new(),
        launch_date: Some(String::new()),
        close_date: Some(String::new()),
        response_count: 0,
        response_rate: Some(0.0),
        findings: Vec::new(),
        themes: Vec::new(),
        recommendations: Vec::new(),
        confidentiality: Some(String::new()),
        consent_process: Vec::new(),
        analysis_id: Some(EntityId::new_serial("base54", "base54")),
        workshop_id: Some(EntityId::new_serial("base54", "base54")),
        owner_id: Some(EntityId::new_serial("base54", "base54")),
        survey_status: LifecycleStatus::Draft,
    };
    let original = item.clone();
    let patch = SurveyPatch {
        name: Some("Patched Survey".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        survey_type: Some("patched-54".to_string()),
        title: Some("patched-54".to_string()),
        objectives: Some(vec!["patched-54".to_string()]),
        questions: Some(vec!["patched-54".to_string()]),
        target_audience: Some(vec![EntityId::new_serial("new54", "new54")]),
        distribution_channels: Some(vec!["patched-54".to_string()]),
        launch_date: Some("patched-54".to_string()),
        close_date: Some("patched-54".to_string()),
        response_count: Some(7),
        response_rate: Some(42.0),
        findings: Some(vec!["patched-54".to_string()]),
        themes: Some(vec!["patched-54".to_string()]),
        recommendations: Some(vec!["patched-54".to_string()]),
        confidentiality: Some("patched-54".to_string()),
        consent_process: Some(vec!["patched-54".to_string()]),
        analysis_id: Some(EntityId::new_serial("new54", "new54")),
        workshop_id: Some(EntityId::new_serial("new54", "new54")),
        owner_id: Some(EntityId::new_serial("new54", "new54")),
        survey_status: Some(LifecycleStatus::Proposed),
    };
    patch_through_program!(item, patch, surveys, crate::standards::v1::subsets::any::schema::diff::ProgramSurveysDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Survey");
    patch_through_program!(item, inverse, surveys, crate::standards::v1::subsets::any::schema::diff::ProgramSurveysDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn issue_patch_round_trips() {
    let mut item = Issue {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("issue", "Base Issue"), "Base Issue") },
        issue_type: String::new(),
        summary: TextField::default(),
        issue_description: TextField::default(),
        severity: IssueSeverity::Cosmetic,
        issue_priority: Priority::Mandatory,
        reporter_id: Some(EntityId::new_serial("base55", "base55")),
        assignee_id: Some(EntityId::new_serial("base55", "base55")),
        affected_entity_ids: Vec::new(),
        root_cause: Some(TextField::default()),
        resolution: Some(TextField::default()),
        workaround: Some(TextField::default()),
        due_date: Some(String::new()),
        resolved_date: Some(String::new()),
        related_conflict_ids: Vec::new(),
        related_risk_ids: Vec::new(),
        decision_id: Some(EntityId::new_serial("base55", "base55")),
        comments: Vec::new(),
        attachments: Vec::new(),
        escalation_level: Some(String::new()),
    };
    let original = item.clone();
    let patch = IssuePatch {
        name: Some("Patched Issue".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        issue_type: Some("patched-55".to_string()),
        summary: Some(TextField::plain("patched-55")),
        issue_description: Some(TextField::plain("patched-55")),
        severity: Some(IssueSeverity::Minor),
        issue_priority: Some(Priority::Essential),
        reporter_id: Some(EntityId::new_serial("new55", "new55")),
        assignee_id: Some(EntityId::new_serial("new55", "new55")),
        affected_entity_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        root_cause: Some(TextField::plain("patched-55")),
        resolution: Some(TextField::plain("patched-55")),
        workaround: Some(TextField::plain("patched-55")),
        due_date: Some("patched-55".to_string()),
        resolved_date: Some("patched-55".to_string()),
        related_conflict_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        related_risk_ids: Some(vec![EntityId::new_serial("new55", "new55")]),
        decision_id: Some(EntityId::new_serial("new55", "new55")),
        comments: Some(vec![TaggedNote { tag: "new55".into(), text: "new-note55".into() }]),
        attachments: Some(vec![EntityId::new_serial("new55", "new55")]),
        escalation_level: Some("patched-55".to_string()),
    };
    patch_through_program!(item, patch, issues, crate::standards::v1::subsets::any::schema::diff::ProgramIssuesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Issue");
    patch_through_program!(item, inverse, issues, crate::standards::v1::subsets::any::schema::diff::ProgramIssuesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn audit_event_patch_round_trips() {
    let mut item = AuditEvent {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("auditevent", "Base AuditEvent"), "Base AuditEvent") },
        action: AuditAction::Created,
        actor_id: Some(EntityId::new_serial("base56", "base56")),
        subject_id: EntityId::new_serial("base56", "base56"),
        subject_kind: String::new(),
        timestamp: String::new(),
        details: TextField::default(),
        before_state: Some(String::new()),
        after_state: Some(String::new()),
        ip_address: Some(String::new()),
        client: Some(String::new()),
        session_id: Some(String::new()),
        change_record_id: Some(EntityId::new_serial("base56", "base56")),
        trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56", "tfrom56"), EntityId::new_serial("tto56", "tto56"), TraceKind::FullAuditTrail)),
        success: false,
        error_message: Some(String::new()),
        correlation_id: Some(String::new()),
        compliance_tags: Vec::new(),
        retention_until: Some(String::new()),
    };
    let original = item.clone();
    let patch = AuditEventPatch {
        name: Some("Patched AuditEvent".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        action: Some(AuditAction::Updated),
        actor_id: Some(EntityId::new_serial("new56", "new56")),
        subject_id: Some(EntityId::new_serial("new56", "new56")),
        subject_kind: Some("patched-56".to_string()),
        timestamp: Some("patched-56".to_string()),
        details: Some(TextField::plain("patched-56")),
        before_state: Some("patched-56".to_string()),
        after_state: Some("patched-56".to_string()),
        ip_address: Some("patched-56".to_string()),
        client: Some("patched-56".to_string()),
        session_id: Some("patched-56".to_string()),
        change_record_id: Some(EntityId::new_serial("new56", "new56")),
        trace_link: Some(TraceLink::new(EntityId::new_serial("tfrom56n", "tfrom56n"), EntityId::new_serial("tto56n", "tto56n"), TraceKind::FullAuditTrail)),
        success: Some(true),
        error_message: Some("patched-56".to_string()),
        correlation_id: Some("patched-56".to_string()),
        compliance_tags: Some(vec!["patched-56".to_string()]),
        retention_until: Some("patched-56".to_string()),
    };
    patch_through_program!(item, patch, audit_events, crate::standards::v1::subsets::any::schema::diff::ProgramAuditEventsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched AuditEvent");
    patch_through_program!(item, inverse, audit_events, crate::standards::v1::subsets::any::schema::diff::ProgramAuditEventsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn template_record_patch_round_trips() {
    let mut item = TemplateRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("templaterecord", "Base TemplateRecord"), "Base TemplateRecord") },
        template_type: String::new(),
        sector: Some(String::new()),
        project_type: Some(String::new()),
        version: String::new(),
        content_ref: Some(String::new()),
        entity_kinds: Vec::new(),
        default_fields: Vec::new(),
        checklists: Vec::new(),
        standards: Vec::new(),
        applicability: Vec::new(),
        author_id: Some(EntityId::new_serial("base57", "base57")),
        approval_status: ValidationStatus::Pending,
        usage_count: 0,
        last_applied: Some(String::new()),
        customization_notes: Vec::new(),
        related_knowledge_ids: Vec::new(),
        benchmark_ids: Vec::new(),
        license: Some(String::new()),
        source_organization: Some(String::new()),
    };
    let original = item.clone();
    let patch = TemplateRecordPatch {
        name: Some("Patched TemplateRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        template_type: Some("patched-57".to_string()),
        sector: Some("patched-57".to_string()),
        project_type: Some("patched-57".to_string()),
        version: Some("patched-57".to_string()),
        content_ref: Some("patched-57".to_string()),
        entity_kinds: Some(vec!["patched-57".to_string()]),
        default_fields: Some(vec!["patched-57".to_string()]),
        checklists: Some(vec!["patched-57".to_string()]),
        standards: Some(vec!["patched-57".to_string()]),
        applicability: Some(vec!["patched-57".to_string()]),
        author_id: Some(EntityId::new_serial("new57", "new57")),
        approval_status: Some(ValidationStatus::Passed),
        usage_count: Some(7),
        last_applied: Some("patched-57".to_string()),
        customization_notes: Some(vec!["patched-57".to_string()]),
        related_knowledge_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
        benchmark_ids: Some(vec![EntityId::new_serial("new57", "new57")]),
        license: Some("patched-57".to_string()),
        source_organization: Some("patched-57".to_string()),
    };
    patch_through_program!(item, patch, templates, crate::standards::v1::subsets::any::schema::diff::ProgramTemplatesDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched TemplateRecord");
    patch_through_program!(item, inverse, templates, crate::standards::v1::subsets::any::schema::diff::ProgramTemplatesDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn knowledge_record_patch_round_trips() {
    let mut item = KnowledgeRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("knowledgerecord", "Base KnowledgeRecord"), "Base KnowledgeRecord") },
        topic: String::new(),
        category: String::new(),
        summary: TextField::default(),
        content: TextField::default(),
        sources: Vec::new(),
        references: Vec::new(),
        lessons_learned: Vec::new(),
        best_practices: Vec::new(),
        applicable_sectors: Vec::new(),
        related_entity_kinds: Vec::new(),
        author_ids: Vec::new(),
        expertise_level: Some(String::new()),
        validation_status: ValidationStatus::Pending,
        last_reviewed: Some(String::new()),
        keywords: Vec::new(),
        attachments: Vec::new(),
        citations: Vec::new(),
        usage_count: 0,
    };
    let original = item.clone();
    let patch = KnowledgeRecordPatch {
        name: Some("Patched KnowledgeRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        topic: Some("patched-58".to_string()),
        category: Some("patched-58".to_string()),
        summary: Some(TextField::plain("patched-58")),
        content: Some(TextField::plain("patched-58")),
        sources: Some(vec!["patched-58".to_string()]),
        references: Some(vec!["patched-58".to_string()]),
        lessons_learned: Some(vec!["patched-58".to_string()]),
        best_practices: Some(vec!["patched-58".to_string()]),
        applicable_sectors: Some(vec!["patched-58".to_string()]),
        related_entity_kinds: Some(vec!["patched-58".to_string()]),
        author_ids: Some(vec![EntityId::new_serial("new58", "new58")]),
        expertise_level: Some("patched-58".to_string()),
        validation_status: Some(ValidationStatus::Passed),
        last_reviewed: Some("patched-58".to_string()),
        keywords: Some(vec!["patched-58".to_string()]),
        attachments: Some(vec![EntityId::new_serial("new58", "new58")]),
        citations: Some(vec!["patched-58".to_string()]),
        usage_count: Some(7),
    };
    patch_through_table!(item, patch, knowledge_payload, knowledge, crate::standards::v1::subsets::any::schema::diff::ProgramKnowledgeDelta);
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched KnowledgeRecord");
    patch_through_table!(item, inverse, knowledge_payload, knowledge, crate::standards::v1::subsets::any::schema::diff::ProgramKnowledgeDelta);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn benchmark_record_patch_round_trips() {
    let mut item = BenchmarkRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("benchmarkrecord", "Base BenchmarkRecord"), "Base BenchmarkRecord") },
        benchmark_name: String::new(),
        sector: String::new(),
        metric: String::new(),
        value: 0.0,
        unit: String::new(),
        sample_size: Some(0),
        source: Some(String::new()),
        collection_year: Some(0),
        geography: Some(String::new()),
        building_type: Some(String::new()),
        confidence: Some(String::new()),
        methodology: Some(String::new()),
        applicable_element_kinds: Vec::new(),
        related_requirement_ids: Vec::new(),
        comparison_notes: Vec::new(),
        limitations: Vec::new(),
        license: Some(String::new()),
        knowledge_id: Some(EntityId::new_serial("base59", "base59")),
        last_verified: Some(String::new()),
    };
    let original = item.clone();
    let patch = BenchmarkRecordPatch {
        name: Some("Patched BenchmarkRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        benchmark_name: Some("patched-59".to_string()),
        sector: Some("patched-59".to_string()),
        metric: Some("patched-59".to_string()),
        value: Some(42.0),
        unit: Some("patched-59".to_string()),
        sample_size: Some(7),
        source: Some("patched-59".to_string()),
        collection_year: Some(7),
        geography: Some("patched-59".to_string()),
        building_type: Some("patched-59".to_string()),
        confidence: Some("patched-59".to_string()),
        methodology: Some("patched-59".to_string()),
        applicable_element_kinds: Some(vec!["patched-59".to_string()]),
        related_requirement_ids: Some(vec![EntityId::new_serial("new59", "new59")]),
        comparison_notes: Some(vec!["patched-59".to_string()]),
        limitations: Some(vec!["patched-59".to_string()]),
        license: Some("patched-59".to_string()),
        knowledge_id: Some(EntityId::new_serial("new59", "new59")),
        last_verified: Some("patched-59".to_string()),
    };
    patch_through_table!(item, patch, benchmarks_payload, benchmarks, crate::standards::v1::subsets::any::schema::diff::ProgramBenchmarksDelta);
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched BenchmarkRecord");
    patch_through_table!(item, inverse, benchmarks_payload, benchmarks, crate::standards::v1::subsets::any::schema::diff::ProgramBenchmarksDelta);
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn assumption_patch_round_trips() {
    let mut item = Assumption {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("assumption", "Base Assumption"), "Base Assumption") },
        statement: TextField::default(),
        basis: Some(TextField::default()),
        confidence_level: Some(String::new()),
        impact_if_false: Some(TextField::default()),
        related_entity_ids: Vec::new(),
        validation_status: ValidationStatus::Pending,
        validated_by: Some(EntityId::new_serial("base60", "base60")),
        validation_date: Some(String::new()),
        owner_id: Some(EntityId::new_serial("base60", "base60")),
        review_cycle: Some(String::new()),
        source: Some(String::new()),
        category: Some(String::new()),
        dependencies: Vec::new(),
        mitigation: Vec::new(),
        linked_requirement_ids: Vec::new(),
        linked_risk_ids: Vec::new(),
        expiration_date: Some(String::new()),
        status_notes: Vec::new(),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = AssumptionPatch {
        name: Some("Patched Assumption".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        statement: Some(TextField::plain("patched-60")),
        basis: Some(TextField::plain("patched-60")),
        confidence_level: Some("patched-60".to_string()),
        impact_if_false: Some(TextField::plain("patched-60")),
        related_entity_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        validation_status: Some(ValidationStatus::Passed),
        validated_by: Some(EntityId::new_serial("new60", "new60")),
        validation_date: Some("patched-60".to_string()),
        owner_id: Some(EntityId::new_serial("new60", "new60")),
        review_cycle: Some("patched-60".to_string()),
        source: Some("patched-60".to_string()),
        category: Some("patched-60".to_string()),
        dependencies: Some(vec!["patched-60".to_string()]),
        mitigation: Some(vec!["patched-60".to_string()]),
        linked_requirement_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        linked_risk_ids: Some(vec![EntityId::new_serial("new60", "new60")]),
        expiration_date: Some("patched-60".to_string()),
        status_notes: Some(vec![TaggedNote { tag: "new60".into(), text: "new-note60".into() }]),
        artifact_refs: Some(vec!["patched-60".to_string()]),
    };
    patch_through_program!(item, patch, assumptions, crate::standards::v1::subsets::any::schema::diff::ProgramAssumptionsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched Assumption");
    patch_through_program!(item, inverse, assumptions, crate::standards::v1::subsets::any::schema::diff::ProgramAssumptionsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn constraint_record_patch_round_trips() {
    let mut item = ConstraintRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("constraintrecord", "Base ConstraintRecord"), "Base ConstraintRecord") },
        constraint_type: String::new(),
        summary: TextField::default(),
        severity: RiskLevel::Negligible,
        affected_entity_ids: Vec::new(),
        source: Some(String::new()),
        regulatory_basis: Vec::new(),
        mitigation_options: Vec::new(),
        owner_id: Some(EntityId::new_serial("base61", "base61")),
        effective_date: Some(String::new()),
        expiry_date: Some(String::new()),
        waiver_status: Some(String::new()),
        waiver_approver: Some(EntityId::new_serial("base61", "base61")),
        impact_assessment: Some(TextField::default()),
        resolution_plan: Vec::new(),
        related_requirement_ids: Vec::new(),
        related_decision_ids: Vec::new(),
        monitoring_frequency: Some(String::new()),
        compliance_status: ValidationStatus::Pending,
        exceptions: Vec::new(),
        trace_links: Vec::new(),
        escalation_contact_id: Some(EntityId::new_serial("base61", "base61")),
    };
    let original = item.clone();
    let patch = ConstraintRecordPatch {
        name: Some("Patched ConstraintRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        constraint_type: Some("patched-61".to_string()),
        summary: Some(TextField::plain("patched-61")),
        severity: Some(RiskLevel::Low),
        affected_entity_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        source: Some("patched-61".to_string()),
        regulatory_basis: Some(vec!["patched-61".to_string()]),
        mitigation_options: Some(vec!["patched-61".to_string()]),
        owner_id: Some(EntityId::new_serial("new61", "new61")),
        effective_date: Some("patched-61".to_string()),
        expiry_date: Some("patched-61".to_string()),
        waiver_status: Some("patched-61".to_string()),
        waiver_approver: Some(EntityId::new_serial("new61", "new61")),
        impact_assessment: Some(TextField::plain("patched-61")),
        resolution_plan: Some(vec!["patched-61".to_string()]),
        related_requirement_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        related_decision_ids: Some(vec![EntityId::new_serial("new61", "new61")]),
        monitoring_frequency: Some("patched-61".to_string()),
        compliance_status: Some(ValidationStatus::Passed),
        exceptions: Some(vec!["patched-61".to_string()]),
        trace_links: Some(vec![TraceLink::new(EntityId::new_serial("tfrom61n", "tfrom61n"), EntityId::new_serial("tto61n", "tto61n"), TraceKind::FullAuditTrail)]),
        escalation_contact_id: Some(EntityId::new_serial("new61", "new61")),
    };
    patch_through_program!(item, patch, constraints, crate::standards::v1::subsets::any::schema::diff::ProgramConstraintsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ConstraintRecord");
    patch_through_program!(item, inverse, constraints, crate::standards::v1::subsets::any::schema::diff::ProgramConstraintsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn compliance_record_patch_round_trips() {
    let mut item = ComplianceRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("compliancerecord", "Base ComplianceRecord"), "Base ComplianceRecord") },
        standard_ref: String::new(),
        obligation: TextField::default(),
        compliance_status: ValidationStatus::Pending,
        evidence_refs: Vec::new(),
        auditor_id: Some(EntityId::new_serial("base62", "base62")),
        audit_date: Some(String::new()),
        next_review: Some(String::new()),
        affected_entity_ids: Vec::new(),
        gap_analysis: Vec::new(),
        remediation_plan: Vec::new(),
        owner_id: Some(EntityId::new_serial("base62", "base62")),
        severity: RiskLevel::Negligible,
        regulatory_body: Some(String::new()),
        certification_target: Some(String::new()),
        waiver_status: Some(String::new()),
        related_requirement_ids: Vec::new(),
        monitoring_method: Some(String::new()),
        reporting_frequency: Some(String::new()),
        penalties: Vec::new(),
        corrective_actions: Vec::new(),
        artifact_refs: Vec::new(),
    };
    let original = item.clone();
    let patch = ComplianceRecordPatch {
        name: Some("Patched ComplianceRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        standard_ref: Some("patched-62".to_string()),
        obligation: Some(TextField::plain("patched-62")),
        compliance_status: Some(ValidationStatus::Passed),
        evidence_refs: Some(vec!["patched-62".to_string()]),
        auditor_id: Some(EntityId::new_serial("new62", "new62")),
        audit_date: Some("patched-62".to_string()),
        next_review: Some("patched-62".to_string()),
        affected_entity_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
        gap_analysis: Some(vec!["patched-62".to_string()]),
        remediation_plan: Some(vec!["patched-62".to_string()]),
        owner_id: Some(EntityId::new_serial("new62", "new62")),
        severity: Some(RiskLevel::Low),
        regulatory_body: Some("patched-62".to_string()),
        certification_target: Some("patched-62".to_string()),
        waiver_status: Some("patched-62".to_string()),
        related_requirement_ids: Some(vec![EntityId::new_serial("new62", "new62")]),
        monitoring_method: Some("patched-62".to_string()),
        reporting_frequency: Some("patched-62".to_string()),
        penalties: Some(vec!["patched-62".to_string()]),
        corrective_actions: Some(vec!["patched-62".to_string()]),
        artifact_refs: Some(vec!["patched-62".to_string()]),
    };
    patch_through_program!(item, patch, compliance_records, crate::standards::v1::subsets::any::schema::diff::ProgramComplianceRecordsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ComplianceRecord");
    patch_through_program!(item, inverse, compliance_records, crate::standards::v1::subsets::any::schema::diff::ProgramComplianceRecordsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn approval_record_patch_round_trips() {
    let mut item = ApprovalRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("approvalrecord", "Base ApprovalRecord"), "Base ApprovalRecord") },
        approval_type: String::new(),
        subject_id: EntityId::new_serial("base63", "base63"),
        approver_ids: Vec::new(),
        approval_date: Some(String::new()),
        conditions: Vec::new(),
        approval_status: LifecycleStatus::Draft,
        expiry_date: Some(String::new()),
        delegation_chain: Vec::new(),
        evidence_refs: Vec::new(),
        related_decision_id: Some(EntityId::new_serial("base63", "base63")),
        related_change_id: Some(EntityId::new_serial("base63", "base63")),
        authority_basis: Vec::new(),
        signature_method: Some(String::new()),
        rejection_reason: Some(TextField::default()),
        resubmission_date: Some(String::new()),
        notification_list: Vec::new(),
        workflow_step: Some(String::new()),
        version: Some(String::new()),
        audit_trail_ref: Some(String::new()),
    };
    let original = item.clone();
    let patch = ApprovalRecordPatch {
        name: Some("Patched ApprovalRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        approval_type: Some("patched-63".to_string()),
        subject_id: Some(EntityId::new_serial("new63", "new63")),
        approver_ids: Some(vec![EntityId::new_serial("new63", "new63")]),
        approval_date: Some("patched-63".to_string()),
        conditions: Some(vec!["patched-63".to_string()]),
        approval_status: Some(LifecycleStatus::Proposed),
        expiry_date: Some("patched-63".to_string()),
        delegation_chain: Some(vec![EntityId::new_serial("new63", "new63")]),
        evidence_refs: Some(vec!["patched-63".to_string()]),
        related_decision_id: Some(EntityId::new_serial("new63", "new63")),
        related_change_id: Some(EntityId::new_serial("new63", "new63")),
        authority_basis: Some(vec!["patched-63".to_string()]),
        signature_method: Some("patched-63".to_string()),
        rejection_reason: Some(TextField::plain("patched-63")),
        resubmission_date: Some("patched-63".to_string()),
        notification_list: Some(vec![EntityId::new_serial("new63", "new63")]),
        workflow_step: Some("patched-63".to_string()),
        version: Some("patched-63".to_string()),
        audit_trail_ref: Some("patched-63".to_string()),
    };
    patch_through_program!(item, patch, approvals, crate::standards::v1::subsets::any::schema::diff::ProgramApprovalsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched ApprovalRecord");
    patch_through_program!(item, inverse, approvals, crate::standards::v1::subsets::any::schema::diff::ProgramApprovalsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}

#[semio_framework_async_macros::async_test]
async fn meeting_record_patch_round_trips() {
    let mut item = MeetingRecord {
        header: EntityHeader { description: Some(TextField::plain("base-desc")), ..EntityHeader::new(EntityId::new_serial("meetingrecord", "Base MeetingRecord"), "Base MeetingRecord") },
        meeting_type: String::new(),
        scheduled_date: Some(String::new()),
        duration: Some(String::new()),
        location: Some(String::new()),
        chair_id: Some(EntityId::new_serial("base64", "base64")),
        attendee_ids: Vec::new(),
        agenda_items: Vec::new(),
        minutes: Some(TextField::default()),
        action_items: Vec::new(),
        decisions_made: Vec::new(),
        artifact_refs: Vec::new(),
        follow_up_date: Some(String::new()),
        recording_ref: Some(String::new()),
        quorum_met: false,
        meeting_status: LifecycleStatus::Draft,
        workshop_id: Some(EntityId::new_serial("base64", "base64")),
        stakeholder_ids: Vec::new(),
        requirement_ids: Vec::new(),
        issue_ids: Vec::new(),
        approval_ids: Vec::new(),
    };
    let original = item.clone();
    let patch = MeetingRecordPatch {
        name: Some("Patched MeetingRecord".to_string()),
        description: Some(TextField::plain("desc")),
        status: Some(LifecycleStatus::Approved),
        priority: Some(Priority::Mandatory),
        ownership: Some(Ownership { owner_id: Some(EntityId::new_serial("owner", "owner")), authority_id: None, consultant_ids: Vec::new(), participant_ids: Vec::new() }),
        tags: Some(vec!["tag".to_string()]),
        notes: Some(vec![TaggedNote { tag: "t".into(), text: "n".into() }]),
        timestamps: Some(TimestampMeta { created: "2020-01-01T00:00:00Z".into(), updated: "2020-01-02T00:00:00Z".into(), created_by: None, updated_by: None }),
        meeting_type: Some("patched-64".to_string()),
        scheduled_date: Some("patched-64".to_string()),
        duration: Some("patched-64".to_string()),
        location: Some("patched-64".to_string()),
        chair_id: Some(EntityId::new_serial("new64", "new64")),
        attendee_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        agenda_items: Some(vec!["patched-64".to_string()]),
        minutes: Some(TextField::plain("patched-64")),
        action_items: Some(vec!["patched-64".to_string()]),
        decisions_made: Some(vec![EntityId::new_serial("new64", "new64")]),
        artifact_refs: Some(vec!["patched-64".to_string()]),
        follow_up_date: Some("patched-64".to_string()),
        recording_ref: Some("patched-64".to_string()),
        quorum_met: Some(true),
        meeting_status: Some(LifecycleStatus::Proposed),
        workshop_id: Some(EntityId::new_serial("new64", "new64")),
        stakeholder_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        requirement_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        issue_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
        approval_ids: Some(vec![EntityId::new_serial("new64", "new64")]),
    };
    patch_through_program!(item, patch, meetings, crate::standards::v1::subsets::any::schema::diff::ProgramMeetingsDelta, item.header.id.0.clone());
    let inverse = patch.inverse(&original);
    assert_ne!(item, original);
    assert_eq!(item.header.name, "Patched MeetingRecord");
    patch_through_program!(item, inverse, meetings, crate::standards::v1::subsets::any::schema::diff::ProgramMeetingsDelta, item.header.id.0.clone());
    assert_eq!(item, original);
}
