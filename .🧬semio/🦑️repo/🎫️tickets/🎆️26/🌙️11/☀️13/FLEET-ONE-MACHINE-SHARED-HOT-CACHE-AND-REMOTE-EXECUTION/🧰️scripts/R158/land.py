import sys,os,json,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
MIRROR=os.environ.get("R158_MIRROR")
FILES={"cost":B+"/🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts","textsjson":B+"/📖️texts/🔣️.json","nodes":B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs","wanted":B+"/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts","failure":B+"/🏡️resident/🏛️service/🧯️failure/🟨️.mjs","resident":B+"/🏡️resident/🧬️schema/🔣️.json","host":B+"/🏡️resident/🏛️service/🌐️host/🟨️.mjs"}
def loc(k): return FILES[k] if MIRROR is None else MIRROR+"/"+k
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(text,old,new):
    assert text.count(old)==1,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
def cost(t):
    a='export function describeHeavyV1('
    assert t.count(a)==1
    i=t.index('/** 🗒️ Words the heavy projects')
    t=one(t,'import{machinePathV1}from','import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";\nimport{machinePathV1}from')
    i=t.index('/** 🗒️ Words the heavy projects')
    return t[:i]+'/** 🐋️ The refusal of a graph read that stopped an owner too heavy for this machine, built in this one place: the owner(s) with their need (`named`), the cap of one project in megabytes (`limit`) and the number of machines of the fleet that delivered records for the read (`producers`; 0 means nobody can serve it, so a hand-off is doomed). */\nexport const refuseHeavyOwnerV1=(values:Readonly<{named:string;limit:number;producers:number}>,options?:ErrorOptions)=>refuseBuildV1("unserved","graph.owner-too-heavy",values,options);\n\n'+t[i:]
def textsjson(t):
    t=one(t,'nothing else of the read runs ({limit} MB). The inference was stopped','nothing else of the read runs ({limit} MB); {producers} machine(s) of the fleet delivered records for this read. The inference was stopped')
    return one(t,'und sonst nichts vom Lesen läuft ({limit} MB). Die Berechnung','und sonst nichts vom Lesen läuft ({limit} MB); {producers} Rechner der Flotte haben für dieses Lesen Datensätze geliefert. Die Berechnung')
def resident(t):
    t=one(t,'can give one project ({limit} MB). Even alone','can give one project ({limit} MB); {producers} machine(s) of the fleet delivered records for this read. Even alone')
    return one(t,'einem Projekt geben kann ({limit} MB). Auch allein','einem Projekt geben kann ({limit} MB); {producers} Rechner der Flotte haben für dieses Lesen Datensätze geliefert. Auch allein')
def nodes(t):
    t=one(t,'const refuseHeavy=left=>refuseBuildV1("unserved","graph.owner-too-heavy",{named:named(left),','const refuseHeavy=left=>enforcement.cost.refuseHeavyOwnerV1({named:named(left),')
    t=one(t,'throw refuseBuildV1("unserved","graph.owner-too-heavy",{named:enforcement.cost.describeHeavyV1(heavy,','throw enforcement.cost.refuseHeavyOwnerV1({named:enforcement.cost.describeHeavyV1(heavy,')
    t=one(t,'.map(row=>row.limitBytes))/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor)});}','.map(row=>row.limitBytes))/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor),producers:Object.keys(fleetPort?.producers?.()??{}).length});}')
    old='neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }); return names.filter(name => { const row = cost.estimateOwnerCostV1(recorder.table(), name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); if (row.source !== "recorded") return false; return row.saturated === true || (childFootprintMegabytes === undefined ? row.megabytes * 1048576 > here : row.megabytes * cost.ownerCostPolicyV1.footprintFactor > childFootprintMegabytes); }); },'
    new='neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }), table = recorder.table(), estimate = (source, name) => cost.estimateOwnerCostV1(source, name, undefined, undefined, { ceilingMegabytes: context.ceilingMegabytes, nowMs: Date.now() }); return names.filter(name => { if (childFootprintMegabytes !== undefined) { const entry = table.owners[name], stopped = entry === undefined ? [] : entry.samples.filter(sample => sample.stopped); return stopped.length > 0 && estimate({ ...table, owners: { [name]: { ...entry, samples: stopped } } }, name).megabytes * cost.ownerCostPolicyV1.footprintFactor > childFootprintMegabytes; } const row = estimate(table, name); if (row.source !== "recorded") return false; return row.saturated === true || row.megabytes * 1048576 > here; }); },'
    return one(t,old,new)
def wanted(t):
    t=one(t,'import{classifyOwnersV1,describeHeavyV1,','import{classifyOwnersV1,describeHeavyV1,refuseHeavyOwnerV1,')
    return one(t,'throw refuseBuildV1("unserved","graph.owner-too-heavy",{named,limit:soloLimit});','throw refuseHeavyOwnerV1({named,limit:soloLimit,producers:0});')
def failure(t):
    i=t.index('export function heavyRefusalOfV1(error,depth=4){')
    j=t.index('/** 🏷️ The text a failure is logged by',i)
    new='''export function heavyRefusalOfV1(error,depth=4){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error.key==="build.graph.owner-too-heavy"||/unserved build\\.graph\\.owner-too-heavy \\{/u.test(String(error.message??""))){
  let values=error.values;
  if(typeof values!=="object"||values===null){try{values=JSON.parse(/build\\.graph\\.owner-too-heavy (\\{.*\\})\\s*$/su.exec(String(error.message))?.[1]??"{}");}catch{values={};}}
  if(typeof values?.named==="string"&&values.named!==""&&values.limit!==undefined&&values.producers!==undefined)return{named:values.named,limit:String(values.limit),producers:String(values.producers)};
 }
 return heavyRefusalOfV1(error.cause,depth-1);
}

'''
    t=t[:i]+new+t[j:]
    return t
def host(t):
    return one(t,'governed=(ports.workers?.active?.()??0)>0;\n  if(!governed&&rss>policy.maximumResidentBytes){','governed=(ports.workers?.active?.()??0)>0||governor.inFlight>0||[...connections].some(connection=>connection.requests.size>0);\n  if(!governed&&rss>policy.maximumResidentBytes){')
PATCH={"host":host,"cost":cost,"textsjson":textsjson,"resident":resident,"nodes":nodes,"wanted":wanted,"failure":failure}
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    os.makedirs(backup,exist_ok=True);manifest={};new={}
    for k in FILES: new[k]=PATCH[k](read(loc(k)))
    for i,k in enumerate(FILES):
        old=read(loc(k));write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
    json.dump(manifest,open(backup+"/manifest.json","w"))
    for k in FILES: write(loc(k),new[k])
    print("applied")
elif cmd=="restore":
    manifest=json.load(open(backup+"/manifest.json"));bad=[]
    for k in FILES:
        m=manifest[k];now=sha(read(loc(k)))
        if now==m["after"]: write(loc(k),read(backup+"/%d.bak"%m["i"]))
        elif now!=m["before"]: bad.append(k)
    print("CONFLICT" if bad else "restored",bad)
    sys.exit(4 if bad else 0)
