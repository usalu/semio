import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()
pairs = [
("""        self.refresh_app_catalogue(&program, session.instance_id, &panel_view).await;
        if scope.wants_section(UiDirtySection::Engagements) {""",
"""        let wg8_t = std::time::Instant::now();
        self.refresh_app_catalogue(&program, session.instance_id, &panel_view).await;
        eprintln!("[DEBUG] wg8 refresh catalogue {:?} rendered-inline={} detached={}", wg8_t.elapsed(), rendered, rendered_surfaces.len());
        let wg8_t = std::time::Instant::now();
        if scope.wants_section(UiDirtySection::Engagements) {"""),
("""        self.refresh_window_action_panes(&measure_windows, &mut faults)?;
        if scope.wants_section(UiDirtySection::Measures) {""",
"""        eprintln!("[DEBUG] wg8 refresh engagements {:?}", wg8_t.elapsed());
        let wg8_t = std::time::Instant::now();
        self.refresh_window_action_panes(&measure_windows, &mut faults)?;
        if scope.wants_section(UiDirtySection::Measures) {"""),
("""        let earned = self.host_effect_earned_scope(&refresh_effects);
        self.queue_host_effects(&session.app.controller_id, refresh_effects);""",
"""        eprintln!("[DEBUG] wg8 refresh panes+measures+tools+spawned {:?}", wg8_t.elapsed());
        let earned = self.host_effect_earned_scope(&refresh_effects);
        self.queue_host_effects(&session.app.controller_id, refresh_effects);"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
