"""🖥️ Exercise each staffage viewer and host case through its package configuration with bounded process lifetimes."""
from pathlib import Path
from hashlib import sha256
import json,os,subprocess,sys
t=Path(__file__).resolve().parent;r=t.parents[6];staff=next(r.glob('🎓*/*architecture/*staffage'));g=t/'🗑️generated';domain=sys.argv[1]
target=next(staff.glob('🎯*/*react')) if domain=='react' else next(staff.glob('*site'))
pkg=next(target.glob('📦*/🟦*'));tests=next(target.glob('🧪*'));config=next(tests.glob('*config/🟦*.ts'))
runner=r/'node_modules/vitest/vitest.mjs';bun=os.environ['BUN_EXE']
for test in sorted(tests.glob('*/🟦*.ts*')):
    if test==config:continue
    name=test.parent.name;log=g/f'viewer-{domain}-{name}.log';report=log.with_suffix('.json')
    if report.is_file():
        previous=json.loads(report.read_bytes())
        if previous.get('code')==0 and previous.get('caseHash')==sha256(test.read_bytes()).hexdigest():continue
    with log.open('w') as stream:
        code=subprocess.run([sys.executable,t/'equipment-guard.py',bun,'--smol',runner,'run','--configLoader','native','--config',config,'--maxWorkers=1','--pool=threads',test],cwd=pkg,env={**os.environ,'EQUIPMENT_RSS_MIB':'960','EQUIPMENT_HEADROOM_PERCENT':'48'},stdout=stream,stderr=stream).returncode
    row={'case':name,'code':code,'caseHash':sha256(test.read_bytes()).hexdigest()}
    report.write_text(json.dumps(row));print(name,code,flush=True)
    if code and 'failed to load config' in log.read_text():break
