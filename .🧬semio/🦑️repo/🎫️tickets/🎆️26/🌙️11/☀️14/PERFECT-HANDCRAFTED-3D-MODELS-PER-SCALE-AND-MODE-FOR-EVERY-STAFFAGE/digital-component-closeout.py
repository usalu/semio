"""✅ Run the final component readbacks before the maintained equipment and section suite."""
import os,shutil,subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent;G=T/'🗑️generated'
subprocess.run([sys.executable,T/'digital-component-final.py',*sys.argv[1:]])
with (G/'component-bounds-final.log').open('w') as out:
    status=subprocess.run([shutil.which('bun'),str(Path('node_modules/vitest/vitest.mjs').resolve()),'run','--config',T/'vs-vitest.config.ts','--maxWorkers','1'],stdout=out,stderr=out).returncode
print('[DEBUG] maintained bounds',status,flush=True)
