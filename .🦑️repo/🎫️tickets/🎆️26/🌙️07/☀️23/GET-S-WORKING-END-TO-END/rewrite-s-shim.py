from pathlib import Path
import re

p = Path(
    "🧰️framework/🛍️product/💻️os/🔨️module/🧑‍💻dev/⚡️implementation/🟦️typescript/🔌️plugin-modules/s/semio_s_plugin_space_component.js"
)
# Resolve actual emoji path
root = Path("🧰️framework/🛍️product/💻️os/🔨️module")
dev = next(d for d in root.iterdir() if d.is_dir() and "dev" in d.name)
p = (
    dev
    / "⚡️implementation"
    / "🟦️typescript"
    / "🔌️plugin-modules"
    / "s"
    / "semio_s_plugin_space_component.js"
)
text = p.read_text()
text2 = re.sub(
    r"(from\s+['\"])@bytecodealliance/preview2-shim/([\w-]+)(['\"])",
    r"\1../_vendor/@bytecodealliance/preview2-shim/\2.js\3",
    text,
)
p.write_text(text2)
print("path", p)
print("rewrote", text != text2)
for line in text2.splitlines()[:8]:
    print(line)
