/** 🔥️ The heater cases of `🧪️tests/🧊️model-bounds` ("keeps the carry grip open and resolves the heater outlet at 1:%i") measured on any document of `electric-fan-heater`, with the numbers the test only asserts: `bun heater.ts <file.json>…` prints per file, scale and view the hand clearance and the bar above the hand at the three lateral positions, the end of the control dial and the number of louvre bands across the outlet (the test wants clear, hit, hit and 8), then how many of the four cases of the test the file passes.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-bounds/🟦️.ts — the test whose rays these are */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const { Ray, Triangle, Vector3 } = await import(Bun.resolveSync("three", join(S, "🧪️tests", "🧊️model-bounds")));
const { tessellateSolid } = await import(join(S, "..", "..", "..", "🧰️framework", "🔨️modules", "🧊️3d", "🗿️model", "🟦️.ts"));
const { modelOf } = await import(join(S, "🔨️modules", "🧊️volume", "🟦️.ts"));

const triangles = (solids: any[]): any[] => {
  const result: any[] = [];
  for (const solid of solids) {
    const mesh = tessellateSolid(solid, { chordM: 0.0001, minSegments: 24, maxSegments: 48 });
    for (let i = 0; i < mesh.indices.length; i += 3) {
      const points = [0, 1, 2].map((j) => new Vector3().fromArray(mesh.positions, 3 * mesh.indices[i + j]!));
      result.push(new Triangle(points[0], points[1], points[2]));
    }
  }
  return result;
};
const hit = (ray: any, faces: any[], minimumX = -Infinity): boolean => {
  const point = new Vector3();
  return faces.some((t) => ray.intersectTriangle(t.a, t.b, t.c, false, point) && point.x >= minimumX);
};
const bands = (samples: boolean[]): number => samples.filter((inside, index) => inside && !samples[index - 1]).length;
const across = (y: number, z: number): any => new Ray(new Vector3(0.2, y, z), new Vector3(-1, 0, 0));

for (const file of process.argv.slice(2)) {
  const heater = JSON.parse(readFileSync(file, "utf8")).asset;
  let passed = 0;
  for (const scale of [50, 100, 200, 500]) {
    let ok = true;
    for (const view of ["light-solid", "thermal", "forces"] as const) {
      const model = await modelOf(heater, { scale, view, poses: [], clips: [] });
      const face = triangles(model.solids.filter((s: any) => s.node === "body" && !s.tags?.includes("part-mass")));
      const clear = [-0.04, 0, 0.04].map((z) => !hit(across(0.286, z), face));
      const bar = [-0.04, 0, 0.04].map((z) => hit(across(0.304, z), face));
      let rest = "";
      let good = clear.every(Boolean) && bar.every(Boolean);
      if (view !== "forces") {
        const dial = hit(new Ray(new Vector3(0, 0.017, -0.15), new Vector3(0, 0, 1)), triangles(model.solids.filter((s: any) => s.node === "knob")));
        const count = bands(Array.from({ length: 155 }, (_, i) => hit(across(0.0673 + i * 0.001, 0.076), face, 0.08)));
        rest = ` dial ${dial} louvre bands ${count}`;
        good &&= dial && count === 8;
      }
      ok &&= good;
      console.log(`${file.split("/").at(-1)} 1:${scale} ${view}: hand clear ${clear.join(",")} bar ${bar.join(",")}${rest} ${good ? "ok" : "FAIL"}`);
    }
    if (ok) passed++;
  }
  console.log(`${file.split("/").at(-1)}: ${passed} of 4 cases pass`);
}
