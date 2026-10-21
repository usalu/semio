import os, sys, json
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits)
        cur = os.path.join(cur, hits[0])
    return cur
base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/"
mod = resolve(base + "🟨️.mjs"); schema = resolve(base + "🧬️schema/🔣️.json")
text = open(mod, encoding="utf8").read()
edits = [
 ("const minimumAvailableBytes=request.minimumAvailableBytes,policy=quotaSchema.policy;let retiredBytes=0,retiredFiles=0;",
  "const minimumAvailableBytes=request.minimumAvailableBytes,policy=quotaSchema.policy,started=Date.now();let retiredBytes=0,retiredFiles=0,partial=false;"),
 ("if(remaining<=policy.lowWaterBytes&&free.bavail*free.bsize>=minimumAvailableBytes)break;await image.recheck();const actual=",
  "if(remaining<=policy.lowWaterBytes&&free.bavail*free.bsize>=minimumAvailableBytes)break;if(Date.now()-started>=policy.retirementWindowMilliseconds){partial=true;break;}if(retiredFiles%64===0)await image.recheck();const actual="),
 ('retiredFiles++;await control.publish({resource:"repository:graph-cache-retirement",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}',
  'retiredFiles++;if(retiredFiles%64===0){await control.publish({resource:"repository:graph-cache-retirement",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}}'),
 ("requiredAvailableBytes:minimumAvailableBytes,retiredBytes,retiredFiles});",
  "requiredAvailableBytes:minimumAvailableBytes,retiredBytes,retiredFiles,...(partial?{partial:true}:{})});"),
 ('const result=await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>retireOptionalGraphHeadroomV1(image,custody.kind==="kernel-ex"?custody.control:control,request));if(!result.acquired)return false;const value=result.value;if(value.retainedOptionalBytes>quotaSchema.policy.maximumBytes||value.availableBytes<value.requiredAvailableBytes)throw Error("Graph cache cannot admit original required publication headroom: "+JSON.stringify(value));return value.retiredFiles>0;',
  'let retired=false;for(;;){const result=await image.withExclusiveMaintenance({version:1,wait:false},async(_port,custody)=>retireOptionalGraphHeadroomV1(image,custody.kind==="kernel-ex"?custody.control:control,request));if(!result.acquired)return retired;const value=result.value;if(value.retiredFiles>0)retired=true;if(value.partial){await control.yieldContinuation();continue;}if(value.retainedOptionalBytes>quotaSchema.policy.maximumBytes||value.availableBytes<value.requiredAvailableBytes)throw Error("Graph cache cannot admit original required publication headroom: "+JSON.stringify(value));return retired;}'),
]
new = text; state = []
for old, rep in edits:
    if rep in new: state.append("applied")
    else:
        assert new.count(old) == 1, old[:60]
        new = new.replace(old, rep); state.append("pending")
raw = open(schema, encoding="utf8").read(); data = json.loads(raw)
if "retirementWindowMilliseconds" in data["definitions"]["OptionalQuota"]["policy"]: state.append("applied schema"); nraw = raw
else:
    data["definitions"]["OptionalQuota"]["policy"]["retirementWindowMilliseconds"] = 15000
    nraw = json.dumps(data, ensure_ascii=False, indent=2) + "\n"; assert json.dumps(json.loads(raw), ensure_ascii=False, indent=2) + "\n" == raw
    state.append("pending schema")
print(state)
if "--write" in sys.argv:
    open(mod, "w", encoding="utf8", newline="").write(new); open(schema, "w", encoding="utf8", newline="").write(nraw); print("WRITTEN")
