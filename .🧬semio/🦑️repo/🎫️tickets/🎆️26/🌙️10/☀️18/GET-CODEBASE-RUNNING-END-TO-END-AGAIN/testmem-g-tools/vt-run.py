#!/usr/bin/env python3
"""vt-run.py <label> <out.json> <root> <config> <workers|default>: build the vitest command through the repo policy (vt-cmd.ts), run it under tree-rss.py."""
import json,subprocess,sys,os
label,out,root,config,workers=sys.argv[1:6]
here=os.path.dirname(os.path.abspath(__file__))
cmd=json.loads(subprocess.run(['bun',os.path.join(here,'vt-cmd.ts'),root,config,workers],capture_output=True,text=True,check=True).stdout)
env=[]
for k,v in cmd['env'].items(): env+=['--env',k+'='+v]
env+=['--env','SEMIO_TEST_LEVEL=quick']
os.execvp('python3',['python3',os.path.join(here,'tree-rss.py'),label,out,'--cwd',cmd['cwd'],*env,'--',*cmd['argv']])
