    /// 🛍️ Every plugin id the `framework.marketplace` leaf lists, in its row order: the plugins this shell HOLDS (`loaded`,
    /// extensions excluded — they nest under their host) and every plugin id the generated activation catalogue claims
    /// (`available`) — the same table [`ShellState::install_plugin`]'s lazy install reads, which is what makes `Install` a real
    /// verb rather than a decoration. Sorted and unique.
    pub(crate) fn marketplace_roster(&self) -> Vec<String> {
        let extension_ids: Vec<&str> = self.extensions.iter().map(|entry| entry.record.extension_id.as_str()).collect();
        let mut ids: Vec<String> = self.plugins.iter().filter(|entry| !extension_ids.contains(&entry.plugin_id.as_str())).map(|entry| entry.plugin_id.clone()).collect();
        for (_, plugin_id) in crate::program_bridge::PLUGIN_ARTIFACT_KIND_ACTIVATIONS.iter() {
            if !ids.iter().any(|id| id.as_str() == *plugin_id) {
                ids.push((*plugin_id).to_string());
            }
        }
        ids.sort();
        ids.dedup();
        ids
    }

    /// 🛍️ The `framework.marketplace` leaf's body — the wgpu twin of React's `buildMarketplaceTree`: one section per plugin
    /// SOURCE, a `label · version · status` row per plugin, and this renderer's own install/reload/uninstall lane.
    ///
    /// 🎯️ Every row carries its verbs as ROW ACTIONS on ONE target (U6's row model): a plugin row targets
    /// `framework` with `{ pluginId }` and names `installPlugin` or `reloadPlugin`/`uninstallPlugin`; an extension row targets
    /// `{ extensionId, enabled: !enabled }` and names `setExtensionEnabled`/`uninstallExtension`. A verb the row cannot run
    /// paints and announces DISABLED (React's `disabled` buttons) instead of disappearing — the session's own program is never
    /// uninstallable (React's `canUninstall`), a plugin mid-install offers nothing, a failed extension cannot be enabled.
    ///
    /// 🪟️ The roster is ONE windowed Tree section (`framework.marketplace.source.local`): the rows the tree window observer
    /// reported for it ([`tree_windows::TreeWindowScheduler::window_of`]) — one viewport of rows before the first report —
    /// materialise inside [`MARKETPLACE_WINDOW_NODE_BUDGET`], and the retained layout pitches the rest as spacers, so a roster of
    /// any length stays inside the retained node ceiling and scrolls to its last row. Store-owned extensions stay nested under
    /// their declared host, including failed loads, so install failures never erase inventory.
    pub(crate) fn build_marketplace_ui(&self) -> UiNode {
        let is_de = self.locale_id == "de";
        let session_plugin = self.session.as_ref().map(|session| session.plugin_id.clone());
        let ids = self.marketplace_roster();
        let installing = self.plugin_install.as_ref().map(|install| install.plugin_id.clone());
        let verb = |action: &str, args: Option<DslValue>, icon: IconName, label_key: &'static str, disabled: bool| ui_wgpu::wgpu::component::ui::UiTreeItemAction {
            icon_id: icon,
            label: Some(Label::data(shell_chrome_string(label_key, is_de))),
            action: ActionDescriptor { controller_id: "framework".into(), action: action.into(), args },
            placement: None,
            disabled,
        };
        let extension_item = |entry: &ShellExtensionProjection| {
            let extension_id = entry.record.extension_id.clone();
            let can_toggle = matches!(entry.load_status, ShellExtensionLoadStatus::Available | ShellExtensionLoadStatus::Loaded);
            let target = crate::action_args_json!({ "extensionId": extension_id.clone(), "enabled": !entry.enabled });
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{}.extension.{extension_id}", entry.record.extends_host),
                label: Label::data(format!("{} · {} · {}", entry.record.label, entry.record.version, shell_chrome_string(if entry.enabled { "plugins.extension.enabled" } else { "plugins.extension.disabled" }, is_de))),
                actions: Some(vec![
                    verb("setExtensionEnabled", target.clone(), if entry.enabled { IconName::EyeOff } else { IconName::Eye }, if entry.enabled { "plugins.extension.disable" } else { "plugins.extension.enable" }, !can_toggle),
                    verb("uninstallExtension", target, IconName::Trash2, "plugins.action.uninstall", false),
                ]),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let plugin_item = |plugin_id: &str| {
            let resident = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id);
            let in_flight = installing.as_deref() == Some(plugin_id);
            let status_key = if in_flight {
                "plugins.status.installing"
            } else if resident.is_some() {
                "plugins.status.loaded"
            } else {
                "plugins.status.available"
            };
            let label = resident.map(|entry| entry.manifest.label.clone()).unwrap_or_else(|| plugin_id.to_string());
            let version = resident.map(|entry| entry.manifest.version.clone()).unwrap_or_default();
            let head = if version.is_empty() { format!("{label} \u{b7} {}", shell_chrome_string(status_key, is_de)) } else { format!("{label} \u{b7} {version} \u{b7} {}", shell_chrome_string(status_key, is_de)) };
            let target = crate::action_args_json!({ "pluginId": plugin_id });
            let can_uninstall = resident.is_some() && session_plugin.as_deref() != Some(plugin_id);
            let actions = if resident.is_some() {
                vec![verb("reloadPlugin", target.clone(), IconName::RotateCcw, "plugins.action.reload", in_flight), verb("uninstallPlugin", target, IconName::Trash2, "plugins.action.uninstall", !can_uninstall || in_flight)]
            } else {
                vec![verb("installPlugin", target, IconName::Download, "plugins.action.install", in_flight)]
            };
            let extensions: Vec<UiTreeItemNode> = self.extensions.iter().filter(|entry| entry.record.extends_host == plugin_id).map(&extension_item).collect();
            UiTreeItemNode {
                id: format!("framework.marketplace.plugin.{plugin_id}"),
                label: Label::data(head),
                default_open: Some(!extensions.is_empty()),
                presence: UiPresence { status: if in_flight { ui_wgpu::wgpu::component::ui::UiStatus::Loading } else { ui_wgpu::wgpu::component::ui::UiStatus::Idle }, ..UiPresence::default() },
                actions: Some(actions),
                items: (!extensions.is_empty()).then_some(extensions),
                ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
            }
        };
        let section_key = format!("{FRAMEWORK_MARKETPLACE_TAB_ID}/{MARKETPLACE_LOCAL_SECTION_ID}");
        let (offset, rows) = self.tree_windows.window_of(FRAMEWORK_MARKETPLACE_TAB_ID, &section_key).unwrap_or((0, self.tree_windows.viewport_rows_or_default()));
        let offset = (offset as usize).min(ids.len().saturating_sub(1));
        let mut credit = MARKETPLACE_WINDOW_NODE_BUDGET;
        let materialised: Vec<UiTreeItemNode> = ids
            .iter()
            .skip(offset)
            .take(rows as usize)
            .map(|plugin_id| plugin_item(plugin_id))
            .map_while(|item| {
                let cost = 1 + item.items.as_ref().map_or(0, Vec::len);
                (cost <= credit).then(|| {
                    credit -= cost;
                    item
                })
            })
            .collect();
        let (items, window) = if ids.is_empty() {
            (vec![UiTreeItemNode::base("framework.marketplace.empty", Label::data(shell_chrome_string("marketplace.unavailable", is_de)))], None)
        } else {
            (materialised, Some(UiTreeWindow { row_extent: UiTreeWindowRowExtent::Standard, total: ids.len() as u32, offset: offset as u32 }))
        };
        let mut missing_hosts: Vec<&str> = self.extensions.iter().map(|entry| entry.record.extends_host.as_str()).filter(|host| !ids.iter().any(|id| id.as_str() == *host)).collect();
        missing_hosts.sort();
        missing_hosts.dedup();
        let install_file = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.file".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromFile", is_de)),
            icon_id: Some(IconName::FileArchive),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionFile".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let install_url = UiTreeItemNode {
            id: "framework.marketplace.extensions.install.url".into(),
            label: Label::data(shell_chrome_string("plugins.extension.installFromUrl", is_de)),
            icon_id: Some(IconName::Download),
            action: Some(ActionDescriptor { controller_id: "framework".into(), action: "installExtensionUrl".into(), args: None }),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        };
        let mut sections = vec![
            UiTreeSectionNode {
                header_toolbar: None,
                id: "framework.marketplace.extensions.install".into(),
                label: Some(Label::data(shell_chrome_string("plugins.extension.install", is_de))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items: vec![install_url, install_file],
                window: None,
            },
            UiTreeSectionNode {
                header_toolbar: None,
                id: MARKETPLACE_LOCAL_SECTION_ID.into(),
                label: Some(Label::data(format!("{}: local", shell_chrome_string("plugins.source", is_de)))),
                default_open: Some(true),
                presence: UiPresence::default(),
                items,
                window,
            },
        ];
        sections.extend(missing_hosts.into_iter().map(|host| UiTreeSectionNode {
            header_toolbar: None,
            id: format!("framework.marketplace.missing-host.{host}"),
            label: Some(Label::data(host.to_string())),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: self.extensions.iter().filter(|entry| entry.record.extends_host.as_str() == host).map(&extension_item).collect(),
            window: None,
        }));
        display_panel_body("framework.marketplace.panel", sections)
    }

