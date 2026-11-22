"""🗃️ Run the candidate and canonical checks for each cut component within a single guard."""
import os,subprocess,sys,shutil
from pathlib import Path
T=Path(__file__).resolve().parent;G=T/'🗑️generated'
for n in sys.argv[1:]:
    if n=='test':
        c=[shutil.which('bun'),str(Path('node_modules/vitest/vitest.mjs').resolve()),'run','--config',T/'vs-vitest.config.ts','--maxWorkers','1']
    else:c=[sys.executable,T/'digital-building-readback.py',n]
    with (G/('component-pass-'+n+'.log')).open('w') as out:code=subprocess.run(c,stdout=out,stderr=out).returncode
    print('[DEBUG]',n,code,flush=True)
