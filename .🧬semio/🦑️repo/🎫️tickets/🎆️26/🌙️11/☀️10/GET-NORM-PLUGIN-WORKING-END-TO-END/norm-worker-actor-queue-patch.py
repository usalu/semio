import re,sys
p=sys.argv[1]
s=open(p,encoding="utf-8").read()
chk=re.compile(r"( +)if \(inFlightTurnActors\.has\(actorId\)\) throw new Error\(`shard worker: actor \$\{actorId\} already has a turn in flight`\);\n +inFlightTurnActors\.add\(actorId\);\n")
assert len(chk.findall(s))==3,len(chk.findall(s))
s=chk.sub(lambda m: m.group(1)+"const leaveTurn = await enterActorTurn(actorId);\n"+m.group(1)+"if (actors.get(actorId) !== actor) { leaveTurn(); throw new Error(\"actor-lifecycle.activation-mismatch\"); }\n",s)
fin=re.compile(r"( +)inFlightTurnActors\.delete\(actorId\);")
assert len(fin.findall(s))==3
s=fin.sub(r"\1leaveTurn();",s)
a="const inFlightTurnActors = new Set();\n"
assert s.count(a)==1
helper=a+'''const actorTurnTails = new Map();
async function enterActorTurn(actorId) {
  const previous = actorTurnTails.get(actorId) ?? Promise.resolve();
  let leave;
  const mine = new Promise((resolve) => { leave = resolve; });
  const tail = previous.then(() => mine);
  actorTurnTails.set(actorId, tail);
  await previous;
  inFlightTurnActors.add(actorId);
  return () => { inFlightTurnActors.delete(actorId); leave(); if (actorTurnTails.get(actorId) === tail) actorTurnTails.delete(actorId); };
}
'''
open(p,"w",encoding="utf-8").write(s.replace(a,helper))
