"""🔬 Check the live neural schema implementation against an exact previously built native dependency graph."""
import json
import os
import re
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
out = ticket / "🗑️generated/bridge-current/registry-check"
out.mkdir(parents=True, exist_ok=True)
build = Path(".cache/b/r/m/tmp/engine-n25/debug/build")
engine = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine")
fingerprints = {}
for fp in build.glob("*/*/fingerprint/lib-*"):
    if fp.suffix or not fp.is_file():
        continue
    try:
        fingerprints[int.from_bytes(bytes.fromhex(fp.read_text()),"little")] = fp
    except ValueError:
        pass
compiler = subprocess.check_output(["rustup","which","rustc"],text=True).strip()
overrides = {}
artifacts = {}
def materialize(fp):
    name = fp.name[4:]
    if name in overrides:
        return overrides[name]
    if fp in artifacts:
        return artifacts[fp]
    unit = fp.parent.parent
    choices = [unit/"out"/("lib"+name+"-"+unit.name+ext) for ext in [".dylib",".rmeta",".rlib"]]
    actual = next((candidate for candidate in choices if candidate.exists()),None)
    if actual is None:
        raise FileNotFoundError(fp)
    link = out/actual.name
    if not link.is_symlink():
        link.symlink_to(actual.resolve())
    artifacts[fp] = link
    dependencies(fp)
    return link

def dependencies(fp):
    edges = json.loads(fp.with_suffix(".json").read_text())["deps"]
    result = {key:materialize(fingerprints[h]) for _,key,_,h in edges if h in fingerprints}
    for _, key, _, h in edges:
        if key in result or key == "build_script_build": continue
        old = next((fp.parent.parent/"out").glob("*.rmeta"),None)
        if old is None: continue
        info = subprocess.check_output([compiler,"-Zls=root",str(old)],text=True)
        match = re.search(r"^\d+ "+re.escape(key)+r"-([0-9a-f]{16}) ",info,re.M)
        if match:
            exact=next(build.glob("*/"+match[1]+"/fingerprint/lib-"+key),None)
            if exact: result[key]=materialize(exact)
    return result

def check(unit, domain):
    fp = next((build/unit).glob("fingerprint/lib-*.json")).with_suffix("")
    name = fp.name[4:]
    manifest = engine/domain/"📦️packages/🦀️rust"
    output = out/("lib"+name+"-live.rmeta")
    args = [compiler,"--crate-name",name,"--crate-type=lib","--edition=2021","--emit=metadata","-Cmetadata=live_schema_1125","-Zthreads=1","-Ldependency="+str(out)]
    for dep,artifact in dependencies(fp).items():
        args.extend(["--extern",dep+"="+str(artifact)])
    args.extend(["-o",str(output),str(manifest/"../../🦀️.rs")])
    print("[DEBUG] checking",name,flush=True)
    subprocess.run(args,env=dict(os.environ,CARGO_MANIFEST_DIR=str(manifest.resolve()),CARGO_PKG_VERSION="0.1.0"),check=True)
    overrides[name] = output

check("semio-os-kernel-neural-engine-model/3afc104a1e7aa74e","🧱️model")
check("semio-os-kernel-neural-engine-registry/d8c362c6c6a211fe","📔️registry")
