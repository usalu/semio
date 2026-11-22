"""🌱 Install verified plant fields after a guarded independent rebuild from the current catalog."""
from hashlib import sha256
import json
import os
from pathlib import Path
from runpy import run_path
import shutil
import subprocess
import sys

ticket = Path(__file__).resolve().parent
gen = ticket / "🗑️generated"
sub, slug = sys.argv[1:]
catalog = next(ticket.parents[6].glob("🎓*/*architecture/*staffage/*catalog/*vegetation/" + slug + "/🔣️.json"))
model = ticket / "volumes" / "vegetation" / (slug + ".json")
candidate = gen / (sub + "-botanical-fit.json")
original, model_source = catalog.read_bytes(), model.read_bytes()
proof = sha256(candidate.read_bytes()).hexdigest()
review = json.loads((gen/"botanical-queue"/(sub+"-look.json")).read_text())
assert review["sourceSha256"] == proof
required = ["50","100","200","500","document","scenes", "grid" if sub == "ivy" else "anatomy"]
if sub in {"willow","bamboo","apple"}:required.append({"willow":"direct","bamboo":"close","apple":"windows"}[sub])
for tag in required:
    record = json.loads((gen/"botanical-queue"/(sub+"-"+tag+".json")).read_text())
    assert record["sourceSha256"] == proof and record["rc"] == 0, tag
    lines = (gen/record["log"]).read_text().splitlines()
    if tag in {"50","100","200","500"}:
        result = json.loads(lines[-1])
        assert len(result["rules"]) == 13 and not result["issues"], tag
    if tag == "scenes":
        result = json.loads(lines[-1])
        assert result["scenes"] == 12 and not result["findings"]
    if tag == "document":
        assert lines[-1].startswith("VALID file") and "validateAsset 0" in lines[-1]
staged = gen/("verified-"+sub+"-plant.json")
author = json.loads((gen/"botanical-queue"/(sub+"-author.json")).read_text())
assert author["rc"]==0 and author["candidateSha256"]==proof and author["sourceSha256"]==sha256(original).hexdigest()
cap = json.loads(author["key"])[0][4]
subprocess.run([sys.executable,ticket/"equipment-guard.py",shutil.which("bun"),"--smol",ticket/(sub+"-botanical-fit.ts"),catalog,staged],env=dict(os.environ,EQUIPMENT_RSS_MIB=str(cap),EQUIPMENT_HEADROOM_PERCENT="42"),check=True)
assert sha256(staged.read_bytes()).hexdigest()==proof
document=json.loads(original);prior=document["asset"];updated=json.loads(candidate.read_bytes())["asset"]
source=json.loads(model_source)
assert source["volumes"]==prior["volumes"]
changed={k for k in updated if updated[k]!=prior.get(k)}
assert changed and changed<={"volumes","representations","thermal","structure","anchors","poses","clips"}
locator=run_path(str(ticket/"r5-range-gaps-lib.py"))["key_value"]
def fields(raw,wrapped,keys):
    at=locator(raw,"asset",0,len(raw))[0] if wrapped else raw.index("{")
    spans=sorted([(*locator(raw,k,at,len(raw)),k) for k in keys],reverse=True)
    for lo,hi,k in spans:raw=raw[:lo]+json.dumps(updated[k],ensure_ascii=False,separators=(",",":"))+raw[hi:]
    return raw.encode()
next_catalog=fields(original.decode(),True,changed)
next_model=fields(model_source.decode(),False,["volumes"])
assert json.loads(next_catalog)["asset"]==updated and json.loads(next_model)==dict(source,volumes=updated["volumes"])
catalog_tmp, model_tmp = gen/(sub+"-catalog-install.tmp"), gen/(sub+"-volumes-install.tmp")
catalog_tmp.write_bytes(next_catalog)
model_tmp.write_bytes(next_model)
assert original==catalog.read_bytes() and model_source==model.read_bytes()
model_tmp.replace(model)
try:
    assert catalog.read_bytes()==original
    catalog_tmp.replace(catalog)
except BaseException:
    if model.read_bytes()==next_model:
        model_tmp.write_bytes(model_source)
        model_tmp.replace(model)
    raise
assert catalog.read_bytes()==next_catalog and model.read_bytes()==next_model
print(slug,sha256(next_catalog).hexdigest(),flush=True)
