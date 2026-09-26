import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [(
"""            let (pages, bytes) = request.command_credits();
            let Ok(mut state) = self.state.try_lock() else {
                return Err((request, slot));
            };
""",
"""            let (pages, bytes) = request.command_credits();
            let Ok(mut state) = self.state.try_lock() else {
                eprintln!("[DEBUG] wg8 kernel push contended producer={}", producer.is_some());
                return Err((request, slot));
            };
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
