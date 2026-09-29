/// 🛍️ One Marketplace row's retained record and its TreeItem props, addressed by React's own row id.
fn marketplace_row<'a>(records: &'a [ui_contract::UiNodeRecord], row_id: &str) -> (&'a ui_contract::UiNodeRecord, &'a ui_contract::TreeItemProps) {
    let key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{row_id}");
    let record = records.iter().find(|record| record.key.as_str() == key).unwrap_or_else(|| panic!("🛍️ the row '{row_id}' is materialised"));
    let ui_contract::Component::TreeItem(props) = &record.component else { panic!("🛍️ '{row_id}' is a Tree row") };
    (record, props)
}

/// 🛍️ The verbs one row's actions name, with their disabled flags, in paint order.
fn marketplace_verbs(props: &ui_contract::TreeItemProps) -> Vec<(String, bool)> {
    props.row_actions.iter().map(|action| (action.verb.as_str().to_string(), action.disabled)).collect()
}

/// 🪟️ Points the Marketplace's roster window at `plugin_id`'s row, as the tree window observer would after a scroll.
fn marketplace_window_at(shell: &mut ShellState, plugin_id: &str) {
    let index = shell.marketplace_roster().iter().position(|id| id == plugin_id).unwrap_or_else(|| panic!("🛍️ '{plugin_id}' is on the roster")) as u32;
    let key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
    shell.tree_windows.report(FRAMEWORK_MARKETPLACE_TAB_ID, &[ui_wgpu::wgpu::tree_window::TreeWindowRequest { key, offset: index, rows: 1 }], 20, 0);
}

/// 🛍️ **The Marketplace action lane.** A resident plugin offers React's reload/uninstall pair, a plugin only the activation
/// catalogue knows offers Install — each a ROW ACTION on the row's ONE target (`framework`, `{ pluginId }`) — and the session's own
/// program may never be uninstalled (React's `canUninstall`): its Uninstall paints and announces disabled.
#[test]
fn the_marketplace_leaf_offers_reacts_install_reload_and_uninstall_verbs() {
    let mut shell = display_shell();
    marketplace_window_at(&mut shell, "space");
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let (_, space) = marketplace_row(&records, "framework.marketplace.plugin.space");
    assert_eq!(marketplace_verbs(space), vec![("reloadPlugin".to_string(), false), ("uninstallPlugin".to_string(), true)], "🛍️ the session's OWN program is never uninstallable — React's `canUninstall`");
    let target = serde_json::to_value(space.target.as_ref().expect("🎯️ the row's one target")).expect("target json");
    assert_eq!((target["scope"].as_str(), target["args"]["pluginId"].as_str()), (Some("framework"), Some("space")), "🎯️ every verb fires on the row's one target: {target}");
    assert!(target.get("activation").is_none(), "🎯️ an action-only row has no activation verb");

    let installable = crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter().map(|(_, plugin_id)| *plugin_id).find(|plugin_id| *plugin_id != "space").expect("the catalogue claims a plugin this shell does not hold");
    marketplace_window_at(&mut shell, installable);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let (_, row) = marketplace_row(&records, &format!("framework.marketplace.plugin.{installable}"));
    assert_eq!(marketplace_verbs(row), vec![("installPlugin".to_string(), false)], "🛍️ a non-resident plugin offers React's Install verb");

    assert!(!shell.uninstall_plugin("space"), "🛍️ uninstalling the running program is refused");
    assert_eq!(shell.plugins.len(), 1, "🛍️ and the roster is untouched");
}

/// 🚫️ **The disabled row action law** (U6 × WG11): the retained accessibility projection announces the session program's
/// Uninstall as the virtual button `<row>::row-action::1` with `disabled == true` — present, never actionable.
#[test]
fn the_marketplace_announces_a_disabled_uninstall_as_a_disabled_row_action_button() {
    let mut shell = display_shell();
    marketplace_window_at(&mut shell, "space");
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let mut document = ui_wgpu::wgpu::tree::UiDocumentTree::new(ui_contract::UiDocumentLeaseHeader {
        generation: 1,
        surface: SurfaceId::try_from(FRAMEWORK_MARKETPLACE_TAB_ID).unwrap(),
        revision: ui_contract::UiRevision(1),
        root: records.first().expect("marketplace document root").id,
        layout_epoch: 0,
        node_count: records.len(),
    })
    .expect("marketplace document header");
    for record in records {
        document.try_upsert_record(record).expect("marketplace record admits");
    }
    let mut engine = ui_wgpu::wgpu::Ui::new();
    assert!(engine.publish_document(FRAMEWORK_MARKETPLACE_TAB_ID, document));
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    settle_tree_surface(&mut engine, &mut atlas, FRAMEWORK_MARKETPLACE_TAB_ID, 1);
    let nodes = ui_wgpu::wgpu::accessibility::accessibility_projection(engine.tree(FRAMEWORK_MARKETPLACE_TAB_ID).expect("the mounted Marketplace"));
    let uninstall_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::1");
    let uninstall = nodes.iter().find(|node| node.key == uninstall_key).unwrap_or_else(|| panic!("🚫️ the Uninstall row action is announced: {:?}", nodes.iter().map(|node| node.key.as_str()).collect::<Vec<_>>()));
    assert_eq!(uninstall.role, "button");
    assert!(uninstall.disabled && !uninstall.actionable, "🚫️ a disabled row action announces disabled and is never actionable");
    let reload = nodes.iter().find(|node| node.key == format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/framework.marketplace.plugin.space::row-action::0")).expect("the Reload row action");
    assert!(!reload.disabled && reload.actionable, "🔁️ the enabled sibling stays actionable");
}

/// 🪟️ **The windowed roster law** (P5 × U6): the roster is ONE windowed Tree section — whatever its length it projects inside
/// the retained node ceiling, it materialises no more than the body's window budget, and the window the tree window observer
/// reports is exactly the slice the leaf republishes (here: the roster's LAST row).
#[test]
fn the_marketplace_streams_its_roster_as_one_windowed_section() {
    let mut shell = display_shell();
    let section_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
    let section = |records: &[ui_contract::UiNodeRecord]| -> (ui_contract::TreeWindow, usize) {
        let record = records.iter().find(|record| record.key.as_str() == section_key).expect("🪟️ the roster section");
        let ui_contract::Component::TreeSection(props) = &record.component else { panic!("🪟️ a Tree section") };
        (*props.window.as_ref().expect("🪟️ the roster section is windowed"), record.children.len())
    };
    let roster = shell.marketplace_roster();
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("🪟️ the whole roster projects inside the retained node ceiling");
    let (window, rows) = section(&records);
    assert_eq!((window.total as usize, window.offset), (roster.len(), 0), "🪟️ the window spans the whole roster and opens at its first row");
    assert!(rows <= MARKETPLACE_WINDOW_NODE_BUDGET && rows <= roster.len(), "🪟️ the first paint materialises inside the window budget: {rows}");

    let last = roster.last().expect("the fixture roster holds plugins").clone();
    marketplace_window_at(&mut shell, &last);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("🪟️ the scrolled roster projects");
    let (window, rows) = section(&records);
    assert_eq!((window.offset as usize, rows), (roster.len() - 1, 1), "🪟️ the reported window is the slice the leaf republishes");
    marketplace_row(&records, &format!("framework.marketplace.plugin.{last}"));
}

#[test]
fn the_marketplace_projects_the_store_record_under_its_declared_host_with_enablement_and_uninstall() {
    let mut shell = display_shell();
    let record: ShellExtensionStoreRecord = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json")).expect("the neutral Store record fixture");
    let host = shell.plugins.first().expect("fixture host").plugin_id.clone();
    shell.project_extension_record(ShellExtensionStoreRecord { extends_host: host.clone(), ..record.clone() }).expect("record admits");
    marketplace_window_at(&mut shell, &host);
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the marketplace body projects");
    let keys: Vec<&str> = records.iter().map(|record| record.key.as_str()).collect();
    for id in ["framework.marketplace.extensions.install", "framework.marketplace.extensions.install.url", "framework.marketplace.extensions.install.file"] {
        assert!(keys.iter().any(|key| key.ends_with(id)), "the Store projection keeps React's Marketplace id {id}, got {keys:?}");
    }
    let (_, extension) = marketplace_row(&records, &format!("framework.marketplace.plugin.{host}.extension.{}", record.extension_id));
    assert_eq!(marketplace_verbs(extension), vec![("setExtensionEnabled".to_string(), false), ("uninstallExtension".to_string(), false)], "an enabled available extension offers disable and uninstall");
    let target = serde_json::to_value(extension.target.as_ref().expect("🎯️ the extension row's one target")).expect("target json");
    assert_eq!((target["args"]["extensionId"].as_str(), target["args"]["enabled"].as_bool()), (Some(record.extension_id.as_str()), Some(false)), "🎯️ the toggle's next state lives in the target: {target}");
    assert_eq!(shell.extensions.len(), 1);
    assert!(shell.extensions[0].enabled);
    assert_eq!(shell.extensions[0].load_status, ShellExtensionLoadStatus::Available);
}

#[test]
fn the_marketplace_keeps_orphaned_and_failed_store_packages_visible_without_executable_controls() {
    let mut shell = display_shell();
    let mut record: ShellExtensionStoreRecord = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🏪️store/📥️installation/🧫️fixtures/🔣️.json")).expect("the neutral Store record fixture");
    record.extends_host = "missing.host".into();
    shell.project_extension_record(record.clone()).expect("record admits");
    shell.extensions[0].load_status = ShellExtensionLoadStatus::Failed("fixture load fault".into());
    let records = panel_ui_records(FRAMEWORK_MARKETPLACE_TAB_ID, &shell.build_marketplace_ui()).expect("the orphaned package projects");
    let keys: Vec<&str> = records.iter().map(|record| record.key.as_str()).collect();
    assert!(keys.iter().any(|key| key.ends_with("framework.marketplace.missing-host.missing.host")), "an absent declared host has its own visible React-parity section, got {keys:?}");
    let (_, extension) = marketplace_row(&records, &format!("framework.marketplace.plugin.missing.host.extension.{}", record.extension_id));
    assert_eq!(marketplace_verbs(extension), vec![("setExtensionEnabled".to_string(), true), ("uninstallExtension".to_string(), false)], "a failed package cannot be enabled until it admits a program, and remains uninstallable");
}
