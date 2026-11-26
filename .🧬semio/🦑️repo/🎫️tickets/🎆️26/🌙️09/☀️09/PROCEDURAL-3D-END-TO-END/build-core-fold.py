"""🧊 Link the bounded-memory core folder against the pinned native encoder and parser."""
from pathlib import Path
import json
import os
import subprocess
t = Path(__file__).resolve().parent
g = t/"🗑️generated/bridge-current"
build = Path(".cache/b/r/m/tmp/flow-native-n25/debug/build").resolve()
unit = build/"wasm-encoder/236e933b11d492c1"
fingerprint = json.loads((unit/"fingerprint/lib-wasm_encoder.json").read_text())
parser_hash = next(dep[3] for dep in fingerprint["deps"] if dep[1]=="wasmparser").to_bytes(8,"little").hex()
parser = next(p.parent.parent for p in (build/"wasmparser").glob("*/fingerprint/lib-wasmparser") if p.read_text().strip()==parser_hash)
args=["rustup","run","nightly-2026-07-20","rustc","--edition=2021","-Copt-level=2","-Cdebuginfo=0",t/"deduplicate-requester.rs","-o",g/"deduplicate-requester"]
for name,folder in [("wasm_encoder",unit),("wasmparser",parser)]:
    for s in ["rlib","rmeta"]:
        artifact=next((folder/"out").glob("*."+s))
        args.extend(["--extern",name+"="+str(artifact)])
for folder in build.glob("*/*/out"):
    args.extend(["-L","dependency="+str(folder)])
subprocess.run(args,check=True)
