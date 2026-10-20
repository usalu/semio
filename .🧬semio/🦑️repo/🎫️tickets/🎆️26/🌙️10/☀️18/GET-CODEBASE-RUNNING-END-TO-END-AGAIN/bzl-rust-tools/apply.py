#!/usr/bin/env python3
"""Apply (or --dry-run) the staged Bazel Cargo adapter patch set; refuses on anchor drift or when the staged module is not in place yet."""
import glob,os,shutil,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import patches
C=os.path.dirname(os.path.dirname(os.path.abspath(__file__))) if False else os.path.dirname(os.path.abspath(__file__))
root=os.getcwd()
B=glob.glob('🧰*/🛍*/🦑*/🔨*/📚*/🏗*/🔧*/🏗*')[0]
script=os.path.join(B,'📜️script.ts')
dry='--dry-run' in sys.argv
staged=glob.glob(os.path.join(C,'staged','🧰*','**','🦀️cargo'),recursive=True)
if len(staged)!=1: raise SystemExit('staged module directory not found')
text=open(script,encoding='utf8').read()
new=patches.apply(text)
dest=os.path.join(B,patches.CRAB+'cargo')
if os.path.exists(dest) and not dry and '--force' not in sys.argv: raise SystemExit('module directory exists: '+dest)
print('patch ok:',len(text),'->',len(new),'bytes;',len(patches.anchors()),'anchors')
if dry: print('dry run: would copy',staged[0],'->',dest); sys.exit(0)
shutil.copytree(staged[0],dest,dirs_exist_ok=True)
open(script,'w',encoding='utf8').write(new)
print('applied')
