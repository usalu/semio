#!/usr/bin/env python3
"""Anchored apply of the cli-latency fixes. Dry run by default; --write applies all selected groups all-or-nothing.
Every anchor must occur exactly once in its file (drift refusal); an edit whose replacement is already present is skipped (idempotent).
  python3 staged/apply.py                      dry run, all groups
  python3 staged/apply.py --group sweep-interval --write
  python3 staged/apply.py --root /private/tmp/cli-latency/ov --prepare   copy the files the groups touch into an overlay root first
Groups: lazy-contract, revision-stamp, door-validate, sweep-interval, command-custody (ambient lease image + recheck window).
Order dependency: command-custody anchors only text that the profile hooks (apply-profile-hooks.py) do not change, so both orders work."""
import argparse,os,shutil,sys

HERE=os.path.dirname(os.path.abspath(__file__))
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B=L+"/🏗️build"
SCRIPT=B+"/📜️script.ts"
REVISION=B+"/📥️inputs/🔏️revision"
CHILD=B+"/📥️caller/📤️child/🟦️.ts"
OPENING=L+"/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening"
PARENTS=OPENING+"/📂️parents/🧬️schema/🔣️.json"
STAMP_FIXTURE=OPENING+"/🧪️tests/🧹️stamp/🔣️.json"

def files(name): return os.path.join(HERE,"files",name)

ROSTER_IMPORT='import{withNativeRecheckWindowV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🟦️.ts";\n'
CUSTODY_DOC='''/** 🧵️ Runs one command's custody work with the command's own open lease image shared by every partition it opens and with physical rechecks reusing a verification younger than the declared window, so a command opens the lease image once instead of once per partition; the partition owners never close the shared image, its opener does. */
export function withRepositoryCommandCustodyV1<T>(workspace:string,leases:NativeImageStorageV1,body:()=>Promise<T>):Promise<T>{return ambientLeases.run(Object.freeze({workspace,image:leases}),()=>withNativeRecheckWindowV1(body));}
const ambientLeases=new AsyncLocalStorage<Readonly<{workspace:string;image:NativeImageStorageV1}>>();
'''

# (group, file, old, new)
EDITS=[
# ---- lazy-contract: compile the Bazel test contract on first use instead of at every CLI start
("lazy-contract",SCRIPT,'const bazelTestContract=compileJsonSchemaSubset(bazelTestSchema);',
 'let bazelTestContractV1:ReturnType<typeof compileJsonSchemaSubset>|undefined;\nconst bazelTestContract=()=>bazelTestContractV1??=compileJsonSchemaSubset(bazelTestSchema);'),
("lazy-contract",SCRIPT,'bazelTestContract.validate({phase}).length','bazelTestContract().validate({phase}).length'),

# ---- revision-stamp: the stale-reader self check trusts an unchanged settled stamp instead of re-hashing the module twice per capture
("revision-stamp",REVISION+"/🟨️.mjs",
 'const ownSourceURL=physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL),ownPath=fileURLToPath(ownSourceURL),loaded=capture(ownPath,maximumSourceBytes,false),ownSchemaPath=fileURLToPath(new URL("./🧬️schema/🔣️.json",ownSourceURL)),loadedSchema=capturedSources?capture(ownSchemaPath,maximumTextBytes,false):undefined;',
 'const ownSourceURL=physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL),ownPath=fileURLToPath(ownSourceURL),ownSchemaPath=fileURLToPath(new URL("./🧬️schema/🔣️.json",ownSourceURL));\n'
 'function ownRow(path){try{const row=lstatSync(path,{bigint:true});return row.isFile()&&!row.isSymbolicLink()?{stamp:stamp(row),settled:Number(row.ctimeNs/1000000n)<Date.now()-settleMilliseconds&&Number(row.mtimeNs/1000000n)<Date.now()-settleMilliseconds}:undefined;}catch{return undefined;}}\n'
 'const ownBefore=[ownRow(ownPath),capturedSources?ownRow(ownSchemaPath):null],loaded=capture(ownPath,maximumSourceBytes,false),loadedSchema=capturedSources?capture(ownSchemaPath,maximumTextBytes,false):undefined,ownAfter=[ownRow(ownPath),capturedSources?ownRow(ownSchemaPath):null];\n'
 'const ownTrusted=ownBefore[0]?.settled&&ownBefore[0].stamp===ownAfter[0]?.stamp&&(ownBefore[1]===null||ownBefore[1]?.settled&&ownBefore[1].stamp===ownAfter[1]?.stamp)?[ownBefore[0].stamp,ownBefore[1]?.stamp??null]:undefined;\n'
 'function ownUnchanged(){return ownTrusted!==undefined&&ownRow(ownPath)?.stamp===ownTrusted[0]&&(ownTrusted[1]===null||ownRow(ownSchemaPath)?.stamp===ownTrusted[1]);}\n'
 ''),
("revision-stamp",REVISION+"/🟨️.mjs",'function current(){if(implementationPinnedV1())return;','function current(){if(implementationPinnedV1())return;if(ownUnchanged())return;'),
("revision-stamp",REVISION+"/🧪️tests/🧊️own-stamp/🟦️.ts",None,"@file:"+files("revision-own-stamp-test.ts")),

# ---- door-validate: an unknown repo command is refused in milliseconds, before capacity, image opens and graph locks
("door-validate",CHILD,
 '  const {createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("../🟦️.ts");\n',
 '  const {createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("../🟦️.ts");\n'
 '  if(arguments_[0]==="repo"){const {buildInvocationRequestsHelpV1}=await import("../../🧭️invocation/🟦️.ts"),command=resolved.args[0];if(command!==undefined&&!buildInvocationRequestsHelpV1(resolved.args)){const known=((await import("../../⚙️operations/🔣️catalog.json",{with:{type:"json"}})).default.operations as readonly {name:string}[]).map(row=>row.name);if(!known.includes(command))throw Error(`Unknown repo command "${command}"; expected one of: ${known.join(", ")} / Unbekannter repo-Befehl "${command}"; erwartet wird einer von: ${known.join(", ")}`);}}\n'),
("door-validate",B+"/📥️caller/🧪️tests/🚪️door/🟦️.ts",None,"@file:"+files("door-validate-test.ts")),
("door-validate",B+"/📥️caller/🧪️tests/🚪️door/🔣️.json",None,"@file:"+files("door-validate-fixture.json")),

# (door-validate light commands: live runBuildCliV1 already classifies the command after the global options; the script edits and the light test are not part of the merged copy)

# ---- sweep-interval: sweep idle leases at most every five minutes instead of every fifteen seconds
("sweep-interval",PARENTS,'"leaseRetirementIntervalMilliseconds": 15000','"leaseRetirementIntervalMilliseconds": 300000'),
("sweep-interval",STAMP_FIXTURE,'"intervalMilliseconds": 15000','"intervalMilliseconds": 300000'),
("sweep-interval",STAMP_FIXTURE,'"ageMilliseconds": 20000, "fresh": false','"ageMilliseconds": 320000, "fresh": false'),
("sweep-interval",STAMP_FIXTURE,'"ageMilliseconds": 14900, "fresh": true','"ageMilliseconds": 299000, "fresh": true'),

# ---- command-custody: one lease image per command (shared by every partition it opens) and a recheck window for the whole command
("command-custody",OPENING+"/🟦️.ts",'import{AsyncLocalStorage}from"node:async_hooks";\n','import{AsyncLocalStorage}from"node:async_hooks";\n'+ROSTER_IMPORT),
("command-custody",OPENING+"/🟦️.ts",
 'const leases=await openRepositoryCachePartitionV1(workspace,"leases",control);let image:NativeImageStorageV1|undefined;try{',
 'const ambient=ambientLeases.getStore(),shared=ambient!==undefined&&ambient.workspace===workspace,leases=shared?ambient.image:await openRepositoryCachePartitionV1(workspace,"leases",control);let image:NativeImageStorageV1|undefined;try{'),
("command-custody",OPENING+"/🟦️.ts",'scope:PartitionOwner={id,image:admitted,leases,control,live:true,','scope:PartitionOwner={id,image:admitted,leases,sharedLeases:shared,control,live:true,'),
("command-custody",OPENING+"/🟦️.ts",'for(const owned of[image,leases])try{await owned?.close();}','for(const owned of[image,...(shared?[]:[leases])])try{await owned?.close();}'),
("command-custody",OPENING+"/🟦️.ts",'type PartitionOwner={id:RepositoryCachePartitionIdV1;image:NativeImageStorageV1;leases:NativeImageStorageV1;','type PartitionOwner={id:RepositoryCachePartitionIdV1;image:NativeImageStorageV1;leases:NativeImageStorageV1;sharedLeases:boolean;'),
("command-custody",OPENING+"/🟦️.ts",'for(const image of[scope.image,scope.leases])try{await image.close();}','for(const image of scope.sharedLeases?[scope.image]:[scope.image,scope.leases])try{await image.close();}'),
("command-custody",OPENING+"/🟦️.ts",'type PartitionOutcome={sequence:number;failed:boolean;error:unknown};\n',CUSTODY_DOC+'type PartitionOutcome={sequence:number;failed:boolean;error:unknown};\n'),
("command-custody",SCRIPT,'const {openRepositoryCachePartitionV1}=await','const {openRepositoryCachePartitionV1,withRepositoryCommandCustodyV1}=await'),
("command-custody",SCRIPT,';await withRepositoryCacheStoreV1(root,"diagnostics",parent,control,async(diagnostics,diagnosticPhase)=>{',';await withRepositoryCommandCustodyV1(root,leases,()=>withRepositoryCacheStoreV1(root,"diagnostics",parent,control,async(diagnostics,diagnosticPhase)=>{'),
("command-custody",SCRIPT,'else process.env.SEMIO_DIAGNOSTIC_DIRECTORY=previous;}});}catch(error){failed=true;throw error;}','else process.env.SEMIO_DIAGNOSTIC_DIRECTORY=previous;}}));}catch(error){failed=true;throw error;}'),
]

def main():
    ap=argparse.ArgumentParser();ap.add_argument("--group",action="append");ap.add_argument("--write",action="store_true");ap.add_argument("--root",default=os.getcwd());ap.add_argument("--prepare",action="store_true",help="copy the files of the selected groups from the cwd checkout into --root first")
    a=ap.parse_args();groups=set(a.group) if a.group else {e[0] for e in EDITS}
    src=os.getcwd();selected=[e for e in EDITS if e[0] in groups]
    if a.prepare:
        for _,rel,old,new in selected:
            if old is None: continue
            dst=os.path.join(a.root,rel);os.makedirs(os.path.dirname(dst),exist_ok=True)
            if not os.path.exists(dst): shutil.copy2(os.path.join(src,rel),dst)
        if any(g=="revision-stamp" for g,*_ in selected):
            rel=REVISION+"/🧬️schema/🔣️.json";d=os.path.join(a.root,rel);os.makedirs(os.path.dirname(d),exist_ok=True);shutil.copy2(os.path.join(src,rel),d)
        print("overlay prepared at",a.root)
    texts={};todo=[];skipped=[];bad=[]
    for group,rel,old,new in selected:
        path=os.path.join(a.root,rel)
        if old is None:
            content=open(new[6:],encoding="utf8").read()
            if os.path.exists(path):
                if open(path,encoding="utf8").read()==content: skipped.append((group,rel));continue
                bad.append((group,rel,"exists with different content"));continue
            texts[path]=content;todo.append((group,rel));continue
        if len(rel.encode())+len(a.root)>1000: bad.append((group,rel,"path too long"));continue
        text=texts.get(path)
        if text is None: text=open(path,encoding="utf8").read()
        co,cn=text.count(old),text.count(new)
        if cn>=1 and co==0: skipped.append((group,rel));continue
        if co!=1: bad.append((group,rel,f"anchor occurs {co} times"));continue
        texts[path]=text.replace(old,new);todo.append((group,rel))
    print(f"{len(todo)} to apply, {len(skipped)} already applied, {len(bad)} refused ({sorted(groups)})")
    for g,r,why in bad: print(f"REFUSED [{g}] {r.split('/')[-1]}: {why}")
    if bad: sys.exit(2)
    if not a.write: print("dry run ok (use --write)");return
    for path,content in texts.items():
        os.makedirs(os.path.dirname(path),exist_ok=True);tmp=path+".tmp";open(tmp,"w",encoding="utf8").write(content);os.replace(tmp,path)
    print("written",len(texts),"files")
main()
