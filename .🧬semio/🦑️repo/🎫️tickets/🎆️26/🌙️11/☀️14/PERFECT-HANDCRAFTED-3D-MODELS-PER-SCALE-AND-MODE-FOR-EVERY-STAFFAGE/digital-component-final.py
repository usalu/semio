"""📏 Verify each reviewed component sequentially, including the previously rejected print stages."""
import hashlib,json,os,shutil,subprocess,sys
from pathlib import Path
from runpy import run_path
T=Path(__file__).resolve().parent;G=T/'🗑️generated';bun=shutil.which('bun')
for n in sys.argv[1:]:
    f=G/('building-'+n+'.json');data=json.loads(f.read_bytes());sha=hashlib.sha256(f.read_bytes()).hexdigest()[:10]
    source=run_path(str(T/'digital-building-equipment.py'))['catalog'](n)
    canonical=G/(n+'-art-canonical.log');sheet=G/(n+'-art-sheet.log')
    if json.loads(source.read_bytes())==data and canonical.exists() and 'schema 0, validateAsset 0, checkVolumes 0' in canonical.read_text() and sheet.exists() and hashlib.sha256(source.read_bytes()).hexdigest()[:10] in sheet.read_text():
        print('[DEBUG]',n,'canonical already verified',flush=True);continue
    def invoke(args,outname):
        with (G/(outname+'.log')).open('w') as out:
            return subprocess.run([str(x) for x in args],stdout=out,stderr=out).returncode
    sem='component-sem-'+n
    if invoke([bun,'--smol',T/'digital-building-semantic.ts',f],sem) or (G/(sem+'.log')).read_text().strip():
        print('[DEBUG]',n,sha,'semantic',flush=True);continue
    band=[str(k)+'/'+str(d) for k,v in data['asset']['representations'].items() if v['status']=='authored' for d in {int(k),v['validRange']['minDenominator'],v['validRange']['maxDenominator']}]
    if invoke([bun,'--smol',T/'digital-paper-candidate.ts',f,*band],'component-print-'+n):
        print('[DEBUG]',n,sha,'print',flush=True);continue
    status=invoke([sys.executable,T/'digital-building-readback.py',n],'component-pass-'+n)
    print('[DEBUG]',n,sha,'publication',status,flush=True)
