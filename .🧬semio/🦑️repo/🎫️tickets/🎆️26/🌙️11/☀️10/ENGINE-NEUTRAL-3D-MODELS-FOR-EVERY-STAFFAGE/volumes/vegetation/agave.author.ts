import { Author, type V3, type Section } from "../../veg-lib.ts";
const a = new Author("agave");
a.loftFromViews("caudex", "caudex", { elev: "caudex", front: "caudex", range: [0, 0.42], n: 5, exp: 2.2, pinch: 0.8 });
const leafBones = a.asset.rig.bones.map((b: any) => b.id).filter((id: string) => /^leaf-/.test(id));
for (const bone of leafBones) {
  const planIds = a.shapesOf("plan", bone);
  const id = planIds.find((x) => x === bone) ?? planIds[0]!;
  const rings = a.rings("plan", id);
  const bb = a.box2("plan", id);
  const raw = a.rawOf(bone)?.solid ?? { z1: 0.02, zTip: [-0.01, 0.01] };
  const thick0 = raw.z1 ?? 0.02, thick1 = raw.zTip ? raw.zTip[1] : thick0 * 0.5;
  const L = a.bone(bone).lengthM;
  const secs: Section[] = [];
  const n = 7;
  for (let q = 0; q < n; q++) {
    const t = q / (n - 1), u = bb.minX + (bb.maxX - bb.minX) * t;
    const r = a.slice(rings, "y", 0) && a.slice(rings, "x", Math.min(bb.maxX - 1e-4, Math.max(bb.minX + 1e-4, u)));
    const w = r ? (r[1] - r[0]) / 2 : 0.01, wc = r ? (r[0] + r[1]) / 2 : 0;
    const f = q === n - 1 ? 0.15 : 1;
    secs.push({ c: [u, wc, 0] as V3, rx: Math.max(0.004, (thick0 + (thick1 - thick0) * t) * (q === n - 1 ? 0.5 : 1)), ry: Math.max(w * f, 0.004), exp: 2.4 });
  }
  a.loft(bone, bone, secs, a.meta(bone), { local: true, axis: [1, 0, 0] });
}
a.note("Agave: the caudex is a short loft; every leaf bone carries one rigid lanceolate blade, a loft along the bone whose width, along the bone normal v, follows the authored outline (a flat outline in bone-local coordinates, the same in all three views), whose thickness along w runs from the authored base half thickness (0.02 m) to the tip, with a sharp tapering end. Because every blade is built in its bone frame, the spread of the default pose (abduction and twist of the leaf bones) produces the 3D rosette, so the bounding box is judged in that pose.", "Agave: der Wurzelstock ist ein kurzer Loft; jeder Blattknochen trägt ein starres lanzettliches Blatt, einen Loft entlang des Knochens, dessen Breite entlang der Knochennormalen v dem gezeichneten Umriss folgt (ein flacher Umriss in Knochenkoordinaten, in allen drei Ansichten gleich) und dessen Dicke entlang w von der gezeichneten halben Basisdicke (0,02 m) zur Spitze abnimmt, mit spitz zulaufendem Ende. Da jedes Blatt in seinem Knochenrahmen gebaut ist, ergibt die Auffächerung der Standardpose (Abduktion und Verdrehung der Blattknochen) die räumliche Rosette, daher wird die Bounding Box in dieser Pose beurteilt.");
a.save();
