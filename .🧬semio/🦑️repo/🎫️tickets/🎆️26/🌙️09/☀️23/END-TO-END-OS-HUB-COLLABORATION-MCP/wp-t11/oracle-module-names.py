"""🏷️ Renames every stdio test-oracle module that is mounted as `any` although its taxonomy subset is not `✳️any` to the
subset's own name, and rewrites every `artifacts::<artifact>::standards::<standard>::subsets::any` path that means it.

Usage: oracle-module-names.py [--write]"""
import re, subprocess, sys
write = "--write" in sys.argv
root = "/Users/ueli/Documents/semio/"
RENAMES = {("avi", "v1_0", "📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl"): "hdrl", ("bcf", "v2_1", "💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup"): "markup",
           ("docx", "v_ecma_376", "📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"): "base", ("dxf", "v_r12", "🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header"): "header",
           ("gif", "v89a", "🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base"): "base", ("jpg", "v_jfif_1_01", "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"): "document",
           ("json", "v_rfc8259", "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base"): "base", ("las", "v1_0", "☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header"): "header",
           ("obj", "v3_0", "🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry"): "geometry", ("pdf", "v1_4", "📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base"): "base",
           ("pdf", "v1_7", "📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"): "base", ("svg", "v1_1", "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base"): "base",
           ("tiff", "v6_0", "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document"): "document", ("xml", "v1_0", "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base"): "base"}
crate_root = root + "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🦀️.rs"
text = open(crate_root, encoding="utf-8").read()
for (art, std, path), new in RENAMES.items():
    mount = re.search(rf'pub mod any \{{\n(\s*)#\[path = "\.\./🗿️artifacts/{re.escape(path)}/🔮️oracles/🦀️.rs"\]', text)
    assert mount, path
    text = text[:mount.start()] + f"pub mod {new} {{\n" + text[mount.start() + len("pub mod any {\n"):]
if write: open(crate_root, "w", encoding="utf-8").write(text)
files = subprocess.run(["/usr/bin/grep", "-rlE", r"subsets::any\b", "--include=🦀️.rs", root + "✏️s", root + "🧰️framework"], capture_output=True, text=True).stdout.split()
for f in files:
    t = open(f, encoding="utf-8").read(); o = t
    for (art, std, _), new in RENAMES.items():
        t = re.sub(rf"\b{art}::standards::{std}::subsets::any\b", f"{art}::standards::{std}::subsets::{new}", t)
    if t != o:
        print("rewrote", f.split("🗿️artifacts/")[-1][:100])
        if write: open(f, "w", encoding="utf-8").write(t)
