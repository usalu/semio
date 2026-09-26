/** 🚦️ `lease <exclusive|shared> <resource> <owner> -- <command…>`: runs one command while holding a queued lease on a
 * repository resource (arrival order, crashed waiters swept, released on every exit), so a shell caller serializes on
 * exactly the lease the product's own chains take — `wasm-build` for every all-plugin wasm build. */
class LeaseScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [mode, resource, owner, separator, command, ...rest] = args;
    if ((mode !== "exclusive" && mode !== "shared") || !resource || !owner || separator !== "--" || !command) throw new Error("usage: lease <exclusive|shared> <resource> <owner> -- <command…>");
    const controller = new AbortController(), abort = () => controller.abort();
    process.once("SIGINT", abort);
    process.once("SIGTERM", abort);
    const lease = await acquireQueuedResourceLease({ directory: repoCacheDirectory(this.repoRoot, "agents", "resource-leases"), resource, mode, owner, signal: controller.signal });
    try {
      const child = spawn(command, rest, { cwd: process.cwd(), stdio: "inherit" });
      process.on("SIGINT", () => child.kill("SIGINT"));
      process.on("SIGTERM", () => child.kill("SIGTERM"));
      process.exitCode = await new Promise<number>((accept) => child.once("exit", (code, signal) => accept(code ?? (signal ? 1 : 0))));
    } finally {
      lease.release();
    }
  }
}

