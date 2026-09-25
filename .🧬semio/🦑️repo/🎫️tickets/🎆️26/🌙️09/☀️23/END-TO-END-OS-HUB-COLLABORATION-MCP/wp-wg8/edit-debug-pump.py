import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()
pairs = [
    ("""        let directory_changed = self.pump_directory_events().await || shell_io_changed;
        // 📌️ ticket §C5 item 2 — the auto check-in poll, folded into the same every-frame pump for the
        // identical reason (no timer wheel exists in this shell; see `auto_checkin_should_fire`'s doc).
        self.poll_auto_checkin().await;
""", """        let debug_started = std::time::Instant::now();
        let directory_changed = self.pump_directory_events().await || shell_io_changed;
        let debug_directory = debug_started.elapsed();
        // 📌️ ticket §C5 item 2 — the auto check-in poll, folded into the same every-frame pump for the
        // identical reason (no timer wheel exists in this shell; see `auto_checkin_should_fire`'s doc).
        self.poll_auto_checkin().await;
        if debug_started.elapsed() > std::time::Duration::from_millis(300) {
            eprintln!("[DEBUG] wg8 pump slow directory={debug_directory:?} checkin={:?}", debug_started.elapsed() - debug_directory);
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
