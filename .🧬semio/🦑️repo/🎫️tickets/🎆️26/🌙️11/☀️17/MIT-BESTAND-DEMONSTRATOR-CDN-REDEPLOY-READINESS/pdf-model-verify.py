"""🔬️ Typecheck the PDF model owners with one coherent compiler-produced dependency lane."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess


def verify(lane, target):
    """🧾️ Replay the split model and its public type surface on one pinned target."""
    ticket = Path(__file__).resolve().parent
    model = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📐️model")
    output = ticket / "🗑️generated" / ("pdf-wasi" if target else "pdf-native")
    dependencies = output / "d"
    dependencies.mkdir(parents=True, exist_ok=True)
    builds = lane / target / "release/build" if target else lane / "release/build"
    for directory in (builds, lane / "release/build"):
        for artifact in directory.glob("*/*/out/lib*"):
            if artifact.suffix in {".rmeta", ".dylib", ".so", ".dll"}:
                link = dependencies / artifact.name
                if not link.is_symlink():
                    link.symlink_to(artifact.resolve())
    pack, = builds.glob("semio-framework-pack/*/out/libpack-*.rmeta")
    exact = json.loads(pack.with_suffix(".rmeta.semio-deps").read_text())["dependencies"]
    names = {
        "semio_framework_pack_json",
        "semio_framework_value",
        "semio_framework_value_derive",
        "semio_framework_value_native_owned",
        "semio_framework_artifact_reference",
    }
    externs = {"pack": pack.resolve()}
    for name in names:
        stem, = [row[0] for row in exact if row[0].startswith(f"{name}-")]
        library, = [item for item in dependencies.glob(f"lib{stem}.*") if item.is_file()]
        externs[name] = library
    base = [
        "repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
        "--edition=2021", "--crate-type=rlib", "--emit=metadata", "--cap-lints=allow",
        "-L", f"dependency={dependencies}", "-L", f"dependency={output}",
    ]
    if target:
        base += ["--target", target]
    prefix = "semio_s_artifact_stdio_pdf_model_"
    for family, directory in (
        ("object", "🔗️object-model"), ("graphics", "🖋️content-operators"),
        ("font", "🔤️font-model"), ("page", "📃️page-model"),
    ):
        root = model / directory / "📦️packages/🦀️rust"
        section = (root / "Cargo.toml").read_text().split("[dependencies]\n")[1].split("\n[")[0]
        names = [match.replace("-", "_") for match in re.findall(r"^([\w-]+) =", section, re.M)]
        arguments = [arg for name in names for arg in ["--extern", f"{name}={externs[name]}"]]
        crate = prefix + family
        artifact = output / f"lib{crate}-check.rmeta"
        subprocess.run([*base, "--crate-name", crate, "-C", "extra-filename=-check", *arguments,
                        str(root / "🦀️.rs"), "-o", str(artifact)], check=True)
        externs[crate] = artifact
        print(f"[DEBUG] {target or 'native'} {crate} {hashlib.sha256(artifact.read_bytes()).hexdigest()}", flush=True)

    facade = (model / "🦀️.rs").read_text()
    source = facade + """
use semio_framework_pack_json::ArtifactCanonicalJsonTree;
pub fn canonical_leaves(object: PdfObject, filter: PdfStreamFilter, file: PdfEmbeddedFile) {
    assert!(object.canonical_tree_node().is_ok());
    assert!(filter.canonical_tree_node().is_ok());
    assert!(file.canonical_tree_node().is_ok());
}
pub fn page_font_probe(page: PdfPage, font: PdfFont, op: PdfOp) -> (usize, PdfFont, PdfOp) {
    (page.text().len(), font, op)
}
"""
    output_file = output / "libpdf_model.rmeta"
    arguments = [arg for name, path in externs.items() if name.startswith(prefix) or name == "semio_framework_pack_json"
                 for arg in ["--extern", f"{name}={path}"]]
    subprocess.run([*base, "--crate-name", "pdf_model_surface", *arguments, "-o", str(output_file), "-"],
                   input=source, text=True, check=True)
    print(f"[DEBUG] {target or 'native'} PDF public type surface {hashlib.sha256(output_file.read_bytes()).hexdigest()}", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("lane", type=Path)
    parser.add_argument("--target", choices=["wasm32-wasip2"])
    args = parser.parse_args()
    verify(args.lane, args.target)
