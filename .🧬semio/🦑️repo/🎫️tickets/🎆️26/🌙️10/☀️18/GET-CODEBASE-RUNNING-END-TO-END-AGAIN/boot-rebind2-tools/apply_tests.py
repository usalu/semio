import json,os,re,shutil,sys
sys.path.insert(0,os.path.dirname(__file__))
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BK=C+"/🗑️generated/boot-rebind2/backup"
L=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
O=L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening"
RESET_TEST=O+"/♻️reset/🧪️tests/🟦️.ts"
RECOVERY_TEST=O+"/♻️reset/⚡️recovery/🧪️tests/🟦️.ts"
RECOVERY_FIXTURE=O+"/♻️reset/⚡️recovery/🧪️tests/🔣️.json"
STAMP_TEST=L+"/🏗️build/🧷️format-stamps/🧪️tests/🟦️.ts"
write="--write" in sys.argv
RENAMES=[
("planInactiveNxCacheResetV1","planInactiveCacheResetV1"),
("requireInactiveNxKernelAbsenceV1","requireInactiveKernelAbsenceV1"),
("requireNxKernelAbsenceV1","requireKernelAbsenceV1"),
("inspectNxResetCapsuleV1","inspectResetCapsuleV1"),
("planInactiveNxResetResumeV1","planInactiveResetResumeV1"),
("readNxResetStateV1","readResetStateV1"),
("sameNxResetStateV1","sameResetStateV1"),
("publishNxResetProgressV1","publishResetProgressV1"),
("requireNxResetExpectedV1","requireResetExpectedV1"),
("resetInactiveNxCacheControlV1","resetInactiveCachePartitionControlV1"),
("readInactiveNxPublicationDataV1","readInactivePublicationDataV1"),
("observeInactiveNxPublicationV1","observeInactivePublicationV1"),
("resetInactiveNxCacheWithinBudgetV1","resetInactiveCachePartitionWithinBudgetV1"),
("InactiveNxResetOutcomeV1","InactivePartitionResetOutcomeV1"),
("InactiveNxRecoveryReasonV1","InactiveRecoveryReasonV1"),
("InactiveNxRecoveryPortsV1","InactiveRecoveryPortsV1"),
("withInactiveNxPublicationRecoveryV1","withInactivePublicationRecoveryV1"),
("actualNxPreflightV1","actualPreflightV1"),
("inactive-nx-reset-intent","inactive-reset-intent"),
("inactive-nx-reset-planned","inactive-reset-planned"),
]
def rename(text):
    for old,new in RENAMES: text=re.sub(r"(?<![A-Za-z0-9-])"+re.escape(old)+r"(?![A-Za-z0-9-])",new,text)
    return text
def once(text,old,new,count=1):
    if text.count(old)!=count: sys.exit(f"anchor count {text.count(old)} (want {count}) for {old[:90]!r}")
    return text.replace(old,new)
def edit(path,transform):
    text=open(path).read();new=transform(text)
    if new==text: print("unchanged",path[-50:]);return
    if write:
        os.makedirs(BK,exist_ok=True);b=BK+"/"+path[len(ROOT)+1:].replace("/","__")
        if not os.path.exists(b): shutil.copyfile(path,b)
        open(path,"w").write(new);print("patched",path[-50:])
    else: print("would patch",path[-50:])

def reset_test(t):
    t=rename(t)
    t=once(t,'import fixture from "./🔣️.json" with {type:"json"};','import fixture from "./🔣️.json" with {type:"json"};\nimport budgetPolicy from "../../../🔣️policy.json" with {type:"json"};')
    t=once(t,'fixture.finalRemovals.map(row=>row.id)]','fixture.finalRemovals.map(row=>row.id),"leases","ledger","ledger-residue",...Object.keys(budgetPolicy.partitions).map(id=>"partition:"+id)]')
    t=once(t,'if(kind==="invalid-intent")await writeFile(join(directory,".reset.json"),"{}");','if(kind==="invalid-intent")await writeFile(join(directory,".reset.json"),"{}");if(kind==="ledger"||kind==="ledger-residue"){const ledger=join(directory,"boot-recovery");await mkdir(ledger);await writeFile(join(ledger,"a.jsonl"),"{}\\n");await writeFile(join(ledger,"a.jsonl.stamp"),"");if(kind==="ledger-residue")await mkdir(join(ledger,"foreign"));}')
    t=once(t,'for(const kind of[','const partitionOf=(kind:string)=>kind.startsWith("partition:")?kind.slice(10):kind==="leases"?"leases":"nx",retiring=(kind:string)=>["success","final-post-remove","leases","ledger"].includes(kind)||kind.startsWith("partition:");\n for(const kind of[')
    t=once(t,'return{partitions:{nx:{directory,capacityBytes:2000,filesystemBytes:2000}}};','return{partitions:{[partitionOf(kind)]:{directory,capacityBytes:2000,filesystemBytes:2000}},controlDirectory:join(root,"c")};')
    t=once(t,'expect(options.resource).toBe(','expect(options.directory).toBe(partitionOf(kind)==="leases"?join(root,"c"):root);expect(options.resource).toBe(')
    t=once(t,'ports={createHash,','ports={createHash,randomBytes,formatStampLanguagesV1:()=>["en","de"],')
    t=once(t,'return resetInactiveCachePartitionControlV1(control);','return resetInactiveCachePartitionControlV1(partitionOf(kind),control);')
    t=once(t,'actualOwner.core(control,','actualOwner.core(partitionOf(kind),control,',2)
    t=once(t,'actualOwner.probe(process.cwd(),control)','actualOwner.probe(process.cwd(),partitionOf(kind),control)',2)
    t=once(t,'if(kind==="success"||kind==="final-post-remove"||kind==="absent"||kind==="empty")','if(retiring(kind)||kind==="absent"||kind==="empty")')
    t=once(t,'toBe(kind==="success"||kind==="final-post-remove"?2:0)','toBe(retiring(kind)?2:0)')
    t=once(t,'if(kind==="success"||kind==="final-post-remove"){','if(retiring(kind)){expect(result.partition).toBe(partitionOf(kind));if(kind==="ledger")expect(events).toContain("cache:reset-publication-nx:ledger-swept");')
    t=once(t,'if(kind!=="unsupported")expect(closed)','if(kind==="leases")expect(opens).toBe(0);if(kind==="leases")expect(closed).toBe(0);else if(kind!=="unsupported")expect(closed)')
    t+='''
test("reset partitions equal the authored cache budget partitions and the obsolete-publication notice exists in every declared language",()=>{
 const db=new Database(":memory:");try{const authored=Object.keys(budgetPolicy.partitions).sort(),declared=[...schema.$defs.Partition.enum].sort();expect(declared).toEqual(authored);const oracle=db.query("select group_concat(key) names from (select key from json_each(?1) order by key)").get(JSON.stringify(budgetPolicy.partitions))as{names:string};expect(oracle.names.split(",")).toEqual(declared);const texts=schema.$defs.InactivePublicationProtocol.const.texts as Record<string,{obsolete:string}>;expect(Object.keys(texts).sort()).toEqual(["de","en"]);for(const language of Object.keys(texts)){expect(texts[language]!.obsolete).toContain("{partition}");expect(texts[language]!.obsolete).toContain("{directory}");}}finally{db.close();}
});
'''
    return t

def recovery_test(t):
    t=rename(t)
    t=once(t,'import fixture from "./🔣️.json" with {type:"json"};','import fixture from "./🔣️.json" with {type:"json"};\nimport budgetPolicy from "../../../../🔣️policy.json" with {type:"json"};')
    t=once(t,'async function actualPreflightV1(workspace:string,directory:string){','async function actualPreflightV1(workspace:string,directories:Readonly<Record<string,string>>){')
    t=once(t,'return{partitions:{nx:{directory}}};},\n','return{partitions:Object.fromEntries(Object.entries(directories).map(([id,directory])=>[id,{directory}]))};},\n') if 'return{partitions:{nx:{directory}}};},\n' in t else once(t,'return{partitions:{nx:{directory}}};},','return{partitions:Object.fromEntries(Object.entries(directories).map(([id,directory])=>[id,{directory}]))};},')
    t=once(t,'inspect=await actualPreflightV1(workspace,directory)','inspect=await actualPreflightV1(workspace,{nx:directory})',3)
    t=once(t,'inspect(workspace,control)','inspect(workspace,"nx",control)',2)
    t=once(t,'const data={version:1,partition:"nx",attempts:1','const data={version:1,attempts:1')
    return t
edit(RESET_TEST,reset_test)
edit(RECOVERY_TEST,recovery_test)
