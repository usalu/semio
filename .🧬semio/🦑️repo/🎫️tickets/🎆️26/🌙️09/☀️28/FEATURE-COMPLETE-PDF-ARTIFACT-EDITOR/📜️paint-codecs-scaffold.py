"""🏗️ Creates the paint-codecs crate, registers it in the PDF workspace and adds it to the page editor and every lockfile that locks the page editor."""

import bisect
import os
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio"
PDF = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
BASE = f"{PDF}/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"
CRATE = f"{BASE}/🔨️modules/🎞️paint-codecs/📦️packages/🦀️rust"
NAME = "semio-s-artifact-stdio-pdf-paint-codecs"
EDITOR = "semio-s-artifact-stdio-pdf-page-editor"
MEMBER = "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🎞️paint-codecs/📦️packages/🦀️rust"

MANIFEST = f'''[package]
workspace = "../../../../../../../.."
name = "{NAME}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "In-house PDF image and paint codecs: CCITT, JBIG2, JPEG 2000, mesh shadings and PostScript calculator functions"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"
'''

LIB = '''//! 🎞️ In-house PDF image and paint codecs. Every decoder reports progress and stops when `Control` returns false.

#[path = "../../../🖼️images/🧮️arithmetic/🦀️.rs"]
mod arithmetic;

#[path = "../../../🗜️filters/🧾️ccitt/🦀️.rs"]
pub mod ccitt;

#[path = "../../../🗜️filters/🧩️jbig2/🦀️.rs"]
pub mod jbig2;

#[path = "../../../🖼️images/🌅️jpx/🦀️.rs"]
pub mod jpx;

#[path = "../../../🌅️mesh-shading/🦀️.rs"]
pub mod mesh;

#[path = "../../../🗜️filters/🧮️postscript/🦀️.rs"]
pub mod postscript;

/// 🎛️ Progress and cancellation hook: receives completion in `0.0..=1.0` and returns `false` to cancel.
pub type Control<'a> = &'a mut dyn FnMut(f32) -> bool;

/// 🚫️ Why a codec stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RasterError {
    Malformed(&'static str),
    Unsupported(&'static str),
    TooLarge,
    Cancelled,
}
'''


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def workspace_member():
    path = f"{PDF}/Cargo.toml"
    text = open(path, encoding="utf-8").read()
    line = f'    "{MEMBER}",\n'
    if line in text:
        return
    anchor = '    "🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/📦️packages/🦀️rust",\n'
    assert anchor in text
    open(path, "w", encoding="utf-8").write(text.replace(anchor, anchor + line, 1))


def editor_dependency():
    path = f"{BASE}/✏️editor/🖼️page/📦️packages/🦀️rust/Cargo.toml"
    text = open(path, encoding="utf-8").read()
    dependency = f'{NAME} = {{ path = "../../../../🔨️modules/🎞️paint-codecs/📦️packages/🦀️rust" }}\n'
    if NAME in text:
        return
    anchor = "semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests" if False else "semio-s-artifact-stdio-pdf-page-coordinates ="
    at = text.index(anchor)
    end = text.index("\n", at) + 1
    open(path, "w", encoding="utf-8").write(text[:end] + dependency + text[end:])


def lock(path, version):
    text = open(path, encoding="utf-8").read()
    if f'name = "{NAME}"' in text or f'name = "{EDITOR}"\n' not in text:
        return False
    headers = [(m.start(), m.group(1)) for m in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"', text)]
    start = text.index(f'name = "{EDITOR}"\n')
    deps_at = text.index("dependencies = [\n", start)
    deps_end = text.index("]\n", deps_at)
    deps = text[deps_at + len("dependencies = [\n"):deps_end].splitlines()
    entry = f' "{NAME}",'
    deps.insert(bisect.bisect_left(deps, entry), entry)
    text = text[:deps_at] + "dependencies = [\n" + "\n".join(deps) + "\n" + text[deps_end:]
    headers = [(m.start(), m.group(1)) for m in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"', text)]
    position = next((index for index, name in headers if name > NAME), None)
    block = f'[[package]]\nname = "{NAME}"\nversion = "{version}"\n\n'
    if position is None:
        text = text.rstrip("\n") + "\n\n" + block.rstrip("\n") + "\n"
    else:
        text = text[:position] + block + text[position:]
    open(path, "w", encoding="utf-8").write(text)
    return True


def main():
    write(f"{CRATE}/Cargo.toml", MANIFEST)
    write(f"{CRATE}/🦀️.rs", LIB)
    workspace_member()
    editor_dependency()
    tracked = subprocess.run(["git", "-C", ROOT, "ls-files", "-z", "--", "*Cargo.lock"], capture_output=True).stdout.decode().split("\0")
    version = re.search(r'version = "([^"]+)"', open(f"{PDF}/Cargo.toml", encoding="utf-8").read().split("[workspace.package]")[1]).group(1)
    changed = [path for path in tracked if path and lock(f"{ROOT}/{path}", version)]
    for path in changed:
        print("[DEBUG] locked", path[-70:])


main()
