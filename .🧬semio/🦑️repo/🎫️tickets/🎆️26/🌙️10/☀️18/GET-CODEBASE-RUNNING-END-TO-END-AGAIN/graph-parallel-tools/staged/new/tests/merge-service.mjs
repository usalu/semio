/** 🧪️ A deterministic stand-in for the project inference of a worker: the node and the facts of owner `n` are a pure function of `n`, and the facts shared by many owners are the same objects every time they are sent. */
import { createInternEncoderV1 } from "../../../../../🏊️farm/🟨️.mjs";

const rows = createInternEncoderV1(), shared = Array.from({ length: 40 }, (_, index) => ({ kind: "file", path: "/world/shared/" + index + ".ts", fact: { size: index, sha256: String(index).padStart(64, "0") } })), own = new Map();
const ownRows = n => { let list = own.get(n); if (!list) own.set(n, list = Array.from({ length: 5 }, (_, index) => ({ kind: "file", path: "/world/owner-" + n + "/" + index + ".ts", fact: { size: n * 10 + index, sha256: String(n * 10 + index).padStart(64, "0") } }))); return list; };

/** 🧬️ The same pure function the serial reference uses. */
export function ownerResult(n) {
  const factsOf = [...ownRows(n), ...shared.filter((_, index) => (index + n) % 3 !== 0)];
  return { node: { name: "owner-" + n, root: "owner-" + n, targets: { build: { command: "build " + n }, test: { command: "test " + n } } }, rows: factsOf };
}

export async function open() { return "merge-law"; }
export async function run(job) {
  await new Promise(done => setTimeout(done, job.delay));
  const result = ownerResult(job.n), encoded = rows.encode(result.rows);
  return { worker: "worker-" + process.pid, node: result.node, rowIds: encoded.ids, freshRows: encoded.fresh };
}
