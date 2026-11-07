import { createHash } from "node:crypto";
import { createReadStream, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ticket = process.env.SEMIO_TICKET_DIR!, root = join(ticket, "🗑️generated"), hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const fileHash = async (path: string) => { const h = createHash("sha256"); for await (const data of createReadStream(path)) h.update(data); return h.digest("hex"); };
const catalogBytes = readFileSync(join(root, "bazel-msvc-manifest.json")), catalogHash = "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef";
if (hash(catalogBytes) !== catalogHash) throw Error("Catalog mismatch");
const catalog = JSON.parse(catalogBytes.toString("utf8")), sdk = catalog.packages.find((entry: { id: string }) => entry.id === "Win11SDK_10.0.26100");
const compilerIds = ["Microsoft.VC.14.50.18.0.Tools.HostX64.TargetX64.base", "Microsoft.VC.14.50.18.0.Tools.HostX64.TargetX64.Res.base", "Microsoft.VC.14.50.18.0.CRT.Headers.base", "Microsoft.VC.14.50.18.0.CRT.x64.Desktop.base", "Microsoft.VC.14.50.18.0.CRT.x64.Store.base"];
const evidence = JSON.parse(readFileSync(join(root, "provider-evidence.json"), "utf8")), sdkPlan = JSON.parse(readFileSync(join(root, "portable-msvc-plan.json"), "utf8")), cabinets = JSON.parse(readFileSync(join(root, "sdk-cabinets.json"), "utf8"));
if (evidence.files.length !== 4947 || sdkPlan.sdk.payloads.length !== 8 || cabinets.length !== 34) throw Error("Evidence inventory mismatch");
const payloads: { id: string; name: string; declaredBytes: number; bytes: number; sha256: string }[] = [];
const record = async (id: string, payload: any, local: string, strictSize: boolean) => {
  const path = join(root, local), bytes = statSync(path).size, sha256 = await fileHash(path);
  if (sha256 !== payload.sha256.toLowerCase() || (strictSize && bytes !== payload.size)) throw Error(`Payload integrity mismatch: ${id}`);
  payloads.push({ id, name: payload.fileName, declaredBytes: payload.size, bytes, sha256 });
};
for (let i = 0; i < compilerIds.length; i++) {
  const entries = catalog.packages.filter((entry: { id: string; language?: string }) => entry.id === compilerIds[i] && (!entry.language || entry.language === "en-US"));
  if (entries.length !== 1 || entries[0].payloads.length !== 1) throw Error("Compiler payload ambiguity");
  await record(compilerIds[i]!, entries[0].payloads[0], `msvc-${i}.vsix`, false);
}
for (let i = 0; i < 8; i++) {
  const matches = sdk.payloads.filter((item: { fileName: string }) => item.fileName === sdkPlan.sdk.payloads[i].fileName);
  if (matches.length !== 1) throw Error("MSI payload ambiguity"); await record(`msi-${i}`, matches[0], `sdk-${i}.msi`, true);
}
for (const row of cabinets) {
  if (!/^[a-f0-9]{32}\.cab$/u.test(row.cabinet)) throw Error("Invalid cabinet");
  const matches = sdk.payloads.filter((item: { fileName: string }) => item.fileName.replaceAll("\\", "/").toLowerCase() === `installers/${row.cabinet}`);
  if (matches.length !== 1) throw Error("Cabinet ambiguity"); await record(row.cabinet, matches[0], row.cabinet, true);
}
const tableHashes = [];
for (let i = 0; i < 8; i++) {
  const a = readFileSync(join(root, `sdk-files-${i}.json`)), b = readFileSync(join(root, `sdk-recheck-${i}.json`));
  if (!a.equals(b)) throw Error("MSI table parity failed"); tableHashes.push(hash(b));
}
const recipeNames = ["extract-msvc-vsix.go", "inspect-msvc-vsix.go", "inspect-sdk-files.go", "extract-sdk-flat.go", "msi-paths.ts", "plan-sdk-files.ts", "materialize-sdk.ts", "build-provider-evidence.ts", "canonical-provider-evidence.ts"];
const recipes = recipeNames.map(name => ({ name, sha256: hash(readFileSync(join(ticket, name))) }));
const recipe = { version: 1, catalogSha256: catalogHash, payloads, tableHashes, sdkPlanSha256: hash(readFileSync(join(root, "sdk-file-plan.json"))), vsixInventorySha256: hash(readFileSync(join(root, "msvc-vsix-inventory-5.json"))), recipes };
const recipeBytes = Buffer.from(JSON.stringify(recipe) + "\n"), recipeSha256 = hash(recipeBytes);
const files = evidence.files.map((row: any) => ({ path: row.path, bytes: row.size, sha256: row.sha256 })).sort((a: any, b: any) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path)));
const manifest = { version: 1, recipeSha256, files }, manifestBytes = Buffer.from(JSON.stringify(manifest) + "\n");
writeFileSync(join(root, "provider-recipe.json"), recipeBytes, { flag: "wx" });
writeFileSync(join(root, "provider-manifest.json"), manifestBytes, { flag: "wx" });
console.log(JSON.stringify({ files: files.length, recipeSha256, manifestSha256: hash(manifestBytes), manifestBytes: manifestBytes.length }));
