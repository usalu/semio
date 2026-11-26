#!/usr/bin/env python3
"""F66: type-checks a handful of files (plus their import closure) with the library compiler options, seconds instead of minutes. Usage: fast_tsc.py <out.txt> <file>..."""
import json,os,subprocess,sys
sys.path.insert(0,os.path.dirname(__file__))
from lib import TSCDIR
out=sys.argv[1];files=[os.path.abspath(f) for f in sys.argv[2:]]
root="/Users/ueli/Documents/semio"
scratch=os.path.dirname(out)
cfg={"extends":root+"/tsconfig.json","compilerOptions":{"allowJs":True,"noEmit":True,"incremental":False,"composite":False,"tsBuildInfoFile":None,"typeRoots":[root+"/node_modules","root"+"/node_modules/@types"] if False else [root+"/node_modules",root+"/node_modules/@types"],"types":["bun-types","node"]},"files":[TSCDIR+"/bun-test-matchers.d.ts"]+files}
cfg["compilerOptions"].pop("tsBuildInfoFile")
p=os.path.join(scratch,"tsconfig.fast.json");json.dump(cfg,open(p,"w"))
env=dict(os.environ,NODE_OPTIONS="--max-old-space-size=8000")
r=subprocess.run([root+"/node_modules/.bin/tsc","-p",p],capture_output=True,text=True,env=env)
open(out,"w",encoding="utf8").write(r.stdout+r.stderr)
print("errors",r.stdout.count("error TS"))
