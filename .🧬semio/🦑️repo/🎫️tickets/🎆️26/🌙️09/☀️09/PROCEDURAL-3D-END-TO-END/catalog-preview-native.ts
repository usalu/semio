/** 🧩️ Graph-free byte comparison of the dev catalog's generated files against `renderCatalogFiles`; `--write` publishes only when fresh output is requested. */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { specificCatalogPublicationV1 } from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
const root = resolve(import.meta.dir, "../../../../../../..");
const publication = specificCatalogPublicationV1(path => readFileSync(join(root, path), "utf8"));
const { files } = renderCatalogFiles(root, undefined, "exclude", publication);
const out = join(root, publication.outputRoot), write = process.argv.includes("--write");
const stale: string[] = [], dump = process.env.DUMP_DIR;
for (const [name, content] of Object.entries(files)) {
  let actual = ""; try { actual = readFileSync(join(out, name), "utf8"); } catch {}
  if (dump) { const target = join(dump, name.replace(/\//gu, "__")); writeFileSync(target, content); }
  if (actual !== content) { stale.push(name); if (write && /playgrounds/u.test(name)) writeFileSync(join(out, name), content); }
}
console.log("[DEBUG] stale:", JSON.stringify(stale));
