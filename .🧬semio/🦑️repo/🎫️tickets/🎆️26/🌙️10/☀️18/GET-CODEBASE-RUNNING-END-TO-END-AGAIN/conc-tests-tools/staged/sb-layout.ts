import { realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { isAbsolute, join, relative, resolve, sep } from "node:path";
import protocol from "../🔣️.json" with { type: "json" };

/** 🏖️ Carries the closed test sandbox protocol: environment names, layout, port range and guard policy. */
export type TestSandboxProtocolV1 = Readonly<typeof protocol>;

/** 📖️ Reads the authored sandbox protocol and refuses incomplete declarations. */
export function readTestSandboxProtocolV1(value: unknown = protocol): TestSandboxProtocolV1 {
  const row = value as TestSandboxProtocolV1;
  if (row?.version !== 1 || !row.environment || !row.layout || !row.ports || !row.guard || !Array.isArray(row.rules)) throw Error("Complete test sandbox protocol required");
  for (const name of ["sandbox", "artifacts", "images", "cache", "guard", "report", "checkout"] as const) if (!/^SEMIO_TEST_[A-Z_]+$/u.test(row.environment[name] ?? "")) throw Error("Test sandbox environment names required");
  if (!(row.ports.minimum >= 1024 && row.ports.maximum > row.ports.minimum && row.ports.maximum <= 65535 && row.ports.attempts >= 1)) throw Error("Finite test port range required");
  return row;
}

/** 📁️ Resolves the shared parent below which every sandbox of this host lives; always a short physical path. */
export function testSandboxParentV1(parent?: string, declared: TestSandboxProtocolV1 = protocol): string {
  let base = resolve(parent ?? tmpdir());
  try { base = realpathSync(base); } catch {}
  return join(base, declared.layout.parent);
}

/** 🫀️ Tells whether a process id still exists on this host. */
export function testProcessAliveV1(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid < 1) return false;
  try { process.kill(pid, 0); return true; } catch (error) { return (error as NodeJS.ErrnoException).code === "EPERM"; }
}

/** 📏️ Tells whether a path lies inside a root, comparing case-insensitively where the file system is. */
export function testPathInsideV1(root: string, path: string, platform: NodeJS.Platform = process.platform): boolean {
  const fold = (value: string): string => platform === "win32" || platform === "darwin" ? value.toLowerCase() : value;
  const local = relative(fold(resolve(root)), fold(resolve(path)));
  return local === "" || (local !== ".." && !local.startsWith(".." + sep) && !isAbsolute(local));
}
