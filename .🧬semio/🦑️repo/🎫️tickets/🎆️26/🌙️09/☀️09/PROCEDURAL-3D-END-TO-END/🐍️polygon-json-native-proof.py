from pathlib import Path
import json
import subprocess
import sys

ticket=Path(__file__).resolve().parent
out=ticket/"🗑️generated"/"polygon-json-native"
out.mkdir(parents=True,exist_ok=True)
cache=Path(".cache/b/r/m/u/l")
artifacts={p.stem.removeprefix("lib"):p for lane in cache.glob("*/prototype/build") for p in lane.glob("*/*/out/*") if p.suffix in (".rlib",".dylib")}
engine=next(p for name,p in artifacts.items() if name.startswith("semio_framework_mesh_engine-"))
lineage=dict(json.loads(engine.with_suffix(".rmeta.semio-deps").read_text())["dependencies"])

def dependency(name):
    identity=next((key for key in lineage if key.startswith(name+"-")),None)
    if identity is None:
        identity=next(key for key in artifacts if key.startswith(name+"-"))
    return artifacts[identity]

def native(source,crate,externs,test=False,edition="2021"):
    binary=out/(crate if test else "lib"+crate+".rlib")
    cmd=["repo","tools","exec","rustup","--","run","nightly-2026-07-20","rustc",str(source),
         "--edition",edition,"--crate-name",crate,"-C","debuginfo=0","-o",str(binary)]
    cmd+=["--test"] if test else ["--crate-type","rlib"]
    for folder in sorted({p.parent for p in artifacts.values()})+[out]:
        cmd+=["-L","dependency="+str(folder)]
    for name,library in externs.items():
        cmd+=["--extern",name+"="+str(library)]
        if library.with_suffix(".rmeta").exists():
            cmd+=["--extern",name+"="+str(library.with_suffix(".rmeta"))]
    print("[DEBUG] compiling "+crate,flush=True)
    status=subprocess.run(cmd).returncode
    if status:
        raise SystemExit(status)
    return binary

mesh=Path("🧰️framework/🔨️modules/🏗️mesh-engine")
deps={name:dependency(name) for name in (
  "semio_framework_mesh_component_references","semio_framework_job","semio_framework_dsl_record",
  "semio_framework_value","semio_framework_value_native_owned","semio_framework_value_derive",
  "semio_framework_pack_json","semio_framework_io_base64")}
deps["protocol"]=dependency("protocol")
deps["pack"]=dependency("pack")
native_engine=out/"libsemio_framework_mesh_engine.rlib"
if "--reuse" not in sys.argv:
    native_engine=native(mesh/"🦀️.rs","semio_framework_mesh_engine",deps)
test=(mesh/"🚪️io/📝️text/🧪️tests/🧬️ownership/🦀️.rs").read_text()
tag="#[test]\nfn authored_polygon_json_writes_exact_wire_under_funded_turns"
body=test[test.index(tag):]
fixture=(mesh/"🚪️io/🧪️tests/🧫️fixtures/🔣️.json").resolve()
driver=out/"proof.rs"
driver.write_text("use semio_framework_mesh_engine::io::text::*;\nuse semio_framework_value::RetainedCloneGrant;\n"+body.replace("../../../🧪️tests/🧫️fixtures/🔣️.json",str(fixture)))
binary=native(driver,"polygon_json_proof",{"semio_framework_mesh_engine":native_engine,
  "semio_framework_value":deps["semio_framework_value"],"semio_framework_job":deps["semio_framework_job"],
  "serde_json":dependency("serde_json")},test=True)
raise SystemExit(subprocess.run([str(binary),"--nocapture"]).returncode)
