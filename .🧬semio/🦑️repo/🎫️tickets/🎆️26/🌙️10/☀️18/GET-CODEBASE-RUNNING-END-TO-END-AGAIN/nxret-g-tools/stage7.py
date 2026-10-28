"""Seventh increment of the nxret-g change: a delegate learns whether its area was named with `--area` (`selected`); the Nx toolkit store waits for an idle tools image only then, a full `cache gc` asks once and moves on. Usage: python3 stage7.py check|apply <root>."""
import hashlib,os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
BACKUP=os.path.join(HERE,"backup7")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧹️cache-gc"
RUNNER=f"{L}/🏃️runner/🟦️.ts"
COMMAND=f"{L}/⌨️command/🟦️.ts"
RUNNER_PATCHES=[
 ('context?:Readonly<{machine:MachinePlanV1|null}>):Promise<string|undefined>;','context?:Readonly<{machine:MachinePlanV1|null;selected:boolean}>):Promise<string|undefined>;'),
 ('ports.delegate(row.owner,row.id,request.dryRun,output.language,{machine})','ports.delegate(row.owner,row.id,request.dryRun,output.language,{machine,selected:request.only!==undefined})'),
]
COMMAND_PATCHES=[
 ('machine:MachinePlanV1|null):Promise<string>{\n const{runNxToolingStoreBoundV1','machine:MachinePlanV1|null,selected:boolean):Promise<string>{\n const{runNxToolingStoreBoundV1'),
 ('policy=resolveNxToolingStoreBoundPolicyV1(process.env),mount=','policy=selected?resolveNxToolingStoreBoundPolicyV1(process.env):{...resolveNxToolingStoreBoundPolicyV1({}),idleWaitMs:0},mount='),
 ('nxToolingReclaim(root,dryRun,language,phaseControl,invocation,context?.machine??null)','nxToolingReclaim(root,dryRun,language,phaseControl,invocation,context?.machine??null,context?.selected??false)'),
]
def patch(patches):
    def apply(text):
        for old,new in patches:
            if new!=old and text.count(new)==1:continue
            if text.count(old)!=1:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
            text=text.replace(old,new)
        return text
    return apply
TARGETS=[(RUNNER,patch(RUNNER_PATCHES)),(COMMAND,patch(COMMAND_PATCHES))]
mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
produced=[]
for rel,transform in TARGETS:
    with open(os.path.join(root,rel),encoding="utf8") as handle:produced.append((rel,transform(handle.read())))
if mode=="check":print("ok");sys.exit(0)
os.makedirs(BACKUP,exist_ok=True)
for rel,text in produced:
    name=hashlib.sha1(rel.encode()).hexdigest()[:16]
    if not os.path.exists(os.path.join(BACKUP,name+".bak")):shutil.copy2(os.path.join(root,rel),os.path.join(BACKUP,name+".bak"))
    temporary=os.path.join(root,rel)+".nxret-apply"
    with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
    os.replace(temporary,os.path.join(root,rel));print("applied",rel[-50:])
