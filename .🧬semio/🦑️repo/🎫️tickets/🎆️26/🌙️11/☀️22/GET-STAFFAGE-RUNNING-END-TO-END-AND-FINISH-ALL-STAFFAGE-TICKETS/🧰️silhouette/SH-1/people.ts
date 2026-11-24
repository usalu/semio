/** 🗺️ Paths of the batch SH-1: the catalog file of a person and the repair ops `ops/<slug>.json` of this folder (the last step after every earlier generator and repair of the person). */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const W = import.meta.dir;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const opsFile = (slug: string): string => `${W}/ops/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
