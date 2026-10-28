/** 🧭️ Contract of the live-start predicate of the front-door script: it gives exactly the verdict of the mode rule of the sealed store (`implementationModeV1`), a value that rule refuses is never a live start, and the module reads the policy table only, so a live start loads nothing of the store. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{implementationModeV1}from"../../🟦️.ts";
import policy from"../../🔣️.json";
import{liveImplementationStartV1}from"../🟦️.ts";

const values:(string|undefined)[]=[undefined,"","live","LIVE","Live","sealed","SEALED","promote","fast"," live","live ","l"];

test("a live start is exactly the verdict of the mode rule of the store",()=>{
 for(const value of values){
  const environment={[policy.modeVariable]:value};
  let verdict:boolean;try{verdict=implementationModeV1(environment)==="live";}catch{verdict=false;}
  expect(liveImplementationStartV1(environment),String(value)).toBe(verdict);
 }
 expect(liveImplementationStartV1({})).toBe(implementationModeV1({})==="live");
});

test("the predicate imports the policy table and nothing else",()=>{
 const text=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8");
 expect([...text.matchAll(/from\s*"([^"]+)"/gu)].map(match=>match[1])).toEqual(["../🔣️.json"]);
});
