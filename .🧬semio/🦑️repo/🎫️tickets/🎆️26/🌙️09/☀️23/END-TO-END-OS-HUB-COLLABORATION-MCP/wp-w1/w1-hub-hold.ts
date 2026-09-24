/** 🌎️ W1: holds one development hub on a published trusted-catalog data root through `startLocalHub` (the
 * local-bootstrap pipe handshake a development hub requires before it binds), with credential sign-in enabled,
 * and prints the `/readyz` body once admitted. Same shape as `wp-tc5/tc5-hub-hold.ts`.
 * usage: OS_HUB_CREDENTIAL_SIGN_IN=1 bun w1-hub-hold.ts <port> <absoluteDataDir> <absoluteBinaryPath> */
import { existsSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const { startLocalHub, waitForReadiness } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const [port, dataRoot, binaryPath] = [Number(process.argv[2]), process.argv[3]!, process.argv[4]!];
if (!existsSync(binaryPath) || !existsSync(join(dataRoot, "trusted-catalog"))) throw new Error(`missing binary or trusted-catalog: ${binaryPath} ${dataRoot}`);
const run = await startLocalHub(repoRoot, join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust"), [{ profileId: "developer", subject: "local-developer-w1", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }], { port, dataDir: dataRoot, binaryPath, capture: false });
const readiness = await waitForReadiness(run, true, 1_800_000);
console.log(`HOLD origin=http://127.0.0.1:${port} pid=${run.child.pid} dataRoot=${dataRoot} status=${readiness.status}`);
console.log(`READINESS ${JSON.stringify(readiness)}`);
await new Promise(() => undefined);
