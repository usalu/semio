"""🌡️ Checks every region at every digital scale in a fresh memory-guarded process and verifies the reported source digest."""
from pathlib import Path
import sys,os,json,subprocess,hashlib,time
t=Path(__file__).resolve().parent
out=t/'🗑️generated'/'hu-review'
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
deadline=time.monotonic()+float(os.environ.get('HU_ROUND_SECONDS','inf'))
for name in (sys.argv[1:] or sorted(p.name for p in root.iterdir() if p.is_dir())):
    path=next((root/name).glob('*'))
    for scale in [50,100,200,500]:
        if time.monotonic()>=deadline:sys.exit(0)
        sha=hashlib.sha256(path.read_bytes()).hexdigest()
        report=out/(name+'-thermal-'+str(scale)+'.json')
        allscales=out/(name+'-all.json')
        full=out/(name+'-all-'+str(scale)+'.json')
        proofs=[json.loads(path.read_text()) for path in [report,allscales,full] if path.exists()]
        if any(q.get('current') and q.get('sha256')==sha and not q['problems'] for q in proofs):continue
        log=out/('heat-'+name+'-'+str(scale)+'.log')
        with log.open('w') as fd:
            result=subprocess.run([sys.executable,str(t/'equipment-guard.py'),os.environ.get('BUN_EXE','bun'),'--smol','hu-checked.ts',name,'thermal',str(scale)],cwd=t,stdout=fd,stderr=fd)
        if result.returncode:
            print(name,scale,result.returncode,flush=True);sys.exit(result.returncode)
        q=json.loads(report.read_text())
        assert q['current'] and q['sha256']==sha
        if q['problems']:print(name,scale,q['problems'],flush=True)
    print(name,'checked',flush=True)
