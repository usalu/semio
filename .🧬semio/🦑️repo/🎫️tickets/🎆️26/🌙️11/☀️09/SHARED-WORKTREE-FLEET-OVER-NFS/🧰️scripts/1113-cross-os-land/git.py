from lib import *
P="🌿️git/🟦️.ts"
def code(t):
    s=read(src(P))
    t=sub(t,'import {copyFile,lstat,mkdir,open,readFile,rename,rm,stat,symlink,writeFile} from "node:fs/promises";','import {copyFile,lstat,mkdir,open,readdir,readFile,rename,rm,stat,symlink,writeFile} from "node:fs/promises";\nimport {devNull} from "node:os";')
    t=sub(t,'import {delimiter,isAbsolute,join,relative,resolve} from "node:path";','import {delimiter,dirname,isAbsolute,join,relative,resolve} from "node:path";')
    t=sub(t,'import {type FleetManifestEntryV1} from "../📒️journal/🟦️.ts";','import {type FleetManifestEntryV1} from "../📒️journal/🟦️.ts";\nimport {absoluteUnitsV1,caseDependentIgnoresV1,foldNameV1,linkRefusalV1,pathBudgetV1,refusePathV1} from "../🚧️boundary/🟦️.ts";')
    t=sub(t,'const child=spawn(file,["--version"],{stdio:["ignore","pipe","ignore"],windowsHide:true});','const child=spawn(file,["--version"],{stdio:["ignore","pipe","ignore"],windowsHide:true,timeout:GIT_PROBE_MILLISECONDS});')
    t=sub(t,'const BATCH_LIMIT=5000,','const GIT_PROBE_MILLISECONDS=10_000,GIT_RUN_MILLISECONDS=600_000,BATCH_LIMIT=5000,')
    t=sub(t,'type Run=Readonly<{index?:string;stdin?:string|Buffer;signal?:AbortSignal;hash?:number;env?:Readonly<Record<string,string>>}>;\n','type Run=Readonly<{index?:string;stdin?:string|Buffer;signal?:AbortSignal;hash?:number;env?:Readonly<Record<string,string>>;view?:boolean}>;\n\n'+between(s,"/** 📌️ The `-c` pins","async function git(")+"")
    t=sub(t,''' const environment:Record<string,string|undefined>={...process.env,GIT_DIR:join(worktree,".git"),GIT_TERMINAL_PROMPT:"0",LC_ALL:"C"};
 for(const name of ["GIT_WORK_TREE","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_INDEX_FILE"])delete environment[name];''',''' const pin=await pinDirectory(worktree);
 const environment:Record<string,string|undefined>={...process.env,GIT_DIR:join(worktree,".git"),GIT_TERMINAL_PROMPT:"0",LC_ALL:"C",GIT_CONFIG_NOSYSTEM:"1",GIT_ATTR_NOSYSTEM:"1",GIT_CONFIG_GLOBAL:pin===undefined?devNull:join(pin,"config"),XDG_CONFIG_HOME:pin===undefined?devNull:join(pin,"xdg")};
 for(const name of Object.keys(environment))if(name.startsWith("GIT_CONFIG_KEY_")||name.startsWith("GIT_CONFIG_VALUE_"))delete environment[name];
 for(const name of ["GIT_WORK_TREE","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_INDEX_FILE","GIT_CONFIG_PARAMETERS","GIT_CONFIG_COUNT","GIT_CONFIG","GIT_EXTERNAL_DIFF","GIT_ATTR_SOURCE"])delete environment[name];''')
    t=sub(t,'spawn(file,["-c","gc.auto=0","-c","core.fsmonitor=false",...(options.index===undefined','spawn(file,[...GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]),...(options.view===true?GIT_VIEW_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]):[]),...(options.index===undefined')
    t=sub(t,'windowsHide:true,signal:options.signal}),out:Buffer[]=[];','windowsHide:true,signal:options.signal,timeout:GIT_RUN_MILLISECONDS}),out:Buffer[]=[];')
    # parse
    t=sub(t,'export function parseGitViewV1(porcelain:string):GitViewV1{','export function parseGitViewV1(porcelain:string,extra:Readonly<{deleted?:readonly string[];untracked?:readonly string[]}>={}):GitViewV1{')
    t=sub(t,'''  lines.push(`${x}${y} ${path}`);
 }
 lines.sort(''','''  lines.push(`${x}${y} ${path}`);
 }
 const known=new Set(lines);
 for(const path of extra.deleted??[]){const line=` D ${path.normalize("NFC")}`;if(!known.has(line)){known.add(line);lines.push(line);deleted++;}}
 for(const path of extra.untracked??[]){const line=`?? ${path.normalize("NFC")}`;if(!known.has(line)){known.add(line);lines.push(line);untracked++;}}
 lines.sort(''')
    old=between(t,"/** 🪟 Reads the working-tree view with","const trees=new Map")
    new=between(s,"/** 🪟 Reads the working-tree view with","const trees=new Map")
    t=t.replace(old,new,1)
    t=sub(t,'''const wanted:[string,string][]=[["core.autocrlf","false"],["core.filemode",platform==="win32"?"false":"true"],["core.symlinks",String(await probeSymlinks(worktree))],["core.ignorecase",String(await probeCase(worktree))],["core.fsmonitor","false"],''','''const wanted:[string,string][]=[["core.autocrlf","false"],["core.safecrlf","false"],["core.eol","lf"],["core.filemode",platform==="win32"?"false":"true"],["core.symlinks",String(await probeSymlinks(worktree))],["core.ignorecase",String(await probeCase(worktree))],["core.protectNTFS","true"],["core.protectHFS","true"],["core.fsmonitor","false"],''')
    t=sub(t,'  if(platform==="darwin")wanted.push(["core.precomposeunicode","true"]);','  if(platform==="win32")wanted.push(["core.longpaths","true"]);\n  if(platform==="darwin")wanted.push(["core.precomposeunicode","true"]);')
    t=sub(t,'/** 🛠️ Normalises `.git/config` through git itself for this machine: `core.autocrlf=false`, `core.filemode` (false on Windows),','/** 🛠️ Normalises `.git/config` through git itself for this machine: `core.autocrlf=false`, `core.safecrlf=false`, `core.eol=lf`, `core.protectNTFS=true`, `core.protectHFS=true`, `core.longpaths=true` on Windows, `core.filemode` (false on Windows),')
    return t
edit(P,code)
def more(t):
    t=sub(t,'["update-ref","-m","linearise concurrent commit",...target,tip,local],{signal});','["update-ref","-m","linearise concurrent commit",...target,tip,local],{signal,env:{GIT_COMMITTER_NAME:"semio fleet",GIT_COMMITTER_EMAIL:"fleet@semio.invalid"}});')
    t=sub(t,' stamps.delete(worktree);verdicts.delete(worktree);trees.delete(worktree);',' stamps.delete(worktree);verdicts.delete(worktree);volumes.delete(worktree);versions=new Map();caseIgnored.delete(worktree);pinned.delete(worktree);trees.delete(worktree);')
    return t
edit(P,more)
