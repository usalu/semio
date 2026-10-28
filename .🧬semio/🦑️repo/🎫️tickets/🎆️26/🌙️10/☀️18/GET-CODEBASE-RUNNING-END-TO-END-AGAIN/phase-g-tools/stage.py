"""🧩️ The staged change of phase-g: new files and anchored patches, applied to a root (the overlay, later the live tree) in one pass. Every patch asserts its anchor exists exactly once (or is already applied), so a live edit of a peer between staging and applying is noticed, never overwritten. Usage: python3 stage.py apply <root> | revert <root> (revert restores the files `apply` saved in `backup/` when it ran on the live tree)"""
import json,os,re,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
STAGED=os.path.join(HERE,"staged")
F="🧰️framework"
PROCESS=f"{F}/🔨️modules/🏃️process"
HC=f"{PROCESS}/🏗️build/🌡️host-capacity"
LIB=f"{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
LHC=f"{LIB}/🌡️host-capacity"
NEW={
 "phases.ts":f"{HC}/🪜️phases/🟦️.ts",
 "phases.test.ts":f"{HC}/🧪️tests/🪜️phases/🟦️.ts",
 "phases.oracle.py":f"{HC}/🧪️tests/🪜️phases/🐍️oracle/🐍️.py",
 "phases.fixture.json":f"{HC}/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json",
 "phases.gate.test.ts":f"{HC}/🧪️tests/🪜️phases/🚪️gate/🟦️.ts",
 "phases.library.test.ts":f"{LHC}/🧪️tests/🪜️phases/🟦️.ts",
 "marks.ts":f"{LHC}/🚏️marks/🟦️.ts",
}
POLICY_FIELDS={"phaseReferenceMegabytes":1024,"rebuildPriorMegabytes":4096,"rebuildShareBasisPoints":5000,"maximumRebuildSlots":4}
MESSAGE_PHASED={"en":"[capacity] {operation}: sized at {percent}% of the default from {samples} earlier run(s) of its class, at least {floor} MB because of its {phase} phase","de":"[capacity] {operation}: auf {percent}% des Standardwerts bemessen, aus {samples} frueheren Laeufen seiner Klasse, mindestens {floor} MB wegen der Phase {phase}"}

PENDING={}

def read(root,rel):
    if rel in PENDING:return PENDING[rel]
    with open(os.path.join(root,rel),encoding="utf8") as handle:return handle.read()

BACKUP=os.path.join(HERE,"backup")

def remember(root,rel):
    """Keeps the first version of every file this change touches in the tree (flat names, a manifest line each), so `revert` can give it back."""
    if os.path.realpath(root)!=os.path.realpath("/Users/ueli/Documents/semio"):return
    import hashlib
    os.makedirs(BACKUP,exist_ok=True)
    name=hashlib.sha1(rel.encode()).hexdigest()[:16]
    manifest=os.path.join(BACKUP,"manifest.tsv")
    known=set(line.split("\t")[0] for line in open(manifest,encoding="utf8")) if os.path.exists(manifest) else set()
    if name in known:return
    path=os.path.join(root,rel)
    if os.path.exists(path):shutil.copy2(path,os.path.join(BACKUP,name+".bak"))
    with open(manifest,"a",encoding="utf8") as handle:handle.write(f"{name}\t{'existing' if os.path.exists(path) else 'new'}\t{rel}\n")

def revert(root):
    manifest=os.path.join(BACKUP,"manifest.tsv")
    for line in reversed(open(manifest,encoding="utf8").read().splitlines()):
        name,kind,rel=line.split("\t");path=os.path.join(root,rel)
        if kind=="new":
            if os.path.exists(path):os.remove(path)
        else:shutil.copy2(os.path.join(BACKUP,name+".bak"),path)
        print("reverted",rel)

def write(root,rel,text):
    PENDING[rel]=text

def flush(root):
    """Writes every pending file only after ALL anchors of ALL patches matched: a peer edit near an anchor stops the batch before the first byte is written."""
    for rel,text in PENDING.items():
        remember(root,rel)
        path=os.path.join(root,rel);os.makedirs(os.path.dirname(path),exist_ok=True)
        temporary=path+".phase-g.tmp"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.replace(temporary,path)
    PENDING.clear()

def replace(root,rel,old,new,done=None):
    text=read(root,rel)
    if (done if done is not None else new) in text:return "kept"
    if text.count(old)!=1:raise SystemExit(f"anchor not unique ({text.count(old)}) in {rel}: {old[:90]}")
    write(root,rel,text.replace(old,new));return "patched"

def regex_all(root,rel,pattern,template,marker):
    text=read(root,rel)
    if marker in text:return "kept"
    fresh,count=re.subn(pattern,template,text)
    if count==0:raise SystemExit(f"pattern missing in {rel}: {pattern}")
    write(root,rel,fresh);return f"patched x{count}"

def schema(root):
    rel=f"{HC}/🧬️schema/🔣️.json";data=json.loads(read(root,rel));definitions=data["definitions"];touched=False
    def put(container,key,value):
        nonlocal touched
        if container.get(key)!=value:container[key]=value;touched=True
    constants=definitions["Protocol"]["const"]
    put(constants,"marksEnvironment","SEMIO_HOST_CAPACITY_PHASE_MARKS");put(constants,"phasesSuffix",".phases");put(constants,"maximumPhaseTransitions",64);put(constants,"maximumPhaseMarkBytes",4096)
    put(definitions,"Phase",{"enum":["assembly","plan","rebuild","execution"]})
    put(definitions["Grant"]["properties"],"phase",{"$ref":"#/definitions/Phase"})
    estimates=definitions["Estimates"]
    for name,value in POLICY_FIELDS.items():
        put(estimates["properties"],name,{"type":"integer","minimum":1,**({"maximum":10000} if name.endswith("BasisPoints") else {"maximum":64} if name.startswith("maximum") else {"maximum":1048576})})
        if name not in estimates["required"]:estimates["required"].append(name);touched=True
    for language in ("en","de"):put(definitions["Messages"]["const"][language],"phased",MESSAGE_PHASED[language])
    message=definitions["Message"]
    put(message["properties"],"phased",{"type":"string"})
    if "phased" not in message["required"]:message["required"].append("phased");touched=True
    if touched:write(root,rel,json.dumps(data,ensure_ascii=False,indent=2)+"\n")
    return "patched" if touched else "kept"

def policy_fields(root):
    report=[]
    insertion=",".join(f'"{name}":{value}' for name,value in POLICY_FIELDS.items())
    for rel in (f"{HC}/🧪️tests/🧫️fixtures/🔣️.json",f"{HC}/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json"):
        report.append((rel,regex_all(root,rel,r'("maximumRampMilliseconds":\d+)',r"\1,"+insertion,'"phaseReferenceMegabytes"')))
    oracle=f"{HC}/🧪️tests/📐️estimates/🐍️oracle/🐍️.py"
    report.append((oracle,regex_all(root,oracle,r'("maximumRampMilliseconds": \d+)',r"\1, "+", ".join(f'"{name}": {value}' for name,value in POLICY_FIELDS.items()),'"phaseReferenceMegabytes"')))
    policy=f"{LIB}/🔣️policy.json"
    text=read(root,policy)
    if '"phaseReferenceMegabytes"' not in text:
        fresh,count=re.subn(r'(\n(\s+)"maximumRampMilliseconds": \d+)(\n)',lambda match:match.group(1)+"".join(f',\n{match.group(2)}"{name}": {value}' for name,value in POLICY_FIELDS.items())+match.group(3),text)
        if count!=1:raise SystemExit("build policy estimates section not found once")
        write(root,policy,fresh);report.append((policy,"patched"))
    else:report.append((policy,"kept"))
    return report

def estimates_module(root):
    rel=f"{HC}/📐️estimates/🟦️.ts";report=[]
    report.append(replace(root,rel,"/** 🧮️ Estimates a request from what its class learned:","export {weights as hostSampleWeightsV1,quantile as hostSampleQuantileV1};\n\n/** 🧮️ Estimates a request from what its class learned:",done="export {weights as hostSampleWeightsV1"))
    report.append(replace(root,rel,"never under `minimumMegabytes` in total","never under `minimumMegabytes` (or the constant `floorMegabytes` the learned assembly and plan phases need, which does not scale with the jobs) in total"))
    report.append(replace(root,rel,"ratioBasisPoints:number,capacityMegabytes:number):HostRequestV1{\n const rules=policy.estimates,perJob","ratioBasisPoints:number,capacityMegabytes:number,floorMegabytes:number=0):HostRequestV1{\n const rules=policy.estimates,perJob"))
    report.append(replace(root,rel,"fixed+=Math.max(0,rules.minimumMegabytes-(fixed+request.minimumJobs*perJob));","fixed+=Math.max(0,Math.max(rules.minimumMegabytes,floorMegabytes)-(fixed+request.minimumJobs*perJob));"))
    return report

def decision_module(root):
    rel=f"{HC}/📏️decision/🟦️.ts";report=[]
    report.append(replace(root,rel,"defaultRampMilliseconds:number;maximumRampMilliseconds:number}>;","defaultRampMilliseconds:number;maximumRampMilliseconds:number;phaseReferenceMegabytes:number;rebuildPriorMegabytes:number;rebuildShareBasisPoints:number;maximumRebuildSlots:number}>;"))
    report.append(replace(root,rel,"handover?:HostHandoverV1;rampMilliseconds?:number;rampSlot?:boolean}>;","handover?:HostHandoverV1;rampMilliseconds?:number;rampSlot?:boolean;phase?:\"assembly\"|\"plan\"|\"rebuild\"|\"execution\"}>;"))
    report.append(replace(root,rel,'name:"waiting"|"admitted"|"estimated"|','name:"waiting"|"admitted"|"estimated"|"phased"|'))
    report.append(replace(root,rel,"`Grant` of the schema); `observedMegabytes`","`Grant` of the schema); `phase` is the phase the run is in once it reports phases; `observedMegabytes`"))
    return report

def index_module(root):
    rel=f"{HC}/🟦️.ts"
    return [replace(root,rel,'export * from "./📐️estimates/🟦️.ts";\n','export * from "./📐️estimates/🟦️.ts";\nexport * from "./🪜️phases/🟦️.ts";\n')]

def install_new(root):
    for name,rel in NEW.items():
        source=os.path.join(STAGED,name)
        if os.path.exists(source):
            with open(source,encoding="utf8") as handle:PENDING[rel]=handle.read()

try:
    import patches_gate
except ImportError:
    patches_gate=None
try:
    import patches_library
except ImportError:
    patches_library=None

def apply(root):
    install_new(root)
    report=[("estimates",x) for x in estimates_module(root)]
    report+=[("schema",schema(root))]
    report+=policy_fields(root)
    report+=[("decision",x) for x in decision_module(root)]
    if patches_gate:report+=[("gate",x) for x in patches_gate.apply(root,replace,read,write)]
    report+=[("index",x) for x in index_module(root)]
    if patches_library:report+=[("library",x) for x in patches_library.apply(root,replace,read,write,callsites=os.environ.get("PHASE_G_CALLSITES","1")=="1")]
    for line in report:print(*line if isinstance(line,tuple) else (line,))
    flush(root);print("applied",len(report),"steps")

if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="apply":apply(sys.argv[2])
    elif len(sys.argv)==3 and sys.argv[1]=="check":
        apply.__globals__["flush"]=lambda root:PENDING.clear();apply(sys.argv[2]);print("check only: nothing written")
    elif len(sys.argv)==3 and sys.argv[1]=="revert":revert(sys.argv[2])
    else:raise SystemExit(__doc__)
