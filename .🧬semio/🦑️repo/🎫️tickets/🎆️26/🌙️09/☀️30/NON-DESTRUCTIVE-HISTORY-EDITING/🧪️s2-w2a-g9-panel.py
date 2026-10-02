"""📡️ S2-W2A G9 runtime adoption, panel and wire half: the document store defers remote replays at construction and on
every store replacement, the history patch carries `remoteReplay`, and the history body shows the remote replay section
(`ui_history_panel` takes the remote status; every call site passes it)."""
import pathlib, subprocess

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
TT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs"
RT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"


def patch(path, pairs):
    t = path.read_text()
    for a, b in pairs:
        if t.count(b) >= 1 and t.count(a) == 0:
            continue
        assert t.count(a) == 1, (path.name, t.count(a), a[:90])
        t = t.replace(a, b)
    path.write_text(t)


patch(TT, [
    ("""    MoreInputs,
    MoreReferences,
""", """    MoreInputs,
    MoreReferences,
    RemoteReplay,
    RemoteReplayProgress,
    RemoteReplayPaused,
    RemoteReplayRefused,
"""),
    ("""                Self::MoreReferences => "{count} more: {names}",
""", """                Self::MoreReferences => "{count} more: {names}",
                Self::RemoteReplay => "Remote history change",
                Self::RemoteReplayProgress => "Replaying a remote history change: {done} of {total} mutations",
                Self::RemoteReplayPaused => "Remote history change paused: this replica still shows the history before it",
                Self::RemoteReplayRefused => "Remote history change refused: {code}",
"""),
    ("""                Self::MoreReferences => "{count} weitere: {names}",
""", """                Self::MoreReferences => "{count} weitere: {names}",
                Self::RemoteReplay => "Entfernte Verlaufsänderung",
                Self::RemoteReplayProgress => "Entfernte Verlaufsänderung wird angewendet: {done} von {total} Mutationen",
                Self::RemoteReplayPaused => "Entfernte Verlaufsänderung pausiert: dieses Replikat zeigt noch den Verlauf davor",
                Self::RemoteReplayRefused => "Entfernte Verlaufsänderung abgelehnt: {code}",
"""),
    ("""/// ✏️ The draft editor as two tree sections: `framework.history.editor`""", """/// 📡️ The remote history change waiting for its replay as the history body's tree section `framework.history.remoteReplay`
/// (design §16.6): one status row — progress in words, paused, or the code of a refused adoption — then Cancel replay
/// while it runs or Replay again while paused, both only while no session is open (a session's own Cancel replay and
/// Replay again address the session).
pub(crate) fn time_travel_remote_section(remote: &HistoryRemoteReplay, session_open: bool, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.remoteReplay";
    let (line, tone) = match (remote.fault.as_deref(), remote.paused) {
        (Some(code), _) if remote.total == 0 => (HistoryPanelText::RemoteReplayRefused.text(locale).replace("{code}", code), Tone::Danger),
        (_, true) => (HistoryPanelText::RemoteReplayPaused.text(locale).to_string(), Tone::Warning),
        _ => (HistoryPanelText::RemoteReplayProgress.text(locale).replace("{done}", &remote.done.to_string()).replace("{total}", &remote.total.to_string()), Tone::Info),
    };
    let mut rows = BuiltChildren::default();
    let status = ui::tree_item(Label(UiText::clipped(&line))).icon(ui_text("cloud-download", "time-travel-panel.remote-icon")?).tone(tone).try_id(format!("{scope}.status")).map_err(|_| error("time-travel-panel.remote-status-id"))?.try_build().map_err(|_| error("time-travel-panel.remote-status"))?;
    rows.try_push(status).map_err(|_| error("time-travel-panel.remote-rows"))?;
    if !session_open && remote.total > 0 {
        let control = match remote.paused {
            true => time_travel_button_row(controller_id, &format!("{scope}.rerun"), TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale), "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, None, true)?,
            false => time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, None, true)?,
        };
        rows.try_push(control).map_err(|_| error("time-travel-panel.remote-rows"))?;
    }
    tree_section(ui_label(HistoryPanelText::RemoteReplay.text(locale), "time-travel-panel.remote-label")?).default_open(true).try_id(scope).map_err(|_| error("time-travel-panel.remote-id"))?.try_children(rows).map_err(|_| error("time-travel-panel.remote-rows"))?.try_build().map_err(|_| error("time-travel-panel.remote"))
}

/// ✏️ The draft editor as two tree sections: `framework.history.editor`"""),
])

patch(RT, [
    ("""            let mut store = ArtifactStore::new(envelope).await.expect("failed to create document store");
            store.enable_convergence_early_exit();""", """            let mut store = ArtifactStore::new(envelope).await.expect("failed to create document store");
            store.enable_convergence_early_exit();
            store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS));"""),
    ("""                self.store.enable_convergence_early_exit();
                let displaced_children""", """                self.store.enable_convergence_early_exit();
                self.store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS));
                let displaced_children"""),
    ("""                time_travel: time_travel.map(|panel| panel.status),
            })""", """                time_travel: time_travel.map(|panel| panel.status),
                remote_replay: self.remote_replay_status(),
            })"""),
    ("""    pub async fn ui_history_panel(history: &HistoryView, time_travel: Option<&TimeTravelPanel>, controller_id: &str, locale: Locale, read_only: bool, view: &ViewModel) -> UiAssemblyResult<BuiltNode> {""", """    pub async fn ui_history_panel(history: &HistoryView, time_travel: Option<&TimeTravelPanel>, remote_replay: Option<&semio_framework::kernel::HistoryRemoteReplay>, controller_id: &str, locale: Locale, read_only: bool, view: &ViewModel) -> UiAssemblyResult<BuiltNode> {"""),
    ("""        let mut sections = BuiltChildren::default();
        let ordered: Vec<BuiltNode> = match time_travel {
            Some(panel) => {
                let mut session = vec![time_travel::time_travel_band_section(panel, controller_id, locale)?];
                if let Some(editor) = panel.editor.as_ref().filter(|_| !read_only) {
                    session.extend(time_travel::time_travel_editor_sections(panel, editor, controller_id, locale)?);
                }
                session.into_iter().chain(alternatives_section).chain([actions_section, commands_section]).collect()
            }
            None => std::iter::once(actions_section).chain(alternatives_section).chain([commands_section]).collect(),
        };""", """        let mut sections = BuiltChildren::default();
        let remote_section = match remote_replay.filter(|_| !read_only) {
            Some(remote) => Some(time_travel::time_travel_remote_section(remote, time_travel.is_some(), controller_id, locale)?),
            None => None,
        };
        let ordered: Vec<BuiltNode> = match time_travel {
            Some(panel) => {
                let mut session = vec![time_travel::time_travel_band_section(panel, controller_id, locale)?];
                if let Some(editor) = panel.editor.as_ref().filter(|_| !read_only) {
                    session.extend(time_travel::time_travel_editor_sections(panel, editor, controller_id, locale)?);
                }
                session.into_iter().chain(remote_section).chain(alternatives_section).chain([actions_section, commands_section]).collect()
            }
            None => remote_section.into_iter().chain([actions_section]).chain(alternatives_section).chain([commands_section]).collect(),
        };"""),
    ("""                let root = ui_history_panel(history, time_travel.as_ref(), &self.registry.controller_id, view_state.locale, A::ROLE == AppRole::Viewer, view_state).await""", """                let remote_replay = self.remote_replay_status();
                let root = ui_history_panel(history, time_travel.as_ref(), remote_replay.as_ref(), &self.registry.controller_id, view_state.locale, A::ROLE == AppRole::Viewer, view_state).await"""),
])

tests = subprocess.run(["/usr/bin/grep", "-rl", "ui_history_panel(&", str(ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests")], capture_output=True, text=True).stdout.split()
for name in tests:
    path = pathlib.Path(name)
    t = path.read_text()
    import re
    n = len(re.findall(r"ui_history_panel\((&[a-z_]+), None, \"ctrl\"", t))
    t = re.sub(r"ui_history_panel\((&[a-z_]+), None, \"ctrl\"", r'ui_history_panel(\1, None, None, "ctrl"', t)
    path.write_text(t)
    print(path.parent.name, n)
print("ok")
