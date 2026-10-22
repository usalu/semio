/** 🧪️ A second deterministic farm service, so a law sees two service kinds share the same worker processes. */
export async function open(session) { return session.identity; }
export async function run(job) { return { kind: "other", twice: job.n * 2, pid: process.pid }; }
