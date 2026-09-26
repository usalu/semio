#!/usr/bin/env python3
"""WG8 s12: split the cross-shell law into one shared meeting (steps 1-4), the edits law and the board-cursor law."""
import pathlib

PATH = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = PATH.read_text()
start = text.index("/// 🤝️ One native wgpu user and one React `s` user on ONE hub document")
end = text.index("/// 📒️ Pumps one shell's frames ([`frame_pump`]) and re-reads its edit ledger")
assert text.count("/// 🤝️ One native wgpu user and one React `s` user on ONE hub document") == 1
old = text[start:end]
assert "fn a_native_and_a_react_user_collaborate_on_one_hub_document()" in old and old.count("#[test]") == 1

NEW = r'''/// 🧩️ The canvas kind the cross-shell cursor law opens: puzzle2d's editor window `2d-overview` is a
/// `SurfaceKind::Board2d` pane on both shells, so each user has a board to point at and to paint the other's
/// cursor over (block2d's only window is a summary surface with no board).
#[cfg(not(target_arch = "wasm32"))]
const CROSS_SHELL_BOARD_SCHEMA: &str = "puzzle.2d.fixture";

/// 🤝️ Steps 1–4 of the cross-shell journey (audit s12 P1-4, ticket 26/09/23 slice WG8), shared by both
/// cross-shell laws: the native wgpu user A signs in, creates a space, seats the React user B as an author,
/// creates an artifact of `schema` through its door and opens it (the hub-resolved component), paints its
/// windows; B opens the same artifact from its Space index in a real browser on a real `s` serve (driven by
/// `wp-wg8/cross-shell.mjs` through [`CrossShellHandshake`]); then each sees the other in its roster.
#[cfg(not(target_arch = "wasm32"))]
struct CrossShellMeeting {
    a: ShellState,
    handshake: CrossShellHandshake,
    ledger: CollaborationLedger,
    a_actor: String,
    b_actor: String,
    boards: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CrossShellMeeting {
    fn meet(schema: &str) -> Self {
        let origin = live_env("SEMIO_HUB_LIVE_ORIGIN");
        let (a_email, a_password) = (live_env("SEMIO_HUB_LIVE_EMAIL"), live_env("SEMIO_HUB_LIVE_PASSWORD"));
        let b_email = live_env("SEMIO_HUB_LIVE_PEER_EMAIL");
        let handshake = CrossShellHandshake { dir: std::path::PathBuf::from(live_env("SEMIO_CROSS_SHELL_DIR")) };
        let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
        let variant = live_env("SEMIO_PLUGIN");
        let mut ledger = CollaborationLedger::default();
        let plugins = drive(crate::program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
        let mut a = ShellState::new(plugins, variant);

        let retry = sign_in_live(&mut a, &origin, &a_email, &a_password);
        let a_user = a.hub_workspace.session.user_id.clone();
        ledger.record("1-a-signs-in", a.hub_workspace.session.phase == HubSessionPhase::SignedIn, format!("A={} {a_user:?} retried-after={retry:?}", a.hub_workspace.session.phase.as_str()));

        let space_name = format!("wg8 cross-shell {}", chrome_now_ms() as u64);
        hub_verb(&mut a, crate::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
        hub_verb(&mut a, crate::hub_connection::action::CREATE_SPACE, &[]);
        let space_id = a.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).unwrap_or_default();
        shell_command(&mut a, "os.directory.upsert-member", &[("spaceId", space_id.as_str()), ("email", b_email.as_str()), ("role", "author")]);
        hub_verb(&mut a, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
        let kind_id = a.hub_workspace.creation.catalog.as_ref().and_then(|catalog| catalog.kinds.iter().find(|kind| kind.schema == schema)).map(|kind| kind.kind_id.clone()).unwrap_or_default();
        hub_verb(&mut a, crate::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
        hub_verb(&mut a, crate::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Cross-shell board")]);
        hub_verb(&mut a, crate::hub_connection::action::CREATE_ARTIFACT, &[]);
        let creation_started = std::time::Instant::now();
        let mut creation_trail = Vec::new();
        while creation_started.elapsed() < std::time::Duration::from_secs(300) && !a.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening) {
            let observed = a.hub_workspace.creation.operation.as_ref().map(|operation| (crate::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
            if creation_trail.last() != Some(&observed) {
                creation_trail.push(observed);
            }
            let _ = frame_pump(&mut a);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        let a_open = settle_document_opening(&mut a);
        println!("cross-shell native creation catalog={} trail={creation_trail:?} after={:?} open={a_open:?} phase={:?} error={:?}", a.hub_workspace.creation.catalog_phase.as_str(), creation_started.elapsed(), a.document_opening.as_ref().map(|opening| &opening.phase), a.error);
        let document_id = a.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.as_ref()).map(|ready| ready.artifact_id.clone()).unwrap_or_default();
        let live_after = pump_until(&mut a, std::time::Duration::from_secs(30), is_live);
        ledger.record(
            "2-a-creates-and-opens",
            !space_id.is_empty() && !document_id.is_empty() && live_after.is_some(),
            format!("space={space_id} schema={schema} kind={kind_id:?} document={document_id} opening={:?} live-after={live_after:?} remote={}", a.hub_workspace.creation.operation.as_ref().map(|operation| operation.opening), remote_of(&a)),
        );
        let boards = paint_session_windows(&mut a, Rect::new(0.0, 40.0, 1200.0, 760.0));
        let a_actor = a.presence_self.as_ref().map(|(actor, _)| actor.clone()).unwrap_or_default();
        handshake.publish("open", serde_json::json!({ "spaceId": space_id, "documentId": document_id, "schema": schema, "actor": a_actor, "userId": a_user, "boards": boards, "live": live_after.is_some() }));

        let react_open = handshake.await_react(&mut a, "open", std::time::Duration::from_secs(600));
        ledger.record("3-b-opens-in-react", react_open.as_ref().is_some_and(|value| value["live"] == true), format!("{react_open:?}"));

        let is_b = |peer: &PresencePeer, a_actor: &str| peer.actor != a_actor && peer.user_id.is_some() && peer.user_id != a_user;
        let presence_after = pump_until(&mut a, std::time::Duration::from_secs(30), |shell| shell.presence_peers.iter().any(|peer| is_b(peer, &a_actor)));
        let b_peer = a.presence_peers.iter().find(|peer| is_b(peer, &a_actor)).cloned();
        let b_actor = b_peer.as_ref().map(|peer| peer.actor.clone()).unwrap_or_default();
        handshake.publish("presence", serde_json::json!({ "seesReact": b_peer.is_some(), "reactActor": b_actor, "peers": a.presence_peers.iter().map(|peer| serde_json::json!({ "actor": peer.actor, "userId": peer.user_id, "label": peer.label, "color": peer.color, "views": peer.views.len() })).collect::<Vec<_>>() }));
        let react_presence = handshake.await_react(&mut a, "presence", std::time::Duration::from_secs(120));
        ledger.record(
            "4-presence-both-ways",
            presence_after.is_some() && react_presence.as_ref().is_some_and(|value| value["seesNative"] == true),
            format!("A sees B after {presence_after:?} as {:?}; B sees A: {react_presence:?}", b_peer.as_ref().map(|peer| (&peer.actor, &peer.label, peer.color))),
        );
        Self { a, handshake, ledger, a_actor, b_actor, boards }
    }

    /// 🏁️ Tells the driver the journey ended, signs A out and asserts every recorded step.
    fn finish(mut self) {
        let failed: Vec<&str> = self.ledger.steps.iter().filter(|(_, passed, _)| !passed).map(|(step, _, _)| *step).collect();
        self.handshake.publish("done", serde_json::json!({ "failed": failed }));
        hub_verb(&mut self.a, crate::hub_connection::action::SIGN_OUT, &[]);
        assert!(failed.is_empty(), "cross-shell steps failed: {failed:?}");
    }
}

/// 🤝️ One native wgpu user and one React `s` user co-edit ONE hub document (the fixture's block2d kind):
/// after [`CrossShellMeeting::meet`], each user's edit reaches the other's ledger, each undoes only their own
/// edit, and B's same-document reload converges (A's later edit reaches the reloaded B). Every step is
/// recorded before the law asserts the whole ledger.
///
/// 🔌️ `#[ignore]`d: needs a live hub (`SEMIO_HUB_LIVE_ORIGIN`, principal A `SEMIO_HUB_LIVE_EMAIL`/`_PASSWORD`,
/// B's `SEMIO_HUB_LIVE_PEER_EMAIL`), a staged native runtime (`SEMIO_PLUGIN_MODULES`/`SEMIO_PLUGIN`) and the
/// browser driver (`CROSS_MODE=edits`) sharing `SEMIO_CROSS_SHELL_DIR`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, a React s serve and its browser driver; see this test's own doc comment"]
fn a_native_and_a_react_user_collaborate_on_one_hub_document() {
    let journey = native_guest_journey();
    let mut meeting = CrossShellMeeting::meet(journey.schema.as_str());
    let CrossShellMeeting { a, handshake, ledger, .. } = &mut meeting;

    let before = applied_edits(a).len();
    let (edit, latency) = author_edit(a, journey.verb.as_str());
    let a_ledger = applied_edits(a);
    handshake.publish("edit", serde_json::json!({ "verb": journey.verb, "ok": edit.is_ok(), "ledger": a_ledger.len() }));
    let react_ingested = handshake.await_react(a, "ingested", std::time::Duration::from_secs(120));
    ledger.record("5-a-edits-b-ingests", edit.is_ok() && a_ledger.len() == before + 1 && react_ingested.as_ref().is_some_and(|value| value["ingested"] == true), format!("A outcome={edit:?} latency={latency:?} ledger {before}->{}; B {react_ingested:?}", a_ledger.len()));

    let remote_before = applied_edits(a).iter().filter(|(action, _)| action == "apply").count();
    let react_edit = handshake.await_react(a, "edit", std::time::Duration::from_secs(120));
    let a_ingests = pump_until_ledger(a, std::time::Duration::from_secs(30), |ledger| ledger.iter().filter(|(action, _)| action == "apply").count() > remote_before);
    let a_after_b = applied_edits(a);
    handshake.publish("ingested", serde_json::json!({ "ingested": a_ingests.is_some(), "ledger": a_after_b.len() }));
    ledger.record("6-b-edits-a-ingests", react_edit.as_ref().is_some_and(|value| value["ok"] == true) && a_ingests.is_some(), format!("B {react_edit:?}; A ingested after {a_ingests:?} ledger {remote_before}->{a_after_b:?}"));

    let (undo, _) = author_edit(a, journey.undo.as_str());
    let a_undone = applied_edits(a);
    let own_reverted = a_undone.iter().filter(|(action, applied)| action == journey.verb.as_str() && !applied).count() == 1;
    handshake.publish("undo", serde_json::json!({ "ok": undo.is_ok(), "ownReverted": own_reverted, "ledger": a_undone.iter().map(|(action, applied)| format!("{action}:{applied}")).collect::<Vec<_>>() }));
    let react_undo = handshake.await_react(a, "undo", std::time::Duration::from_secs(120));
    let a_after_b_undo = pump_until_ledger(a, std::time::Duration::from_secs(20), |ledger| ledger.iter().any(|(action, applied)| action == "apply" && !applied));
    let a_final = applied_edits(a);
    ledger.record(
        "7-each-undoes-own",
        undo.is_ok() && own_reverted && react_undo.as_ref().is_some_and(|value| value["ok"] == true && value["ownReverted"] == true) && a_after_b_undo.is_some(),
        format!("A undo={undo:?} own-reverted={own_reverted}; B {react_undo:?}; A sees B's undo after {a_after_b_undo:?} ledger={a_final:?}"),
    );

    let react_reloaded = handshake.await_react(a, "reloaded", std::time::Duration::from_secs(600));
    let (after_reload, _) = author_edit(a, journey.verb.as_str());
    handshake.publish("after-reload-edit", serde_json::json!({ "ok": after_reload.is_ok(), "ledger": applied_edits(a).len() }));
    let react_converged = handshake.await_react(a, "converged", std::time::Duration::from_secs(120));
    ledger.record(
        "8-reload-converges",
        react_reloaded.as_ref().is_some_and(|value| value["live"] == true) && after_reload.is_ok() && react_converged.as_ref().is_some_and(|value| value["converged"] == true),
        format!("B reloaded {react_reloaded:?}; A edit {after_reload:?}; B {react_converged:?}"),
    );
    meeting.finish();
}

/// 🖱️ One native wgpu user and one React `s` user point at ONE hub board ([`CROSS_SHELL_BOARD_SCHEMA`]):
/// after [`CrossShellMeeting::meet`], A's pointer over its painted board travels as a canvas-presence window
/// view on the one presence wire and React paints A's cursor over the same window; B moves its mouse over
/// React's board canvas and A's verified roster carries B's view, which A's chrome turns into a peer cursor
/// over its own board (`board_peer_overlays`).
///
/// 🔌️ `#[ignore]`d: the same live inputs as [`a_native_and_a_react_user_collaborate_on_one_hub_document`], with
/// the browser driver in `CROSS_MODE=cursors`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, a React s serve and its browser driver; see this test's own doc comment"]
fn a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board() {
    let mut meeting = CrossShellMeeting::meet(CROSS_SHELL_BOARD_SCHEMA);
    let CrossShellMeeting { a, handshake, ledger, b_actor, boards, .. } = &mut meeting;

    let board = a.board2d_states.iter().next().map(|(_, surface)| (surface.window_id.clone(), surface.bounds));
    if let Some((_, bounds)) = board.as_ref() {
        a.presence_pointer = Some((bounds.x + bounds.w * 0.75, bounds.y + bounds.h * 0.25));
    }
    let (views, _) = a.board_presence_views();
    let _ = pump_until(a, std::time::Duration::from_secs(2), |_| false);
    handshake.publish("cursor", serde_json::json!({ "board": board.as_ref().map(|(window, _)| window), "views": views.iter().map(|view| serde_json::json!({ "windowId": view.window_id, "space": view.space, "pointer": view.pointer })).collect::<Vec<_>>() }));
    let react_cursor = handshake.await_react(a, "cursor", std::time::Duration::from_secs(120));
    let b_actor = b_actor.clone();
    let native_sees = pump_until(a, std::time::Duration::from_secs(20), |shell| shell.board_peer_overlays().iter().any(|(_, overlays)| overlays.cursors.iter().any(|cursor| cursor.actor == b_actor)));
    let a_overlays: Vec<String> = a.board_peer_overlays().iter().flat_map(|(_, overlays)| overlays.cursors.iter().map(|cursor| format!("{}@{:?} chip={}", cursor.actor, cursor.at, cursor.chip))).collect();
    let b_views: Vec<String> = a.presence_peers.iter().filter(|peer| peer.actor == b_actor).flat_map(|peer| peer.views.iter().map(|view| format!("{}:{}:{:?}", view.window_id, view.space, view.pointer))).collect();
    ledger.record(
        "5-cursors-both-ways",
        native_sees.is_some() && react_cursor.as_ref().is_some_and(|value| value["seesNativeCursor"] == true),
        format!("A boards={boards:?} views={views:?} B sees A's cursor: {react_cursor:?}; A sees B's cursor after {native_sees:?}: {a_overlays:?} (B's views {b_views:?})"),
    );
    handshake.publish("cursor-seen", serde_json::json!({ "seesReactCursor": native_sees.is_some(), "overlays": a_overlays }));
    meeting.finish();
}

'''
text = text[:start] + NEW + text[end:]
PATH.write_text(text)
print("split ok", len(old), "->", len(NEW))
