//! 🧪️ Ticket-local declared-verb probe harness (P8): runs the framework declared-verb law over every surface and prints
//! one JSON line per surface — findings and agent-lane divergences — so the whole fleet is measured without touching the tree.
use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_findings, probe_declared_verbs};

fn report(label: &str, probes: Vec<semio_framework_plugin::artifact_app_laws::DeclaredVerbProbe>) {
    let findings: Vec<String> = probes.iter().flat_map(declared_verb_findings).map(|finding| finding.to_string()).collect();
    let agent = declared_verb_agent_divergences(&probes);
    if std::env::var_os("P8_DUMP").is_some() {
        for probe in probes.iter().filter(|probe| !declared_verb_findings(probe).is_empty()) {
            eprintln!("{label} {probe:#?}");
        }
    }
    let quote = |text: &str| format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " "));
    println!("{{\"surface\":{},\"verbs\":{},\"findings\":[{}],\"agentDivergences\":[{}]}}", quote(label), probes.len(), findings.iter().map(|f| quote(f)).collect::<Vec<_>>().join(","), agent.iter().map(|a| quote(a)).collect::<Vec<_>>().join(","));
}

fn run(filter: Option<String>) {
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-writer-writer::WriterPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-writer-writer::WriterPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_writer_writer::editor::writer::WriterPlayApp>, <semio_s_artifact_writer_writer::editor::writer::WriterPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_writer_writer::editor::writer::create_writer_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-writer-writer::WriterPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-writer-writer::WriterPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-writer-writer::WriterViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-writer-writer::WriterViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_writer_writer::viewer::writer::WriterViewer>, <semio_s_artifact_writer_writer::viewer::writer::WriterViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_writer_writer::viewer::writer::create_writer_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-writer-writer::WriterViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-writer-writer::WriterViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-mathematical-equation::EquationPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-mathematical-equation::EquationPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_mathematical_equation::editor::equation::EquationPlayApp>, <semio_s_artifact_mathematical_equation::editor::equation::EquationPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_mathematical_equation::editor::equation::create_equation_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-mathematical-equation::EquationPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-mathematical-equation::EquationPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-mathematical-equation::EquationViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-mathematical-equation::EquationViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_mathematical_equation::viewer::equation::EquationViewer>, <semio_s_artifact_mathematical_equation::viewer::equation::EquationViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_mathematical_equation::viewer::equation::create_equation_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-mathematical-equation::EquationViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-mathematical-equation::EquationViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-2d::Wfc2dEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-2d::Wfc2dEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_wfc_2d::editor::wfc2d::Wfc2dEditor>, <semio_s_artifact_wfc_2d::editor::wfc2d::Wfc2dEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_wfc_2d::editor::wfc2d::create_wfc2d_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-2d::Wfc2dEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-2d::Wfc2dEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-2d::Wfc2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-2d::Wfc2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_wfc_2d::viewer::wfc2d::Wfc2dViewer>, <semio_s_artifact_wfc_2d::viewer::wfc2d::Wfc2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_wfc_2d::viewer::wfc2d::create_wfc2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-2d::Wfc2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-2d::Wfc2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-grid2d::Grid2dEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-grid2d::Grid2dEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_wfc_grid2d::editor::grid2d::Grid2dEditor>, <semio_s_artifact_wfc_grid2d::editor::grid2d::Grid2dEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_wfc_grid2d::editor::grid2d::create_grid2d_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-grid2d::Grid2dEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-grid2d::Grid2dEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-grid2d::Grid2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-grid2d::Grid2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_wfc_grid2d::viewer::grid2d::Grid2dViewer>, <semio_s_artifact_wfc_grid2d::viewer::grid2d::Grid2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_wfc_grid2d::viewer::grid2d::create_grid2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-grid2d::Grid2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-grid2d::Grid2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-bitmap::BitmapEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-bitmap::BitmapEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_wfc_bitmap::editor::bitmap::BitmapEditor>, <semio_s_artifact_wfc_bitmap::editor::bitmap::BitmapEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_wfc_bitmap::editor::bitmap::create_bitmap_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-bitmap::BitmapEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-bitmap::BitmapEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-bitmap::BitmapViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-bitmap::BitmapViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_wfc_bitmap::viewer::bitmap::BitmapViewer>, <semio_s_artifact_wfc_bitmap::viewer::bitmap::BitmapViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_wfc_bitmap::viewer::bitmap::create_bitmap_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-bitmap::BitmapViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-bitmap::BitmapViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-3d::Wfc3dEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-3d::Wfc3dEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor>, <semio_s_artifact_wfc_3d::editor::wfc3d::Wfc3dEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_wfc_3d::editor::wfc3d::create_wfc3d_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-3d::Wfc3dEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-3d::Wfc3dEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-3d::Wfc3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-3d::Wfc3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer>, <semio_s_artifact_wfc_3d::viewer::wfc3d::Wfc3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_wfc_3d::viewer::wfc3d::create_wfc3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-3d::Wfc3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-3d::Wfc3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-grid3d::Grid3dEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-grid3d::Grid3dEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_wfc_grid3d::editor::grid3d::Grid3dEditor>, <semio_s_artifact_wfc_grid3d::editor::grid3d::Grid3dEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_wfc_grid3d::editor::grid3d::create_grid3d_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-grid3d::Grid3dEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-grid3d::Grid3dEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-wfc-grid3d::Grid3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-wfc-grid3d::Grid3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_wfc_grid3d::viewer::grid3d::Grid3dViewer>, <semio_s_artifact_wfc_grid3d::viewer::grid3d::Grid3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_wfc_grid3d::viewer::grid3d::create_grid3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-wfc-grid3d::Grid3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-wfc-grid3d::Grid3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-procedural-generation2d::Generation2dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-procedural-generation2d::Generation2dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp>, <semio_s_artifact_procedural_generation2d::editor::generation2d::Generation2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_procedural_generation2d::editor::generation2d::create_generation2d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-procedural-generation2d::Generation2dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-procedural-generation2d::Generation2dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-procedural-generation2d::Generation2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-procedural-generation2d::Generation2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer>, <semio_s_artifact_procedural_generation2d::viewer::generation2d::Generation2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_procedural_generation2d::viewer::generation2d::create_generation2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-procedural-generation2d::Generation2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-procedural-generation2d::Generation2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-procedural-generation3d::Generation3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-procedural-generation3d::Generation3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp>, <semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_procedural_generation3d::editor::generation3d::create_generation3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-procedural-generation3d::Generation3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-procedural-generation3d::Generation3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-procedural-generation3d::Generation3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-procedural-generation3d::Generation3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer>, <semio_s_artifact_procedural_generation3d::viewer::generation3d::Generation3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_procedural_generation3d::viewer::generation3d::create_generation3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-procedural-generation3d::Generation3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-procedural-generation3d::Generation3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-flow-flow::FlowPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-flow-flow::FlowPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_flow_flow::editor::flow::FlowPlayApp>, <semio_s_artifact_flow_flow::editor::flow::FlowPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_flow_flow::editor::flow::create_flow_app, Some(include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/⚖️declared-verb-examples.json")))))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-flow-flow::FlowPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-flow-flow::FlowPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-flow-flow::FlowViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-flow-flow::FlowViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_flow_flow::viewer::flow::FlowViewer>, <semio_s_artifact_flow_flow::viewer::flow::FlowViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_flow_flow::viewer::flow::create_flow_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-flow-flow::FlowViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-flow-flow::FlowViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-gis-gisterrain::Gis3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-gis-gisterrain::Gis3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp>, <semio_s_artifact_gis_gisterrain::editor::gis3d::Gis3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_gis_gisterrain::editor::gis3d::create_gis3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-gis-gisterrain::Gis3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-gis-gisterrain::Gis3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-gis-gisterrain::GisTerrainViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-gis-gisterrain::GisTerrainViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer>, <semio_s_artifact_gis_gisterrain::viewer::gisterrain::GisTerrainViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_gis_gisterrain::viewer::gisterrain::create_gisterrain_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-gis-gisterrain::GisTerrainViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-gis-gisterrain::GisTerrainViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-gis-gismap::Gis2dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-gis-gismap::Gis2dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp>, <semio_s_artifact_gis_gismap::editor::gis2d::Gis2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_gis_gismap::editor::gis2d::create_gis2d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-gis-gismap::Gis2dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-gis-gismap::Gis2dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-gis-gismap::GisMapViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-gis-gismap::GisMapViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer>, <semio_s_artifact_gis_gismap::viewer::gismap::GisMapViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_gis_gismap::viewer::gismap::create_gismap_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-gis-gismap::GisMapViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-gis-gismap::GisMapViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-vcs-vcs::VcsPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-vcs-vcs::VcsPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_vcs_vcs::editor::vcs::VcsPlayApp>, <semio_s_artifact_vcs_vcs::editor::vcs::VcsPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_vcs_vcs::editor::vcs::create_vcs_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-vcs-vcs::VcsPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-vcs-vcs::VcsPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-vcs-vcs::VcsViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-vcs-vcs::VcsViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_vcs_vcs::viewer::vcs::VcsViewer>, <semio_s_artifact_vcs_vcs::viewer::vcs::VcsViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_vcs_vcs::viewer::vcs::create_vcs_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-vcs-vcs::VcsViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-vcs-vcs::VcsViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-animate-presentation::AnimatePresentationPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-animate-presentation::AnimatePresentationPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_animate_presentation::editor::animate::AnimatePresentationPlayApp>, <semio_s_artifact_animate_presentation::editor::animate::AnimatePresentationPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_animate_presentation::editor::animate::create_animate_presentation_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-animate-presentation::AnimatePresentationPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-animate-presentation::AnimatePresentationPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-animate-presentation::AnimatePresentationViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-animate-presentation::AnimatePresentationViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_animate_presentation::viewer::animate::AnimatePresentationViewer>, <semio_s_artifact_animate_presentation::viewer::animate::AnimatePresentationViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_animate_presentation::viewer::animate::create_animate_presentation_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-animate-presentation::AnimatePresentationViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-animate-presentation::AnimatePresentationViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-shooting-shooting::ShootingPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-shooting-shooting::ShootingPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_shooting_shooting::editor::shooting::ShootingPlayApp>, <semio_s_artifact_shooting_shooting::editor::shooting::ShootingPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_shooting_shooting::editor::shooting::create_shooting_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-shooting-shooting::ShootingPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-shooting-shooting::ShootingPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-shooting-shooting::ShootingViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-shooting-shooting::ShootingViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_shooting_shooting::viewer::shooting::ShootingViewer>, <semio_s_artifact_shooting_shooting::viewer::shooting::ShootingViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_shooting_shooting::viewer::shooting::create_shooting_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-shooting-shooting::ShootingViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-shooting-shooting::ShootingViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-demonstrator-playground::PlaygroundEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-demonstrator-playground::PlaygroundEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_demonstrator_playground::editor::playground::PlaygroundEditor>, <semio_s_artifact_demonstrator_playground::editor::playground::PlaygroundEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_demonstrator_playground::editor::playground::create_playground_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-demonstrator-playground::PlaygroundEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-demonstrator-playground::PlaygroundEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-demonstrator-playground::PlaygroundViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-demonstrator-playground::PlaygroundViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_demonstrator_playground::viewer::playground::PlaygroundViewer>, <semio_s_artifact_demonstrator_playground::viewer::playground::PlaygroundViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_demonstrator_playground::viewer::playground::create_playground_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-demonstrator-playground::PlaygroundViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-demonstrator-playground::PlaygroundViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-sequence-sequence::SequencePlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-sequence-sequence::SequencePlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_sequence_sequence::editor::sequence::SequencePlayApp>, <semio_s_artifact_sequence_sequence::editor::sequence::SequencePlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_sequence_sequence::editor::sequence::create_sequence_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-sequence-sequence::SequencePlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-sequence-sequence::SequencePlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-sequence-sequence::SequenceViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-sequence-sequence::SequenceViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_sequence_sequence::viewer::sequence::SequenceViewer>, <semio_s_artifact_sequence_sequence::viewer::sequence::SequenceViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_sequence_sequence::viewer::sequence::create_sequence_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-sequence-sequence::SequenceViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-sequence-sequence::SequenceViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-fem-2d::Fem2dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-fem-2d::Fem2dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_fem_2d::editor::fem2d::Fem2dPlayApp>, <semio_s_artifact_fem_2d::editor::fem2d::Fem2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_fem_2d::editor::fem2d::create_fem2d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-fem-2d::Fem2dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-fem-2d::Fem2dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-fem-2d::Fem2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-fem-2d::Fem2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer>, <semio_s_artifact_fem_2d::viewer::fem2d::Fem2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_fem_2d::viewer::fem2d::create_fem2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-fem-2d::Fem2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-fem-2d::Fem2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-fem-3d::Fem3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-fem-3d::Fem3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_fem_3d::editor::fem3d::Fem3dPlayApp>, <semio_s_artifact_fem_3d::editor::fem3d::Fem3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_fem_3d::editor::fem3d::create_fem3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-fem-3d::Fem3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-fem-3d::Fem3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-fem-3d::Fem3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-fem-3d::Fem3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer>, <semio_s_artifact_fem_3d::viewer::fem3d::Fem3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_fem_3d::viewer::fem3d::create_fem3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-fem-3d::Fem3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-fem-3d::Fem3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-architect-program::ArchitectPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-architect-program::ArchitectPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_architect_program::editor::architect::ArchitectPlayApp>, <semio_s_artifact_architect_program::editor::architect::ArchitectPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_architect_program::editor::architect::create_architect_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-architect-program::ArchitectPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-architect-program::ArchitectPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-architect-program::ArchitectViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-architect-program::ArchitectViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_architect_program::viewer::architect::ArchitectViewer>, <semio_s_artifact_architect_program::viewer::architect::ArchitectViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_architect_program::viewer::architect::create_architect_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-architect-program::ArchitectViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-architect-program::ArchitectViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-process-process3d::Process3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-process-process3d::Process3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_process_process3d::editor::process3d::Process3dPlayApp>, <semio_s_artifact_process_process3d::editor::process3d::Process3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_process_process3d::editor::process3d::create_process3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-process-process3d::Process3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-process-process3d::Process3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-process-process3d::Process3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-process-process3d::Process3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_process_process3d::viewer::process3d::Process3dViewer>, <semio_s_artifact_process_process3d::viewer::process3d::Process3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_process_process3d::viewer::process3d::create_process3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-process-process3d::Process3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-process-process3d::Process3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-lowpoly-lowpoly::LowpolyPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-lowpoly-lowpoly::LowpolyPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_lowpoly_lowpoly::editor::lowpoly::LowpolyPlayApp>, <semio_s_artifact_lowpoly_lowpoly::editor::lowpoly::LowpolyPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_lowpoly_lowpoly::editor::lowpoly::create_lowpoly_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-lowpoly-lowpoly::LowpolyPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-lowpoly-lowpoly::LowpolyPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-lowpoly-lowpoly::LowpolyViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-lowpoly-lowpoly::LowpolyViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_lowpoly_lowpoly::viewer::lowpoly::LowpolyViewer>, <semio_s_artifact_lowpoly_lowpoly::viewer::lowpoly::LowpolyViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_lowpoly_lowpoly::viewer::lowpoly::create_lowpoly_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-lowpoly-lowpoly::LowpolyViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-lowpoly-lowpoly::LowpolyViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-reasoning-wires::ReasoningWiresPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-reasoning-wires::ReasoningWiresPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_reasoning_wires::editor::wires::ReasoningWiresPlayApp>, <semio_s_artifact_reasoning_wires::editor::wires::ReasoningWiresPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_reasoning_wires::editor::wires::create_wires_app, Some(include_str!("/Users/ueli/Documents/semio/✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/⚖️declared-verb-examples.json")))))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-reasoning-wires::ReasoningWiresPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-reasoning-wires::ReasoningWiresPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-reasoning-wires::WiresViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-reasoning-wires::WiresViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_reasoning_wires::viewer::wires::WiresViewer>, <semio_s_artifact_reasoning_wires::viewer::wires::WiresViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_reasoning_wires::viewer::wires::create_wires_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-reasoning-wires::WiresViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-reasoning-wires::WiresViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-forms-forms::FormsPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-forms-forms::FormsPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_forms_forms::editor::forms::FormsPlayApp>, <semio_s_artifact_forms_forms::editor::forms::FormsPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_forms_forms::editor::forms::create_forms_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-forms-forms::FormsPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-forms-forms::FormsPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-forms-forms::FormsViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-forms-forms::FormsViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_forms_forms::viewer::forms::FormsViewer>, <semio_s_artifact_forms_forms::viewer::forms::FormsViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_forms_forms::viewer::forms::create_forms_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-forms-forms::FormsViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-forms-forms::FormsViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-layout-layout::LayoutPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-layout-layout::LayoutPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_layout_layout::editor::layout::LayoutPlayApp>, <semio_s_artifact_layout_layout::editor::layout::LayoutPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_layout_layout::editor::layout::create_layout_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-layout-layout::LayoutPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-layout-layout::LayoutPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-layout-layout::LayoutViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-layout-layout::LayoutViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_layout_layout::viewer::layout::LayoutViewer>, <semio_s_artifact_layout_layout::viewer::layout::LayoutViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_layout_layout::viewer::layout::create_layout_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-layout-layout::LayoutViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-layout-layout::LayoutViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-cad-cad::CadPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-cad-cad::CadPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_cad_cad::editor::cad::CadPlayApp>, <semio_s_artifact_cad_cad::editor::cad::CadPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_cad_cad::editor::cad::create_cad_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-cad-cad::CadPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-cad-cad::CadPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-cad-cad::CadViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-cad-cad::CadViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_cad_cad::viewer::cad::CadViewer>, <semio_s_artifact_cad_cad::viewer::cad::CadViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_cad_cad::viewer::cad::create_cad_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-cad-cad::CadViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-cad-cad::CadViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1990::En1990PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1990::En1990PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp>, <semio_s_artifact_norm_en1990::editor::en1990::En1990PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1990::editor::en1990::create_en1990_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1990::En1990PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1990::En1990PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1990::En1990Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1990::En1990Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer>, <semio_s_artifact_norm_en1990::viewer::en1990::En1990Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1990::viewer::en1990::create_en1990_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1990::En1990Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1990::En1990Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din18599::Din18599PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din18599::Din18599PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp>, <semio_s_artifact_norm_din18599::editor::din18599::Din18599PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_din18599::editor::din18599::create_din18599_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din18599::Din18599PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din18599::Din18599PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din18599::Din18599Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din18599::Din18599Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer>, <semio_s_artifact_norm_din18599::viewer::din18599::Din18599Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_din18599::viewer::din18599::create_din18599_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din18599::Din18599Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din18599::Din18599Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1997::En1997PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1997::En1997PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp>, <semio_s_artifact_norm_en1997::editor::en1997::En1997PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1997::editor::en1997::create_en1997_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1997::En1997PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1997::En1997PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1997::En1997Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1997::En1997Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer>, <semio_s_artifact_norm_en1997::viewer::en1997::En1997Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1997::viewer::en1997::create_en1997_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1997::En1997Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1997::En1997Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din16798::Din16798PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din16798::Din16798PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp>, <semio_s_artifact_norm_din16798::editor::din16798::Din16798PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_din16798::editor::din16798::create_din16798_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din16798::Din16798PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din16798::Din16798PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din16798::Din16798Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din16798::Din16798Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer>, <semio_s_artifact_norm_din16798::viewer::din16798::Din16798Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_din16798::viewer::din16798::create_din16798_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din16798::Din16798Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din16798::Din16798Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1991::En1991PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1991::En1991PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp>, <semio_s_artifact_norm_en1991::editor::en1991::En1991PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1991::editor::en1991::create_en1991_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1991::En1991PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1991::En1991PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1991::En1991Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1991::En1991Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer>, <semio_s_artifact_norm_en1991::viewer::en1991::En1991Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1991::viewer::en1991::create_en1991_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1991::En1991Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1991::En1991Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1992::En1992PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1992::En1992PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp>, <semio_s_artifact_norm_en1992::editor::en1992::En1992PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1992::editor::en1992::create_en1992_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1992::En1992PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1992::En1992PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1992::En1992Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1992::En1992Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer>, <semio_s_artifact_norm_en1992::viewer::en1992::En1992Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1992::viewer::en1992::create_en1992_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1992::En1992Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1992::En1992Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-vdi3805::Vdi3805PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-vdi3805::Vdi3805PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp>, <semio_s_artifact_norm_vdi3805::editor::vdi3805::Vdi3805PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_vdi3805::editor::vdi3805::create_vdi3805_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-vdi3805::Vdi3805PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-vdi3805::Vdi3805PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-vdi3805::Vdi3805Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-vdi3805::Vdi3805Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer>, <semio_s_artifact_norm_vdi3805::viewer::vdi3805::Vdi3805Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_vdi3805::viewer::vdi3805::create_vdi3805_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-vdi3805::Vdi3805Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-vdi3805::Vdi3805Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-iso16757::Iso16757PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-iso16757::Iso16757PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp>, <semio_s_artifact_norm_iso16757::editor::iso16757::Iso16757PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_iso16757::editor::iso16757::create_iso16757_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-iso16757::Iso16757PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-iso16757::Iso16757PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-iso16757::Iso16757Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-iso16757::Iso16757Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer>, <semio_s_artifact_norm_iso16757::viewer::iso16757::Iso16757Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_iso16757::viewer::iso16757::create_iso16757_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-iso16757::Iso16757Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-iso16757::Iso16757Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1993::En1993PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1993::En1993PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp>, <semio_s_artifact_norm_en1993::editor::en1993::En1993PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1993::editor::en1993::create_en1993_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1993::En1993PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1993::En1993PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1993::En1993Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1993::En1993Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer>, <semio_s_artifact_norm_en1993::viewer::en1993::En1993Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1993::viewer::en1993::create_en1993_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1993::En1993Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1993::En1993Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1994::En1994PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1994::En1994PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp>, <semio_s_artifact_norm_en1994::editor::en1994::En1994PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1994::editor::en1994::create_en1994_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1994::En1994PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1994::En1994PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1994::En1994Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1994::En1994Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer>, <semio_s_artifact_norm_en1994::viewer::en1994::En1994Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1994::viewer::en1994::create_en1994_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1994::En1994Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1994::En1994Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din4108::Din4108PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din4108::Din4108PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp>, <semio_s_artifact_norm_din4108::editor::din4108::Din4108PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_din4108::editor::din4108::create_din4108_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din4108::Din4108PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din4108::Din4108PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-din4108::Din4108Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-din4108::Din4108Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer>, <semio_s_artifact_norm_din4108::viewer::din4108::Din4108Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_din4108::viewer::din4108::create_din4108_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-din4108::Din4108Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-din4108::Din4108Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1996::En1996PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1996::En1996PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp>, <semio_s_artifact_norm_en1996::editor::en1996::En1996PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1996::editor::en1996::create_en1996_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1996::En1996PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1996::En1996PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1996::En1996Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1996::En1996Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer>, <semio_s_artifact_norm_en1996::viewer::en1996::En1996Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1996::viewer::en1996::create_en1996_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1996::En1996Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1996::En1996Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1995::En1995PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1995::En1995PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp>, <semio_s_artifact_norm_en1995::editor::en1995::En1995PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1995::editor::en1995::create_en1995_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1995::En1995PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1995::En1995PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1995::En1995Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1995::En1995Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer>, <semio_s_artifact_norm_en1995::viewer::en1995::En1995Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1995::viewer::en1995::create_en1995_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1995::En1995Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1995::En1995Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1999::En1999PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1999::En1999PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp>, <semio_s_artifact_norm_en1999::editor::en1999::En1999PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1999::editor::en1999::create_en1999_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1999::En1999PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1999::En1999PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1999::En1999Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1999::En1999Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer>, <semio_s_artifact_norm_en1999::viewer::en1999::En1999Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1999::viewer::en1999::create_en1999_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1999::En1999Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1999::En1999Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1998::En1998PlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1998::En1998PlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp>, <semio_s_artifact_norm_en1998::editor::en1998::En1998PlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_norm_en1998::editor::en1998::create_en1998_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1998::En1998PlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1998::En1998PlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-norm-en1998::En1998Viewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-norm-en1998::En1998Viewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer>, <semio_s_artifact_norm_en1998::viewer::en1998::En1998Viewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_norm_en1998::viewer::en1998::create_en1998_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-norm-en1998::En1998Viewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-norm-en1998::En1998Viewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-playbook-playbook::PlaybookPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-playbook-playbook::PlaybookPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_playbook_playbook::editor::playbook::PlaybookPlayApp>, <semio_s_artifact_playbook_playbook::editor::playbook::PlaybookPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_playbook_playbook::editor::playbook::create_playbook_play_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-playbook-playbook::PlaybookPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-playbook-playbook::PlaybookPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-playbook-playbook::PlaybookViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-playbook-playbook::PlaybookViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_playbook_playbook::viewer::playbook::PlaybookViewer>, <semio_s_artifact_playbook_playbook::viewer::playbook::PlaybookViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_playbook_playbook::viewer::playbook::create_playbook_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-playbook-playbook::PlaybookViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-playbook-playbook::PlaybookViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-imperative-procedure::ImperativePlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-imperative-procedure::ImperativePlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_imperative_procedure::editor::procedure::ImperativePlayApp>, <semio_s_artifact_imperative_procedure::editor::procedure::ImperativePlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_imperative_procedure::editor::procedure::create_imperative_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-imperative-procedure::ImperativePlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-imperative-procedure::ImperativePlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-imperative-procedure::ImperativeViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-imperative-procedure::ImperativeViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_imperative_procedure::viewer::procedure::ImperativeViewer>, <semio_s_artifact_imperative_procedure::viewer::procedure::ImperativeViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_imperative_procedure::viewer::procedure::create_imperative_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-imperative-procedure::ImperativeViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-imperative-procedure::ImperativeViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-remodel-remodeling::RemodelingPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-remodel-remodeling::RemodelingPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_remodel_remodeling::editor::remodeling::RemodelingPlayApp>, <semio_s_artifact_remodel_remodeling::editor::remodeling::RemodelingPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_remodel_remodeling::editor::remodeling::create_remodeling_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-remodel-remodeling::RemodelingPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-remodel-remodeling::RemodelingPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-remodel-remodeling::RemodelingViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-remodel-remodeling::RemodelingViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_remodel_remodeling::viewer::remodeling::RemodelingViewer>, <semio_s_artifact_remodel_remodeling::viewer::remodeling::RemodelingViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_remodel_remodeling::viewer::remodeling::create_remodeling_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-remodel-remodeling::RemodelingViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-remodel-remodeling::RemodelingViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-energy-model::EnergyModelEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-energy-model::EnergyModelEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_energy_model::editor::model::EnergyModelEditor>, <semio_s_artifact_energy_model::editor::model::EnergyModelEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_energy_model::editor::model::create_energy_model_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-energy-model::EnergyModelEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-energy-model::EnergyModelEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-energy-model::EnergyModelViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-energy-model::EnergyModelViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_energy_model::viewer::model::EnergyModelViewer>, <semio_s_artifact_energy_model::viewer::model::EnergyModelViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_energy_model::viewer::model::create_energy_model_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-energy-model::EnergyModelViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-energy-model::EnergyModelViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-trinity-rewriting::TrinityRewritingPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-trinity-rewriting::TrinityRewritingPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_trinity_rewriting::editor::rewriting::TrinityRewritingPlayApp>, <semio_s_artifact_trinity_rewriting::editor::rewriting::TrinityRewritingPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_trinity_rewriting::editor::rewriting::create_rewriting_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-trinity-rewriting::TrinityRewritingPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-trinity-rewriting::TrinityRewritingPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-trinity-rewriting::TrinityRewritingViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-trinity-rewriting::TrinityRewritingViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer>, <semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_trinity_rewriting::viewer::rewriting::create_trinity_rewriting_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-trinity-rewriting::TrinityRewritingViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-trinity-rewriting::TrinityRewritingViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-trinity-jack::TrinityJackPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-trinity-jack::TrinityJackPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_trinity_jack::editor::jack::TrinityJackPlayApp>, <semio_s_artifact_trinity_jack::editor::jack::TrinityJackPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_trinity_jack::editor::jack::create_trinity_jack_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-trinity-jack::TrinityJackPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-trinity-jack::TrinityJackPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-trinity-jack::TrinityJackViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-trinity-jack::TrinityJackViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer>, <semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_trinity_jack::viewer::jack::create_trinity_jack_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-trinity-jack::TrinityJackViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-trinity-jack::TrinityJackViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-dag-dag::DagPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-dag-dag::DagPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_dag_dag::editor::dag::DagPlayApp>, <semio_s_artifact_dag_dag::editor::dag::DagPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_dag_dag::editor::dag::create_dag_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-dag-dag::DagPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-dag-dag::DagPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-dag-dag::DagViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-dag-dag::DagViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_dag_dag::viewer::dag::DagViewer>, <semio_s_artifact_dag_dag::viewer::dag::DagViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_dag_dag::viewer::dag::create_dag_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-dag-dag::DagViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-dag-dag::DagViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-draw-drawing::DrawingPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-draw-drawing::DrawingPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_draw_drawing::editor::drawing::DrawingPlayApp>, <semio_s_artifact_draw_drawing::editor::drawing::DrawingPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_draw_drawing::editor::drawing::create_drawing_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-draw-drawing::DrawingPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-draw-drawing::DrawingPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-draw-drawing::DrawingViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-draw-drawing::DrawingViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_draw_drawing::viewer::drawing::DrawingViewer>, <semio_s_artifact_draw_drawing::viewer::drawing::DrawingViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_draw_drawing::viewer::drawing::create_drawing_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-draw-drawing::DrawingViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-draw-drawing::DrawingViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-raster-raster::RasterPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-raster-raster::RasterPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_raster_raster::editor::raster::RasterPlayApp>, <semio_s_artifact_raster_raster::editor::raster::RasterPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_raster_raster::editor::raster::create_raster_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-raster-raster::RasterPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-raster-raster::RasterPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-raster-raster::RasterViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-raster-raster::RasterViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_raster_raster::viewer::raster::RasterViewer>, <semio_s_artifact_raster_raster::viewer::raster::RasterViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_raster_raster::viewer::raster::create_raster_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-raster-raster::RasterViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-raster-raster::RasterViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-html::HtmlEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-html::HtmlEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>, <semio_s_artifact_stdio_html::editor::html::HtmlEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_html::editor::html::create_html_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-html::HtmlEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-html::HtmlEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-html::HtmlViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-html::HtmlViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>, <semio_s_artifact_stdio_html::viewer::html::HtmlViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_html::viewer::html::create_html_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-html::HtmlViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-html::HtmlViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-csv::CsvEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-csv::CsvEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>, <semio_s_artifact_stdio_csv::editor::csv::CsvEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_csv::editor::csv::create_csv_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-csv::CsvEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-csv::CsvEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-csv::CsvViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-csv::CsvViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>, <semio_s_artifact_stdio_csv::viewer::csv::CsvViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_csv::viewer::csv::create_csv_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-csv::CsvViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-csv::CsvViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-tsv::TsvEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-tsv::TsvEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>, <semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_tsv::editor::tsv::create_tsv_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-tsv::TsvEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-tsv::TsvEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-tsv::TsvViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-tsv::TsvViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>, <semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_tsv::viewer::tsv::create_tsv_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-tsv::TsvViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-tsv::TsvViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-md::MdEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-md::MdEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_md::editor::md::MdEditor>, <semio_s_artifact_stdio_md::editor::md::MdEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_md::editor::md::create_md_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-md::MdEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-md::MdEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-md::MdViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-md::MdViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_md::viewer::md::MdViewer>, <semio_s_artifact_stdio_md::viewer::md::MdViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_md::viewer::md::create_md_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-md::MdViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-md::MdViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-xml::XmlValidEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-xml::XmlValidEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>, <semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_xml::editor::xml_valid::create_xml_valid_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-xml::XmlValidEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-xml::XmlValidEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-xml::XmlValidViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-xml::XmlValidViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>, <semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_xml::viewer::xml_valid::create_xml_valid_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-xml::XmlValidViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-xml::XmlValidViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-xml::XmlAnyEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-xml::XmlAnyEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>, <semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_xml::editor::xml_any::create_xml_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-xml::XmlAnyEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-xml::XmlAnyEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-xml::XmlAnyViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-xml::XmlAnyViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>, <semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_xml::viewer::xml_any::create_xml_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-xml::XmlAnyViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-xml::XmlAnyViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-txt::TxtEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-txt::TxtEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>, <semio_s_artifact_stdio_txt::editor::txt::TxtEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_txt::editor::txt::create_txt_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-txt::TxtEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-txt::TxtEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-txt::TxtViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-txt::TxtViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>, <semio_s_artifact_stdio_txt::viewer::txt::TxtViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_txt::viewer::txt::create_txt_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-txt::TxtViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-txt::TxtViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-json::JsonIJsonEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-json::JsonIJsonEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>, <semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_json::editor::json_i_json::create_json_i_json_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-json::JsonIJsonEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-json::JsonIJsonEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-json::JsonIJsonViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-json::JsonIJsonViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>, <semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_json::viewer::json_i_json::create_json_i_json_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-json::JsonIJsonViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-json::JsonIJsonViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-json::JsonAnyEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-json::JsonAnyEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>, <semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_stdio_json::editor::json_any::create_json_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-json::JsonAnyEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-json::JsonAnyEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-stdio-json::JsonAnyViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-stdio-json::JsonAnyViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>, <semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_stdio_json::viewer::json_any::create_json_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-stdio-json::JsonAnyViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-stdio-json::JsonAnyViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-note-note::NotePlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-note-note::NotePlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_note_note::editor::note::NotePlayApp>, <semio_s_artifact_note_note::editor::note::NotePlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_note_note::editor::note::create_note_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-note-note::NotePlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-note-note::NotePlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-note-note::NoteViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-note-note::NoteViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_note_note::viewer::note::NoteViewer>, <semio_s_artifact_note_note::viewer::note::NoteViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_note_note::viewer::note::create_note_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-note-note::NoteViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-note-note::NoteViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-2d::Puzzle2dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-2d::Puzzle2dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp>, <semio_s_artifact_puzzle_2d::editor::puzzle2d::Puzzle2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_puzzle_2d::editor::puzzle2d::create_puzzle2d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-2d::Puzzle2dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-2d::Puzzle2dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-2d::Puzzle2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-2d::Puzzle2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer>, <semio_s_artifact_puzzle_2d::viewer::puzzle2d::Puzzle2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_puzzle_2d::viewer::puzzle2d::create_puzzle2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-2d::Puzzle2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-2d::Puzzle2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-5d::Puzzle5dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-5d::Puzzle5dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp>, <semio_s_artifact_puzzle_5d::editor::puzzle5d::Puzzle5dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_puzzle_5d::editor::puzzle5d::create_puzzle5d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-5d::Puzzle5dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-5d::Puzzle5dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-5d::Puzzle5dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-5d::Puzzle5dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer>, <semio_s_artifact_puzzle_5d::viewer::puzzle5d::Puzzle5dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_puzzle_5d::viewer::puzzle5d::create_puzzle5d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-5d::Puzzle5dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-5d::Puzzle5dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-3d::Puzzle3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-3d::Puzzle3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp>, <semio_s_artifact_puzzle_3d::editor::puzzle3d::Puzzle3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_puzzle_3d::editor::puzzle3d::create_puzzle3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-3d::Puzzle3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-3d::Puzzle3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-puzzle-3d::Puzzle3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-puzzle-3d::Puzzle3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer>, <semio_s_artifact_puzzle_3d::viewer::puzzle3d::Puzzle3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_puzzle_3d::viewer::puzzle3d::create_puzzle3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-puzzle-3d::Puzzle3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-puzzle-3d::Puzzle3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-2d::Block2dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-2d::Block2dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp>, <semio_s_artifact_block_2d::editor::block2d::Block2dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_block_2d::editor::block2d::create_block2d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-2d::Block2dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-2d::Block2dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-2d::Block2dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-2d::Block2dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_block_2d::viewer::block2d::Block2dViewer>, <semio_s_artifact_block_2d::viewer::block2d::Block2dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_block_2d::viewer::block2d::create_block2d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-2d::Block2dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-2d::Block2dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-5d::Block5dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-5d::Block5dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp>, <semio_s_artifact_block_5d::editor::block5d::Block5dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_block_5d::editor::block5d::create_block5d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-5d::Block5dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-5d::Block5dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-5d::Block5dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-5d::Block5dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_block_5d::viewer::block5d::Block5dViewer>, <semio_s_artifact_block_5d::viewer::block5d::Block5dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_block_5d::viewer::block5d::create_block5d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-5d::Block5dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-5d::Block5dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-3d::Block3dPlayApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-3d::Block3dPlayApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp>, <semio_s_artifact_block_3d::editor::block3d::Block3dPlayApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_block_3d::editor::block3d::create_block3d_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-3d::Block3dPlayApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-3d::Block3dPlayApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-block-3d::Block3dViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-block-3d::Block3dViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_block_3d::viewer::block3d::Block3dViewer>, <semio_s_artifact_block_3d::viewer::block3d::Block3dViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_block_3d::viewer::block3d::create_block3d_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-block-3d::Block3dViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-block-3d::Block3dViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-space-space::SpaceIndexEditor".contains(filter)) {
        eprintln!("probing semio-s-artifact-space-space::SpaceIndexEditor");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor>, <semio_s_artifact_space_space::editor::space_index::SpaceIndexEditor as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_space_space::editor::space_index::create_space_index_editor, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-space-space::SpaceIndexEditor", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-space-space::SpaceIndexEditor\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-space-space::SpaceIndexViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-space-space::SpaceIndexViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer>, <semio_s_artifact_space_space::viewer::space_index::SpaceIndexViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_space_space::viewer::space_index::create_space_index_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-space-space::SpaceIndexViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-space-space::SpaceIndexViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-sourcing-curation::SourcingCurationApp".contains(filter)) {
        eprintln!("probing semio-s-artifact-sourcing-curation::SourcingCurationApp");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::EditorApp<semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationApp>, <semio_s_artifact_sourcing_curation::editor::sourcing::SourcingCurationApp as semio_framework_plugin::ArtifactEditor>::Members>(semio_s_artifact_sourcing_curation::editor::sourcing::create_sourcing_curation_app, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-sourcing-curation::SourcingCurationApp", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-sourcing-curation::SourcingCurationApp\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
    if filter.as_deref().is_none_or(|filter| "semio-s-artifact-sourcing-curation::SourcingViewer".contains(filter)) {
        eprintln!("probing semio-s-artifact-sourcing-curation::SourcingViewer");
        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::ViewerApp<semio_s_artifact_sourcing_curation::viewer::sourcing::SourcingViewer>, <semio_s_artifact_sourcing_curation::viewer::sourcing::SourcingViewer as semio_framework_plugin::ArtifactViewer>::Members>(semio_s_artifact_sourcing_curation::viewer::sourcing::create_sourcing_viewer, None)))).expect("probe thread").join();
        match run { Ok(probes) => report("semio-s-artifact-sourcing-curation::SourcingViewer", probes), Err(panic) => println!("{{\"surface\":\"semio-s-artifact-sourcing-curation::SourcingViewer\",\"panic\":{:?}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }
    }
}

fn main() {
    let filter = std::env::args().nth(1);
    run(filter);
}
