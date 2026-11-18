import sys,os
ROOT="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
S=L+"🧪️testing/🔬️slice-lab/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R149/backup-labtools"
files={
S+"🟦️.ts":[
('import type {ReadyStageV1} from "../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";','import {READY_STAGES_V1,type ReadyStageV1} from "../../🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts";'),
('workspaceArguments?:readonly string[];execute?:boolean;','workspaceArguments?:readonly string[];execute?:boolean;tools?:boolean;'),
(' const skipped=P.skippedStages.map(name=>({name,kind:"repo" as const,','const skipped=P.skippedStages.map(name=>options.tools===true&&name==="tools"?READY_STAGES_V1.find(stage=>stage.name===name)!:({name,kind:"repo" as const,'),
('${t("skipped",{stages:P.skippedStages.join(", ")})}','${t("skipped",{stages:P.skippedStages.filter(name=>!(options.tools===true&&name==="tools")).join(", ")})}'),
],
S+"🧬️schema/🔣️.json":[
('        "execute": {"type": "boolean"},\n','        "execute": {"type": "boolean"},\n        "tools": {"type": "boolean"},\n'),
('[--execute] [--stagger=<seconds>|-1] [--out=<dir>] [--lang=en|de]"','[--execute] [--tools] [--stagger=<seconds>|-1] [--out=<dir>] [--lang=en|de]"'),
('[--execute] [--stagger=<Sekunden>|-1] [--out=<dir>] [--lang=en|de]"','[--execute] [--tools] [--stagger=<Sekunden>|-1] [--out=<dir>] [--lang=en|de]"'),
],
S+"📜️script.ts":[
('[--execute] [--stagger=<seconds>|-1]','[--execute] [--tools] [--stagger=<seconds>|-1]'),
('...argv.includes("--execute")?{execute:true}:{},','...argv.includes("--execute")?{execute:true}:{},...argv.includes("--tools")?{tools:true}:{},'),
],
S+"🧪️tests/🟦️.ts":[
(' test("the verdict reports the owners a machine had at most',' test("--tools keeps the real tools stage of the readiness plan instead of replacing it by --help",()=>{\n  const real=sliceLabStagesV1(readSliceLabOptionsV1({...options,tools:true}));\n  expect(real.map(stage=>stage.name)).toEqual(["native","repo-mcp","dependencies","tools","workspace"]);\n  expect(real[3]!.arguments).toEqual(["tools","setup","all"]);\n  expect(real.filter((_,at)=>at<3).every(stage=>stage.arguments.join(" ")==="--help")).toBe(true);\n  expect(sliceLabStagesV1(options)[3]!.arguments).toEqual(["--help"]);\n  expect(()=>readSliceLabOptionsV1({...options,tools:"yes"})).toThrow();\n });\n test("the verdict reports the owners a machine had at most'),
],
}
mode=sys.argv[1]
os.makedirs(SC,exist_ok=True)
root=sys.argv[2] if len(sys.argv)>2 else ROOT
for rel,pairs in files.items():
    path=os.path.join(root,rel);s=open(path,encoding="utf8").read()
    if mode=="apply":
        open(os.path.join(SC,rel.replace("/","_")),"w",encoding="utf8").write(s)
        for a,b in pairs:
            assert s.count(a)==1,("anchor",rel,a[:70],s.count(a));s=s.replace(a,b)
    else:
        for a,b in pairs:
            assert s.count(b)==1,("anchor",rel,b[:70],s.count(b));s=s.replace(b,a)
    open(path,"w",encoding="utf8").write(s)
print(mode,"ok")
