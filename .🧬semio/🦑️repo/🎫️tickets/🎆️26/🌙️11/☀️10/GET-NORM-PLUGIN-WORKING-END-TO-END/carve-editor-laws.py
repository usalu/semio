"""🧪️ Carves the window, mode, panel and command unit suites of a norm editor into a tests-only laws crate that runs them against the public editor surface; the shared fixture app moves to its own file mounted by both crates.
usage: carve-editor-laws.py <family-dir> <enNNNN> <Pascal>   e.g. 🔩️en1993 en1993 En1993"""
import os, re, sys
family, code, pascal = sys.argv[1:4]
fam = f"✏️s/🔌️plugins/📕️norm/🗿️artifacts/{family}"
editor = f"{fam}/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
crate = f"semio-s-artifact-norm-{code}-editor"
mounts = {
    "🎭️modes/✏️edit/🪟️windows/🧊️viewport": "viewport_window_tests",
    "🎭️modes/✏️edit/🪟️windows/✅️checks": "checks_window_tests",
    "🎭️modes/✏️edit/🪟️windows/🧾️properties": "properties_window_tests",
    "🎭️modes/✏️edit/🪟️windows/📈️diagrams": "diagrams_window_tests",
    "🎭️modes/✏️edit/🪟️windows/🌲️outliner": "outliner_window_tests",
    "🎭️modes/✏️edit": "edit_mode_tests",
    "📌️panels/📚️catalogue": "catalogue_panel_tests",
    "📌️panels/🔍️inspection": "inspection_panel_tests",
    "📌️panels/🗿️artifact": "document_panel_tests",
    "🎮️commands/☑️selected-check": "selected_check_command_tests",
}
mount = re.compile(r'//#region 🧪️Tests\n#\[cfg\(test\)\]\n#\[path = "🧪️tests/🔬️unit/🦀️\.rs"\]\nmod tests;\n//#endregion 🧪️Tests\n\n?')
for module in mounts:
    path = f"{editor}/{module}/🦀️.rs"
    text = open(path, encoding="utf8").read()
    new, n = mount.subn("", text)
    assert n == 1, path
    open(path, "w", encoding="utf8").write(new.rstrip("\n") + "\n")

unit = f"{editor}/🧪️tests/🔬️unit/🦀️.rs"
body = open(unit, encoding="utf8").read()
start = body.index("pub(crate) mod context {\n")
end = body.index("\n}\n", start) + 3
block = body[start:end]
inner = block[len("pub(crate) mod context {\n"):-3]
lines = [line[4:] if line.startswith("    ") else line for line in inner.split("\n")]
context = "//! 🧰️ The fixture app of the editor unit suites: the real registry, one original caller scope, dispatch, settle, render and close.\n\nuse crate::modes::edit::windows::checks;\n" + "\n".join(lines).rstrip() + "\n"
os.makedirs(f"{editor}/🧪️tests/🧰️context", exist_ok=True)
open(f"{editor}/🧪️tests/🧰️context/🦀️.rs", "w", encoding="utf8").write(context)
body = body[:start] + '#[path = "../🧰️context/🦀️.rs"]\npub(crate) mod context;\n' + body[end:]
open(unit, "w", encoding="utf8").write(body)

laws = f"{editor}/🧪️tests/⚖️laws"
manifest_dir = f"{laws}/📦️packages/🦀️rust"
os.makedirs(manifest_dir, exist_ok=True)
root = [f"//! ⚖️ {code.upper().replace('EN', 'EN ')} editor laws: the window, mode, panel and command unit suites run against the public surface of the editor crate.", "", "#![allow(async_fn_in_trait)]", "#![cfg(test)]", "", f"pub use semio_s_artifact_norm_{code}_editor::*;", "", '#[path = "."]', "pub(crate) mod unit_tests {", '    #[path = "../🧰️context/🦀️.rs"]', "    pub(crate) mod context;", "}", ""]
for module, name in mounts.items():
    root += [f'#[path = "../../{module}/🧪️tests/🔬️unit/🦀️.rs"]', f"mod {name};", ""]
open(f"{laws}/🦀️.rs", "w", encoding="utf8").write("\n".join(root).rstrip("\n") + "\n")

src = open(f"{editor}/📦️packages/🦀️rust/Cargo.toml", encoding="utf8").read()
def section(name):
    m = re.search(rf"^\[{re.escape(name)}\]\n(.*?)(?=^\[|\Z)", src, re.M | re.S)
    return m.group(1) if m else ""
def rebase(line):
    m = re.search(r'path = "([^"]+)"', line)
    if not m:
        return line
    target = os.path.normpath(os.path.join(f"{editor}/📦️packages/🦀️rust", m.group(1)))
    return line.replace(m.group(1), os.path.relpath(target, manifest_dir))
deps = "".join(rebase(l) + "\n" for l in section("dependencies").splitlines() if l.strip())
dev = "".join(rebase(l) + "\n" for l in section("dev-dependencies").splitlines() if l.strip())
up = "/".join([".."] * len(os.path.relpath(fam, manifest_dir).split("/")) ) if False else os.path.relpath(fam, manifest_dir)
manifest = f'''[package]
workspace = "{up}"
name = "{crate}-laws"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "{code.upper().replace('EN', 'EN ')} editor laws: window, mode, panel and command unit suites against the public editor surface"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"

[dependencies]
{crate} = {{ workspace = true }}
{deps}
[dev-dependencies]
{dev}'''
open(f"{manifest_dir}/Cargo.toml", "w", encoding="utf8").write(manifest)
print("ok", crate + "-laws")
