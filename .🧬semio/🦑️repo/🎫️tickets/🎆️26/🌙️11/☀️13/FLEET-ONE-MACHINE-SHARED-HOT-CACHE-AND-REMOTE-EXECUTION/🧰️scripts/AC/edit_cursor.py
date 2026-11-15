import sys
src,dst=sys.argv[1],sys.argv[2]
t=open(src).read()
def rep(old,new):
    global t
    assert t.count(old)==1,(t.count(old),old[:60])
    t=t.replace(old,new)
rep('import schema from "./🧬️schema/🔣️.json" with {type:"json"};',
    'import {createVerdictMemoV1,verdictCodeDigestV1,verdictKeyV1} from "./🧠️verdicts/🟦️.ts";\nimport schema from "./🧬️schema/🔣️.json" with {type:"json"};')
rep('export interface ManifestJournalPortV1 extends JournalPortV1{readonly watches:"governed";owns(instance:string):Promise<boolean>;readonly store:SnapshotStoreV1;lastReason():string|undefined}',
    'export interface ManifestJournalPortV1 extends JournalPortV1{readonly watches:"governed";owns(instance:string):Promise<boolean>;readonly store:SnapshotStoreV1;lastReason():string|undefined;flushVerdicts():void}')
rep(' let reason:string|undefined,shared:((path:string)=>boolean)|undefined,seen:{files:Set<string>;directories:Set<string>}|undefined;',
    ' let reason:string|undefined,shared:((path:string)=>boolean)|undefined,seen:{files:Set<string>;directories:Set<string>}|undefined,verdicts:ReturnType<typeof createVerdictMemoV1>|undefined;')
rep(''' const boundary=async()=>{
  const text=await readFile(join(options.root,".gitignore"),"utf8").catch(()=>""),ignore=compileIgnoreV1(text);
  shared=path=>refusePathV1(path)===undefined&&sharedV1(path,true,ignore);
 };''',''' const boundary=async()=>{
  const text=await readFile(join(options.root,".gitignore"),"utf8").catch(()=>""),ignore=compileIgnoreV1(text);
  shared=path=>refusePathV1(path)===undefined&&sharedV1(path,true,ignore);
  verdicts=createVerdictMemoV1({directory:join(store.directory,"..","verdicts"),key:codeDigest===undefined?undefined:verdictKeyV1({ignoreText:text,code:codeDigest})});
 };''')
rep(' const now=async():Promise<SnapshotV1|undefined>=>{',
    ' const codeDigest=verdictCodeDigestV1(VERDICT_CODE_FILES_V1),now=async():Promise<SnapshotV1|undefined>=>{')
rep('''  excluded(){
   const inside=shared,known=seen,memo=new Map<string,boolean>();
   return(path:string)=>{
    if(path===""||known?.files.has(path)||known?.directories.has(path))return false;
    let verdict=memo.get(path);
    if(verdict===undefined){verdict=classifyGraphInputV1(path)==="ignored"||inside===undefined||!inside(path);memo.set(path,verdict);}
    return verdict;
   };
  },''','''  flushVerdicts(){verdicts?.save();},
  excluded(){
   const inside=shared,known=seen,remembered=verdicts,memo=new Map<string,boolean>();
   return(path:string)=>{
    if(path===""||known?.files.has(path)||known?.directories.has(path))return false;
    let verdict=memo.get(path);
    if(verdict===undefined){
     verdict=inside===undefined?undefined:remembered?.get(path);
     if(verdict===undefined){verdict=classifyGraphInputV1(path)==="ignored"||inside===undefined||!inside(path);if(inside!==undefined)remembered?.set(path,verdict);}
     memo.set(path,verdict);
    }
    return verdict;
   };
  },''')
rep('const alive=(pid:number)',
'''/** 🧾️ The files whose code decides the verdict of a path that no snapshot names: the input classification and the roster behind it, the ignore rules and the path refusals of the boundary. */
const VERDICT_CODE_FILES_V1:readonly string[]=["../🟦️.ts","../🧬️schema/🔣️.json","../../🕸️graph/🧭️roster/🟨️.mjs","../../../../🤝️shared-worktree/🚧️boundary/🟦️.ts","../../../../🤝️shared-worktree/🚧️boundary/🔤️fold/🔣️.json"].map(path=>fileURLToPath(new URL(path,import.meta.url)));

const alive=(pid:number)''')
rep('import {join} from "node:path";','import {fileURLToPath} from "node:url";\nimport {join} from "node:path";')
open(dst,'w').write(t)
