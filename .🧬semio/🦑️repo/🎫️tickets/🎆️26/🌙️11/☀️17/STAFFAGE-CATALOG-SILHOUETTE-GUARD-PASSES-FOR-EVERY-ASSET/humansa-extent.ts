/** 📏 Volume extent and drawn extent per authored scale against the declared dimensions. bun humansa-extent.ts <asset>... */
import { VIEWPOINTS, featureOf, loadAsset, stateOf } from "./humansa-lib.ts";
import { extentOf, worldSolids, boundsOf, drawnFor } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
for (const name of process.argv.slice(2)) {
  const asset = await loadAsset(name);
  const d = asset.dimensions;
  console.log(name, "dims w", d.widthM, "h", d.heightM, "d", d.depthM);
  for (const scale of [500, 200, 100, 50]) {
    if ((asset.volumes as any)[String(scale)]?.status !== "authored") continue;
    const state = stateOf(asset, scale);
    const e = extentOf(worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true));
    const dr = VIEWPOINTS.map((v) => { const g = drawnFor(asset, state, v, scale); let b = [Infinity, -Infinity, Infinity, -Infinity]; for (const s of g) { const bb = boundsOf(s.rings); b = [Math.min(b[0]!, bb[0]), Math.max(b[1]!, bb[1]), Math.min(b[2]!, bb[2]), Math.max(b[3]!, bb[3])]; } return `${v[0]}[${b.map((x) => x.toFixed(3)).join(",")}]`; });
    console.log(` 1:${scale} feat ${featureOf(scale)} vol x ${e.min[0]!.toFixed(4)}..${e.max[0]!.toFixed(4)} (${(e.max[0]! - e.min[0]!).toFixed(4)}) y ${e.min[1]!.toFixed(4)}..${e.max[1]!.toFixed(4)} (${(e.max[1]! - e.min[1]!).toFixed(4)}) z ${e.min[2]!.toFixed(4)}..${e.max[2]!.toFixed(4)} (${(e.max[2]! - e.min[2]!).toFixed(4)})  drawn ${dr.join(" ")}`);
  }
}
