#!/usr/bin/env python3
"""Anchored patch of authorizers/🟦️.ts: derived literal sites, installed packages, semio-root imports. Usage: patch_authorizers.py <file> (applies in place; every anchor must match exactly once)."""
import sys
path = sys.argv[1]
t = open(path, encoding="utf8").read()
def rep(a, b):
    global t
    n = t.count(a)
    assert n == 1, (n, a[:100])
    t = t.replace(a, b)

rep('import authorizerSchema from "./🧬️schema/🔣️.json" with { type: "json" };',
    'import authorizerSchema from "./🧬️schema/🔣️.json" with { type: "json" };\nimport {closureLiteralSitesV1,closurePackageNameV1,closureSpecifierKindV1,closureTargetSchemesV1} from "./🧭️sites/🟦️.ts";')
rep('import {lstat,open,opendir,realpath} from "node:fs/promises";', 'import {lstat,open,opendir,readFile,realpath} from "node:fs/promises";')
rep('import {isBuiltin} from "node:module";\n', '')

old_companions = 'runtimeDataCompanions=new Map<string,readonly string[]>([...authorizerSchema.$defs.RuntimeDataCompanions.const.map(row=>[row.owner,Object.freeze([...row.paths])] as const),...authorizerSchema.$defs.ComputedImportSites.const.filter(row=>row.roots.length).map(row=>[row.owner,Object.freeze([...row.roots])] as const)]);'
assert t.count(old_companions) == 1
new_companions = ('computedRoots=new Map<string,readonly string[]>(authorizerSchema.$defs.ComputedImportSites.const.filter(row=>row.roots.length).map(row=>[row.owner,Object.freeze([...row.roots])] as const)),'
                  'runtimeDataCompanions=new Map<string,readonly string[]>([...new Set([...authorizerSchema.$defs.RuntimeDataCompanions.const.map(row=>row.owner),...computedRoots.keys()])].map(owner=>[owner,Object.freeze([...authorizerSchema.$defs.RuntimeDataCompanions.const.filter(row=>row.owner===owner).flatMap(row=>row.paths),...(computedRoots.get(owner)??[])])] as const));')
t = t.replace(old_companions, new_companions)

start = t.index('/** 🔎️ Reads scanner imports plus literal relative createRequire')
end = t.index('function scannedImport(path:string):string{')
t = t[:start] + '''/** 🔎️ Reads the literal sites a capture follows: scanner imports and relative createRequire calls, plus, when third-party packages are derived, every literal require, resolve and package site. */
function scannedImportRowsV1(path:string,source:string,installed:boolean):readonly {path:string;lenient:boolean}[]{
 return closureLiteralSitesV1(path,source).filter(row=>installed||row.scanned||row.kind==="create-require"&&row.specifier.startsWith(".")).map(row=>({path:row.specifier,lenient:row.lenient}));
}
''' + t[end:]

rep('const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>();',
    '''const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>(),capturedInstalled=new WeakMap<NxAuthorizerSourcesV1,Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>>();
/** 🛠️ Selects what a capture derives beyond the compiler toolkit: installed third-party packages and computed-site roots for command closures. */
export interface NxAuthorizerCaptureOptionsV1{readonly installed?:boolean;readonly roots?:ReadonlyMap<string,readonly string[]>}''')

# capture
rep('export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal):Promise<NxAuthorizerSourcesV1>{',
    '''function installedPathV1(inside:string):boolean{return !inside.startsWith("../")&&inside.split("/").includes("node_modules");}
/** 📦️ Pins one installed third-party package by name and installed version, the declared dependency image a command closure resolves natively. */
async function installedMarkV1(root:string,inside:string,specifier:string,versions:Map<string,string>,signal:AbortSignal):Promise<string>{
 const parts=inside.split("/"),at=parts.lastIndexOf("node_modules"),name=closurePackageNameV1(specifier),count=name.split("/").length,directory=parts.slice(0,at+1+count).join("/");
 let version=versions.get(directory);if(version===undefined){const manifest=join(root,directory,"package.json"),facts=await lstat(manifest);signal.throwIfAborted();if(!facts.isFile()||facts.size>1_048_576)throw Error("Bounded installed package manifest required: "+directory);const value:unknown=JSON.parse(Buffer.from(await readFile(manifest)).toString("utf8"));signal.throwIfAborted();version=value!==null&&typeof value==="object"&&typeof (value as {version?:unknown}).version==="string"?(value as {version:string}).version:"unversioned";versions.set(directory,version);}
 return closureTargetSchemesV1.installed+name+"@"+version;
}
export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal,options:NxAuthorizerCaptureOptionsV1={}):Promise<NxAuthorizerSourcesV1>{''')
rep('const root=resolve(workspace),files=new Map<string,string>(),contents=new Map<string,Uint8Array>(),pending=[entry],resolutions=new Map<string,string>(),discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),entries:0,bytes:0};let total=0;',
    'const installed=options.installed===true,roots=options.roots??computedRoots,root=resolve(workspace),files=new Map<string,string>(),contents=new Map<string,Uint8Array>(),pending=[entry],resolutions=new Map<string,string>(),marks=new Map<string,string>(),versions=new Map<string,string>(),discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),entries:0,bytes:0};let total=0;')
rep('  for(const companion of runtimeDataCompanions.get(path)??[]){sourcePath(root,companion);pending.push(companion);}',
    '  for(const companion of [...(runtimeDataCompanions.get(path)??[]),...(options.roots?.get(path)??[])]){sourcePath(root,companion);pending.push(companion);}')
old_loop_start = '  const scan=scannedImportRowsV1(path,source);\n'
i = t.index(old_loop_start)
j = t.index(' }\n const manifest:NxAuthorizerSourceManifestV1=Object.freeze(', i)
new_loop = '''  for(const row of scannedImportRowsV1(path,source,installed)){
   const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);
   if(kind==="builtin"||installed&&(kind==="absolute"||kind==="scheme")||!installed&&external.has(specifier))continue;
   let child:string;
   if(kind==="relative"){child=relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\\\","/");if(row.lenient){try{await lstat(sourcePath(root,child));}catch(error){signal.throwIfAborted();continue;}signal.throwIfAborted();}}
   else{const earlier=resolutions.get(specifier);if(earlier!==undefined)child=earlier;else if(marks.has(specifier))continue;else{
    try{const rootBytes=await capture("package.json"),located=await Bun.resolve(specifier,dirname(join(root,path)));signal.throwIfAborted();const actual=await realpath(located);signal.throwIfAborted();const candidate=relative(root,actual).replaceAll("\\\\","/");
     if(installed&&installedPathV1(candidate)){marks.set(specifier,await installedMarkV1(root,candidate,specifier,versions,signal));continue;}
     sourcePath(root,candidate);const manifest=nxAuthorizerWorkspaceCandidateV1(rootBytes,candidate);await capture(manifest);signal.throwIfAborted();child=resolveNxAuthorizerWorkspaceV1(nxAuthorizerWorkspaceIndexV1(contents),specifier);if(child!==candidate)throw Error("Located package differs from its captured exact export");resolutions.set(specifier,child);
    }catch(error){signal.throwIfAborted();if(row.lenient)continue;throw error;}
   }}
   sourcePath(root,child);pending.push(child);
  }
'''
t = t[:i] + new_loop + t[j:]
rep('const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\\n")});capturedBytes.set(result,contents);return result;',
    'const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\\n")});capturedBytes.set(result,contents);if(installed)capturedInstalled.set(result,Object.freeze({installed:Object.freeze(Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right)))),roots}));return result;')

# projection
ps = t.index('/** 🕸️ Resolves only executable compiler imports')
pe = t.index('/** 🧷️ Reads import edges only from an actual privately issued compiler capture. */')
t = t[:ps] + '''/** 🕸️ Resolves only executable compiler imports against the same privately captured byte census; with installed markers it also resolves third-party packages and computed-site roots. */
export function nxAuthorizerImportProjectionV1(workspace:string,bytes:ReadonlyMap<string,Uint8Array>,resolutions:readonly NxAuthorizerResolutionV1[],derived?:Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>):ReadonlyMap<string,Readonly<Record<string,string>>>{
 const root=resolve(workspace),packages=new Map(resolutions.map(row=>[row.specifier,row.path])),result=new Map<string,Readonly<Record<string,string>>>();
 for(const[path,value]of bytes){const imports:Record<string,string>={};if(extname(path)!==".json"){const text=Buffer.from(value).toString("utf8").replace(/^#![^\\r\\n]*/u,"");
  for(const row of scannedImportRowsV1(path,text,derived!==undefined)){const specifier=scannedImport(row.path),kind=closureSpecifierKindV1(specifier);if(kind==="builtin"||derived!==undefined&&(kind==="absolute"||kind==="scheme")||derived===undefined&&external.has(specifier))continue;const target=kind==="relative"?relative(root,resolve(dirname(join(root,path)),specifier)).replaceAll("\\\\","/"):packages.get(specifier)??derived?.installed[specifier];if(!target||!target.startsWith(closureTargetSchemesV1.installed)&&!bytes.has(target)){if(row.lenient&&derived!==undefined)continue;throw Error("Complete immutable compiler import required: "+specifier);}if(!target.startsWith(closureTargetSchemesV1.installed))sourcePath(root,target);imports[specifier]=target;}
  if(derived!==undefined)for(const target of derived.roots.get(path)??[])if(bytes.has(target)){sourcePath(root,target);imports[closureTargetSchemesV1.root+target]=target;}}
  result.set(path,Object.freeze(imports));}
 return result;}
''' + t[pe:]
rep('return nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions);}',
    'return nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions,capturedInstalled.get(source));}')
rep('try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal)).digest===original.digest;}',
    'try{return (await captureNxAuthorizerSourcesV1(root,original.manifest.entry,signal,{installed:capturedInstalled.has(original),roots:capturedInstalled.get(original)?.roots})).digest===original.digest;}')
open(path, "w", encoding="utf8").write(t)
print("patched", path)
