/** 🧰️ Shared pieces of the multiview fixers of batch MS: where an asset lives, how it is read, and how a changed asset is written back with every untouched byte kept (`spans.ts`). The file is read again right before it is written.
 * @see ./spans.ts */
import { Glob } from "bun";
import { patched } from "./spans.ts";

export const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";

/** 📍️ The catalog file of an asset id; a path stays a path. */
export function fileOf(id: string): string {
  if (id.endsWith(".json")) return id;
  const found = [...new Glob(`*/${id}/🔣️.json`).scanSync({ cwd: `${S}/🗂️catalog`, absolute: true })];
  if (found.length !== 1) throw new Error(`${id}: ${found.length} catalog files`);
  return found[0]!;
}

/** 📥️ The text and the parsed document of an asset (`--in file` of the arguments replaces the catalog file). */
export async function load(id: string, args: readonly string[] = process.argv): Promise<{ file: string; out: string; text: string; doc: any; asset: any }> {
  const flag = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
  const file = flag("--in") ?? fileOf(id);
  const text = await Bun.file(file).text();
  const doc = JSON.parse(text);
  return { file, out: flag("--out") ?? file, text, doc, asset: doc.asset };
}

/** 📤️ Writes a changed document: only the changed shapes, solids, anchors, decisions and dimensions are edited in the text. Returns whether the file changed. */
export async function save(loaded: { file: string; out: string; text: string; doc: any }): Promise<boolean> {
  if ((await Bun.file(loaded.file).text()) !== loaded.text) throw new Error(`${loaded.file}: changed since it was read`);
  const next = patched(loaded.text, loaded.doc);
  if (next === loaded.text && loaded.out === loaded.file) return false;
  await Bun.write(loaded.out, next);
  return next !== loaded.text;
}

/** 🔢️ A number written with at most `digits` decimals. */
export const round = (value: number, digits = 5): number => Number(value.toFixed(digits));
