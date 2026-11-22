"""📐 Locate the remaining generator contour and layered surface differences."""
from pathlib import Path
import subprocess,shutil
T=Path(__file__).resolve().parent
G=T/'🗑️generated';file=G/'building-diesel-generator.json'
for name,args in [('thermal-100',[T/'digital-scale-proof.ts',file,'100','volumes-thermal-coverage']),('thermal-50',[T/'digital-scale-proof.ts',file,'50','volumes-thermal-coverage']),('elevation-200',[T/'digital-outline-difference.ts','diesel-generator','200','elevation',file,'--bands'])]:
    with (G/('building-generator-'+name+'.log')).open('w') as out:
        status=subprocess.run([shutil.which('bun'),'--smol',*args],stdout=out,stderr=out).returncode
    print('[DEBUG]',name,status,flush=True)
