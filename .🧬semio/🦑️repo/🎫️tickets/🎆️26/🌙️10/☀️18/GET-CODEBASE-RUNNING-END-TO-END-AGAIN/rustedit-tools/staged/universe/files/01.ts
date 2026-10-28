import {GlobV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🌐️glob/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Policy.const,prunable=new RegExp(policy.prunableNegation,"u");

/** 🌌️ The directories a world witness observes for one plan: complete walks below `roots`, and a pruned walk of the whole checkout when a pattern is root-anchored.
 * @see ./🧬️schema/🔣️.json */
export interface BazelSourceUniverseV1{readonly roots:readonly string[];readonly whole:null|"unbounded"|Readonly<{names:readonly string[];dot:boolean}>}

/** 🧭️ The directory below which every path a pattern can match lies: its literal prefix cut back to the last complete directory; `""` is the checkout root. */
export function bazelPatternRootV1(pattern:string):string{
 const index=pattern.search(/[?*{[]/u),prefix=index<0?pattern:pattern.slice(0,index),cut=index<0||prefix.endsWith("/")?prefix:prefix.slice(0,prefix.lastIndexOf("/")+1);
 const root=cut.replaceAll("\\","/").replace(/^\.\//u,"").replace(/\/+$/u,"");
 return root==="."?"":root;
}

/** 🪜️ Drops every root that lies below another root of the list; `""` subsumes all. */
export function bazelMinimalRootsV1(roots:readonly string[]):string[]{
 const sorted=[...new Set(roots)].sort();if(sorted[0]==="")return [""];const selected=new Set(sorted);
 return sorted.filter(root=>{for(let index=root.indexOf("/");index>=0;index=root.indexOf("/",index+1))if(selected.has(root.slice(0,index)))return false;return true;});
}

/** ✂️ The directory names one task's negations exclude completely, each confirmed by the runtime glob on the policy probes. */
export function bazelPrunedNamesV1(negations:readonly string[]):Readonly<{names:readonly string[];dot:boolean}>{
 const names=new Set<string>();let dot=false;
 const excludes=(glob:GlobV1,name:string)=>policy.probes.every(probe=>glob.match(probe.replaceAll("{name}",name)));
 for(const negation of negations){
  const shape=prunable.exec(negation);if(!shape)continue;const glob=new GlobV1(negation),body=shape[1]!;
  for(const name of body.startsWith("{")?body.slice(1,-1).split(","):[body]){
   if(name===policy.dotSegment){if(excludes(glob,policy.dotProbeName))dot=true;}
   else if(excludes(glob,name))names.add(name);
  }
 }
 return Object.freeze({names:Object.freeze([...names].sort()),dot});
}

/** 🌌️ Derives the universe of a plan from the expanded inputs of its tasks: a root-anchored pattern prunes only the names its own task excludes, and the whole walk prunes the names all root-anchored patterns agree on. */
export function bazelSourceUniverseV1(groups:readonly (readonly string[])[]):BazelSourceUniverseV1{
 const roots:string[]=[];let whole=false,unbounded=false,names:Set<string>|undefined,dot=true;
 for(const group of groups){
  let pruned:ReturnType<typeof bazelPrunedNamesV1>|undefined;
  for(const pattern of group){
   if(pattern.startsWith("!"))continue;const root=bazelPatternRootV1(pattern);
   if(root!==""){roots.push(root);continue;}
   whole=true;pruned??=bazelPrunedNamesV1(group.filter(value=>value.startsWith("!")).map(value=>value.slice(1)));
   const own=new Set(pruned.names);names=names===undefined?own:new Set([...names].filter(name=>own.has(name)));dot&&=pruned.dot;
  }
  if(pruned&&!pruned.names.length&&!pruned.dot)unbounded=true;
 }
 const minimal=bazelMinimalRootsV1(roots);
 if(!whole)return Object.freeze({roots:Object.freeze(minimal),whole:null});
 if(unbounded||!names!.size&&!dot)return Object.freeze({roots:Object.freeze(minimal),whole:"unbounded"});
 return Object.freeze({roots:Object.freeze(minimal),whole:Object.freeze({names:Object.freeze([...names!].sort()),dot})});
}

/** 🚧️ Whether the pruned whole walk skips a directory entry of this name. */
export function bazelUniversePrunesV1(whole:Readonly<{names:readonly string[];dot:boolean}>,name:string):boolean{return whole.dot&&name.startsWith(".")||whole.names.includes(name);}
