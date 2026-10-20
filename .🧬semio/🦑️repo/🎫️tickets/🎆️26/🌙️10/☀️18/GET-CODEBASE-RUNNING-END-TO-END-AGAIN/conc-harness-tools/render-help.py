#!/usr/bin/env python3
"""Renders the help of the live and of the wire-patched catalogue in an overlay of COPIES with the repository's own help renderer and derives help-snapshots.json: for every snapshot key whose rendering changes the old and the new lines, plus the new pages. Fails when the live rendering differs from the live help fixture anywhere (the fixture would not describe the baseline)."""
import json,os,shutil,subprocess,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
OPS=wire.OPS;root="/private/tmp/conc-harness/help";shutil.rmtree(root,ignore_errors=True)
def stage(name,patch):
    for rel in (f"{OPS}/{names.CATALOG}",f"{OPS}/{names.HELP}/{names.TS}"):
        dst=os.path.join(root,name,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(os.path.join(names.R,rel),dst)
    if patch:
        cat=os.path.join(root,name,OPS,names.CATALOG);text=open(cat,encoding="utf8").read();open(cat,"w",encoding="utf8").write(wire.catalog(text))
    return os.path.join(root,name)
fixture=json.load(open(os.path.join(names.R,OPS,names.TESTS,names.FIXTURES,names.HELP,names.JSON),encoding="utf8"))["snapshots"]
keys=list(fixture["en"].keys())
new_keys=["concurrency","concurrency list","concurrency plan","concurrency run"]
def render(base,wanted):
    helpts=os.path.join(base,OPS,names.HELP,names.TS);cat=os.path.join(base,OPS,names.CATALOG);runner=os.path.join(base,"render.ts")
    open(runner,"w").write('''import {renderBuildHelpV1,readBuildHelpRequestV1} from "%s";import catalog from "%s" with {type:"json"};
const keys=%s,out:Record<string,Record<string,string[]>>={};
for(const lang of catalog.languages){out[lang]={};for(const name of keys){const args=name==="root"?[]:name.split(" ");out[lang][name]=renderBuildHelpV1(catalog as never,readBuildHelpRequestV1(catalog as never,args),lang).split("\\n");}}
console.log(JSON.stringify(out));'''%(helpts,cat,json.dumps(wanted)))
    run=subprocess.run(["bun",runner],capture_output=True,text=True,cwd=base)
    if run.returncode:sys.exit(run.stderr)
    return json.loads(run.stdout)
old=render(stage("old",False),keys);new=render(stage("new",True),keys+new_keys)
result={}
for lang in old:
    for key in keys:
        if old[lang][key]!=fixture[lang][key]:sys.exit(f"{lang}/{key}: the live fixture differs from the live rendering")
    changed={key:{"old":old[lang][key],"new":new[lang][key]} for key in keys if new[lang][key]!=old[lang][key]}
    result[lang]={"changed":changed,"pages":{key:new[lang][key] for key in new_keys}}
    print(lang,"changed:",sorted(changed),"pages:",new_keys)
json.dump(result,open(os.path.join(here,"help-snapshots.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
shutil.rmtree(root,ignore_errors=True)
