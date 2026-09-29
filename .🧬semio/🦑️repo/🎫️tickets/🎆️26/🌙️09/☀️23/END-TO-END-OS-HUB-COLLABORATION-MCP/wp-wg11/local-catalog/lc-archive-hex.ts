/** 🗃️ WG11 s15 — fills `vocabulary.json`'s `archiveHex` expectations with the TS encoders (the patched `🗂️local-catalog` module,
 * staged under `stage/` with its imports pointed at the live tree), then answers every vector with that module exactly as the
 * engine-contract law does. Usage: bun lc-archive-hex.ts (writes vocabulary.json in place, prints the verdict). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";

const here = import.meta.dir;
const config = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
mkdirSync(`${here}/stage`, { recursive: true });
writeFileSync(`${here}/stage/🔣️.json`, readFileSync(`${here}/vocabulary.json`));
writeFileSync(`${here}/stage/module.ts`, readFileSync(`${here}/new.ts`, "utf8").replace(`"../../../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts"`, JSON.stringify(config)));
const lane = await import(`${here}/stage/module.ts`);
const { decodeDocumentArchiveBytes, decodePackValue } = await import("@semio-tech/framework-os");
const hex = (bytes: Uint8Array) => Buffer.from(bytes).toString("hex");
const vocabulary = JSON.parse(readFileSync(`${here}/vocabulary.json`, "utf8"));
const fill = process.argv.includes("--fill");
let failures = 0;
for (const vector of vocabulary.admissions) {
  const admission = lane.localCatalogAdmissionV1(vector.args ?? undefined, vector.dataDir ?? undefined, vector.nowMs);
  const actual = "refusal" in admission ? { refusal: admission.refusal } : { document: admission.document, folder: admission.binding.path, archiveHex: hex(admission.archive) };
  if (fill && "archiveHex" in vector.expect) vector.expect.archiveHex = actual.archiveHex ?? "";
  const same = JSON.stringify(actual) === JSON.stringify(vector.expect);
  if (!same) failures += 1;
  console.log(`${same ? "ok  " : "FAIL"} admission: ${vector.name}${same ? "" : `\n     expected ${JSON.stringify(vector.expect)}\n     actual   ${JSON.stringify(actual)}`}`);
}
for (const vector of vocabulary.catalogArchives) {
  const bytes = lane.localCatalogArchiveV1(vector.catalog);
  if (fill) vector.archiveHex = hex(bytes);
  const decoded = lane.decodeLocalCatalogArchiveV1(Uint8Array.from(Buffer.from(vector.archiveHex, "hex")));
  const same = hex(bytes) === vector.archiveHex && JSON.stringify(decoded) === JSON.stringify(vector.catalog);
  if (!same) failures += 1;
  const pack = decodePackValue(Uint8Array.from(decodeDocumentArchiveBytes(bytes).parent_pack)) as { documents: Record<string, unknown>[] };
  console.log(`${same ? "ok  " : "FAIL"} catalog archive (${vector.catalog.documents.length} documents, ${bytes.length} bytes, admittedAtMs carried as ${pack.documents.map((document) => typeof document.admittedAtMs === "object" ? (document.admittedAtMs as { kind: string }).kind : typeof document.admittedAtMs).join(",") || "—"})`);
}
for (const [key, copy] of Object.entries(lane.LOCAL_CATALOG_NOTICES_V1) as [string, { en: string; de: string }][]) {
  const spoken = [lane.localCatalogNoticeTextV1(key, "en", "Plan"), lane.localCatalogNoticeTextV1(key, "de", "Plan"), lane.localCatalogNoticeTextV1(key, "fr", "Plan"), lane.localCatalogNoticeCodeV1(key)];
  const same = spoken[0] === copy.en.replace("{name}", "Plan") && spoken[1] === copy.de.replace("{name}", "Plan") && spoken[2] === spoken[0] && spoken[3] === `shell.localCatalog.${key}` && copy.en !== copy.de;
  if (!same) failures += 1;
  console.log(`${same ? "ok  " : "FAIL"} notice ${key}: ${spoken[3]}`);
}
const legacy = lane.decodeLocalCatalogPayloadV1({ documents: [{ documentId: "a", schema: "s", name: "", storage: "file", target: "/a", admittedAtMs: 5 }] });
console.log(`${legacy?.documents[0]?.admittedAtMs === 5 ? "ok  " : "FAIL"} an envelope payload's JSON number still decodes`);
if (fill) writeFileSync(`${here}/vocabulary.json`, `${JSON.stringify(vocabulary, null, 2)}\n`);
console.log(failures === 0 ? "ALL VECTORS ANSWERED" : `${failures} FAILURES`);
process.exit(failures === 0 ? 0 : 1);
