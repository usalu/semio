import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset, type Json } from "./volume-lib.ts";
const [slug, beforeFile, scale = "50", view = "elevation", pose] = process.argv.slice(2);
const before = JSON.parse(readFileSync(beforeFile!, "utf8")).asset as Json;
const after = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
const q = { kind: "pose", pose: pose ?? after.presentation.defaultPose };
const get = (a: Json) => (evaluate(a, q as never, { kind: "scale", scale: { denominator: Number(scale) } } as never) as Json).shapes[view] as Json[];
const B = get(before), A = get(after);
const ext = (s: Json) => { const p = s.rings.flat(); return p.length ? [Math.min(...p.map((q: Json) => q.x)), Math.max(...p.map((q: Json) => q.x)), Math.min(...p.map((q: Json) => q.y)), Math.max(...p.map((q: Json) => q.y))].map((x) => +x.toFixed(3)) : []; };
const tot = (L: Json[]) => { const p = L.flatMap((s) => s.rings.flat()); return [Math.min(...p.map((q: Json) => q.x)), Math.max(...p.map((q: Json) => q.x)), Math.min(...p.map((q: Json) => q.y)), Math.max(...p.map((q: Json) => q.y))].map((x) => +x.toFixed(3)); };
console.log("total before", JSON.stringify(tot(B)), "after", JSON.stringify(tot(A)));
for (let i = 0; i < A.length; i++) { const b = ext(B[i]), a = ext(A[i]); if (JSON.stringify(a) !== JSON.stringify(b)) console.log(A[i].shape.id, A[i].shape.bone, "before", JSON.stringify(b), "after", JSON.stringify(a)); }
