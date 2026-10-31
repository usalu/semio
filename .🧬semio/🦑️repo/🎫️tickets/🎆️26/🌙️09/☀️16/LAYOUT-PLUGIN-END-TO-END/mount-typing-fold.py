from pathlib import Path
import json
import os
import re

ROOT = Path(__file__).resolve().parents[7]
PLUGIN = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
DECL = PLUGIN / "📜️declaration"
WRITER = ROOT / "✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer"
FOLD = WRITER / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⌨️typing/🔗️fold"
PURE = ROOT / "🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit"
SDK = PLUGIN / "🛠️tool-machine/⌨️typing/✍️edit"
COMPOSER = ROOT / "🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🧬️native"
changed = []


def write(path, before, after):
    assert path.read_text() == before, path
    if before != after:
        path.write_text(after)
        changed.append(str(path.relative_to(ROOT)))


def replace(path, before, after):
    text = path.read_text()
    if after in text:
        return
    assert text.count(before) == 1, (path, before)
    write(path, text, text.replace(before, after, 1))


def dependency(manifest, name, package_root):
    text = manifest.read_text()
    if re.search(r"^" + re.escape(name) + r"\s*=", text, re.M):
        return
    relative = os.path.relpath(package_root / "📦️packages/🦀️rust", manifest.parent)
    line = name + " = { path = " + json.dumps(relative, ensure_ascii=False) + " }\n"
    assert "[dependencies]\n" in text, manifest
    write(manifest, text, text.replace("[dependencies]\n", "[dependencies]\n" + line, 1))


def member(workspace, package_root):
    text = workspace.read_text()
    relative = os.path.relpath(package_root / "📦️packages/🦀️rust", workspace.parent)
    quoted = json.dumps(relative, ensure_ascii=False)
    if quoted in text.split("exclude =", 1)[0]:
        return
    assert "members = [\n" in text
    write(workspace, text, text.replace("members = [\n", "members = [\n    " + quoted + ",\n", 1))


replace(DECL / "👷️job/🦀️.rs", "use semio_framework_job_publication::", '#[path="⌨️typing/🔗️fold/🦀️.rs"]\npub mod original_typing_fold;\npub use original_typing_fold::{OriginalTypingFoldPolicy,ArtifactOriginalTypingFoldFactory,ArtifactOriginalTypingFoldProducer};\nuse semio_framework_job_publication::')
app = DECL / "🧩️app/🎛️artifact-app-trait/🦀️.rs"
text = app.read_text()
start = text.index("    /// ⌨️ This app's typing algebra:")
end = text.index("    /// 🪧️", start)
new = "    /// ⌨️ The original typing algebra keeps independent deliveries separate unless the app declares a bounded fold.\n    fn original_typing_fold() -> crate::tool_contract::OriginalTypingFoldPolicy<Self::Mutation> {\n        crate::tool_contract::OriginalTypingFoldPolicy::Split\n    }\n"
write(app, text, text[:start] + new + text[end:])
editor = DECL / "🪟️surface/✏️artifact-editor-trait/🦀️.rs"
text = editor.read_text()
start = text.index("    /// ⌨️ This editor's typing algebra")
end = text.index("    /// 🔰️", start)
new = "    /// ⌨️ The editor's original typing algebra is forwarded by `EditorApp<Self>`; independent deliveries split by default.\n    fn original_typing_fold() -> semio_framework_plugin_decl_app::tool_contract::OriginalTypingFoldPolicy<Self::Mutation> {\n        semio_framework_plugin_decl_app::tool_contract::OriginalTypingFoldPolicy::Split\n    }\n"
write(editor, text, text[:start] + new + text[end:])
replace(DECL / "🪟️surface/🖊️editor-app/🦀️.rs", "    fn typing_fold(net: &[Self::Mutation], next: &[Self::Mutation]) -> semio_framework_tool_machine::TypingFold<Self::Mutation> {\n        E::typing_fold(net, next)\n    }", "    fn original_typing_fold() -> semio_framework_plugin_decl_app::tool_contract::OriginalTypingFoldPolicy<Self::Mutation> {\n        E::original_typing_fold()\n    }")
writer = WRITER / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎛️writer-play-app-editor-and-constructor/🦀️.rs"
text = writer.read_text()
start = text.index("    /// ⌨️ Writer's typing algebra")
end = text.index("    fn genesis_child_pack", start)
new = "    /// ⌨️ Writer composes relative prose edits through its retained original typing producer.\n    fn original_typing_fold() -> semio_framework_plugin::tool_contract::OriginalTypingFoldPolicy<Self::Mutation> {\n        semio_s_artifact_writer_writer_typing_fold::original_typing_fold()\n    }\n\n"
write(writer, text, text[:start] + new + text[end:])
path = WRITER / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️text-splice/🦀️.rs"
replace(path, "WriterPlayApp::typing_fold", "WriterPlayApp::original_typing_fold")
manifest = FOLD / "📦️packages/🦀️rust/Cargo.toml"
text = manifest.read_text()
updated = re.sub(r'"(?:[^"\\]|\\.)*"', lambda m: json.dumps(json.loads(m.group()), ensure_ascii=False) if "\\u" in m.group() else m.group(), text)
write(manifest, text, updated)
dependency(DECL / "🧩️app/📦️packages/🦀️rust/Cargo.toml", "semio-framework-tool-typing-edit", PURE)
dependency(WRITER / "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml", "semio-s-artifact-writer-writer-typing-fold", FOLD)
member(ROOT / "🧰️framework/Cargo.toml", PURE)
member(ROOT / "Cargo.toml", SDK)
member(WRITER / "Cargo.toml", FOLD)
owners = ROOT / "📋️build-owners.json"
text = owners.read_text()
data = json.loads(text)
for name, package in [("semio-framework-tool-typing-edit", PURE), ("semio-framework-plugin-typing-edit", SDK), ("semio-s-artifact-writer-writer-typing-fold", FOLD)]:
    matches = [owner for owner in data["owners"] if owner["name"] == name]
    root = str((package / "📦️packages/🦀️rust").relative_to(ROOT))
    row = {"name": name, "root": root, "manifest": root + "/Cargo.toml", "packages": []}
    assert not matches or matches == [row], name
    if not matches:
        data["owners"].append(row)
write(owners, text, json.dumps(data, ensure_ascii=False, indent=2) + "\n")
packet = Path(__file__).with_name("typing-fold-working-files.json")
text = packet.read_text()
data = json.loads(text)
data["files"] = sorted(set(data["files"] + changed))
data["reportsInputs"] = sorted(set(data["reportsInputs"] + [str(Path(__file__).relative_to(ROOT))]))
write(packet, text, json.dumps(data, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"changed": changed}, ensure_ascii=False))
