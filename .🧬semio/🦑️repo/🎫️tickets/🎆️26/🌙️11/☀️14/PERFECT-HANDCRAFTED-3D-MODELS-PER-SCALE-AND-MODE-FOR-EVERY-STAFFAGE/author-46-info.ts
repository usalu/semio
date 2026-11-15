import { openCtx, flag } from "./kit-lib.ts";
const a = process.argv.slice(2);
const c = openCtx(a[0]!, Number(flag(a, "--scale")));
console.log("dims", JSON.stringify(c.raw.dimensions), "feature", c.feature);
console.log("bones", c.raw.rig.bones.map((b: any) => `${b.id}(${b.parent ?? "-"}:${b.lengthM},${b.mass?.kg ?? ""})`).join(" "));
console.log("regions", (c.raw.thermal?.regions ?? []).map((r: any) => `${r.id}[${r.kind ?? r.model ?? ""}]${r.areaM2}${r.hidden ? "H" : ""}${(r.layers ?? []).length ? "L" + r.layers.length : ""}`).join(" "));
console.log("emitters", JSON.stringify((c.raw.thermal?.emitters ?? []).map((e: any) => [e.id, e.anchor])));
console.log("body", (c.vol.body ?? []).length, "th", (c.vol.thermal ?? []).length, "st", (c.vol.structure ?? []).length);
