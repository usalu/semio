/// 🎛️ Starts native task controls immediately, then connects and discovers asynchronously.
pub fn run(root: &Path) -> i32 { run_with(root, &crate::args::ParsedArgs::default()) }

/// ⚙️ Starts a native dashboard with strict optional configuration overrides.
pub fn run_with(root: &Path, parsed: &crate::args::ParsedArgs) -> i32 {
    let started = std::time::Instant::now();
    if parsed.has_flag("help") { println!("semio [dashboard] [--root PATH] [--config JOURNAL] [--language en|de] [--appearance dark|light] [--terminology native|reuse] [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows]\nCtrl+Space: n commands, h tasks, p settings, f refresh, e cancel discovery, r restart, c cancel, k kill, d detach, Q shutdown"); return 0; }
    let root = crate::ipc::canonical_path(&parsed.flag("root").map_or_else(|| root.to_path_buf(), PathBuf::from));
    let preferences = match crate::preferences::load(&root, parsed) { Ok(preferences) => preferences, Err(error) => { eprintln!("[dashboard] {error}"); return 2; } };
    let Ok(mut term) = NativeTerminal::new() else { eprintln!("[dashboard] failed to attach to the terminal"); return 1; };
    if term.enter().is_err() { return 1; }
    let size = term.size().unwrap_or(Size { width: 100, height: 32 });
    let light = preferences.appearance == "light";
    let mut tui = Tui::new(size, Theme::new(if light { AppearanceName::Light } else { AppearanceName::Dark }));
    let navbar = NavbarState { left: vec![NavItem { id: "logo".into(), label: "semio".into(), active: true }], center: vec![NavItem { id: "mode".into(), label: "dashboard".into(), active: false }], right: vec![] };
    let footer = FooterState { hints: vec![], status: "connecting · discovering commands".into() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["Tasks".into()]));
    let shell = shell(&mut tui.scene, navbar, footer, &layout);
    let locale = if preferences.language == "de" { Locale::German } else { Locale::English };
    let tree = crate::command_tree::seed(&root);
    let commands = crate::inventory::commands(&tree);
    let mut dash = Dashboard { root: root.clone(), tree, commands, inventory: None, inventory_status: "discovering commands".into(), preferences, preference_path: crate::preferences::local_path(&root, parsed), saving_preferences: None, shared_preferences: false, layout, shell, windows: Vec::new(), next_serial: 2, focused: "w1".into(), leader: LeaderMode::Idle, terminal_input: false, connection: None, connecting: None, connection_status: "connecting".into(), next_reconnect: std::time::Instant::now(), restoring: true, sessions: Default::default(), hidden: Default::default(), locale, light };
    let (w1_id, w1_chrome) = dash.shell.windows[0].clone();
    let w1 = dash.attach_view(&mut tui, w1_chrome, &w1_id, "overview");
    dash.windows.push(w1); dash.remount(&mut tui);
    tui.set_focus(Some(dash.windows[0].focus)); dash.sync_chrome_focus(&mut tui);
    if let Some(ChromeState::Footer(footer)) = tui.scene.node_mut(dash.shell.footer).chrome() { footer.hints = dash.footer_hints(); }
    term.present(&tui.render_full()).ok();
    if std::env::var_os("SEMIO_DASHBOARD_TRACE").is_some() { eprintln!("[DEBUG] dashboard usable first frame elapsed_us={}", started.elapsed().as_micros()); }
    dash.inventory = Some(crate::inventory::start(root, false));

