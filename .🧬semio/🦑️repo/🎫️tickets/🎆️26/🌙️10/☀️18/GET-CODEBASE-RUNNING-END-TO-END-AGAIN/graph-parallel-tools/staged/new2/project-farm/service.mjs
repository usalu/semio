import { randomUUID } from "node:crypto";
import { libraryBootstrap } from "../../../🟨️.mjs";
import { createInternEncoderV1 } from "../../../../../../🏗️build/🏊️farm/🟨️.mjs";
import { observationRowDigestV1, observeDetachedProjectInferenceV1, sealInferenceRecordV1, writeSealedBlocksV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";
import { withImplementationPinV1 } from "../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";
import { withBuildGraphWorkerWriterV1 } from "../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import { createProjectInferenceEnvironmentV1, inferProjectBodyV1, projectInferenceIdentityV1 } from "../../🟨️.mjs";

const MAXIMUM_RECORD_BYTES = 268435456;
let environment, roots, worker, rows, keys, validation, recordTotal = 0;
const recordBytes = new Map(), peerKnown = new Set(), peerBlocks = new Set(), seeded = new Set(), emitted = new Set();

/** 🔌️ Opens one worker session, again for every later build of the same process: the same inference implementation as the parent with fresh memos and tables, the directories the parent publishes to, and the parser records the parent already holds (the records this process kept from an earlier build stay, they are named by the bytes they describe). */
export async function open(session) {
  await libraryBootstrap;
  environment = createProjectInferenceEnvironmentV1(session);
  roots = session.roots;
  worker = randomUUID();
  validation = new Map();
  peerKnown.clear(); peerBlocks.clear(); seeded.clear(); emitted.clear();
  for (const [key, bytes] of session.seed ?? []) if (recordBytes.has(key) || recordTotal + bytes.byteLength <= MAXIMUM_RECORD_BYTES) { if (!recordBytes.has(key)) { recordBytes.set(key, bytes); recordTotal += bytes.byteLength; } seeded.add(key); }
  rows = createInternEncoderV1();
  keys = createInternEncoderV1();
  return projectInferenceIdentityV1();
}

/** ▶️ Infers one project and answers its value, the facts it consumed, the parser records it produced and the names of the graph leaves it wrote itself (closure inventories, input precision receipts, the blocks of its record), each shared fact only once per worker; the cargo resolution receipts it would refresh travel as manifest names, because a receipt is shared by every project of its closure and has exactly one writer, the parent. */
export async function run({ known, knownBlocks, digest, directory, grantBytes, ...job }) {
  for (const row of known ?? []) peerKnown.add(row);
  for (const block of knownBlocks ?? []) peerBlocks.add(block);
  return withImplementationPinV1(() => withBuildGraphWorkerWriterV1({ roots, grantBytes }, async writer => {
    const puts = [], records = {
      get: key => recordBytes.get(key),
      put: (key, bytes) => { puts.push(key); if (!recordBytes.has(key) && recordTotal + bytes.byteLength <= MAXIMUM_RECORD_BYTES) { recordBytes.set(key, bytes); recordTotal += bytes.byteLength; } },
    };
    const detached = await observeDetachedProjectInferenceV1({ validation, records }, () => inferProjectBodyV1(environment, job, undefined));
    const encodedRows = rows.encode(detached.rows), encodedKeys = keys.encode(puts), full = sealInferenceRecordV1({ digest, image: detached.image, rows: detached.rows, known: peerBlocks, emitted });
    const created = grantBytes > 0 && directory !== undefined ? writeSealedBlocksV1(directory, full) : undefined;
    if (created === undefined) for (const block of full.blocks) emitted.delete(block.digest);
    const { blocks: _blocks, ...sealed } = full, report = writer.report();
    return { worker, image: detached.image, sealed: created === undefined ? undefined : sealed, started: detached.started, rowIds: encodedRows.ids, freshRows: encodedRows.fresh.map(([id, row]) => { const identity = observationRowDigestV1(row); return peerKnown.has(identity) ? [id, identity] : [id, identity, row]; }), putIds: encodedKeys.ids, freshRecords: encodedKeys.fresh.map(([id, key]) => [id, seeded.has(key) ? { key, seeded: true } : { key, bytes: recordBytes.get(key) }]), leaves: report.leaves, receipts: report.receipts };
  }));
}

/** 🧹️ Nothing to release: a worker writes its content addressed leaves straight into the graph directory (tmp then rename, within the byte grant of the job) and keeps no private files; the parent accounts the leaves it names. */
export function close() {}
