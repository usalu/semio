/** ⏱️ W1: boots one captured development hub on a data root and prints, every second, the byte count of its
 * output and the /readyz answer — measures where a candidate hub stays silent. usage: bun w1-boot-probe.ts <port> <dataRoot> <binary> */
import { join } from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const { startLocalHub, finishLocalHub } = await import(join(repoRoot, "🌎️hub", "🚀️local-bootstrap", "🏃️execution", "🟦️.ts"));
const [port, dataDir, binaryPath] = [Number(process.argv[2]), process.argv[3]!, process.argv[4]!];
const started = Date.now();
const run = await startLocalHub(repoRoot, join(repoRoot, "🌎️hub", "📦️packages", "🦀️rust"), [{ profileId: "probe", subject: "w1-probe", displayName: "Probe", allowedClientClasses: ["native"] }], { port, dataDir, binaryPath, capture: true });
let last = 0;
for (let second = 0; second < 560; second += 1) {
  let answer = "no-answer";
  try { const response = await fetch(`http://127.0.0.1:${port}/readyz`, { signal: AbortSignal.timeout(900) }); const body = await response.json() as any; answer = `http=${response.status} status=${body.status} authority=${body.artifactAuthority?.ready}`; if (response.status === 200) { console.log(`${((Date.now() - started) / 1000).toFixed(1)}s READY ${answer}`); break; } } catch {}
  const out = run.output();
  if (out.length !== last) { console.log(`${((Date.now() - started) / 1000).toFixed(1)}s bytes=${out.length} ${answer} :: ${out.slice(last).replace(/\s+/g, " ").slice(0, 300)}`); last = out.length; }
  await new Promise((resolve) => setTimeout(resolve, 1000));
}
await finishLocalHub(run);
