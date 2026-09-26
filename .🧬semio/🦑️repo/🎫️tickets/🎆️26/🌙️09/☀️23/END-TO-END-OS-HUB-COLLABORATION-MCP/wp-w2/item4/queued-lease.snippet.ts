/** 🚦️ One waiter's place in a resource's arrival queue: `<directory>/<resource-hash>.queue/<arrivedAtMs>-<pid>-<owner>`. */
export type QueuedLeaseOptions = LeaseOptions & { readonly owner: string };

/** 🚦️ Whether the process that wrote a queue ticket still exists (EPERM means it exists under another user). */
function ticketAlive(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid <= 0) return false;
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🚦️ Serves one resource in ARRIVAL order: every waiter files a ticket named by its arrival time, only the oldest live
 * ticket may try the lease, and a ticket whose process is gone is swept, so a crashed waiter never blocks the queue and a
 * long queue (a fleet of agents sharing one wasm build) is neither starved nor overtaken. The lease itself is
 * {@link acquireResourceLease}, so the kernel still releases a crashed holder. The ticket is removed on every exit. */
export async function acquireQueuedResourceLease(options: QueuedLeaseOptions): Promise<ResourceLease> {
  options.signal.throwIfAborted();
  if (!/^[A-Za-z0-9._-]{1,64}$/u.test(options.owner)) throw new Error("Invalid lease queue owner");
  const queue = databasePath(options.directory, options.resource).replace(/\.sqlite$/u, ".queue");
  mkdirSync(queue, { recursive: true });
  const ticket = `${String(Date.now()).padStart(15, "0")}-${String(process.pid).padStart(10, "0")}-${options.owner}`;
  writeFileSync(join(queue, ticket), `${process.pid}\n`, { flag: "wx" });
  const started = Date.now();
  let nextProgress = 0;
  try {
    for (;;) {
      options.signal.throwIfAborted();
      const live = readdirSync(queue).sort().filter((name) => {
        const pid = Number(name.split("-")[1]);
        if (name === ticket || ticketAlive(pid)) return true;
        rmSync(join(queue, name), { force: true });
        return false;
      });
      if (live[0] === ticket) {
        const lease = await acquireResourceLease({ ...options, timeoutMs: options.timeoutMs === undefined ? undefined : Math.max(0, options.timeoutMs - (Date.now() - started)) });
        return { resource: lease.resource, mode: lease.mode, release() {
          try { lease.release(); } finally { rmSync(join(queue, ticket), { force: true }); }
        } };
      }
      const elapsedMs = Date.now() - started;
      if (elapsedMs >= (options.timeoutMs ?? Infinity)) throw new Error(`Resource lease timed out: ${options.resource}`);
      if (elapsedMs >= nextProgress) {
        const progress = { resource: options.resource, mode: options.mode, elapsedMs };
        if (options.onWait) options.onWait(progress);
        else console.log(`Waiting for ${options.mode} access to ${options.resource} behind ${live.indexOf(ticket)} queued owner(s): ${live.slice(0, live.indexOf(ticket)).map((name) => name.split("-").slice(2).join("-")).join(", ")}`);
        nextProgress = elapsedMs + 15_000;
      }
      await delay(Math.min(250, (options.timeoutMs ?? Infinity) - elapsedMs), undefined, { signal: options.signal });
    }
  } catch (error) {
    rmSync(join(queue, ticket), { force: true });
    throw error;
  }
}
