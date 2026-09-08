#!/usr/bin/env node
/** 🔍️ Probes every non-hub scope export the hub script must bind to, against the real fixture data. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";

const repoRoot = process.argv[2] ?? process.cwd();
const catalog = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json"), "utf8"));

const externalModules = {
  "s.stdio.registry": "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🔣️.json",
};

function modulePath(scope) {
  const entry = catalog.scopes[scope];
  if (entry) return join(entry.path, entry.formats["🔣️jsonschema"]);
  const declared = externalModules[scope];
  if (!declared) throw new Error(`scope ${scope} is in neither the catalog nor the bridge table`);
  return declared;
}

const ajv = new Ajv({ strict: true, allErrors: true });
const ids = new Map();
function load(scope) {
  if (ids.has(scope)) return ids.get(scope);
  const document = JSON.parse(readFileSync(join(repoRoot, modulePath(scope)), "utf8"));
  if (document.$schema !== "http://json-schema.org/draft-07/schema#") throw new Error(`${scope} is not draft-07: ${document.$schema}`);
  ajv.addSchema(document);
  ids.set(scope, document.$id);
  for (const dependency of catalog.scopes[scope]?.dependsOn ?? []) load(dependency);
  return document.$id;
}
function compile(scope, exportId) {
  return ajv.compile({ $ref: `${load(scope)}#/$defs/${exportId}` });
}

const cases = [
  ["os.directory", "BrowserDocumentOpenTransportV1", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", (f) => f],
  ["os.directory", "BrowserDocumentOpenTransportPlan", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json", (f) => f.plan],
  ["os.directory", "DocumentBrowserActorReservationV1", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️browser-actor-reservation-v1.json", (f) => f],
  ["os.directory", "ExecutionTargetBodyReadV1", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️execution-target-body-read-v1.json", (f) => f],
  ["os.directory", "DocumentBrowserActorSessionV1", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️browser-actor-session-v1.json", (f) => f],
  ["os.directory", "DirectoryArtifactBootstrapOwnerV1", "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧵️artifact-bootstrap-owner-v1.json", (f) => f],
  ["os.directory", "CheckpointPublicationCommandV1", "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📣️checkpoint-publication-command-v1/🔣️.json", (f) => f],
  ["os.db.storage", "MemoryBackingV1", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing/🔣️.json", (f) => f],
  ["os.plugin.builder", "TopicContributionsV1", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️fixtures/📇️topic-contributions/🔣️.json", (f) => f],
  ["framework.value.codec", "CodecFixture", "🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🔣️.json", (f) => f],
  ["s.stdio.registry", "NativeCatalogSurface", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/🔣️.json", (f) => f],
  ["s.stdio.registry", "NativeCatalogSurfaceCommitmentCases", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/🧪️commitment.json", (f) => f],
  ["s.stdio.registry", "NativeCatalogSurfaceImports", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/🧪️imports.json", (f) => f],
  ["s.stdio.registry", "NativeCatalogSurfaceBudget", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/🧪️budget.json", (f) => f],
  ["s.stdio.registry", "ClaimAuthority", "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/🧾️claim-authority/🔣️.json", (f) => f],
  ["s.stdio.registry", "NativeCodecFactories", "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", (f) => f],
];

let failures = 0;
for (const [scope, exportId, fixture, pick] of cases) {
  try {
    const validate = compile(scope, exportId);
    const value = pick(JSON.parse(readFileSync(join(repoRoot, fixture), "utf8")));
    const ok = validate(value);
    console.log(`${ok ? "OK  " : "FAIL"} ${scope}/${exportId} <- ${fixture}${ok ? "" : ` :: ${JSON.stringify(validate.errors?.slice(0, 3))}`}`);
    if (!ok) failures++;
  } catch (error) {
    console.log(`ERR  ${scope}/${exportId} <- ${fixture} :: ${error.message}`);
    failures++;
  }
}
/* 🧬️ Compile-only exports (no committed fixture instance in the hub script). */
for (const [scope, exportId] of [["s.stdio.registry", "NativeCatalogSurfaceCommitment"], ["os.directory", "SpaceArtifactCreationV1"]]) {
  try {
    compile(scope, exportId);
    console.log(`OK   compile ${scope}/${exportId}`);
  } catch (error) {
    console.log(`ERR  compile ${scope}/${exportId} :: ${error.message}`);
    failures++;
  }
}
console.log(`failures=${failures}`);
process.exit(failures === 0 ? 0 : 1);
