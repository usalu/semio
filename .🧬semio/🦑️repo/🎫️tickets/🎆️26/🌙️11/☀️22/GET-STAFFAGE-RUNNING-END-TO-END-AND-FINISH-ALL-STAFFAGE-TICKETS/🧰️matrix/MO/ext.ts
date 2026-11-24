/** 📏️ The shapes that reach the ends of the drawn extent of each view of an asset at a scale in a pose (within `--within` metres, default 0.01), to see which part makes a view longer or shorter than another; ops applied in memory unless `--no-ops`.
 *  bun ext.ts <id> <scale> <pose> [--within 0.01] [--no-ops] */
import { applyAll } from "./apply.ts";
import { S, catalogFile, opsFile } from "./files.ts";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const args = process.argv.slice(2);
const asset = (await Bun.file(catalogFile(args[0]!)).json()).asset;
if (!args.includes("--no-ops") && (await Bun.file(opsFile(args[0]!)).exists())) applyAll(asset, JSON.parse(await Bun.file(opsFile(args[0]!)).text()));
const within = args.includes("--within") ? Number(args[args.indexOf("--within") + 1]) : 0.01;
const state: any = evaluate(asset, { kind: "pose", pose: args[2]! }, { kind: "scale", scale: { denominator: Number(args[1]) } });
const f = (n: number): string => n.toFixed(3);
for (const view of ["elevation", "front", "plan"]) {
  const boxes = state.shapes[view].flatMap((s: any) => {
    const pts = s.rings.flat();
    if (pts.length === 0) return [];
    const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
    return [{ id: `${s.shape.id}[${s.shape.bone}]${s.shape.solid ? "*" : ""}`, b: [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)] }];
  });
  const ends = [0, 1, 2, 3].map((k) => (k % 2 === 0 ? Math.min : Math.max)(...boxes.map((x: any) => x.b[k])));
  console.log(`${view}: x ${f(ends[0]!)} ${f(ends[1]!)} y ${f(ends[2]!)} ${f(ends[3]!)}`);
  ["x-", "x+", "y-", "y+"].forEach((name, k) => console.log(`   ${name} ${boxes.filter((x: any) => Math.abs(x.b[k] - ends[k]!) <= within).map((x: any) => `${x.id} ${f(x.b[k])}`).slice(0, 6).join(", ")}`));
}
