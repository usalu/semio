import sys,os,re
p=sys.argv[1]
s=open(p,encoding='utf8').read().split("\n")
blk=s[9:20]
assert blk[0].startswith("type ActorInstanceLifetime") and blk[-1].startswith("type Vitest"), (blk[0],blk[-1])
names=[re.match(r"type (\w+)",l).group(1) for l in blk]
new="/** 📐️ The type aliases the GIS document open tests share across their phases: the framework types they reach through dynamic module paths. */\n"+"\n".join("export "+l for l in blk)+"\n"
d=os.path.join(os.path.dirname(p),"📐️document-open-types")
os.makedirs(d,exist_ok=True)
rest=s[:9]+s[20:]
lines=rest
last=max(i for i,l in enumerate(lines) if l.startswith("import "))
lines.insert(last+1,'import type { '+", ".join(names)+' } from "./📐️document-open-types/🟦️.ts";')
s="\n".join(lines)
m=re.search(r"const probe: (\{.*?\}) = \{ renderWakes", s)
inline=m.group(1)
K="ServiceCompositionTestDependenciesV1"
inline2=inline.replace('InstanceType<typeof VerifiedColdDocumentPair>',f'InstanceType<{K}["VerifiedColdDocumentPair"]>').replace('Awaited<ReturnType<typeof reserveDocumentBrowserActorChild>>',f'Awaited<ReturnType<{K}["reserveDocumentBrowserActorChild"]>>')
t=new.replace("\n","\nimport type { ServiceCompositionTestDependenciesV1 } from \"../../🧰️dependencies/🟦️.ts\";\n",1)
t=t.rstrip("\n")+f"\n\n/** 🧮️ The mutable observations of the cold pair transfer, shared by its fake worker and its phases. */\nexport type ColdPairProbe = {inline2};\n"
open(os.path.join(d,"🟦️.ts"),"w",encoding="utf8").write(t)
s=s.replace(f"const probe: {inline} =","const probe: ColdPairProbe =")
s=s.replace('import type { ActorInstanceLifetime,','import type { ColdPairProbe, ActorInstanceLifetime,',1)
lines=s.split("\n")
ci=next(i for i,l in enumerate(lines) if l.startswith("      class ColdPairWorker {"))
ce=next(i for i in range(ci,len(lines)) if lines[i]=="      }")
wi=next(i for i,l in enumerate(lines) if l.startswith("      (globalThis as unknown as { Worker: unknown }).Worker = ColdPairWorker;"))
cls=lines[ci:ce+1]
lines=lines[:ci]+lines[ce+1:wi]+cls+lines[wi:]
s="\n".join(lines)
s=s.replace("\n\n\nexport async function registerGis","\n\nexport async function registerGis")
s=s.replace("  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetHarness, executionTargetBodyResponse, exactDocumentBackboneMessage } = workerLawFakesV1(vitest, dependencies, source);","  const fakes = workerLawFakesV1(vitest, dependencies, source);\n  const { resetWorkerLaw, executionTargetLeaseFixture, executionTargetHarness, executionTargetBodyResponse, exactDocumentBackboneMessage } = fakes;",1)
open(p,"w",encoding="utf8").write(s)
print("class moved")
