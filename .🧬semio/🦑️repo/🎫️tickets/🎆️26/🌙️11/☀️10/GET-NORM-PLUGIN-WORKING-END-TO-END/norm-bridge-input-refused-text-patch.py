import sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
a='throw Object.assign(new Error("original-turn.input-refused"), { originalCall: call });'
b='throw Object.assign(new Error("original-turn.input-refused: refusal=" + JSON.stringify(optionValue(call.refusal) ?? null) + " ready=" + call.ready + " disposition=" + call.disposition), { originalCall: call });'
assert s.count(a)==1
open(p,"w",encoding="utf-8").write(s.replace(a,b))
