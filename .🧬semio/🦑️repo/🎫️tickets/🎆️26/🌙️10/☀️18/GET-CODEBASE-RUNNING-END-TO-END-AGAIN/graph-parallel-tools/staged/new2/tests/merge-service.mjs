/** 🧪️ A deterministic stand-in for the project inference of a worker: the node and the facts of owner `n` are a pure function of `n`, the facts shared by many owners are the same objects every time they are sent, and the graph leaves of owner `n` (an input precision receipt, the block shared by every owner) are written by the worker itself into the graph directory the parent named. */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { createInternEncoderV1 } from "../../../../../🏊️farm/🟨️.mjs";
import { withBuildGraphOptionalPublicationV1, withBuildGraphWorkerWriterV1 } from "../../../../../📥️inputs/💾️storage/🟨️.mjs";

let rows, roots;
const shared = Array.from({ length: 40 }, (_, index) => ({ kind: "file", path: "/world/shared/" + index + ".ts", fact: { size: index, sha256: String(index).padStart(64, "0") } })), own = new Map();
const ownRows = n => { let list = own.get(n); if (!list) own.set(n, list = Array.from({ length: 5 }, (_, index) => ({ kind: "file", path: "/world/owner-" + n + "/" + index + ".ts", fact: { size: n * 10 + index, sha256: String(n * 10 + index).padStart(64, "0") } }))); return list; };

/** 🧬️ The same pure function the serial reference uses. */
export function ownerResult(n) {
  const factsOf = [...ownRows(n), ...shared.filter((_, index) => (index + n) % 3 !== 0)];
  return { node: { name: "owner-" + n, root: "owner-" + n, targets: { build: { command: "build " + n }, test: { command: "test " + n } } }, rows: factsOf };
}

/** 🍃️ The leaves of owner `n` as (name relative to the inference namespace, bytes): one receipt of its own and the content addressed block every owner shares. */
export function ownerLeaves(n) {
  const block = Buffer.from("shared block of the law"), name = createHash("sha256").update(block).digest("hex");
  return [{ name: "ip/" + createHash("sha256").update("owner " + n).digest("hex").slice(0, 24) + ".json", bytes: Buffer.from(JSON.stringify({ owner: n, rows: ownRows(n).length })) }, { name: "np/ob/" + name + ".br", bytes: block }];
}

/** 💾️ Writes one leaf the way every producer of the graph does: temporary name, then rename, named by its optional publication. */
export function writeLeaf(inference, leaf, publish) {
  const path = join(inference, leaf.name), temporary = path + "." + process.pid + "." + Date.now() + ".tmp";
  return publish(path, { peakBytes: leaf.bytes.length, retainedDeltaBytes: leaf.bytes.length }, () => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(temporary, leaf.bytes, { flag: "wx" }); renameSync(temporary, path); });
}

export async function open(session) { rows = createInternEncoderV1(); roots = session.roots; return "merge-law"; }
export async function run(job) {
  await new Promise(done => setTimeout(done, job.delay));
  const result = ownerResult(job.n), encoded = rows.encode(result.rows);
  return withBuildGraphWorkerWriterV1({ roots, grantBytes: 4096 }, async writer => {
    for (const leaf of ownerLeaves(job.n)) if (!existsSync(join(roots.inference, leaf.name))) writeLeaf(roots.inference, leaf, withBuildGraphOptionalPublicationV1);
    return { worker: "worker-" + process.pid, node: result.node, rowIds: encoded.ids, freshRows: encoded.fresh, leaves: writer.report().leaves };
  });
}
