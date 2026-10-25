"""Adds a live `test-<id>` target for every suite or code suite of the caching route table that no target runs. Existing targets are never rewritten."""
import json,re
mod="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
p=mod+"/📋️project.json"
d=json.load(open(p,encoding="utf8"))
t=json.load(open(mod+"/🗂️suites/🔣️.json",encoding="utf8"))
ids=[x["id"] for x in t["suites"]]+t["codeSuites"]
pattern=re.compile(r"📜️script\.ts test (\S+)")
have={}
for name,target in d["targets"].items():
    m=pattern.search(target.get("options",{}).get("command",""))
    if m:have.setdefault(m.group(1),[]).append(name)
added=[]
lines=[]
for sid in ids:
    if sid in have:continue
    name="test-"+sid
    if name in d["targets"]:print("COLLISION",name,d["targets"][name]["options"]["command"]);continue
    target={"executor":"repo:script","cache":False,"dependsOn":[],"inputs":["{projectRoot}/**/*"],"outputs":[],"options":{"cwd":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching","command":"repo ./📜️script.ts test "+sid,"forwardAllArgs":False},"metadata":{"semio":{"execution":{"version":1,"kind":"source"},"cache":{"version":1,"class":"live","reason":"exercises-host-resources"}}}}
    lines.append("    "+json.dumps(name)+": "+json.dumps(target,ensure_ascii=False,separators=(",",":")))
    added.append(name)
text=open(p,encoding="utf8").read().split("\n")
close=text.index("  },")
if lines:
    text[close-1]=text[close-1].rstrip(",")+","
    text[close:close]=[l+("," if i<len(lines)-1 else "") for i,l in enumerate(lines)]
    open(p,"w",encoding="utf8").write("\n".join(text))
    json.load(open(p,encoding="utf8"))
orphans=[n for sid,ns in have.items() if sid not in ids and ns for n in ns]
print("added",len(added),"orphan targets (command id not in table):",orphans)
