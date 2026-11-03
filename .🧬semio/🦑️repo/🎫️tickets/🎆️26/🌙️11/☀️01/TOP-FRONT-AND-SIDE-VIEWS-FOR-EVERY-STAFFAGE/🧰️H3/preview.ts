import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/H3/pv";
const { runPreview } = await import(pathToFileURL(`${S}/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts`).href);
const scales = (process.env.PV_SCALE ?? "100").split(",");
const ids = process.argv.slice(2).filter((a) => !a.startsWith("--"));
for (const scale of scales) for (const id of ids) {
  const asset0 = JSON.parse(readFileSync(`${S}/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
  if (asset0.representations[scale]?.status !== "authored") continue;
  const asset = JSON.parse(readFileSync(`${S}/🗂️catalog/👤️humans/${id}/🔣️.json`, "utf8")).asset;
  const jobs: string[][] = [];
  for (const p of asset.poses.filter((p: any) => !p.key)) jobs.push(["--pose", p.id]);
  for (const c of asset.clips) for (const f of [0.15, 0.5, 0.85]) jobs.push(["--clip", c.id, "--time", String((c.durationS * f).toFixed(3))]);
  for (const j of jobs) {
    try { await runPreview(["--asset", id, "--scale", scale, "--views", "--dpi", "70", "--out", `${OUT}/${id}-${scale}`, ...j]); }
    catch (e) { console.log("FAIL", id, j.join(" "), (e as Error).message); }
  }
}
console.log("done");
