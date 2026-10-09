import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep("remove_property, rename_entity, set_camera, set_field, set_property, set_view, split_wall,", "remove_property, rename_entity, select_findings, set_camera, set_field, set_property, set_view, split_wall,")
rep("use crate::editor::bim::panels::{library as library_panel, outliner as outliner_panel, properties as properties_panel};", "use crate::editor::bim::panels::{diagnostics as diagnostics_panel, library as library_panel, outliner as outliner_panel, properties as properties_panel};")
rep('''            "engagementInput" as "engagement-input" => engagement_input::EngagementInput,''', '''            "selectFindings" as "select-findings" => select_findings::SelectFindings, [HostOnly]; View, cmd_select_findings, cmd_select_findings_describe;
            "engagementInput" as "engagement-input" => engagement_input::EngagementInput,''')
rep('''            "engagementInput" => BimCommand::EngagementInput(''', '''            "selectFindings" => BimCommand::SelectFindings(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "engagementInput" => BimCommand::EngagementInput(''')
rep('''    "bim.place.unsupported" => fault_place_unsupported;
''', '''    "bim.place.unsupported" => fault_place_unsupported;
    "bim.diagnostic.target-missing" => fault_diagnostic_target_missing;
''')
rep('''        library_panel::BODY_KEY => library_panel::render(snapshot, labels, &windows()),
''', '''        library_panel::BODY_KEY => library_panel::render(snapshot, labels, &windows()),
        diagnostics_panel::BODY_KEY => diagnostics_panel::render(snapshot, inference, labels, &windows()),
''')
rep('''        library_panel::BODY_KEY => (labels.panel_library, None),
''', '''        library_panel::BODY_KEY => (labels.panel_library, None),
        diagnostics_panel::BODY_KEY => (labels.panel_diagnostics, Some(labels.diag_surface_describe)),
''')
rep('''        "removeProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set).required(), text("property", |labels| labels.arg_property).required()],
''', '''        "removeProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set).required(), text("property", |labels| labels.arg_property).required()],
        "selectFindings" => vec![ids()],
''')
rep('''        .panel_tab_def(library_panel::definition())
''', '''        .panel_tab_def(library_panel::definition())
        .panel_tab_def(diagnostics_panel::definition())
''')
open(p, 'w', encoding='utf-8', newline='').write(s)
