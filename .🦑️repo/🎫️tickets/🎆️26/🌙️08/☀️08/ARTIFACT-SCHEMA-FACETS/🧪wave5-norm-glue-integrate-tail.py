def ensure_artifact_set_snapshot(key, folder, prefix):
    path = NORM / ('🗿️artifacts/%s/🧬️schema/🦀️component.rs' % folder)
    text = path.read_text()
    if 'fn set_snapshot' in text:
        return
    method = (
        '\n    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.\n'
        '    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::%s::%sSnapshot) {\n' % (key, prefix)
        + '        let selected = self.selected_check_index;\n'
        + '        *self = Self::from_snapshot(snapshot);\n'
        + '        self.selected_check_index = selected;\n'
        + '    }\n'
    )
    if '//#endregion 🔖️Conversions' in text:
        text = text.replace('//#endregion 🔖️Conversions', method + '//#endregion 🔖️Conversions', 1)
    else:
        text += '\n' + method
    path.write_text(text)
    print('added set_snapshot', key)

def fix_din4108_layer_list():
    folder = '📕️din4108'
    rust = NORM / ('🗿️artifacts/%s/🔺️diff/🧬️schema/🦀️component.rs' % folder)
    text = rust.read_text()
    text = text.replace('pub layers: Option<Din4108StringList>', 'pub layers: Option<Din4108LayerList>')
    if 'struct Din4108LayerList' not in text:
        text = text.replace(
            'pub struct Din4108StringList { pub values: Vec<String> }',
            'pub struct Din4108StringList { pub values: Vec<String> }\n\n'
            '#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]\n'
            '#[serde(rename_all = "camelCase", default)]\n'
            'pub struct Din4108LayerList { pub values: Vec<crate::artifacts::din4108::LayerDocument> }',
        )
    rust.write_text(text)
    for name in ['🟦️component.ts', '🔣️component.json', '🔗️component.graphql', '🛰️component.proto']:
        p = NORM / ('🗿️artifacts/%s/🔺️diff/🧬️schema' % folder) / name
        if p.exists():
            p.write_text(p.read_text().replace('Din4108StringList', 'Din4108LayerList'))
    snap = NORM / ('🗿️artifacts/%s/📸️snapshot/🧬️schema/🦀️component.rs' % folder)
    st = snap.read_text()
    if 'LayerDocument {' in st and 'use crate::artifacts::din4108::LayerDocument' not in st:
        st = st.replace(
            'use crate::document::ClimateZoneDe;',
            'use crate::artifacts::din4108::LayerDocument;\nuse crate::document::ClimateZoneDe;',
        )
        snap.write_text(st)
    print('fixed din4108 layer list')

def migrate_app_command(key, folder, prefix):
    app = NORM / ('🎛️apps/%s' % folder)
    old = app / '🎮️commands/📤️set-document'
    new = app / '🎮️commands/📤️set-snapshot'
    if old.exists() and not new.exists():
        shutil.move(str(old), str(new))
    elif old.exists() and new.exists():
        shutil.rmtree(old)
    cmd = new / '🦀️component.rs'
    cmd.parent.mkdir(parents=True, exist_ok=True)
    parts = []
    parts.append('//! 📤️ %s play app command — replace the whole compliance document.\n\n' % prefix)
    parts.append('use crate::artifacts::%s::op::%sMutation;\n' % (key, prefix))
    parts.append('use crate::artifacts::%s::%sSnapshot;\n' % (key, prefix))
    parts.append('use crate::config::{NormConfig, NormConfigMutation};\n')
    parts.append('use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};\n')
    parts.append('use serde::{Deserialize, Serialize};\n\n')
    parts.append('//#region 🔖️Payload\n')
    parts.append('#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]\n')
    parts.append('#[dsl(keyword = "set-snapshot")]\n')
    parts.append('pub struct SetSnapshot {\n')
    parts.append('    #[dsl(block)]\n')
    parts.append('    pub snapshot: %sSnapshot,\n' % prefix)
    parts.append('}\n')
    parts.append('//#endregion 🔖️Payload\n\n')
    parts.append('//#region 🔖️Handler\n')
    parts.append('pub fn handle(payload: &SetSnapshot, _doc: &DocumentView<'_, %sSnapshot>, _cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<%sMutation, NormConfigMutation>, Fault> {\n' % (prefix, prefix))
    parts.append('    crate::app_surface::commit_snapshot(%sMutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")\n' % prefix)
    parts.append('}\n')
    parts.append('//#endregion 🔖️Handler\n\n')
    parts.append('//#region 🧪️Tests\n')
    parts.append('#[cfg(test)]\n')
    parts.append('mod tests {\n')
    parts.append('    use super::*;\n')
    parts.append('    use crate::artifacts::%s::op::%sMutation;\n' % (key, prefix))
    parts.append('    use semio_framework_plugin::HistoryView;\n\n')
    parts.append('    #[test]\n')
    parts.append('    fn handle_commits_the_payload_document_under_its_action_id() {\n')
    parts.append('        let projection = %sSnapshot::default();\n' % prefix)
    parts.append('        let config = NormConfig::default();\n')
    parts.append('        let emit = handle(\n')
    parts.append('            &SetSnapshot { snapshot: %sSnapshot::default() },\n' % prefix)
    parts.append('            &DocumentView { snapshot: &projection, history: &HistoryView::empty() },\n')
    parts.append('            &ConfigView { snapshot: &config },\n')
    parts.append('        )\n')
    parts.append('        .expect("handle");\n')
    parts.append('        assert_eq!(emit.document_mutations, vec![%sMutation::SetSnapshot { snapshot: %sSnapshot::default() }]);\n' % (prefix, prefix))
    parts.append('        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));\n')
    parts.append('        assert!(emit.config_mutations.is_empty());\n')
    parts.append('    }\n')
    parts.append('}\n')
    parts.append('//#endregion 🧪️Tests\n')
    cmd.write_text(''.join(parts))

def patch_app_root(key, folder, prefix):
    path = NORM / ('🎛️apps/%s/🦀️component.rs' % folder)
    text = path.read_text()
    text = text.replace('set_document', 'set_snapshot')
    text = text.replace('"setDocument" as "set-document" => set_snapshot::SetDocument', '"setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot')
    text = text.replace('set_snapshot::SetDocument', 'set_snapshot::SetSnapshot')
    text = text.replace('SetDocument(', 'SetSnapshot(')
    text = text.replace('SetDocument { document:', 'SetSnapshot { snapshot:')
    text = text.replace('use crate::artifacts::%s::Document;' % key, 'use crate::artifacts::%s::%sSnapshot;' % (key, prefix))
    text = text.replace('for Document,', 'for %sSnapshot,' % prefix)
    text = text.replace('type Projection =', 'type Snapshot =')
    text = text.replace('fn initial_projection', 'fn initial_snapshot')
    text = text.replace('Document::default()', '%sSnapshot::default()' % prefix)
    text = text.replace('DocumentView<'_, Document>', 'DocumentView<'_, %sSnapshot>' % prefix)
    text = text.replace('.mutation("setDocument"', '.mutation("setSnapshot"')
    text = text.replace('"Set Document"', '"Set Snapshot"')
    text = text.replace('vec!["setDocument"', 'vec!["setSnapshot"')
    text = text.replace('doc.projection', 'doc.snapshot')
    text = text.replace('cfg.projection', 'cfg.snapshot')
    text = text.replace('app.projection()', 'app.snapshot()')
    # Replace remaining bare Document type refs cautiously
    text = re.sub(r'(?<![A-Za-z])Document(?![A-Za-z])', '%sSnapshot' % prefix, text)
    text = text.replace('%sSnapshotApp' % prefix, 'DocumentApp')
    text = text.replace('Vcs%sSnapshotApp' % prefix, 'VcsDocumentApp')
    text = text.replace('%sSnapshotView' % prefix, 'DocumentView')
    text = text.replace('create_%sSnapshot_envelope' % prefix, 'create_document_envelope')
    text = text.replace('%sSnapshotStore' % prefix, 'DocumentStore')
    text = text.replace('%sSnapshotCommand' % prefix, 'DocumentCommand') if False else text
    # Fix over-replacement of Document in comments/strings carefully later if needed
    path.write_text(text)
    for sub in ['🎮️commands/🧮️evaluate/🦀️component.rs', '🎮️commands/☑️selected-check/🦀️component.rs']:
        p = NORM / ('🎛️apps/%s' % folder) / sub
        if not p.exists():
            continue
        mechanical_rename_rs(p, key, prefix)
        t = p.read_text()
        t = t.replace('commit_document(', 'commit_snapshot(')
        if 'evaluate' in sub:
            t = re.sub(
                r'commit_snapshot\((doc\.snapshot\.clone\(\)), "evaluate"\)',
                r'commit_snapshot(%sMutation::SetSnapshot { snapshot: \1 }, "evaluate")' % prefix,
                t,
            )
        p.write_text(t)
    print('patched app', key)

def patch_app_surface():
    path = APP_SURFACE
    text = path.read_text()
    text = text.replace('cfg.projection.', 'cfg.snapshot.')
    text = text.replace('doc.projection', 'doc.snapshot')
    if 'fn commit_snapshot' not in text:
        helper = (
            '\n/// 📤️ Commit a typed document mutation (typically `XMutation::SetSnapshot { snapshot }`).\n'
            'pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {\n'
            '    Ok(Emit::commit(vec![mutation], description))\n'
            '}\n\n'
        )
        needle = (
            'pub fn commit_document<D>(document: D, description: &str) -> Result<Emit<crate::document::SetDocumentMutation<D>, crate::config::NormConfigMutation>, Fault> {\n'
            '    Ok(Emit::commit(vec![crate::document::SetDocumentMutation::SetDocument { document }], description))\n'
            '}'
        )
        if needle in text:
            text = text.replace(needle, helper + needle)
        else:
            text += helper
    text = text.replace(
        'pub fn projection<\'a, D>(doc: &\'a DocumentView<'_, D>) -> &\'a D {\n    doc.projection\n}',
        'pub fn projection<\'a, D>(doc: &\'a DocumentView<'_, D>) -> &\'a D {\n    doc.snapshot\n}',
    )
    # also handle already partially updated
    text = text.replace('doc.projection', 'doc.snapshot')
    path.write_text(text)
    print('patched app_surface')

def patch_leaf_files(key, folder, prefix):
    art = NORM / ('🗿️artifacts/%s' % folder)
    for path in art.rglob('🦀️component.rs'):
        mechanical_rename_rs(path, key, prefix)
    patch_op(key, folder, prefix)

def ensure_register_schema_all():
    for key, folder, prefix in ARTIFACTS:
        path = NORM / ('🗿️artifacts/%s/⚙️engine/🦀️component.rs' % folder)
        text = path.read_text()
        if 'fn register_artifact_schema' in text:
            if 'register_artifact_schema();' not in text.split('register_pilot_languages',1)[-1][:200]:
                text = text.replace(
                    'pub fn register_pilot_languages() {',
                    'pub fn register_pilot_languages() {\n    register_artifact_schema();',
                    1,
                )
                path.write_text(text)
            continue
        block = (
            '\n\n//#region 🔖️SchemaRegistry\n'
            'use std::sync::{Mutex, OnceLock};\n\n'
            'static SCHEMA_REGISTRY: OnceLock<Mutex<schema::ArtifactSchemaRegistry>> = OnceLock::new();\n\n'
            '/// 📌️ Registers the fifteen handcrafted schema leaves for `s.norm.%s`.\n' % key
            'pub fn register_artifact_schema() {\n'
            '    let registry = SCHEMA_REGISTRY.get_or_init(|| Mutex::new(schema::ArtifactSchemaRegistry::new()));\n'
            '    registry\n'
            '        .lock()\n'
            '        .expect("schema registry")\n'
            '        .register(crate::artifacts::%s::schema::%s_artifact_schema_descriptor());\n' % (key, key)
            '}\n'
            '//#endregion 🔖️SchemaRegistry\n'
        )
        text += block
        text = text.replace(
            'pub fn register_pilot_languages() {',
            'pub fn register_pilot_languages() {\n    register_artifact_schema();',
            1,
        )
        path.write_text(text)
        print('added schema registry', key)

def migrate_din_iso(key, folder, prefix):
    ensure_set_snapshot_mutation_files(key, folder, prefix)
    ensure_artifact_set_snapshot(key, folder, prefix)
    patch_engine(key, folder, prefix)
    patch_leaf_files(key, folder, prefix)
    migrate_app_command(key, folder, prefix)
    patch_app_root(key, folder, prefix)
    print('migrated din-iso', key)

def migrate_partial_apps():
    for key, folder, prefix in ARTIFACTS:
        if key not in NEEDS_APP or key in DIN_ISO:
            continue
        migrate_app_command(key, folder, prefix)
        patch_app_root(key, folder, prefix)
        print('migrated partial app', key)

def main():
    patch_glue()
    patch_cargo()
    patch_index()
    patch_setup()
    patch_app_surface()
    fix_din4108_layer_list()
    for key, folder, prefix in ARTIFACTS:
        if key in DIN_ISO:
            migrate_din_iso(key, folder, prefix)
        else:
            ensure_artifact_set_snapshot(key, folder, prefix)
    migrate_partial_apps()
    ensure_register_schema_all()
    print('DONE')

if __name__ == '__main__':
    main()
