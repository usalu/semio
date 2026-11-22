"""📏 Run the public schema, semantic, print and volume checks with bounded process lifetimes."""
import json,shutil,subprocess,sys
from pathlib import Path
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
file=Path(sys.argv[1])
asset=json.loads(file.read_bytes())['asset']
bun=shutil.which('bun')
def run(script,args):
    result=subprocess.run([bun,'--smol',T/script,*args],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    return result.returncode,result.stdout
status,output=run('r3-validate.ts',['--file',file,'--compact','--semantics'])
print(output,flush=True)
failed=bool(status)
if failed:sys.exit(1)
for k,rep in asset['representations'].items():
    if rep.get('status')!='authored':continue
    for end in sorted({int(k),rep['validRange']['minDenominator'],rep['validRange']['maxDenominator']}):
        status,out=run('digital-paper-candidate.ts',[file,f'{k}/{end}']);print(out,flush=True);failed|=bool(status)
groups=('availability,feature,bones,solids,copied,slender,monotonic','bounds','silhouette,views-disagree','cross-scale','thermal-coverage','structure-coverage')
expected=['volumes-'+rule for group in groups for rule in group.split(',')]
status,out=run('digital-scale-proof.ts',['--rules']);assert not status and sorted(expected)==sorted(json.loads(out))
for k in asset['volumes']:
    for group in groups:
        keys=['volumes-'+rule for rule in group.split(',')]
        status,out=run('digital-scale-proof.ts',[file,k,','.join(keys)])
        rows=[json.loads(l) for l in out.splitlines() if l.startswith('{')]
        assert len(rows)==1 and sorted(rows[0]['rules'])==sorted(keys),out[-1000:]
        print('[DEBUG]',k,group,rows[0]['issues'],flush=True)
        failed|=bool(status or rows[0]['issues'])
if not failed:print('VALID '+asset['id']+': schema 0, validateAsset 0, checkVolumes 0')
sys.exit(int(failed))
