/** 🧰️ Shared loader of the s2 probes: the staffage root comes from `ROOT` (a mirror built by `mirror.sh`) or is the worktree, read only. */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

export const REAL = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
export const ROOT = process.env.ROOT || REAL;
export const mod = <T = any>(relative: string): Promise<T> => import(join(ROOT, relative));
export const assetFile = (id: string): string => {
  const catalog = join(REAL, "🗂️catalog");
  for (const group of readdirSync(catalog, { withFileTypes: true })) if (group.isDirectory() && existsSync(join(catalog, group.name, id, "🔣️.json"))) return join(catalog, group.name, id, "🔣️.json");
  throw new Error(`no asset ${id}`);
};
export const readAsset = (idOrFile: string): any => JSON.parse(readFileSync(idOrFile.endsWith(".json") ? idOrFile : assetFile(idOrFile), "utf8")).asset;
