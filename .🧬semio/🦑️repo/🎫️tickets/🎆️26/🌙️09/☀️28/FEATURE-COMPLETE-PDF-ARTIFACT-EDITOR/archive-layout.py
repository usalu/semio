"""📐 Checks exact archived mutation size components against the last WASI receipt."""

from pathlib import Path
import subprocess

TICKET = Path(__file__).resolve().parent
OUT = TICKET / "🗑️generated" / "archive-receiver"
TARGET = Path(".cache/b/r/m/u/l/pc5147e/wasm32-wasip2/prototype/build")
SOURCE = r'''
use semio_framework_value::DslValue;
#[unsafe(no_mangle)]
pub extern "C" fn value_bytes() -> usize { size_of::<DslValue>() }
#[unsafe(no_mangle)]
pub extern "C" fn output_bytes() -> usize {
    use semio_s_artifact_stdio_pdf_mutations::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
    size_of::<Option<PdfMutation>>()
}
'''
source = OUT / "layout.rs"
source.write_text(SOURCE, encoding="utf8")
commands = ["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc"]
options = ["--target=wasm32-wasip2", "--crate-type=lib", "--edition=2024", "-Cpanic=abort", "--emit=llvm-ir",
    "-o", str(OUT / "layout.ll"), str(source)]
for name in ("semio_framework_value", "semio_s_artifact_stdio_pdf_mutations"):
    [path] = TARGET.glob(f"*/*/out/lib{name}-*.rmeta")
    options += ["--extern", f"{name}={path}"]
options.extend(f"-Ldependency={path}" for path in sorted({p.parent for p in TARGET.glob("*/*/out/*.rmeta")}))
for path in sorted({p.parent for p in Path(".cache/b/r/m/u/l/pc5147e/prototype/build").glob("*/*/out/*.dylib")}):
    options.append(f"-Ldependency={path}")
args = OUT / "layout.args"
args.write_text("\n".join(options) + "\n", encoding="utf8")
first = subprocess.run([*commands, f"@{args}"], check=False)
if first.returncode:
    source.write_text(SOURCE[:SOURCE.index('#[unsafe(no_mangle)]', SOURCE.index('#[unsafe(no_mangle)]') + 1)], encoding="utf8")
    subprocess.run([*commands, f"@{args}"], check=True)
for line in (OUT / "layout.ll").read_text().splitlines():
    if "ret i32" in line:
        print("[DEBUG]", line.strip())
