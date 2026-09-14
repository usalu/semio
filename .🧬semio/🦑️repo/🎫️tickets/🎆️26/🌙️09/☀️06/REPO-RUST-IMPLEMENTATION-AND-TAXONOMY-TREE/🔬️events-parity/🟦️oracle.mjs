// 🟦️ Ticket-local oracle + parity comparison for the 📡️events module.
//
// Third-party judges: `ajv` (JSON Schema draft 2020-12) for every golden payload encoding and the
// envelope, and node:crypto (OpenSSL) for the record checksums, the log bytes and the export
// snapshot identity. Then diffs the Go probe report, the Rust probe report and this oracle.

import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";

const [, , repoRoot, outDir] = process.argv;
const require = createRequire(join(repoRoot, "package.json"));
const Ajv2020 = require("ajv/dist/2020");

const moduleRoot = join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events");
const readJson = (path) => JSON.parse(readFileSync(path, "utf8"));
const fixture = (name) => readJson(join(moduleRoot, "🧫️fixtures", name));

const schema = readJson(join(moduleRoot, "🧬️schema/🔣️.json"));
const kindCatalog = readJson(join(moduleRoot, "🧬️schema/🔣️event-kinds.json"));
const payloadVectors = fixture("✉️payload-vectors.json");
const storeVectors = fixture("🗄️store-vectors.json");
const exportVectors = fixture("📤️export-vectors.json");

const failures = [];
const checks = [];
const check = (name, ok, detail = "") => {
  checks.push({ name, ok, detail });
  if (!ok) failures.push(`${name}${detail ? `: ${detail}` : ""}`);
};

// #region 🔮️ ajv — the goldens must satisfy the schema they claim to describe
const ajv = new Ajv2020({ strict: false });
ajv.addSchema(schema, "events");
let schemaAccepted = 0;
for (const vector of payloadVectors.cases) {
  const validate = ajv.getSchema(`events#/$defs/${vector.type}`);
  if (!validate) {
    check(`ajv/$defs/${vector.type}`, false, "no such subschema");
    continue;
  }
  const ok = validate(JSON.parse(vector.encoded)) === true;
  if (ok) schemaAccepted += 1;
  else check(`ajv/${vector.id}`, false, ajv.errorsText(validate.errors));
}
check("ajv accepts every golden payload encoding", schemaAccepted === payloadVectors.cases.length, `${schemaAccepted}/${payloadVectors.cases.length}`);

const validateEnvelope = ajv.getSchema("events#/$defs/Event");
const envelopeValue = { kind: kindCatalog.kinds[0].kind, source: "repo-cli", payload: { id: "a" } };
check("ajv accepts a declared-kind envelope", validateEnvelope(envelopeValue) === true, ajv.errorsText(validateEnvelope.errors));
check("ajv rejects an undeclared-kind envelope", validateEnvelope({ ...envelopeValue, kind: "ticket.open.not-a-kind" }) === false);
// #endregion

// #region 🔏️ node:crypto — the SHA-256 the implementations claim to compute
const SCHEMA = "semio.event/1";
const recordChecksum = (sequence, input) =>
  createHash("sha256")
    .update(`${SCHEMA}\u0000${sequence}\u0000${input.id}\u0000${input.kind}\u0000`, "utf8")
    .update(JSON.stringify(input.data), "utf8")
    .digest("hex");

const expectedLog =
  storeVectors.inputs
    .map((input, index) =>
      JSON.stringify({
        schema: SCHEMA,
        sequence: index + 1,
        id: input.id,
        kind: input.kind,
        data: input.data,
        checksum: recordChecksum(index + 1, input),
      }),
    )
    .join("\n") + "\n";
const expectedStoreDigest = createHash("sha256").update(expectedLog, "utf8").digest("hex");

const canonical = (value) => JSON.stringify(Object.fromEntries(Object.keys(value).sort().map((key) => [key, value[key]])));
const exportInputs = exportVectors.entities
  .map((entity) => ({ id: `${entity.kind}:${entity.id}`, kind: `${entity.kind}.recorded`, data: canonical(entity.value) }))
  .sort((left, right) => (left.id < right.id ? -1 : left.id > right.id ? 1 : 0));
const snapshotHash = createHash("sha256");
for (const input of exportInputs) snapshotHash.update(`${input.id}\u0000${input.kind}\u0000${input.data}`, "utf8");
const expectedSnapshot = snapshotHash.digest("hex");
const expectedInputIds = exportInputs.map((input) => `snapshot:${expectedSnapshot}:${input.id}`);

check("node:crypto reproduces the frozen export snapshot", expectedSnapshot === exportVectors.snapshot, `${expectedSnapshot} vs ${exportVectors.snapshot}`);
check("node:crypto reproduces the frozen namespaced input ids", JSON.stringify(expectedInputIds) === JSON.stringify(exportVectors.inputIds));
// #endregion

// #region ⚖️ parity — oracle vs Go vs Rust
const oracle = {
  implementation: "oracle",
  kinds: kindCatalog.kinds.map((entry) => entry.kind),
  encodings: Object.fromEntries(payloadVectors.cases.map((vector) => [vector.id, vector.encoded])),
  envelope: JSON.stringify(envelopeValue),
  snapshot: expectedSnapshot,
  inputIds: expectedInputIds,
  storeDigest: expectedStoreDigest,
};

const go = readJson(join(outDir, "🐹️report.json"));
const rust = readJson(join(outDir, "🦀️report.json"));
const goLog = readFileSync(join(outDir, "🐹️store.jsonl"));
const rustLog = readFileSync(join(outDir, "🦀️store.jsonl"));

check("go and rust write byte-identical event logs", goLog.equals(rustLog), `${goLog.length} vs ${rustLog.length} bytes`);
check("the event log equals the bytes node:crypto predicts", goLog.equals(Buffer.from(expectedLog, "utf8")));

/** 🔤️ Canonical JSON of a report field: object keys sorted, so a map's iteration order is not the subject. */
const stable = (value) =>
  JSON.stringify(value, (_key, inner) =>
    inner && typeof inner === "object" && !Array.isArray(inner)
      ? Object.fromEntries(Object.keys(inner).sort().map((key) => [key, inner[key]]))
      : inner,
  );

/** 🔍️ The first differing entry of two encoding maps, so a failure names the payload. */
const firstDifference = (actual, expected) =>
  Object.keys(expected).find((key) => actual[key] !== expected[key]) ?? "";

for (const field of ["kinds", "encodings", "envelope", "snapshot", "inputIds", "storeDigest"]) {
  const expected = stable(oracle[field]);
  for (const [name, report] of [["go", go], ["rust", rust]]) {
    const ok = stable(report[field]) === expected;
    const detail = ok ? "" : field === "encodings" ? `first difference at ${firstDifference(report[field], oracle[field])}` : stable(report[field]).slice(0, 200);
    check(`${name}.${field} matches the oracle`, ok, detail);
  }
}
// #endregion

writeFileSync(join(outDir, "⚖️parity.json"), `${JSON.stringify({ checks, failures }, null, 2)}\n`);
for (const entry of checks) console.log(`${entry.ok ? "PASS" : "FAIL"}  ${entry.name}${entry.ok ? "" : ` — ${entry.detail}`}`);
console.log(`\n${checks.filter((entry) => entry.ok).length}/${checks.length} checks passed`);
process.exit(failures.length === 0 ? 0 : 1);
