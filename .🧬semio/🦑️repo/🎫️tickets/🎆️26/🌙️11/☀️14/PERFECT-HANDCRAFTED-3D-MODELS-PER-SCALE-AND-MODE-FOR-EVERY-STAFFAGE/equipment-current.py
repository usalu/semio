"""📋 Distinguish current per-document measurements from earlier checks and unversioned logs."""
from collections import Counter
from datetime import datetime
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import sys

ticket=Path(__file__).resolve().parent
catalog=next((ticket.parents[6]/"🎓️teaching").glob("*architecture/*staffage/*catalog"))
generated=ticket/"🗑️generated"
schema=sha256((catalog.parent/"🧬️schema/🔣️.json").read_bytes()).hexdigest()
schema_rows=[json.loads(s) for s in (generated/"equipment-check/catalog-schema.jsonl").read_text().splitlines()]
historical_schemas={(r["id"],r["sha256"]):r.get("schemaSha256","unrecorded") for r in schema_rows if r["valid"]}
schemas={(r["id"],r["sha256"]) for r in schema_rows if r.get("schemaSha256")==schema and r["valid"]}
assets=sorted(p for c in ("components","machinery","vehicles") for p in next(catalog.glob("*"+c)).glob("*/*.json"))
states={};signatures={}
for mode in ("glb","strict","document"):
    signatures[mode]=subprocess.check_output([sys.executable,str(ticket/"equipment-fleet.py"),mode,"--signature"],text=True).strip()
    state=json.loads(subprocess.check_output([sys.executable,str(ticket/"equipment-fleet.py"),mode,"--status"]))
    states[mode]={asset:key for key,slugs in state.items() for asset in slugs}
history={}
history_path=generated/"equipment-fleet/history.jsonl"
for line in history_path.read_text().splitlines() if history_path.exists() else []:
    r=json.loads(line)
    if r.get("completed") and r.get("current"):history[r["asset"],r["mode"],r["input"]]=r
def new_measurement(slug,mode,digest):
    group=states[mode][slug]
    if group=="pending":
        previous=generated/"equipment-fleet"/f"{slug}-{mode}.json"
        if previous.exists():
            old=json.loads(previous.read_text())
            if old.get("input")==digest and old.get("completed"):
                result="pass" if old.get("passed") else "findings"
                return f"Earlier {result} · same asset, previous implementation"
        saved=history.get((slug,mode,digest))
        if saved:
            state="pass" if saved["passed"] else "findings"
            return f"Earlier {state} · same asset, previous implementation"
        return "Pending"
    record=json.loads((generated/"equipment-fleet"/f"{slug}-{mode}.json").read_text())
    assert record["input"]==digest
    if group=="findings":
        if mode=="glb":return "; ".join(i["message"] for i in record["result"].get("findings",[])) or record["result"].get("error","Open findings")
        return "; ".join(l for l in record["result"]["output"] if l.startswith(("ASSET ","PRINT ","SCHEMA ","VOLUMES ")))[:380].replace("|","/")
    if mode=="glb":return f"Pass · {record['result']['bytes']:,} B"
    return "Pass"
quick={r["asset"]:r for line in (generated/"equipment-check/analytic.jsonl").read_text().splitlines() if (r:=json.loads(line))}
source=generated/"equipment-check/semantic.jsonl"
semantics={r["asset"]:r for line in source.read_text().splitlines() if (r:=json.loads(line))} if source.exists() else {}
source=generated/"equipment-check/print.jsonl"
prints={r["asset"]:r for line in source.read_text().splitlines() if (r:=json.loads(line))} if source.exists() else {}
rows=[];summary=Counter()
for f in assets:
    slug=f.parent.name;digest=sha256(f.read_bytes()).hexdigest()
    old=generated/"digital-live"/(slug+".json");old=json.loads(old.read_text()) if old.exists() else {}
    h="Earlier matching schema" if (slug,digest) in schemas else ("Earlier schema "+historical_schemas[(slug,digest)][:12] if (slug,digest) in historical_schemas else "—")
    if old.get("digest")==digest and old.get("current") and old.get("output"):
        h+=" + earlier full strict "+("pass" if old["exit"]==0 else "findings")
    semantic=semantics.get(slug,{})
    if semantic.get("input")==digest and not semantic.get("issues"):
        summary["semantic"]+=1;h+=" + catalog contract"
    g=new_measurement(slug,"glb",digest);s=new_measurement(slug,"strict",digest)
    document=new_measurement(slug,"document",digest)
    if document=="Pending":
        full=generated/"equipment-fleet"/(slug+"-strict.json")
        if full.exists():
            proof=json.loads(full.read_bytes())
            if proof.get("completed") and proof.get("input")==digest:
                lines=proof["result"].get("output",[])
                if any("schema 0, validateAsset 0" in l for l in lines) or s=="Pass":document="Pass within full strict"
                elif any(l.startswith(("ASSET ","PRINT ","SCHEMA ")) for l in lines):document="Open findings"
    shard_path=generated/"equipment-fleet"/(slug+"-shards.json")
    if shard_path.exists():
        shard=json.loads(shard_path.read_bytes())
        if shard.get("input")==digest and "document" in shard["parts"]:
            if document=="Pending":document="Pass · checkpointed full document" if not shard["parts"]["document"] else "Open paper or document findings"
            if s=="Pending":
                fullscales=[k for k in ("500","200","100","50") if k in shard["parts"]]
                single=sum("-" in k for k in shard["parts"])
                if fullscales or single:s="Partial · "+", ".join(["1:"+v for v in fullscales]+([str(single)+" individual rules"] if single else []))
    paper=prints.get(slug,{})
    if (slug,digest) in schemas and semantic.get("input")==digest and not semantic.get("issues") and paper.get("input")==digest:
        document="Pass · schema, contract and paper" if not paper.get("issues") else str(paper["issues"][0][-1])[:150]
    summary["same_document"]+=document.startswith(("Pass","Earlier pass"))
    summary["glb"]+=g.startswith("Pass");summary["strict"]+=s=="Pass";summary["sameglb"]+=g.startswith(("Pass","Earlier pass"));summary["samestrict"]+=s.startswith(("Pass","Earlier pass"));summary["schema"]+=(slug,digest) in schemas;summary["historical"]+=(slug,digest) in historical_schemas
    q=quick.get(slug,{})
    analytical="Pass" if q.get("input")==digest and q.get("code") and not q.get("issues") else "Pending"
    summary["analytical"]+=analytical=="Pass"
    rows.append(f"| {slug} | `{digest}` | {h} | {document} | {g} | {s} | {analytical} |")
parts=["# Current equipment verification","",f"Measured {datetime.now().astimezone():%Y-%m-%d %H:%M %Z}. Successful twelve-scene GLB results in this batch match {summary['sameglb']} of the 43 current asset hashes. Document and office-paper passes match {summary['same_document']}. Full strict passes match {summary['samestrict']}. Of these, {summary['glb']} GLB and {summary['strict']} strict passes also match every transitive implementation dependency at this snapshot. The catalog contract passed for {summary['semantic']} matching documents. Normative-schema evidence matches {summary['historical']} current asset file hashes. Of those, {summary['schema']} also match the current schema revision.","",f"The independently bounded analytical run passed availability, bone binding, physical feature, copying, slenderness and solid validity for {summary['analytical']} matching live assets. The earlier column records checks with a matching input hash. Earlier plain-text product runs are documented in [the equipment audit](equipment-audit.md). The new batch hashes the complete catalog file and the local transitive implementation dependencies. A GLB pass includes topology, scale triangle progression, glTF round-trip, scene and variant checks and the 8 MiB ceiling. A full strict pass includes schema, semantics, paper checks and all volume rules.","","| Asset | Source SHA-256 | Earlier matching evidence | Document and paper | New GLB | New full strict | Six analytical rules |","| --- | --- | --- | --- | --- | --- | --- |",*rows,"",f"Signature of the current GLB implementation: `{signatures['glb']}`. Signature of the current strict implementation: `{signatures['strict']}`. Normative schema: `{schema}`.","", "The file-level tests and dimensional changes are described in [the equipment revisions](equipment-scoped.md). The crate’s independent intermediate-scale and digital geometry checks are in [the crate report](r3-objects-storage-crate-stack.md)."]
(ticket/"equipment-current.md").write_text("\n".join(parts)+"\n")
