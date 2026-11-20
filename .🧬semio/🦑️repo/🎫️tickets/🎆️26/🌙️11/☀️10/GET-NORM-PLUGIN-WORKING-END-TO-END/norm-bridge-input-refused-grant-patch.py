import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a="    try { call = await reactor.admitActorInput(actorInputWire.encode(value), originalContextBudgetWit(turn)); }"
assert s.count(a)==1
s=s.replace(a,"    const wireBytes = actorInputWire.encode(value);\n    try { call = await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn)); }")
b='" disposition=" + call.disposition), { originalCall: call });'
assert s.count(b)==1,"apply norm-bridge-input-refused-text-patch.py first"
s=s.replace(b,'" disposition=" + call.disposition + " wireBytes=" + (wireBytes?.length ?? 0) + " grant=" + JSON.stringify(turn.input.grant, (k, v) => typeof v === "bigint" ? v.toString() : v)), { originalCall: call });')
open(p,"w",encoding="utf-8").write(s)
