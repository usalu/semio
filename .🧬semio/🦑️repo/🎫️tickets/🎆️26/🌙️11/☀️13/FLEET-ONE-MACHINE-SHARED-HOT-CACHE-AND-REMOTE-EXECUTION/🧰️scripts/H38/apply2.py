import os,sys
DRY="--dry" in sys.argv
S="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/"
out=[]
def chain(path,steps):
    t=open(path,encoding="utf8").read()
    for old,new,n in steps:
        if t.count(old)!=n: sys.exit("anchor %d!=%d in %s: %s"%(t.count(old),n,path,old[:70]))
        t=t.replace(old,new)
    out.append((path,t))
chain(S+"🟨️.mjs",[
("async function evictOptionalGraphLruV1(workspace,root,own,namespace,control){\n",
 """/** ⏱️ The cancellation of one inline eviction: it ends with the lease control or after the declared budget, so a reader that opens the graph store never waits minutes for a cleanup; what it retired stays retired and the next writer goes on (the ledger declines records only while it is full). */
export function optionalEvictionCancelV1(control,{budgetMilliseconds=quotaSchema.definitions.Protocol.const.inlineEvictionMilliseconds,now=Date.now}={}){const deadline=now()+budgetMilliseconds;return()=>control.signal.aborted||now()>deadline;}
async function evictOptionalGraphLruV1(workspace,root,own,namespace,control){\n const cancelled=optionalEvictionCancelV1(control);\n""",1),
("cancelled:()=>control.signal.aborted","cancelled",2),
])
chain(S+"🟨️.d.mts",[("export function withRepositoryBuildGraphSelectionV1<T>(","/** ⏱️ The cancellation of one inline optional-cache eviction: the lease control or the declared budget. */\nexport function optionalEvictionCancelV1(control:{readonly signal:AbortSignal},options?:{budgetMilliseconds?:number;now?:()=>number}):()=>boolean;\nexport function withRepositoryBuildGraphSelectionV1<T>(",1)])
chain(S+"🧬️schema/🔣️.json",[('            "retirementBatchEntries": 64\n','            "retirementBatchEntries": 64,\n            "inlineEvictionMilliseconds": 5000\n',1)])
if DRY: print("dry ok");sys.exit()
for p,t in out:
    open(p+".h38tmp","w",encoding="utf8").write(t);os.replace(p+".h38tmp",p)
print("landed")
