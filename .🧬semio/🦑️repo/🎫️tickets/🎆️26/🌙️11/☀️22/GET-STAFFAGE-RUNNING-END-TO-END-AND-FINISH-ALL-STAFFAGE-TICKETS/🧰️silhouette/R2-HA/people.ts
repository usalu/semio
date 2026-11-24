/** 🗺️ The seven people of the batch R2-HA and where their files live: the repair ops of this round are `ops/<slug>.json` of this folder and run after the ops of the first round (batch SH-2). */
export const R = "/Users/ueli/Documents/semio";
export const S = `${R}/🎓️teaching/🏛️architecture/🧍️staffage`;
export const catalogFile = (slug: string): string => `${S}/🗂️catalog/👤️humans/${slug}/🔣️.json`;
export const opsFile = (slug: string): string => `${import.meta.dir}/ops/${slug}.json`;
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : catalogFile(arg));
