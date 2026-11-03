import sys,re,os,time,glob
ROOT='/Users/ueli/Documents/semio'
apply=sys.argv[1]=='--apply'
MODE=sys.argv[2]
EMIT=sys.argv[3]
files=sys.argv[4:]
ROOTS={'semio-s-artifact-process-process3d-editor','semio-s-artifact-sourcing-curation','semio-s-artifact-gis-gismap-gis2d-unit-tests','semio-s-artifact-cad-cad','semio-s-artifact-puzzle-3d-editor','semio-s-artifact-fem-3d-editor','semio-s-artifact-energy-model-editor-mounted-tests','semio-s-composition-laws'}
fam=lambda n:(re.match(r'semio-s-artifact-stdio-([a-z0-9]+)',n) or [0,None])[1]
plan={}
for f in files:
    own=None;odir=None
    for l in open(f,encoding='utf8'):
        l=l.rstrip('\n')
        if l.startswith('semio'):
            own,odir=l.split(' ',1)
            continue
        p=l.split()
        if len(p)<2: continue
        sec,dep=p[0],p[1]
        ostd=own.startswith('semio-s-artifact-stdio-')
        if 'dev-dependencies' in sec and own not in ROOTS: continue
        if ostd and MODE=='A': continue
        if ostd and MODE=='AS' and not own.startswith('semio-s-artifact-stdio-semio-'): continue
        if ostd:
            if fam(own)=='pdf' or fam(own)==fam(dep) or fam(dep) in('contract','binary'): continue
        plan.setdefault((own,odir),set()).add((sec,dep))
tot=0;skipped=[];em=open(EMIT,'w')
for (own,odir),items in sorted(plan.items()):
    path=os.path.join(ROOT,odir,'Cargo.toml')
    if time.time()-os.path.getmtime(path)<600 and not os.environ.get('FORCE'): skipped.append((own,'recent mtime'));continue
    lines=open(path,encoding='utf8').read().split('\n')
    sec=None;out=[];removed=[];bad=False
    want=items
    for ln in lines:
        h=re.match(r'\[(.*)\]\s*$',ln)
        if h: sec=h.group(1)
        m=re.match(r'([A-Za-z0-9_-]+)\s*=',ln)
        if m and sec is not None and (sec,m.group(1)) in want:
            if ln.count('{')!=ln.count('}') or ln.rstrip().endswith(('[',',','(')) : bad=True
            removed.append((sec,m.group(1)));continue
        out.append(ln)
    if bad or set(removed)!=set(want): skipped.append((own,'mismatch/multiline %s'%(set(want)-set(removed))));continue
    rest='\n'.join(out)
    for _,dep in removed:
        for ln in out:
            if re.match(r'\s*[A-Za-z0-9_-]+\s*=',ln) and re.match(r'\s*'+re.escape(dep)+r'\s*=',ln): continue
            if re.search(r'(?<![A-Za-z0-9_-])'+re.escape(dep)+r'(?![A-Za-z0-9_-])',ln) and not ln.strip().startswith('#'):
                bad=True;skipped.append((own,'dep still referenced: %s | %s'%(dep,ln.strip()[:80])));break
        if bad:break
    if bad: continue
    tot+=len(removed)
    for _,dep in removed: em.write(own+' '+dep+'\n')
    print(('APPLY ' if apply else 'PLAN  ')+own,len(removed))
    if apply: open(path,'w',encoding='utf8').write('\n'.join(out))
print('total deps',tot,'crates',len(plan)-len(skipped))
for s in skipped: print('SKIP',s)
