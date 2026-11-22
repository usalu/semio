"""🧪 Run exact native document or volume checks as separate jobs when a combined run reaches its memory limit."""
from pathlib import Path
from hashlib import sha256
import json,os,subprocess,sys
t=Path(__file__).resolve().parent
groups=('availability,feature,bones,solids,copied,slender,monotonic','bounds','silhouette,views-disagree','cross-scale','thermal-coverage','structure-coverage')
def measure(bun,file,part,signature):
    file=Path(file);raw=file.read_bytes();digest=sha256(raw).hexdigest();asset=json.loads(raw)['asset']
    env={**os.environ,'EQUIPMENT_RSS_MIB':'950','EQUIPMENT_HEADROOM_PERCENT':'48'}
    folder=t/'🗑️generated'/'split-rules';folder.mkdir(exist_ok=True)
    def run(key,script,*args):
        path=folder/f'{asset["id"]}-{digest[:12]}-{signature[:12]}-{key}.json'
        if path.exists():return json.loads(path.read_bytes())
        limit='1120' if script=='digital-scale-proof.ts' and args[-1]=='volumes-thermal-coverage' else env['EQUIPMENT_RSS_MIB']
        process=subprocess.run([sys.executable,t/'equipment-guard.py',bun,'--smol',t/script,*args],capture_output=True,text=True,env={**env,'EQUIPMENT_RSS_MIB':limit})
        output=process.stdout.strip()
        if process.returncode not in (0,1) or not output:return None
        if not output.startswith(('{','VALID ','ASSET ','SCHEMA ','PRINT ','INVALID ')):return None
        data={'code':process.returncode,'output':output}
        tmp=path.with_suffix('.'+str(os.getpid())+'.partial');tmp.write_text(json.dumps(data));os.replace(tmp,path)
        return data
    problems=[]
    if part=='document':
        result=run('semantic','r3-validate.ts','--file',file,'--compact','--semantics')
        if result is None:return None
        text=result['output'].splitlines()
        if not any(s.startswith(('VALID ','INVALID ')) for s in text):return None
        problems.extend(s for s in text if s.startswith(('ASSET ','SCHEMA ','PRINT ')))
        for scale,rep in asset['representations'].items():
            if rep['status']!='authored':continue
            for end in sorted({int(scale),rep['validRange']['minDenominator'],rep['validRange']['maxDenominator']}):
                row=run(f'print-{scale}-{end}','digital-paper-candidate.ts',file,f'{scale}/{end}')
                if row is None:return None
                vector=json.loads(row['output']);assert vector['d']==int(scale) and vector['k']==end
                if not vector['suitable']:problems.append(f'PRINT {scale}/{end}: '+json.dumps(vector['violations'],ensure_ascii=False))
        return problems
    for i,g in enumerate(groups):
        keys=['volumes-'+x for x in g.split(',')]
        row=run(f'{part}-{i}','digital-scale-proof.ts',file,part,','.join(keys))
        if row is None:return None
        data=json.loads(row['output']);assert sorted(data['rules'])==sorted(keys)
        problems.extend(data['issues'])
    return problems
