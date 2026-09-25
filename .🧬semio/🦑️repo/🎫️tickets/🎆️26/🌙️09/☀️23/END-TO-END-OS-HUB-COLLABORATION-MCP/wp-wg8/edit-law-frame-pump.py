import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:90])
    text = text.replace(old, new)


replace(
    """/// 🔁️ Drives both shells' real frame pump (`pump_sync_events`: directory lane, auto check-in,
/// document actor events) until `done` holds or the budget ends, recording every distinct remote
/// state either shell passed through.
#[cfg(not(target_arch = "wasm32"))]
fn pump_pair(a: &mut ShellState, b: &mut ShellState, budget: std::time::Duration, trail: &mut Vec<String>, done: impl Fn(&ShellState, &ShellState) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    while started.elapsed() < budget {
        drive(a.pump_sync_events());
        drive(b.pump_sync_events());
""",
    """/// 🖼️ What one painted frame does for an open document, headless: the frame pump
/// (`pump_sync_events`: directory lane, auto check-in, document actor events) and the chrome walk's
/// presence phase, which arms the maintenance lane's heartbeat step (`ShellChromeFramePhase::Presence`
/// → `advance_presence_preview_step`). Without the second half a headless shell never beat, so the hub
/// saw no peer and dropped each idle document socket after its presence lease (two-user gate run 13).
#[cfg(not(target_arch = "wasm32"))]
fn frame_pump(shell: &mut ShellState) {
    drive(shell.pump_sync_events());
    shell.request_presence_preview();
    if shell.chrome_present.maintenance.presence_requested {
        shell.advance_presence_preview_step();
    }
}

/// 🔁️ Drives both shells' frames ([`frame_pump`]) until `done` holds or the budget ends, recording
/// every distinct remote state either shell passed through.
#[cfg(not(target_arch = "wasm32"))]
fn pump_pair(a: &mut ShellState, b: &mut ShellState, budget: std::time::Duration, trail: &mut Vec<String>, done: impl Fn(&ShellState, &ShellState) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    while started.elapsed() < budget {
        frame_pump(a);
        frame_pump(b);
""",
)
replace(
    "offline_edit.is_ok() && offline_latency < std::time::Duration::from_secs(2) && pump_elapsed",
    "offline_edit.is_ok() && offline_latency <= b_latency.saturating_mul(2) && pump_elapsed",
)
path.write_text(text)
print("ok")
