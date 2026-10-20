#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
python3 - "$@" <<'P'
import sys,json,os
sys.path.insert(0,os.environ.get("C",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN")+"/law-drift-tools")
from stage import load,apply_text,ROOT
dry="--dry-run" in sys.argv
bad=0
for r in load():
    if r['id'] in os.environ.get('SKIP','').split(','): print('SKIPPED',r['id']); continue
    if r['file'].endswith('project.json') and 'ALLOW_PROJECT' not in os.environ: print('SKIPPED (project.json frozen)',r['id']); continue
    p=ROOT+"/"+r['file']
    s=open(p,encoding='utf-8').read()
    try: out=apply_text(s,[tuple(x) for x in r['pairs']])
    except SystemExit as e: print("SKIP",r['id'],e); bad+=1; continue
    if out==s: print("DONE ",r['id']); continue
    if dry: print("WOULD",r['id'],r['file'][-70:])
    else: open(p,'w',encoding='utf-8').write(out); print("APPLIED",r['id'])
sys.exit(1 if bad else 0)
P
