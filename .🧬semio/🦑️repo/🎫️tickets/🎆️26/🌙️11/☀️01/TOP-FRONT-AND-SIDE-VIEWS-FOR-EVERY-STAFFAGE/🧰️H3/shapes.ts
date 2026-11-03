import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(pathToFileURL(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`).href);
const [id, scale, view, axis] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) } });
const rows = st.shapes[view].map((e: any) => {
  const v = e.rings.flat().map((p: any) => p[axis]);
  return [e.shape.id, Math.min(...v).toFixed(3), Math.max(...v).toFixed(3)];
});
rows.sort((a: any, b: any) => Number(b[2]) - Number(a[2]));
console.log(rows.slice(0, 8).map((r: any) => r.join(" ")).join("\n"));
rows.sort((a: any, b: any) => Number(a[1]) - Number(b[1]));
console.log("--min\n" + rows.slice(0, 4).map((r: any) => r.join(" ")).join("\n"));
