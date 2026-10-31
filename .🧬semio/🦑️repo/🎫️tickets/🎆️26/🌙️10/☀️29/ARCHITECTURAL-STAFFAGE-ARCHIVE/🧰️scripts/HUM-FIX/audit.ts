/** [DEBUG] Audit humans: validateAsset plus suitability violation codes. */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { measureSuitability, PRINT_PROFILES } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const root = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const profile = PRINT_PROFILES.profiles.find((p) => p.id === "office-laser")!;
const only = process.argv[2];
const out: Record<string, unknown> = {};
for (const folder of (await readdir(root)).sort()) {
  if (only && folder !== only) continue;
  const path = resolve(root, folder);
  const files = await readdir(path);
  const docName = files.find((f) => f.endsWith(".json"));
  if (!docName) continue;
  const doc = JSON.parse(await readFile(resolve(path, docName), "utf8"));
  const asset = doc.asset;
  const issues = validateAsset(asset).map((i) => `${i.code} ${i.path}: ${i.message.en}`);
  const suits: string[] = [];
  for (const den of [500, 200, 100, 50] as const) {
    const rep = asset.representations[`${den}`];
    if (!rep || rep.status !== "authored") {
      suits.push(`1:${den} ${rep?.status ?? "missing"}`);
      continue;
    }
    const s = measureSuitability(asset, rep, den, profile);
    const codes = new Map<string, number>();
    for (const v of s.violations) codes.set(v.check, (codes.get(v.check) ?? 0) + 1);
    const sample = s.violations.slice(0, 8).map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}<${v.requiredMm}`);
    suits.push(`1:${den} suitable=${s.suitable} stroke=${s.minStrokeMm.toFixed(3)} gap=${Number.isFinite(s.minGapMm) ? s.minGapMm.toFixed(3) : "-"} feat=${Number.isFinite(s.minFeatureMm) ? s.minFeatureMm.toFixed(3) : "-"} ${[...codes].map(([k, n]) => `${k}:${n}`).join(",")} | ${sample.join(" ; ")}`);
  }
  out[asset.id] = { issues, suits };
  console.log("\n==", asset.id, "issues", issues.length);
  for (const i of issues) console.log(" ", i);
  for (const line of suits) console.log(" ", line);
}
const dest = process.argv[3];
if (dest) await writeFile(dest, JSON.stringify(out, null, 1));
