"""🔎 Execute only production plan eligibility predicates against current source."""
import ast,hashlib,json
from pathlib import Path
T=Path(__file__).parent
R=next(p for p in T.parents if (p/'AGENTS.md').exists() and (p/'package.json').exists())
B=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
P=B/'🎨️visuals/🏛️savoye/📋️plan/🐍️.py'
raw=(B/'🧱️model/🔣️.json').read_bytes();m=json.loads(raw);tree=ast.parse(P.read_text(encoding='utf8'))
ns={};exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in ('plan_part_rank','plan_part_visible')],type_ignores=[]),str(P),'exec'),ns)
rows=[]
for tag in ('FIN-PILOTIS-GRAVEL','FIN-CELLAR-CEMENT','FIN-HALL'):
 e=next(e for e in m['elements'] if e['tag']==tag);d={**e,'kind':e['geometry']['kind']};parts=e['geometry']['parts'];z=next(s['elevation']['value'] for s in m['storeys'] if s['tag']==e['storey']);parts=[{**p,**({'z0':min(v[2] for v in p['mesh']['vertices'])+z,'z1':max(v[2] for v in p['mesh']['vertices'])+z} if 'mesh' in p else {'z0':p['z0']+z,'z1':p['z1']+z})} for p in parts];visible=[i for i,p in enumerate(parts) if ns['plan_part_visible'](d,p,e['storey'],(z,z+3.17),True,False)];rows.append({'tag':tag,'parts':len(parts),'predicate_visible_parts':len(visible),'roles':sorted(set(p.get('role','') for p in parts))})
e=next(e for e in m['elements'] if e['tag']=='FIN-PILOTIS-GRAVEL');candidate={**e,'kind':e['geometry']['kind']};accepted=sum(ns['plan_part_visible'](candidate,{**p,'role':'wearing','z0':-.19,'z1':-.15},'ground',(0.,3.17),True,False) for p in e['geometry']['parts']);assert accepted==len(e['geometry']['parts']);rows.append({'tag':e['tag'],'scope':'nonmutating proposed wearing-role and -0.15 terrain test','predicate_visible_parts':accepted})
e=next(e for e in m['elements'] if e['tag']=='TUBE-STAIR-CENTRAL');rows.append({'tag':e['tag'],'source_storey':e['storey'],'domains':e['metadata']['domains'],'part_z':[min(p['z0'] for p in e['geometry']['parts']),max(p['z1'] for p in e['geometry']['parts'])],'excluded_by_plan_solids_storey_gate':['cellar','first','roof'],'reason':'not structural, kind railings != stairs, source storey != requested storey'})
out={'source':hashlib.sha256(raw).hexdigest(),'production_plan':hashlib.sha256(P.read_bytes()).hexdigest(),'scope':'Actual AST predicates without importing drawing engine or building geometry','rows':rows}
(T/'🗑️generated/production-view-filter-probes-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps(out))
