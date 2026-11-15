import {existsSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
const R="/Users/Shared/semio/",G=JSON.parse(readFileSync(process.argv[2],"utf8")) as {graph:Record<string,string[]>};
const crates=Object.keys(G.graph),memo=new Map<string,string>();
const projectOf=(dir:string):string=>{const hit=memo.get(dir);if(hit!==undefined)return hit;let x=dir,r="";while(true){if(existsSync(join(R,x,"📋️project.json"))){r=x;break;}const up=dirname(x);if(up===x||up===".")break;x=up;}memo.set(dir,r);return r;};
const owner=new Map(crates.map(c=>[c,projectOf(c)]));
const closure=new Map<string,Set<string>>();
const clo=(d:string):Set<string>=>{let s=closure.get(d);if(s)return s;s=new Set();closure.set(d,s);for(const e of G.graph[d]??[]){s.add(e);for(const f of clo(e))s.add(f);}return s;};
for(const c of crates)clo(c);
const crateLevel=new Map<string,number>(),projectLevel=new Map<string,number>(),perOwner=new Map<string,number>();
for(const d of crates){
 for(const x of closure.get(d)!)crateLevel.set(x,(crateLevel.get(x)??0)+1);
 const owners=new Set([...closure.get(d)!].map(x=>owner.get(x)!));owners.delete(owner.get(d)!);
 for(const o of owners)perOwner.set(o,(perOwner.get(o)??0)+1);
}
let sumCrate=0,sumProject=0,withDependents=0;const reductions:number[]=[];
for(const c of crates){const a=crateLevel.get(c)??0,b=perOwner.get(owner.get(c)!)??0;sumCrate+=a;sumProject+=Math.max(a,b);if(a>0){withDependents++;reductions.push(b===0?0:1-a/Math.max(a,b));}}
reductions.sort((a,b)=>a-b);
console.log(JSON.stringify({crates:crates.length,withDependents,meanCrateLevelDependents:+(sumCrate/crates.length).toFixed(1),meanProjectLevelStaledCrates:+(sumProject/crates.length).toFixed(1),medianReduction:+reductions[reductions.length>>1]!.toFixed(3),sharedProjectCrates:crates.filter(c=>crates.filter(d=>owner.get(d)===owner.get(c)).length>1).length}));
