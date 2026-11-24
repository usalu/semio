/** 🌳️ Where the plants of R4-P live: an id maps to its catalog file, a path stays a path. */
export const CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : `${CATALOG}/${arg}/🔣️.json`);
/** 🧷️ The repair ops of a plant of R4-P. */
export const opsFile = (id: string): string => `${import.meta.dir}/ops/${id}.json`;
