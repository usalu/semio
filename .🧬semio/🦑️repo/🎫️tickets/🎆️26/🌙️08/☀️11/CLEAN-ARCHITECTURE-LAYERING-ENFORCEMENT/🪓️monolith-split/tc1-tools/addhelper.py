"""Appends the cfg(test) `runtime_fixture` module to an oracle artifact root. usage: addhelper.py <root 🦀️.rs> <bytes|text|both>"""
import os, shutil, sys, hashlib
path, which = sys.argv[1], sys.argv[2]
ORIG = os.path.join(os.path.dirname(os.path.abspath(__file__)), "orig")
os.makedirs(ORIG, exist_ok=True)
key = hashlib.sha256(path.encode()).hexdigest()[:16]
if not os.path.exists(os.path.join(ORIG, key + ".rs")):
    shutil.copy2(path, os.path.join(ORIG, key + ".rs"))
    open(os.path.join(ORIG, "index.tsv"), "a").write(f"{key}\t{path}\n")
text = open(path, encoding="utf-8").read()
assert "runtime_fixture" not in text
fns = []
if which in ("bytes", "both"):
    fns.append('''    pub(crate) fn bytes(relative: &str) -> Vec<u8> {
        let path = path(relative);
        std::fs::read(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }
''')
if which in ("text", "both"):
    fns.append('''    pub(crate) fn text(relative: &str) -> String {
        let path = path(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display()))
    }
''')
block = '''
/// 📥️ Run-time readers of the committed test fixtures, with paths relative to the artifact folder: the oracle tests read them here instead of embedding them with `include_bytes!` or `include_str!`, so the test closure stays inside the source gate.
/// @see <https://doc.rust-lang.org/std/macro.include_bytes.html>
#[cfg(test)]
pub(crate) mod runtime_fixture {
    fn path(relative: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(relative)
    }

''' + "\n".join(fns) + "}\n"
if not text.endswith("\n"): text += "\n"
open(path, "w", encoding="utf-8").write(text + block)
print("helper added", path[-50:])
