import { load, save } from "../humans/lib.ts";
import { bandOf } from "../humans/bands.ts";
import { spec200, spec500 } from "./skate-specs.ts";

const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const write = process.argv[2] === "write";

const doc = load("skateboarder");
const a = doc.asset;
const r5 = (n: number) => +n.toFixed(5);

const rep200 = bandOf(a, spec200 as any);
const rep500 = bandOf(a, spec500 as any);

for (const v of ["elevation", "front", "plan"] as const)
  for (const s of rep500.geometry[v]) {
    if (/^shoe/.test(s.id)) s.penMm = 0.2;
    if (v === "front" && /^wheel-front-[lr]$/.test(s.id)) {
      s.w = 0.045;
      if (s.x < 0) s.x = r5(s.x - 0.005);
    }
  }

a.representations["200"] = rep200;
a.representations["500"] = rep500;

const dims = a.dimensions;
const planExtent = () => {
  const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 500 } });
  const p = st.shapes.plan.filter((s: any) => s.shape.role !== "ground").flatMap((s: any) => s.rings.flat());
  const xs = p.map((q: any) => q.x), ys = p.map((q: any) => q.y);
  return { w: Math.max(...xs) - Math.min(...xs), d: Math.max(...ys) - Math.min(...ys) };
};
const error = () => {
  const e = planExtent();
  return Math.max(Math.abs(e.w - dims.widthM), Math.abs(e.d - dims.depthM));
};
const hands = rep500.geometry.plan.filter((s: any) => /^hand-[lr]$/.test(s.id) && s.kind === "capsule");
for (let it = 0; it < 12 && error() > 0.012; it++) {
  let best = { e: error(), apply: () => {}, undo: () => {} };
  for (const h of hands)
    for (const [f, d] of [["x1", 0.01], ["x1", -0.01], ["y0", 0.01], ["y0", -0.01], ["y1", 0.01], ["y1", -0.01]] as const) {
      const old = h[f];
      h[f] = r5(old + d);
      const e = error();
      h[f] = old;
      if (e < best.e - 1e-6) best = { e, apply: () => (h[f] = r5(old + d)), undo: () => (h[f] = old) };
    }
  if (best.e >= error() - 1e-6) break;
  best.apply();
}
console.log("plan extent after compensation", JSON.stringify(planExtent()), "target", dims.widthM, dims.depthM);

const IGNORED: string[] = [];
const issues = validateAsset(a).filter((i: any) => !IGNORED.includes(i.code));
console.log("issues", issues.length);
for (const i of issues.slice(0, 14)) console.log("  ", i.code, i.path, (i.message?.en ?? "").slice(0, 200));
for (const rep of [rep200, rep500]) {
  const s = measureSuitability(a, rep, rep.denominator, DEFAULT_PRINT_PROFILE);
  console.log(rep.denominator, "suitable", s.suitable, "violations", s.violations.length);
  for (const v of s.violations.slice(0, 8)) console.log("  ", JSON.stringify(v).slice(0, 200));
}
if (write) {
  save("skateboarder", doc);
  console.log("WRITTEN");
}
