#!/usr/bin/env python3
"""R100 (with --existing TEST_FILE the live test file itself is run, no copy): runs a staged test against the live tree without touching it. usage: run.py RUN_DIR TEST_FILE [--staged STAGE_DIR] [--live-dir LIVE_TEST_DIR]. With --staged the live script-inputs module is replaced (a bun resolve plugin) by STAGE_DIR/new.mjs; without it the test runs against the live code."""
import os,re,sys,subprocess,shutil
R="/Users/ueli/Documents/semio"
LIB=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SI=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
BUN=R+"/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
args=sys.argv[1:];RUN=args[0];TEST=args[1];stage=None;live=None
if "--staged" in args:stage=args[args.index("--staged")+1]
if "--live-dir" in args:live=args[args.index("--live-dir")+1]
live=live or os.path.dirname(SI)+"/🔌️provider-closures/🧪️tests/🛑️tool-boundary"
shutil.rmtree(RUN,ignore_errors=True);os.makedirs(RUN)
pat=re.compile(r'''((?:^[ \t]*(?:import|export)\b[^"'\n]*?\bfrom[ \t]*|^[ \t]*import[ \t]*|\bimport[ \t]*\([ \t]*))(["'])(\.{1,2}/[^"']*)\2''',re.M)
def absolutize(text,ld):return pat.sub(lambda m:m.group(1)+m.group(2)+os.path.normpath(os.path.join(ld,m.group(3)))+m.group(2),text)
if "--existing" in args:
    target_test=TEST
else:
    oracle=os.path.join(os.path.dirname(TEST),"oracle","oracle.py")
    if os.path.exists(oracle):shutil.copy(oracle,RUN+"/🐍️.py")
    test=absolutize(open(TEST,encoding="utf8").read(),live).replace('join(import.meta.dir,"🐍️oracle/🐍️.py")','"'+RUN+'/🐍️.py"').replace("import.meta.dir",'"'+live+'"').replace('from "esbuild"','from "'+R+'/node_modules/esbuild"')
    open(RUN+"/t.ts","w",encoding="utf8").write(test);target_test=RUN+"/t.ts"
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
cmd=[BUN,"test"]
if stage:
    open(RUN+"/si.mjs","w",encoding="utf8").write(absolutize(open(stage+"/new.mjs",encoding="utf8").read(),os.path.dirname(SI)))
    open(RUN+"/plug.ts","w",encoding="utf8").write('import {plugin} from "bun";import {dirname,resolve} from "node:path";const live=%s,staged=%s;plugin({name:"r100",setup(build){build.onResolve({filter:/\\.mjs$/},args=>{if(!args.path.startsWith(".")&&!args.path.startsWith("/"))return;const full=args.path.startsWith("/")?args.path:resolve(dirname(args.importer),args.path);if(full===live)return{path:staged};});}});'%(repr(SI).replace("'",'"'),repr(RUN+"/si.mjs").replace("'",'"')))
    cmd+=["--preload",RUN+"/plug.ts"]
cmd+=[target_test,"--timeout","180000"]
r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True)
print((r.stdout+r.stderr)[-7000:]);print("EXIT",r.returncode)
