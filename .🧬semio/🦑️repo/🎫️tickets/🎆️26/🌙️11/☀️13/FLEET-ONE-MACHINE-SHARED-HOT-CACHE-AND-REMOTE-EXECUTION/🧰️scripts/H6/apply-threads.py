import json,os,re,sys
U="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
S=os.path.dirname(os.path.abspath(__file__))
def write(path,text):
    t=path+".h6.tmp"; open(t,"w").write(text); os.replace(t,path)
# schema
p=U+"/🧬️schema/🔣️.json"; s=open(p).read()
anchor='        "pool": {\n          "maximumLanes": 16,'
assert s.count(anchor)==1
new='''        "threads": {
          "description": "The front-end thread count of one compile unit. The repository's rustflags give every unit the parallel front end (`-Z threads=0`, all cores), which multiplies the front end's memory; a unit gets `constrained` threads instead when its measured peak (learned peak or carving measurement) is above the library budget of the admission policy, or when the host warden reports a level other than calm or less available memory than its guard line. The flag is appended after the unit key is taken and the outputs are identical for any thread count.",
          "constrained": 1
        },
'''
s=s.replace(anchor,new+anchor); json.loads(s); write(p,s)
# script
p=U+"/📜️script.ts"; s=open(p).read()
a='...compilerTemporaryArgumentsV1(settings.laneRoot?join(settings.laneRoot,policy.layout.compilerTemporaries):lanePathsV1(settings.root,settings.lane).temporaries,args)]'
assert s.count(a)==1
s=s.replace(a,a[:-1]+',...await frontEndThreadsArgumentsV1(settings.root,crate,unit.target,args)]')
imp='import {compilerTemporaryArgumentsV1,lanePathsV1} from "./🧵️lane/🟦️.ts";\n'
assert s.count(imp)==1
s=s.replace(imp,imp+'import {unitThreadsArgumentsV1,unitThreadsVerdictV1} from "./🧵️threads/🟦️.ts";\n')
helper='''/** 🧵️ The `-Z threads` argument the policy gives this unit (see `🧵️threads`): its peak is the larger of the carving measurement and the learned peak, the host is the warden's published state; any failure to learn either leaves the repository's rustflags in force. */
async function frontEndThreadsArgumentsV1(root:string,crate:string,target:string|undefined,args:readonly string[]):Promise<readonly string[]>{
 try{
  const limits=(await import("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧬️schema/🔣️.json")).default.definitions.Policy.const.unitCeiling;
  const budgetMegabytes=limits.classes[limits.workerClass as keyof typeof limits.classes].libraryMegabytes as number,carved=limits.carving.find(entry=>entry.crate===crate&&entry.target===(target??"host"))?.measuredMegabytes;
  const offload=await import("../../../🏗️build/🌐️remote-execution/🚀️offload/🟦️.ts"),learned=offload.readUnitPeakV1(root,offload.unitPeakKeyV1(crate,offload.unitProfileKeyV1(args)),1);
  const peaks=[carved,learned].filter((value):value is number=>typeof value==="number"),warden=(await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")).readHostWardenStateV1();
  return unitThreadsArgumentsV1(unitThreadsVerdictV1({args,budgetMegabytes,...peaks.length>0?{peakMegabytes:Math.max(...peaks)}:{},...warden===null?{}:{host:{level:warden.level,availableMegabytes:warden.host.availableMegabytes,guardMegabytes:warden.guardMegabytes}}}));
 }catch{return[];}
}

'''
m='/** 🏔️ Compares the peak resident memory'
assert s.count(m)==1
s=s.replace(m,helper+m); write(p,s)
os.makedirs(U+"/🧵️threads",exist_ok=True); os.makedirs(U+"/🧪️tests/🧵️threads",exist_ok=True)
import shutil
shutil.copy(S+"/🧵️threads/🟦️.ts",U+"/🧵️threads/🟦️.ts")
for n in ["🟦️.ts","🔣️.json","🐍️.py"]: shutil.copy(S+"/🧪️tests/🧵️threads/"+n,U+"/🧪️tests/🧵️threads/"+n)
print("applied")
