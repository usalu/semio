import {diskZeroTouchTreeV1,scanModuleSpecifiersV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🟦️.ts";
import {posix} from "node:path";
const R="/Users/Shared/semio";const tree=diskZeroTouchTreeV1(R);
const EXT=["",".ts",".tsx",".mts",".js",".mjs"],CODE=/\.(?:[cm]?[jt]s|tsx)$/;
const rt=(from:string,s:string)=>{const b=posix.normalize(posix.join(posix.dirname(from),s.split("?")[0]));for(const e of EXT){const c=b+e;if(CODE.test(c)&&tree.read(c)!==undefined)return c}};
const kids=(f:string)=>[...new Set(scanModuleSpecifiersV1(f,tree.read(f)!).filter(x=>x.kind==="static"&&x.specifier.startsWith(".")).map(x=>rt(f,x.specifier)).filter(Boolean) as string[])];
const clo=(roots:string[],skip=new Set<string>())=>{const s=new Set<string>(),q=[...roots];while(q.length){const f=q.pop()!;if(s.has(f)||skip.has(f))continue;s.add(f);for(const k of kids(f))q.push(k)}return s};
const B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const entry=B+"🚢️fleet-status/⌨️command/🟦️.ts";
const all=clo([entry]);console.log("static closure",all.size);
const readers=B+"🚢️fleet-status/📥️readers/🟦️.ts";
const rk=kids(readers);
for(const k of rk){const others=clo([entry],new Set([k]));const ex=[...all].filter(f=>!others.has(f));console.log(ex.length,k.replace(B,"B/"));if(process.argv[2]===k.replace(B,"B/"))ex.forEach(x=>console.log("   ",x.replace(B,"B/")))}
