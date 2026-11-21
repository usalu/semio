"""🔗 Link the PDF 1.4 native receiver against the actual paid receiving contract."""
from pathlib import Path
import os
import subprocess
import sys
ROOT=Path(__file__).resolve().parent
OUT=ROOT/"🗑️generated"/"pdf14-wire"
OUT.mkdir(parents=True,exist_ok=True)
CACHE=Path(sys.argv[1]).resolve()
seen={}
for pattern in ("libsemio_framework_value-*.rlib","libsemio_framework_value_native_owned-*.rlib","libsemio_framework_replication_codec-*.rlib","libserde_json-*.rlib","libsemio_framework_replication_error-*.rlib","libsemio_framework_value_derive-*.dylib","libsemio_framework_value_derive-*.so","libsemio_framework_value_derive-*.dll"):
    for file in CACHE.glob("*/*/out/"+pattern):
        seen.setdefault(file.stem.split("-")[0].removeprefix("lib"),[]).append(file)
libs={}
for name,paths in seen.items():
    if len(paths)!=1:
        raise RuntimeError((name,paths))
    libs[name]=paths[0]
required=["semio_framework_value","semio_framework_value_native_owned","semio_framework_replication_codec","serde_json","semio_framework_value_derive","semio_framework_replication_error"]
for name in required:
    if name not in libs: raise RuntimeError(name)
(OUT/"owner.rs").write_text((ROOT/"📜️pdf14-wire-receiving.rs").read_text()+(ROOT/"📜️pdf14-wire-proof.rs").read_text().replace("EFFECT_FIXTURE_FILE",Path(os.environ["SEMIO_PDF14_EFFECT_FIXTURE"]).resolve().as_posix()))
(OUT/"law.rs").write_bytes((ROOT/"📜️pdf14-wire-law.rs").read_bytes())
args=["--edition=2021","--crate-name","pdf14_wire_law","--test",str(OUT/"law.rs"),"-o",str(OUT/"law")]
for n in required:
    f=libs[n]
    if f.suffix==".rlib":args+=["--extern",n+"="+str(f.with_suffix(".rmeta"))]
    args+=["--extern",n+"="+str(f)]
for link in CACHE.glob("*/*/out"):args+=["-L","dependency="+str(link)]
response=OUT/"args"
response.write_text("\n".join(args)+"\n")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+str(response)],check=True)
subprocess.run([OUT/"law","--nocapture"],check=True,env={**os.environ,"PDF14_WIRE_FRAMES":str(OUT/"frames.json")})
