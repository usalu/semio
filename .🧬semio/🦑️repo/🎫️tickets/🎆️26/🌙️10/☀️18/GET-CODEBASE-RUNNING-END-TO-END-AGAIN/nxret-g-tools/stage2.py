"""Second increment of the nxret-g change, applied on top of the live state of stage.py (anchors are the live text): the per-run limit counts actual retirements instead of cutting the candidate list, so generations the retirement keeps (unauthenticated, current, held) never starve the younger ones behind them. Usage: python3 stage2.py check|apply <root>; every file is replaced by atomic rename, the first original is kept in `backup2/`."""
import hashlib,json,os,shutil,sys

HERE=os.path.dirname(os.path.abspath(__file__))
BACKUP=os.path.join(HERE,"backup2")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R=f"{L}/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention"
PHYSICAL=f"{R}/📥️physical/🟦️.ts"
PHYSICAL_SCHEMA=f"{R}/📥️physical/🧬️schema/🔣️.json"
COMMAND=f"{L}/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
LEDGER=f"{L}/⚡️caching/💾️storage/🧾️ledger"

PHYSICAL_PATCHES=[
 ("export type ToolingRetirementSelectionV1=Readonly<{identities:readonly string[];progress?:(identity:string,index:number,total:number)=>void|Promise<void>}>;",
  "export type ToolingRetirementSelectionV1=Readonly<{identities:readonly string[];limit?:number;progress?:(identity:string,index:number,total:number)=>void|Promise<void>}>;"),
 ("let reached=0;for(const identity of candidates){checkResourceLeaseControl(control);await selection?.progress?.(identity,reached++,candidates.length);",
  "let reached=0;for(const identity of candidates){checkResourceLeaseControl(control);if(selection?.limit!==undefined&&retired.length>=selection.limit)break;await selection?.progress?.(identity,reached++,candidates.length);"),
 ('Readonly<{version:1;mode:"count-bound";identities:readonly string[]}>,progress?:',
  'Readonly<{version:1;mode:"count-bound";identities:readonly string[];limit?:number}>,progress?:'),
 ('{identities:request.identities,...progress?{progress}:{}}',
  '{identities:request.identities,...request.limit!==undefined?{limit:request.limit}:{},...progress?{progress}:{}}'),
]

COMMAND_PATCHES=[
 ('const retire=async(identities:readonly string[],onIdentity:(identity:string,index:number,total:number)=>Promise<void>)=>{',
  'const retire=async(identities:readonly string[],limit:number,onIdentity:(identity:string,index:number,total:number)=>Promise<void>)=>{'),
 ('request={version:1 as const,mode:"count-bound" as const,identities},',
  'request={version:1 as const,mode:"count-bound" as const,identities,limit},'),
]

def schema_patch(text):
    document=json.loads(text)
    properties=document["definitions"]["CountBoundRequest"]["properties"]
    if "limit" in properties:return text
    properties["limit"]={"type":"integer","minimum":1,"maximum":1024}
    return json.dumps(document,indent=2,ensure_ascii=False)+"\n"

def patch_text(text,patches):
    for old,new in patches:
        if new!=old and text.count(new)==1:continue
        if text.count(old)!=1:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
        text=text.replace(old,new)
    return text

LEDGER_PATCHES=[
 ('return cacheTextV1(language,"reason."+reason,{days:Math.round((unusedAgeMs??0)/86400000)});',
  'if(reason==="age"&&unusedAgeMs!==null&&unusedAgeMs<86400000)return cacheTextV1(language,"reason.ageHours",{hours:Math.max(1,Math.round(unusedAgeMs/3600000))});\n return cacheTextV1(language,"reason."+reason,{days:Math.round((unusedAgeMs??0)/86400000)});'),
]

def texts_patch(text):
    if '"reason.ageHours"' in text:return text
    anchor='    "reason.age": { "en": "unused for {days} days", "de": "seit {days} Tagen unbenutzt" },\n'
    if text.count(anchor)!=1:raise SystemExit("reason.age anchor")
    return text.replace(anchor,anchor+'    "reason.ageHours": { "en": "unused for {hours} hours", "de": "seit {hours} Stunden unbenutzt" },\n')

TARGETS=[(f"{LEDGER}/🟦️.ts",lambda text:patch_text(text,LEDGER_PATCHES)),(f"{LEDGER}/📖️texts/🔣️.json",texts_patch),(PHYSICAL,lambda text:patch_text(text,PHYSICAL_PATCHES)),(PHYSICAL_SCHEMA,schema_patch),(COMMAND,lambda text:patch_text(text,COMMAND_PATCHES))]

def remember(root,rel):
    os.makedirs(BACKUP,exist_ok=True)
    name=hashlib.sha1(rel.encode()).hexdigest()[:16]
    manifest=os.path.join(BACKUP,"manifest.tsv")
    known=set(line.split("\t")[0] for line in open(manifest,encoding="utf8")) if os.path.exists(manifest) else set()
    if name in known:return
    shutil.copy2(os.path.join(root,rel),os.path.join(BACKUP,name+".bak"))
    with open(manifest,"a",encoding="utf8") as handle:handle.write(f"{name}\t{rel}\n")

def main():
    mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
    produced=[]
    for rel,transform in TARGETS:
        with open(os.path.join(root,rel),encoding="utf8") as handle:produced.append((rel,transform(handle.read())))
    if mode=="check":
        for rel,_ in produced:print("ok",rel[-60:])
        return
    for rel,text in produced:
        remember(root,rel)
        temporary=os.path.join(root,rel)+".nxret-apply"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.replace(temporary,os.path.join(root,rel));print("applied",rel[-60:])

main()
