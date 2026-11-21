import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
workspace = Path.cwd()
generated = ticket / "🗑️generated" / "store-source"
dependency_dir = generated / "receiver-deps"
dependency_dir.mkdir(exist_ok=True)
lane = Path(sys.argv[1]).resolve()
rustc = Path(sys.argv[2])
for suffix in ("rlib", "rmeta", "dylib"):
    for artifact in lane.glob(f"*/*/out/*.{suffix}"):
        link = dependency_dir / artifact.name
        if link.is_symlink() and not link.exists():
            link.unlink()
        if not link.is_symlink():
            link.symlink_to(artifact)
root, = lane.glob("semio-framework-value/*/out/libsemio_framework_value-*.rmeta")
externs = ["--extern", f"semio_framework_value={root}", "--extern", f"semio_framework_value={root.with_suffix('.rlib')}"]
source = workspace / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs"
store = source.read_text()
live_quotes = store[store.index("fn admission_demand"):store.index("fn authority_ref")]
binary = generated / "receiver-law"
subprocess.run([rustc, "--edition=2021", "--crate-name", "canonical_store_receiver_law", "-L", f"dependency={dependency_dir}", "-o", binary, *externs, "-"], input=(ticket / "store-canonical-receiver-law.rs").read_text() + live_quotes, text=True, check=True)
subprocess.run([binary], check=True)
