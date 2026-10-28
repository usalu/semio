import pathlib,json,hashlib,re
p=pathlib.Path(__file__).resolve().parent;root=p.parents[6];s=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema'
m=json.loads((s/'👁️mvd/🔣️.json').read_text(encoding='utf-8-sig'));defs=[]
for f in (s/'👁️mvd/🧩️concepts').rglob('*definition.json'):
 d=json.loads(f.read_text(encoding='utf-8-sig'));defs.append({'path':str(f.relative_to(root)),'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'keys':list(d),'definition':d})
paths=[s/'⚖️validation'/n for n in ['compiler.ts','evaluator.ts','population.ts','admission.ts','control.ts','index.ts']]+[s/'👁️mvd/🟦️.ts',s/'👁️mvd/header.ts']
out={'mvdViews':m['views'],'mvdEntities':len(m['entities']),'mvdConcepts':len(m['concepts']),'conformanceEvaluated':m['conformanceEvaluated'],'executableDefinitions':defs,'sourceHashes':[{'path':str(f.relative_to(root)),'sha256':hashlib.sha256(f.read_bytes()).hexdigest()}for f in paths]}
(p/'🗑️generated/compliance-infrastructure-census.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({'entities':out['mvdEntities'],'concepts':out['mvdConcepts'],'definitions':len(defs),'views':m['views']}))
