/** 📌️ The detail pins of lane CL-O measured softly on a catalog asset or a trial copy, outside the test runner: `bun pins.ts <id|file.json>…` repeats the expectations of the cases "laces both bicycle wheels through open nipple seats", "bores every scissor arm for its cross pins" and "passes each idler pin through its yoke" of `🧪️tests/🧊️model-skeleton` with the vectors of its fixture and prints every failing expectation instead of stopping at the first, then `PINS <id>: OK|<n> failing`. The case itself stays the judge; this is the measuring copy.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-skeleton/🟦️.ts — the pinned cases */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Ray, Vector3 } from "three";
import { S, readDoc } from "../MR/lib.ts";

const { VOLUME_LIMITS, featureOf, isSheet, sortedExtents } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const vectors = JSON.parse(readFileSync(join(S, "🧫️fixtures/🧊️model-skeleton/🔣️.json"), "utf8"));
const f0 = vectors.vectors ?? vectors;

const admits = (part: any, scale: string, member = false): boolean => {
  if (scale === "50" || part === undefined || !("kind" in part)) return true;
  const feature = featureOf(Number(scale)), [thin, wide, long] = sortedExtents(part);
  if (thin >= feature - 1e-9 || part.texture === true || VOLUME_LIMITS.thinRoles.includes(part.role) || isSheet(part, feature)) return true;
  const bar = member && long >= VOLUME_LIMITS.slenderMemberFeatures * feature - 1e-9;
  return long >= (bar ? VOLUME_LIMITS.slenderMemberRatio : VOLUME_LIMITS.slenderRatio) * wide - 1e-9;
};
const close = (a: number, b: number, digits: number): boolean => Math.abs(a - b) < 10 ** -digits / 2;

for (const arg of process.argv.slice(2)) {
  const asset = readDoc(arg).doc.asset, fails: string[] = [], high = asset.volumes["50"];
  const fail = (text: string) => void fails.push(text);
  const wheels = f0.cycleWheels[asset.id];
  if (wheels) for (const [scale, volume] of Object.entries<any>(asset.volumes)) for (const end of ["f", "r"]) {
    const wheel = volume.body.filter((s: any) => s.bone === "wheel-" + end), spokes = wheel.filter((s: any) => s.id.startsWith("spoke-"));
    const count = spokes.reduce((n: number, s: any) => n + (s.repeat?.count ?? 1), 0);
    if (count !== wheels.spokes) fail(`1:${scale} wheel-${end}: ${count} spokes, pinned ${wheels.spokes}`);
    for (const spoke of spokes) {
      const nipple = wheel.find((s: any) => s.id === spoke.id.replace("spoke-", "nipple-"));
      if (nipple?.kind !== "polyhedron") { fail(`1:${scale} ${spoke.id}: Missing nipple`); continue; }
      const vertices = nipple.vertices.map((p: number[]) => new Vector3(...p));
      const center = (points: Vector3[]) => points.reduce((sum, p) => sum.add(p), new Vector3()).divideScalar(points.length);
      const start = center(vertices.slice(0, 8)), stop = center(vertices.slice(-8)), axis = stop.clone().sub(start).normalize();
      const ray = new Ray(start.clone().addScaledVector(axis, -0.01), axis);
      if (!close(start.distanceTo(stop), wheels.nippleM, 5)) fail(`1:${scale} ${nipple.id}: length ${start.distanceTo(stop).toFixed(6)}, pinned ${wheels.nippleM}`);
      if (nipple.triangles.some(([i, j, k]: number[]) => ray.intersectTriangle(vertices[i], vertices[j], vertices[k], false, new Vector3()))) fail(`1:${scale} ${nipple.id}: the bore is closed`);
    }
  }
  const b = f0.scissorBearings;
  if (b.asset === asset.id) for (const [scale, volume] of Object.entries<any>(asset.volumes)) for (const side of ["a", "b"]) for (const index of [1, 2, 3, 4]) {
    if (!volume.body.some((s: any) => s.bone === side + index)) fail(`1:${scale} ${side}${index}: no body solid`);
    if (!admits(high.body.find((s: any) => s.id === `arm-${side}${index}`), scale)) continue;
    const arm = volume.body.find((s: any) => s.id === `arm-${side}${index}`);
    if (arm?.kind !== "polyhedron") { fail(`1:${scale} arm-${side}${index}: Missing bored link (${arm?.kind})`); continue; }
    const vertices = arm.vertices.map((p: number[]) => new Vector3(...p));
    for (const [station, x] of b.stationsM.entries()) {
      const hits = (y: number) => arm.triangles.some(([i, j, k]: number[]) => new Ray(new Vector3(x, y, -1), new Vector3(0, 0, 1)).intersectTriangle(vertices[i], vertices[j], vertices[k], false, new Vector3()) !== null);
      if (hits(b.boreRadiusM)) fail(`1:${scale} arm-${side}${index} station ${station}: the arm is solid at the bore radius ${b.boreRadiusM}`);
      if (!hits(b.sleeveRadiusM + 0.0025)) fail(`1:${scale} arm-${side}${index} station ${station}: no arm outside the sleeve`);
      if (!admits(high.body.find((s: any) => s.id === `sleeve-${side}${index}-${station}`), scale)) continue;
      const sleeve = volume.body.find((s: any) => s.id === `sleeve-${side}${index}-${station}`);
      if (sleeve?.kind !== "revolve") { fail(`1:${scale} sleeve-${side}${index}-${station}: Missing sleeve`); continue; }
      const bore = Math.min(...sleeve.profile.map((p: number[]) => p[1]));
      if (!close(bore, b.boreRadiusM, 6)) fail(`1:${scale} sleeve-${side}${index}-${station}: bore ${bore}, pinned ${b.boreRadiusM}`);
    }
  }
  const c = f0.crawler;
  if (c.asset === asset.id) {
    const kept = (scale: string, prefix: string) => high.body.filter((s: any) => s.id.startsWith(prefix)).every((s: any) => admits(s, scale));
    for (const [scale, volume] of Object.entries<any>(asset.volumes)) {
      const cheeks = volume.body.filter((s: any) => s.id.startsWith("idler-yoke-"));
      if (cheeks.length !== 2) fail(`1:${scale}: ${cheeks.length} idler yokes, pinned 2`);
      if (kept(scale, "idler-cross-pin")) {
        const pin = volume.body.find((s: any) => s.id === "idler-cross-pin");
        if (pin?.kind !== "revolve") fail(`1:${scale}: Missing idler cross pin`);
        else if (kept(scale, "idler-yoke-")) for (const cheek of cheeks) {
          if (cheek.kind !== "extrude" || !cheek.holes?.[0]) { fail(`1:${scale} ${cheek.id}: Missing yoke bore`); continue; }
          const gaps = cheek.holes[0].map(([x, y]: number[]) => Math.hypot(x - pin.origin[0], y - pin.origin[1]) - pin.profile[0][1]);
          const bad = gaps.filter((g: number) => !close(g, c.idlerClearanceM, 4));
          if (bad.length) fail(`1:${scale} ${cheek.id}: ${bad.length} of ${gaps.length} bore points with clearance ${Math.min(...bad).toFixed(5)} to ${Math.max(...bad).toFixed(5)}, pinned ${c.idlerClearanceM}`);
        }
      }
      for (const side of ["l", "r"]) {
        if (!kept(scale, "roller-" + side)) continue;
        const roller = volume.body.find((s: any) => s.id === "roller-" + side);
        if (roller?.repeat?.count !== c.rollersPerSide) fail(`1:${scale} roller-${side}: ${roller?.repeat?.count} rollers, pinned ${c.rollersPerSide}`);
        if (!volume.structure.some((s: any) => "of" in s && s.of === roller?.id)) fail(`1:${scale} roller-${side}: no structure reference`);
      }
    }
  }
  for (const line of fails.slice(0, Number(process.env.SHOW ?? 12))) console.log(`   ${line}`);
  console.log(`PINS ${asset.id}: ${fails.length === 0 ? "OK" : `${fails.length} failing`}`);
}
