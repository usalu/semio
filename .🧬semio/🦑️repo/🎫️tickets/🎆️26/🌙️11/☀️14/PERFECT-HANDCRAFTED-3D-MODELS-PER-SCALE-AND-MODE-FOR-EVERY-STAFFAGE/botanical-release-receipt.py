"""🌱 Compare installed catalog, editable volumes and both render directions to the measured candidate."""
from hashlib import sha256
from pathlib import Path
import json
import sys

ticket = Path(__file__).resolve().parent
sub, slug = sys.argv[1:3]
g = ticket/"🗑️generated"
records = g/"botanical-queue"
def read(tag):
    return json.loads((records/(sub+"-"+tag+".json")).read_text())
def digest(path):
    return sha256(path.read_bytes()).hexdigest()
catalog = next(ticket.parents[6].glob("🎓*/*architecture/*staffage/*catalog/*vegetation/"+slug+"/🔣️.json"))
candidate = g/(sub+"-botanical-fit.json")
volumes = ticket/"volumes"/"vegetation"/(slug+".json")
installed = read("installed")
assert installed["catalog"] == digest(catalog) and installed["candidate"] == digest(candidate)
doc = json.loads(catalog.read_bytes())
assert doc == json.loads(candidate.read_bytes())
assert doc["asset"]["volumes"] == json.loads(volumes.read_bytes())["volumes"]
for tag in ["50","100","200","500","document","scenes","grid" if sub=="ivy" else "anatomy","look"]:
    stamp = read(tag)
    assert stamp["sourceSha256"] == installed["candidate"] and stamp.get("rc",0)==0
    if tag in ["50","100","200","500"]:
        result = json.loads((g/stamp["log"]).read_text().splitlines()[-1])
        assert len(result["rules"])==13 and not result["issues"]
    if tag=="scenes":
        scenes=json.loads((g/stamp["log"]).read_text().splitlines()[-1])
        assert scenes["scenes"]==12 and not scenes["findings"]
images = {}
for tag in ["calm","reverse"]:
    old,new = read(tag),read("installed-"+tag)
    assert new["sourceSha256"]==installed["catalog"] and new["rc"]==0
    def image_file(r):
        value = (g/r["log"]).read_text().splitlines()[-1].removeprefix("[DEBUG] ")
        return Path(value)
    preview,shipped = image_file(old),image_file(new)
    assert digest(preview)==digest(shipped) and installed["catalog"][:10] in shipped.name
    images[tag]=shipped.name
print(json.dumps(dict(asset=slug,catalog=installed["catalog"],candidate=installed["candidate"],glbBytes=scenes["bytes"],rules=52,scenes=12,images=images)))
