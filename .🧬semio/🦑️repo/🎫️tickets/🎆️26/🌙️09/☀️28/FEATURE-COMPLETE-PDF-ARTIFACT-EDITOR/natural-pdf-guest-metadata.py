"""Build a rustc metadata argument file for the focused real-dependency receiver cut."""
from pathlib import Path
import sys

output = Path(__file__).parent / "🗑️generated" / "natural-pdf-admission" / "receiver-check"
artifacts = Path(sys.argv[1])
assert output.is_dir() and artifacts.is_dir()
names = (
    "serde",
    "wit-bindgen",
    "semio-framework-value-derive",
    "semio-framework-value",
    "semio-framework-value-persistent",
    "semio-framework-job",
    "semio-framework-replication-command-ingress",
)
if (output / "publication-owner.rs").exists():
    names += ("semio-plugin-decl-emission", "semio-plugin-ownership")
arguments = [
    "--crate-name", "natural_source_receiver_check", "--crate-type", "lib", "--edition=2024",
    "--target=wasm32-wasip2", "-Cpanic=abort", "--emit=metadata", "--out-dir", str(output), str(output / "lib.rs"),
]
for folder in artifacts.glob("*/*/out"):
    arguments.extend(("-L", f"dependency={folder}"))
for folder in (artifacts.parents[2] / artifacts.parent.name / "build").glob("*/*/out"):
    arguments.extend(("-L", f"dependency={folder}"))
for name in names:
    stem = name.replace("-", "_")
    if name.endswith("-derive"):
        libraries = (artifacts.parents[2] / artifacts.parent.name / "build" / name).glob(f"*/out/*{stem}-*.*")
        libraries = [file for file in libraries if file.suffix in {".so", ".dll", ".dylib"}]
    else:
        libraries = (artifacts / name).glob(f"*/out/lib{stem}-*.rmeta")
    libraries = sorted(libraries, key=lambda file: file.stat().st_mtime)
    assert libraries, name
    arguments.extend(("--extern", f"{stem}={libraries[-1]}"))
(output / "check.args").write_bytes(("\n".join(arguments) + "\n").encode("utf-8"))
