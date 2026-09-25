import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    """#[cfg(not(target_arch = "wasm32"))]
fn frame_pump(shell: &mut ShellState) {
    drive(shell.pump_sync_events());
    shell.request_presence_preview();
    if shell.chrome_present.maintenance.presence_requested {
        shell.advance_presence_preview_step();
    }
}
""",
    """#[cfg(not(target_arch = "wasm32"))]
fn frame_pump(shell: &mut ShellState) {
    drive(shell.pump_sync_events());
    shell.request_presence_preview();
    if shell.chrome_present.maintenance.presence_requested {
        shell.advance_presence_preview_step();
    }
    if shell.settle_pump_pending() {
        drive(shell.settle_pump_step());
    }
}

/// ⏱️ How one frame-pumped open went, frame by frame: the frames while its steps were out and the
/// frames that then rendered what it owed, each with its count and its longest single frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
struct OpenFrames {
    opening: usize,
    opening_longest: std::time::Duration,
    rendering: usize,
    rendering_longest: std::time::Duration,
    elapsed: std::time::Duration,
}

/// 🚪️ Pumps frames ([`frame_pump`]) until the shell's frame-pumped document open settles and the
/// settle lane has rendered what it owed, timing every frame. A settled failure or cancellation is left
/// for the caller to assert.
#[cfg(not(target_arch = "wasm32"))]
fn settle_document_opening(shell: &mut ShellState) -> OpenFrames {
    let started = std::time::Instant::now();
    let mut frames = OpenFrames::default();
    while shell.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        frame_pump(shell);
        frames.opening += 1;
        frames.opening_longest = frames.opening_longest.max(frame.elapsed());
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    while shell.settle_pump_pending() && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        frame_pump(shell);
        frames.rendering += 1;
        frames.rendering_longest = frames.rendering_longest.max(frame.elapsed());
    }
    frames.elapsed = started.elapsed();
    frames
}
""",
)

replace(
    """    let document_id = format!("native-guest-journey-{}", chrome_now_ms() as u64);
    let started = std::time::Instant::now();
    shell_command(&mut shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str())]);
    let session = shell.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    println!("native-guest-journey open latency={:?} session={session:?} error={:?}", started.elapsed(), shell.error);
""",
    """    let document_id = format!("native-guest-journey-{}", chrome_now_ms() as u64);
    let started = std::time::Instant::now();
    shell_command(&mut shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str())]);
    let command = started.elapsed();
    let frames = settle_document_opening(&mut shell);
    let session = shell.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    println!("native-guest-journey open command={command:?} frames={frames:?} session={session:?} error={:?}", shell.error);
    assert!(shell.document_opening.is_none(), "the open settled and cleared its band: {:?}", shell.document_opening.as_ref().map(|opening| &opening.phase));
""",
)

replace(
    """/// 🌱️ One native wgpu shell creates a hub-bound artifact through its own creation door against a""",
    """/// 🖼️ Opening a document never holds the shell across a guest turn: the relay only starts the open,
/// every frame while the guest instantiates and loads the document stays short, and the band's cancel
/// stops an open whose app instance is still being created — the instance is destroyed and the session
/// stays as it was. Before, the relay held the shell (and so the frame build) for the whole open: 17 s
/// in a debug build (ticket 26/09/23 slice WG8). The frames that then render the new session are
/// measured and reported; each spends the guest's own render turns.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_open_keeps_the_frame_loop_painting_and_is_cancellable() {
    let journey = native_guest_journey();
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let variant = live_env("SEMIO_PLUGIN");
    let plugins = drive(crate::program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    let mut shell = ShellState::new(plugins, variant);
    let open = |shell: &mut ShellState, document_id: &str| {
        let started = std::time::Instant::now();
        shell_command(shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id), ("schema", journey.schema.as_str())]);
        started.elapsed()
    };

    let command = open(&mut shell, "native-guest-open-cancelled");
    assert_eq!(shell.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Instantiating), "the relay only starts the open");
    shell.cancel_document_opening();
    let cancelled = settle_document_opening(&mut shell);
    println!("native-guest-open cancel command={command:?} frames={cancelled:?} phase={:?}", shell.document_opening.as_ref().map(|opening| &opening.phase));
    assert_eq!(shell.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Cancelled), "the cancel settled the open");
    assert!(shell.session.is_none() && shell.sync_channel.is_none(), "a cancelled open mounts nothing");
    shell.cancel_document_opening();
    assert!(shell.document_opening.is_none(), "closing the settled band clears it");

    let command = open(&mut shell, "native-guest-open-painted");
    let frames = settle_document_opening(&mut shell);
    println!("native-guest-open command={command:?} frames={frames:?}");
    assert!(command < std::time::Duration::from_millis(500), "the relay started the open without a guest turn: {command:?}");
    assert!(frames.opening > 1 && frames.opening_longest < std::time::Duration::from_millis(500), "every frame stayed short while the guest opened: {frames:?}");
    assert_eq!(shell.sync_channel.as_ref().map(|channel| channel.document_id.as_str()), Some("native-guest-open-painted"));
}

/// 🌱️ One native wgpu shell creates a hub-bound artifact through its own creation door against a""",
)

replace(
    """    shell_command(&mut b, "os.open-artifact", &open_args);
""",
    """    shell_command(&mut b, "os.open-artifact", &open_args);
    let b_open = settle_document_opening(&mut b);
    println!("g7w-live B open frames={b_open:?}");
""",
)
path.write_text(text)
print("ok")
