import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a="        result = await reactor.poll(originalContextBudgetWit(turn));"
b='''        const slice = originalContextBudgetWit(turn);
        if (typeof budget.wallMs === "number" && budget.wallMs > 0) { const sliceEnd = originalSource.operationClockUs() + BigInt(Math.ceil(budget.wallMs)) * 1000n; if (sliceEnd < slice.deadlineUs) slice.deadlineUs = sliceEnd; }
        if (typeof budget.fuel === "number" && budget.fuel > 0 && BigInt(budget.fuel) < slice.fuel) slice.fuel = BigInt(budget.fuel);
        result = await reactor.poll(slice);'''
assert s.count(a)==1
open(p,"w",encoding="utf-8").write(s.replace(a,b))
