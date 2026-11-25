import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { CheckScript } from '../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts';
import { specificCatalogPublicationV1 } from '../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts';
const root = process.cwd();
try { CheckScript.prototype.run.call({ repoRoot: root, publication: specificCatalogPublicationV1(path => readFileSync(resolve(root, path), 'utf8')) } as CheckScript, []); } catch (error) { console.error("CHECK ERROR", String(error)); process.exitCode = 1; }
