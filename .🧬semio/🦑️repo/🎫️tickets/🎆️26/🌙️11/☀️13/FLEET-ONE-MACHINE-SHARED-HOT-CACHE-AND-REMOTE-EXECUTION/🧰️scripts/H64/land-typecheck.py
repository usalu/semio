import os,sys,json
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️typecheck/"
APPLY="--apply" in sys.argv
HELPER='''/** 🩹️ Self-heals a missing compiler: when the script is absent and the run carries an `ensureCompiler` port, announces it in the language of the run and lets `repo dependencies ensure` (progress and cancellation included) restore it; a failed ensure is a typed `failed` refusal, a script still absent a typed `missing` one, a cancellation `cancelled`. */
async function requireCompilerV1(script:string,ports:TypecheckPortsV1,language:TypecheckLanguageV1,limits:TypecheckPolicyV1,notice:(line:string)=>void):Promise<void>{
 if(existsSync(script))return;
 if(ports.ensureCompiler!==undefined){
  notice(typecheckTextV1(language,"compiler.ensuring",{script:limits.compiler.script}));
  try{await ports.ensureCompiler(ports.signal);}catch(error){if(ports.signal?.aborted)throw refuseTypecheckV1("cancelled","cancelled");throw refuseTypecheckV1("failed","compiler.ensure-failed",{reason:String(error instanceof Error?error.message:error).slice(0,limits.limits.errorCharacters)});}
 }
 if(!existsSync(script))throw refuseTypecheckV1("missing","compiler.missing",{script:limits.compiler.script});
}

'''
edits=[(B+"🟦️.ts",[
 ("export type TypecheckPortsV1=Readonly<{cacheDirectory?:string;compile?:TypecheckCompileV1;","export type TypecheckPortsV1=Readonly<{cacheDirectory?:string;compile?:TypecheckCompileV1;ensureCompiler?:(signal?:AbortSignal)=>Promise<void>;",1),
 ('if(!project||!existsSync(script))throw refuseTypecheckV1("missing","compiler.missing",{script:limits.compiler.script});','if(!project)throw refuseTypecheckV1("missing","compiler.missing",{script:limits.compiler.script});await requireCompilerV1(script,ports,language,limits,notice);',1),
 ('if(plan.files.length&&!existsSync(script))throw refuseTypecheckV1("missing","compiler.missing",{script:limits.compiler.script});','if(plan.files.length)await requireCompilerV1(script,ports,language,limits,notice);',1),
 ("/** 🏛️ Type-checks the whole library project once",HELPER+"/** 🏛️ Type-checks the whole library project once",1)]),
(B+"⌨️command/🟦️.ts",[
 ("notice,cacheDirectory:typecheckCacheDirectoryV1(realpathSync(root),environment),...ports.run}","notice,cacheDirectory:typecheckCacheDirectoryV1(realpathSync(root),environment),ensureCompiler:signal=>ensureTypecheckCompilerV1(root,environment,signal),...ports.run}",1),
 ("/** 🎛️ What the command touches besides the checkout:",'''/** 🩹️ Restores the compiler through the owner of `repo dependencies ensure` (one run per process, progress lines on stderr in the help language, cancellation ends its process tree). */
async function ensureTypecheckCompilerV1(root:string,environment:NodeJS.ProcessEnv,signal:AbortSignal=new AbortController().signal):Promise<void>{
 const{rootCommandRegistryV1}=await import("../../../🎛️commands/🗂️registry/🟦️.ts"),{ensureRootDependenciesV1}=await import("../../../🎛️commands/🗂️registry/🥾️provision/🟦️.ts");
 await ensureRootDependenciesV1(root,rootCommandRegistryV1(),"typecheck",signal,environment);
}

/** 🎛️ What the command touches besides the checkout:''',1)]),
]
TEXTS=B+"📖️texts/🔣️.json"
OLD_EN='The TypeScript compiler is not installed at {script}; install the dependencies first (repo tools setup all)'
OLD_DE='Der TypeScript-Compiler fehlt unter {script}; zuerst die Abhängigkeiten installieren (repo tools setup all)'
NEW_EN='The TypeScript compiler is not installed at {script} and could not be restored; run repo dependencies ensure'
NEW_DE='Der TypeScript-Compiler fehlt unter {script} und konnte nicht wiederhergestellt werden; repo dependencies ensure ausführen'
ADD='''    "compiler.ensuring": {
      "en": "The TypeScript compiler is missing at {script}; restoring the dependencies (repo dependencies ensure)",
      "de": "Der TypeScript-Compiler fehlt unter {script}; die Abhängigkeiten werden wiederhergestellt (repo dependencies ensure)"
    },
    "compiler.ensure-failed": {
      "en": "Restoring the TypeScript compiler failed: {reason}; run repo dependencies ensure",
      "de": "Die Wiederherstellung des TypeScript-Compilers schlug fehl: {reason}; repo dependencies ensure ausführen"
    },
'''
TEST='''
test("a missing compiler is restored through the ensure port with progress in the run language, then checked; an unfinished or failed ensure is a typed refusal",scratch(async directory=>{
 write(directory,"lib/a/🟦️.ts","export const a=1;");write(directory,"tsconfig.json","{}");
 const limits=miniPolicy({compiler:{...policy.compiler,script:"node_modules/typescript/lib/tsc.js",typeRoots:"node_modules/@types"}}),touched={check:["lib/a/🟦️.ts"],pairs:[]},options=typecheckArgumentsV1(["lib/a/🟦️.ts"],limits),cache=join(directory,"cache"),lines:string[]=[];
 const run=(ports:object)=>runTypecheckV1(directory,touched,options,"de",{cacheDirectory:cache,compile:compileWith("",0),notice:line=>lines.push(line),...ports},limits);
 expect(((await run({}).catch(error=>error)) as {key:string}).key).toBe("typecheck.compiler.missing");
 let calls=0;
 const healed=await run({ensureCompiler:async()=>{calls++;write(directory,"node_modules/typescript/lib/tsc.js","");}});
 expect(healed.status).toBe("passed");expect(calls).toBe(1);expect(lines.some(line=>line.includes("Abhängigkeiten werden wiederhergestellt"))).toBe(true);
 rmSync(join(directory,"node_modules"),{recursive:true,force:true});
 expect(((await run({ensureCompiler:async()=>{}}).catch(error=>error)) as {key:string}).key).toBe("typecheck.compiler.missing");
 const failed=await run({ensureCompiler:async()=>{throw Error("offline");}}).catch(error=>error) as {key:string;code:string};
 expect(failed.key).toBe("typecheck.compiler.ensure-failed");expect(failed.code).toBe("failed");
}));
'''
ok=True;work=[]
for f,subs in edits:
    s=open(f,encoding="utf8").read()
    if "requireCompilerV1" in s or "ensureTypecheckCompilerV1" in s: print("already",f);continue
    for old,new,n in subs:
        c=s.count(old)
        if c!=n:print("ANCHOR",c,n,f[-30:],old[:60]);ok=False
        s=s.replace(old,new)
    work.append((f,s))
t=open(TEXTS,encoding="utf8").read()
if "compiler.ensuring" not in t:
    if t.count(OLD_EN)!=1 or t.count(OLD_DE)!=1 or t.count('    "compiler.timeout": {')!=1:print("TEXT anchors");ok=False
    t=t.replace(OLD_EN,NEW_EN).replace(OLD_DE,NEW_DE).replace('    "compiler.timeout": {',ADD+'    "compiler.timeout": {');json.loads(t);work.append((TEXTS,t))
tf=B+"🧪️tests/🟦️.ts";ts=open(tf,encoding="utf8").read()
if "restored through the ensure port" not in ts:work.append((tf,ts.rstrip("\n")+"\n"+TEST))
if not ok:sys.exit(1)
print("anchors ok",len(work))
if APPLY:
    for f,s in work:
        open(f+".h64tmp","w",encoding="utf8").write(s);os.replace(f+".h64tmp",f)
    print("applied")
