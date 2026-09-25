import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = path.read_text()
pairs = [
    ("""fn frame_pump(shell: &mut ShellState) {
    drive(shell.pump_sync_events());
""", """fn frame_pump(shell: &mut ShellState) {
    let debug_started = std::time::Instant::now();
    drive(shell.pump_sync_events());
    if debug_started.elapsed() > std::time::Duration::from_millis(300) {
        eprintln!("[DEBUG] wg8 frame pump slow {:?} remote={}", debug_started.elapsed(), remote_of(shell));
    }
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
