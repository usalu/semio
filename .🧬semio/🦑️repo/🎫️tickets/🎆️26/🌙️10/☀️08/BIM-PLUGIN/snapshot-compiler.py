import os
import re
import shlex
import sys
from pathlib import Path

generated = Path(__file__).resolve().parent / "🗑️generated"
log = generated / "wasm-mounted.log"
match = re.search(r"process didn't exit successfully: `([^`]+?--crate-name semio_s_artifact_bim_model_snapshot [^\n`]+)`", log.read_text())
args = shlex.split(match[1])
source = next(Path(arg) for arg in args if arg.endswith(".rs"))
out = generated / "snapshot-compiler"
out.mkdir(exist_ok=True)
args[args.index("--out-dir")+1] = str(out)
os.environ.update(CARGO_MANIFEST_DIR=str(source.parent), CARGO_PKG_NAME="semio-s-artifact-bim-model-snapshot", CARGO_PKG_VERSION="0.1.0", CARGO_PRIMARY_PACKAGE="1")
args.extend(["-Zno-parallel-backend", "-Zthreads=1"])
os.execv(args[0],args)
