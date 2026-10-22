import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {policyServiceInvocationBreaches} from "../🟦️.ts";

const workspace=(()=>{for(let at=import.meta.dir;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"nx.json")))return at;if(dirname(at)===at)throw Error("Workspace root not found");}})();
const BUILD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const temporary:string[]=[];
afterAll(()=>{for(const path of temporary)rmSync(path,{recursive:true,force:true});});
function tree(files:Record<string,string>){
 const root=mkdtempSync(join(tmpdir(),"svc-law-"));temporary.push(root);
 for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,BUILD,path)),{recursive:true});writeFileSync(join(root,BUILD,path),text);}
 return root;
}
const consumer=(calls:number,extra="")=>"const a=declaredCapabilitiesV1(x);".repeat(calls)+"const role=declaredCapabilitiesV1(y);const capabilities=declaredCapabilitiesV1(z);"+extra;
const sound={"📥️caller/🟦️.ts":"const capabilities=declaredCapabilitiesV1(a),role=declaredCapabilitiesV1(b),c=declaredCapabilitiesV1(c),d=declaredCapabilitiesV1(d);capabilities.launches;role.launches;declared.launches;","🧭️invocation/🟦️.ts":"const caller=(v?v:declaredCapabilitiesV1(v));caller.launches;","📥️caller/🛰️service/🟦️.ts":"export function declaredCapabilitiesV1(){}\nholdServiceInvocationV1 activeServiceTicketV1 createServiceAdmissionV1 createServiceGrantV1","📜️script.ts":"holdServiceInvocationV1(","📥️caller/📤️child/🟦️.ts":"createServiceGrantV1("};

test("the repository has exactly one service invocation kind: no other reader of launches, no other holder, no resident import",()=>{
 expect(policyServiceInvocationBreaches(workspace).map(breach=>breach.summary)).toEqual([]);
});

test("a sound synthetic tree raises nothing and each way around the declaration is named",()=>{
 expect(policyServiceInvocationBreaches(tree(sound))).toEqual([]);
 const rule=(files:Record<string,string>)=>policyServiceInvocationBreaches(tree({...sound,...files})).map(breach=>breach.id.split("-")[2]+" "+breach.scope.split("/").slice(-2).join("/"));
 expect(rule({"📥️caller/🟦️.ts":sound["📥️caller/🟦️.ts"]+"const other=cap.launches;"})).toEqual(["reader 📥️caller/🟦️.ts"]);
 expect(rule({"📥️caller/🟦️.ts":"const capabilities=declaredCapabilitiesV1(a);capabilities.launches;"})).toEqual(["calls 📥️caller/🟦️.ts"]);
 expect(rule({"🧭️invocation/🟦️.ts":"const caller=value;caller.launches;"})).toEqual(["calls 🧭️invocation/🟦️.ts","reader 🧭️invocation/🟦️.ts"]);
 expect(rule({"🏃️execution/🟦️.ts":"import {declaredCapabilitiesV1} from 'x';"})).toEqual(["holder 🏃️execution/🟦️.ts"]);
 expect(rule({"🔧️tools/🟦️.ts":"createServiceAdmissionV1({})"})).toEqual(["holder 🔧️tools/🟦️.ts"]);
 expect(rule({"🏡️resident/🚪️client/🟨️.mjs":"import '../../📥️caller/🛰️service/🟦️.ts'"})).toEqual(["resident 🚪️client/🟨️.mjs"]);
 expect(rule({"🏡️resident/🟨️.mjs":"import '../📥️caller/🛰️service/🔑️session/🟨️.mjs'"})).toEqual([]);
 expect(policyServiceInvocationBreaches(tree({"📜️script.ts":""})).map(breach=>breach.id.split("-")[2])).toEqual(["missing","missing"]);
});
