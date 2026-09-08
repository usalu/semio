mod tests {
    use super::*;
    use crate::sample_plugin;
    use crate::standards::v1::subsets::any::schema::inferences::{export_registers_csv, export_registers_tsv};

    #[semio_framework_async_macros::async_test]
    async fn apply_template_returns_plugin_operations() {
        let mut program = crate::empty_plugin();
        let template = TemplateRecord {
            header: EntityHeader::new(EntityId::new_serial("template", "Clinic Starter"), "Clinic Starter"),
            template_type: "sector".into(),
            sector: Some("healthcare".into()),
            project_type: None,
            version: "1".into(),
            content_ref: None,
            entity_kinds: vec!["stakeholder".into(), "element".into()],
            default_fields: vec!["stakeholder".into(), "room".into(), "requirement".into()],
            checklists: vec!["intake".into()],
            standards: vec!["ISO-41001".into()],
            applicability: Vec::new(),
            author_id: None,
            approval_status: ValidationStatus::Passed,
            usage_count: 0,
            last_applied: None,
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: None,
            source_organization: Some("Semio".into()),
        };
        let operations = apply_template(&mut program, &template);
        assert!(!operations.is_empty());
        assert_eq!(program.stakeholders.len(), 1);
        assert_eq!(program.elements.len(), 1);
        assert_eq!(program.requirements.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn template_ops_replay_on_empty_plugin() {
        let mut source = crate::empty_plugin();
        let template = TemplateRecord {
            header: EntityHeader::new(EntityId::new_serial("template", "Replay"), "Replay"),
            template_type: "sector".into(),
            sector: None,
            project_type: None,
            version: "1".into(),
            content_ref: None,
            entity_kinds: vec!["function".into()],
            default_fields: vec!["function".into()],
            checklists: Vec::new(),
            standards: Vec::new(),
            applicability: Vec::new(),
            author_id: None,
            approval_status: ValidationStatus::Passed,
            usage_count: 0,
            last_applied: None,
            customization_notes: Vec::new(),
            related_knowledge_ids: Vec::new(),
            benchmark_ids: Vec::new(),
            license: None,
            source_organization: None,
        };
        let operations = apply_template(&mut source, &template);
        let mut target = crate::empty_plugin();
        for operation in &operations {
            use protocol::{Mutation, MutationDiff};
            target = operation.diff(&target).diff().apply(&target).expect("template operation applies");
        }
        assert_eq!(target.functions.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn build_report_and_record_persists() {
        let mut program = sample_plugin();
        let before = program.reports.len();
        build_report_and_record(&mut program, ReportKind::AdjacencyMatrix);
        assert_eq!(program.reports.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn run_analysis_and_record_persists() {
        let mut program = sample_plugin();
        let before = program.analyses.len();
        run_analysis_and_record(&mut program, AnalysisKind::Risk);
        assert_eq!(program.analyses.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn csv_round_trip_preserves_element_names() {
        let program = sample_plugin();
        let csv = export_registers_csv(&program).expect("csv export");
        let mut reloaded = crate::empty_plugin();
        import_registers_csv(&mut reloaded, &csv, MergeStrategy::Upsert).expect("csv import");
        assert_eq!(reloaded.elements.len(), program.elements.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn quoted_csv_parses_commas_in_name() {
        let csv = "register,id,name,status,priority,tags,source\nelements,e1,\"Room, A\",Draft,Preferred,,src\n";
        let snapshot = stdio_csv::schema::snapshot::decode_csv_with(csv, true);
        let rows = csv_snapshot_to_rows(&snapshot).expect("parse");
        assert_eq!(rows[0].name, "Room, A");
        assert_eq!(rows[0].source, "src");
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_import_is_rejected() {
        let csv = "register,id,name,status,priority,tags,source\nelements,e1,A,Draft,Preferred,,\nelements,e1,B,Draft,Preferred,,\n";
        let mut program = crate::empty_plugin();
        assert!(import_registers_csv(&mut program, csv, MergeStrategy::Upsert).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn tsv_round_trip_preserves_element_names() {
        let program = sample_plugin();
        let tsv = export_registers_tsv(&program).expect("tsv export");
        let mut reloaded = crate::empty_plugin();
        import_registers_tsv(&mut reloaded, &tsv, MergeStrategy::Upsert).expect("tsv import");
        assert_eq!(reloaded.elements.len(), program.elements.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn trace_chain_follows_links() {
        let mut program = sample_plugin();
        let a = program.elements[0].header.id.clone();
        let b = program.elements[1].header.id.clone();
        add_trace_link(&mut program, a.clone(), b.clone(), TraceKind::FunctionToProgramElement);
        let chain = trace_chain(&mut program, &a);
        assert_eq!(chain.links.len(), 1);
        assert_eq!(chain.links[0].to_id, b);
    }

    #[semio_framework_async_macros::async_test]
    async fn trace_impact_collects_upstream() {
        let mut program = sample_plugin();
        let req_id = EntityId::new_serial("requirement", "requirement");
        let elem_id = program.elements[0].header.id.clone();
        add_trace_link(&mut program, req_id.clone(), elem_id.clone(), TraceKind::ObjectiveToRequirement);
        let impact = trace_impact(&mut program, &elem_id);
        assert!(impact.upstream_ids.contains(&req_id));
    }
}
