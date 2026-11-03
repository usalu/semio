import { PLAYGROUND_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/playgrounds";
import { runtimeComponentClosure } from "@semio-tech/repo-lib/inventory/tooling";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "@semio-tech/s-dev-catalog-ts/generated/plugins";
const comps=[...PLUGIN_BUILD_TARGETS,...EXTENSION_TARGETS];
for(const v of ["generator","energy","fem3d"]){const t:any=PLAYGROUND_BUILD_TARGETS.find(r=>r.variant===v)!; console.log(v,Object.keys(t), JSON.stringify(t).slice(0,600));}
