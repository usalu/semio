/** 🔎️ One-feature markers of assets from 1 cm to 10 m at the four scales, kernel triangles against three's SphereGeometry: mark.ts */
import * as THREE from "three";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { ROUNDNESS, qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
for (const L of [0.01, 0.03, 0.1, 0.3, 0.6, 0.9, 1.7, 2.5, 4.8, 10]) {
  const row: string[] = [];
  for (const s of [50, 100, 200, 500]) {
    const asset: any = { dimensions: { heightM: L, widthM: L, depthM: L } };
    const q = qualityOf(asset, s);
    const r = (ROUNDNESS.paperFeatureM * s) / 2;
    const t = tessellateSolid({ id: "a", node: "n", material: "m", kind: "ellipsoid", radii: [r, r, r] } as any, q).indices.length / 3;
    row.push(`${t}`);
  }
  console.log(L, row.join(" "));
}
const n = 24; console.log("three 24x12", new THREE.SphereGeometry(1, n, n / 2).index!.count / 3);
