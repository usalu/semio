import sys,os
base=os.path.join(os.path.dirname(os.path.abspath(__file__)),'real','🔬️slice-lab')
import shutil as _s
LIVE='/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab'
_s.rmtree(base,ignore_errors=True); _s.copytree(LIVE,base)
for rel in ['🧊️frozen','🧪️tests/🧊️frozen']:
    src=os.path.join(os.path.dirname(os.path.abspath(__file__)),'new',rel); dst=os.path.join(base,rel); _s.rmtree(dst,ignore_errors=True); _s.copytree(src,dst)
import hashlib,json
BEFORE={r:hashlib.sha256(open(os.path.join(LIVE,r),'rb').read()).hexdigest() for r in ['🟦️.ts','📜️script.ts','🧬️schema/🔣️.json']}
json.dump(BEFORE,open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'before.json'),'w'))
def edit(rel,pairs):
    p=os.path.join(base,rel); t=open(p,encoding='utf8').read()
    for a,b in pairs:
        assert t.count(a)==1,(rel,a[:60],t.count(a))
        t=t.replace(a,b)
    open(p,'w',encoding='utf8').write(t)
edit('🧬️schema/🔣️.json',[
('"loopLagLimitMilliseconds": 200\n','"loopLagLimitMilliseconds": 200,\n        "frozen": {"exclude": [".🧬semio/🦑️repo/🎫️tickets"], "include": [], "directory": "r", "author": "slice-lab"}\n'),
('"execute": {"type": "boolean"},','"execute": {"type": "boolean"},\n        "frozen": {"type": "boolean"},\n        "frozenDirectory": {"type": "string", "minLength": 1},\n        "edit": {"type": "string", "minLength": 1},'),
('"usage": "usage: slice-lab run [--machines=1|2] [--memory=16384|8192] [--cores=8] [--slice=a,b,...] [--target=build] [--timeout=1800] [--execute] [--stagger=<seconds>|-1]',
 '"frozen": "frozen view: {files} files ({megabytes} MB logical, cloned in {seconds} s without copying bytes where the file system clones); {drift} files of the live tree changed since the freeze and stay invisible to the machines",\n          "frozenChurn": "files of the frozen view that changed during the run (must be 0): {count}{sample}",\n          "usage": "usage: slice-lab run [--machines=1|2] [--memory=16384|8192] [--cores=8] [--slice=a,b,...] [--target=build] [--timeout=1800] [--execute] [--frozen[=<dir>]] [--edit=<file>] [--stagger=<seconds>|-1]'),
('"usage": "Aufruf: slice-lab run [--machines=1|2] [--memory=16384|8192] [--cores=8] [--slice=a,b,...] [--target=build] [--timeout=1800] [--execute] [--stagger=<Sekunden>|-1]',
 '"frozen": "eingefrorene Sicht: {files} Dateien ({megabytes} MB logisch, in {seconds} s geklont ohne Bytes zu kopieren, wo das Dateisystem klont); {drift} Dateien des Live-Baums haben sich seit dem Einfrieren geändert und bleiben für die Maschinen unsichtbar",\n          "frozenChurn": "Dateien der eingefrorenen Sicht, die sich während des Laufs geändert haben (muss 0 sein): {count}{sample}",\n          "usage": "Aufruf: slice-lab run [--machines=1|2] [--memory=16384|8192] [--cores=8] [--slice=a,b,...] [--target=build] [--timeout=1800] [--execute] [--frozen[=<dir>]] [--edit=<file>] [--stagger=<Sekunden>|-1]'),
])
edit('🟦️.ts',[
('import {fileURLToPath} from "node:url";\n','import {fileURLToPath} from "node:url";\nimport {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1} from "./🧊️frozen/🟦️.ts";\n'),
('execute?:boolean;stagger?:number;','execute?:boolean;frozen?:boolean;frozenDirectory?:string;edit?:string;stagger?:number;'),
('detached?:number;churn?:Readonly<{count:number;sample:readonly string[]}>}>):SliceLabVerdictV1{','detached?:number;churn?:Readonly<{count:number;sample:readonly string[]}>;frozen?:Readonly<{files:number;megabytes:number;seconds:number;drift:number}>}>):SliceLabVerdictV1{'),
('churn:Readonly<{count:number;sample:readonly string[]}>;readySeconds','churn:Readonly<{count:number;sample:readonly string[]}>;frozen:Readonly<{files:number;megabytes:number;seconds:number;drift:number}>|null;readySeconds'),
('&&input.leftoverImages===0,readyAtSeconds','&&input.leftoverImages===0&&(input.frozen===undefined||(input.churn?.count??0)===0),readyAtSeconds'),
('churn:input.churn??{count:0,sample:[]}});','churn:input.churn??{count:0,sample:[]},frozen:input.frozen??null});'),
('`- ${t("churn",{count:verdict.churn?.count??0,sample:(verdict.churn?.sample.length??0)>0?": "+verdict.churn!.sample.join(", "):""})}`','`- ${t(verdict.frozen===null||verdict.frozen===undefined?"churn":"frozenChurn",{count:verdict.churn?.count??0,sample:(verdict.churn?.sample.length??0)>0?": "+verdict.churn!.sample.join(", "):""})}`,...verdict.frozen===null||verdict.frozen===undefined?[]:[`- ${t("frozen",verdict.frozen)}`]'),
])
edit('🟦️.ts',[
('const marker=join(work,"start.marker");writeFileSync(marker,"");\n const plans=sliceLabPlansV1(options,id,work,resolve(root)),',
 'const live=resolve(root),driftMarker=join(work,"start.marker");writeFileSync(driftMarker,"");\n const frozen=options.frozen===true||options.frozenDirectory!==undefined?(say("freezing the tree / Baum wird eingefroren"),options.frozenDirectory===undefined?freezeSliceLabRootV1(live,join(work,P.frozen.directory)):frozenRootV1(live,options.frozenDirectory)):undefined,view=frozen?.root??live,marker=frozen===undefined?driftMarker:join(work,"frozen.marker");writeFileSync(marker,"");\n if(frozen!==undefined)say(`frozen view ${frozen.files} files, ${frozen.megabytes} MB, ${frozen.seconds} s / eingefrorene Sicht ${frozen.files} Dateien`);\n const plans=sliceLabPlansV1(options,id,work,view),'),
('for(const path of openFiles(rows.map(row=>row.pid),resolve(root)))if(leaks.size<20)leaks.add(path);','for(const path of[...openFiles(rows.map(row=>row.pid),live),...frozen===undefined?[]:openFiles(rows.map(row=>row.pid),view)])if(leaks.size<20)leaks.add(path);'),
('churn:treeChurnV1(root,marker)});','churn:treeChurnV1(view,marker,options.edit===undefined?[]:[options.edit]),...frozen===undefined?{}:{frozen:{files:frozen.files,megabytes:frozen.megabytes,seconds:frozen.seconds,drift:treeChurnV1(live,driftMarker).count}}});'),
])
edit('📜️script.ts',[
('...argv.includes("--execute")?{execute:true}:{},','...argv.includes("--execute")?{execute:true}:{},...argv.includes("--frozen")?{frozen:true}:{},...option("frozen")===undefined?{}:{frozenDirectory:option("frozen")},...option("edit")===undefined?{}:{edit:option("edit")},'),
('[--execute] [--stagger=','[--execute] [--frozen] [--stagger='),
])

edit('🟦️.ts',[
('export function treeChurnV1(root:string,marker:string):Readonly<{count:number;sample:readonly string[]}>{','export function treeChurnV1(root:string,marker:string,ignore:readonly string[]=[]):Readonly<{count:number;sample:readonly string[]}>{'),
('rows=(run.stdout??"").split("\\n").filter(Boolean);\n return Object.freeze({count:rows.length,sample:rows.slice(0,5).map(row=>row.slice(resolve(root).length+1))});','rows=(run.stdout??"").split("\\n").filter(Boolean).filter(row=>!ignore.includes(row.slice(resolve(root).length+1)));\n return Object.freeze({count:rows.length,sample:rows.slice(0,5).map(row=>row.slice(resolve(root).length+1))});'),
(' return value as SliceLabOptionsV1;',' const read=value as SliceLabOptionsV1;\n if(read.edit!==undefined&&(read.frozen!==true&&read.frozenDirectory===undefined||read.machines<2||(read.stagger??0)===0))throw Error("An edited file needs a frozen view, two machines and a stagger / Eine bearbeitete Datei braucht eine eingefrorene Sicht, zwei Maschinen und einen Versatz");\n return read;'),
(' let timedOut=false,leftoverProcesses=0,',' let restoreEdit:(()=>void)|undefined,timedOut=false,leftoverProcesses=0,'),
('  const launch=(plan:SliceLabPlanV1):void=>{\n','  const launch=(plan:SliceLabPlanV1):void=>{\n   if(options.edit!==undefined&&plan.index===plans.length&&restoreEdit===undefined)restoreEdit=editFrozenFileV1(view,options.edit);\n'),
('  await stop();\n  await fleet.stop();','  await stop();\n  restoreEdit?.();\n  await fleet.stop();'),
])
