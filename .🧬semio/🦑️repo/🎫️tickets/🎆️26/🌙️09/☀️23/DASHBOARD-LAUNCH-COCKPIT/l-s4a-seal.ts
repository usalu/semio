/** 🧷️ Prints the frozen-coordinate seal and coordinate digest of one contract id (reseal evidence for slice L-S4a). */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { frozenCoordinateEvidenceSeal, loadCatalogTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { canonicalJson } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧾️serialization/🔣️json/🟦️.ts";
import { frozenCoordinateEvidenceCoordinates } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts";

const root = "/Users/ueli/Documents/semio", sha = (value: string) => createHash("sha256").update(value).digest("hex");
const contracts = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts, seal = frozenCoordinateEvidenceSeal(contracts);
for (const id of process.argv.slice(2)) {
  const { retired: _retired, ...live } = contracts[id]!;
  const coordinates = frozenCoordinateEvidenceCoordinates(live.path, readFileSync(join(root, live.path)), { [id]: live })!.map(({ pointer, kind, value }) => ({ pointer, kind, value }));
  console.log(JSON.stringify({ id, seal: sha(canonicalJson(seal[id]!)), coordinatesSha256: sha(canonicalJson(coordinates)) }));
}
