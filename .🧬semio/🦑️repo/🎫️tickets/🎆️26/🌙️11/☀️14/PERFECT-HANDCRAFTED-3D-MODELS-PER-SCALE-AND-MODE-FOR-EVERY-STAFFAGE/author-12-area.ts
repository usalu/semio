import { entryOf, loadSource, merged, scaleMeshes, meshArea, minExtent, type Json } from "./volumes-lib.ts";
const [spec, sc, listName] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const vol = (m.volumes as Json)[sc!];
const sm = scaleMeshes(asset, Number(sc), vol);
const list = (listName ?? "thermal") as "body" | "thermal" | "structure";
sm[list].forEach((mesh, i) => {
  const s = sm.solids[list][i]!;
  let lo = [1e9, 1e9, 1e9], hi = [-1e9, -1e9, -1e9];
  for (let k = 0; k < mesh.world.length; k += 3) for (let a = 0; a < 3; a++) { lo[a] = Math.min(lo[a]!, mesh.world[k + a]!); hi[a] = Math.max(hi[a]!, mesh.world[k + a]!); }
  console.log(s.id.padEnd(22), s.bone.padEnd(11), s.kind.slice(0,3), (s.region ?? s.member ?? ""), "area", meshArea(mesh).toFixed(4), "lo", lo.map((v) => v.toFixed(3)).join(","), "hi", hi.map((v) => v.toFixed(3)).join(","), "min", minExtent(asset, Number(sc), s).toFixed(4));
});
