import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), hash = (data: Uint8Array) => createHash("sha256").update(data).digest("hex");
const sourceRecipe = readFileSync(join(root, "provider-recipe.json")), sourceManifest = readFileSync(join(root, "provider-manifest.json"));
if (hash(sourceRecipe) !== "a1437bb39b2edbf9cc18f8460b1eb079ecd64425f31790f794a336bb2d7a3bcc" || hash(sourceManifest) !== "1baafc58de2b0d376d6d60c20735c485fbb64931fe32a8004b1c7d97374aa01d") throw Error("Original evidence mismatch");
const prior = JSON.parse(sourceRecipe.toString("utf8")), priorManifest = JSON.parse(sourceManifest.toString("utf8"));
if (prior.payloads.length !== 47 || prior.tableHashes.length !== 8 || priorManifest.files.length !== 4947) throw Error("Evidence cardinality mismatch");
const recipe = { version: 1, algorithm: "semio-msvc-1", catalogUrl: "https://download.visualstudio.microsoft.com/download/pr/fdc37f6e-59f6-4054-838a-b476eeaa6ec3/1d82370739911457e0a2f6be15d8b5f569531b352b2eabb961429eea1e99e356/VisualStudio.vsman", catalogSha256: prior.catalogSha256, payloads: prior.payloads, tableHashes: prior.tableHashes }, recipeBytes = Buffer.from(JSON.stringify(recipe) + "\n"), recipeSha256 = hash(recipeBytes);
const manifest = { version: 1, recipeSha256, files: priorManifest.files }, manifestBytes = Buffer.from(JSON.stringify(manifest) + "\n"), manifestSha256 = hash(manifestBytes);
writeFileSync(join(root, "provider-recipe-v1.json"), recipeBytes, { flag: "wx" });
writeFileSync(join(root, "provider-manifest-v1.json"), manifestBytes, { flag: "wx" });
console.log(JSON.stringify({ recipeSha256, manifestSha256, recipeBytes: recipeBytes.length, manifestBytes: manifestBytes.length, files: manifest.files.length }));
