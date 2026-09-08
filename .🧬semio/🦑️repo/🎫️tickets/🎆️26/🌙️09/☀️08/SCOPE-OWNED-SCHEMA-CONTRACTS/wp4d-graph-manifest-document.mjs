/**
 * 🕸️ Holds `framework.graph.manifest`'s two exports against the real corpus: every committed graph
 * manifest document must satisfy `GraphManifestDocument`, and the live `📇️outputs.json` must satisfy
 * `Outputs`. Ajv is the third-party oracle; the schema is read from the owner module, never restated.
 */
import { readFileSync } from "node:fs";
import { execSync } from "node:child_process";
import { createRequire } from "node:module";

const root = new URL("../../../../../../../", import.meta.url).pathname.replace(/\/$/u, "");
const m = JSON.parse(readFileSync(`${root}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json`, "utf8"));
const Ajv = createRequire(import.meta.url)(`${root}/node_modules/ajv`);
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(m);
const validate = ajv.getSchema(`${m.$id}#/$defs/GraphManifestDocument`);
const files = execSync(`git -C ${JSON.stringify(root)} ls-files -z`, { maxBuffer: 1 << 28 }).toString().split("\0").filter((p) => p.endsWith("manifest.json") && p.startsWith("✏️s/"));
let ok = 0, bad = 0;
for (const rel of files) {
  let doc;
  try { doc = JSON.parse(readFileSync(`${root}/${rel}`, "utf8")); } catch { continue; }
  if (doc?.schema !== "manifest") continue;
  if (validate(doc)) ok += 1;
  else { bad += 1; console.log("FAIL", rel, JSON.stringify(validate.errors?.slice(0, 4))); }
}
console.log(`GraphManifestDocument: ${ok} real manifest document(s) accepted, ${bad} rejected`);
const outputs = ajv.getSchema(`${m.$id}#/$defs/Outputs`);
console.log("Outputs accepts 📇️outputs.json:", outputs(JSON.parse(readFileSync(`${root}/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️outputs.json`, "utf8"))));
