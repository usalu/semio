import re
import subprocess
import sys
from pathlib import Path

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated" / "store-source"
lane = Path(sys.argv[1]).resolve()
rustc = Path(sys.argv[2])
source = Path(sys.argv[3]).resolve()
wasi_run = "--wasi-run" in sys.argv[4:]
wasi = wasi_run or "--wasi" in sys.argv[4:]
publication, = lane.glob("semio-os-store-one-item-publication/*/out/*.rmeta")
dependencies = generated / ("wasi-deps" if wasi else "deps")
dependencies.mkdir(exist_ok=True)
artifact_roots = [lane, lane.parents[2] / "release/build"] if wasi else [lane]
for root in artifact_roots:
    for artifact in root.glob("*/*/out/*"):
        if artifact.suffix not in (".rlib", ".rmeta", ".dylib"):
            continue
        link = dependencies / artifact.name
        if link.is_symlink() and not link.exists():
            link.unlink()
        if not link.is_symlink():
            link.symlink_to(artifact)
metadata = subprocess.check_output([rustc, "-Z", "ls=all", publication], text=True)
externs = []
for name in ("semio_os_spr", "semio_framework_job", "protocol", "semio_framework_pack_json", "semio_framework_value", "semio_framework_hash", "semio_os_store_one_item_publication"):
    if name == "semio_os_store_one_item_publication":
        stem = publication.stem
    else:
        match = re.search(rf"^\d+ ({name}-[a-f0-9]+) hash ", metadata, re.M)
        stem = "lib" + match[1]
    for suffix in (("rmeta",) if wasi and not wasi_run else ("rmeta", "rlib")):
        externs.extend(["--extern", f"{name}={dependencies / (stem + '.' + suffix)}"])
binary = generated / "source-law"
entry = f'#[path = "{source}"]\nmod owner;\n'
options = ["--edition=2021", "--crate-name", "canonical_store_source_law", "-L", f"dependency={dependencies}"]
if wasi_run:
    entry += (ticket / "store-canonical-source-law.rs").read_text()
    if (generated / "hash-trace.rs").exists():
        entry += f'\n#[path = "{generated / "hash-trace.rs"}"]mod hash_trace;\nuse semio_os_spr::Edit;\n'
        entry += (generated / "settle-trace.rs").read_text()
    binary = generated / "store-canonical-source.wasm"
    options += ["--target=wasm32-wasip2", "-Cdebuginfo=0", "-o", binary]
elif wasi:
    entry += "pub use semio_os_store_one_item_publication::ArtifactStoreOneItemLiveAuthority; pub use owner::*;"
    options += ["--target=wasm32-wasip2", "--crate-type=rlib", "--emit=metadata", "-o", generated / "store-source-wasi.rmeta"]
else:
    entry += (ticket / "store-canonical-source-law.rs").read_text()
    options += ["-o", binary]
subprocess.run([rustc, *options, *externs, "-"], input=entry, text=True, check=True)
if not wasi:
    subprocess.run([binary], check=True)
