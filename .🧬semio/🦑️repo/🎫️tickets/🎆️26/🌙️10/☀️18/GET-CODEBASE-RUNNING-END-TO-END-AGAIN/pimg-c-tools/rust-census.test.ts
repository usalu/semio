import {test} from "bun:test";
import {unitProfilesV1,measureUnitsV1,newestUnitsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🧱️units/🟦️.ts";
const store="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m",G=1024**3,H=3600000;
test("census",async()=>{
 const now=Date.now(),units=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>true});
 const newest=newestUnitsV1(units),sum=(rows:typeof units)=>rows.reduce((a,u)=>a+u.bytes,0)/G;
 const by=(f:(u:typeof units[number])=>string)=>{const m=new Map<string,number>();for(const u of units)m.set(f(u),(m.get(f(u))??0)+u.bytes/G);return [...m].map(([k,v])=>k+"="+v.toFixed(1)).join(" ");};
 const prof=(u:typeof units[number])=>u.path.slice(store.length+1).split("/build/")[0]!;
 console.log("[DEBUG] units",units.length,"total GiB",sum(units).toFixed(1),"newest2",sum(units.filter(u=>newest.has(u.path))).toFixed(1),"held",sum(units.filter(u=>u.held)).toFixed(1),"<3h",sum(units.filter(u=>now-u.lastUsedMs<3*H)).toFixed(1),"<24h",sum(units.filter(u=>now-u.lastUsedMs<24*H)).toFixed(1),">24h",sum(units.filter(u=>now-u.lastUsedMs>=24*H)).toFixed(1));
 console.log("[DEBUG] by profile",by(prof));
 console.log("[DEBUG] newest2 or <3h or held GiB",sum(units.filter(u=>newest.has(u.path)||now-u.lastUsedMs<3*H||u.held)).toFixed(1));
},600000);
