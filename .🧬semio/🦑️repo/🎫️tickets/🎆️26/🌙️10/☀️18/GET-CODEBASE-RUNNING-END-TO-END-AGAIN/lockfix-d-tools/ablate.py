#!/usr/bin/env python3
"""ablate.py <staged-scratch> <scope> <hypothesis...>: copy a staged oracle scratch, re-add removed deps, frozen dry-run with the pinned Bun.
hypothesis = '<member-suffix>|<section>|<name>|<range>'"""
import json,os,shutil,subprocess,sys
src,scope=sys.argv[1],sys.argv[2];hyp=[h.split('|') for h in sys.argv[3:]]
dst=src+'.ab';shutil.rmtree(dst,ignore_errors=True);shutil.copytree(src,dst,symlinks=True)
for suffix,section,name,rng in hyp:
    hits=[os.path.join(d,'package.json') for d,_,f in os.walk(dst) if 'package.json' in f and (d+'/').endswith(suffix+'/') and '.bun-cache' not in d and 'node_modules' not in d]
    assert len(hits)==1,(suffix,hits)
    doc=json.load(open(hits[0]));doc.setdefault(section,{})[name]=rng;json.dump(doc,open(hits[0],'w'))
r=subprocess.run([os.environ['BUN_BIN'],'install','--frozen-lockfile','--dry-run','--ignore-scripts','--cache-dir',os.path.join(src,'.bun-cache')],cwd=os.path.join(dst,scope),capture_output=True,text=True)
print(sys.argv[3:] ,'rc',r.returncode,(r.stdout+r.stderr).strip().splitlines()[-2:])
shutil.rmtree(dst,ignore_errors=True)
