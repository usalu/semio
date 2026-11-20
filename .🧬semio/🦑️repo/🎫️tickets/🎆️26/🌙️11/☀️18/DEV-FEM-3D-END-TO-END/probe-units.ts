import {unitProfilesV1,measureUnitsV1,planUnitsV1} from "C:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🟦️.ts";
const store="C:/semio/.cache/b/r/m";
const p=unitProfilesV1(store);console.log(p.map(x=>x.profileDirectory));
const u=await measureUnitsV1(p,{nowMs:Date.now(),busy:()=>false});
const by:Record<string,[number,number,number]>={};
for(const x of u){const k=x.path.split("build")[0];const r=by[k]??=[0,0,0];r[0]++;r[1]+=x.bytes/1e9;if(x.held)r[2]++;}
console.log(by);
const plan=planUnitsV1({needBytes:1e12,nowMs:Date.now(),units:u});console.log(plan.length,plan.reduce((s,r)=>s+r.bytes,0)/1e9);
