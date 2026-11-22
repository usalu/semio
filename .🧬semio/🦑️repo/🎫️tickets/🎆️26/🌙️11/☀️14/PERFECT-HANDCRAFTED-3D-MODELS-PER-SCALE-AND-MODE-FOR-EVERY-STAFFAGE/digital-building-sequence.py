"""🪜 Run independent measured building-equipment checks in a single bounded worker."""
import shutil,subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
bun=shutil.which('bun')
checks=[('plant-baseline-red',[bun,str(Path('node_modules/vitest/vitest.mjs').resolve()),'run','--config',T/'vs-vitest.config.ts','--maxWorkers','1','-t','open airflow passage|four pierced isolators'])]
for name in sys.argv[1:]:
    checks.append((name+'-publication',[sys.executable,T/'digital-building-readback.py',name]))
for label,cmd in checks:
    with (G/(label+'.log')).open('w') as out:status=subprocess.run(cmd,stdout=out,stderr=out).returncode
    print('[DEBUG]',label,status,flush=True)
