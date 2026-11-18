import sys,os,shutil
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
root=sys.argv[1] if len(sys.argv)>1 else "/Users/ueli/Documents/semio"
G=L+"🕸️graph/"
pairs={
G+"🚪️admission/🟦️.ts":[
('files:string[];observedFiles?:string[];dependencies:string[];dependencyInputs?','files:string[];dependencies:string[];dependencyInputs?'),
('export interface BuildOwnerAdmissionV1{owners:string[];files:string[];observedFiles?:string[];','export interface BuildOwnerAdmissionV1{owners:string[];files:string[];'),
('files=new Set<string>(),admitting=new Set<string>(),observed=new Set<string>(),providers=','files=new Set<string>(),providers='),
('for(const file of value.files){files.add(file);admitting.add(file);const dependency=sourceOwner(file);if(dependency)admit(dependency);}for(const file of value.observedFiles??[]){files.add(file);observed.add(file);}','for(const file of value.files){files.add(file);const dependency=sourceOwner(file);if(dependency)admit(dependency);}'),
('const foreign=[...observed].filter(file=>!admitting.has(file)).sort(),sorted=(values:Map<string,Set<string>>)','const sorted=(values:Map<string,Set<string>>)'),
('files:[...files].sort(),...(foreign.length?{observedFiles:foreign}:{}),','files:[...files].sort(),'),
('/** 🔏️ Carries exact original owner observations and source-owned dependency identities; `files` admit the owners that hold them, `observedFiles` are file facts of foreign owners that are members and invalidate but admit nothing. */','/** 🔏️ Carries exact original owner observations and source-owned dependency identities. */')],
G+"📥️inference/🔎️owner-discovery/🟨️.mjs":[
('observedFiles=new Set(),dependencies=new Set(),','dependencies=new Set(),'),
('observedFiles.add(file);}for(const contribution','addFile(file);}for(const contribution'),
('for(const path of matched)observedFiles.add(path);','for(const path of matched)addFile(path);'),
('...(observedFiles.size?{observedFiles:[...observedFiles].filter(file=>!files.has(file)).sort()}:{}),','')],
G+"🧭️discovery/🟦️.ts":[
(',foreign=new Set(admission?.observedFiles??[]);',';'),
('files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")?!foreign.has(file):javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),{...options,catalogFiles:[...foreign].sort()},{workspaceRoot:root,invocation,phaseControl,registry});','files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,{workspaceRoot:root,invocation,phaseControl,registry});')],
G+"📥️inference/📦️project-nodes/🟨️.mjs":[
('const catalogFiles=options?.catalogFiles?.length?[...found.files,...inferenceCandidates(options.catalogFiles,workspaceRoot)]:found.files;return{catalog,files:found.files,catalogFiles,','return{catalog,files:found.files,'),
('playground:collectPlaygroundCatalog(catalogFiles,workspaceRoot,catalog)','playground:collectPlaygroundCatalog(found.files,workspaceRoot,catalog)'),
('catalogContributions=setup.catalog,catalogFiles=setup.catalogFiles,','catalogContributions=setup.catalog,'),
('playgroundSessionTargets(catalogFiles, workspaceRoot,catalogContributions)','playgroundSessionTargets(configFiles, workspaceRoot,catalogContributions)'),
('playgroundPreparationTargets(catalogFiles, workspaceRoot, root,','playgroundPreparationTargets(configFiles, workspaceRoot, root,'),
('componentCrateRoots(catalogFiles, workspaceRoot)','componentCrateRoots(configFiles, workspaceRoot)')],
}
G2=L+"🧬️schema/🚪️admission.json"
pairs[G2]=[('        "observedFiles": {\n          "description": "File facts of other owners that this owner only reads as inputs (for example the Cargo manifests a plugin catalog consumer hashes): members of the closure whose change invalidates it, never a reason to admit the owner that holds them. A real dependency edge or an entry of files admits.",\n          "type": "array",\n          "uniqueItems": true,\n          "items": {\n            "type": "string",\n            "minLength": 1,\n            "maxLength": 256\n          }\n        },\n',""),('    "observedFiles": {\n      "description": "The members of files that admit no owner: file facts of foreign owners, observed with their digest, invalidating, never admitting.",\n      "type": "array",\n      "uniqueItems": true,\n      "items": {\n        "type": "string",\n        "minLength": 1,\n        "maxLength": 256\n      },\n      "maxItems": 1048576\n    },\n',"")]
check=len(sys.argv)>2 and sys.argv[2]=="check"
for rel,rows in pairs.items():
    p=os.path.join(root,rel);s=open(p,encoding="utf8").read()
    for a,b in rows:
        if s.count(a)!=1: print("ANCHOR",rel[-40:],a[:60],s.count(a)); sys.exit(1)
        s=s.replace(a,b)
    if not check: open(p,"w",encoding="utf8").write(s)
print("ok")
