import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs")
text = path.read_text()
pairs = [(
"""    while shell.settle_pump_pending() && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let _ = frame_pump(shell);
        frames.rendering += 1;
        frames.rendering_longest = frames.rendering_longest.max(frame.elapsed());
    }
""",
"""    let mut wg8_probe = std::time::Instant::now();
    while shell.settle_pump_pending() && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let _ = frame_pump(shell);
        frames.rendering += 1;
        frames.rendering_longest = frames.rendering_longest.max(frame.elapsed());
        if wg8_probe.elapsed() > std::time::Duration::from_secs(20) {
            wg8_probe = std::time::Instant::now();
            println!("[DEBUG] wg8 settle pending close={} owed={} scope_empty={} rendering={} topo={} journal={} deferred={} uri={} settling={} computing={} steps={}", crate::interpreter::ui_document_close_pending(), shell.settle_pump.owed, shell.owed_refresh_scope.asks_for_nothing(), shell.settle_pump.rendering.is_some(), shell.window_topology_refresh_owed, shell.window_topology_journal_dispatch_owed, shell.deferred_actions.len(), shell.pending_shell_uri_apply, shell.settling, shell.live_compute_surfaces().count(), shell.settle_pump.steps);
        }
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
