import * as THREE from "three";
import { tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
for (const n of [12, 13, 14, 15, 16, 17, 24, 25, 31, 32]) {
  const r = 1;
  const chord = r * (1 - Math.cos(Math.PI / n)) * 1.0000001;
  const t = tessellateSolid({ id: "a", node: "n", material: "m", kind: "ellipsoid", radii: [r, r, r] } as any, { chordM: chord, minSegments: 12, maxSegments: 96 } as any).indices.length / 3;
  console.log(n, t, [Math.floor(n / 2), Math.ceil(n / 2)].map((h) => new THREE.SphereGeometry(1, n, h).index!.count / 3).join("/"));
}
