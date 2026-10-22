#!/usr/bin/env python3
"""Anchored patch of closure/🟦️.ts: installed-package and root targets, resolve-only outside bases, owner-aware schema reads, EN/DE refusals. Usage: patch_closure.py <file>."""
import sys
path = sys.argv[1]
t = open(path, encoding="utf8").read()
def rep(a, b):
    global t
    n = t.count(a)
    assert n == 1, (n, a[:120])
    t = t.replace(a, b)

rep('import{nxAuthorizerInstalledPackagesV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";',
    'import{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1}from"../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";')

rep('const policy=schema.definitions.Policy.const,hash=',
    'const installedTargetPattern=/^installed:(?:@[a-z0-9._-]+\\/)?[a-z0-9._-]+@[^\\s]{1,64}$/u,installedTargetV1=(target:string)=>target.startsWith(closureTargetSchemesV1.installed)&&installedTargetPattern.test(target);\nconst policy=schema.definitions.Policy.const,hash=')

rep('const selected=new Map<string,string>();let previous="";for(const row of value.sources)',
    'const selected=new Map<string,string>();let previous="";for(const row of value.sources)')

rep('if(!specifier||!result.has(target))throw Error("Complete admitted Source resolution target required");pathIn(root,target);}return result;',
    'if(!specifier)throw Error("Complete admitted Source resolution target required");if(installedTargetV1(target))continue;if(!result.has(target))throw Error("Complete admitted Source resolution target required");pathIn(root,target);}return result;')

rep('if(!row||!Object.hasOwn(row.imports,specifier)||!files.has(row.imports[specifier]!))throw Error("Admitted implementation declaring image and import required");return row.imports[specifier]!;}',
    'if(!row||!Object.hasOwn(row.imports,specifier)||!installedTargetV1(row.imports[specifier]!)&&!files.has(row.imports[specifier]!))throw Error("Admitted implementation declaring image and import required");return row.imports[specifier]!;}')

rep('for(const specifier of Object.keys(selected.imports))pending.push(resolveSourceImportV1(admitted,path,specifier));',
    'for(const specifier of Object.keys(selected.imports)){const target=resolveSourceImportV1(admitted,path,specifier);if(!installedTargetV1(target))pending.push(target);}')

# loader resolution validation
rep("if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}",
    "if(typeof target==='string'&&/^installed:(?:@[a-z0-9._-]+[/])?[a-z0-9._-]+@[^\\s]{1,64}$/.test(target)&&specifier)continue;if(typeof target!=='string'||!specifier||path.isAbsolute(target)||target.split('/').some(part=>!part||part==='.'||part==='..')||target.includes(String.fromCharCode(92))||!images.has(path.join(value.root,target)))throw Error('Closed captured resolution target required');}")

# refusal text
rep("const capturedSourceImportDiagnosticV1=(${capturedSourceImportDiagnosticV1.toString()}),sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>Error((kind==='import'?'Uncaptured implementation import refused: ':kind==='require'?'Uncaptured implementation require refused: ':'Captured require declaring module required: ')+JSON.stringify(capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage)));",
    "const capturedSourceImportDiagnosticV1=(${capturedSourceImportDiagnosticV1.toString()}),closureRefusalTextV1=(${closureRefusalTextV1.toString()}),refusalMessages=${JSON.stringify(closureRefusalMessagesV1)},rootScheme=${JSON.stringify(closureTargetSchemesV1.root)},installedScheme=${JSON.stringify(closureTargetSchemesV1.installed)},refusal=(code,fields)=>Error(closureRefusalTextV1(refusalMessages,code,fields,process.env)),sourceImportRefusal=(kind,specifier,importer,declaringImage,declared,targetImage)=>{const diagnostic=capturedSourceImportDiagnosticV1(kind,specifier,importer&&path.isAbsolute(importer)?path.relative(value.root,importer).split(path.sep).join('/'):importer,declaringImage,declared,targetImage),error=refusal(kind,{specifier:diagnostic.specifier,importer:diagnostic.importer});error.message+=' '+JSON.stringify(diagnostic);return error;};")

# schema reads are refused only for captured owners
rep("const originalRead=fs.readFileSync,currentImage=",
    "const originalRead=fs.readFileSync,codeDirectories=new Set([...images.keys()].filter(name=>/\\.[cm]?[jt]sx?$/.test(name)).map(name=>path.dirname(name))),capturedSchemaOwner=file=>{if(path.extname(file)!=='.json')return false;const relative=path.relative(value.root,file);if(!relative||relative.startsWith('..')||path.isAbsolute(relative))return false;const parts=relative.split(path.sep),at=parts.indexOf('🧬️schema');return at>=0&&codeDirectories.has(path.join(value.root,...parts.slice(0,at)));},currentImage=")
rep("if(file&&path.extname(file)==='.json'&&path.relative(value.root,file).split(path.sep).includes('🧬️schema'))throw Error('Uncaptured implementation schema read refused');return originalRead(name,options);};",
    "if(file&&capturedSchemaOwner(file))throw refusal('schema-read',{specifier:path.relative(value.root,file).split(path.sep).join('/')});return originalRead(name,options);};")
rep("if(file&&(images.has(file)||path.extname(file)==='.json'&&path.relative(value.root,file).split(path.sep).includes('🧬️schema')))return capturedRead(name,options);",
    "if(file&&(images.has(file)||capturedSchemaOwner(file)))return capturedRead(name,options);")

# createRequire
rep("const installedPackages=new Set(${JSON.stringify(nxAuthorizerInstalledPackagesV1)}),nativeRequire=require,commonJs=new Map(),capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);const actual=awaitModule.createRequire(base),resolve=specifier=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(installedPackages.has(specifier))return actual.resolve(specifier);const target=resolutions.get(path.resolve(name))?.[specifier];if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);return path.join(value.root,target);},load=specifier=>{if(installedPackages.has(specifier))return actual(specifier);const target=resolve(specifier);",
    "const nativeRequire=require,commonJs=new Map(),installedTarget=target=>typeof target==='string'&&target.startsWith(installedScheme),declaredTarget=(importer,specifier)=>{const map=resolutions.get(importer);if(!map)return undefined;if(Object.hasOwn(map,specifier))return map[specifier];const file=specifier.startsWith('file:')?url.fileURLToPath(specifier):specifier;if(!path.isAbsolute(file))return undefined;const relative=path.relative(value.root,file);return relative&&!relative.startsWith('..')&&!path.isAbsolute(relative)&&Object.hasOwn(map,rootScheme+relative.split(path.sep).join('/'))?map[rootScheme+relative.split(path.sep).join('/')]:undefined;},installedChecked=new Map(),resolving={depth:0},physicalResolve=(requireFrom,specifier)=>{resolving.depth++;try{return requireFrom.resolve(specifier);}finally{resolving.depth--;}},installedResolve=(requireFrom,specifier,target)=>{const resolved=physicalResolve(requireFrom,specifier);if(installedChecked.get(specifier)!==target){const marker=resolved.lastIndexOf(path.sep+'node_modules'+path.sep),pieces=marker<0?[]:resolved.slice(marker+path.sep.length+13).split(path.sep),segments=target.slice(installedScheme.length).replace(/@[^@]*$/,'').split('/'),directory=marker<0?'':resolved.slice(0,marker+path.sep.length+13)+pieces.slice(0,segments.length).join(path.sep);let version;try{version=JSON.parse(originalRead(path.join(directory,'package.json'),'utf8')).version;}catch{version=undefined;}if(!directory||installedScheme+segments.join('/')+'@'+version!==target)throw Object.assign(refusal('installed-drift',{specifier,detail:target.slice(installedScheme.length)+' -> '+(version??'unreadable')}),{code:'SEMIO_INSTALLED_DRIFT'});installedChecked.set(specifier,target);}return resolved;},capturedCreateRequire=base=>{const name=base instanceof URL||typeof base==='string'&&base.startsWith('file:')?url.fileURLToPath(base):String(base);const actual=awaitModule.createRequire(base),resolve=specifier=>{check();if(typeof specifier!=='string')throw Error('Closed captured require specifier required');specifier=({'./.semio-captured-fs':'node:fs','./.semio-captured-fs-promises':'node:fs/promises','./.semio-captured-module':'node:module'})[specifier]??specifier;if(awaitModule.isBuiltin(specifier))return specifier;if(!images.has(path.resolve(name)))return physicalResolve(actual,specifier);const target=declaredTarget(path.resolve(name),specifier);if(!target)throw sourceImportRefusal('require',specifier,name,true,false,false);if(installedTarget(target))return installedResolve(actual,specifier,target);return path.join(value.root,target);},load=specifier=>{if(!images.has(path.resolve(name)))throw sourceImportRefusal('declaring-module',undefined,name,false,false,false);if(installedTarget(declaredTarget(path.resolve(name),specifier))){installedResolve(actual,specifier,declaredTarget(path.resolve(name),specifier));return actual(specifier);}const target=resolve(specifier);")

rep("build.onResolve({filter:/.*/},item=>{if(item.path.startsWith('node:')","build.onResolve({filter:/.*/},item=>{if(resolving.depth||item.path.startsWith('node:')")
rep("const module=await import(entry);check();","for(const[name,map]of resolutions)for(const[specifier,target]of Object.entries(map))if(installedTarget(target)&&installedChecked.get(specifier)!==target)try{installedResolve(awaitModule.createRequire(name),specifier,target);}catch(error){if(error?.code==='SEMIO_INSTALLED_DRIFT')throw error;}\nconst module=await import(entry);check();")
# static import resolution
rep("const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&resolutions.get(importer)?.[item.path],candidate=declared?path.join(value.root,declared):",
    "const importer=item.importer?.replace(/^[/]?semio-source:/,''),declared=importer&&declaredTarget(importer,item.path);if(declared&&installedTarget(declared)){installedResolve(awaitModule.createRequire(importer),item.path,declared);return;}if(importer&&!images.has(importer)&&importer.split(path.sep).includes('node_modules'))return;const candidate=declared?path.join(value.root,declared):")
open(path, "w", encoding="utf8").write(t)
print("patched", path)
