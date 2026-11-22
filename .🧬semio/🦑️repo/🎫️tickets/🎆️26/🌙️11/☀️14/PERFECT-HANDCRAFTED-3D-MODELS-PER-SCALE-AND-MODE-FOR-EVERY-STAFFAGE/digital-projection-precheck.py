"""📐 Reopen the valve's physical projection before native boiler publication."""
from pathlib import Path
from runpy import run_path
import shutil, subprocess, sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated'
cat=run_path(str(t/'digital-building-equipment.py'))['catalog']
with (g/'valve-front-probe.log').open('w') as out:
    subprocess.run([shutil.which('bun'),'--smol',t/'digital-valve-diagnosis.ts',cat('expansion-valve')],stdout=out,stderr=out,check=True)
subprocess.run([sys.executable,t/'digital-component-final.py','gas-boiler'],check=True)
