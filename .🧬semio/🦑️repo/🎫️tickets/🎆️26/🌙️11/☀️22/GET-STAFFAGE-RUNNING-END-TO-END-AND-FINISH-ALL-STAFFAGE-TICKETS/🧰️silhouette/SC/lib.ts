/** 🧰️ Shared paths and file access of the silhouette fixer of batch SC. */
export const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
export const CAT = `${S}/🗂️catalog/🧱️components`;

/** 📄️ The catalog file of a component, or the given json file. */
export const fileOf = (arg: string): string => (arg.endsWith(".json") ? arg : `${CAT}/${arg}/🔣️.json`);

/** 📥️ Reads a document with its indentation. */
export async function readDoc(arg: string): Promise<{ doc: any; indent: number; text: string }> {
  const text = await Bun.file(fileOf(arg)).text();
  const second = text.split("\n", 2)[1] ?? "";
  return { doc: JSON.parse(text), indent: second.length - second.trimStart().length, text };
}

/** 📤️ Writes a document back in its own indentation when it changed; returns whether it did. */
export async function writeDoc(arg: string, doc: any, indent: number, before: string): Promise<boolean> {
  const text = JSON.stringify(doc, null, indent) + (before.endsWith("\n") ? "\n" : "");
  if (text === before) return false;
  await Bun.write(fileOf(arg), text);
  return true;
}
