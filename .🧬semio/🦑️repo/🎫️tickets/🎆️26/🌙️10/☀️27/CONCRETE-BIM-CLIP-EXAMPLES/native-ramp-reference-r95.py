"""🛝️ Retain actual owned/full-reference ramp source contracts and native bilingual samples."""
import hashlib,importlib,json,sys,time,traceback
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/native-ramp-reference-r95';O.mkdir(parents=True,exist_ok=True);C=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';M=C/'🧱️model';F=M/'🔌️exchange/🧪️tests/🛝️reference.py'
INPUTS={Path(__file__),F,F.with_suffix('.json'),F.with_suffix('.schema.json'),M/'🐍️.py',M/'🔌️exchange/🐍️.py',M/'🔣️.json',C/'🧪️tests/🐍️.py'}


def hashes():
    """🔐 Bind selected source stock, actual native producers and independent fixture bytes before import."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in INPUTS}


before=hashes();T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🛝️reference');started=time.monotonic();error=None
try:T.test_native_flight_body_and_full_reference_are_separate_exact_products()
except Exception as failure:error=traceback.format_exc();print('[DEBUG]',type(failure).__name__,str(failure).splitlines()[0],flush=True)
after=hashes();report={'success':error is None,'stable':before==after,'before':before,'after':after,'error':error,'seconds':time.monotonic()-started};(O/(sys.argv[1]+'.json')).write_text(json.dumps(report,indent=2),encoding='utf8')
if sys.argv[1]=='red':assert error;raise SystemExit(0)
assert report['success'] and report['stable'];m=T.fixture();samples=[]
for language in T.CASE['languages']:
    fragment=T.A.step_excerpt('NEUTRAL-FLIGHT',m,language=language,spatial=False);samples.append({'language':language,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'body_volume':T.CASE['owned_volume'],'reference_volume':T.CASE['reference_volume']})
assert hashes()==after;(O/'samples.json').write_text(json.dumps({'before':before,'samples':samples}),encoding='utf8');print('[DEBUG] Native ramp owned/full Reference GREEN',len(samples),'fragments',flush=True)
