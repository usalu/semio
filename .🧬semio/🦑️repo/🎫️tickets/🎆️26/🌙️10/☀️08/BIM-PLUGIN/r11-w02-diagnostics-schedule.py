import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new, n=1):
    global s
    assert s.count(old) == n, (old, s.count(old))
    s = s.replace(old, new)
rep('''fn table(columns: Vec<DslValue>, rows: Vec<DslValue>, pick: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&DslValue::Array(rows)));
    if pick {
        scene.domain_id = Some(BIM_ELEMENT_DOMAIN.into());
        scene.domain_granularity_id = Some("wall".into());
    }''', '''fn table(columns: Vec<DslValue>, rows: Vec<DslValue>, granularity: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&DslValue::Array(rows)));
    if let Some(granularity) = granularity {
        scene.domain_id = Some(BIM_ELEMENT_DOMAIN.into());
        scene.domain_granularity_id = Some(granularity.into());
    }''')
rep("table(columns, list_rows(snapshot, inference, labels), false)", "table(columns, list_rows(snapshot, inference, labels), None)")
rep("table(columns, editor_rows(snapshot, &config.schedule, labels), false)", "table(columns, editor_rows(snapshot, &config.schedule, labels), None)")
rep('''    let (columns, rows) = table_parts(schedule, inference, &config.schedule, labels);
    table(columns, rows, true)''', '''    let (columns, rows) = table_parts(schedule, inference, &config.schedule, labels);
    table(columns, rows, row_kind(snapshot, inference, &config.schedule).as_deref())''')
rep('''fn table_surface(''', '''/// 🎯️ The granularity a row of the table is picked under: the kind of the elements its item rows stand for, none when they are not placed elements (a material take-off picks nothing).
pub fn row_kind(snapshot: &ModelSnapshot, inference: &ModelInference, id: &str) -> Option<String> {
    let found = inference.schedules.get(id)?;
    let element = found.rows.iter().filter(|row| row.kind == RowKind::Item).find_map(|row| row.elements.first())?;
    crate::editor::bim::entities::kind_holding(snapshot, element).filter(|row| !row.library).map(|row| row.kind.to_string())
}

fn table_surface(''')
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
