#!/usr/bin/env python3
"""Applies the staged hub set to the live checkout: the Go toolchain module (new directory) and two anchored edits of the Bazel adapter. Dry run by default; --write applies; refuses on drift (existing module differs, anchor missing or ambiguous) and writes nothing unless every check passes."""
import glob,hashlib,json,os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
import patches
ROOT='/Users/ueli/Documents/semio'
os.chdir(ROOT)
BZ=glob.glob('🧰*/🛍*/🦑*/🔨*/📚*/🏗️build/🔧*/🏗*')[0]
STAGED=os.path.join(HERE,'staged',BZ)
MODULE='🐹️go'
write='--write' in sys.argv
sha=lambda b:hashlib.sha256(b).hexdigest()
def tree(path):
    return {os.path.relpath(os.path.join(d,f),path):sha(open(os.path.join(d,f),'rb').read()) for d,_,fs in os.walk(path) for f in fs}
script=os.path.join(BZ,'📜️script.ts')
text=open(script,encoding='utf8').read()
patched=patches.apply(text) if 'bazelGoClassV1' not in text else text
test=os.path.join(BZ,'🧪️tests','📥️fingerprint','🟦️.ts')
test_text=open(test,encoding='utf8').read()
test_patched=patches.apply_fingerprint_test(test_text) if 'bazelGoClassV1' not in test_text else test_text
dest=os.path.join(BZ,MODULE)
source=tree(os.path.join(STAGED,MODULE))
if os.path.exists(dest) and tree(dest)!=source: raise SystemExit('module directory exists and differs: '+dest)
longest=max(len(os.path.join(dest,p)) for p in source)
if longest>=256: raise SystemExit(f'path budget exceeded: {longest}')
print(json.dumps({'adapter':'already patched' if patched is text else f'{len(text)} -> {len(patched)} bytes','fingerprintTest':'already patched' if test_patched is test_text else 'two bindings','anchors':patches.anchors(),'module':'present' if os.path.exists(dest) else f'{len(source)} files to copy','longestPath':longest,'mode':'write' if write else 'dry run'},ensure_ascii=False))
if not write: sys.exit(0)
if not os.path.exists(dest): shutil.copytree(os.path.join(STAGED,MODULE),dest)
if patched is not text: open(script,'w',encoding='utf8').write(patched)
if test_patched is not test_text: open(test,'w',encoding='utf8').write(test_patched)
print('applied')
