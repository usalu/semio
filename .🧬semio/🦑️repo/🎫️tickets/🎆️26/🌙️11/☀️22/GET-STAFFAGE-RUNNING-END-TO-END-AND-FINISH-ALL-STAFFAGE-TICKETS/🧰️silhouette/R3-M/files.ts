/** 🗂️ Where an asset is read from: `catalogFile` is the catalog file of an id of any category; `fileOf` is the path itself when a `.json` path is given, the pre-repair copy `$SAM_SOURCE/<id>.json` when that variable names a folder that has one (the authoring scripts are run against the asset as it was before the repair), else the catalog file. */
import { existsSync, globSync } from "node:fs";
export const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
export const catalogFile = (id: string): string => {
  const hits = globSync(`${CATALOG}/*/${id}/🔣️.json`);
  if (hits.length !== 1) throw new Error(`${id}: ${hits.length} catalog files`);
  return hits[0]!;
};
export const fileOf = (arg: string): string => {
  if (arg.endsWith(".json")) return arg;
  const source = process.env.SAM_SOURCE;
  return source !== undefined && source !== "" && existsSync(`${source}/${arg}.json`) ? `${source}/${arg}.json` : catalogFile(arg);
};
