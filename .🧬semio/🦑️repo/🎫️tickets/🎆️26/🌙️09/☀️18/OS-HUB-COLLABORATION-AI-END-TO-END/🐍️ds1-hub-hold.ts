/** 🌎️ Slice DS1 session 5 — §9 gap 4. Holds ONE live hub against a data root whose trusted
 * stdio+GIS catalog was just published, using a hub binary this slice built itself, and prints the
 * `/readyz` body so `artifactAuthority.ready` is read at runtime rather than reasoned about.
 *
 * Why not `os-hub:dev` / `hubDevBinaryPath`: that route reads `🌎️hub/📦️packages/🦀️rust/dist/build-dev/os-hub`,
 * the file a SIBLING slice's hub (C1c, pid 5468) is executing right now. Re-staging it would overwrite a
 * running binary in place — a silent SIGKILL for that peer on macOS. So this takes `startLocalHub`'s own
 * `binaryPath` option and points it at DS1's private artifact directory instead, touching nothing shared.
 *
 * Like `🐍️c1c-hub-hold.ts` it goes through `startLocalHub` rather than spawning the binary bare: a
 * development hub completes an authenticated local-bootstrap handshake over an inherited pipe on fd 3,
 * and without it the process aborts before it binds a port.
 *
 * Usage: `bun 🐍️ds1-hub-hold.ts <port> <absoluteDataDir> <absoluteBinaryPath>` */
import { existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

function findRepoRoot(start: string): string {
  let current = start;
  for (let depth = 0; depth < 32; depth += 1) {
    if (existsSync(join(current, "🌎️hub", "📦️packages", "🦀️rust", "Cargo.toml"))) return current;
    const parent = dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw new Error("probe could not locate the repository root above " + start);
}

const repoRoot = findRepoRoot(import.meta.dir);
const hubRustRoot = join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust");
const { startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));

const port = Number(process.argv[2] ?? 7611);
const dataRoot = process.argv[3]!;
const binaryPath = process.argv[4]!;
mkdirSync(dataRoot, { recursive: true });
if (!existsSync(binaryPath)) throw new Error(`build the hub first: ${binaryPath} does not exist`);

const run = await startLocalHub(repoRoot, hubRustRoot, [{ profileId: "developer", subject: "local-developer-ds1", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], {
  port,
  dataDir: dataRoot,
  binaryPath,
  capture: false,
});
const readiness = await waitForReadiness(run, true);
console.log(`HOLD origin=http://127.0.0.1:${port} dataRoot=${dataRoot} status=${readiness.status} artifactAuthority=${JSON.stringify(readiness.artifactAuthority)}`);
console.log(`READINESS ${JSON.stringify(readiness)}`);
await new Promise(() => undefined);
