#!/usr/bin/env python3
"""Renders the help of the patched catalogue in an overlay of COPIES with the repository's own help renderer and derives help-snapshots.json: the rows `wire.py` inserts into the help fixture. Fails when the rendering differs from the old snapshots anywhere but in the expected places."""
import difflib,json,os,shutil,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
ov="/private/tmp/bzlincr/help";shutil.rmtree(ov,ignore_errors=True)
OPS=wire.OPS
for rel in (f"{OPS}/{names.CATALOG}",f"{OPS}/{names.n(0x1F4D6,'help')}/{names.TS}"):
    dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(os.path.join(names.R,rel),dst)
cat=os.path.join(ov,OPS,names.CATALOG);text=open(cat,encoding="utf8").read();open(cat,"w",encoding="utf8").write(wire.catalog(text))
helpts=os.path.join(ov,OPS,names.n(0x1F4D6,'help'),names.TS)
runner=os.path.join(ov,"render.ts")
open(runner,"w").write('''import {renderBuildHelpV1,readBuildHelpRequestV1} from "%s";import catalog from "%s" with {type:"json"};
const out:Record<string,Record<string,string[]>>={};
for(const lang of catalog.languages){out[lang]={};for(const name of ["root","incremental","incremental list","incremental plan","incremental prepare","incremental run","incremental discard"]){const args=name==="root"?[]:name.split(" ");out[lang][name]=renderBuildHelpV1(catalog as never,readBuildHelpRequestV1(catalog as never,args),lang).split("\\n");}}
console.log(JSON.stringify(out));'''%(helpts,cat))
run=subprocess.run(["bun",runner],capture_output=True,text=True,cwd=ov)
if run.returncode:sys.exit(run.stderr)
new=json.loads(run.stdout)
old=json.load(open(os.path.join(names.R,OPS,names.TESTS,names.FIXTURES,names.n(0x1F4D6,"help"),names.JSON),encoding="utf8"))["snapshots"]
result={}
for lang in new:
    o=old[lang]["root"];n=new[lang]["root"]
    oldset=set(o);fresh=[l for l in n if l not in oldset]
    cmd=[l for l in fresh if l.startswith("  incremental")];flags=("--scenario","--implementations","--root","--phases","--fresh")
    options=[l for l in fresh if l.strip().startswith(flags)];at=[i for i,l in enumerate(n) if l.startswith("  repo incremental")]
    changed=[l for l in fresh if l.startswith(("  --file=<path>","  --json "))]
    if len(cmd)!=1 or len(options)!=5 or len(at)!=1 or len(changed)!=2:sys.exit(f"{lang}: unexpected shape {len(cmd)} {len(options)} {len(at)} {len(changed)}")
    examples=n[at[0]:at[0]+2];used=[]
    for line in changed:
        prefix="  "+line.split()[0];before=[l for l in o if l.startswith(prefix)][0];used.append([prefix,before[before.rindex("("):],line[line.rindex("("):]])
    stripped=[l for l in o if l not in set(n)]
    if len(stripped)!=2:sys.exit(f"{lang}: unexpected removals {stripped}")
    result[lang]={"commandAfter":"  preflight ","commandRow":cmd[0],"optionBefore":"  --scope <owner>","optionRows":options,"used":used,"exampleAfter":"  repo preflight","exampleRows":examples,
      "pages":{k:v for k,v in new[lang].items() if k!="root"}}
json.dump(result,open(os.path.join(here,"help-snapshots.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
print("ok",{l:list(v["pages"]) for l,v in result.items()})
shutil.rmtree(ov,ignore_errors=True)
