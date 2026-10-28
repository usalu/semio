"""Staged for after the measurement window (11:30): fold-safe Windows case mode in preflight and Bazel readiness.
usage: python3 apply-foldsafe-preflight.py --check | --apply
Every edit is an exact, unique-anchor replacement; --check proves all anchors before --apply writes anything."""
import json,sys
R="/Users/ueli/Documents/semio/"
D=R+"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts"
FAC=R+"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts"
L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
PF=L+"🏗️build/🩺️preflight/"
BZ=L+"⚡️caching/🚀️bootstrap/🛠️tools/🏗️bazel/🩺️readiness/🟦️.ts"
edits={
 D:[("/** 📂️ Proves that a fresh volume directory beneath the host can hold case-sensitive content once the platform prepared it. */",
 "/** 🔬️ Classifies what a fresh volume beneath the host can hold: case-sensitive content, fold-safe content when only a Windows host without the optional feature refuses the per-directory flag, or neither. */\nexport function probeNativeDirectoryCaseModeV1(directory:string,port:NativeDirectoryPlatformPortV1=nativeDirectorySystemPortV1()):\"sensitive\"|\"fold-safe\"|\"none\"{\n const scratch=join(directory,\".capability-\"+randomBytes(4).toString(\"hex\"));mkdirSync(scratch,{mode:0o700});\n try{port.prepareVolume(scratch);return probeNativeDirectoryCaseSensitivityV1(scratch)?\"sensitive\":\"none\";}\n catch(error){if(port.kind===\"journaled\"&&(protocol.foldSafeRefusals as readonly string[]).some(key=>(protocol.refusals as Readonly<Record<string,string>>)[key]===(error as Error)?.message))return\"fold-safe\";throw error;}\n finally{rmSync(scratch,{recursive:true,force:true});}\n}\n/** 📂️ Proves that a fresh volume directory beneath the host can hold case-sensitive content once the platform prepared it. */")],
 FAC:[("nativeDirectoryHostAvailableBytesV1, nativeDirectoryProtocolV1,","nativeDirectoryHostAvailableBytesV1, nativeDirectoryProtocolV1,"),("probeNativeDirectoryCaseCapabilityV1,","probeNativeDirectoryCaseCapabilityV1, probeNativeDirectoryCaseModeV1,")],
 PF+"🟦️.ts":[
  ("probeNativeDirectoryCaseCapabilityV1,","probeNativeDirectoryCaseCapabilityV1,probeNativeDirectoryCaseModeV1,"),
  ('if(required.exchange&&actual.caseSensitive!=="yes")problems.push','if(required.exchange&&actual.caseSensitive!=="yes"&&actual.caseSensitive!=="fold-safe")problems.push'),
  ('caseSensitive=probeNativeDirectoryCaseCapabilityV1(host)?"yes":"no";','caseSensitive=({sensitive:"yes",none:"no","fold-safe":"fold-safe"} as const)[probeNativeDirectoryCaseModeV1(host)];')],
 PF+"🧬️schema/🔣️.json":[('"caseSensitive": {"enum": ["yes", "no", "unprobed"]}','"caseSensitive": {"enum": ["yes", "fold-safe", "no", "unprobed"]}')],
 PF+"🧪️tests/🧮️matrix/🧬️schema/🔣️.json":[('"caseSensitive": { "type": "array", "items": { "enum": ["yes", "no", "unprobed"] } }','"caseSensitive": { "type": "array", "items": { "enum": ["yes", "fold-safe", "no", "unprobed"] } }')],
 PF+"🧪️tests/🧮️matrix/🔣️.json":[('  "caseSensitive": [\n   "yes",\n   "no",\n   "unprobed"\n  ]','  "caseSensitive": [\n   "yes",\n   "fold-safe",\n   "no",\n   "unprobed"\n  ]')],
 PF+"🧪️tests/🧮️matrix/🐍️oracle/🐍️.py":[('facts["caseSensitive"] != "yes":','facts["caseSensitive"] not in ("yes", "fold-safe"):')],
 BZ:[('readonly caseSensitive:"yes"|"no"|"unprobed"','readonly caseSensitive:"yes"|"fold-safe"|"no"|"unprobed"'),
  ('actual.caseSensitive!=="yes")refusals.push','actual.caseSensitive!=="yes"&&!(actual.platform==="win32"&&actual.caseSensitive==="fold-safe"))refusals.push'),
  ('try{caseSensitive=probeNativeDirectoryCaseCapabilityV1(host)?"yes":"no";}catch{caseSensitive="no";}','try{caseSensitive=({sensitive:"yes",none:"no","fold-safe":"fold-safe"} as const)[probeNativeDirectoryCaseModeV1(host)];}catch{caseSensitive="no";}'),
  ('import {probeNativeDirectoryCaseCapabilityV1,probeNativeDirectoryExchangeV1}','import {probeNativeDirectoryCaseModeV1,probeNativeDirectoryExchangeV1}')],
}
def run(apply):
    plan={}
    for path,pairs in edits.items():
        t=open(path,encoding="utf8").read()
        for a,b in pairs:
            if a==b: continue
            n=t.count(a)
            if n!=1: print("ANCHOR",n,path[-60:],a[:70]); sys.exit(1)
            t=t.replace(a,b)
        plan[path]=t
    print("all anchors unique:",sum(len(v) for v in edits.values()))
    if apply:
        for path,t in plan.items():
            if path.endswith(".json"): json.loads(t)
            open(path,"w",encoding="utf8").write(t)
        print("applied")
run("--apply" in sys.argv)
