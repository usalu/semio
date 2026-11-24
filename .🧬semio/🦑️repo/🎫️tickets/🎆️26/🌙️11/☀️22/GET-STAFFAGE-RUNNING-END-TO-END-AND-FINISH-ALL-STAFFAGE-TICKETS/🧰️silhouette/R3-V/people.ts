/** 🗺️ The files of the objects of batch R3-V: the catalog file and the repair ops of each (`ops/<id>.json` beside this file). */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const W = import.meta.dir;
export const WORKER: Readonly<Record<string, string>> = new Proxy({}, { get: () => "R3-V" });
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/🚜️vehicles/${slug}/🔣️.json`;
export const opsFile = (slug: string): string => `${W}/ops/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
