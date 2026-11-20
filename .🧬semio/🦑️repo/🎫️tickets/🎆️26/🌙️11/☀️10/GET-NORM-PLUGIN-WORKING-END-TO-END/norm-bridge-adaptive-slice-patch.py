import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
old='''        const slice = originalContextBudgetWit(turn);
        if (typeof budget.wallMs === "number" && budget.wallMs > 0) { const sliceEnd = originalSource.operationClockUs() + BigInt(Math.ceil(budget.wallMs)) * 1000n; if (sliceEnd < slice.deadlineUs) slice.deadlineUs = sliceEnd; }
        if (typeof budget.fuel === "number" && budget.fuel > 0 && BigInt(budget.fuel) < slice.fuel) slice.fuel = BigInt(budget.fuel);
        result = await reactor.poll(slice);'''
new='''        const slice = originalContextBudgetWit(turn);
        const startedUs = originalSource.operationClockUs();
        if (typeof budget.wallMs === "number" && budget.wallMs > 0) { const wallUs = BigInt(Math.ceil(budget.wallMs)) * 1000n, measuredUs = lastPollUs * 3n, wanted = measuredUs > wallUs ? measuredUs : wallUs, sliceEnd = startedUs + (wanted < 8000000n ? wanted : 8000000n); if (sliceEnd < slice.deadlineUs) slice.deadlineUs = sliceEnd; }
        if (typeof budget.fuel === "number" && budget.fuel > 0 && BigInt(budget.fuel) < slice.fuel) slice.fuel = BigInt(budget.fuel);
        result = await reactor.poll(slice);
        lastPollUs = originalSource.operationClockUs() - startedUs;'''
assert s.count(old)==1
s=s.replace(old,new)
a="  let contextReady = false;"
assert s.count(a)==1
s=s.replace(a,a+"\n  let lastPollUs = 0n;")
open(p,"w",encoding="utf-8").write(s)
