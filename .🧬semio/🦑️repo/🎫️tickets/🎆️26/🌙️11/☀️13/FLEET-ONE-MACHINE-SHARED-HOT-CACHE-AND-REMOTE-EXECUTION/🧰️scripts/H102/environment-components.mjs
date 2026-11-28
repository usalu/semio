import{createHash}from"node:crypto";

/** 🧩️ H102 prototype: the environment of the graph as addressable components, and the re-stale rule of an owner edge record as a pure function of (record, environment). Not wired into the build library; the oracle is `oracle.py`, the check is `check.mjs`. */

export const H=value=>createHash("sha256").update(JSON.stringify(value)).digest("hex");
export function cp(a,b){const x=[...a],y=[...b];for(let i=0;i<Math.min(x.length,y.length);i++){const d=x[i].codePointAt(0)-y[i].codePointAt(0);if(d)return d;}return x.length-y.length;}
const within=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
const related=(a,b)=>within(a,b)||within(b,a);
const dirOf=path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):".";
const ancestorsOf=path=>{const found=[path];for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1))found.push(path.slice(0,cut));if(path!==".")found.push(".");return found;};
const lower=(sorted,key,at=entry=>entry[0])=>{let low=0,high=sorted.length;while(low<high){const mid=(low+high)>>>1;if(cp(at(sorted[mid]),key)<0)low=mid+1;else high=mid;}return low;};

/** 📇️ Indexes environment parts `{owners:[[name,root,native|null]],packages:[[k,v]],modules:[[k,v]],workspaces:[[manifestPath,digest]],lock,taxonomy}` once; `digest(key)` answers the digest of one component, memoised. */
export function indexEnvironmentV1(parts){
 const rows=new Map(parts.owners.map(row=>[row[0],row])),rowDigest=new Map(parts.owners.map(row=>[row[0],H(row)]));
 const entries=[];for(const[name,root,native]of parts.owners){entries.push([root,name]);if(native!==null&&native!==root)entries.push([native,name]);}
 entries.sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));
 const byLoc=new Map();for(const entry of entries)(byLoc.get(entry[0])??byLoc.set(entry[0],[]).get(entry[0])).push(entry);
 const multi=list=>{const map=new Map();for(const[k,v]of list)(map.get(k)??map.set(k,[]).get(k)).push(v);return map;};
 const packages=multi(parts.packages),modules=multi(parts.modules);
 const workspaces=parts.workspaces.map(([path,digest])=>[dirOf(path),digest]).sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));
 const strictlyInside=root=>{const start=root==="."?"":root+"/",out=[];for(let at=lower(entries,start);at<entries.length&&entries[at][0].startsWith(start);at++)if(entries[at][0]!==root)out.push(entries[at]);return out;};
 const memo=new Map();
 const entryDigest=list=>list.length?H(list.map(([loc,name])=>[loc,name,rowDigest.get(name)])):"-";
 const compute=key=>{
  const kind=key[0],arg=key.slice(2);
  if(key==="T")return String(parts.taxonomy);
  if(key==="L")return parts.lock===null||parts.lock===undefined?"-":String(parts.lock);
  if(kind==="o")return rowDigest.get(arg)??"-";
  if(kind==="t")return entryDigest(strictlyInside(arg));
  if(kind==="c"){const found=[];for(const ancestor of ancestorsOf(arg))found.push(...byLoc.get(ancestor)??[]);found.push(...strictlyInside(arg));found.sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));return entryDigest(found);}
  if(kind==="p"||kind==="m"){const list=(kind==="p"?packages:modules).get(arg);return list?H([...list].sort(cp)):"-";}
  if(kind==="w"){const list=workspaces.filter(([dir])=>dir===arg).map(row=>row[1]).sort(cp);return list.length?H(list):"-";}
  if(kind==="W"){const list=workspaces.filter(([dir])=>related(dir,arg));return list.length?H(list):"-";}
  throw Error("Closed component kind required: "+key);
 };
 return Object.freeze({rows,digest:key=>{let found=memo.get(key);if(found===undefined)memo.set(key,found=compute(key));return found;},tableDigest:H([parts.taxonomy,parts.lock??null,parts.owners,parts.packages,parts.modules,parts.workspaces])});
}

/** 🔑️ The component keys an owner edge record reads, sorted: the taxonomy, its own row and territory (both roots), every resolved owner with its territory, the lookups that found no owner, packages, modules, workspace directories, the related workspaces of its roots when it inherits from one, the lock when it consulted it. `reads` is the stored closed edge reads (`undefined` = recorded nothing = reads everything: the key `*`). */
export function recordKeysV1(row,reads,env){
 if(!reads)return["*"];
 const keys=new Set(["T","o|"+row.name]),roots=[row.root,row.nativeRoot].filter(root=>root);
 for(const root of roots)keys.add("t|"+root);
 for(const target of reads.owners??[]){keys.add("o|"+target);const found=env.rows.get(target);for(const root of found?[found[1],found[2]]:[])if(root)keys.add("t|"+root);}
 for(const path of reads.paths??[])keys.add("c|"+path);
 for(const name of reads.packages??[])keys.add("p|"+name);
 for(const name of reads.modules??[])keys.add("m|"+name);
 for(const dir of reads.workspaces??[])keys.add("w|"+dir);
 if(reads.workspaces?.length)for(const root of roots)keys.add("W|"+root);
 if(reads.lock)keys.add("L");
 return[...keys].sort(cp);
}

/** 🧾️ The environment digest of a record: the digests of exactly the components in `keys`. `*` (no recorded reads) reads the whole table. */
export function readsDigestV1(keys,env){return keys.length===1&&keys[0]==="*"?H(["*",env.tableDigest]):H(keys.map(key=>[key,env.digest(key)]));}

/** 📼️ Makes the record of an owner edge derivation: keys + readsDigest under `env`; `inputs` is the digest of the owner's own inputs (facts, targets, implementation components). */
export function makeRecordV1({row,reads,inputs,implementation},env){
 const keys=recordKeysV1(row,reads,env),readsDigest=readsDigestV1(keys,env);
 return{name:row.name,inputs,implementation,keys,readsDigest,id:H([row.name,inputs,implementation,readsDigest])};
}

/** 🔍️ A record is valid under `env` iff the components it read still have the digests it saw. The re-stale rule: nothing else is stale. */
export const validV1=(record,env)=>readsDigestV1(record.keys,env)===record.readsDigest;
export function staleOwnersV1(records,env){return[...records].filter(record=>!validV1(record,env)).map(record=>record.name).sort(cp);}

/** 🌳️ The consistent view a reader assembles: for every owner the newest record (`heads`: name to records newest first) that is valid under the pinned `env`; owners with only invalid records are `stale` (re-derived locally, edges only), owners without records are `missing`. The generation is the merkle root over the sorted leaves, so two readers with the same pinned environment and the same chosen records agree on it without a coordinator. */
export function assembleViewV1(heads,env,names){
 const chosen=[],stale=[],missing=[];
 for(const name of[...names].sort(cp)){const list=heads.get(name)??[];if(!list.length){missing.push(name);continue;}const ok=list.find(record=>validV1(record,env));if(ok)chosen.push(ok);else stale.push(name);}
 const leaves=chosen.map(record=>H([record.name,record.id]));
 return{environment:env.tableDigest,chosen:chosen.map(record=>record.id),stale,missing,generation:H([env.tableDigest,leaves]),coverage:names.length?chosen.length/names.length:1};
}
