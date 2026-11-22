/** 🚜️ PW-11 driver: builds one desert plant, writes the candidate to `🗑️generated/PW-11/<id>/🔣️.json`, and on request validates it, renders sheets and 2D drawings with the round 3 tools, runs the product check and ships it (catalog file and manifest row).
 * Usage: `repo tools exec bun -- run.ts <saguaro-cactus|prickly-pear|yucca> [--validate "<r3-validate flags>"] [--sheet 50,100[:list[:pose]]] [--2d 50,100[:pose]] [--ship] [--check]`
 * @see ./kit.ts */
import { copyFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
import { CATALOG_VEG, GENERATED, STAFFAGE, WAVE, documentOf, manifestRow, writeJson } from "./kit.ts";

const R3 = join(WAVE, "../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE");
const MODULES: Record<string, string> = { "saguaro-cactus": "./saguaro.ts", "prickly-pear": "./prickly-pear.ts", yucca: "./yucca.ts" };
const args = process.argv.slice(2);
const id = args[0]!;
const flag = (name: string) => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const mod = await import(MODULES[id]!);
const asset = mod.asset();
const candidate = join(GENERATED, id, "🔣️.json");
writeJson(candidate, documentOf(asset));
process.stdout.write(`[DEBUG] ${id}: candidate ${candidate}; rig frame error ${mod.REPORT.rigError.toExponential(2)}; dims ${JSON.stringify(asset.dimensions)}; report ${JSON.stringify(mod.REPORT).slice(0, 600)}\n`);
for (const s of ["50", "100", "200", "500"]) process.stdout.write(`[DEBUG] 1:${s} shapes ${asset.representations[s].geometry.elevation.length}/${asset.representations[s].geometry.front.length}/${asset.representations[s].geometry.plan.length} body ${asset.volumes[s].body.length} thermal ${asset.volumes[s].thermal.length} structure ${asset.volumes[s].structure.length}\n`);

const run = (cwd: string, script: string, rest: string[]) => {
  process.stdout.write(`[DEBUG] > ${script} ${rest.join(" ")}\n`);
  const r = spawnSync(process.execPath, [script, ...rest], { cwd, stdio: "inherit" });
  return r.status ?? 1;
};
const v = flag("--validate");
if (v !== undefined) run(R3, "r3-validate.ts", ["--file", candidate, ...v.split(" ").filter((x) => x !== "")]);
const sheet = flag("--sheet");
if (sheet !== undefined) {
  const [scales, list, pose] = sheet.split(":");
  for (const s of scales!.split(",")) run(R3, "r3-sheet.ts", ["--file", candidate, "--scale", s, ...(list ? ["--list", list] : []), ...(pose ? ["--pose", pose] : [])]);
}
const two = flag("--2d");
if (two !== undefined) {
  const [scales, pose, vws] = two.split(":");
  for (const s of scales!.split(",")) run(R3, "r3-2d.ts", ["--file", candidate, "--scale", s, ...(pose ? ["--pose", pose] : []), ...(vws ? ["--views", vws] : [])]);
}
if (args.includes("--ship")) {
  const target = join(CATALOG_VEG, id, "🔣️.json");
  mkdirSync(join(CATALOG_VEG, id), { recursive: true });
  copyFileSync(candidate, target);
  writeJson(join(WAVE, "📥️manifest-rows", `${id}.json`), manifestRow(asset, mod.DISTINCTION), true);
  process.stdout.write(`[DEBUG] shipped ${target}\n`);
}
if (args.includes("--check")) run(STAFFAGE, "🔨️modules/🗂️catalog/🩺️check/🟦️.ts", ["--file", join(CATALOG_VEG, id, "🔣️.json"), ...(flag("--check-args") ?? "").split(" ").filter((x) => x !== "")]);
