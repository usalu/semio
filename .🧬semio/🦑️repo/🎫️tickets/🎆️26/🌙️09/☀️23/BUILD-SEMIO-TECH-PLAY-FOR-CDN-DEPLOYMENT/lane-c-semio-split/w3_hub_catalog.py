#!/usr/bin/env python3
"""📇️ Wave 3: semio-hub-stdio's artifact catalog becomes the feature-free crate `semio-hub-stdio-catalog`, so gis, vcs and
the native hub link the catalog without the stdio component crate."""
import sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *

HUB = REPO + "/🌎️hub"
STDIO = HUB + "/🧩️compositions/🗄️stdio"
CAT = STDIO + "/🔌️plugin/📇️catalog"
pending = {}
def get(p): return pending.get(p) or read(p)
def put(p, s): pending[p] = s

src = get(CAT + "/🦀️.rs")
src = re.sub(r'(?m)^#\[cfg\(feature = "full-artifact-catalog"\)\]\n', "", src)
src = src.replace('#[cfg(all(test, feature = "full-artifact-catalog"))]', "#[cfg(test)]")
src = src.replace("pub(crate) fn component_package_id(", "pub fn component_package_id(").replace("pub(crate) fn selected_registry(", "pub fn selected_registry(")
src = src.replace("//! 📇 Component-level composition and validation for selected artifact packages.\n", "//! 📇 Component-level composition and validation for selected artifact packages: the feature-free full stdio catalog\n//! that the stdio component, the native hub and every composition validating stdio contributions link.\n\n#![allow(async_fn_in_trait)]\n#![allow(long_running_const_eval)]\n\nextern crate semio_framework_value_derive as value_derive;\n\n#[path = \"../../🛂️manifest/🦀️.rs\"]\npub mod manifest;\n", 1)
assert "#![allow(async_fn_in_trait)]" in src and "full-artifact-catalog" not in src
put(CAT + "/🦀️.rs", src)
put(STDIO + "/🛂️manifest/🦀️.rs", get(STDIO + "/🛂️manifest/🦀️.rs").replace("crate::catalog::", "crate::"))

root = get(STDIO + "/🦀️.rs")
root = root.replace('\n#[path = "🔌️plugin/📇️catalog/🦀️.rs"]\npub mod catalog;\n\n#[cfg(feature = "full-artifact-catalog")]\n#[path = "🛂️manifest/🦀️.rs"]\npub mod manifest;\n', "\n")
assert "pub mod catalog" not in root
put(STDIO + "/🦀️.rs", root)

main = STDIO + "/📦️packages/🦀️rust/Cargo.toml"
m = get(main)
deps = re.search(r"\[dependencies\]\n(.*?)# 🧩️ Composition Dependencies", m, re.S).group(1)
m = re.sub(r"# 🧩️ Composition Features\n.*?# /🧩️ Composition Features\n", "component-app-assembly = []\n", m, flags=re.S)
put(main, m)
cat_lines = ["[package]", 'workspace = "../../../../../.."', 'name = "semio-hub-stdio-catalog"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true", 'description = "Feature-free full stdio artifact catalog: contribution registry, capability ledger, native codec receipts and the catalog topic"', "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]", 'path = "../../🦀️.rs"', "", "[dependencies]"]
for line in deps.strip().split("\n"):
    if line.startswith("semio-framework-dispatch-macros") or line.startswith("semio-s-plugin-stdio"):
        continue
    line = re.sub(r'path = "\.\./\.\./\.\./\.\./\.\./', 'path = "../../../../../../../', line)
    cat_lines.append(line)
cat_lines += ["# 🧩️ Composition Dependencies", "# /🧩️ Composition Dependencies", 'serde = { workspace = true }', 'serde_json = { workspace = true }']
put(CAT + "/📦️packages/🦀️rust/Cargo.toml", "\n".join(dict.fromkeys(cat_lines)) + "\n")

ws = HUB + "/Cargo.toml"
w = get(ws)
w = w.replace('    "🧩️compositions/🗄️stdio/📦️packages/🦀️rust",\n', '    "🧩️compositions/🗄️stdio/📦️packages/🦀️rust",\n    "🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📦️packages/🦀️rust",\n', 1)
w = w.replace('semio-hub-stdio = { path = "../🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust", default-features = false }\n', 'semio-hub-stdio = { path = "../🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust", default-features = false }\nsemio-hub-stdio-catalog = { path = "../🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📦️packages/🦀️rust" }\n', 1)
assert "semio-hub-stdio-catalog" in w
put(ws, w)
for rel in ("🧩️compositions/🌍️gis/📦️packages/🦀️rust/Cargo.toml", "🧩️compositions/🌿️vcs/📦️packages/🦀️rust/Cargo.toml"):
    p = HUB + "/" + rel
    put(p, get(p).replace('semio-hub-stdio = { workspace = true, default-features = false, features = ["full-artifact-catalog"] }', "semio-hub-stdio-catalog = { workspace = true }"))
    assert "semio-hub-stdio-catalog" in pending[p]
hm = HUB + "/📦️packages/🦀️rust/Cargo.toml"
h = get(hm).replace('"dep:semio-hub-stdio", "semio-hub-stdio/full-artifact-catalog", ', '"dep:semio-hub-stdio-catalog", ').replace("semio-hub-stdio = { workspace = true, optional = true }", "semio-hub-stdio-catalog = { workspace = true, optional = true }")
assert "dep:semio-hub-stdio-catalog" in h and "semio-hub-stdio = " not in h, "hub manifest"
put(hm, h)
for p in subprocess.run(["rg", "-l", r"semio_hub_stdio::(catalog|manifest)\b|crate::(catalog|manifest)::", "-g", "*.rs", HUB], capture_output=True, text=True).stdout.split():
    s = get(p)
    s = re.sub(r"\bsemio_hub_stdio::catalog::", "semio_hub_stdio_catalog::", s)
    s = re.sub(r"\bsemio_hub_stdio::manifest::", "semio_hub_stdio_catalog::manifest::", s)
    if p.startswith(STDIO + "/🧪️tests/") or p == STDIO + "/🔌️plugin/🦀️.rs":
        s = re.sub(r"\bcrate::catalog::", "semio_hub_stdio_catalog::", s)
        s = re.sub(r"\bcrate::manifest::", "semio_hub_stdio_catalog::manifest::", s)
    put(p, s)
for p, s in pending.items():
    write(p, s)
print("wrote", len(pending))
