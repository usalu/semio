#!/usr/bin/env python3
"""ts-scn.py: soundness + cost law of warm incremental tsc against the full non-incremental compiler on a corpus of edits.
usage: ts-scn.py <overlay root> <project dir (relative to root)> <out dir> <variant> [--steps a,b,c] [--no-oracle]
Each step is a declarative tree (original + edits). The warm state persists across steps (the realistic stale-state case);
after each step the incremental diagnostics must equal the diagnostics of a cold `tsc --noEmit` (the oracle).
Variants: `inc` = the live command (`--incremental true --tsBuildInfoFile`), `inc-nobuild` = same without tsbuildinfo reuse (control)."""
import json,os,shutil,subprocess,sys,time,resource
root,proj,out,variant=sys.argv[1:5]
opts=sys.argv[5:]
only=None
if '--steps' in opts: only=opts[opts.index('--steps')+1].split(',')
oracle='--no-oracle' not in opts
NODE=os.environ.get('NODE_FLAGS','').split()
R=os.path.abspath(root);P=os.path.join(R,proj);O=os.path.abspath(out);os.makedirs(O,exist_ok=True)
TSC=os.environ.get('TSC','/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc')
COURSE='🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts'
ENTRY='🎓️teaching/🏛️architecture/📚️course/🟦️.ts'
FW='🧰️framework/📦️packages/🟦️typescript/🟦️.ts'
BACKOFF='🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/🔁️jittered-backoff/🟦️.ts'
DEPLOYJSON='🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json'
TSCONFIG=proj+'/tsconfig.json'
AMBIENT='🎓️teaching/🏛️architecture/📚️course/🧪️tests/tsg-ambient.ts'
def app(f,t): return ('append',f,t)
def rep(f,a,b): return ('replace',f,a,b)
STEPS=[
 ('original',[]),
 ('noop',[]),
 ('leaf-comment',[app(COURSE,'\n// ts-g probe\n')]),
 ('leaf-body',[app(COURSE,'\nconst tsGInternal = 1 + 1;\n')]),
 ('leaf-export',[app(COURSE,'\nexport const tsGProbe: number = 1;\n')]),
 ('leaf-error',[app(COURSE,'\nexport const tsGBad: number = "x";\n')]),
 ('leaf-error-fixed',[]),
 ('mid-body',[app(BACKOFF,'\nconst tsGInternal = 1 + 1;\n')]),
 ('mid-export',[app(BACKOFF,'\nexport const tsGProbe: number = 1;\n')]),
 ('mid-error',[app(BACKOFF,'\nexport const tsGBad: number = "x";\n')]),
 ('api-break',[rep(FW,'export { retryWithJitteredBackoff } from','export { retryWithJitteredBackoff as tsGRenamed } from')]),
 ('api-break-fixed',[]),
 ('json-key',[rep(DEPLOYJSON,'"legal": {}','"legalX": {}')]),
 ('json-key-fixed',[]),
 ('delete-file',[('delete',COURSE)]),
 ('delete-file-restored',[]),
 ('ambient-global',[('create',AMBIENT,'declare global { interface Array<T> { at(index: number): never } }\nexport {};\n')]),
 ('ambient-global-removed',[]),
 ('tsconfig-option',[rep(TSCONFIG,'"types":','"noUncheckedIndexedAccess": true, "types":')]),
 ('tsconfig-option-removed',[]),
 ('tsconfig-paths',[rep(TSCONFIG,'"@semio-tech/course": ["../../../../../🧰️framework/🛍️products/📚️course/📦️packages/🟦️typescript/🟦️.ts"]','"@semio-tech/course": ["../../../../../🧰️framework/nonexistent.ts"]')]),
 ('tsconfig-paths-fixed',[]),
]
if os.environ.get('PROFILE')=='ui':
    UI='🧰️framework/🔨️modules/🖱️ui/'
    HUB=UI+'🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx'
    MID=UI+'🌐️i18n/🟦️.ts'
    TSCONFIG=proj+'/tsconfig.json'
    STEPS=[('original',[]),('noop',[]),
     ('hub-comment',[app(HUB,'\n// ts-g probe\n')]),
     ('hub-body',[app(HUB,'\nconst tsGInternal = 1 + 1;\n')]),
     ('hub-error',[app(HUB,'\nexport const tsGBad: number = "x";\n')]),
     ('hub-error-fixed',[]),
     ('mid-body',[app(MID,'\nconst tsGInternal = 1 + 1;\n')]),
     ('mid-export',[app(MID,'\nexport const tsGProbe: number = 1;\n')]),
     ('mid-error',[app(MID,'\nexport const tsGBad: number = "x";\n')]),
     ('mid-error-fixed',[]),
     ('tsconfig-option',[rep(TSCONFIG,'"strict": true','"strict": true, "noUncheckedIndexedAccess": true')]),
     ('tsconfig-option-removed',[])]
originals={}
def path(f): return os.path.join(R,f)
def remember(f):
    if f not in originals: originals[f]=open(path(f),encoding='utf8').read() if os.path.exists(path(f)) else None
def restore_all():
    for f,t in originals.items():
        if t is None:
            if os.path.exists(path(f)): os.remove(path(f))
        else:
            os.makedirs(os.path.dirname(path(f)),exist_ok=True);open(path(f),'w',encoding='utf8').write(t)
def apply(edits):
    for e in edits:
        remember(e[1])
        if e[0]=='append': open(path(e[1]),'a',encoding='utf8').write(e[2])
        elif e[0]=='replace':
            s=open(path(e[1]),encoding='utf8').read()
            if e[2] not in s: raise SystemExit('anchor missing '+e[1]+' '+e[2][:40])
            open(path(e[1]),'w',encoding='utf8').write(s.replace(e[2],e[3],1))
        elif e[0]=='delete': os.remove(path(e[1]))
        elif e[0]=='create':
            os.makedirs(os.path.dirname(path(e[1])),exist_ok=True);open(path(e[1]),'w',encoding='utf8').write(e[2])
def run(label,args,log):
    t=time.time();b=resource.getrusage(resource.RUSAGE_CHILDREN)
    with open(log,'w') as f: rc=subprocess.run(['/usr/bin/time','-l','nice','-n','10','node',*NODE,TSC,*args],cwd=P,stdout=f,stderr=subprocess.STDOUT).returncode
    w=time.time()-t;a=resource.getrusage(resource.RUSAGE_CHILDREN)
    d=[];rss=0
    with open(log) as f:
        for l in f:
            if ': error TS' in l or l.startswith('error TS'): d.append(l.rstrip())
            elif l.rstrip().endswith('maximum resident set size'): rss=int(l.split()[0])//1048576
    return {'label':label,'rc':rc,'wallS':round(w,2),'cpuS':round(a.ru_utime+a.ru_stime-b.ru_utime-b.ru_stime,2),'peakRssMb':rss,'errors':len(d)},sorted(d)
state=os.path.join(O,'state-'+variant+'.tsbuildinfo')
if os.path.exists(state) and not only: os.remove(state)
args=['--noEmit','-p','tsconfig.json']
if variant=='inc': args+=['--incremental','true','--tsBuildInfoFile',state]
elif variant=='inc-pack': args+=['--incremental','true','--tsBuildInfoFile',state,'--assumeChangesOnlyAffectDirectDependencies','false']
UNITS_CLI=os.environ.get('UNITS_CLI','')
UNITS_OUT=os.path.join(O,'units-run')
OV='{"compiler":{"forced":{"composite":true,"declaration":true,"emitDeclarationOnly":true,"noEmit":false,"incremental":true,"declarationMap":false,"sourceMap":false,"noEmitOnError":false,"types":[]},"declarationCodes":[[4000,4199],[2742,2742],[7056,7056],[9005,9006]],"extraTypes":["vitest/importMeta"]}}'
def norm(lines):
    import re
    out=set()
    for l in lines:
        m=re.match(r'^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$',l)
        if m: out.add((m.group(1).replace('\\','/').split('/')[-2:].__str__(),m.group(2),m.group(3),m.group(4),m.group(5)[:90]))
    return out
def run_units(label,log):
    t=time.time();b=resource.getrusage(resource.RUSAGE_CHILDREN)
    base=['bun',UNITS_CLI]
    with open(log,'w') as f:
        subprocess.run(['nice','-n','10',*base,'plan',R,proj+'/tsconfig.json','--out',UNITS_OUT],stdout=f,stderr=subprocess.STDOUT)
        rc=subprocess.run(['/usr/bin/time','-l','nice','-n','10',*base,'check',R,proj+'/tsconfig.json','--out',UNITS_OUT,'--session','s1','--quiet','--override',OV],stdout=f,stderr=subprocess.STDOUT).returncode
    w=time.time()-t;a=resource.getrusage(resource.RUSAGE_CHILDREN)
    info={};rss=0
    for l in open(log):
        if l.startswith('{"wallMs"'): info=json.loads(l)
        elif l.rstrip().endswith('maximum resident set size'): rss=int(l.split()[0])//1048576
    diag=[]
    dp=os.path.join(UNITS_OUT,'diagnostics-s1.txt')
    if os.path.exists(dp): diag=[l.rstrip() for l in open(dp) if '): error TS' in l]
    return {'label':label,'rc':rc,'wallS':round(w,2),'cpuS':round(a.ru_utime+a.ru_stime-b.ru_utime-b.ru_stime,2),'peakRssMb':rss,'errors':info.get('errors'),'ran':info.get('ran'),'hits':info.get('hits'),'sumRanMs':info.get('sumRanMs')},diag
rows=[];bad=0
for name,edits in STEPS:
    if only and name not in only: continue
    restore_all();apply(edits)
    if variant=='units':
        row,udiag=run_units(variant+':'+name,os.path.join(O,variant+'-'+name+'.log'));diag=udiag
    else: row,diag=run(variant+':'+name,args,os.path.join(O,variant+'-'+name+'.log'))
    row['step']=name;row['variant']=variant
    if oracle:
        orow,odiag=run('full:'+name,['--noEmit','-p','tsconfig.json'],os.path.join(O,'full-'+name+'.log'))
        row['oracle']={k:orow[k] for k in('wallS','cpuS','peakRssMb','errors')}
        if variant=='units':
            u,o=norm(diag),norm(odiag);row['same']=(u==o)
            if u!=o: bad+=1;row['onlyUnits']=sorted(u-o)[:6];row['onlyFull']=sorted(o-u)[:6]
        else: row['same']=(diag==odiag)
        if variant!='units' and diag!=odiag:
            bad+=1;row['onlyInc']=[x for x in diag if x not in odiag][:6];row['onlyFull']=[x for x in odiag if x not in diag][:6]
    print(json.dumps(row,ensure_ascii=False),flush=True)
    open(os.path.join(O,'rows-'+variant+'.jsonl'),'a').write(json.dumps(row,ensure_ascii=False)+'\n')
restore_all()
print('DONE variant',variant,'mismatches',bad,flush=True)
