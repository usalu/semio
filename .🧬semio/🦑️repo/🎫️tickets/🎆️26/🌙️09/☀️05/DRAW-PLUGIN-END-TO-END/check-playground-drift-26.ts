/** 🔍️ Compares the live Specific owner's in-memory playground projection with published bytes. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { renderCatalogFiles } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts";
import { specificCatalogPublicationV1 } from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts";
const root = process.cwd(), publication = specificCatalogPublicationV1(path => readFileSync(resolve(root, path), "utf8"));
const { files } = renderCatalogFiles(root, undefined, "refuse", publication);
for (const name of ["🎠️playgrounds.json", "🚀️playgrounds.json", "🎮️playgrounds/🟦️.ts"]) {
  const old = readFileSync(resolve(root, publication.outputRoot, name), "utf8"), fresh = files[name];
  if (fresh === undefined) throw Error(`Missing generated output ${name}`);
  console.log(JSON.stringify({ name, oldBytes: Buffer.byteLength(old), freshBytes: Buffer.byteLength(fresh), equal: old === fresh }));
  await Bun.write(resolve(import.meta.dir, "🗑️generated", `preview-${name.replaceAll("/", "-")}`), fresh);
}
