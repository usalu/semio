from pathlib import Path

space = next(Path("/Users/ueli/Documents/semio").glob("*/🔌️plugins/🪐️space"))

# Update HomeSpaceRow literals in tests
for p in space.rglob("*.rs"):
    if "target" in p.parts:
        continue
    t = p.read_text(encoding="utf-8")
    if "HomeSpaceRow {" not in t:
        continue
    orig = t
    # hub rows with origin hub
    t = t.replace(
        'origin: "hub", role: None }',
        'origin: "hub", data_class: "persistedShared", role: None }',
    )
    t = t.replace(
        'origin: "hub", role: Some(crate::DirectorySpaceRole::Author) }',
        'origin: "hub", data_class: "persistedShared", role: Some(crate::DirectorySpaceRole::Author) }',
    )
    t = t.replace(
        'origin: "local", role: None }',
        'origin: "local", data_class: "persistedLocalOnly", role: None }',
    )
    # multiline variants with origin then role on next lines already handled by core; handle:
    # origin: "hub",\n            role:
    if 'data_class:' not in t and 'origin: "hub"' in t:
        t = t.replace('origin: "hub",\n', 'origin: "hub",\n            data_class: "persistedShared",\n')
        t = t.replace('origin: "local",\n', 'origin: "local",\n            data_class: "persistedLocalOnly",\n')
    if t != orig:
        p.write_text(t)
        print("updated literals", p)

# Terminology labels
term = next(space.rglob("*/🏠️home/**/🗣️terminology/🦀️.rs"))
tt = term.read_text()
if "action_promote" not in tt:
    tt = tt.replace(
        '        action_manage: native_en "manage", native_de "verwalten", reuse_en "manage", reuse_de "verwalten";\n    }',
        '        action_manage: native_en "manage", native_de "verwalten", reuse_en "manage", reuse_de "verwalten";\n'
        '        action_promote: native_en "Promote to hub", native_de "Zum Hub hochstufen", reuse_en "Promote to hub", reuse_de "Zum Hub hochstufen";\n'
        '        action_persist: native_en "Persist locally", native_de "Lokal speichern", reuse_en "Persist locally", reuse_de "Lokal speichern";\n'
        '        ephemeral_share_blocked: native_en "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.", native_de "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.", reuse_en "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.", reuse_de "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.";\n'
        "    }",
    )
    term.write_text(tt)
    print("terminology patched", term)
else:
    print("terminology already patched")

# row_actions in home main
main = next(p for p in space.rglob("*/explore/**/🏠️main/🦀️.rs") if "tests" not in p.parts)
mt = main.read_text()
old_actions = '''fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<TableRowAction>> {
    let mut actions = semio_framework_plugin::UiFixedList::default();
    actions.try_push(home_row_action(IconName::FolderOpen, labels.action_open, "openSpace", &row.id)?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
    if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {
        for action in [
            home_row_action(IconName::Pencil, labels.action_rename, "renameSpace", &row.id)?,
            home_row_action(IconName::Link, labels.action_share, "shareSpace", &row.id)?,
            home_row_action(IconName::Trash2, labels.action_delete, "deleteSpace", &row.id)?,
            home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)?,
        ] {
            actions.try_push(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
    }
    Ok(actions)
}'''
new_actions = '''fn row_actions(labels: &SHomeLabels, row: &crate::HomeSpaceRow) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<TableRowAction>> {
    let mut actions = semio_framework_plugin::UiFixedList::default();
    actions.try_push(home_row_action(IconName::FolderOpen, labels.action_open, "openSpace", &row.id)?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
    if row.data_class == "ephemeralLocalOnly" {
        for action in [
            home_row_action(IconName::Upload, labels.action_promote, "promoteToHubSpace", &row.id)?,
            home_row_action(IconName::Save, labels.action_persist, "persistLocally", &row.id)?,
        ] {
            actions.try_push(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
        return Ok(actions);
    }
    if row.origin == "hub" && row.role == Some(crate::DirectorySpaceRole::Author) {
        for action in [
            home_row_action(IconName::Pencil, labels.action_rename, "renameSpace", &row.id)?,
            home_row_action(IconName::Link, labels.action_share, "shareSpace", &row.id)?,
            home_row_action(IconName::Trash2, labels.action_delete, "deleteSpace", &row.id)?,
            home_row_action(IconName::Users, labels.action_manage, "manageSpace", &row.id)?,
        ] {
            actions.try_push(action).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.table.row-actions", "fixed row action admission failed"))?;
        }
    }
    Ok(actions)
}'''
if old_actions not in mt:
    raise SystemExit("row_actions block missing")
main.write_text(mt.replace(old_actions, new_actions, 1))
print("row_actions patched", main)
