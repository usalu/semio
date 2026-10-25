"""Adds a live `test-<id>` target for every suite of the route table that no target runs and repairs targets that run the wrong suite."""
import json,re,sys
mod="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process"
p=mod+"/📋️project.json"
s=open(p,encoding="utf8").read();d=json.loads(s)
t=json.load(open(mod+"/🧪️testing/🗂️suites/🔣️.json",encoding="utf8"))
ids=[x["id"] for x in t["suites"]]+t["codeSuites"]
pattern=re.compile(r"📜️script\.ts test (\S+)$")
def suite_of(target):
    m=pattern.search(target.get("options",{}).get("command",""));return m.group(1) if m else None
fixed=[]
for name,target in d["targets"].items():
    sid=suite_of(target)
    if sid and name.startswith("test-") and name!="test-"+sid and name[5:] in ids and sid!=name[5:]:
        target["options"]["command"]=re.sub(r"test \S+$","test "+name[5:],target["options"]["command"]);fixed.append(name)
have={suite_of(v) for v in d["targets"].values()}
added=[]
for sid in ids:
    if sid in have or sid in [n[5:] for n in d["targets"] if n.startswith("test-")] and False: continue
    name="test-"+sid
    if name in d["targets"]: continue
    d["targets"][name]={"executor":"repo:script","cache":False,"dependsOn":[],"inputs":["{projectRoot}/**/*"],"outputs":[],"options":{"cwd":"🧰️framework/🔨️modules/🏃️process","command":"repo ./📜️script.ts test "+sid},"metadata":{"semio":{"execution":{"version":1,"kind":"source"},"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}}
    added.append(name)
open(p,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
print("fixed",fixed,"added",added)
