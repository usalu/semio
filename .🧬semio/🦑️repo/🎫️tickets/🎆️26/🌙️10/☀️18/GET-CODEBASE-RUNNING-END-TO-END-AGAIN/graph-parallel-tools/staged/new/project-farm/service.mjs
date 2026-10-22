import { mkdtempSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { tmpdir } from "node:os";
import { join, relative, sep } from "node:path";
import { libraryBootstrap } from "../../../🟨️.mjs";
import { createInternEncoderV1 } from "../../../../../../🏗️build/🏊️farm/🟨️.mjs";
import { observationRowDigestV1, observeDetachedProjectInferenceV1, sealInferenceRecordV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";
import { withImplementationPinV1 } from "../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { withBuildGraphStagingV1 } from "../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import { createProjectInferenceEnvironmentV1, inferProjectBodyV1, projectInferenceIdentityV1 } from "../../🟨️.mjs";

const MAXIMUM_RECORD_BYTES = 268435456;
let environment, staging, worker, rows, keys, validation = new Map(), recordBytes = new Map(), recordTotal = 0;
const peerKnown = new Set(), peerBlocks = new Set(), seeded = new Set();
const sentFiles = new Set();

/** 🔌️ Opens one worker session: the same inference implementation as the parent, its own memos, a private staging directory for everything the inference would publish. */
export async function open(session) {
  await libraryBootstrap;
  environment = createProjectInferenceEnvironmentV1(session);
  staging = { root: mkdtempSync(join(tmpdir(), "semio-graph-farm-")), readRoot: session.readRoot };
  worker = randomUUID();
  for (const [key, bytes] of session.seed ?? []) if (recordTotal + bytes.byteLength <= MAXIMUM_RECORD_BYTES) { recordBytes.set(key, bytes); seeded.add(key); recordTotal += bytes.byteLength; }
  rows = createInternEncoderV1();
  keys = createInternEncoderV1();
  return projectInferenceIdentityV1();
}

/** 📤️ Lists the files the inference staged that this worker has not sent yet. */
function stagedFiles() {
  const files = [];
  const walk = directory => { for (const entry of readdirSync(directory, { withFileTypes: true })) { const path = join(directory, entry.name); if (entry.isDirectory()) walk(path); else if (entry.isFile() && !entry.name.endsWith(".tmp")) { const status = statSync(path), name = relative(staging.root, path).split(sep).join("/"), mark = name + ":" + status.size + ":" + status.mtimeMs; if (!sentFiles.has(mark)) { sentFiles.add(mark); files.push({ name, bytes: readFileSync(path) }); } } } };
  walk(staging.root);
  return files;
}

/** ▶️ Infers one project and answers its value, the facts it consumed and the parser records and published files it produced, each shared fact only once per worker. */
export async function run({ known, knownBlocks, digest, ...job }) {
  for (const row of known ?? []) peerKnown.add(row);
  for (const block of knownBlocks ?? []) peerBlocks.add(block);
  return withImplementationPinV1(() => withBuildGraphStagingV1(staging, async () => {
    const puts = [], records = {
      get: key => recordBytes.get(key),
      put: (key, bytes) => { puts.push(key); if (!recordBytes.has(key) && recordTotal + bytes.byteLength <= MAXIMUM_RECORD_BYTES) { recordBytes.set(key, bytes); recordTotal += bytes.byteLength; } },
    };
    const detached = await observeDetachedProjectInferenceV1({ validation, records }, () => inferProjectBodyV1(environment, job, undefined));
    const encodedRows = rows.encode(detached.rows), encodedKeys = keys.encode(puts), sealed = sealInferenceRecordV1({ digest, image: detached.image, rows: detached.rows, known: peerBlocks });
    return { worker, image: detached.image, sealed, started: detached.started, rowIds: encodedRows.ids, freshRows: encodedRows.fresh.map(([id, row]) => { const identity = observationRowDigestV1(row); return peerKnown.has(identity) ? [id, identity] : [id, identity, row]; }), putIds: encodedKeys.ids, freshRecords: encodedKeys.fresh.map(([id, key]) => [id, seeded.has(key) ? { key, seeded: true } : { key, bytes: recordBytes.get(key) }]), files: stagedFiles() };
  }));
}

/** 🧹️ Removes the staging directory. */
export function close() { if (staging) rmSync(staging.root, { recursive: true, force: true }); }
