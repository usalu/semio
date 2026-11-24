/** 🗺️ The catalog file of a plant by its slug (or the file itself). */
export const fileOf = (name: string): string => name.endsWith(".json") ? name : `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/${name}/🔣️.json`;
