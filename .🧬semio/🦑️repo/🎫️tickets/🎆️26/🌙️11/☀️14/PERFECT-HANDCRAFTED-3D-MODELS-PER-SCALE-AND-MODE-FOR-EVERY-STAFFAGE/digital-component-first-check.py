"""📋 Verify the repaired physical controls and the first component candidates sequentially."""
import shutil,subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent;G=T/'🗑️generated'
names=('etics-brick-wall-12','etics-brick-wall-30','external-venetian-blind','solar-thermal-collector')
for name in names:
    with (G/('component-semantic-'+name+'.log')).open('w') as out:
        subprocess.run([shutil.which('bun'),'--smol',T/'digital-building-semantic.ts',G/('building-'+name+'.json')],stdout=out,stderr=out)
    print('[DEBUG]',name,flush=True)
for name in sys.argv[1:]:
    with (G/('component-publish-'+name+'.log')).open('w') as out:status=subprocess.run([sys.executable,T/'digital-building-readback.py',name],stdout=out,stderr=out).returncode
    print('[DEBUG]',name,status,flush=True)
