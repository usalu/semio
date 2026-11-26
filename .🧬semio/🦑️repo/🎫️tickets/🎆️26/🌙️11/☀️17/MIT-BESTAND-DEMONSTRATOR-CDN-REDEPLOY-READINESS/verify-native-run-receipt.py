"""🧾️ Bind one staged browser receipt to a successful component within an exact Native run."""
import json
from pathlib import Path
import sys

REQUIRED_STAGES={"stage","transpile","describe","model","optimize","browser","complete"}

def read(path):
    return json.loads(path.read_bytes())

def main():
    run,modules=map(Path,sys.argv[1:3])
    plugin=sys.argv[3]
    package=f"semio-hub-{plugin}-component"
    crate=package.replace("-","_")
    provenance=run/"r/provenance"
    if not read(provenance/"reads.json")["closed"]:
        raise ValueError("run did not finish provenance custody")
    report=read(run/"r/units.json")
    stages=[row for row in report["stages"] if row["component"]==package]
    if any(not row["ok"] for row in stages) or not REQUIRED_STAGES<={row["stage"] for row in stages}:
        raise ValueError(f"{run.name} did not complete browser publication for {plugin}")
    receipt_paths=list(modules.glob(f"*️{plugin}/🧾️receipt.json"))
    if len(receipt_paths)!=1:
        raise ValueError(f"expected one staged receipt for {plugin}")
    receipt=read(receipt_paths[0])
    passes=[value for value in read(provenance/"leaves.json").values() if package in value["components"]]
    if len(passes)!=1:
        raise ValueError(f"{run.name} did not record exactly one component provenance for {plugin}")
    selected=passes[0]
    candidate=selected["components"][package]
    if (candidate["root"],candidate["sourceTreeDigest"])!=(receipt["provenanceSha256"],receipt["sourceTreeDigest"]):
        raise ValueError(f"{plugin} staged provenance differs from {run.name}")
    outcome=read(provenance/Path(selected["receipt"]).name)
    wasms=[artifact for unit in outcome["units"] if unit["message"]["target"]["name"]==crate
           for artifact in unit["artifacts"] if "stagedPath" in artifact]
    if len(wasms)!=1 or {wasms[0]["sha256"],wasms[0]["stagedSha256"]}!={receipt["wasmSha256"]}:
        raise ValueError(f"{plugin} staged Wasm differs from {run.name}")
    print(f"{run.name} {plugin} stages and exact raw provenance confirmed")

if __name__=="__main__":
    main()
