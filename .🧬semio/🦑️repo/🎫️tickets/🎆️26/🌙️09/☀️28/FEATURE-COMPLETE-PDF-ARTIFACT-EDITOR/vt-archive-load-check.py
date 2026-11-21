"""🧪 Checks the staged VT Rust syntax and, when available, the exact cached owner type bounds."""

from pathlib import Path
import json
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "vt-load"
RUSTC = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]

for name in ("editor.rs", "viewer.rs", "tests.rs"):
    subprocess.run([*RUSTC, "--edition=2021", "-Zunpretty=ast-tree", str(OUT / name)], stdout=subprocess.DEVNULL, check=True)
    print(f"[DEBUG] parsed staged PDF/VT {name}")

if "--typecheck" in sys.argv:
    deps = OUT / "deps"
    externs = json.loads((OUT / "externs.json").read_text(encoding="utf8"))
    kernel, = deps.glob("libsemio_os_kernel-*.rmeta")
    externs["protocol"] = kernel.name
    source = OUT / "owner-types.rs"
    viewer = (OUT / "viewer.rs").read_text(encoding="utf8")
    first = viewer.index("    fn document_store_owners_source_demands()")
    last = viewer.index("    fn reference_birth()", first)
    hooks = viewer[first:last].replace("Self::Snapshot", "PdfSnapshot").replace("Self::Mutation", "PdfMutation")
    source.write_text('''
use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
''' + hooks, encoding="utf8")
    args = [str(source), "--edition=2021", "--crate-type=lib", "--emit=metadata", "-o", str(OUT / "owner-types.rmeta"),
            f"-Ldependency={deps}"]
    for alias, file in externs.items():
        args += ["--extern", f"{alias}={deps / file}"]
    argfile = OUT / "owner-types.args"
    argfile.write_text("\n".join(args) + "\n")
    subprocess.run([*RUSTC, "@" + str(argfile)], check=True)
    print("[DEBUG] exact native PdfSnapshot and PdfMutation issuer bounds typechecked")
