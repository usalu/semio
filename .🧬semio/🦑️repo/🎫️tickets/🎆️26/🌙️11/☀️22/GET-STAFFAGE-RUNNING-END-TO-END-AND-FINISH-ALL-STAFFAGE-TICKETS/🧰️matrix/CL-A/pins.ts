/** 📌️ The three red pins of lane CL-A measured without the test runner (soft copies of the cases, every expectation counted, none skipped at the first failure): `bun pins.ts goat <file.json>` prints the torso junctions of `🧊️model-skeleton` ("keeps both sections of each doe torso junction coincident through every named pose") and the steps of the horn case of `🧊️model-silhouette` ("retains the measured horn stations at print scales and detects deformed substitutes"); `bun pins.ts red-squirrel <file.json>` the guard locks, their thermal surfaces and the member solid of every tail bone per scale ("keeps the squirrel's tapered caudal chain and articulated brush together"). The real cases run after the write (`suites.sh`).
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-skeleton/🟦️.ts */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { featureRule, thinnestDrawn, sortedExtents } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts"));
const [which, file] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const vectors = JSON.parse(readFileSync(join(S, "🧫️fixtures/🧊️model-skeleton/🔣️.json"), "utf8"));
if (which === "goat") {
  const f = vectors.goatBarrel;
  const worst: Record<string, number> = {};
  let failures = 0, expectations = 0;
  for (const pose of asset.poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id });
    for (const [scale, vol] of Object.entries(asset.volumes) as [string, any][]) {
      const ends = f.parts.map((id: string) => {
        const segment = vol.body.find((q: { id: string }) => q.id === id), fr = state.bones[segment.bone].frame;
        const world = (p: number[]) => (["x", "y", "z"] as const).map((k) => fr.o[k] + fr.u[k] * p[0]! + fr.v[k] * p[1]! + fr.w[k] * p[2]!);
        const [start, end] = [segment.sections[0], segment.sections.at(-1)];
        return { start: world(start.at), end: world(end.at), first: [start.rx, start.ry], last: [end.rx, end.ry] };
      });
      for (let i = 1; i < ends.length; i++) {
        const d = Math.hypot(...ends[i - 1].end.map((v: number, k: number) => v - ends[i].start[k]));
        const key = `1:${scale} ${f.parts[i - 1]} -> ${f.parts[i]}`;
        worst[key] = Math.max(worst[key] ?? 0, d);
        expectations += 2;
        failures += (d < f.jointToleranceM ? 0 : 1) + (JSON.stringify(ends[i - 1].last) === JSON.stringify(ends[i].first) ? 0 : 1);
      }
    }
  }
  for (const [key, d] of Object.entries(worst)) console.log(`junction ${key}: worst gap ${(d * 1000).toFixed(6)} mm over ${asset.poses.length} poses (limit ${f.jointToleranceM * 1000} mm)`);
  console.log(`junction: ${failures} failures of ${expectations} expectations`);
  const fine = asset.volumes[50].body.find((s: { id: string }) => s.id === "horn-n");
  const coarse = asset.volumes[500].body.find((s: { id: string }) => s.id === "horn-n");
  const index = asset.volumes[500].body.indexOf(coarse);
  const findings = () => featureRule(asset, [500]).findings.filter((x: { path: string }) => x.path === `volumes.500.body[${index}]`);
  const d = (a: number[], b: number[]) => Math.hypot(a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!);
  console.log(`horn: limit of bone head at 1:500 (thinnest drawing) ${thinnestDrawn(asset, 500).get("head")} m, extents of the solid ${sortedExtents(coarse).map((v: number) => v.toFixed(5))}, radii ${coarse.sections.map((s: { rx: number; ry: number }) => `${s.rx}/${s.ry}`).join(" ")}`);
  console.log(`horn: stations that are 1:50 stations: ${coarse.sections.filter((s: unknown) => fine.sections.some((t: unknown) => JSON.stringify(s) === JSON.stringify(t))).length} of ${coarse.sections.length}`);
  const steps: [string, unknown, unknown][] = [["first station at the 1:50 place", d(coarse.sections[0].at, fine.sections[0].at), 0], ["last station at the 1:50 place", d(coarse.sections.at(-1).at, fine.sections.at(-1).at), 0], ["fewer stations than 1:50", coarse.sections.length < fine.sections.length, true]];
  coarse.id = "near-horn";
  steps.push(["findings as authored", findings().length, 0]);
  const station = coarse.sections[1], radius = station.rx;
  station.rx *= 0.5;
  steps.push(["findings with station 1 thinned to half", findings().length, 1]);
  station.rx = radius;
  coarse.transform = { translation: [0.01, 0, 0] };
  steps.push(["findings with a translation of 10 mm", findings().length, 1]);
  delete coarse.transform;
  coarse.material = "short-coat";
  steps.push(["findings with the material short-coat", findings().length, 1]);
  for (const [name, got, want] of steps) console.log(`horn step: ${name}: ${got} (case expects ${want}) ${got === want ? "ok" : "RED"}`);
} else {
  const f = vectors.squirrelBrush;
  const bones = asset.rig.bones.filter((b: { id: string }) => /^tail-\d+$/.test(b.id));
  let failures = 0;
  ["50", "100", "200", "500"].forEach((scale, i) => {
    const vol = asset.volumes[scale];
    const locks = vol.body.filter((s: { id: string }) => s.id.startsWith("brush-lock-"));
    const bare = locks.filter((lock: { id: string }) => !vol.thermal.some((s: { of?: string; region: string }) => s.of === lock.id && s.region === "tail-fur")).length;
    const carried = bones.filter((b: { id: string }) => vol.structure.some((s: { part?: string; member?: string; members?: string[] }) => s.part === "member" && (s.member === b.id || (s.members ?? []).includes(b.id)))).length;
    const centra = bones.map((b: { id: string }) => vol.structure.filter((s: { member?: string; id: string }) => s.member === b.id && s.id.startsWith("caudal-centrum-")).length);
    failures += (locks.length === f.guardLocks[i] ? 0 : 1) + bare + (bones.length - carried);
    console.log(`1:${scale}: ${locks.length} guard locks (pin ${f.guardLocks[i]}), ${bare} without a tail-fur surface, ${carried} of ${bones.length} tail bones with a member solid, centra per bone ${centra} (pin ${f.centra} where the feature admits them)`);
  });
  console.log(`brush: ${failures} failures`);
}
