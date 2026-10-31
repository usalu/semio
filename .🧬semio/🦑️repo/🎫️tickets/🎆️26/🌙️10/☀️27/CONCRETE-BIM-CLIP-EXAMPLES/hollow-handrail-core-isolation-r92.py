"""🔬 Isolate exact tube core material contact from retained original construction operands."""
import importlib,json,sys,traceback,time
from fractions import Fraction
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
ticket=Path(__file__).parent;api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
def decode(obj):return Fraction(*map(int,obj['$rational']))if set(obj)=={'$rational'}else obj
f=json.loads((ticket/'🗑️generated'/'hollow-handrail-failure-r92.json').read_text(encoding='utf8'),object_hook=decode);rows=[];start=time.monotonic()
for si in (22,23):
    v=f['stock'][si]['mesh']['vertices'];cv=f['cutters'][0]['mesh']['vertices'];sj=(si+1)%24
    print('[DEBUG] sharedinner '+str(si)+' '+str([v[2]==cv[sj],v[3]==cv[si],v[6]==cv[24+sj],v[7]==cv[24+si]]),flush=True)
    for label,part in [('stock',f['stock'][si]),('core',f['cutters'][0])]:
        vv=api.part_vertices(part);violations=[]
        for fi,face in enumerate(api.part_faces(part)):
            a,b,c=[vv[k]for k in face[:3]];u=[b[k]-a[k]for k in range(3)];w=[c[k]-a[k]for k in range(3)];n=[u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0]]
            deviations=[sum(n[k]*(vv[j][k]-a[k])for k in range(3))for j in face]
            if any(deviations):violations.append([fi,[float(d)for d in deviations]])
        print('[DEBUG] exactface '+str(si)+' '+label+' '+str(violations),flush=True)
for i,part in enumerate(f['stock']):
    for j,cut in enumerate(f['cutters']):
        owned,removed=api._subtract_native_material([part],[cut]);bad=[]
        for k,p in enumerate(owned):
            try:api._polyhedron_part(api.part_vertices(p),api.part_faces(p))
            except Exception:bad.append({'index':k,'part':p})
        if bad or removed>1e-16:rows.append({'stock':i,'cutter':j,'removed':removed,'original':api._polyhedron_volume(part),'parts':len(owned),'bad':bad})
print('[DEBUG] '+json.dumps([{'stock':r['stock'],'cutter':r['cutter'],'removed':r['removed'],'original':r['original'],'parts':r['parts'],'bad':len(r['bad'])}for r in rows]),flush=True)
(ticket/'🗑️generated'/'hollow-handrail-core-isolation-r92.json').write_text(json.dumps({'rows':rows,'seconds':time.monotonic()-start}),encoding='utf8')
