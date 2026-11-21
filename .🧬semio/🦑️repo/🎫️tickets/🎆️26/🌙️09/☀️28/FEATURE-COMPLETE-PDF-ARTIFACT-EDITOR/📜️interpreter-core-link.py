"""🔗 Check the staged interpreter composer against actual repository Value, Actor and Job artifacts."""
import json, os, re, subprocess, sys
from pathlib import Path
TICKET = Path(__file__).resolve().parent
WORK = TICKET / "🗑️generated" / "scalar-law"
ROOT = Path.cwd()
PREFIX = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/"
CACHE = Path(sys.argv[1]).resolve()
rows = json.loads((TICKET/"🗑️generated"/"interpreter-scalar-overlay.json").read_text())
candidates = {Path(path): source for path, _, source in rows if path.startswith(PREFIX) and path.endswith(".rs")}
source = Path(PREFIX + "🦀️.rs")
root = candidates[source]
work_root = WORK/"core.rs"
for local in ("🧱️core-types-and-module-model/🦀️.rs","🧮️numeric-execution/🦀️.rs"):
    selected = Path(PREFIX+local)
    if selected in candidates:
        output = WORK / local
        output.parent.mkdir(parents=True,exist_ok=True)
        output.write_text(candidates[selected])
def path_for(base, local):
    path = Path(PREFIX + local)
    selected = WORK/local if path in candidates and local != "🦀️.rs" else ROOT/path
    return base + json.dumps(selected.as_posix(),ensure_ascii=False)
root=re.sub(r'(include_str!\()"(.*?)"',lambda m: path_for(m[1],m[2]),root)
root=re.sub(r'(#\[path\s*=\s*)"(.*?)"',lambda m: path_for(m[1],m[2]),root)
work_root.write_text(root)
args = ["--edition=2021","--crate-name","semio_plugin_interpreter","--crate-type","rlib","--emit","metadata",str(work_root),"-o",str(WORK/"libsemio_plugin_interpreter.rmeta")]
for name in ("semio_framework_actor","semio_framework_job","semio_framework_value"):
    library, = CACHE.glob(f"*/*/out/lib{name}-*.rmeta")
    args.extend(("--extern",f"{name}={library}"))
args.extend(("--extern",f"semio_plugin_interpreter_scalar={WORK}/libsemio_plugin_interpreter_scalar.rlib"))
for d in CACHE.glob("*/*/out"):
    args.extend(("-L","dependency="+str(d)))
response=WORK/"core-rustc.rsp"
response.write_text("\n".join(args)+"\n")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+str(response)],check=True)
print("[DEBUG] Exact staged core interpreter compiled against the active repository library types")
