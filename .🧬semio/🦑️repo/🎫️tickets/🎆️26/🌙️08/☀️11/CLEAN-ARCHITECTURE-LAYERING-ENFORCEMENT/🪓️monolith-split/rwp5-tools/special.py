# 🎯️ The few hand-shaped rewrites of the WP5 conversion: free functions that took the whole ShellState and the test call sites of those.
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx
from mover import read, write

def sub1(t, old, new, count=1):
    assert t.count(old) == count, (old, t.count(old))
    return t.replace(old, new)

def run(root, report):
    wa = cfg.stg(root, cfg.CRATE2DIR["shell-window-actions"]) + "/🎓️tutorial-runtime/🦀️.rs"
    t = read(wa)
    t = sub1(t, "fn tutorial_capture_ui_snapshot(state: &ShellState)", "fn tutorial_capture_ui_snapshot(state: &WindowActionsCtx<'_>)")
    t = sub1(t, "fn tutorial_apply_ui_snapshot(state: &mut ShellState,", "fn tutorial_apply_ui_snapshot(state: &mut WindowActionsCtx<'_>,")
    t = sub1(t, "fn tutorial_apply_ui_change_to_shell(state: &mut ShellState,", "fn tutorial_apply_ui_change_to_shell(state: &mut WindowActionsCtx<'_>,")
    t = sub1(t, "fn tutorial_recorder_sample(state: &mut ShellState,", "fn tutorial_recorder_sample(state: &mut WindowActionsCtx<'_>,")
    t = sub1(t, "fn render_tutorial_gesture_overlay_node(state: &ShellState, overlay: &mut DrawList, theme: &Theme)", "fn render_tutorial_gesture_overlay_node(chrome: &ChromeState, surfaces: &SurfaceState, dock: &ShellDockState, tutorial: &TutorialState, overlay: &mut DrawList, theme: &Theme)")
    t = sub1(t, "state.tutorial.tutorial.as_ref()", "tutorial.tutorial.as_ref()")
    t = sub1(t, "tutorial_resolve_gesture_point(&state.chrome, &state.surfaces, &state.dock,", "tutorial_resolve_gesture_point(chrome, surfaces, dock,", 2)
    write(wa, t)
    pv = cfg.stg(root, cfg.CRATE2DIR["shell-view-panels"]) + "/🖼️chrome-frame-and-main-window/🦀️.rs"
    t = read(pv)
    t = sub1(t, "render_tutorial_gesture_overlay_node(self, overlay, theme);", "render_tutorial_gesture_overlay_node(&*self.chrome, &*self.surfaces, &*self.dock, &*self.tutorial, overlay, theme);")
    write(pv, t)
    ov = cfg.stg(root, cfg.CRATE2DIR["shell-view-overlays"]) + "/🗨️chrome-overlays-and-tour/🦀️.rs"
    t = read(ov)
    t = sub1(t, "fn chrome_tour_anchor_rect(state: &ShellState,", "fn chrome_tour_anchor_rect(state: &OverlaysViewCtx<'_>,")
    t = sub1(t, "fn chrome_tour_layout(state: &ShellState,", "fn chrome_tour_layout(state: &OverlaysViewCtx<'_>,")
    write(ov, t)
    sh = root + "/"
    tw = sh + "🧪️tests/🔬️wgpu-tutorial/🦀️.rs"
    t = read(tw)
    t = sub1(t, "tutorial_capture_ui_snapshot(&state)", "tutorial_capture_ui_snapshot(&WindowActionsCtx::of(&mut state))")
    t = sub1(t, "tutorial_apply_ui_snapshot(&mut fresh, &snapshot)", "tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut fresh), &snapshot)")
    t = sub1(t, "tutorial_apply_ui_snapshot(&mut state, &empty_snapshot)", "tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut state), &empty_snapshot)")
    t = re.sub(r"tutorial_apply_ui_change_to_shell\(&mut state, &change\)", "tutorial_apply_ui_change_to_shell(&mut WindowActionsCtx::of(&mut state), &change)", t)
    write(tw, t)
    ta = sh + "🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs"
    t = read(ta)
    t = sub1(t, "tutorial_apply_ui_snapshot(&mut shell, &snapshot)", "tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut shell), &snapshot)")
    t = sub1(t, "tutorial_apply_ui_snapshot(&mut shell, &semio_framework::onboarding::TutorialUiSnapshot::default())", "tutorial_apply_ui_snapshot(&mut WindowActionsCtx::of(&mut shell), &semio_framework::onboarding::TutorialUiSnapshot::default())")
    write(ta, t)
    tt = sh + "🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs"
    t = read(tt)
    t = sub1(t, "chrome_tour_layout(shell, atlas, theme,", "chrome_tour_layout(&OverlaysViewCtx::of(shell), atlas, theme,")
    write(tt, t)
    report.append("special rewrites applied: tutorial and tour free functions, 4 test files")

def outside(report):
    bw = cfg.ENG + "/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"
    ov = f"{cfg.GEN}/new-ext/{bw}"
    t = read(ov if os.path.exists(ov) else f"{cfg.ROOT}/{bw}")
    t = sub1(t, 'let result = self.shell.as_mut().expect("bootstrap shell exists").boot().await;', 'let result = crate::shell::ActionsCtx::of(self.shell.as_mut().expect("bootstrap shell exists")).boot().await;')
    write(ov, t)
    report.append("special rewrites applied: browser-worker boot")

def tests_late(root, report):
    p = root + "/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs"
    t = read(p)
    t = sub1(t, 'let action_only = super::window_actions_search_pane_tests::actions_shell();\n    let action_chips = action_only.window_pane_chips("pane-top");', 'let mut action_only = super::window_actions_search_pane_tests::actions_shell();\n    let action_chips = SessionCtx::of(&mut action_only).window_pane_chips("pane-top");')
    write(p, t)

def boot_load(root, report):
    p = cfg.stg(root, "🐙️state") + "/🧩️session-and-topology/🦀️.rs"
    t = read(p)
    t = sub1(t, "        state.load_persisted_dock();\n        state\n", "        state\n")
    write(p, t)
    q = cfg.stg(root, cfg.CRATE2DIR["shell-actions"]) + "/🧩️session-and-topology/🦀️.rs"
    t = read(q)
    t = sub1(t, "    pub async fn boot(&mut self) -> Result<(), String> {\n        self.plugins.plugin_faults.clear();", "    pub async fn boot(&mut self) -> Result<(), String> {\n        self.as_session().load_persisted_dock();\n        self.plugins.plugin_faults.clear();")
    write(q, t)
    report.append("ShellState::new no longer loads the persisted dock (an upward call into shell-session); ActionsCtx::boot does, first")

def tests_hazards(root, report):
    p = root + "/🧪️tests/⚙️settings-general-layout/🦀️.rs"
    t = read(p)
    t = sub1(t, "    let mut footer_cursor = ShellChromeChildCursor::default();\n    for _ in 0..4_096 {\n        if PanelsViewCtx::of(&mut shell).render_footer_step(&mut footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, shell.chrome.screen_w, shell.chrome.screen_h) {",
                "    let (screen_w, screen_h) = (shell.chrome.screen_w, shell.chrome.screen_h);\n    let mut footer_cursor = ShellChromeChildCursor::default();\n    for _ in 0..4_096 {\n        if PanelsViewCtx::of(&mut shell).render_footer_step(&mut footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, screen_w, screen_h) {")
    t = sub1(t, "render_footer_step(&mut settled_footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, shell.chrome.screen_w, shell.chrome.screen_h)", "render_footer_step(&mut settled_footer_cursor, &mut draw, &mut atlas, &icons, &mut input, &theme, screen_w, screen_h)")
    write(p, t)
    p = root + "/🧪️tests/🛟️panel-window-reservation/🦀️.rs"
    t = read(p)
    t = sub1(t, "caller.turn(|cx|PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, theme, SessionCtx::of(&mut shell).body_rect(theme), &mut world, cx));",
                "caller.turn(|cx| {\n            let body = SessionCtx::of(&mut shell).body_rect(theme);\n            PanelsViewCtx::of(&mut shell).render_main_window_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, theme, body, &mut world, cx)\n        });")
    t = sub1(t, "        let panel = ChromeViewCtx::of(&mut shell).anchor_rect(anchor, SessionCtx::of(&mut shell).body_rect(&theme), &theme);", "        let body = SessionCtx::of(&mut shell).body_rect(&theme);\n        let panel = ChromeViewCtx::of(&mut shell).anchor_rect(anchor, body, &theme);")
    write(p, t)
    p = root + "/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"
    t = read(p)
    t = sub1(t, "    let mobile = PanelsViewCtx::of(&mut shell).footer_chrome_layout(&mut atlas, &theme, shell.chrome.screen_w, btn_y, theme.control_height);", "    let mobile_width = shell.chrome.screen_w;\n    let mobile = PanelsViewCtx::of(&mut shell).footer_chrome_layout(&mut atlas, &theme, mobile_width, btn_y, theme.control_height);")
    write(p, t)
    p = root + "/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs"
    t = read(p)
    t = sub1(t, "    let semantic = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&shell.input.pane_overlay_hits);", "    let pane_hits = shell.input.pane_overlay_hits.clone();\n    let semantic = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&pane_hits);")
    write(p, t)
    report.append("test hazard sites hoisted: 6")

def borrow_fixes(root, report):
    st = lambda crate, slug: cfg.stg(root, cfg.CRATE2DIR[crate]) + "/" + slug + "/🦀️.rs"
    p = st("shell-actions", "🎬️shell-actions"); t = read(p)
    t = sub1(t, "if let Some(session) = self.docs.session.as_ref() {\n                        self.as_window_actions().queue_host_effects(&session.app.controller_id.clone(), remaining);",
                "if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {\n                        self.as_window_actions().queue_host_effects(&controller_id, remaining);")
    write(p, t)
    p = st("shell-actions", "📨️command-dispatch"); t = read(p)
    t = sub1(t, "if let (Some(plugin_id), Some(session)) = (operation.get(\"pluginId\").and_then(|v| v.as_str()), &self.docs.session) {\n                        self.as_session().spawn_plugin(plugin_id, session.view_state.clone()).await?;",
                "if let (Some(plugin_id), Some(view_state)) = (operation.get(\"pluginId\").and_then(|v| v.as_str()), self.docs.session.as_ref().map(|session| session.view_state.clone())) {\n                        self.as_session().spawn_plugin(plugin_id, view_state).await?;")
    write(p, t)
    p = st("shell-pointer", "🖱️pointer-input"); t = read(p)
    t = sub1(t, "                if let Some(session) = &self.docs.session {\n                    self.as_actions().dispatch_action(ActionDescriptor { controller_id: session.app.controller_id.clone(), action: \"setActiveExample\"",
                "                if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {\n                    self.as_actions().dispatch_action(ActionDescriptor { controller_id, action: \"setActiveExample\"")
    write(p, t)
    p = st("shell-pointer", "⌨️keyboard-and-palette"); t = read(p)
    t = sub1(t, "        if let Some(session) = &self.docs.session {\n            self.as_actions().dispatch_action(ActionDescriptor {\n                controller_id: session.app.controller_id.clone(),\n                action: \"setMediaNodeSelection\".into(),",
                "        if let Some(controller_id) = self.docs.session.as_ref().map(|session| session.app.controller_id.clone()) {\n            self.as_actions().dispatch_action(ActionDescriptor {\n                controller_id,\n                action: \"setMediaNodeSelection\".into(),")
    write(p, t)
    report.append("borrow fixes: 4 sites where a borrow of the session was alive across a bridged call")
