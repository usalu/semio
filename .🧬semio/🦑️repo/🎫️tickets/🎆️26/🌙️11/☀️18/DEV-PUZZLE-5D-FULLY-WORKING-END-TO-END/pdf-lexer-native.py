from pathlib import Path
import os
import re
import subprocess
import sys

root, output, compiler = map(Path, sys.argv[1:])
manifest = next(Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf").rglob("🔤️lexer/📦️packages/🦀️rust/Cargo.toml"))
library = max(root.glob("semio-s-artifact-stdio-pdf-lexer/*/out/lib*.rmeta"), key=lambda path: path.stat().st_mtime)
metadata = subprocess.check_output([compiler, "-Z", "ls=root", library], text=True)
output.mkdir(exist_ok=True, parents=True)
binary = output / "lexer-tests"
arguments = [compiler, "--test", manifest.parents[2] / "🦀️.rs", "--edition=2021", "--crate-name", "pdf_lexer_tests", "-Clinker-features=+lld", "-Clink-self-contained=+linker", "-Zunstable-options", "-o", binary]
for name, fingerprint in re.findall(r"^\d+ (semio_s_artifact_stdio_pdf_(?:model|snapshot_2))-([\da-f]+) hash", metadata, re.M):
    dependency = next(root.glob(f"*/{fingerprint}/out/lib{name}-{fingerprint}.rmeta"))
    arguments.extend(["--extern", f"{name}={dependency}"])
for directory in root.glob("*/*/out"):
    arguments.extend(["-L", f"dependency={directory}"])
subprocess.run(arguments, check=True)
subprocess.run([binary], check=True, env={**os.environ, "RUST_MIN_STACK": "67108864"})
