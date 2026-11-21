"""🔬 Type-check the staged production owners with governed WASI prerequisite metadata."""

from pathlib import Path
from runpy import run_path
import re
from shutil import copyfile
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
STAGE = run_path(str(TICKET / "raw-png-placement-stage.py"))
ROOT = STAGE["ROOT"]
PIX = STAGE["DECODE"]["PIXELS"]
EDITOR = STAGE["AUTHOR"].parent
CACHE = Path(sys.argv[1]).resolve()
OUT = TICKET / "🗑️generated" / "raw-png" / ("type-" + CACHE.name)
WASI = CACHE / "wasm32-wasip2/prototype/build"
HOST = CACHE / "prototype/build"
OUT.mkdir(parents=True, exist_ok=True)
for original in [WASI, HOST]:
    if not original.is_dir():
        raise ValueError(f"expected governed dependency family {original}")
old = dict(STAGE["production_changes"]())
metadata = OUT / "deps"
metadata.mkdir(exist_ok=True)
for owner in [WASI, HOST]:
    for path in owner.glob("*/*/out/*"):
        if path.suffix not in (".rmeta", ".so", ".dll", ".dylib"):
            continue
        link = metadata / path.name
        if not link.exists():
            try:
                link.symlink_to(path.resolve())
            except OSError:
                copyfile(path, link)
all_artifacts = [metadata]
built = {}


def materialize(source, target, replacements=None):
    """🪟 Redirect only changed modules and keep exact unchanged code from its production owner."""
    replacements = {} if replacements is None else replacements
    original = old.get(source)
    if original is None:
        original = source.read_text(encoding="utf8")

    def route(match):
        resolved = (source.parent / match[2]).resolve()
        path = replacements.get(resolved, resolved)
        assert path.is_file(), path
        return f'{match[1]}"{path.as_posix()}"'

    target.write_text(re.sub(r'(\#\[path\s*=\s*)"([^"]+)"', route, original), encoding="utf8")
    return target


def check(name, owner, source, dependencies):
    """🛠️ Compile metadata only, reusing the exact linked prerequisite family."""
    output = OUT / f"lib{name}.rmeta"
    args = ["--edition=2021", "--crate-type=rlib", "--emit=metadata", "--target=wasm32-wasip2",
            "--crate-name", name, source, "-o", output, "-Awarnings"]
    for dependency in dependencies:
        artifact = built.get(dependency)
        if artifact is None:
            [artifact] = WASI.glob(f"*/*/out/lib{dependency}-*.rmeta")
        args.extend(("--extern", f"{dependency}={artifact}"))
    args.extend(f"-Ldependency={path}" for path in [OUT, *all_artifacts])
    rsp = OUT / f"{owner}.rsp"
    rsp.write_text("\n".join(map(str, args)) + "\n", encoding="utf8")
    subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", f"@{rsp}"], check=True)
    built[name] = output


decode = STAGE["DECODE"]["LIVE"]
component = PIX / "🦀️.rs"
decoded = materialize(decode, OUT / "decode.rs")
component_file = materialize(component, OUT / "component.rs", {decode: decoded})
full = PIX / "🎞️codecs/📦️packages/🦀️rust/🦀️.rs"
full_file = materialize(full, OUT / "codecs.rs", {component: component_file})
check("semio_framework_pixels_codecs", "pixels-codecs", full_file, [
    "semio_framework_value", "semio_framework_value_persistent", "semio_framework_io_binary_source",
    "semio_framework_io_base64", "semio_framework_deflate", "semio_framework_2d_retire",
])
facade = PIX / "📦️packages/🦀️rust/🦀️.rs"
facade_file = materialize(facade, OUT / "pixels.rs")
check("semio_framework_pixels", "pixels", facade_file, ["semio_framework_pixels_codecs", "semio_framework_value",
    "semio_framework_value_persistent", "semio_framework_2d"])

author = STAGE["AUTHOR"] / "🦀️.rs"
replacements = {}
for source, text in old.items():
    if source.parent != author.parent / "📥️raw":
        continue
    stage = OUT / source.name
    stage.write_text(text, encoding="utf8")
    replacements[source] = stage
authored = materialize(author, OUT / "author.rs", replacements)
document = EDITOR / "🦀️.rs"
doc_file = materialize(document, OUT / "document.rs", {author: authored})
package = EDITOR / "📦️packages/🦀️rust/🦀️.rs"
entry = materialize(package, OUT / "pdf.rs", {document: doc_file})
extras = list(STAGE["DEP"])
extras = ["pack" if n == "semio-framework-pack" else n.replace("-", "_") for n in extras]
check("semio_s_artifact_stdio_pdf_editor_document", "editor-document", entry, [
    "semio_s_artifact_stdio_contract", "semio_framework_pixels", "semio_plugin",
    "semio_s_artifact_stdio_pdf_snapshot_2", "semio_s_artifact_stdio_pdf_mut_family_8", *extras,
])
print("[DEBUG] staged PDF PNG current decoder, full pixels facade and paid editor-document cohort passed WASI metadata type-check")
