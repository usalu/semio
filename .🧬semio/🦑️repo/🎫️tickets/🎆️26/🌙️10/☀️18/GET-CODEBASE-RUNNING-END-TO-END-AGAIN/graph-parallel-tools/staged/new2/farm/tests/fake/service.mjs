/** 🧪️ A deterministic farm service for the laws: answers a job with a pure function of it after a delay the job names, and can crash, fail or report a large footprint on request; every opening of a session is counted so a law sees a worker reopen. */
let identity, opens = 0;
export async function open(session) { identity = session.identity; opens++; return identity; }
export async function run(job) {
  await new Promise(done => setTimeout(done, job.delay ?? 0));
  if (job.crash) process.exit(9);
  if (job.fail) throw Object.assign(new Error("failed " + job.n), { code: "LAW" });
  return { value: { n: job.n, square: job.n * job.n }, ids: Uint32Array.from([job.n, job.n + 1]), pid: process.pid, opens, identity, big: job.big ? "x".repeat(job.big) : undefined };
}
