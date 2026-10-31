"""🌳 Retain native analytic operand RED/GREEN and self-contained geometry samples for independent IFC parsing."""
import hashlib,importlib,inspect,json,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/native-analytic-difference-r95';O.mkdir(parents=True,exist_ok=True)
C=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';M=C/'🧱️model';F=M/'🔌️exchange/🧪️tests/➖️difference.py'
INPUTS={Path(__file__),F,M/'🔌️exchange/🐍️.py',M/'🔌️exchange/➖️difference/🐍️.py',C/'🧪️tests/🐍️.py',M/'🧊️solids/➖️difference/🧪️tests/🔣️.json',M/'🧊️solids/➖️difference/🧬️.schema.json'}


def hashes():
    """🔐 Bind the shared schema and source vectors to actual native planner, writer and parser bytes."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in INPUTS}


before=hashes();T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.➖️difference');started=time.monotonic();rows=[]
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
    for scope,operand in (('Body',row['part']['csg']),('Stock',row['part']['csg']['stock'])):
        plan,writer,entities=T.fragment(operand);samples.append({'case':row['id'],'scope':scope,'text':'\n'.join(writer.lines),'root':writer.ids[writer.world_root],'plan_root':writer.ids[plan.root],'frame':plan.frame,'operations':{str(writer.ids[operation['node']]):{name:value for name,value in operation.items() if name!='node'} for operation in plan.operations},'operand':operand,'expected':row['expected']})
assert hashes()==after;(O/'samples.json').write_text(json.dumps({'before':before,'samples':samples},ensure_ascii=False),encoding='utf8');print('[DEBUG] Native analytic source trees GREEN',len(samples),'Body/declared-stock fragments',flush=True)
