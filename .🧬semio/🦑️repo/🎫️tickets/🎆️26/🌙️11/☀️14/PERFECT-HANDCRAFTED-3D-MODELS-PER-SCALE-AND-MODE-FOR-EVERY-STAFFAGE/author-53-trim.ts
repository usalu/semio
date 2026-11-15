import { entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks, distanceField, meshMask, featureM, type Json, type View } from "./volumes-lib.ts";
import { openCtx, saveLists } from "./kit-lib.ts";
import { projectWorld, toWorld } from "./volumes-base.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3])];
const tolM = Number(process.argv[4] ?? 0.3) * featureM(sc);
const c = openCtx(spec, sc);
const { entry, asset } = entryOf(spec);
const orig = new Map<string, Json>((loadSource(entry)!.volumes[String(sc)].structure as Json[]).map((s) => [s.id, JSON.parse(JSON.stringify(s))]));
const age = new Map<string, number>();
for (let round = 0; round < 14; round++) {
  const m = merged(asset, loadSource(entry)!);
  const state = poseState(m, sc);
  const sm = scaleMeshes(m, sc, m.volumes[String(sc)]);
  const bad = new Set<string>();
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const vm = viewMasks(view, state, sm.body);
    const df = distanceField(vm.volume);
    sm.structure.forEach((mesh: any, i: number) => {
      const sd = sm.solids.structure[i];
      if (sd.part === "support") return;
      const mk = meshMask(view, [mesh], vm.box, vm.cell);
      for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && df[k]! > tolM) { bad.add(sd.id); break; }
    });
  }
  if (bad.size === 0) { console.log(`trim ${sc}: clean after ${round} rounds`); break; }
  const fresh = loadSource(entry)!.volumes[String(sc)].structure as Json[];
  const next = fresh.map((s) => {
    if (!bad.has(s.id)) return s;
    const t = JSON.parse(JSON.stringify(s));
    age.set(s.id, (age.get(s.id) ?? 0) + 1);
    if ((age.get(s.id) ?? 0) > 5 || (t.kind === "loft" && t.sections.length <= 2 && (age.get(s.id) ?? 0) > 2)) {
      const o = orig.get(s.id)!;
      const pts: number[][] = o.kind === "loft" ? o.sections.map((q: Json) => q.at) : o.kind === "capsule" ? [o.p0, o.p1] : [o.transform?.translation ?? [0, 0, 0]];
      const m2 = merged(asset, loadSource(entry)!);
      const st2 = poseState(m2, sc);
      const sm2 = scaleMeshes(m2, sc, m2.volumes[String(sc)]);
      const fr = st2.frames[o.bone]!;
      const vms = (["elevation", "front", "plan"] as View[]).map((v) => ({ v, vm: viewMasks(v, st2, sm2.body) }));
      const r0 = o.kind === "loft" ? Math.min(o.sections[0].rx, o.sections[0].ry) : o.kind === "capsule" ? o.r0 : o.kind === "box" ? Math.min(o.size[1], o.size[2]) / 2 : o.radii[0];
      const okPt = (p: number[]): boolean => {
        const w = toWorld(fr, p as [number, number, number]);
        return vms.every(({ v, vm }) => { const q = projectWorld(v, w); const df = (vm as any).df ?? ((vm as any).df = distanceField(vm.volume)); const i = Math.floor((q.x - vm.box.minX) / vm.cell), j = Math.floor((q.y - vm.box.minY) / vm.cell); const k = j * vm.volume.width + i; return vm.volume.cells[k] === 1 && Math.min(...[[0, 0]].map(() => 1)) === 1; });
      };
      const good = pts.find(okPt) ?? pts[0]!;
      return { id: s.id, kind: "ellipsoid", bone: o.bone, material: o.material, role: "body", tone: o.tone, radii: [Math.max(r0 * 0.6, 0.55 * featureM(sc)), Math.max(r0 * 0.6, 0.55 * featureM(sc)), Math.max(r0 * 0.6, 0.55 * featureM(sc))], transform: { translation: good }, member: o.member, part: o.part };
    }
    if (t.kind === "loft") {
      if (t.sections.length > 2) t.sections.pop();
      else { t.sections = t.sections.map((q: Json) => ({ ...q, rx: q.rx * 0.8, ry: q.ry * 0.8 })); }
    } else if (t.kind === "capsule") {
      t.p1 = t.p1.map((v: number, i: number) => t.p0[i] + (v - t.p0[i]) * 0.8);
      t.r0 *= 0.9; t.r1 *= 0.9;
    } else if (t.kind === "box") t.size = t.size.map((v: number) => v * 0.8);
    else if (t.kind === "ellipsoid") t.radii = t.radii.map((v: number) => v * 0.85);
    return t;
  });
  saveLists(c, { structure: next });
  if (round === 13) console.log(`trim ${sc}: still outside: ${[...bad].join(", ")}`);
}
