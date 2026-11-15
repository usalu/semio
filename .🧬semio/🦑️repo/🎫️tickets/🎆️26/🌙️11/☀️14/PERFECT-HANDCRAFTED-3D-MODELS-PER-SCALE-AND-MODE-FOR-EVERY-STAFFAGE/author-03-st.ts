import { entryOf, loadSource, merged, scaleMeshes, viewMasks, poseState, distanceField, meshMask, VIEWS } from "./volumes-lib.ts";
const [spec, sc, list] = [process.argv[2]!, Number(process.argv[3] ?? 50), process.argv[4] ?? "structure"];
const { entry, asset } = entryOf(spec);
const source = loadSource(entry)!;
const a = merged(asset, source);
const sm: any = scaleMeshes(a, sc, source.volumes[String(sc)]);
const meshes = (sm as any)[list] as any[];
const solids = sm.solids[list] as any[];
for (const view of VIEWS) {
  const vm = viewMasks(view, poseState(a, sc), sm.body);
  const f = distanceField(vm.volume);
  const out: string[] = [];
  meshes.forEach((m, i) => {
    const mk = meshMask(view, [m], vm.box, vm.cell);
    let worst = 0;
    for (let k = 0; k < mk.cells.length; k++) if (mk.cells[k] && f[k]! > worst) worst = f[k]!;
    if (worst > 0.02) out.push(`${solids[i].id}:${worst.toFixed(3)}`);
  });
  console.log(view, out.join(" "));
}
