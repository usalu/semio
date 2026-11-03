/** 🧭️ World outflow direction of every directional emitter (exhaust, vent, flue, fan) of the catalog at the default pose: `bun emitter-directions.ts [--all]`. Prints one line per emitter with the unit direction (ahead, up, left) and flags the ones that blow backward or downward in the default pose. */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRoot(): string {
  let dir = import.meta.dirname;
  while (!existsSync(join(dir, ".git"))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error("repository root not found");
    dir = parent;
  }
  return dir;
}

const L = join(findRoot(), "🎓️teaching/🏛️architecture/🧍️staffage");
const load = async (path: string): Promise<any> => import(`file:///${join(L, path).replaceAll("\\", "/")}`);
const { solveTransport } = await load("🔨️modules/💨️transport/🟦️.ts");
const all = process.argv.includes("--all");
const root = join(L, "🗂️catalog");
for (const category of readdirSync(root, { withFileTypes: true }).filter((e) => e.isDirectory()))
  for (const folder of readdirSync(join(root, category.name), { withFileTypes: true }).filter((e) => e.isDirectory())) {
    const file = join(root, category.name, folder.name, "🔣️.json");
    if (!existsSync(file)) continue;
    const asset = JSON.parse(readFileSync(file, "utf8")).asset;
    const directional = asset.thermal.emitters.filter((e: any) => ["exhaust", "vent", "flue", "fan"].includes(e.kind));
    if (directional.length === 0) continue;
    const field = solveTransport(asset, { kind: "pose", pose: asset.presentation.defaultPose }, asset.thermal.environment, { pathlines: false, streamlines: false, maxAgeS: 1 });
    for (const state of field.emitters) {
      if (!directional.some((e: any) => e.id === state.emitter)) continue;
      const d = state.direction;
      const flag = d.x < -0.5 ? "BACKWARD" : d.y < -0.5 ? "DOWN" : "";
      if (all || flag !== "") console.log(`${asset.id} ${state.emitter} ${state.kind} dir=(${d.x.toFixed(2)}, ${d.y.toFixed(2)}, ${d.z.toFixed(2)}) ${flag}`);
    }
  }
