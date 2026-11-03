import { demonstratorActivationLanes, demonstratorActivationComponents } from "/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/♻️activation/🟦️.ts";
import { demonstratorRuntimeComponentIds } from "/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
import { PLAYGROUND_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/playgrounds";
const comps=[...PLUGIN_BUILD_TARGETS,...EXTENSION_TARGETS];
const lanes=demonstratorActivationLanes(); console.log("lanes",lanes);
const exp=new Set(demonstratorRuntimeComponentIds()); console.log("expected",exp.size);
const un=new Set<string>();
for(const l of lanes){const p=PLAYGROUND_BUILD_TARGETS.find(r=>r.variant===l)!; const c=runtimeComponentClosure(comps,[p.pluginId]); console.log(l,p.pluginId,c.length); c.forEach(x=>un.add(x));}
console.log("missing",[...exp].filter(x=>!un.has(x)),"extra",[...un].filter(x=>!exp.has(x)));
console.log("components", demonstratorActivationComponents("/Users/ueli/Documents/semio").length);
