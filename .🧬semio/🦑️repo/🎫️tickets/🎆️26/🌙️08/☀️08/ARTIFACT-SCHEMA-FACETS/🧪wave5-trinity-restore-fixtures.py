from pathlib import Path
import subprocess

root = Path("✏️s") / "🔌️plugins" / "🔱️trinity"

# rewrite example from history
src = subprocess.check_output(
    [
        "git",
        "show",
        "503afb28b4:✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.trinity.rewrite.dsl.semio",
    ],
    text=True,
)
lines = src.splitlines()
if lines and lines[0].startswith("semio "):
    lines[0] = "semio trinity.rewrite.dsl v1"
out = root / "🗿️artifacts" / "♻️rewrite" / "📚️examples" / "🎬️demo" / "🖼️assets" / "🗣️example.dsl.semio"
out.write_text("\n".join(lines) + "\n")
print("wrote rewrite", out.stat().st_size)

old = 'include_str!("📚️examples/🎬️demo-session/🖼️assets/🎮️demo.cmd.semio")'
new = 'include_str!("../../🗿️artifacts/🔌️jack/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")'
for app in ["🔌️jack", "♻️rewrite"]:
    p = root / "🎛️apps" / app / "🦀️component.rs"
    text = p.read_text()
    if old not in text:
        print("missing app include", app)
        continue
    p.write_text(text.replace(old, new))
    print("fixed app", app)

p = root / "🎛️apps" / "♻️rewrite" / "🌍️world" / "🦀️component.rs"
text = p.read_text()
old_w = 'include_str!("../📚️examples/🎬️demo-session/🖼️assets/🎮️demo.cmd.semio")'
new_w = 'include_str!("../../../🗿️artifacts/🔌️jack/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")'
if old_w in text:
    p.write_text(text.replace(old_w, new_w))
    print("fixed world")
else:
    print("world missing")
