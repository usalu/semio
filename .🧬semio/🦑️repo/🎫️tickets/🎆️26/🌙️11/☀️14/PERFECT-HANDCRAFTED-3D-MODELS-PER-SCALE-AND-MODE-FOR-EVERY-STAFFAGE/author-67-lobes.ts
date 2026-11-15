import { Kit, bboxOfShapes, ivalsAll } from "./author-02-lib.ts";
const [spec, S] = [process.argv[2]!, Number(process.argv[3])];
const k = new Kit(spec, S);
const regs: Record<string, any> = Object.fromEntries(k.asset.thermal.regions.map((r: any) => [r.id, r]));
const fol = (s: any) => regs[s.region]?.kind === "foliage";
const f = (b: any) => `x[${b.minX.toFixed(2)},${b.maxX.toFixed(2)}] y[${b.minY.toFixed(2)},${b.maxY.toFixed(2)}]`;
const EB = k.E((s) => fol(s));
for (const fs of k.shapes("front", (s) => fol(s) && s.role === "body")) {
  const id = fs.shape.id as string;
  const key = id.replace(/^f-/, "");
  const Pk = k.P((s) => s.id === `p-${key}`);
  const Fk = k.F((s) => s.id === id);
  const fb = bboxOfShapes(Fk);
  const pb = Pk.length ? bboxOfShapes(Pk) : null;
  let cov = 0, tot = 0;
  if (pb) for (let a = 0; a < 12; a++) for (let b = 0; b < 12; b++) {
    const x = pb.minX + ((pb.maxX - pb.minX) * (a + 0.5)) / 12, y = fb.minY + ((fb.maxY - fb.minY) * (b + 0.5)) / 12;
    tot++; if (ivalsAll(EB, "x", x).some((v) => y >= v[0] && y <= v[1])) cov++;
  }
  console.log(id, fs.shape.bone, "front z" + f({ minX: fb.minX, maxX: fb.maxX, minY: fb.minY, maxY: fb.maxY }), pb ? "plan " + f(pb) : "noplan", "elevcov", (cov / Math.max(1, tot)).toFixed(2));
}
for (const e of k.shapes("elevation", (s) => fol(s) && s.role === "body")) console.log("E", e.shape.id, f(bboxOfShapes(k.E((s) => s.id === e.shape.id))));
