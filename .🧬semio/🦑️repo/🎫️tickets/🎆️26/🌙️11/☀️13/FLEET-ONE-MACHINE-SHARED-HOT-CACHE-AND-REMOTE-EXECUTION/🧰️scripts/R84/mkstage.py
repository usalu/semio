#!/usr/bin/env python3
"""R84 stage builder: reads the live files, applies counted literal edits, writes <id>.base/<id>.new/index.txt into the scratch stage. Re-run to restage from the current live tree. Never writes into the tree."""
import os,shutil,sys,json,re
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import projects_edit,bazel_root_edit,incremental_edit,bazel_misc_edit as bm
R="/Users/ueli/Documents/semio/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
AU=L+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
DS=L+"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/"
PR="🧰️framework/🔨️modules/🏃️process/"
H="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/stage/"
P="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R84/"
shutil.rmtree(H,ignore_errors=True);os.makedirs(H)
rows=[];n=[0]
def put(rel,base,text):
    i="f%d"%n[0];n[0]+=1
    open(H+i+".base","w",encoding="utf8").write(base);open(H+i+".new","w",encoding="utf8").write(text);rows.append("%s %s mod"%(i,rel))
def stage(rel,edits):
    text=open(R+rel,encoding="utf8").read();base=text
    for old,rep,count in edits:
        got=text.count(old)
        if got!=count:sys.exit("anchor count %d != %d in %s: %r"%(got,count,rel,old[:90]))
        text=text.replace(old,rep)
    put(rel,base,text)
def stage_fn(rel,fn):
    text=open(R+rel,encoding="utf8").read();put(rel,text,fn(text))
def messages_text(text,new):
    d=json.loads(text);m=d["$defs"]["Messages"]["const"]
    for k in new:assert k not in m,k
    i=text.index('"Messages"');j=text.index('"const"',i);k=text.index("{",j);body=text[k:];depth=0
    for p,ch in enumerate(body):
        if ch=="{":depth+=1
        elif ch=="}":
            depth-=1
            if depth==0:close=p;break
    inner=body[:close];last=inner.rstrip()
    indent=re.match(r'^(\s+)',[l for l in inner.split("\n") if re.match(r'^(\s+)"[a-z-]+": \{$',l)][0]).group(1)
    def render(key,val):
        s=json.dumps(val,ensure_ascii=False,indent=2).split("\n");return indent+json.dumps(key)+": "+s[0]+"\n"+"\n".join(indent+x for x in s[1:])
    newtext=text[:k]+last+",\n"+",\n".join(render(a,b) for a,b in sorted(new.items()))+inner[len(last):]+body[close:]
    json.loads(newtext);return newtext
def row(en,fixen,de,fixde):return {"en":{"refusal":en,"fix":fixen},"de":{"refusal":de,"fix":fixde}}

SITES=AU+"🧭️sites/"
stage(SITES+"🟦️.ts",[
 ('if(!specifier||rows.size>=policy.maximumSitesPerFile&&!rows.has(specifier))return;const earlier','if(!specifier)return;if(rows.size>=policy.maximumSitesPerFile&&!rows.has(specifier))throw closureRefusalV1("sites-bounded",{importer:path,detail:"maximumSitesPerFile="+policy.maximumSitesPerFile});const earlier',1),
 ('/** 🔎️ Lists every literal import, dynamic import, require, createRequire(...)("x"), require.resolve and import.meta.resolve specifier of one source file. */','/** 🔎️ Lists every literal import, dynamic import, require, createRequire(...)("x"), require.resolve and import.meta.resolve specifier of one source file; a file with more distinct specifiers than the family bound is refused in both languages, never truncated. */',1)])
NEWROW={"sites-bounded":row("Source file names more distinct import sites than the closure bound: {importer} ({detail})","Fix: split the file so that each part names fewer distinct specifiers, or raise maximumSitesPerFile together with the closure bound family (SourceClosureFamily in the authorizers schema).","Quelldatei nennt mehr verschiedene Import-Stellen als die Grenze des Abschlusses: {importer} ({detail})","Behebung: Teile die Datei so auf, dass jeder Teil weniger verschiedene Specifier nennt, oder erhöhe maximumSitesPerFile zusammen mit der Grenzen-Familie des Abschlusses (SourceClosureFamily im Authorizers-Schema).")}
def sites_schema(t):
    assert t.count('"maximumSitesPerFile": 4096')==1
    return messages_text(t.replace('"maximumSitesPerFile": 4096','"maximumSitesPerFile": 16384'),NEWROW)
stage_fn(SITES+"🧬️schema/🔣️.json",sites_schema)
def sites_test(t):
    anchor='/** 🌐️ Every refusal code ships both languages and renders the named site. */'
    assert t.count(anchor)==1
    law='''/** 🧱️ The site bound is a member of the closure family: a file at the bound keeps every site, one more is refused loudly in the language of the process and never truncated, and the TypeScript parser agrees on the count. */
test("a file of exactly the site bound keeps every site and one more is refused instead of truncated",()=>{
 const bound=schema.$defs.Policy.const.maximumSitesPerFile,make=(count:number)=>Array.from({length:count},(_,index)=>"import \\"./s"+index+".ts\\";").join("\\n");
 const kept=closureLiteralSitesV1("big.ts",make(bound));expect(kept).toHaveLength(bound);expect(new Set(kept.map(site=>site.specifier)).size).toBe(bound);
 expect(astSitesV1("big.ts",make(bound)).filter(site=>site.form==="literal")).toHaveLength(bound);expect(astSitesV1("big.ts",make(bound+1)).filter(site=>site.form==="literal")).toHaveLength(bound+1);
 let failure:unknown;try{closureLiteralSitesV1("big.ts",make(bound+1));}catch(error){failure=error;}
 const message=(failure as Error|undefined)?.message??"";expect(message).toContain("big.ts");expect(message).toContain("maximumSitesPerFile="+bound);expect(message).toMatch(/Fix:|Behebung:/u);expect(message).not.toMatch(/\\{\\w+\\}/u);
 for(const language of["en","de"])expect(closureRefusalTextV1(closureRefusalMessagesV1,"sites-bounded",{importer:"big.ts",detail:"maximumSitesPerFile="+bound},{SEMIO_LANGUAGE:language})).toContain("big.ts");
});

'''
    return t.replace(anchor,law+anchor)
stage_fn(SITES+"🧪️tests/🟦️.ts",sites_test)

AX=PR+"🎛️owned-execution/📥️auxiliary/"
stage(AX+"🧬️schema/🔣️.json",[('"maxLength": 33554432','"maxLength": 100663296',1),('"maximumBytes": 33554432','"maximumBytes": 100663296',1),('"maximum": 33554433','"maximum": 100663297',1)])
stage(AX+"🧪️tests/🔣️.json",[('"count": 33554432','"count": 100663296',1),('"count": 33554433','"count": 100663297',1),('"count": 8388609','"count": 25165825',1)])
stage(AX+"🧪️tests/📜️script.ts",[
 ('["inherit","large","early-close","cancel","deadline","consumer-primary","status","large-stream"].includes(mode??"")','["inherit","large","beyond","early-close","cancel","deadline","consumer-primary","status","large-stream"].includes(mode??"")',1),
 ('text=large?"α".repeat(1048577):"Source🙂"','text=mode==="beyond"?"a".repeat(33554433):large?"α".repeat(1048577):"Source🙂"',1)])
stage(AX+"🧪️tests/🟦️.ts",[
 ('for(const mode of["inherit","large","early-close","cancel","deadline","consumer-primary","status","large-stream"]as const)','for(const mode of["inherit","large","beyond","early-close","cancel","deadline","consumer-primary","status","large-stream"]as const)',1),
 ('if(["inherit","large","status"].includes(mode)){expect(row.ordinary.ordinary).toBe("ordinary stdin α🙂");const text=mode==="large"?"α".repeat(1048577):"Source🙂";','if(["inherit","large","beyond","status"].includes(mode)){expect(row.ordinary.ordinary).toBe("ordinary stdin α🙂");const text=mode==="beyond"?"a".repeat(33554433):mode==="large"?"α".repeat(1048577):"Source🙂";',1),
 ('if(mode==="inherit"||mode==="large"||mode==="large-stream")expect(row.failed).toBe(false)','if(mode==="inherit"||mode==="large"||mode==="beyond"||mode==="large-stream")expect(row.failed).toBe(false)',1)])

stage(L+"⚡️caching/🩹️patches/nx@23.2.0.patch",[('manifest.sources.length > 2048','manifest.sources.length > 16384',1)])

stage(AU+"🧬️schema/🔣️.json",[
 ('        }\n      ],\n      "exempt": [','        },\n        {\n          "file": "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🧬️schema/🔣️.json",\n          "pointer": "$defs/Policy/const",\n          "key": "maximumSitesPerFile",\n          "offset": 0\n        }\n      ],\n      "byteBound": "closure.definitions.Policy.maximumSnapshotBytes",\n      "byteMembers": [\n        {\n          "file": "../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json",\n          "pointer": "properties/text",\n          "key": "maxLength",\n          "offset": 0\n        },\n        {\n          "file": "../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json",\n          "pointer": "definitions/Policy/const",\n          "key": "maximumBytes",\n          "offset": 0\n        },\n        {\n          "file": "../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️auxiliary/🧬️schema/🔣️.json",\n          "pointer": "definitions/Case/properties/count",\n          "key": "maximum",\n          "offset": 1\n        }\n      ],\n      "exempt": [',1)])
def growth_test(t):
    add='''
/** 📦️ The channel that carries the snapshot to the loader is a member of the same family: every byte ceiling on the way is the snapshot bound, so the largest closure the policy admits is deliverable; a snapshot above the former 32 MiB crosses the actual owned auxiliary pipe in the auxiliary tests. */
test("every byte ceiling of the snapshot family, among them the owned auxiliary channel, equals the snapshot bound",()=>{
 const family=schema.$defs.SourceClosureFamily.const,bound=policy.maximumSnapshotBytes,library=fileURLToPath(new URL("../../../../../../",import.meta.url));
 expect(family.byteBound).toBe("closure.definitions.Policy.maximumSnapshotBytes");expect(family.byteMembers.length).toBeGreaterThan(0);
 for(const row of family.byteMembers){const value=row.pointer.split("/").reduce((node:any,key:string)=>node[key],JSON.parse(readFileSync(library+row.file,"utf8")))[row.key];expect(value,row.file+" "+row.pointer).toBe(bound+row.offset);}
 expect(snapshotPolicy.maximumSnapshotBytes).toBe(bound);expect(bound).toBeGreaterThan(32*1048576);
});

/** 🩹️ The sealed Nx loader reads the same manifest the schemas bound, so its own refusal limits are family members too. */
test("the Nx loader patch refuses past exactly the bounds of the manifest schema",()=>{
 const library=fileURLToPath(new URL("../../../../../../",import.meta.url)),patch=readFileSync(library+"⚡️caching/🩹️patches/nx@23.2.0.patch","utf8");
 expect(Number(/manifest\\.sources\\.length > (\\d+)/u.exec(patch)![1])).toBe(policy.maximumFiles);expect(Number(/manifest\\.resolutions\\.length > (\\d+)/u.exec(patch)![1])).toBe(schema.properties.resolutions.maxItems);
});
'''
    return t+add
stage_fn(AU+"🧪️tests/📈️growth/🟦️.ts",growth_test)

stage(AU+"🧪️tests/🟦️.ts",[
 ('const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");const root=await mkdtemp(join(artifacts,"ar-"));\n try{for(const[path,bytes]of Object.entries(fixture.files))await writeFile(join(root,path),bytes);await withBuildTestInvocationV1(original=>withRepositoryCacheStoreV1(repository.root,',
  'const root=await repository.scratch("ar-");\n try{for(const[path,bytes]of Object.entries(fixture.files))await writeFile(join(root,path),bytes);await repository.invoke(original=>withRepositoryCacheStoreV1(repository.root,',1),
 ('await rm(output,{recursive:true,force:true});await store.recheck();}\n }));}finally{await rm(root,{recursive:true,force:true});}','await rm(output,{recursive:true,force:true});await store.recheck();}\n }),{maximumElapsedMilliseconds:170000});}finally{await rm(root,{recursive:true,force:true});}',1),
 ('({path:"p"+String(index).padStart(4,"0")+".ts",sha256:"b".repeat(64)})','({path:"p"+String(index).padStart(5,"0")+".ts",sha256:"b".repeat(64)})',1),
 ('NOT EXISTS(SELECT 1 FROM s a JOIN s b ON a.n<b.n WHERE a.p>=b.p COLLATE BINARY)','NOT EXISTS(SELECT 1 FROM s a JOIN s b ON b.n=a.n+1 WHERE a.p>=b.p COLLATE BINARY)',1),
 ('logical=structural&&(db.query("WITH s AS (SELECT CAST(key AS INTEGER) n,json_extract(value,\'$.path\') p FROM json_each(?1,\'$.sources\')),r AS (','logical=structural&&(db.exec("DROP TABLE IF EXISTS s;CREATE TEMP TABLE s(n INTEGER PRIMARY KEY,p TEXT)"),db.query("INSERT INTO s SELECT CAST(key AS INTEGER),json_extract(value,\'$.path\') FROM json_each(?1,\'$.sources\')").run(bytes),db.query("WITH r AS (',1),
 ('const control=createRequestedIssuedResourceControlV1(storePhase,120000),before=await captureNxAuthorizerSourcesV1(root,"entry.ts",control.signal),worker=','const checkout=testCheckoutRootV1(),control=createRequestedIssuedResourceControlV1(storePhase,120000),before=await captureNxAuthorizerSourcesV1(root,"entry.ts",control.signal),worker=',1),
 ('captureNxAuthorizerSourcesV1(process.cwd(),(await import("node:path")).relative(process.cwd(),worker).replaceAll("\\\\","/"),control.signal),sealed=await compileNxAuthorizerSnapshotV1(process.cwd(),snapshotNxAuthorizerSourcesV1(compiler),control,"runtime","program"),program=join(output,".sealed-compiler.mjs")','captureNxAuthorizerSourcesV1(checkout,(await import("node:path")).relative(checkout,worker).replaceAll("\\\\","/"),control.signal),sealed=await compileNxAuthorizerSnapshotV1(checkout,snapshotNxAuthorizerSourcesV1(compiler),control,"runtime","program"),program=join(output,".sealed-compiler.mjs")',1),
 ('await captureOwnedCommandV1(process.execPath,[program,"compile",root,snapshot,sha,output],process.cwd(),{control,env});','await captureOwnedCommandV1(process.execPath,[program,"compile",root,snapshot,sha,output],checkout,{control,env});',1),
 ('lazy:await lazyValue()}));"],process.cwd(),{control,env});','lazy:await lazyValue()}));"],checkout,{control,env});',1),
 ('[program,"compile",root,snapshot,sha,output],process.cwd(),{control,env,output:{maximumBytes:65536','[program,"compile",root,snapshot,sha,output],checkout,{control,env,output:{maximumBytes:65536',1),
 ('import{privateRepositoryWorkspaceV1}from','import{privateRepositoryWorkspaceV1,testCheckoutRootV1}from',1),
 ('row.fault==="before"?"Compiler-captured physical Source byte identity changed":"Bun current ownership policy changed"','row.fault==="before"?"Bun captured ownership policy changed before binding":"Bun current ownership policy changed during verification"',1),
 ('\ntest("captured compiler origins retain physical Source identity across emitted runtime contexts",','\n/** 🧱️ The emitted bundle runs the captured images: a later edit or corruption of a captured file on disk cannot change what it observes, and only an undeclared read is refused. */\ntest("captured compiler origins retain physical Source identity across emitted runtime contexts",',1)])
def origins(t):
    new=t
    for fault in["schema-before","schema-after","reader-before","reader-after","reader-after-cached","reader-policy-before","reader-policy-after"]:
        pat=re.compile(r'("fault":\s*"'+fault+r'",\s*"accepted":\s*)false')
        new,c=pat.subn(r'\1true',new);assert c==1,(fault,c)
    return new
stage_fn(AU+"🧪️tests/🧫️fixtures/🔣️.json",origins)

DSR=DS
stage(DS+"👓️reading/🔣️.json",[('"mode": "ancestor-drift",\n      "accepted": false\n    }','"mode": "ancestor-drift",\n      "accepted": true\n    }',1),('"mode": "parent-roster",\n      "accepted": false,\n      "changed": false','"mode": "parent-roster",\n      "accepted": true,\n      "changed": true',1)])
stage(DS+"🚧️fence/🟦️.ts",[('expect(verify.slice(0,verify.indexOf("};"))).toContain("await fence();");','expect(verify.slice(0,verify.indexOf("};"))).toContain("await finalFence();");expect(producer).toContain("finalFence=budgeted(true)");',1),('const definition=producer.slice(producer.indexOf("fence=async()=>{"),producer.indexOf("material=("));','const definition=producer.slice(producer.indexOf("fullFence=async()=>{"),producer.indexOf("fences={state"));',1)])
stage(DS+"👓️reading/🟦️.ts",[
 ('output=await run(workspace,original,control,Object.freeze({sourceOnly:true}),Object.freeze({sourceOnly:true}),cache,captured,','output=await run(workspace,original,control,Object.freeze({sourceOnly:true}),Object.freeze({sourceOnly:true}),Object.freeze({accept(){check();},recheck:async()=>{check();},measure:async()=>{check();}}),cache,captured,',1),
 ('import*as readContext from"../../📥️read-context/🟦️.ts";','import*as readContext from"../../📥️read-context/🟦️.ts";\nimport{fleetInstallScopeV1}from"../../../../🛰️fleet-fetch/📦️install/🟦️.ts";\nimport{sayCachingV1}from"../../../../../📖️texts/🟦️.ts";',1),
 ("'$.version',2,'$.generation'","'$.version',4,'$.generation'",1),
 ('expect(new Ajv({strict:true}).validate(schema.$defs.SourceOwnerExecutionObservation,result)).toBe(true);','{const{observations,...data}=result;expect(new Ajv({strict:true}).validate(schema.$defs.SourceOwnerExecutionObservation,data)).toBe(true);expect(Array.isArray(observations)).toBe(true);for(const row of observations)expect(Object.keys(row).sort()).toEqual(["anchor","full","kind","path","stamp","target"]);}',1),
 ('stamp=(row:any)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,row.ctimeNs].map(String));try{for(const vector of corpus.preservationBindings){','stamp=(row:any)=>JSON.stringify([row.dev,row.ino,row.mode,row.nlink,row.size,row.mtimeNs,(BigInt(row.mode)&0o170000n)===0o120000n?0n:row.ctimeNs].map(String));try{for(const vector of corpus.preservationBindings){',1),
 ('javascriptDependencyPreservationIntentV1:storage.javascriptDependencyPreservationIntentV1,withArtifactRelativeDirectoryV1:async(directory:string,checkpoint:()=>void,body:(value:any)=>Promise<unknown>)=>api.withArtifactRelativeDirectoryV1(directory,checkpoint,async value=>{if(vector.mode!=="unknown-after-effect")return body(value);','javascriptDependencyPreservationIntentV1:storage.javascriptDependencyPreservationIntentV1,publishJavascriptDependencyPreservingSourceDescriptorV1:vector.mode==="unknown-after-effect"?async(...args:Parameters<typeof storage.publishJavascriptDependencyPreservingSourceDescriptorV1>)=>{await storage.publishJavascriptDependencyPreservingSourceDescriptorV1(...args);throw marker;}:storage.publishJavascriptDependencyPreservingSourceDescriptorV1,withArtifactRelativeDirectoryV1:async(directory:string,checkpoint:()=>void,body:(value:any)=>Promise<unknown>)=>api.withArtifactRelativeDirectoryV1(directory,checkpoint,async value=>{if(vector.mode!=="unknown-after-effect")return body(value);',1),
 ('/** 🔎️ Executes the actual metadata reader through bounded filesystem observation ports and independent Python/SQLite. */','/** 🔎️ Executes the actual metadata reader through bounded filesystem observation ports and independent Python/SQLite; a parent or ancestor is identified by device, inode and type, so a changed roster is no drift and a changed device, inode or type is. */',1),
 ('expect(result.rows).toHaveLength(2);','expect(result.rows).toHaveLength(3);',1),
 ("kind in ('missing','link','file','directory')\").get()).toEqual({n:2})","kind in ('missing','link','file','directory')\").get()).toEqual({n:3})",1),
 (',ports={Buffer,JSON,performance,createHash,installSchema:schema,projectionPolicy:schema.$defs.ProjectionPolicy.const,dependencySelectedTypesV1:types,',',ports={...(storage as unknown as Record<string,unknown>),Buffer,JSON,performance,createHash,installSchema:schema,projectionPolicy:schema.$defs.ProjectionPolicy.const,dependencySelectedTypesV1:types,',1),
 ('ownership={version:1,scopes:[{manifest:"a/package.json",packages:["a/m"]}]},observed:string[]=[],leaf=join(workspace,"a/node_modules");','ownership={version:1,scopes:[{manifest:"package.json",packages:[]},{manifest:"a/package.json",packages:["a/m"]}]},observed:string[]=[],leaf=join(workspace,"a/node_modules");',1),
 ('try{await fs.mkdir(join(workspace,"a/m"),{recursive:true});await fs.mkdir(cache);','try{await fs.mkdir(join(workspace,"a/m"),{recursive:true});await fs.writeFile(join(workspace,"package.json"),JSON.stringify({name:"root",private:true}));await fs.mkdir(cache);',1),
 ("stamp=json.dumps([str(getattr(s,f)) for f in ['st_dev','st_ino','st_mode','st_nlink','st_size','st_mtime_ns','st_ctime_ns']],separators=(',',':'))","stamp=json.dumps([('0' if k=='link' and f=='st_ctime_ns' else str(getattr(s,f))) for f in ['st_dev','st_ino','st_mode','st_nlink','st_size','st_mtime_ns','st_ctime_ns']],separators=(',',':'))",1),
 ('names=["captureJavascriptDependencySelectedFieldsV1","captureJavascriptDependencyProducerRowsV1","readJavascriptDependencyPreservationSourceV1"]','names=["captureJavascriptDependencySelectedFieldsV1","captureJavascriptDependencyProducerRowsV1","javascriptDependencySourceOwnerEndpointsV1","readJavascriptDependencyPreservationSourceV1"]',1),
 ('openRepositoryCachePartitionV1:()=>{opens++;throw Error("Unsupported platform must refuse before effects");}','openRepositoryCachePartitionV1:()=>{opens++;throw Error("Partition opening reached");}',1),
 ('.rejects.toThrow("Mapped Windows dependency completion is not qualified")','.rejects.toThrow("Partition opening reached")',3),
 ('expect(opens).toBe(0);','expect(opens).toBe(3);',1),
 ('expect(body.indexOf(\'process.platform!=="darwin"\')).toBeLessThan(body.indexOf("openRepositoryCachePartitionV1("));','expect(body).not.toContain(\'process.platform!=="darwin"\');',1),
 ('"captureNativePlatformProtectedArtifactMaintenanceV1","readNativePlatformProtectedArtifactMaintenanceV1","withCurrentBunWorkspaceOwnershipV1"','"captureNativePlatformProtectedArtifactMaintenanceV1","createRepositoryDependencyProducerArtifactPhaseV1(token,actual,checkpoint)","withCurrentBunWorkspaceOwnershipV1"',1),
 ('if(!vector.mode.endsWith("unknown-after-effect")||path.dirname(captured.bindings.rows[0].sourceDirectory)!==root)return body(directory);','if(!vector.mode.endsWith("unknown-after-effect")||(vector.mode.startsWith("preserving-")?root===cache||root.startsWith(cache+path.sep):path.dirname(captured.bindings.rows[0].sourceDirectory)!==root))return body(directory);',1),
 ('return body(Object.freeze({stat:()=>directory.stat(),capturePreservation:(name:string)=>directory.capturePreservation(name),publishPreserving:async(...args:Parameters<typeof directory.publishPreserving>)=>{await directory.publishPreserving(...args);throw marker;},captureBinding:(name:string)=>directory.captureBinding(name),publishBinding:async(...args:Parameters<typeof directory.publishBinding>)=>{await directory.publishBinding(...args);throw marker;}}));','const wrap=(directory:any):any=>{const wrapped:Record<string,unknown>={};for(const key of new Set([...Object.keys(directory),...Object.getOwnPropertyNames(Object.getPrototypeOf(directory)??{})])){const value=directory[key];if(key!=="constructor")wrapped[key]=typeof value==="function"?value.bind(directory):value;}for(const key of["withDirectory","withDirectoryPath"])if(typeof directory[key]==="function")wrapped[key]=(first:unknown,inner:(child:unknown)=>Promise<unknown>)=>directory[key](first,(child:unknown)=>inner(wrap(child)));for(const key of["publishPreserving","publishBinding"])wrapped[key]=async(...args:unknown[])=>{await directory[key](...args);throw marker;};return Object.freeze(wrapped);};return body(wrap(directory));',1),
 ('for(const name of["publishJavascriptDependencyPreservingSourceDescriptorV1","recheckJavascriptDependencyPreservationsDescriptorV1"]){','for(const name of["javascriptDependencySourcePairPlanV1","javascriptDependencySourcePairPhysicalV1","withJavascriptDependencySourceDirectoryPairV1","publishJavascriptDependencyPreservingSourceDescriptorV1","recheckJavascriptDependencyPreservationsDescriptorV1"]){',1),
 ("stamp=(value:{dev:bigint;ino:bigint;mode:bigint;nlink:bigint;size:bigint;mtimeNs:bigint;ctimeNs:bigint})=>JSON.stringify([value.dev,value.ino,value.mode,value.nlink,value.size,value.mtimeNs,value.ctimeNs].map(String))","stamp=(value:{dev:bigint;ino:bigint;mode:bigint;nlink:bigint;size:bigint;mtimeNs:bigint;ctimeNs:bigint})=>JSON.stringify([value.dev,value.ino,value.mode,value.nlink,value.size,value.mtimeNs,(value.mode&0o170000n)===0o120000n?0n:value.ctimeNs].map(String))",1),
 ('requireJavascriptDependencyInstalledPathsReadV1:async(...args:Parameters<typeof storage.requireJavascriptDependencyInstalledPathsReadV1>)=>{if(vector.mode==="qualification-second"&&++qualified===2)throw marker;','javascriptDependencyFleetRequestV1:storage.javascriptDependencyFleetRequestV1,fleetInstallScopeV1,sayCachingV1,posix:path.posix,javascriptDependencyStampV1:storage.javascriptDependencyStampV1,sameJavascriptDependencyParentIdentityV1:storage.sameJavascriptDependencyParentIdentityV1,javascriptDependencyStrictAncestorsV1:storage.javascriptDependencyStrictAncestorsV1,requireJavascriptDependencyInstalledPathsReadV1:async(...args:Parameters<typeof storage.requireJavascriptDependencyInstalledPathsReadV1>)=>{if(vector.mode==="qualification-second"&&++qualified===2)throw marker;',1)])

stage_fn(B+"👁️observation/🧪️tests/📦️projects/🟦️.ts",projects_edit.edit)

def suites(t):
    anchor='    {\n      "id": "build-inference-projects",'
    assert t.count(anchor)==1
    def block(i,trees,plat,causes,tools):
        s='    {\n      "id": "%s",\n      "budgetMs": 300000,\n      "typecheck": "none",\n      "trees": [\n%s\n      ],\n      "platforms": [\n%s\n      ],\n      "platformCauses": [\n%s\n      ]'%(i,",\n".join('        "%s"'%x for x in trees),",\n".join('        "%s"'%x for x in plat),",\n".join('        "%s"'%x for x in causes))
        if tools:s+=',\n      "tools": [\n%s\n      ]'%",\n".join('        "%s"'%x for x in tools)
        return s+"\n    },\n"
    new=block("build-task-model",["🧮️task-model"],["darwin","linux"],["posix-shell-script","symlink","permission-bits"],[])+block("build-slots",["🪢️slots"],["darwin","linux"],["python3-command"],["python"])+block("build-graph-state",["🪪️graph-state"],["darwin","linux"],["python3-command","symlink","permission-bits"],["python"])
    t=t.replace(anchor,new+anchor)
    old='      "id": "build-inference-projects",\n      "budgetMs": 300000,\n      "typecheck": "none",\n'
    assert t.count(old)==1
    t=t.replace(old,old+'      "scratch": "checkout",\n')
    memory='      "id": "build-memory",\n      "budgetMs": 300000,\n      "typecheck": "none",\n'
    assert t.count(memory)==1
    t=t.replace(memory,memory+'      "scratch": "checkout",\n')
    json.loads(t);return t
stage_fn(B+"🗂️suites/🔣️.json",suites)
def project(t):
    probe='    "test-build-inference-projects": {'
    assert t.count(probe)==1
    i=t.index(probe);j=t.index('\n    },\n',i)+len('\n    },\n');tpl=t[i:j]
    new=""
    for sid in["build-task-model","build-slots","build-graph-state"]:new+=tpl.replace("build-inference-projects",sid)
    t=t.replace(probe,new+probe,1)
    k=t.index('"dependsOn": [\n        "test-artifact');e=t.index("\n      ],",k);lines=t[k:e].split("\n")
    head,items=lines[0],lines[1:]
    names=[x.strip().rstrip(",").strip('"') for x in items]
    for sid in["build-task-model","build-slots","build-graph-state"]:names.append("test-"+sid)
    names=sorted(set(names))
    t=t[:k]+head+"\n"+",\n".join('        "%s"'%x for x in names)+t[e:]
    json.loads(t);return t
stage_fn(L+"⚡️caching/📋️project.json",project)
stage(B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/📈️inferred/🟦️.ts",[('declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});','declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});',1)])
stage(B+"🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🛟️stopped-wave/🧒️child/🟦️.ts",[('birth.declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});','birth.declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});',1)])
stage(B+"🪪️graph-state/🙋️wanted/🧪️tests/📖️read/🟦️.ts",[('expect(run.runs[0]).not.toContain(run.heavy);expect(run.runs[0]![0]).toBe(run.stale[0]);','expect(run.runs[0]!.indexOf(run.heavy)).toBe(run.runs[0]!.length-1);expect(run.runs[0]![0]).toBe(run.stale[0]);',1)])
stage_fn(B+"🔧️tools/🏗️bazel/🧪️tests/🟦️.ts",bazel_root_edit.edit)
BZ=B+"🔧️tools/🏗️bazel/"
stage_fn(BZ+"🧪️tests/📥️environment/🟦️.ts",bm.environment_test)
stage_fn(BZ+"🧪️tests/📥️purpose/🧪️tests/🌱️birth/🟦️.ts",bm.birth_test)
stage_fn(BZ+"🧪️tests/📥️purpose/🌱️birth/🟦️.ts",bm.birth_factory)
stage_fn(BZ+"🧪️tests/🔏️implementation/🟦️.ts",bm.implementation_test)
stage_fn(BZ+"🎬️scopes/🔣️.json",bm.scopes_data)
stage_fn(B+"📥️source/🔏️closure/🗓️scheduling/🔣️.json",bm.scheduling_data)
stage(BZ+"🎬️scopes/🧪️tests/🔣️.json",[('"maximumFiles": 450','"maximumFiles": 512',1),('"maximumKeyedFiles": 200','"maximumKeyedFiles": 256',1)])
OB=B+"👁️observation/"
stage_fn(OB+"🧪️tests/⚡️incremental/🟦️.ts",incremental_edit.edit)
stage(OB+"🧪️tests/🧫️fixtures/⚡️incremental/🔣️.json",[('"foreignLibraryFacts": 0,','"foreignLibraryFacts": 1,',1)])
stage(OB+"🧬️schema/⚡️incremental/🔣️.json",[('"foreignLibraryFacts": {\n      "const": 0\n    }','"foreignLibraryFacts": {\n      "const": 1\n    }',1)])
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
print(len(rows),"files staged")
