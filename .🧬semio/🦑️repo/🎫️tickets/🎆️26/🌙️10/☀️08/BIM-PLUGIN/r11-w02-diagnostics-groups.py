import sys
E, D = sys.argv[1], sys.argv[2]
def patch(p, pairs):
    s = open(p, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (p, old, s.count(old))
        s = s.replace(old, new)
    open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(E + "/📌️panels/🚨️diagnostics/🦀️.rs", [
("""//#endregion 🔖️Grouping
""", """
/// ⚖️ The canonical JSON of the groups of every severity, `{ <severity>: [ { storey, kinds: { <category>: <count> } } ] }` in the order of the panel: the table the third-party oracle recomputes from the exported findings.
pub fn groups_json(snapshot: &ModelSnapshot, found: &[Diagnostic]) -> String {
    use semio_framework_plugin::DslValue;
    let rows = |severity: Severity| {
        DslValue::Array(
            groups(snapshot, found, severity)
                .into_iter()
                .map(|group| {
                    let kinds = DslValue::object(group.kinds.iter().map(|kind| (kind.category.to_string(), DslValue::float(kind.findings.len() as f64))));
                    DslValue::object([("storey".to_string(), group.storey.map_or(DslValue::Null, DslValue::String)), ("kinds".to_string(), kinds)])
                })
                .collect(),
        )
    };
    semio_framework_pack_json::to_json_string(&DslValue::object(SEVERITIES.into_iter().map(|severity| (severity_key(severity).to_string(), rows(severity)))))
}
//#endregion 🔖️Grouping
"""),
])
patch(E + "/📌️panels/🚨️diagnostics/🧪️tests/🔬️unit/🦀️.rs", [
("""#[semio_framework_async_macros::async_test]
async fn storeys_follow_their_level""", """#[semio_framework_async_macros::async_test]
async fn the_group_table_lists_the_severities_in_order_with_a_count_per_kind() {
    let (snapshot, inference) = model(DEFECTS);
    let table: serde_json::Value = serde_json::from_str(&groups_json(&snapshot, &inference.diagnostics)).expect("JSON");
    assert_eq!(table.as_object().expect("an object").keys().cloned().collect::<std::collections::BTreeSet<_>>(), ["error", "note", "warning"].map(String::from).into());
    let total: f64 = table.as_object().expect("an object").values().flat_map(|rows| rows.as_array().expect("rows").iter()).flat_map(|row| row["kinds"].as_object().expect("kinds").values()).map(|count| count.as_f64().expect("count")).sum();
    assert_eq!(total as usize, inference.diagnostics.len(), "every finding is counted once");
}

#[semio_framework_async_macros::async_test]
async fn storeys_follow_their_level"""),
])
patch(D + "/🦀️.rs", [
("""    /// 📊️ Every finding read back from the CSV export of the model.""", """    /// 🚨️ The groups of the diagnostics panel of the model: per severity, the storeys by level and the kinds of finding with their counts.
    pub fn panel_groups(ctx: &Context) -> Result<Outcome, String> {
        use semio_s_artifact_bim_model::editor::bim::panels::diagnostics::groups_json;
        use semio_s_artifact_bim_model::ModelInference;
        let snapshot = snapshot_of(ctx)?;
        let inferred = <ModelInference as protocol::Inference<semio_s_artifact_bim_model::ModelSnapshot>>::infer(&snapshot).map_err(|error| error.to_string())?;
        let table = groups_json(&snapshot, &inferred.diagnostics);
        let parsed = parse_json(&table)?;
        Ok(Outcome::with_raw(table.into_bytes(), parsed))
    }

    /// 📊️ Every finding read back from the CSV export of the model."""),
(""".subject("diagnostics-export-csv", subject::export_csv);""", """.subject("diagnostics-export-csv", subject::export_csv).subject("diagnostics-panel-groups", subject::panel_groups);"""),
])
