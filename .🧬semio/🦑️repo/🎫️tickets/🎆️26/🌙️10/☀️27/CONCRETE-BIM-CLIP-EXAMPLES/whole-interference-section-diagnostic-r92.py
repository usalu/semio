"""🔬 Isolate independent section failures without changing source or healing geometry."""
import importlib.util,json,sys,time,warnings
from pathlib import Path
from scipy.integrate import IntegrationWarning
T=Path(__file__).parent;G=T/'🗑️generated';spec=importlib.util.spec_from_file_location('owned_audit',T/'whole-owned-interference-r92.py');audit=importlib.util.module_from_spec(spec);spec.loader.exec_module(audit)
data=json.loads((G/'whole-owned-capture-r92.json').read_text(encoding='utf8'));result=json.loads((G/'whole-interference-result-r92.json').read_text(encoding='utf8'));records={(r['tag'],r['index']):r for r in data['records']};rows=[];failures=[];original=audit.Solid.section

def inspected(s,z):
 try:return original(s,z)
 except ValueError as error:
  vertices=sorted(set(float(p[2]) for p in s.v));lo=max((v for v in vertices if v<=z),default=None);hi=min((v for v in vertices if v>=z),default=None);failures.append({'tag':s.r['tag'],'index':s.r['index'],'z':float(z),'at_source_vertex':z in vertices,'nearest_lower_vertex_z':lo,'nearest_upper_vertex_z':hi,'source_vertex_ulp_distance':min(abs(float(v)-float(z)) for v in vertices),'reason':str(error)});raise

audit.Solid.section=inspected;started=time.monotonic()
for r in result['unresolved_part_pairs']:
 if r['reason']!='Open section graph':continue
 a=audit.Solid(records[(r['source']['tag'],r['source']['index'])]);b=audit.Solid(records[(r['target']['tag'],r['target']['index'])]);before=len(failures)
 try:
  with warnings.catch_warnings():warnings.simplefilter('error',IntegrationWarning);value=audit.intersection(a,b)
  rows.append({'source':r['source'],'target':r['target'],'result':value,'failures':failures[before:]})
 except Exception as e:rows.append({'source':r['source'],'target':r['target'],'error':str(e),'failures':failures[before:]})
audit.save('whole-interference-section-diagnostic-r92.json',{'source':data['source'],'rows':rows,'seconds':time.monotonic()-started});print(json.dumps({'cases':len(rows),'exact_vertex_failures':sum(f['at_source_vertex'] for f in failures),'seconds':time.monotonic()-started}),flush=True)
