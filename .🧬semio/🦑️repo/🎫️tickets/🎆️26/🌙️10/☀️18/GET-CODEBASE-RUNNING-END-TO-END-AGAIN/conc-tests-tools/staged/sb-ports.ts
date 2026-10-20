import { randomInt } from "node:crypto";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { join } from "node:path";
import { testProcessAliveV1, type TestSandboxProtocolV1, readTestSandboxProtocolV1 } from "../📐️layout/🟦️.ts";

/** 🔌️ Names the sandbox that holds one port lease. */
export type TestPortLeaseV1 = Readonly<{ port: number; pid: number; sandbox: string }>;

/** 📖️ Reads the lease row of one port, or undefined when the lease is absent or torn. */
export function readTestPortLeaseV1(directory: string, port: number, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): TestPortLeaseV1 | undefined {
  try {
    const row = JSON.parse(readFileSync(join(directory, declared.ports.directory, String(port), declared.layout.owner), "utf8")) as TestPortLeaseV1;
    return Number.isSafeInteger(row.port) && Number.isSafeInteger(row.pid) && typeof row.sandbox === "string" ? row : undefined;
  } catch { return undefined; }
}

/** 🔓️ Releases one port lease held by the sandbox. */
export function releaseTestPortV1(directory: string, port: number, sandbox: string, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): void {
  const row = readTestPortLeaseV1(directory, port, declared);
  if (row && row.sandbox !== sandbox) return;
  rmSync(join(directory, declared.ports.directory, String(port)), { recursive: true, force: true });
}

async function bindable(port: number): Promise<boolean> {
  return new Promise<boolean>(done => {
    const probe = createServer();
    probe.once("error", () => done(false));
    probe.listen(port, "127.0.0.1", () => probe.close(() => done(true)));
  });
}

/** 🎟️ Reserves one free loopback port for the sandbox through an atomic lease directory shared by every sandbox of the host, so two concurrent runs never receive the same number. */
export async function reserveTestPortV1(directory: string, sandbox: string, signal?: AbortSignal, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): Promise<number> {
  const parent = join(directory, declared.ports.directory);
  mkdirSync(parent, { recursive: true, mode: 0o700 });
  for (let attempt = 0; attempt < declared.ports.attempts; attempt++) {
    if (signal?.aborted) throw Error("Test port reservation cancelled");
    const port = randomInt(declared.ports.minimum, declared.ports.maximum + 1), lease = join(parent, String(port));
    try { mkdirSync(lease, { mode: 0o700 }); } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "EEXIST") throw error;
      const held = readTestPortLeaseV1(directory, port, declared);
      if (held && testProcessAliveV1(held.pid)) continue;
      rmSync(lease, { recursive: true, force: true });
      try { mkdirSync(lease, { mode: 0o700 }); } catch { continue; }
    }
    writeFileSync(join(lease, declared.layout.owner), JSON.stringify({ port, pid: process.pid, sandbox }));
    if (await bindable(port)) return port;
    rmSync(lease, { recursive: true, force: true });
  }
  throw Error("No free test port within the declared range");
}
