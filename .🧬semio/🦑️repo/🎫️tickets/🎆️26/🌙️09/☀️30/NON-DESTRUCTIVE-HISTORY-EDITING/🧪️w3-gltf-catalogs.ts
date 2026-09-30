/** 🗂️ W3-GLTF: aligns every glTF mutation catalog vector with the committed corpus. A vector's scenario `directoryName` is
 * the physical fixture bundle and implementation case directory (`♾️any/🧫️fixtures/🧬️mutations/<entity>/<verb>/<case>`), so
 * the canonical case pair projects onto itself; the one uncatalogued leaf, `set-snapshot`, joins the any subset's catalog.
 * `--check` reports drift and writes nothing. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";

const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets";
const CORPUS = `${SUBSETS}/♾️any/🧫️fixtures/🧬️mutations`;
const TAXONOMY = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const check = process.argv.includes("--check");

interface Scenario { id: string; directoryName: string }
interface Vector { mutationId: string; sourceMutationDirectoryName: string; mutationDirectoryName: string; scenarios: Scenario[] }
interface Catalog { id: string; capability: string; standardDirectoryName: string; subsetDirectoryName: string; vectors: Vector[]; kinds: string[] }

const domains = JSON.parse(readFileSync(TAXONOMY, "utf8")).mutationDomainOwners[`${SUBSETS.replace("/Users/ueli/Documents/semio/", "")}/♾️any/🧬️schema/🧬️mutations`] as Record<string, Record<string, string>>;
const cases = new Map<string, { verb: string; directory: string }>();
for (const [entity, operations] of Object.entries(domains))
  for (const [verb, kind] of Object.entries(operations)) {
    const found = readdirSync(`${CORPUS}/${entity}/${verb}`);
    if (found.length !== 1) throw new Error(`${kind}: ${found.length} committed cases`);
    cases.set(kind, { verb, directory: found[0]! });
  }

const slug = (directory: string): string => directory.replace(/^(?:\p{Extended_Pictographic}️?)+/u, "");
let drift = 0;
for (const subset of readdirSync(SUBSETS).filter((name) => !name.includes("."))) {
  const path = `${SUBSETS}/${subset}/🔮️oracles/🔣️.json`;
  const text = readFileSync(path, "utf8");
  const document = JSON.parse(text) as { mutationCatalogs: Catalog[] };
  for (const catalog of document.mutationCatalogs) {
    if (subset === "♾️any" && !catalog.vectors.some((vector) => vector.mutationId === "set-snapshot")) {
      const { verb, directory } = cases.get("set-snapshot")!;
      catalog.vectors.push({ mutationId: "set-snapshot", sourceMutationDirectoryName: verb, mutationDirectoryName: verb, scenarios: [{ id: slug(directory), directoryName: directory }] });
      if (!catalog.kinds.includes("set-snapshot")) catalog.kinds.push("set-snapshot");
    }
    for (const vector of catalog.vectors) {
      const committed = cases.get(vector.mutationId);
      if (!committed || vector.scenarios.length !== 1) throw new Error(`${subset} ${vector.mutationId}: no single committed case`);
      vector.scenarios[0]!.directoryName = committed.directory;
    }
  }
  const aligned = `${JSON.stringify(document, null, 2)}\n`;
  if (aligned === text) continue;
  drift += 1;
  console.log(`${check ? "drift" : "aligned"} ${subset}/🔮️oracles/🔣️.json`);
  if (!check) writeFileSync(path, aligned);
}
console.log(`[w3-gltf-catalogs] ${check ? "drift" : "aligned"}=${drift}`);
if (check && drift > 0) process.exit(1);
