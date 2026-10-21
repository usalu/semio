#!/usr/bin/env python3
"""Regenerates the help snapshot fixture of the operations catalogue with the repository's own renderer, after the staged set is applied (or in an overlay: --root). It keeps every existing page, adds the pages the catalogue now has, and refuses when a page changed in any way other than the expected ones: the tool alternation, the default wording of --implementations, and the new conformance rows. Dry run by default; --write replaces the fixture."""
import argparse,difflib,json,os,re,subprocess,sys,tempfile
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names
ap=argparse.ArgumentParser();ap.add_argument("--root",default=names.R);ap.add_argument("--write",action="store_true");a=ap.parse_args()
OPS=os.path.join(a.root,names.B,names.T["OPERATIONS"]);fixture=os.path.join(OPS,names.T["TESTS"],names.T["FIXTURES"],names.T["HELP"],names.JSON)
runner=os.path.join(tempfile.mkdtemp(prefix="regen-help-"),"render.ts")
open(runner,"w",encoding="utf8").write('''import {renderBuildHelpV1,readBuildHelpRequestV1} from %s;import catalog from %s with {type:"json"};
const old=JSON.parse(await Bun.file(%s).text()) as {snapshots:Record<string,Record<string,string[]>>};
const pages=new Set<string>(["root"]);
for(const operation of catalog.operations){pages.add(operation.name);for(const sub of (operation as {subcommands?:{name:string}[]}).subcommands??[])pages.add(operation.name+" "+sub.name);}
for(const lang of Object.keys(old.snapshots))for(const key of Object.keys(old.snapshots[lang]!))pages.add(key);
const out:Record<string,Record<string,string[]>>={};
for(const lang of catalog.languages){out[lang]={};for(const name of [...pages]){const args=name==="root"?[]:name.split(" ");try{out[lang]![name]=renderBuildHelpV1(catalog as never,readBuildHelpRequestV1(catalog as never,args),lang).split("\\n");}catch{}}}
console.log(JSON.stringify(out));'''%(json.dumps(os.path.join(OPS,names.T["HELP"],names.TS)),json.dumps(os.path.join(OPS,names.n(0x1F523,"catalog.json"))),json.dumps(fixture)))
run=subprocess.run(["nice","-n","10","bun",runner],capture_output=True,text=True,cwd=a.root)
if run.returncode:sys.exit(run.stderr[-2000:])
new=json.loads(run.stdout);old=json.load(open(fixture,encoding="utf8"))
expected_words=("nx|bazel","nx,bazel","semio","Konformit","Beurteilt","Judge","default: both","Standard: beide","conformance","--keep","--scenario","--implementations","--file","--json")
unexpected=[]
for lang,pages in new.items():
    for page,lines in pages.items():
        before=old["snapshots"].get(lang,{}).get(page)
        if before is None:continue
        flat=lambda rows:[re.sub(r"\s+"," ",row).strip() for row in rows]
        for line in difflib.unified_diff(flat(before),flat(lines),lineterm="",n=0):
            if line.startswith(("---","+++","@@")):continue
            if not any(w in line for w in expected_words):unexpected.append(f"{lang}/{page}: {line[:140]}")
print(f"pages: {sum(len(p) for p in new.values())}; unexpected differences: {len(unexpected)}")
for row in unexpected[:20]:print(" ",row)
if unexpected:sys.exit(2)
merged={"version":old["version"],"snapshots":{lang:{key:pages[key] for key in pages} for lang,pages in new.items()}}
if a.write:
    if not os.path.realpath(fixture).startswith(os.path.realpath(a.root)+os.sep):sys.exit('REFUSED: the fixture resolves outside the root (a symlinked overlay?)')
    open(fixture,"w",encoding="utf8").write(json.dumps(merged,ensure_ascii=False,indent=2)+"\n");print("written",fixture[-60:])
else:print("dry run: pass --write")
