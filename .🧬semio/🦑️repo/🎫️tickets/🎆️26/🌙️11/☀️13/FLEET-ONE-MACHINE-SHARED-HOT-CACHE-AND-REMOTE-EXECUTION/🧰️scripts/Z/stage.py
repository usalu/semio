#!/usr/bin/env python3
"""Z/src holds the real files (relative imports, landing form). stage.py writes Z/stage/<same path> where every relative import that does not point at another Z/src file is rewritten to the live absolute path. Nothing outside Z is touched."""
import os,re,sys,shutil,subprocess
Z=os.path.dirname(os.path.abspath(__file__))
SRC=os.path.join(Z,'src');STAGE=os.path.join(Z,'stage');LIVE='/Users/Shared/semio'
if os.path.realpath(STAGE).startswith(LIVE):sys.exit('stage under live tree')
files=[]
for r,_,fs in os.walk(SRC):
    for f in fs:files.append(os.path.relpath(os.path.join(r,f),SRC))
fileset=set(files)
BUN='/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun'
bare={}
def resolve_bare(spec):
    if spec not in bare:
        r=subprocess.run([BUN,'-e','console.log(Bun.resolveSync(%r,%r))'%(spec,LIVE)],capture_output=True,text=True)
        bare[spec]=r.stdout.strip() if r.returncode==0 else None
    return bare[spec]
patb=re.compile(r'''((?:from|import)\s*\(?\s*["'])([A-Za-z@][^"':]*)(["'])''')
pat=re.compile(r'''((?:from|import)\s*\(?\s*["'])(\.{1,2}/[^"']*)(["'])''')
patu=re.compile(r'''(new URL\(\s*["'])(\.{1,2}/[^"']*)(["']\s*,\s*import\.meta\.url)''')
if os.path.isdir(STAGE):
    assert os.path.realpath(STAGE).startswith(os.path.realpath(Z)+os.sep)
    shutil.rmtree(STAGE)
for rel in files:
    text=open(os.path.join(SRC,rel),encoding='utf8').read()
    d=os.path.dirname(rel)
    def sub(m):
        target=os.path.normpath(os.path.join(d,m.group(2)))
        if target in fileset:return m.group(0)
        return m.group(1)+os.path.normpath(os.path.join(LIVE,d,m.group(2)))+m.group(3)
    if rel.endswith('.ts'):
        text=pat.sub(sub,text)
        def subu(m):
            target=os.path.normpath(os.path.join(d,m.group(2)))
            if target in fileset or target+'/' in fileset:return m.group(0)
            return m.group(1)+'file://'+os.path.normpath(os.path.join(LIVE,d,m.group(2)))+m.group(3)
        text=patu.sub(subu,text)
        text=patb.sub(lambda m:(m.group(1)+resolve_bare(m.group(2))+m.group(3)) if resolve_bare(m.group(2)) else m.group(0),text)
    out=os.path.join(STAGE,rel);os.makedirs(os.path.dirname(out),exist_ok=True)
    open(out,'w',encoding='utf8').write(text)
print(len(files),'files staged')
