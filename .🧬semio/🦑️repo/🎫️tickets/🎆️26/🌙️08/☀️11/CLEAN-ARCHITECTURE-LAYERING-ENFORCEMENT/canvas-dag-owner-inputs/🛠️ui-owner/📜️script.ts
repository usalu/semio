import { existsSync, readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { collectIconShortcodeSources, generateIconShortcodeTables } from "./🔎️shortcodes/🛠️generator/🟦️.ts";
import { validateJsonSchemaSubset } from "../../🧬️schema/✅️validator/🟦️.ts";

/** 🔎️ Generates direct Canvas asset bindings in the defining General UI owner. */
export async function generateCanvasShortcodes(repoRoot: string, control: { readonly signal: AbortSignal; readonly progress: (directory: string, entries: number) => void }): Promise<void> {
  const assets = resolve(repoRoot, "🧰️framework/🔨️modules/🖼️assets"), schema = JSON.parse(readFileSync(resolve(import.meta.dir, "🔎️shortcodes/🧬️schema/📇️sources/🔣️.json"), "utf8")), data = JSON.parse(readFileSync(resolve(assets, "🔣️icons/🔣️shortcodes.json"), "utf8"));
  const errors = validateJsonSchemaSubset(schema, data);
  if (errors.length) throw Error("shortcode source contract: " + JSON.stringify(errors));
  const catalog = collectIconShortcodeSources(resolve(assets, "🔣️icons"), repoRoot, control.signal, control.progress), themed = collectIconShortcodeSources(resolve(assets, "🌱️metabolism/🔣️icons"), repoRoot, control.signal, control.progress);
  if (catalog.length !== data.catalog.length || new Set(data.catalog).size !== data.catalog.length || !themed.length) throw Error("shortcode catalog identity closure differs");
  for (const id of data.catalog) if (!catalog.some(row => row.key === id)) throw Error("missing shortcode catalog identity: " + id);
  const directory = resolve(import.meta.dir, "🔎️shortcodes/🤖️generated"), after = generateIconShortcodeTables({ directory: relative(repoRoot, directory).replaceAll("\\", "/"), emoji: data.emoji, catalog, themed }, control.signal).source, destination = resolve(directory, "🦀️.rs");
  control.signal.throwIfAborted();
  if (!existsSync(destination) || readFileSync(destination, "utf8") !== after) {
    mkdirSync(dirname(destination), { recursive: true });
    writeFileSync(destination, after);
  }
  control.progress(directory, Object.keys(data.emoji).length + themed.length + catalog.length);
}
