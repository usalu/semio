/** 🧰️ Shared helpers of the multiview fixer tools: where the staffage lives, how an asset document is found, read and written back in its own indentation, and the vector algebra of the bone frames. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🏠️ The staffage root. */
export const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";

/** 🗺️ The asset document of an id (searched in every category folder) or the file itself when the argument ends in `.json`. */
export function fileOf(arg: string): string {
  if (arg.endsWith(".json")) return arg;
  const root = join(S, "🗂️catalog");
  for (const folder of readdirSync(root, { withFileTypes: true })) {
    if (!folder.isDirectory()) continue;
    try {
      if (readdirSync(join(root, folder.name)).includes(arg)) return join(root, folder.name, arg, "🔣️.json");
    } catch {
      continue;
    }
  }
  throw new Error(`no asset "${arg}" in ${root}`);
}

/** 📖️ An asset document with the text it was read from. */
export function readDoc(arg: string): { file: string; text: string; doc: { asset: any } } {
  const file = fileOf(arg);
  const text = readFileSync(file, "utf8");
  return { file, text, doc: JSON.parse(text) };
}

/** 💾️ Writes the document back in the indentation it was read in, unless the file changed on disk since. */
export function writeDoc(read: { file: string; text: string; doc: unknown }, out?: string): boolean {
  const second = read.text.split("\n", 2)[1] ?? "";
  const indent = second.length - second.trimStart().length;
  const next = JSON.stringify(read.doc, null, indent) + (read.text.endsWith("\n") ? "\n" : "");
  if (readFileSync(read.file, "utf8") !== read.text) throw new Error(`${read.file} changed on disk since it was read: nothing written, run again`);
  if (next === read.text && out === undefined) return false;
  writeFileSync(out ?? read.file, next);
  return true;
}

export type V3 = readonly [number, number, number];
export const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const round = (value: number, digits = 4): number => Number(value.toFixed(digits));

/** 🧲️ The support function of a point set in the plane at an angle. */
export const support = (points: readonly (readonly [number, number])[], angle: number): number => {
  const [c, s] = [Math.cos(angle), Math.sin(angle)];
  let best = Number.NEGATIVE_INFINITY;
  for (const [x, y] of points) best = Math.max(best, x * c + y * s);
  return best;
};
