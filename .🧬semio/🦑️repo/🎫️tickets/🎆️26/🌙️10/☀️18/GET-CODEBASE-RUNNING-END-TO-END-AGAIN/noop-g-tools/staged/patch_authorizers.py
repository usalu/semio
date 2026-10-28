#!/usr/bin/env python3
"""Patches the live authorizers module with the source memo: `patch_authorizers.py <in> <out>` (each anchor must occur exactly once)."""
import sys

def patch(s: str) -> str:
    def once(old: str, new: str) -> None:
        nonlocal s
        if s.count(old) != 1:
            raise SystemExit("anchor count %d: %s" % (s.count(old), old[:90]))
        s = s.replace(old, new)

    once('import {lstat,open,opendir,readFile,realpath} from "node:fs/promises";',
         'import {lstat,open,opendir,readFile,realpath} from "node:fs/promises";\n'
         'import{nxAuthorizerMemoDirectoriesV1,nxAuthorizerMemoImplementationV1,readNxAuthorizerMemoV1,replayNxAuthorizerMemoV1,writeNxAuthorizerMemoV1,type NxAuthorizerObservationV1}from"./🧊️memo/🟦️.ts";')

    once('export interface NxAuthorizerCaptureOptionsV1{readonly installed?:boolean;readonly roots?:ReadonlyMap<string,readonly string[]>}',
         'export interface NxAuthorizerCaptureOptionsV1{readonly installed?:boolean;readonly roots?:ReadonlyMap<string,readonly string[]>;readonly memo?:Readonly<{file:string;served?:(served:"memo"|"capture")=>void}>}')

    once('const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>(),capturedInstalled=new WeakMap<NxAuthorizerSourcesV1,Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>>();',
         'const capturedBytes=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Uint8Array>>(),capturedInstalled=new WeakMap<NxAuthorizerSourcesV1,Readonly<{installed:Readonly<Record<string,string>>;roots:ReadonlyMap<string,readonly string[]>}>>(),capturedImports=new WeakMap<NxAuthorizerSourcesV1,ReadonlyMap<string,Readonly<Record<string,string>>>>();')

    once('interface RuntimeDataDiscoveryV1{readonly directories:Set<string>;readonly results:Set<string>;entries:number;bytes:number}',
         'interface RuntimeDataDiscoveryV1{readonly directories:Set<string>;readonly results:Set<string>;readonly observed?:NxAuthorizerObservationV1[];entries:number;bytes:number}')

    once('try{ancestry=await discoveryAncestorsV1(root,current.path,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}',
         'try{ancestry=await discoveryAncestorsV1(root,current.path,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.observed?.push({kind:"absent",path:current.path});continue;}throw error;}')

    once('const stream=await opendir(sourcePath(root,current.path),{bufferSize:1});let failed=false,primary:unknown;',
         'const stream=await opendir(sourcePath(root,current.path),{bufferSize:1}),seen:(readonly[string,"d"|"f"])[]=[];let failed=false,primary:unknown;')

    once('if(facts.isSymbolicLink())throw Error("No-follow authorizer DATA entry required");',
         'if(facts.isSymbolicLink())throw Error("No-follow authorizer DATA entry required");seen.push([entry.name,facts.isDirectory()?"d":"f"]);')

    once('await discoveryDirectoryHoldsV1(ancestry,signal);signal.throwIfAborted();\n }',
         'await discoveryDirectoryHoldsV1(ancestry,signal);signal.throwIfAborted();state.observed?.push({kind:"directory",path:current.path,entries:seen.sort((a,b)=>compare(a[0],b[0]))});\n }')

    once('try{const parent=dirname(child);ancestry=parent==="."?[]:await discoveryAncestorsV1(root,parent,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}',
         'try{const parent=dirname(child);ancestry=parent==="."?[]:await discoveryAncestorsV1(root,parent,signal);}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.observed?.push({kind:"absent",path:child});continue;}throw error;}')

    once('facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}',
         'facts=await lstat(sourcePath(root,child),{bigint:true});signal.throwIfAborted();}catch(error){signal.throwIfAborted();if((error as NodeJS.ErrnoException).code==="ENOENT"){state.observed?.push({kind:"absent",path:child});continue;}throw error;}')

    once('if(row.lenient){try{await lstat(sourcePath(root,child));}catch(error){signal.throwIfAborted();continue;}signal.throwIfAborted();}',
         'if(row.lenient){try{await lstat(sourcePath(root,child));}catch(error){signal.throwIfAborted();observed.push({kind:"absent",path:child});continue;}signal.throwIfAborted();}')

    once('if(installed&&installedPathV1(candidate)){marks.set(specifier,await installedMarkV1(root,candidate,specifier,versions,signal));continue;}',
         'if(installed&&installedPathV1(candidate)){const mark=await installedMarkV1(root,candidate,specifier,versions,signal);marks.set(specifier,mark);observed.push({kind:"package",specifier,from:dirname(path),candidate,mark});continue;}')

    once('resolutions.set(specifier,child);\n',
         'resolutions.set(specifier,child);observed.push({kind:"package",specifier,from:dirname(path),candidate:child,mark:null});\n')

    once('}catch(error){signal.throwIfAborted();if(row.lenient)continue;throw error instanceof Error',
         '}catch(error){signal.throwIfAborted();if(row.lenient){observed.push({kind:"package",specifier,from:dirname(path),candidate:null,mark:null});continue;}throw error instanceof Error')

    once('discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),entries:0,bytes:0};let total=0;',
         'observed:NxAuthorizerObservationV1[]=[],discovery:RuntimeDataDiscoveryV1={directories:new Set(),results:new Set(),observed,entries:0,bytes:0};let total=0;')

    once('export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal,options:NxAuthorizerCaptureOptionsV1={}):Promise<NxAuthorizerSourcesV1>{\n',
         'export async function captureNxAuthorizerSourcesV1(workspace:string,entry:string,signal:AbortSignal,options:NxAuthorizerCaptureOptionsV1={}):Promise<NxAuthorizerSourcesV1>{\n'
         ' const remembered=options.memo!==undefined&&options.roots===undefined?await replayCapturedV1(workspace,entry,signal,options.installed===true,options.memo.file):undefined;\n'
         ' if(remembered){options.memo!.served?.("memo");return remembered;}\n')

    once('capturedInstalled.set(result,Object.freeze({installed:Object.freeze(Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right)))),roots}));return result;',
         'capturedInstalled.set(result,Object.freeze({installed:Object.freeze(Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right)))),roots}));\n'
         ' if(options.memo!==undefined&&options.roots===undefined){options.memo.served?.("capture");await rememberCapturedV1(root,entry,installed,options.memo.file,result,marks,observed);}\n'
         ' return result;')

    once('export function nxAuthorizerCapturedImportsV1(workspace:string,source:NxAuthorizerSourcesV1):ReadonlyMap<string,Readonly<Record<string,string>>>{return nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions,capturedInstalled.get(source));}',
         'export function nxAuthorizerCapturedImportsV1(workspace:string,source:NxAuthorizerSourcesV1):ReadonlyMap<string,Readonly<Record<string,string>>>{return capturedImports.get(source)??nxAuthorizerImportProjectionV1(workspace,nxAuthorizerCapturedBytesV1(source),source.manifest.resolutions,capturedInstalled.get(source));}')

    once('export async function captureNxAuthorizerSourcesV1(',
         '''/** 💾️ Remembers a finished capture together with the filesystem facts it depended on, so that the next process proves it instead of repeating it; a memo that cannot be written costs only that next capture. */
    async function rememberCapturedV1(root:string,entry:string,installed:boolean,file:string,result:NxAuthorizerSourcesV1,marks:ReadonlyMap<string,string>,observations:readonly NxAuthorizerObservationV1[]):Promise<void>{
     const directories=await nxAuthorizerMemoDirectoriesV1(root,result.manifest.sources.map(row=>row.path));
     if(!directories)return;
     const imports=nxAuthorizerImportProjectionV1(root,nxAuthorizerCapturedBytesV1(result),result.manifest.resolutions,capturedInstalled.get(result));
     capturedImports.set(result,imports);
     await writeNxAuthorizerMemoV1(file,{version:1,installed,implementation:nxAuthorizerMemoImplementationV1(),root,entry,directories,observations,manifest:result.manifest,marks:Object.fromEntries([...marks].sort(([left],[right])=>compare(left,right))),imports:Object.fromEntries(imports)});
    }
    /** 🔁️ Serves a capture from its memo when the present tree proves every fact the memo rests on: file digests, ancestor identities, remembered absences and listings and each package resolution; otherwise nothing. */
    async function replayCapturedV1(workspace:string,entry:string,signal:AbortSignal,installed:boolean,file:string):Promise<NxAuthorizerSourcesV1|undefined>{
     const root=resolve(workspace),memo=await readNxAuthorizerMemoV1(file);
     if(!memo||memo.entry!==entry||memo.root!==root||memo.installed!==installed)return undefined;
     const contents=await replayNxAuthorizerMemoV1(root,memo,signal);
     if(!contents)return undefined;
     const versions=new Map<string,string>();
     for(const row of memo.observations){
      if(row.kind!=="package")continue;
      let candidate:string|null=null,mark:string|null=null;
      try{
       const located=await runtimeResolveV1(row.specifier,join(root,row.from));signal.throwIfAborted();
       const actual=await realpath(located),physicalRoot=await realpath(root);signal.throwIfAborted();
       candidate=relative(physicalRoot,actual).replaceAll("\\\\","/");
       if(installed&&installedPathV1(candidate))mark=await installedMarkV1(root,candidate,row.specifier,versions,signal);
      }catch(error){signal.throwIfAborted();if(row.candidate===null)continue;return undefined;}
      if(row.candidate===null||candidate!==row.candidate||mark!==row.mark)return undefined;
     }
     const manifest:NxAuthorizerSourceManifestV1=Object.freeze({version:1,mode:"release",entry:memo.manifest.entry,resolutions:Object.freeze(memo.manifest.resolutions.map(row=>Object.freeze({specifier:row.specifier,path:row.path}))),sources:Object.freeze(memo.manifest.sources.map(row=>Object.freeze({path:row.path,sha256:row.sha256})))});
     const result=Object.freeze({manifest,digest:hash(JSON.stringify(manifest)+"\\n")});
     capturedBytes.set(result,contents);
     if(installed)capturedInstalled.set(result,Object.freeze({installed:Object.freeze({...memo.marks}),roots:computedRoots}));
     capturedImports.set(result,new Map(Object.entries(memo.imports).map(([path,map])=>[path,Object.freeze({...map})] as const)));
     return result;
    }
    export async function captureNxAuthorizerSourcesV1(''')


    return s


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    open(dst, "w", encoding="utf8").write(patch(open(src, encoding="utf8").read()))
    print("patched", dst)
