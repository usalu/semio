import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep("use crate::ModelSnapshot;\n", "use crate::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts;\nuse crate::ModelSnapshot;\n")
rep('''/// ⌨️ The entry field of a window''', '''/// 🚨️ The problems line of the status: the findings of the diagnostics by severity, or that there are none. The same line in every window, so the author sees the health of the model wherever they work.
pub fn problems(labels: &BimLabels, counts: &SeverityCounts) -> String {
    if counts.total() == 0 {
        return labels.status_no_problems.as_str().to_string();
    }
    [(counts.error, labels.status_errors), (counts.warning, labels.status_warnings), (counts.info, labels.status_notes)].into_iter().filter(|(count, _)| *count > 0).map(|(count, label)| BimLabels::counted(label, count as usize)).collect::<Vec<_>>().join(" · ")
}

/// ⌨️ The entry field of a window''')
rep('''/// 📟️ The engagement of the addressed window: its status line and, for every window that has gestures, the entry field.
pub fn engagements(snapshot: &ModelSnapshot, view_state: &ViewModel, plan_storey: Option<&str>, selected: usize) -> HashMap<String, WindowEngagement> {''', '''/// 📟️ The engagement of the addressed window: its status line, the problems line and, for every window that has gestures, the entry field.
pub fn engagements(snapshot: &ModelSnapshot, view_state: &ViewModel, plan_storey: Option<&str>, selected: usize, found: &SeverityCounts) -> HashMap<String, WindowEngagement> {''')
rep('''    let text = status(snapshot, view_state, plan_storey, selected);
''', '''    let text = status(snapshot, view_state, plan_storey, selected);
    let found = problems(bim_labels(view_state), found);
''')
rep('''            status: Some(vec![WindowEngagementStatus { id: format!("{window}.status"), text }]),''', '''            status: Some(vec![WindowEngagementStatus { id: format!("{window}.status"), text }, WindowEngagementStatus { id: format!("{window}.problems"), text: found }]),''')
open(p, 'w', encoding='utf-8', newline='').write(s)

p2 = sys.argv[2]
s = open(p2, encoding='utf-8', newline='').read()
rep('''        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), 0)''', '''        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), 0, &problems_of(doc))''')
rep('''        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), selected)''', '''        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), selected, &problems_of(doc))''')
rep('''fn instance_of(doc: &ArtifactView<'_, ModelSnapshot>) -> Option<u32> {
    doc.render_operation().map(|operation| operation.app_instance_id)
}
''', '''fn instance_of(doc: &ArtifactView<'_, ModelSnapshot>) -> Option<u32> {
    doc.render_operation().map(|operation| operation.app_instance_id)
}

/// 🚨️ The findings of the model by severity, from the instance's inference (the diagnostic index).
fn problems_of(doc: &ArtifactView<'_, ModelSnapshot>) -> crate::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts {
    crate::editor::bim::inference::with_inference(instance_of(doc), doc.snapshot, |inference| inference.diagnostic_index.total)
}
''')
open(p2, 'w', encoding='utf-8', newline='').write(s)
