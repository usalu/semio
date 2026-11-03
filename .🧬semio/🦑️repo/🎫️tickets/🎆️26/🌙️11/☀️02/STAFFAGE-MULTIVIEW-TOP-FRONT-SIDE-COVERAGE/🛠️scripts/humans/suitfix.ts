import { load, save } from "./lib.ts";
import { simplifyShape } from "./simplify.ts";
const g = (d: any, k: string, v: string) => d.asset.representations[k].geometry[v];
{ const d = load("elderly-with-cane"); const a = g(d, "500", "elevation"); const i = a.findIndex((s: any) => s.id === "thumb-r"); if (i >= 0) a.splice(i, 1); save("elderly-with-cane", d); }
{ const d = load("parent-with-stroller"); const s = g(d, "500", "elevation").find((x: any) => x.id === "hips"); simplifyShape(s, 0.012); save("parent-with-stroller", d); }
{ const d = load("runner-with-blade-prosthesis"); for (const v of ["elevation", "front", "plan"]) { const a = g(d, "500", v); const i = a.findIndex((s: any) => s.id === "blade-toe"); if (i >= 0) a.splice(i, 1); } save("runner-with-blade-prosthesis", d); }
{ const d = load("person-with-rollator"); for (const s of g(d, "200", "plan")) if (/^tyre-rear-[lr]$/.test(s.id)) { const sg = s.id.endsWith("l") ? 1 : -1; s.solid.z0 = sg > 0 ? 0.265 : -0.315; s.solid.z1 = sg > 0 ? 0.315 : -0.265; } save("person-with-rollator", d); }
{ const d = load("person-with-white-cane"); simplifyShape(g(d, "200", "elevation").find((x: any) => x.id === "bag"), 0.012); save("person-with-white-cane", d); }
