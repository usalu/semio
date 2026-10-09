import sys, time
p2 = sys.argv[1]
s = open(p2, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
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
for attempt in range(5):
    try:
        open(p2, 'w', encoding='utf-8', newline='').write(s)
        break
    except OSError as error:
        print('retry', error)
        time.sleep(1)
