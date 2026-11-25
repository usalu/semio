#!/usr/bin/env python3
"""F33 staged landing: anchored, idempotent edits. Run from /Users/ueli/Documents/semio after 19:00 under the area locks."""
import sys, hashlib
R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FT = R + "🏗️build/🔧️tools/🧶️nx/🪵️failure-tail/"
RD = R + "🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
DS = R + "⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/"
edits = []

def edit(path, old, new, marker):
    edits.append((path, old, new, marker))

# 1. failure-tail: unattributed output fallback, never zero lines
edit(FT + "🧬️schema/🔣️.json",
 '''            "no": "another rerun is unlikely to help: the task itself failed"
          },''',
 '''            "no": "another rerun is unlikely to help: the task itself failed",
            "unattributed": "no output carries this task's header; the output printed before the summary follows",
            "silent": "the task printed no output at all; its exit status and signal are in the stage log"
          },''', '"unattributed"')
edit(FT + "🧬️schema/🔣️.json",
 '''            "no": "eine weitere Wiederholung hilft wahrscheinlich nicht: die Aufgabe selbst ist fehlgeschlagen"
          }''',
 '''            "no": "eine weitere Wiederholung hilft wahrscheinlich nicht: die Aufgabe selbst ist fehlgeschlagen",
            "unattributed": "keine Ausgabe trägt die Kopfzeile dieser Aufgabe; es folgt die Ausgabe vor der Zusammenfassung",
            "silent": "die Aufgabe gab gar nichts aus; Exit-Status und Signal stehen im Stufenprotokoll"
          }''', '"silent": "die Aufgabe')
edit(FT + "🟦️.ts",
 '''/** 🧯️ One failed task and the last lines of its own output. */''',
 '''const orphansOf=(lines:readonly string[]):string[]=>{
 const rows:string[]=[];let current:string|undefined;
 for(const line of lines){
  if(/^ NX /u.test(line))break;
  const header=/^(?:> |✖\\s+)nx run (\\S+)/u.exec(line);if(header){current=header[1];continue;}
  if(/^(?:✔|✓)\\s+nx run /u.test(line)){current=undefined;continue;}
  if(current===undefined)rows.push(line);
 }
 return rows;
};
const bothLanguagesV1=(key:"unattributed"|"silent"):string=>(summaryPolicy.texts.en as Record<string,string>)[key]+" / "+(summaryPolicy.texts.de as Record<string,string>)[key];
const errorLinePattern=/^(?:error|Error)(?:\\[\\w+\\])?:/u;
/** 🧯️ One failed task and the last lines of its own output. */''', "const orphansOf=")
edit(FT + "🟦️.ts",
 '''const lines=tail.split("\\n").map(plainLine),failed=listOf(lines,/^Failed tasks:\\s*$/u),sections=sectionsOf(lines,failed),noise=new RegExp(summaryPolicy.tailNoisePattern,"u");
 return Object.freeze(failed.slice(0,summaryPolicy.maximumTasks).map(task=>Object.freeze({task,lines:Object.freeze((sections.get(task)??[]).filter(line=>line.trim()!==""&&!noise.test(line)).slice(-count).map(line=>[...line].slice(0,summaryPolicy.maximumTailLineCharacters).join("")))})));''',
 '''const lines=tail.split("\\n").map(plainLine),failed=listOf(lines,/^Failed tasks:\\s*$/u),sections=sectionsOf(lines,failed),noise=new RegExp(summaryPolicy.tailNoisePattern,"u"),usable=(rows:readonly string[])=>rows.filter(line=>line.trim()!==""&&!noise.test(line));
 const orphans=usable(orphansOf(lines)),cut=(rows:readonly string[])=>rows.map(line=>[...line].slice(0,summaryPolicy.maximumTailLineCharacters).join(""));
 const unattributed=():readonly string[]=>{
  if(orphans.length===0)return[bothLanguagesV1("silent")];
  const tailRows=orphans.slice(-count),at=orphans.findLastIndex(line=>errorLinePattern.test(line)),cause=at>=0&&at<orphans.length-count?orphans.slice(at,at+3):[];
  return[bothLanguagesV1("unattributed"),...cut([...cause,...tailRows])];
 };
 return Object.freeze(failed.slice(0,summaryPolicy.maximumTasks).map(task=>{const own=usable(sections.get(task)??[]);return Object.freeze({task,lines:Object.freeze(own.length>0?cut(own.slice(-count)):unattributed())});}));''', "const unattributed=")

# 2. ready: a cargo write into the checkout that vanished is transient
edit(RD + "🧬️schema/🔣️.json",
 '''"plan cannot continue[^\\\\n]*same text: [^\\\\n]*{root}"],''',
 '''"plan cannot continue[^\\\\n]*same text: [^\\\\n]*{root}", "^error: failed to write [`']{root}[^`']*[`']\\\\s*$"],''', "failed to write [`']{root}")

# 3. storage scan: name the path and keys, tolerate directory ChangeTime on Windows, rescan on a real change
edit(DS + "🧬️schema/📥️install.json",
 '''        "scanCustodyMilliseconds": 20
''',
 '''        "scanCustodyMilliseconds": 20,
        "maximumScanAttempts": 3
''', '"maximumScanAttempts"')
edit(DS + "🟦️.ts", ''',hash=createHash("sha256"),started=performance.now()''', ''',started=performance.now()''', 'let hash=createHash')
edit(DS + "🟦️.ts", '''let entries=0,files=0,links=0,maximumPathUnits=0,work=0,nextProgress=policy.scanProgressWorkUnits;
 const same=(left:BigIntStats,right:BigIntStats)=>keys.every(key=>left[key]===right[key]);''',
 '''let hash=createHash("sha256"),entries=0,files=0,links=0,maximumPathUnits=0,work=0,nextProgress=policy.scanProgressWorkUnits;
 const same=(left:BigIntStats,right:BigIntStats)=>keys.every(key=>left[key]===right[key]);
 const drift=(path:string,left:BigIntStats,right:BigIntStats):string|undefined=>{const moved=keys.filter(key=>left[key]!==right[key]&&!(process.platform==="win32"&&key==="ctimeNs"));return moved.length===0?undefined:path+" ("+moved.map(key=>key+" "+String(left[key])+" -> "+String(right[key])).join(", ")+")";};''', 'const drift=(')
edit(DS + "🟦️.ts", '''for(const row of ancestors)if(!same(row.facts,lstatSync(row.path,{bigint:true})))throw Error("Original installed dependency directory changed");};''',
 '''for(const row of ancestors){const moved=drift(row.path,row.facts,lstatSync(row.path,{bigint:true}));if(moved!==undefined)throw Error("Original installed dependency directory changed: "+moved);}};''', 'directory changed: "+moved')
edit(DS + "🟦️.ts", '''throw Error("Original installed dependency entry changed");await advance();''', '''throw Error("Original installed dependency entry changed: "+path);await advance();''', 'entry changed: "+path')
edit(DS + "🟦️.ts", '''if(!same(facts,lstatSync(path,{bigint:true})))throw Error("Original installed dependency directory changed");ledger?.directory(''',
 '''{const moved=drift(path,facts,lstatSync(path,{bigint:true}));if(moved!==undefined)throw Error("Original installed dependency directory changed: "+moved);}ledger?.directory(''', 'const moved=drift(path,facts')
edit(DS + "🟦️.ts", ''' await visit(root,".",0,undefined);check();return Object.freeze({version:1,entries,''',
 ''' for(let attempt=1;;attempt++){
  try{await visit(root,".",0,undefined);check();break;}
  catch(error){
   const message=String((error as Error)?.message??error);
   if(attempt>=policy.maximumScanAttempts||control.signal.aborted||!/^Original installed dependency (?:directory|entry) changed/u.test(message))throw error;
   console.error("[DATA] Installed dependency scan restarted "+JSON.stringify({version:1,purpose:"installed-dependency-scan-restart",attempt,message:message.slice(0,400)}));
   hash=createHash("sha256");entries=0;files=0;links=0;maximumPathUnits=0;work=0;nextProgress=policy.scanProgressWorkUnits;directories.clear();ancestors.length=0;if(ledger)ledger.rows.length=0;
  }
 }
 return Object.freeze({version:1,entries,''', 'installed-dependency-scan-restart')

# 4. tests
SUMMARY = FT + "🧪️tests/🧾️summary/🟦️.ts"
edit(SUMMARY, """expect(tails[1]!.lines).toEqual([]);""", """expect(tails[1]!.lines).toEqual([(policy.texts.en as Record<string,string>).silent+" / "+(policy.texts.de as Record<string,string>).silent]);""", '.silent+" / "')
edit(SUMMARY, """test("a coloured transcript is read like a plain one""", """test("a failed task whose output carries no header is read from the output before the summary, with its error line kept, and a task without any output says so",()=>{
 const warnings=Array.from({length:80},(_,index)=>"warning: unused "+index).join("\\n"),text=warnings+"\\nerror: failed to write `/x/invoked.timestamp`\\nCaused by:\\n  No such file or directory (os error 2)\\n"+Array.from({length:12},(_,index)=>"  at frame "+index).join("\\n")+"\\n✔  nx run a:ok\\n NX   failed\\nFailed tasks:\\n- a:build\\n",rows=nxFailedTaskTailsV1(text,5)[0]!.lines;
 expect(rows[0]).toContain((policy.texts.en as Record<string,string>).unattributed);
 expect(rows).toContain("error: failed to write `/x/invoked.timestamp`");expect(rows).toContain("  No such file or directory (os error 2)");expect(rows.at(-1)).toBe("  at frame 11");
 expect(nxFailedTaskTailsV1("✔  nx run a:ok\\n NX   failed\\nFailed tasks:\\n- a:build\\n")[0]!.lines.length).toBe(1);
});
test("a coloured transcript is read like a plain one""", 'failed to write `/x/invoked')

SOURCE = RD + "🧪️tests/🧱️source-transient/🟦️.ts"
edit(SOURCE, """ {lines:["stage failed with exit code 1"],root,want:false},""", """ {lines:["stage failed with exit code 1"],root,want:false},
 {lines:[`error: failed to write \`${root}/.cache/b/r/m/b/p/prototype/build/semio-framework-pack-http/cf085c40b268a8e7/fingerprint/invoked.timestamp\``],root,want:true},
 {lines:["error: failed to write `/elsewhere/cache/x.timestamp`"],root,want:false},""", 'prototype/build/semio-framework-pack-http')

NEW = DS + "🧪️tests/🔁️rescan/🟦️.ts"
NEWTEXT = r'''/** 🔁️ Law: the installed dependency path scan names the path and the stat keys of a directory that changed under it, and rescans from scratch (bounded by `maximumScanAttempts`, ledger and counters reset) so a change it observed once does not fail the stage when the second walk is stable; the same digest as an undisturbed scan.
 * @see ../../🟦️.ts */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {javascriptInstallSchemaV1 as installSchema} from "../../🧱️schemas/🟦️.ts";

type Scan=(root:string,control:unknown,check:()=>void,ledger?:{rows:unknown[];directory:(...rest:unknown[])=>void})=>Promise<{entries:number;sha256:string}>;
const build=async(attempts:number,stat:unknown)=>{
 const ts=await import("typescript"),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("dependency-path.ts",source,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="scanJavascriptDependencyInstalledPathsV1")!,code=ts.transpileModule(fn.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,descriptors=await import("@semio-tech/framework-filesystem/observation/descriptor");
 return new Function("installSchema","createHash","lstatSync","withArtifactReadDirectoryV1","join","requireIssuedResourceControlV1","checkResourceLeaseControl",code+";return scanJavascriptDependencyInstalledPathsV1;")({$defs:{InstalledPathProtocol:{const:{...installSchema.$defs.InstalledPathProtocol.const,maximumScanAttempts:attempts}}}},createHash,stat,descriptors.withArtifactReadDirectoryV1,join,()=>{},()=>{})as Scan;
};
const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}};
const disturb=(target:string,key:"mtimeNs"|"size")=>{let calls=0;return((path:string,options:unknown)=>{const facts=(lstatSync as any)(path,options);if(path===target&&++calls===2)return Object.create(facts,{[key]:{value:facts[key]+1n}});return facts;})as unknown;};
test("a directory that changed once is named with its stat keys and the scan restarts and agrees with an undisturbed scan",async()=>{
 const root=mkdtempSync(join(tmpdir(),"scan-rescan-"));
 try{
  for(let directory=0;directory<3;directory++){mkdirSync(join(root,"d"+directory));for(let file=0;file<5;file++)writeFileSync(join(root,"d"+directory,"f"+file+".js"),"x"+directory);}
  const clean=await(await build(3,lstatSync))(root,control,()=>{}),target=join(root,"d1");
  await expect((await build(1,disturb(target,"mtimeNs")))(root,control,()=>{})).rejects.toThrow(new RegExp("Original installed dependency directory changed: .*d1 \\(mtimeNs \\d+ -> \\d+\\)","u"));
  const seen:string[]=[],original=console.error;console.error=(...rest:unknown[])=>{seen.push(rest.join(" "));};
  const rows:unknown[]=[],ledger={rows,directory:(...row:unknown[])=>{rows.push(row);}};
  try{const again=await(await build(3,disturb(target,"size")))(root,control,()=>{},ledger);expect(again.sha256).toBe(clean.sha256);expect(again.entries).toBe(clean.entries);}finally{console.error=original;}
  expect(seen.filter(line=>line.includes("installed-dependency-scan-restart")).length).toBe(1);
  expect(rows.length).toBe(4);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);
'''
NEWFILE = (NEW, NEWTEXT)

def main():
    apply = "--apply" in sys.argv
    import os
    if not os.path.exists(NEWFILE[0]):
        if apply:
            os.makedirs(os.path.dirname(NEWFILE[0]), exist_ok=True)
            open(NEWFILE[0], "w", encoding="utf8").write(NEWFILE[1]); print("created", NEWFILE[0][-40:])
        else:
            print("new file", NEWFILE[0][-40:])
    for path, old, new, marker in edits:
        text = open(path, encoding="utf8").read()
        if marker in text:
            print("already", path[-40:], marker[:30]); continue
        if text.count(old) != 1:
            print("ANCHOR", text.count(old), path, old[:60]); sys.exit(1)
        if apply:
            open(path, "w", encoding="utf8").write(text.replace(old, new))
            print("applied", path[-40:], marker[:30])
        else:
            print("ok", path[-40:], marker[:30])
main()
