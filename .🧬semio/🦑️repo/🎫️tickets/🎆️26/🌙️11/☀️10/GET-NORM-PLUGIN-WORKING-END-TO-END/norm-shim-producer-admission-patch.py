import re,sys
for p in sys.argv[1:]:
    s=open(p,encoding="utf-8").read()
    rep={
     "operationProgress":"export function operationProgress(completed, total, owned) { const refused = originalCheck(); if (refused) return refused; return owned > BigInt(originalTurn.input.grant.maximumCapacityBytes) ? 2 : 0; }\n",
     "operationAllocation":"export function operationAllocation(bytes, owned, next, maximum) { const refused = originalCheck(); if (refused) return refused; const capacity = BigInt(originalTurn.input.grant.maximumCapacityBytes); if (maximum !== capacity) return 4; return owned > capacity || next > capacity - owned ? 2 : 0; }\n",
     "operationReserveReturn":"export function operationReserveReturn(kind, count) { const refused = originalCheck(); if (refused) return refused; return count > BigInt(originalTurn.input.grant.maximumItems) ? 2 : 0; }\n",
     "operationFinish":"export function operationFinish(owned) { const refused = originalCheck(); if (refused) return refused; return owned > BigInt(originalTurn.input.grant.maximumCapacityBytes) ? 2 : 0; }\n"}
    for name,new in rep.items():
        pat=re.compile(r"export function "+name+r"\(.*\n")
        assert len(pat.findall(s))==1,(p,name)
        s=pat.sub(lambda m:new,s)
    open(p,"w",encoding="utf-8").write(s)
    print("patched",p)
