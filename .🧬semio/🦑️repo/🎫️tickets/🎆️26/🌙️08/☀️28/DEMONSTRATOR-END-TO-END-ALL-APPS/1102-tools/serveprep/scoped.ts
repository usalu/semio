import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { PLAYGROUND_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/playgrounds";
import { demonstratorRuntimeComponentIds } from "/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
const comps=[...PLUGIN_BUILD_TARGETS,...EXTENSION_TARGETS];
const exp=new Set(demonstratorRuntimeComponentIds()); const un=new Set<string>();
console.log("hosts",comps.filter((c:any)=>c.host).map((c:any)=>c.pluginId));
for(const v of ["generator","energy","fem3d"]){const p:any=PLAYGROUND_BUILD_TARGETS.find(r=>r.variant===v)!;
 const scoped=runtimeComponentClosure(comps as any,[{id:p.pluginId,appScoped:p.app!==undefined}] as any); const plain=runtimeComponentClosure(comps,[p.pluginId]);
 console.log(v,"app",p.app,"scoped",scoped.length,"plain",plain.length,"sameSet",JSON.stringify(scoped)===JSON.stringify(plain)); scoped.forEach(x=>un.add(x));}
console.log("missing",[...exp].filter(x=>!un.has(x)),"extra",[...un].filter(x=>!exp.has(x)));
