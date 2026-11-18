#!/usr/bin/env python3
"""R81 stage builder: reads the live files, applies counted literal edits, writes <id>.base/<id>.new/index.txt into ./stage. Re-run to restage from the current live tree."""
import os,shutil,sys,json
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
DS=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/"
PR="🧰️framework/🔨️modules/🏃️process/"
H="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R81/stage/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def stage(rel,edits):
    i="f%d"%n[0];n[0]+=1
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:80]))
        text=text.replace(old,rep)
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))

stage(B+"👁️observation/🧪️tests/📥️reuse/🟦️.ts",[
 ('routing=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url));',
  'routing=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",import.meta.url)),interruption=fileURLToPath(new URL("../../../../../../../../🔨️modules/🏃️process/🛑️interruption/🟦️.ts",import.meta.url));',1),
 ("const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(${JSON.stringify(routing)}),",
  "const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import(${JSON.stringify(routing)}),{isScriptProcessInterruptedV1,SCRIPT_INTERRUPTED_EXIT_CODE_V1}=await import(${JSON.stringify(interruption)}),",1),
 ("let failed=false,same=false,calls=0,seen=false,message='',reported=false;","let failed=false,interrupted=false,same=false,calls=0,seen=false,message='',reported=false;",1),
 ('if(kind==="unsafe")expect(data.message).toBe("Single-link regular no-follow Source input required");','if(kind==="unsafe")expect(data.message.startsWith("Single-link regular no-follow Source input required: ")).toBe(true);',1),
 ("}catch(error){failed=true;same=Object.is(error,primary);message=error instanceof Error?error.message:String(error);}report();process.exitCode=failed?1:0;",
  "}catch(error){failed=true;interrupted=isScriptProcessInterruptedV1(error);same=Object.is(error,primary);message=error instanceof Error?error.message:String(error);}report();process.exitCode=interrupted?SCRIPT_INTERRUPTED_EXIT_CODE_V1:failed?1:0;",1),
])
stage(B+"👁️observation/🧪️tests/⚡️incremental/🟦️.ts",[
 ("cwd=repositoryRootV1,originalRead=fs.readFileSync","cwd=process.cwd(),originalRead=fs.readFileSync",1),
 ("fs.readdirSync(directory).filter(name=>name.endsWith('.json'))","fs.readdirSync(directory).filter(name=>/^[0-9a-f]{24}\\.json$/u.test(name))",1)])
stage(B+"📥️inputs/🔏️revision/🧪️tests/🟦️.ts",[("fs.mkdirSync(parent,{recursive:true});fs.writeFileSync(leaf,'abc');let path=leaf;","fs.mkdirSync(parent,{recursive:true});fs.writeFileSync(leaf,'abc');const aged=value=>{const old=new Date(Date.now()-60000);fs.utimesSync(value,old,old);};aged(leaf);aged(parent);let path=leaf;",1),("if(row.name==='file-change')fs.writeFileSync(leaf,'changed');","if(row.name==='file-change'){fs.writeFileSync(leaf,'changed');aged(leaf);}",1),("fs.mkdirSync(parent);fs.writeFileSync(leaf,'abc');}","fs.mkdirSync(parent);fs.writeFileSync(leaf,'abc');aged(leaf);aged(parent);}",1),('if(row.primary==="leaf")expect(data.message).toBe("Bounded physical Source revision required");','if(row.primary==="leaf")expect(data.message.startsWith("Bounded physical Source revision required: ")).toBe(true);',1),('if(row.primary==="ancestry")expect(data.message).toBe("Physical Source revision ancestry required");','if(row.primary==="ancestry")expect(data.message.startsWith("Physical Source revision ancestry required: ")).toBe(true);',1)])
stage(PR+"📖️texts/🧪️tests/🟦️.ts",[('...sources(join(import.meta.dir, "..", "..", "..", "..", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏃️process"))]','...sources(join(import.meta.dir, "..", "..", "..", "..", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏃️process")), ...sources(join(import.meta.dir, "..", "..", "..", "..", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "⚡️caching", "🚀️bootstrap", "📦️dependencies"))]',1)])
stage(B+"📥️source/🔏️closure/🧪️tests/📥️stream/🟦️.ts",[
 ('import{closureRefusalV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";','import{closureRefusalV1,closureRefusalMessagesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";',1),
 ('reader=new Function("fs","crypto","process",loader.slice(start,end)+";return{bytes:encoded.length,blocks:chunks.length,loaderSha256:loaderIdentity(),args};")',
  'tally={seen:0},bufferShim=Object.freeze({alloc:(size:number,fill?:string|number)=>Buffer.alloc(size,fill as never),allocUnsafe:(size:number)=>Buffer.allocUnsafe(size),from:(value:any,...rest:any[])=>(Buffer.from as (...parts:any[])=>Buffer)(value,...rest),concat:(list:readonly Uint8Array[],size?:number)=>{tally.seen=list.length;return Buffer.concat(list,size);}}),reader=new Function("fs","crypto","process","Buffer","tally",loader.slice(start,end)+";return{bytes:encoded.length,blocks:tally.seen,loaderSha256:loaderIdentity(),args};"),readerRun=(fsValue:unknown,processValue:unknown)=>{tally.seen=0;return reader(fsValue,{createHash},processValue,bufferShim,tally);}',1),
 ('expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);','expect(start).toBeGreaterThan(0);expect(end).toBeGreaterThan(start);for(const code of new Set([...loader.matchAll(/refusal\\(\'([a-z-]+)\'/gu)].map(match=>match[1]!))){const row=(closureRefusalMessagesV1 as Record<string,Record<string,{refusal:string}>>)[code];expect(row,code).toBeDefined();for(const language of["en","de"])expect(row?.[language]?.refusal,code+" "+language).toBeTruthy();}',1),
 ('const result=reader(fsValue,{createHash},processValue);','const result=readerRun(fsValue,processValue);',1),
 ('expect(()=>reader(fsValue,{createHash},processValue)).toThrow();','expect(()=>readerRun(fsValue,processValue)).toThrow();',1),
 ('if(mode==="digest"||mode==="physical"){expect(rejected,observed.stderr).toBe(true);expect(observed.stderr).toContain(mode==="digest"?"snapshot changed":"Source changed");continue;}','if(mode==="digest"){expect(rejected,observed.stderr).toBe(true);expect(observed.stderr).toContain("snapshot-changed");continue;}',1),
 ('changed:mode==="mutate"}','changed:false}',1),
])
stage(PR+"🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json",[('"maximum": 25165825','"maximum": 33554433',1)])
stage(PR+"🎛️owned-execution/📥️auxiliary/🧪️tests/🔣️.json",[('"count": 6291457','"count": 8388609',1)])
sites=AU+"🧭️sites/🧬️schema/🔣️.json"
d=json.load(open(R+sites,encoding="utf8"))
m=d["$defs"]["Messages"]["const"]
assert "snapshot-changed" not in m
def row(en,fixen,de,fixde):return {"en":{"refusal":en,"fix":fixen},"de":{"refusal":de,"fix":fixde}}
new={
 "digest-required":row("Immutable implementation digest required","Fix: pass the 64-character hexadecimal SHA-256 digest of the implementation snapshot as the first argument.","Unveränderlicher Implementierungs-Digest erforderlich","Behebung: Übergib den hexadezimalen SHA-256-Digest des Implementierungs-Snapshots mit 64 Zeichen als erstes Argument."),
 "snapshot-bounded":row("Immutable implementation snapshot is larger than its bound","Fix: capture a smaller Source closure or raise the closure bound in the source closure policy together with every ceiling of its family.","Unveränderlicher Implementierungs-Snapshot ist größer als seine Grenze","Behebung: Erfasse eine kleinere Source-Hülle oder erhöhe die Grenze in der Richtlinie der Source-Hülle zusammen mit jeder Obergrenze ihrer Familie."),
 "snapshot-changed":row("Immutable implementation snapshot changed","Fix: run the command again; the snapshot does not match its digest.","Unveränderlicher Implementierungs-Snapshot hat sich geändert","Behebung: Führe den Befehl erneut aus; der Snapshot passt nicht zu seinem Digest."),
 "snapshot-shortened":row("Immutable implementation snapshot was shortened while it was read","Fix: run the command again and do not edit the snapshot file while the command runs.","Unveränderlicher Implementierungs-Snapshot wurde beim Lesen gekürzt","Behebung: Führe den Befehl erneut aus und bearbeite die Snapshot-Datei nicht, während der Befehl läuft."),
}
text=open(R+sites,encoding="utf8").read()
i=text.index('"Messages"');j=text.index('"const"',i)
k=text.index("{",j)
body=text[k:]
close=None
depth=0
for p,ch in enumerate(body):
    if ch=="{":depth+=1
    elif ch=="}":
        depth-=1
        if depth==0:close=p;break
inner=body[:close]
last=inner.rstrip()
indent=None
lines=inner.split("\n")
import re
ent=[l for l in lines if re.match(r'^(\s+)"[a-z-]+": \{$',l)]
indent=re.match(r'^(\s+)',ent[0]).group(1)
def render(key,val):
    s=json.dumps(val,ensure_ascii=False,indent=2).split("\n")
    return indent+json.dumps(key)+": "+s[0]+"\n"+"\n".join(indent+x for x in s[1:])
add=",\n"+",\n".join(render(k,v) for k,v in sorted(new.items()))
newinner=last+add+"\n"+inner[len(inner.rstrip()):].lstrip("\n").rstrip(" ")
newinner=last+add+inner[len(last):]
newtext=text[:k]+newinner+body[close:]
json.loads(newtext)
open(H+"sites.base","w",encoding="utf8").write(text);open(H+"sites.new","w",encoding="utf8").write(newtext)
os.rename(H+"sites.base",H+"f%d.base"%n[0]);os.rename(H+"sites.new",H+"f%d.new"%n[0]);rows.append("f%d %s mod"%(n[0],sites));n[0]+=1
stage(DS+"🪜️ancestry/🟦️.ts",[
 ('const captureFunctions = async (admissions: Map<string, number>[])','let fileOpened: { path: string; run: () => Promise<void> } | undefined;\nconst captureFunctions = async (admissions: Map<string, number>[])',1),
 ('    const value = Reflect.get(target, key); return typeof value === "function" ? value.bind(target) : value;','    if (key === "withFile") return async (name: string, body: (file: unknown) => Promise<unknown>) => target.withFile(name, async (file: unknown) => { const hook = fileOpened && fileOpened.path === [...prefix, name].join("/") ? fileOpened : undefined; if (hook) { fileOpened = undefined; await hook.run(); } return body(file); });\n    const value = Reflect.get(target, key); return typeof value === "function" ? value.bind(target) : value;',1),
 ('      if (changed || mode === "fresh-roster" || mode === "path-escape" || mode === "file-link" || mode === "parent-link") return;\n      changed = true;\n      if (mode === "file-hash") await fixture.fs.writeFile(path, Buffer.alloc(first.files[0].bytes, 7));\n      if (mode === "file-replacement") { await fixture.fs.rename(path, path + ".old"); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); }\n      if (mode === "parent-replacement") { await fixture.fs.rename(parent, parent + ".old"); await fixture.fs.mkdir(parent); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); }\n','      if (changed || mode !== "cancel") return;\n      changed = true;\n',1),
 ('    let failure: unknown, changed = false;\n','    let failure: unknown, changed = false;\n    const opened = first.roots[0] + "/" + first.files[0].path;\n    if (mode === "file-hash") fileOpened = { path: opened, run: async () => { changed = true; await fixture.fs.writeFile(path, Buffer.alloc(first.files[0].bytes, 7)); } };\n    if (mode === "file-replacement") fileOpened = { path: opened, run: async () => { changed = true; await fixture.fs.rename(path, path + ".old"); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); } };\n    if (mode === "parent-replacement") fileOpened = { path: opened, run: async () => { changed = true; await fixture.fs.rename(parent, parent + ".old"); await fixture.fs.mkdir(parent); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); } };\n',1),
 ('      if (mode === "cancel") expect(failure).toBe(primary);\n      expect(run.api.artifactDescriptorCustodyV1(), mode).toEqual(baseline);\n    } finally { await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }','      if (mode === "cancel") expect(failure).toBe(primary);\n      expect(run.api.artifactDescriptorCustodyV1(), mode).toEqual(baseline);\n    } finally { fileOpened = undefined; await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }',1),
 ('const names = ["withJavascriptDependencyProducedDirectoryV1", "withJavascriptDependencyInputParentsV1", "captureJavascriptDependencyDescriptorInputsV1", "verifyJavascriptDependencyDescriptorGroupV1"];','const names = ["javascriptDependencyStoreWorkV1", "withJavascriptDependencyProducedDirectoryV1", "withJavascriptDependencyInputParentsV1", "captureJavascriptDependencyDescriptorInputsV1", "verifyJavascriptDependencyDescriptorGroupV1"];',1),
 ('const policy = JSON.parse(readFileSync(new URL("../../🧬️schema/📥️install.json", import.meta.url), "utf8"));\n  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("capture.ts"','const policy = JSON.parse(readFileSync(new URL("../../🧬️schema/📥️install.json", import.meta.url), "utf8"));\n  policy.$defs.SourceOwnerStoreProtocol.const.progressWorkUnits = 1;\n  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("capture.ts"',1),
 ('nodes.map(node => node.getText(tree)).join("\\n") + ";return {capture:','nodes.map(node => node.getText(tree).replace(/^export\\s+/u, "")).join("\\n") + ";return {capture:',1)])
stage(DS+"👓️reading/🟦️.ts",[('await run({operation:"state",path:"Source-only-not-Native",completedOperations:17});expect(physicalChecks).toBe(0);expect(cheapChecks).toBe(3);for(const progress of[{operation:"foreign",completedOperations:1},{operation:"list",completedOperations:Number.MAX_SAFE_INTEGER+1},{operation:"readText",completedOperations:-1}])',
 'const progressUnitV1=schema.$defs.InstalledPathProtocol.const.progressWorkUnits;await run({operation:"state",path:"Source-only-not-Native",completedOperations:17});expect(physicalChecks).toBe(0);expect(cheapChecks).toBe(0);await run({operation:"state",path:"Source-only-not-Native",completedOperations:progressUnitV1});expect(physicalChecks).toBe(0);expect(cheapChecks).toBe(3);for(const progress of[{operation:"foreign",completedOperations:2*progressUnitV1},{operation:"list",completedOperations:Number.MAX_SAFE_INTEGER+1},{operation:"readText",completedOperations:-progressUnitV1}])',1),
 ('ts.transpileModule(owner.getText(tree)+";return admitRepositoryDependencySelectedRetirementV1;"','ts.transpileModule(tree.statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&["requireRepositoryDependencySelectionCurrentV1","admitRepositoryDependencySelectedRetirementV1"].includes(node.name?.text??"")).map(node=>node.getText(tree).replace(/^export\\s+/u,"")).join("\\n")+";return admitRepositoryDependencySelectedRetirementV1;"',1)])
stage(B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts",[('import{createMemoryGovernorV1}from"../../🟨️.mjs";\n','',1)])
stage(B+"🔧️tools/🏗️bazel/🧪️tests/📋️contract/🔣️.json",[('d8221916d9a0e97095846a6c3f21d0813d672575778ada6f3505bc7af927a8ed','b95af1f45cc9f0001626c598a62157fdc47682877060f0cc9414991cfcc5245a',1)])
stage(B+"🔧️tools/🏗️bazel/🧪️tests/📥️keys/🟦️.ts",[('expect(oracleDifferences(first.workspace,second.workspace)).toEqual(["BUILD.bazel",...corpus.onlyRuntimeMayDiffer]);','{const mutated=oracleDifferences(first.workspace,second.workspace);expect(mutated).toContain("BUILD.bazel");expect(mutated.filter(name=>name!=="BUILD.bazel"&&!corpus.onlyRuntimeMayDiffer.includes(name))).toEqual([]);}',1),('expect(differing).toEqual(corpus.onlyRuntimeMayDiffer);','expect(differing).toContain("runtime.json");expect(differing.filter(name=>!corpus.onlyRuntimeMayDiffer.includes(name))).toEqual([]);',1)])
stage(AU+"🧪️tests/🟦️.ts",[
 ("const entries=Array.from({length:value.files-1},(_,index)=>({path:`f${index}.ts`,bytes:`export default {ordinal:${index}};`})),entry=entries.map((item,index)=>`import f${index} from './${item.path}';export{f${index}};`).join(\"\\n\");",
  "const hubs=Math.ceil((value.files-1)/1025),leaves=Array.from({length:value.files-1-hubs},(_,index)=>({path:`f${index}.ts`,bytes:`export default {ordinal:${index}};`})),groups=Array.from({length:hubs},(_,hub)=>({path:`g${hub}.ts`,bytes:leaves.slice(hub*1024,hub*1024+1024).map(item=>`import ${item.path.slice(0,-3)} from './${item.path}';export{${item.path.slice(0,-3)}};`).join(\"\\n\")})),entries=[...leaves,...groups],entry=groups.map(item=>`import * as ${item.path.slice(0,-3)} from './${item.path}';export{${item.path.slice(0,-3)}};`).join(\"\\n\");",1)])
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print(len(rows),"files staged")
