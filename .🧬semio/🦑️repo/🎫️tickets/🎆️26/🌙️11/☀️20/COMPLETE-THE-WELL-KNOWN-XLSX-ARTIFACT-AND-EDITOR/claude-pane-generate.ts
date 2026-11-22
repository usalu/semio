import { readFileSync } from "node:fs";
import { join } from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const { GenerateScript } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts"));
const { specificCatalogPublicationV1 } = await import(join(repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts"));
const self = { repoRoot, publication: specificCatalogPublicationV1((p: string) => readFileSync(join(repoRoot, p), "utf8")) };
(GenerateScript.prototype.run as (this: unknown, s: string[]) => void).call(self, []);
console.log("[DEBUG] catalog generated");
