"""🏭 Verify connected intake framing and the complete maintained model-bounds suite."""
import subprocess,shutil,sys
from pathlib import Path
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
for label,cmd in [('generator-uprights',[sys.executable,T/'digital-building-readback.py','diesel-generator']),('building-maintained-final',[shutil.which('bun'),str(Path('node_modules/vitest/vitest.mjs').resolve()),'run','--config',T/'vs-vitest.config.ts','--maxWorkers','1'])]:
    with (G/(label+'.log')).open('w') as out:status=subprocess.run(cmd,stdout=out,stderr=out).returncode
    print('[DEBUG]',label,status,flush=True)
