/** 🔍️ Per-command census of module edges: follows relative edges and workspace-bare edges (through package exports) to a fixpoint and classifies every bare specifier as workspace or external; usage: bun census.ts <root> <entries.json> [out.json]. */
import {readFileSync,writeFileSync} from "node:fs";
import {isBuiltin} from "node:module";
import {posix} from "node:path";
import {diskZeroTouchTreeV1,scanModuleSpecifiersV1,workspaceSpecifierTargetV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
const [rootArgument,entriesArgument,outArgument]=process.argv.slice(2),tree=diskZeroTouchTreeV1(rootArgument!),entries=JSON.parse(readFileSync(entriesArgument!,"utf8")) as Record<string,string[]>;
const CODE=/\.(?:[cm]?[jt]s|tsx)$/u,EXT=[".ts",".tsx",".mts",".js",".mjs"];
const relativeTarget=(from:string,specifier:string)=>{const base=posix.normalize(posix.join(posix.dirname(from),specifier.split("?")[0]!));for(const candidate of [base,...EXT.map(extension=>base+extension)])if(CODE.test(candidate)&&tree.read(candidate)!==undefined)return candidate;return undefined;};
type Edge={file:string;specifier:string;kind:string;target?:string};
const result:Record<string,{files:number;list:string[];workspace:Edge[];external:Edge[];unresolved:Edge[]}>={};
for(const [id,starts] of Object.entries(entries)){
  const seen=new Set<string>(),pending=[...starts],workspace:Edge[]=[],external:Edge[]=[],unresolved:Edge[]=[];
  while(pending.length){
    const file=pending.pop()!;
    if(seen.has(file))continue;
    seen.add(file);
    const text=tree.read(file);
    if(text===undefined||!CODE.test(file))continue;
    for(const {specifier,kind} of scanModuleSpecifiersV1(file,text)){
      if(specifier.startsWith(".")){const target=relativeTarget(file,specifier);if(target)pending.push(target);else{const data=posix.normalize(posix.join(posix.dirname(file),specifier.split("?")[0]!));if(tree.read(data)!==undefined)seen.add(data);}}
      else if(specifier.startsWith("node:")||specifier.startsWith("bun:")||specifier==="bun"||isBuiltin(specifier))continue;
      else if(Object.keys(tree.packages).some(name=>specifier===name||specifier.startsWith(name+"/"))){
        const target=workspaceSpecifierTargetV1(tree.packages,specifier);
        if(target===undefined||tree.read(target)===undefined)unresolved.push({file,specifier,kind});
        else{workspace.push({file,specifier,kind,target});pending.push(target);}
      }else external.push({file,specifier,kind});
    }
  }
  result[id]={files:seen.size,list:[...seen].filter(file=>tree.read(file)!==undefined).sort(),workspace,external,unresolved};
}
const summary=Object.fromEntries(Object.entries(result).map(([id,row])=>[id,{files:row.files,workspace:row.workspace.length,external:[...new Set(row.external.map(edge=>edge.specifier))].length,unresolved:row.unresolved.length}]));
if(outArgument)writeFileSync(outArgument,JSON.stringify(result,null,1));
console.log(JSON.stringify(summary,null,1));
