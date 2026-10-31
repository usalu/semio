"""🧪 Renew bounded independent semantic and existing whole-audit controls in one normal resource grant."""
import hashlib,json,runpy,sys,time,traceback
from pathlib import Path
sys.dont_write_bytecode=True
T=Path(__file__).parent;R=next(p for p in T.parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(R));scripts=['whole-semantic-invalid-r95.py','whole-exact-separation-r95.py','whole-expanded-prism-contract-r95.py','analytic-difference-oracle-tests-r95.py','analytic-difference-intersections-r95.py','whole-semantic-csg-r95.py'];rows=[];start=time.monotonic()
for name,args in[(name,[])for name in scripts]+[('whole-expanded-interference-r92.py',['controls'])]:
    then=time.monotonic();row={'script':name,'pass':False};sys.argv=[str(T/name),*args]
    try:runpy.run_path(str(T/name),run_name='__main__');row['pass']=True
    except SystemExit as status:row.update({'pass':status.code in(None,0),'exit_code':status.code})
    except Exception:row['error']=traceback.format_exc()
    row['seconds']=time.monotonic()-then;rows.append(row);print('[DEBUG] regression '+json.dumps(row),flush=True)
out={'pass':all(r['pass']for r in rows),'rows':rows,'seconds':time.monotonic()-start,'scope':'Bounded neutral semantic predicates and existing independent control geometries; not current-house clearance.'};(T/'🗑️generated/whole-semantic-regression-r95.json').write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] complete '+json.dumps(out),flush=True);raise SystemExit(not out['pass'])
