/**
 * 🧮️ Recomputes every literal digest that the reworked `InferenceIdentityV1` binding shape derives,
 * for the ledger and committed-WAL corpora, mirroring the Rust serialization order exactly.
 * Run: `bun .🧬semio/…/fable-ai-map-proposal-identity-recompute.mjs` from the repository root.
 */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const root = process.cwd();
const ledgerPath = `${root}/🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json`;
const walPath = `${root}/🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json`;
const frozenPath = `${root}/🌎️hub/🧪️fixtures/🗺️gis-map-frozen-binding-v1/🔣️.json`;

const sha256 = (value) => createHash("sha256").update(value).digest("hex");
const ledger = JSON.parse(readFileSync(ledgerPath, "utf8"));
const wal = JSON.parse(readFileSync(walPath, "utf8"));
const frozen = JSON.parse(readFileSync(frozenPath, "utf8"));

const binding = {
  digest: frozen.expectedDigest,
  catalogGenerationId: frozen.binding.catalogGenerationId,
  packageId: frozen.binding.package.packageId,
  packageVersion: frozen.binding.package.version,
  componentSha256: frozen.binding.package.componentSha256,
  componentBlake3: frozen.binding.package.componentBlake3,
  artifactKind: frozen.binding.artifact.kind,
  documentSchema: frozen.binding.artifact.schema,
  parentDialect: { artifactKind: frozen.binding.parentDialect.artifactKind, standard: frozen.binding.parentDialect.standard, subset: frozen.binding.parentDialect.subset },
  surfaceId: frozen.binding.surface.surfaceId,
  grantedMode: "read-write-observe",
  serviceId: frozen.binding.service.inferenceSchema,
  serviceVersion: frozen.binding.service.inferenceSchemaVersion,
  algorithmVersion: frozen.binding.service.algorithmVersion,
};

const previous = ledger.identity;
ledger.identity = {
  request: previous.request,
  userId: previous.userId,
  sessionId: previous.sessionId,
  authorizationGeneration: previous.authorizationGeneration,
  spaceId: previous.spaceId,
  documentId: previous.documentId,
  descriptorDigest: previous.descriptorDigest,
  binding,
  headOrdinal: previous.headOrdinal,
  headEditId: previous.headEditId,
  lastCommitSeq: previous.lastCommitSeq,
  chainHash: previous.chainHash,
  inputHash: previous.inputHash,
};
ledger.identityDigest = sha256(`semio.hub.inference-identity/v1\0${JSON.stringify(ledger.identity)}`);
const jobId = sha256(`semio.hub.inference-job-id/v1\0${ledger.identityDigest}`).slice(0, 32);
const proposalHash = ledger.outbox.proposalHash;
const mutationId = sha256(`semio.hub.inference-approval-mutation/v1\0${jobId}\0${proposalHash}`).slice(0, 32);

const encoded = [];
const integer = (value) => {
  let remaining = BigInt(value);
  do { const byte = Number(remaining & 127n); remaining >>= 7n; encoded.push(byte | (remaining ? 128 : 0)); } while (remaining);
};
const bytes = (value) => { integer(value.byteLength); encoded.push(...value); };
const text = (value) => bytes(Buffer.from(value, "utf8"));
const command = { ...wal.command, mutationId };
text(command.mutationId); text(command.documentId); text(command.actor);
integer(command.dependencies.length); command.dependencies.forEach(text);
text(command.diff.schema); bytes(Buffer.from(command.diff.payloadHex, "hex"));
text(command.inverse.schema); bytes(Buffer.from(command.inverse.payloadHex, "hex"));
integer(command.timestamp.actor); integer(command.timestamp.physicalMs); integer(command.timestamp.logical);
const canonical = Buffer.from(encoded);
const encodedHex = canonical.toString("hex");
const commandHash = sha256(canonical);

console.log(JSON.stringify({ identityJson: JSON.stringify(ledger.identity), bindingJson: JSON.stringify(binding), identityDigest: ledger.identityDigest, jobId, mutationId, commandHash, encodedHex }, null, 1));
