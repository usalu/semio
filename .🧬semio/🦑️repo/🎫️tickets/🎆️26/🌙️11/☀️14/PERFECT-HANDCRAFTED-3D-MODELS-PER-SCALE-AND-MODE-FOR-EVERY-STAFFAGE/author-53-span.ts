import { entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks, type Json, type View } from "./volumes-lib.ts";
const [spec, sc, view, axis, ...vals] = [process.argv[2]!, Number(process.argv[3]), process.argv[4] as View, process.argv[5]!, ...process.argv.slice(6)];
const { entry, asset } = entryOf(spec);
const m = merged(asset, loadSource(entry)!);
const state = poseState(m, sc);
const sm = scaleMeshes(m, sc, m.volumes[String(sc)]);
const vm = viewMasks(view, state, sm.body);
const spans = (mk: any, at: number): string => {
  const out: string[] = [];
  const w = mk.width, h = mk.height;
  if (axis === "y") { const j = Math.floor((at - mk.box.minY) / mk.cell); let s = -1; for (let i = 0; i <= w; i++) { const on = i < w && mk.cells[j * w + i]; if (on && s < 0) s = i; if (!on && s >= 0) { out.push(`[${(mk.box.minX + s * mk.cell).toFixed(2)},${(mk.box.minX + i * mk.cell).toFixed(2)}]`); s = -1; } } }
  else { const i = Math.floor((at - mk.box.minX) / mk.cell); let s = -1; for (let j = 0; j <= h; j++) { const on = j < h && mk.cells[j * w + i]; if (on && s < 0) s = j; if (!on && s >= 0) { out.push(`[${(mk.box.minY + s * mk.cell).toFixed(2)},${(mk.box.minY + j * mk.cell).toFixed(2)}]`); s = -1; } } }
  return out.join(" ");
};
for (const v of vals) console.log(`${axis}=${v}\n  drawn ${spans(vm.truth, Number(v))}\n  3D    ${spans(vm.volume, Number(v))}`);
