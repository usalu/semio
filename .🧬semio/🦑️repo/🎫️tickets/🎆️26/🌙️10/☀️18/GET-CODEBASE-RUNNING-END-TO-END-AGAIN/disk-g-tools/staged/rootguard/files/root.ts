import {lstatSync,readFileSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {cacheTextV1} from "../../../🧾️ledger/🟦️.ts";

const markers=Object.freeze(["📋️project.json","package.json"]),scratch=new Set([".cache",".tmp",".🧬semio"]),roots=new Map<string,string>();
/** 🧭️ The checkout root at or above a directory: the nearest directory with the paired build and package markers whose package declares workspaces, the same rule as the workspace root of the process module; `null` outside any checkout. @see https://git-scm.com/docs/git-rev-parse#Documentation/git-rev-parse.txt---show-toplevel */
export function repositoryCheckoutRootV1(start:string):string|null{
 const first=resolve(start),known=roots.get(first);if(known!==undefined)return known;
 for(let directory=first;;directory=dirname(directory)){
  if(markers.every(name=>lstatSync(join(directory,name),{throwIfNoEntry:false})?.isFile())){
   try{const {workspaces}=JSON.parse(readFileSync(join(directory,"package.json"),"utf8")) as {workspaces?:unknown};if(Array.isArray(workspaces)||workspaces!==null&&typeof workspaces==="object"){roots.set(first,directory);return directory;}}catch{}
  }
  if(dirname(directory)===directory)return null;
 }
}

/** 🚧️ Refuses cache storage for a directory that lies inside a checkout without being its root (a command run from a module directory would create a second set of images there, as `🧰️framework/.cache/b` did with 8.1 GiB); the checkout root, directories outside every checkout and scratch directories of the checkout (`.cache`, `.tmp`, `.🧬semio`) pass. Refused in English and German. */
export function requireRepositoryWorkspaceRootV1(workspace:string):string{
 const root=repositoryCheckoutRootV1(workspace);
 if(root===null||root===workspace||scratch.has(relative(root,workspace).split(sep)[0]!))return workspace;
 throw Error(cacheTextV1("en","workspace.nested",{workspace,root})+"\n"+cacheTextV1("de","workspace.nested",{workspace,root}));
}
