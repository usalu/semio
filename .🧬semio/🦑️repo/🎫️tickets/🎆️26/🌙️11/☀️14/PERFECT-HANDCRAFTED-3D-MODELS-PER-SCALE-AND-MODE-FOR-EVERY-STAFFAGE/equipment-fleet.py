"""🚚 Run one equipment publication or strict measurement at a time and retain each revision’s result."""
import argparse
from hashlib import sha256
import json,os,re,shutil,subprocess,sys,time
from pathlib import Path

ticket=Path(__file__).resolve().parent
root=ticket.parents[6];staffage=next((root/'🎓️teaching').glob('*architecture/*staffage'));catalog=staffage/'🗂️catalog'
output=ticket/'🗑️generated/equipment-fleet';output.mkdir(exist_ok=True,parents=True)
bun=shutil.which('bun') or next((p for p in (root/'.cache/b/t/m/bun').glob('*/*') if p.name in ('bun','bun.exe')),None)
parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['glb','document','strict']);parser.add_argument('assets',nargs='*');parser.add_argument('--seconds',type=float,default=0);parser.add_argument('--limit',type=int,default=0);parser.add_argument('--status',action='store_true');parser.add_argument('--group',choices=['equipment','household','humans'],default='equipment');parser.add_argument('--signature',action='store_true');parser.add_argument('--shards',action='store_true');args=parser.parse_intermixed_args()
groups={'equipment':('vehicles','machinery','components'),'household':('objects','furniture','appliances'),'humans':('humans',)}[args.group]
files=sorted((f for folder in catalog.iterdir() if folder.name.endswith(groups) for f in folder.glob('*/*.json')),key=lambda f:f.stat().st_size)
def priority(path):
    record=output/f'{path.parent.name}-{args.mode}.json'
    if not record.exists():return (1,path.stat().st_size)
    last=json.loads(record.read_bytes())
    if not last.get('completed'):return (3,path.stat().st_size)
    if not last.get('passed') or sha256(path.read_bytes()).hexdigest()!=last.get('input'):return (0,path.stat().st_size)
    return (2,path.stat().st_size)
files.sort(key=priority)
if args.assets:files=[f for n in args.assets for f in files if f.parent.name==n]
def signatures():
    code=sha256();sources=set();pending=[staffage/'🧬️schema/🔣️.json',ticket/('digital-scenes.ts' if args.mode=='glb' else 'r3-validate.ts')]
    if args.mode!='glb':pending.extend((staffage/'🗂️catalog').glob('*environments/*.json'))
    imports=re.compile(r"(?:\bfrom\s*|\bimport\s*\(?\s*)[\"'](\.[^\"']+)[\"']")
    while pending:
        src=pending.pop().resolve()
        if src in sources or not src.is_file():continue
        sources.add(src)
        if src.suffix=='.ts':pending.extend(src.parent/path for path in imports.findall(src.read_text()))
    prior_code=sha256()
    for src in sorted(sources):
        name=str(src.relative_to(root)).encode();content=src.read_bytes()
        prior_code.update(name);prior_code.update(content)
        if args.mode=="glb" and src.suffix==".json":continue
        code.update(name);code.update(content)
    return prior_code.hexdigest(),code.hexdigest()
prior_version,version=signatures()
start=time.monotonic();done=0;status={'passed':[],'findings':[],'pending':[]}
if args.signature:print(version);raise SystemExit(0)
for path in files:
    if not args.status:prior_version,version=signatures()
    asset=path.parent.name;digest=sha256(path.read_bytes()).hexdigest();record=output/f'{asset}-{args.mode}.json';prev=json.loads(record.read_bytes()) if record.exists() else {}
    if prev.get('input')==digest and prev.get('code') in (version,prior_version) and prev.get('current') and prev.get('completed'):
        if prev['code']!=version and not args.status:
            prev['code']=version;temp=record.with_suffix('.'+str(os.getpid())+'.partial');temp.write_text(json.dumps(prev,ensure_ascii=False));os.replace(temp,record)
        status['passed' if prev['passed'] else 'findings'].append(asset);continue
    status['pending'].append(asset)
    if args.status:continue
    if args.seconds and time.monotonic()-start>=args.seconds or args.limit and done>=args.limit:break
    stamp=time.monotonic();category=re.sub(r'^[^a-z]+','',path.parent.parent.name)
    command=[ticket/'digital-scenes.ts',asset] if args.mode=='glb' else [ticket/'r3-validate.ts',category+'/'+asset,'--catalog-only','--volumes']
    external=[sys.executable,str(ticket/'equipment-guard.py'),str(bun),'--smol',*map(str,command)]
    if args.mode=='strict' and (args.shards or prev.get('completed') is False and prev.get('input')==digest):
        remaining=max(1,args.seconds-time.monotonic()+start) if args.seconds else 0
        external=[sys.executable,str(ticket/'equipment-shard.py'),str(bun),version,digest,str(path),str(remaining)]
    if args.mode=='document':external=external[:-1]
    worker=subprocess.Popen(external,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,cwd=root)
    cap=max(.1,args.seconds-time.monotonic()+start) if args.seconds and external[1].endswith('equipment-guard.py') else None
    try:
        stdout,stderr=worker.communicate(timeout=cap)
    except subprocess.TimeoutExpired:
        worker.terminate();stdout,stderr=worker.communicate();stderr+='\n[DEBUG] bounded slot ended'
    proc=subprocess.CompletedProcess(external,worker.returncode,stdout,stderr)
    outputText=proc.stdout+'\n'+proc.stderr;lines=outputText.splitlines();summary={}
    if args.mode=='glb':
        summary=next((json.loads(l) for l in lines if l.startswith('{')),{});complete=bool(summary.get('scenes')) or bool(summary.get('error'));passed=complete and not summary.get('error') and not summary.get('findings') and proc.returncode==0
    else:
        complete=any(l.startswith(('VALID ','INVALID ')) and (args.mode=='document' or 'checkVolumes not run' not in l) for l in lines);passed=complete and proc.returncode==0;summary['output']=lines
    current=digest==sha256(path.read_bytes()).hexdigest() and signatures()[1]==version
    if not current:
        print('[DEBUG] source or validator changed during verification of',asset,flush=True)
        continue
    res={'asset':asset,'input':digest,'code':version,'current':current,'completed':complete,'passed':passed,'seconds':round(time.monotonic()-stamp,2),'result':summary}
    if not complete:res['interruption']=outputText[:500]
    history=output/'history.jsonl'
    with history.open('a') as log:
        if prev.get('completed'):log.write(json.dumps({'mode':args.mode,**prev},ensure_ascii=False)+'\n')
        if complete:log.write(json.dumps({'mode':args.mode,**res},ensure_ascii=False)+'\n')
    temp=record.with_suffix('.'+str(os.getpid())+'.partial');temp.write_text(json.dumps(res,ensure_ascii=False));os.replace(temp,record);done+=1
    print(asset,args.mode,'passed' if passed else 'findings' if complete else 'interrupted',res['seconds'],summary.get('bytes',''),flush=True)
    if not complete and 'Host headroom below' in outputText:break
if args.status:print(json.dumps(status,ensure_ascii=False))
