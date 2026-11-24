/** 🗺️ The files of the seven people of batch R2-HB: the catalog file and the repair ops of this round (`ops/<slug>.json` beside this file, applied after the ops of batch SH-2). */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const W = import.meta.dir;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const opsFile = (slug: string): string => `${W}/ops/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
