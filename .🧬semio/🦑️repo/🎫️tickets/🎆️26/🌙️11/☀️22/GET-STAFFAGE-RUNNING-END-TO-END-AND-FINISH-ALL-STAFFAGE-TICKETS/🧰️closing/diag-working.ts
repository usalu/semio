/** 🔎️ Prints measured against pinned working dimensions at every scale (closing diagnosis of `🧊️model-silhouette`). */
import { readFileSync } from "node:fs";
import { Box3, Vector3 } from "three";
import { copyMesh } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { workingDimensions } = JSON.parse(readFileSync(`${S}/🧫️fixtures/🧊️catalog-silhouette/🔣️.json`, "utf8"));
for (const w of workingDimensions) {
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/${w.asset}/🔣️.json`, "utf8")).asset;
  const row = [50, 100, 200, 500].map((scale) => {
    const piece = asset.volumes?.[scale]?.body?.find((s: { id: string }) => s.id === w.solid);
    if (!piece) return `1:${scale} missing`;
    const mesh = copyMesh(piece, { chordM: 0.0002, minSegments: 32, maxSegments: 32 });
    return `1:${scale} ${new Box3().setFromArray(mesh.positions).getSize(new Vector3()).getComponent(w.axis).toFixed(5)}`;
  });
  console.log(w.asset, w.solid, "axis", w.axis, "pinned", w.metres, "|", row.join("  "));
}
