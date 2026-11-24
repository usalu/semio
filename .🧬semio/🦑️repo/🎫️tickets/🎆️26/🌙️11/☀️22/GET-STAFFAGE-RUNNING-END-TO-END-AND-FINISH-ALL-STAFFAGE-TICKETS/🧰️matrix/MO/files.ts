/** 🗺️ The files of the assets of batch MO: the catalog document of an id in whatever category folder it lies, and its repair ops `ops/<id>.json` beside this file. */
import { Glob } from "bun";
export const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const W = import.meta.dir;
export const catalogFile = (slug: string): string => {
  const found = [...new Glob(`*/${slug}/🔣️.json`).scanSync({ cwd: `${S}/🗂️catalog`, absolute: true })];
  if (found.length !== 1) throw new Error(`${found.length} catalog documents for "${slug}"`);
  return found[0]!;
};
export const opsFile = (slug: string): string => `${W}/ops/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
