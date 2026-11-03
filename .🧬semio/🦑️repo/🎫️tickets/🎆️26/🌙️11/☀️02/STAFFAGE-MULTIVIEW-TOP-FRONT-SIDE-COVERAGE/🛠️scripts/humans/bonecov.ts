import { load } from "./lib.ts";
import { readdirSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const limb = /^(upper-arm|arm|forearm|hand|thigh|shank|foot|head)(-[lr])?$/;
for (const slug of readdirSync(`${L}/🗂️catalog/👤️humans`).sort()) {
  const a = load(slug).asset; const out: string[] = [];
  for (const r of Object.values<any>(a.representations)) {
    if (r.status !== "authored") continue;
    const set = (v: string) => new Set(r.geometry[v].filter((s: any) => s.role !== "ground").map((s: any) => s.bone));
    const all = new Set([...set("elevation"), ...set("front"), ...set("plan")]);
    for (const v of ["elevation", "front", "plan"]) { const miss = [...all].filter((b: any) => limb.test(b) && !set(v).has(b)); if (miss.length) out.push(`${r.denominator}/${v}:${miss.join(",")}`); }
  }
  console.log(slug.padEnd(30), out.join("  "));
}
