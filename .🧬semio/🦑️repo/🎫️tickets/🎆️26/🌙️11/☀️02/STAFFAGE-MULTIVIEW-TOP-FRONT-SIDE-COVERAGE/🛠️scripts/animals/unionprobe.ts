import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [, , id, scale, pose] = process.argv;
const base: any = catalog().find((x) => x.id === id)!;
const sel = { kind: "scale", scale: { denominator: +scale } } as any;
const q: any = pose.includes("@") ? { kind: "clip", clip: pose.split("@")[0], timeS: +pose.split("@")[1] } : { kind: "pose", pose };
const rep = base.representations[scale].geometry;
const all: [string, number][] = [];
for (const v of ["elevation", "front", "plan"]) rep[v].forEach((s: any, i: number) => { if (s.solid) all.push([v, i]); });
const ok = (a: any) => { try { evaluate(a, q, sel); return true; } catch { return false; } };
console.log("baseline", ok(base));
for (const [v, i] of all) { const a = structuredClone(base); delete a.representations[scale].geometry[v][i].solid; a.representations[scale].geometry[v][i].kind; if (ok(a)) console.log("fixed by dropping solid of", v, rep[v][i].id, rep[v][i].bone, JSON.stringify(rep[v][i].solid)); }
