// 🔍️ Lists the mounted files of a package whose own `super::` chains reach beyond the file's module, with the reaching lines.
import {readFileSync, existsSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {collectDecls, superReach} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const manifest = resolve(ROOT, process.argv[2]!, "Cargo.toml");
const only = process.argv[3];
const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
const entry = resolve(dirname(manifest), parsed.lib?.path ?? "src/lib.rs");
const readText = (p: string) => existsSync(p) ? readFileSync(p, "utf8") : null;
const tree = collectDecls(entry, readText(entry)!, readText);
const seen = new Set<string>();
for (const d of tree.decls.values()) {
  if (!d.file || seen.has(d.file)) continue;
  seen.add(d.file);
  const text = readText(d.file); if (text === null) continue;
  const r = superReach(text);
  if (r > 0 && (!only || (only === "notest" ? !d.testOnly : true))) {
    const rel = d.file.replace(/^.*🪆️subsets\/♾️any\//u, "");
    console.log(`${r}\t${d.testOnly ? "T" : "L"}\t${rel}\t${d.id.split("::").slice(-3).join("::")}`);
    if (process.env.LINES) text.split("\n").forEach((l, i) => { if (/\bsuper\b/u.test(l)) console.log(`      ${i + 1}: ${l.trim().slice(0, 150)}`); });
  }
}
