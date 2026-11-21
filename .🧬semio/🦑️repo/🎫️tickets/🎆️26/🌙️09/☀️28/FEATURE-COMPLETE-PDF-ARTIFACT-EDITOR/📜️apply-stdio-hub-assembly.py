"""🔌️ Applies the reviewed Stdio and PDF Hub assembly postimages."""

import difflib
import hashlib
import json
from pathlib import Path
import sys

ticket = Path(__file__).resolve().parent
hub = Path("🌎️hub")
base = hub / "🧩️compositions/🗄️stdio"
pdf = base / "🧩️extensions/📘️pdf"
pdf_manifest = pdf / "📦️packages/🦀️rust/Cargo.toml"
lock = hub / "Cargo.lock"

def change(text, old, new):
    assert text.count(old) == 1, old
    return text.replace(old, new, 1)

def composition(path, app):
    original = path.read_text()
    imports = "use semio_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};"
    result = change(original, imports, imports.replace("PluginAssemblyError", "PluginAssemblyError, PluginBuilder"))
    return original, change(result, f"Plugin::<{app}>::builder(", f"PluginBuilder::<_, {app}>::new(")

paths = {}
for path, app in [(base / "🔌️plugin/🦀️.rs", "StdioApps"), (pdf / "🦀️.rs", "StdioPdfApps")]:
    original, fixed = composition(path, app)
    paths[path] = (original, fixed)
manifest = pdf_manifest.read_text()
line = "semio-framework-job = { workspace = true }\n"
assert "semio-framework-value =" not in manifest
assert "semio-framework-value = { path =" in (hub / "Cargo.toml").read_text()
paths[pdf_manifest] = (manifest, change(manifest, line, line + "semio-framework-value = { workspace = true }\n"))
raw_lock = lock.read_text()
packages = raw_lock.split("[[package]]")
matching = [i for i, entry in enumerate(packages) if '\nname = "semio-hub-stdio-pdf"\n' in entry]
assert len(matching) == 1
i = matching[0]
assert '"semio-framework-value",' not in packages[i]
packages[i] = change(packages[i], ' "semio-framework-pack",\n', ' "semio-framework-pack",\n "semio-framework-value",\n')
assert any('\nname = "semio-framework-value"\n' in record for record in packages)
paths[lock] = (raw_lock, "[[package]]".join(packages))

def digest(text):
    return hashlib.sha256(text.encode()).hexdigest()

out = ticket / "🗑️generated/hub-assembly"
out.mkdir(exist_ok=True)
patch = "".join("".join(difflib.unified_diff(a.splitlines(keepends=True), b.splitlines(keepends=True), "a/" + str(path), "b/" + str(path))) for path, (a, b) in paths.items())
(out / "reviewed-hub-assembly.patch").write_text(patch)
receipt = {str(path): {"preimage": digest(a), "postimage": digest(b)} for path, (a, b) in paths.items()}
(out / "reviewed-hub-assembly.json").write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n")
print(patch)
if "--apply-reviewed" in sys.argv:
    for path, (a, _) in paths.items():
        assert path.read_bytes().decode() == a, path
    for path, (a, b) in paths.items():
        assert path.read_bytes().decode() == a, path
        path.write_bytes(b.encode())
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest(b), path
    print(f"[DEBUG] Applied and re-read {len(paths)} reviewed Hub postimages")
