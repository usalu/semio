import sys,os
R=os.environ.get("R39_ROOT","/Users/ueli/Documents/semio")
UC="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
SCRIPT=UC+"/📜️script.ts"
L1='import {admitFactsV1,expandRustcArgumentsV1,keyEnvironmentV1,normalizeRustcArgumentsV1,parseRustcUnitV1,rustcBypassV1,toolchainIdentityV1,unitKeyComponentsV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "./🔏️key/🟦️.ts";'
L1N='import {admitFactsV1,expandRustcArgumentsV1,keyEnvironmentV1,laneRestoreV1,normalizeRustcArgumentsV1,parseRustcUnitV1,rustcBypassV1,sha256V1,toolchainIdentityV1,unitKeyComponentsV1,unitKeyPartsV1,unitKeyV1,UNIT_CACHE_POLICY_V1} from "./🔏️key/🟦️.ts";'
L2='import {externIdentityV1,markExternIdentityV1} from "./🪪️extern/🟦️.ts";'
L2N=L2+'\nimport {dependencyDirectoriesV1,interfaceEligibilityV1,interfaceIdentityV1,rebaseRestoredUnitV1,settleUnitInterfaceV1,writeDependenciesV1} from "./🔭️interface/🟦️.ts";'
L3=' const digests=new Map<string,string>();\n'
L3N=' const digests=new Map<string,string>(),metadataOnly=unit.emit.every(kind=>kind==="metadata"||kind==="dep-info");let ifaceKeyed=false;\n'
L4='digests.set(row.path,externIdentityV1(path,sha,settings.stamps)??sha);}'
L4N='const identity=metadataOnly?interfaceIdentityV1(path,sha):undefined;if(identity!==undefined)ifaceKeyed=true;digests.set(row.path,identity??externIdentityV1(path,sha,settings.stamps)??sha);}'
L5=' const lookup=(outcome:"local-hit"|"fleet-hit"|"miss",remote?:string):void=>{\n'
L5N=''' const producer=interfaceEligibilityV1(unit,args)===undefined;
 const settle=(outputs:readonly string[],entry:Parameters<typeof settleUnitInterfaceV1>[0]["entry"]):void=>{
  if(!ifaceKeyed&&!producer)return;
  try{settleUnitInterfaceV1({outputs,entry,producer,identityOf:digest=>sha256V1(JSON.stringify(["iface",portableKeyV1({...keyInput,sourceSha256:"iface:"+digest},roots),sensitive,entry.environment.map(row=>[row.name,row.value===null?null:portableTextV1(row.value,roots)])])),neutralName:path=>portableTextV1(path,roots),readFile:path=>{try{return readFileSync(resolve(cwd,laneRestoreV1(path,settings.laneRoot)));}catch{return null;}},digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment});}catch{}
 };
'''+L5
L6='  shim(entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name)));\n  markExternIdentityV1(entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name)),portableKeyV1(keyInput,roots),sensitive,settings.stamps);\n'
L6N='''  const restoredFiles=entry.outputs.filter(row=>!row.template).map(row=>join(outDir,row.name));
  if(ifaceKeyed&&!rebaseRestoredUnitV1({outputs:restoredFiles,directories:dependencyDirectoriesV1(args,unit,cwd),digest:path=>digestFileV1(path,settings.stamps),inner:settings.inner,environment})){event("incomplete",key,0,0,"interface: the restored metadata cannot be rebased onto its dependencies");return false;}
  settle(restoredFiles,entry);
'''+L6
L7='   markExternIdentityV1(outputs,portableKeyV1(keyInput,roots),sensitive,settings.stamps);\n'
L7N=L7+'''   let tables=true;
   if(ifaceKeyed||producer)for(const path of outputs.filter(path=>path.endsWith(".rmeta"))){const written=writeDependenciesV1(path,settings.inner,environment);if(written!==undefined)outputs.push(written);else if(ifaceKeyed)tables=false;}
'''
L8='   if(outputs.length===0||lacking.length>0)entry=undefined;else try{entry=publish();}'
L8N='   if(outputs.length===0||lacking.length>0||!tables)entry=undefined;else try{entry=publish();}'
L9='   else event("publish-skip",key,Date.now()-compiled,0,outputs.length===0?"no-outputs":lacking.length>0?"incomplete: "+lacking.join(","):outputs.some(path=>path.endsWith(".d"))?"refused":"no-dep-info");\n'
L9N='   else event("publish-skip",key,Date.now()-compiled,0,outputs.length===0?"no-outputs":lacking.length>0?"incomplete: "+lacking.join(","):!tables?"interface: the compiler did not list the crate hashes":outputs.some(path=>path.endsWith(".d"))?"refused":"no-dep-info");\n   if(entry)settle(outputs,entry);\n'
EDITS=[(SCRIPT,L1,L1N,1),(SCRIPT,L2,L2N,1),(SCRIPT,L3,L3N,1),(SCRIPT,L4,L4N,1),(SCRIPT,L5,L5N,1),(SCRIPT,L6,L6N,1),(SCRIPT,L7,L7N,1),(SCRIPT,L8,L8N,1),(SCRIPT,L9,L9N,1)]
BASELINE_SHA256="ec3ca8ab442e7e13"
NEWFILES=[(UC+"/🔭️interface/🧬️schema/🔣️.json","n01.json"),(UC+"/🔭️interface/🟦️.ts","n02.ts"),(UC+"/🔭️interface/🧪️tests/🟦️.ts","n03.ts"),(UC+"/🔭️interface/🧪️tests/🔣️.json","n04.json"),(UC+"/🔭️interface/🧪️tests/🐍️.py","n05.py"),(UC+"/🧪️tests/🔭️cutoff/🟦️.ts","n06.ts")]
def apply(text):
    for rel,old,new,count in EDITS:
        if text.count(new)==count and old!=new and new not in old:continue
        n=text.count(old)
        if n!=count:sys.exit("anchor occurs %d times, expected %d: %r"%(n,count,old[:80]))
        text=text.replace(old,new)
    return text
