#!/usr/bin/env python3
"""Builds /private/tmp/hubbzl/{base,patched}: copies of the live Bazel adapter directory whose imports that leave it point at the live files by absolute path; `patched` carries the staged Go set. Copies only, no symlinks."""
import glob,os,re,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import patches
os.chdir('/Users/ueli/Documents/semio')
ROOT=os.getcwd()
BZ=glob.glob('🧰*/🛍*/🦑*/🔨*/📚*/🏗️build/🔧*/🏗*')[0]
NAME=os.path.basename(BZ)
SPEC=re.compile(r'''((?:from\s*|import\s*\(\s*|import\s+)["'])(\.{1,2}/[^"']+)(["'])''')
def rewrite(text,origin_dir):
    def sub(m):
        target=os.path.normpath(os.path.join(origin_dir,m.group(2)))
        inside=os.path.commonpath([os.path.join(ROOT,target),os.path.join(ROOT,BZ)])==os.path.join(ROOT,BZ)
        return m.group(0) if inside else m.group(1)+os.path.join(ROOT,target)+m.group(3)
    return SPEC.sub(sub,text)
def build(kind):
    dest=f'/private/tmp/hubbzl/{kind}'
    shutil.rmtree(dest,ignore_errors=True); os.makedirs(dest)
    out=os.path.join(dest,NAME); shutil.copytree(BZ,out)
    if kind=='patched':
        shutil.copytree(os.path.join(HERE,'staged',BZ,'🐹️go'),os.path.join(out,'🐹️go'))
        p=os.path.join(out,'📜️script.ts'); text=open(p,encoding='utf8').read(); open(p,'w',encoding='utf8').write(patches.apply(text))
    if kind=='patched':
        t=os.path.join(out,'🧪️tests','📥️fingerprint','🟦️.ts'); text=open(t,encoding='utf8').read(); open(t,'w',encoding='utf8').write(patches.apply_fingerprint_test(text))
    for d,_,fs in os.walk(out):
        for f in fs:
            if f.endswith(('.ts','.mjs','.tsx')):
                path=os.path.join(d,f); rel=os.path.relpath(d,out)
                origin=os.path.normpath(os.path.join(BZ,rel))
                text=open(path,encoding='utf8').read(); new=rewrite(text,origin)
                if new!=text: open(path,'w',encoding='utf8').write(new)
    return out
for kind in ('base','patched'): print(build(kind))
