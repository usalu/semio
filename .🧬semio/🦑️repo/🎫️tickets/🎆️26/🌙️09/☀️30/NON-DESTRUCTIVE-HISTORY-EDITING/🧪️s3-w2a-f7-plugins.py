"""🖱️ S3-W2A F7 (every `Menu::of` caller under `✏️s/🔌️plugins`): menus resolve in the call's axes (`Menu::of(registry,
view_state)`), selection phrases name glossary kinds (`selection_count_phrase(locale, &[(count, SelectionKind)])`), the
node-graph delete row stays visible but disabled with "Nothing selected" when empty, and disabled selection rows carry that
reason. Compile-atomic per crate: every helper signature changes together with its callers and tests."""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins")
E = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
VM = "semio_framework_plugin::ViewModel"
EN = "&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)"
IS_DE = "let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;"
DELETE_LABEL = "semio_framework_plugin::delete_selection().resolve(view_state.terminology, view_state.locale)"
NOTHING = "&semio_framework_plugin::nothing_selected()"

plan = {
    f"✒️writer/🗿️artifacts/✒️writer/{E}/🦀️.rs": [
        ("fn writer_context_menu_items(registry: &AppActionRegistry, text: Option<&ContextMenuTextContext>, is_de: bool) -> Vec<ContextMenuItemSpec> {\n",
         f"fn writer_context_menu_items(registry: &AppActionRegistry, text: Option<&ContextMenuTextContext>, view_state: &{VM}) -> Vec<ContextMenuItemSpec> {{\n    {IS_DE}\n"),
        ("    let bespoke = |id: &str, label: &str, icon: &str, action: &str, disabled: bool| ContextMenuItemSpec {\n        id: id.into(),\n        label: Some(label.into()),\n        icon: Some(icon.into()),\n        action: Some(action.into()),\n        disabled: disabled.then_some(true),\n        ..Default::default()\n    };\n    Menu::of(registry)\n",
         "    let reason = semio_framework_plugin::nothing_selected().resolve(view_state.terminology, view_state.locale).to_string();\n    let bespoke = |id: &str, label: &str, icon: &str, action: &str, disabled: bool| {\n        let row = ContextMenuItemSpec { id: id.into(), label: Some(label.into()), icon: Some(icon.into()), action: Some(action.into()), ..Default::default() };\n        if disabled {\n            row.disabled_because(reason.clone())\n        } else {\n            row\n        }\n    };\n    Menu::of(registry, view_state)\n"),
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        let text = request.surface.as_ref().and_then(|surface| surface.text.as_ref());\n        writer_context_menu_items(registry, text, is_de)\n",
         "        let text = request.surface.as_ref().and_then(|surface| surface.text.as_ref());\n        writer_context_menu_items(registry, text, view_state)\n"),
    ],
    f"🌀️procedural/🗿️artifacts/🌀️generation2d/{E}/🦀️.rs": [
        ("        let labels = semio_framework_plugin::resolve_labels::<Generation2dLabels>(view_state);\n        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n",
         "        let labels = semio_framework_plugin::resolve_labels::<Generation2dLabels>(view_state);\n"),
        ("        let mut menu = Menu::of(registry).action(\"addWidget\").action(\"reorganize\").action(\"generate\");\n",
         "        let mut menu = Menu::of(registry, view_state).action(\"addWidget\").action(\"reorganize\").action(\"generate\");\n"),
        ("        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit) {\n            menu = menu.item(spec);\n        }\n        menu.build()\n",
         "        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()\n"),
    ],
    f"🌀️procedural/🗿️artifacts/🌀️generation2d/{E}/🧪️tests/🔬️unit/🦀️.rs": [
        ("    assert!(items.iter().all(|item| item.id != \"delete-selection\"), \"an empty selection must not offer delete-selection\");\n",
         "    assert!(\n        items.iter().any(|item| item.id == \"delete-selection\" && item.disabled == Some(true) && item.reason.as_deref() == Some(\"Nothing selected\") && item.action.is_none() && item.args.is_none()),\n        \"an empty selection keeps delete-selection visible, disabled with its reason, and dispatching nothing: {items:?}\"\n    );\n"),
    ],
    f"🌀️procedural/🗿️artifacts/🧊️generation3d/{E}/🦀️.rs": [
        ("        let labels = generation3d_labels(view_state);\n        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.host_snapshot);\n",
         "        let labels = generation3d_labels(view_state);\n        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.host_snapshot);\n"),
        ("        let mut menu = Menu::of(registry).action(\"reorganize\");\n",
         "        let mut menu = Menu::of(registry, view_state).action(\"reorganize\");\n"),
        ("        if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit) {\n            menu = menu.item(spec);\n        }\n        menu.build()\n",
         "        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()\n"),
    ],
    f"🌊️flow/🗿️artifacts/🌊️flow/{E}/🦀️.rs": [
        ("fn flow_context_menu_items(registry: &AppActionRegistry, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, labels: &FlowPlayLabels, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {\n    use semio_framework_plugin::{selection_count_phrase, Menu};\n",
         f"fn flow_context_menu_items(registry: &AppActionRegistry, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, labels: &FlowPlayLabels, view_state: &{VM}, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {{\n    use semio_framework_plugin::{{node_graph_delete_selection_spec, Menu, NodeGraphDeleteDispatch}};\n"),
        ("    // groups; `delete-selection` stays a direct destructive item last — `organize_context_menu`\n",
         "    // groups; `delete-selection` stays a direct destructive item last (disabled with its reason while nothing is\n    // selected) — `organize_context_menu`\n"),
        ("        let mut menu = Menu::of(registry);\n        if hits.is_empty() {\n",
         "        let mut menu = Menu::of(registry, view_state);\n        if hits.is_empty() {\n"),
        ("            let phrase = selection_count_phrase(is_de, &[(nodes.len(), if is_de { \"Knoten\" } else { \"node\" }, if is_de { \"Knoten\" } else { \"nodes\" }), (edges.len(), if is_de { \"Kante\" } else { \"edge\" }, if is_de { \"Kanten\" } else { \"edges\" })]);\n            if !phrase.is_empty() {\n                menu = menu.item(ContextMenuItemSpec {\n                    id: \"delete-selection\".into(),\n                    label: Some(format!(\"{} ({phrase})\", labels.delete_selection.as_str())),\n                    icon: Some(\"trash\".into()),\n                    destructive: Some(true),\n                    action: Some(\"deleteSelection\".into()),\n                    ..Default::default()\n                });\n            }\n        }\n        menu.build()\n",
         "        }\n        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()\n"),
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else { return Vec::new() };\n        flow_context_menu_items(registry, &composed, &config, flow_play_labels(view_state), is_de, request.surface.as_ref())\n",
         "        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else { return Vec::new() };\n        flow_context_menu_items(registry, &composed, &config, flow_play_labels(view_state), view_state, request.surface.as_ref())\n"),
    ],
    f"🌊️flow/🗿️artifacts/🌊️flow/{E}/🧪️tests/🔬️unit/🦀️.rs": [
        ("    assert!(!menu_json.contains(r#\"\"id\":\"delete-selection\"\"#), \"empty canvas must omit delete: {menu_json}\");\n",
         "    assert!(menu_json.contains(r#\"\"id\":\"delete-selection\"\"#) && menu_json.contains(r#\"\"reason\":\"Nothing selected\"\"#) && !menu_json.contains(r#\"\"action\":\"deleteSelection\"\"#), \"empty canvas keeps delete disabled with its reason: {menu_json}\");\n"),
        ("    assert!(!before.contains(r#\"\"id\":\"delete-selection\"\"#), \"preview starts without delete: {before}\");\n",
         "    assert!(before.contains(r#\"\"reason\":\"Nothing selected\"\"#) && !before.contains(r#\"\"action\":\"deleteSelection\"\"#), \"preview starts with delete disabled: {before}\");\n"),
        ("async fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {\n",
         "async fn context_menu_annotates_mixed_selection_counts_and_disables_delete_without_selection() {\n"),
        ("    assert!(!empty.contains(r#\"\"id\":\"delete-selection\"\"#), \"empty must omit delete: {empty}\");\n",
         "    assert!(empty.contains(r#\"\"reason\":\"Nothing selected\"\"#) && !empty.contains(r#\"\"action\":\"deleteSelection\"\"#), \"empty keeps delete disabled with its reason: {empty}\");\n"),
    ],
    f"🌍️gis/🗿️artifacts/🗺️gismap/{E}/🦀️.rs": [
        ("/// disclosure via `Menu::of(registry)`; `organize_context_menu` (run automatically at the\n",
         "/// disclosure via `Menu::of(registry, view_state)`; `organize_context_menu` (run automatically at the\n"),
        ("/// it is what decides whether `clearSelection` renders enabled.\nasync fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n",
         f"/// it is what decides whether `clearSelection` renders enabled or disabled with its reason.\nasync fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, view_state: &{VM}, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {{\n"),
        ("        let mut menu = Menu::of(registry)\n            .action_args(INTERACTION_SELECT_ACTION_ID",
         "        let mut menu = Menu::of(registry, view_state)\n            .action_args(INTERACTION_SELECT_ACTION_ID"),
        ("    let mut items = Menu::of(registry).action(\"selectAll\").action(\"fitWorld\").destructive(\"clearSelection\").build();\n    if let Some(clear) = items.iter_mut().find(|entry| entry.id == \"clearSelection\") {\n        clear.disabled = selected_ids.is_empty().then_some(true);\n    }\n    items\n",
         f"    Menu::of(registry, view_state).action(\"selectAll\").action(\"fitWorld\").destructive(\"clearSelection\").when(selected_ids.is_empty(), |menu| menu.disabled_because({NOTHING})).build()\n"),
        ("        _view_state: &semio_framework_plugin::ViewModel,\n        registry: &semio_framework_plugin::AppActionRegistry,\n    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, request.surface.as_ref(), &[]).await })\n",
         "        view_state: &semio_framework_plugin::ViewModel,\n        registry: &semio_framework_plugin::AppActionRegistry,\n    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, view_state, request.surface.as_ref(), &[]).await })\n"),
        ("        _view_state: &semio_framework_plugin::ViewModel,\n        interaction: &InteractionView<'_>,\n        registry: &semio_framework_plugin::AppActionRegistry,\n    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n        let selected = Gis2dInteractionSnapshot::from_interaction(interaction).ids;\n        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, request.surface.as_ref(), &selected).await })\n",
         "        view_state: &semio_framework_plugin::ViewModel,\n        interaction: &InteractionView<'_>,\n        registry: &semio_framework_plugin::AppActionRegistry,\n    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n        let selected = Gis2dInteractionSnapshot::from_interaction(interaction).ids;\n        ::semio_framework_async::poll::resolve_ready(async { gis2d_context_menu_items(registry, view_state, request.surface.as_ref(), &selected).await })\n"),
    ],
    f"🎬️sequence/🗿️artifacts/🎬️sequence/{E}/🦀️.rs": [
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        sequence_context_menu_items(registry, is_de, request.surface.as_ref(), &[])\n",
         "        sequence_context_menu_items(registry, view_state, request.surface.as_ref(), &[])\n"),
        ("fn sequence_context_menu_items(registry: &AppActionRegistry, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected: &[String]) -> Vec<ContextMenuItemSpec> {\n    use semio_framework_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};\n",
         f"fn sequence_context_menu_items(registry: &AppActionRegistry, view_state: &{VM}, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected: &[String]) -> Vec<ContextMenuItemSpec> {{\n    use semio_framework_plugin::{{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch}};\n    {IS_DE}\n"),
        ("    let mut menu = Menu::of(registry).action(\"run\").action(\"stop\")",
         "    let mut menu = Menu::of(registry, view_state).action(\"run\").action(\"stop\")"),
        ("    if let Some(spec) = node_graph_delete_selection_spec(\"Delete selection\", is_de, &nodes, &edges, NodeGraphDeleteDispatch::Direct) {\n        menu = menu.item(spec);\n    }\n    menu.build()\n",
         f"    menu.item(node_graph_delete_selection_spec({DELETE_LABEL}, view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()\n"),
    ],
    f"🎬️sequence/🗿️artifacts/🎬️sequence/{E}/🧪️tests/🔬️unit/🦀️.rs": [
        ("    let items = sequence_context_menu_items(&registry, false, None, &[\"step-1\".to_string()]);\n",
         f"    let items = sequence_context_menu_items(&registry, {EN}, None, &[\"step-1\".to_string()]);\n"),
    ],
    f"🏭️process/🗿️artifacts/🧊️process3d/{E}/🦀️.rs": [
        ("fn process3d_context_menu_items(registry: &AppActionRegistry, selected_ids: &[String]) -> Vec<ContextMenuItemSpec> {\n    let menu = Menu::of(registry).action(\"addStep\");\n",
         f"fn process3d_context_menu_items(registry: &AppActionRegistry, view_state: &{VM}, selected_ids: &[String]) -> Vec<ContextMenuItemSpec> {{\n    let menu = Menu::of(registry, view_state).action(\"addStep\");\n"),
        ("    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, _view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n        process3d_context_menu_items(registry, &[])\n",
         "    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n        process3d_context_menu_items(registry, view_state, &[])\n"),
        ("        _view_state: &semio_framework_plugin::ViewModel,\n        interaction: &semio_framework_plugin::app::InteractionView<'_>,\n        registry: &AppActionRegistry,\n    ) -> Vec<ContextMenuItemSpec> {\n        process3d_context_menu_items(registry, &interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids)\n",
         "        view_state: &semio_framework_plugin::ViewModel,\n        interaction: &semio_framework_plugin::app::InteractionView<'_>,\n        registry: &AppActionRegistry,\n    ) -> Vec<ContextMenuItemSpec> {\n        process3d_context_menu_items(registry, view_state, &interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids)\n"),
    ],
    f"🏭️process/🗿️artifacts/🧊️process3d/{E}/🧪️tests/🔬️unit/🦀️.rs": [
        ("process3d_context_menu_items(&registry, selected)",
         f"process3d_context_menu_items(&registry, {EN}, selected)"),
    ],
    f"📏️layout/🗿️artifacts/📏️layout/{E}/🦀️.rs": [
        ("    labels: &LayoutLabels,\n    is_de: bool,\n    surface: Option<&ContextMenuSurfaceTarget>,\n    fallback_selected: &[String],\n) -> Vec<ContextMenuItemSpec> {\n    use semio_framework_plugin::{selection_count_phrase, Menu};\n",
         f"    labels: &LayoutLabels,\n    view_state: &{VM},\n    surface: Option<&ContextMenuSurfaceTarget>,\n    fallback_selected: &[String],\n) -> Vec<ContextMenuItemSpec> {{\n    use semio_framework_plugin::{{selection_count_phrase, Menu, SelectionKind}};\n    {IS_DE}\n"),
        ("            return Menu::of(registry).item(layout_context_menu_item(\"set-active-page\"",
         "            return Menu::of(registry, view_state).item(layout_context_menu_item(\"set-active-page\""),
        ("            return Menu::of(registry)\n                .item(layout_context_menu_item(\n                    \"select-link-frames\",",
         "            return Menu::of(registry, view_state)\n                .item(layout_context_menu_item(\n                    \"select-link-frames\","),
        ("            return Menu::of(registry).item(layout_context_menu_item(\"focus-preflight-issue\"",
         "            return Menu::of(registry, view_state).item(layout_context_menu_item(\"focus-preflight-issue\""),
        ("            return Menu::of(registry).action(\"selectAll\").build();\n        }\n        let mut menu = Menu::of(registry);\n",
         "            return Menu::of(registry, view_state).action(\"selectAll\").build();\n        }\n        let mut menu = Menu::of(registry, view_state);\n"),
        ("        let mut items = menu.action(\"addPage\").action(\"selectAll\").action(\"paste\").destructive(\"clearSelection\").build();\n        if let Some(clear) = items.iter_mut().find(|entry| entry.id == \"clearSelection\") {\n            clear.disabled = Some(true);\n        }\n        return items;\n",
         f"        return menu.action(\"addPage\").action(\"selectAll\").action(\"paste\").destructive(\"clearSelection\").disabled_because({NOTHING}).build();\n"),
        ("    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { \"Rahmen\" } else { \"frame\" }, if is_de { \"Rahmen\" } else { \"frames\" })]);\n    let mut menu = Menu::of(registry);\n",
         "    let phrase = selection_count_phrase(view_state.locale, &[(selected.len(), SelectionKind::Frame)]).unwrap_or_default();\n    let mut menu = Menu::of(registry, view_state);\n"),
        ("    let mut items = menu.destructive(\"clearSelection\").build();\n    if let Some(clear) = items.iter_mut().find(|entry| entry.id == \"clearSelection\") {\n        clear.disabled = None;\n    }\n    items\n}\n",
         "    menu.destructive(\"clearSelection\").build()\n}\n"),
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), is_de, request.surface.as_ref(), &[])\n",
         "        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &[])\n"),
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        let fallback = LayoutInteractionSnapshot::from_interaction(interaction).ids;\n        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), is_de, request.surface.as_ref(), &fallback)\n",
         "        let fallback = LayoutInteractionSnapshot::from_interaction(interaction).ids;\n        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &fallback)\n"),
    ],
    f"📐️cad/🗿️artifacts/📐️cad/{E}/🦀️.rs": [
        ("    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n        Menu::of(registry).action(",
         "    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {\n        Menu::of(registry, view_state).action("),
    ],
    f"🔱️trinity/🗿️artifacts/♻️rewriting/{E}/🦀️.rs": [
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        // 🕹️ Selection is framework-owned now (domain \"graph\")",
         "        // 🕹️ Selection is framework-owned now (domain \"graph\")"),
        ("        let mut menu = Menu::of(registry)\n            .action(\"addRuleClause\")",
         "        let menu = Menu::of(registry, view_state)\n            .action(\"addRuleClause\")"),
        ("        if let Some(spec) = node_graph_delete_selection_spec(\"Delete selection\", is_de, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit) {\n            menu = menu.item(spec);\n        }\n        menu.build()\n",
         f"        menu.item(node_graph_delete_selection_spec({DELETE_LABEL}, view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()\n"),
    ],
    f"🔱️trinity/🗿️artifacts/🔌️jack/{E}/🦀️.rs": [
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        // 🕹️ Selection is framework-owned now (domain \"ast\")",
         "        // 🕹️ Selection is framework-owned now (domain \"ast\")"),
        ("        let mut menu = Menu::of(registry).action(\"runQuery\")",
         "        let menu = Menu::of(registry, view_state).action(\"runQuery\")"),
        ("        if let Some(spec) = node_graph_delete_selection_spec(\"Delete selection\", is_de, &nodes, &edges, NodeGraphDeleteDispatch::Direct) {\n            menu = menu.item(spec);\n        }\n        menu.build()\n",
         f"        menu.item(node_graph_delete_selection_spec({DELETE_LABEL}, view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()\n"),
    ],
    f"🕸️dag/🗿️artifacts/🕸️dag/{E}/🦀️.rs": [
        ("fn dag_context_menu_items(registry: &AppActionRegistry, labels: &crate::editor::dag::terminology::DagPlayLabels, is_de: bool, selected: &[String], request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {\n",
         f"fn dag_context_menu_items(registry: &AppActionRegistry, labels: &crate::editor::dag::terminology::DagPlayLabels, view_state: &{VM}, selected: &[String], request: &ContextMenuRequest) -> Vec<ContextMenuItemSpec> {{\n"),
        ("    let mut menu = Menu::of(registry).action_args(\"addNode\"",
         "    let mut menu = Menu::of(registry, view_state).action_args(\"addNode\""),
        ("    if let Some(spec) = node_graph_delete_selection_spec(labels.delete_selection.as_str(), is_de, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit) {\n        menu = menu.item(spec);\n    }\n    menu.build()\n",
         "    menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()\n"),
        ("        let labels = dag_play_labels(view_state);\n        let is_de = is_de_locale(view_state);\n        dag_context_menu_items(registry, labels, is_de, &[], request)\n",
         "        let labels = dag_play_labels(view_state);\n        dag_context_menu_items(registry, labels, view_state, &[], request)\n"),
    ],
    f"🧩️puzzle/🗿️artifacts/◻️2d/{E}/🦀️.rs": [
        ("async fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, fixture: &Value, selected: &[String], is_de: bool) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};\n",
         f"async fn puzzle2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, fixture: &Value, selected: &[String], view_state: &{VM}) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {{\n    use semio_framework_plugin::{{selection_count_phrase, ContextMenuItemSpec, Menu, SelectionKind}};\n    {IS_DE}\n"),
        ("        return Menu::of(registry)\n            .item(item(\"openAddNodeDialog\"",
         "        return Menu::of(registry, view_state)\n            .item(item(\"openAddNodeDialog\""),
        ("    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { \"Element\" } else { \"item\" }, if is_de { \"Elemente\" } else { \"items\" })]);\n",
         "    let phrase = selection_count_phrase(view_state.locale, &[(selected.len(), SelectionKind::Item)]).unwrap_or_default();\n"),
        ("    let mut menu = Menu::of(registry);\n    if let [only] = selected_handle_ids.as_slice() {\n",
         "    let mut menu = Menu::of(registry, view_state);\n    if let [only] = selected_handle_ids.as_slice() {\n"),
        ("        let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;\n        let mut selected: Vec<String> = request.surface.as_ref()",
         "        let mut selected: Vec<String> = request.surface.as_ref()"),
        ("        ::semio_framework_async::poll::resolve_ready(puzzle2d_context_menu_items(registry, doc.snapshot.value(), &selected, is_de))\n",
         "        ::semio_framework_async::poll::resolve_ready(puzzle2d_context_menu_items(registry, doc.snapshot.value(), &selected, view_state))\n"),
    ],
    f"🧩️puzzle/🗿️artifacts/🖐️5d/{E}/🦀️.rs": [
        ("use crate::editor::puzzle5d::terminology::{puzzle5d_is_de_locale, puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};\n",
         "use crate::editor::puzzle5d::terminology::{puzzle5d_labels, puzzle5d_localized, Puzzle5dLabels};\n"),
        ("    labels: &Puzzle5dLabels,\n    is_de: bool,\n    registry: &semio_framework_plugin::AppActionRegistry,\n) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};\n",
         f"    labels: &Puzzle5dLabels,\n    view_state: &{VM},\n    registry: &semio_framework_plugin::AppActionRegistry,\n) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {{\n    use semio_framework_plugin::{{selection_count_phrase, ContextMenuItemSpec, Menu, SelectionKind}};\n"),
        ("    if selection.is_empty() {\n        return Menu::of(registry)\n",
         "    if selection.is_empty() {\n        return Menu::of(registry, view_state)\n"),
        ("        let phrase = selection_count_phrase(is_de, &[(part_ids.len(), if is_de { \"Teil\" } else { \"part\" }, if is_de { \"Teile\" } else { \"parts\" })]);\n        return Menu::of(registry)\n",
         "        let phrase = selection_count_phrase(view_state.locale, &[(part_ids.len(), SelectionKind::Part)]).unwrap_or_default();\n        return Menu::of(registry, view_state)\n"),
        ("        let mut menu = Menu::of(registry);\n        // 🎣️",
         "        let mut menu = Menu::of(registry, view_state);\n        // 🎣️"),
        ("    Menu::of(registry).item(bespoke(\"delete\", labels.delete.into(), \"trash\", \"deleteFastener\"",
         "    Menu::of(registry, view_state).item(bespoke(\"delete\", labels.delete.into(), \"trash\", \"deleteFastener\""),
        ("        let Some(is_de) = puzzle5d_is_de_locale(view_state) else { return Vec::new() };\n",
         ""),
        ("        puzzle5d_context_menu_items(&envelope, &selection, labels, is_de, registry)\n",
         "        puzzle5d_context_menu_items(&envelope, &selection, labels, view_state, registry)\n"),
    ],
    f"🧩️puzzle/🗿️artifacts/🧊️3d/{E}/🦀️.rs": [
        ("fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {\n",
         f"fn puzzle3d_context_menu_items(envelope: &Puzzle3dScene, selection: &Puzzle3dContextSelection, labels: &Puzzle3dLabels, view_state: &{VM}, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {{\n"),
        ("        return {\n            Menu::of(registry)\n                .item(puzzle3d_context_menu_row(\"duplicate\"",
         "        return {\n            Menu::of(registry, view_state)\n                .item(puzzle3d_context_menu_row(\"duplicate\""),
        ("        let mut menu = Menu::of(registry);\n        if let [only] = selection.vortex_ids.as_slice() {\n",
         "        let mut menu = Menu::of(registry, view_state);\n        if let [only] = selection.vortex_ids.as_slice() {\n"),
        ("        return Menu::of(registry).item(puzzle3d_context_menu_row(\"delete\", labels.delete, \"trash\", \"deleteAttraction\"",
         "        return Menu::of(registry, view_state).item(puzzle3d_context_menu_row(\"delete\", labels.delete, \"trash\", \"deleteAttraction\""),
        ("        return {\n            Menu::of(registry)\n                .group(\"targets\", |m| {",
         "        return {\n            Menu::of(registry, view_state)\n                .group(\"targets\", |m| {"),
        ("            Menu::of(registry).item(puzzle3d_context_menu_row(\"zoom\", labels.zoom_to_selection, \"crosshair\", \"focusSelection\", None, false)).item(puzzle3d_context_menu_row(\"delete\", labels.delete, \"trash\", \"deleteSelection\", None, true)).build()\n",
         "            Menu::of(registry, view_state).item(puzzle3d_context_menu_row(\"zoom\", labels.zoom_to_selection, \"crosshair\", \"focusSelection\", None, false)).item(puzzle3d_context_menu_row(\"delete\", labels.delete, \"trash\", \"deleteSelection\", None, true)).build()\n"),
        ("        puzzle3d_context_menu_items(&envelope, &selection, labels, registry)\n",
         "        puzzle3d_context_menu_items(&envelope, &selection, labels, view_state, registry)\n"),
    ],
}

texts = {}
for rel, edits in plan.items():
    path = ROOT / rel
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit(f"{rel}: anchor count {text.count(old)}: {old[:110]!r}")
        text = text.replace(old, new)
    texts[path] = text
for path, text in texts.items():
    path.write_text(text, encoding="utf-8")
remaining = [str(path.relative_to(ROOT)) for path in texts if "Menu::of(registry)" in path.read_text(encoding="utf-8") or "is_de, &nodes" in path.read_text(encoding="utf-8")]
print(f"F7 plugins: {sum(len(e) for e in plan.values())} edits over {len(plan)} files; leftovers: {remaining}")
