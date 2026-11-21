"""🔬 Link the genesis snapshot read authority law to a real repository compiler cohort."""
from pathlib import Path
import os,subprocess,sys
ROOT=Path(__file__).resolve().parent
OUT=ROOT/"🗑️generated"/"store-read-authority"
OUT.mkdir(parents=True,exist_ok=True)
CACHE=Path(sys.argv[1]).resolve()
response=OUT/"rustc.rsp"
flags=["--edition=2021","--crate-name","pdf_store_read_stamp_law","--test",str(ROOT/"📜️pdf-store-read-law.rs"),"-o",str(OUT/"law")]
for name in ("semio_framework_value_snapshot_read","serde_json"):
    candidates=sorted(CACHE.glob(f"*/*/out/lib{name}-*.rlib"))
    artifact=candidates[0]
    for suffix in (".rmeta",".rlib"):flags += ["--extern",f"{name}={artifact.with_suffix(suffix)}"]
for directory in CACHE.glob("*/*/out"):flags += ["-L",f"dependency={directory}"]
response.write_text("\n".join(flags)+"\n")
subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","@"+str(response)],check=True)
subprocess.run([OUT/"law","--nocapture"],check=True,env={**os.environ,"PDF_STORE_AUTHORITY_LAW":str(ROOT/"🔣️pdf-store-read-authority.json")})
