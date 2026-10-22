/** 🧪️ A worker service whose every job writes the same content addressed blocks as the other workers do, all of them starting at the instant the job names, through the very function a farm worker uses. */
import { writeSealedBlocksV1 } from "../../../../../../👁️observation/🟨️.mjs";
import { withBuildGraphWorkerWriterV1 } from "../../../../🟨️.mjs";

let roots;
export async function open(session) { roots = session.roots; return "duplicate-law"; }
export async function run(job) {
  while (Date.now() < job.startAt) await new Promise(done => setTimeout(done, 1));
  return withBuildGraphWorkerWriterV1({ roots, grantBytes: 1 << 20 }, writer => {
    const sealed = { blocks: job.blocks.map(block => ({ digest: block.digest, packed: Buffer.from(block.text) })) };
    const created = writeSealedBlocksV1(roots.inference + "/np", sealed);
    return { pid: process.pid, created, leaves: writer.report().leaves };
  });
}
