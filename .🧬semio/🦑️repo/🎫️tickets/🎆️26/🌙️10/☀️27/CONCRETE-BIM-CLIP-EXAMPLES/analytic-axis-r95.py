"""🧿 Retain source-bound analytic placement RED/GREEN and bilingual native IFC samples."""
import hashlib,importlib,inspect,json,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/analytic-axis-r95';O.mkdir(parents=True,exist_ok=True)
C=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';M=C/'🧱️model';F=M/'🔌️exchange/🧪️tests/🧿️axes.py'
INPUTS={Path(__file__),F,F.with_suffix('.json'),F.with_suffix('.schema.json'),M/'🐍️.py',M/'🔣️.json',M/'🔌️exchange/🐍️.py',C/'🧪️tests/🐍️.py'}


def hashes():
    """🔐 Bind freshly imported physical/profile/native/fixture producers before executing the controls."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in INPUTS}


before=hashes();T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🧿️axes');rows=[];started=time.monotonic()
for name,fn in inspect.getmembers(T,inspect.isfunction):
    if not name.startswith('test_'):continue
    error=None
    try:fn()
    except Exception as failure:error=traceback.format_exc();print('[DEBUG]',name,type(failure).__name__,str(failure).splitlines()[0],flush=True)
    rows.append({'id':name,'error':error})
after=hashes();report={'success':not any(row['error'] for row in rows),'stable':before==after,'before':before,'after':after,'rows':rows,'seconds':time.monotonic()-started};(O/(sys.argv[1]+'.json')).write_text(json.dumps(report,indent=2),encoding='utf8')
if sys.argv[1]=='red':assert not report['success'];raise SystemExit(0)
assert report['success'] and report['stable'];samples=[]
for row in T.CASE['cases']:
    m,obj=T.fixture(row)
    for language in T.CASE['languages']:
        fragment=T.E.step_excerpt(obj['tag'],m,language=language,spatial=False)
        samples.append({'case':row['id'],'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'points':T.expected_points(row,fragment)})
assert hashes()==after;(O/'samples.json').write_text(json.dumps({'before':before,'samples':samples},ensure_ascii=False),encoding='utf8');print('[DEBUG] Exact native analytic axes GREEN',len(samples),'bilingual fragments',flush=True)
