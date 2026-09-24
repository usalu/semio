/** 🔭️ H7 live probe: boots os-hub (no trusted catalog) at SEMIO_TRACE_LEVEL=info, drives a directory command,
 * a directory socket open/close and a refused document socket, shuts it down, then validates every
 * trace line with Ajv against the trace record schema. Without a catalog the hub stays `not-ready` on
 * `artifactAuthority` only; the directory lane is live, which is all this probe drives. Usage: bun h7-trace-probe.ts <os-hub binary> <port> */
import Ajv from "ajv";
import { randomBytes } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const [binaryPath, portText] = process.argv.slice(2);
const execution = await import(join(repoRoot, "🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts"));
const { issueLocalCredential } = await import(join(repoRoot, "🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts"));
const trace = join(repoRoot, "🧰️framework/🔨️modules/⏱️trace");
const validate = new Ajv({ allErrors: true, strict: true }).compile(JSON.parse(readFileSync(join(trace, "📝️record/🧬️schema/🔣️.json"), "utf8")));
const vocabulary = JSON.parse(readFileSync(join(trace, "🧫️fixtures/🛰️span-vocabulary/🔣️.json"), "utf8")) as { events: string[] };

process.env.SEMIO_TRACE_LEVEL = "info";
delete process.env.SEMIO_TRACE_SINK;
const dataDir = mkdtempSync(join(tmpdir(), "h7-trace-probe-"));
const profile = { profileId: "h7", subject: "h7-author", displayName: "H7 Author", allowedClientClasses: ["native"] };
const run = await execution.startLocalHub(repoRoot, join(repoRoot, "🌎️hub/📦️packages/🦀️rust"), [profile], { port: Number(portText), dataDir, binaryPath, capture: true });
const origin = `http://127.0.0.1:${run.port}`;
try {
  for (let deadline = Date.now() + 60_000; ; await Bun.sleep(200)) {
    const body = await fetch(`${origin}/readyz`).then((response) => response.json()).catch(() => undefined) as Record<string, any> | undefined;
    if (body?.directory?.ready === true && body?.authentication?.bootstrapReady === true) {
      console.log(`readiness: ${body.status} blockedBy=${JSON.stringify(body.blockedBy)}`);
      break;
    }
    if (Date.now() > deadline) throw new Error("hub directory never became ready");
  }
  const author = await issueLocalCredential(run, profile.profileId, "native", 2);
  const created = await fetch(`${origin}/directory/commands`, {
    method: "POST",
    headers: { authorization: `Bearer ${author.capability}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.directory.command-request.v1", requestId: randomBytes(16).toString("hex"), command: { kind: "create-space", name: "H7 Trace Probe", spaceKind: "studio", visibility: "private" } }),
  });
  console.log(`directory command create-space -> ${created.status}`);
  const grant = await fetch(`${origin}/directory/socket-grants`, { method: "POST", headers: { authorization: `Bearer ${author.capability}`, "content-type": "application/json" }, body: "{}" });
  const receipt = (await grant.json()) as { protocol: string; grant: string };
  console.log(`directory socket grant -> ${grant.status} protocol=${receipt.protocol}`);
  const socket = new WebSocket(`ws://127.0.0.1:${run.port}/directory/socket/v1?since=0`, [receipt.protocol, receipt.grant]);
  await new Promise<void>((resolve, reject) => {
    socket.onopen = () => resolve();
    socket.onerror = (event) => reject(new Error(`directory socket error ${String(event)}`));
  });
  console.log("directory socket open");
  await Bun.sleep(300);
  const closed = new Promise<void>((resolve) => (socket.onclose = () => resolve()));
  socket.close(1000, "h7 probe done");
  await closed;
  console.log("directory socket closed");
  const refused = new WebSocket(`ws://127.0.0.1:${run.port}/scopes/${encodeURIComponent("space-x/doc-y")}/document/ws?surface=s`, ["semio.session.v1", "not-a-session"]);
  await new Promise<void>((resolve) => {
    refused.onerror = () => resolve();
    refused.onclose = () => resolve();
  });
  console.log("document socket with a bogus credential refused");
  await Bun.sleep(500);
} finally {
  run.child.kill("SIGTERM");
  await execution.waitForChildExit(run.child).catch(() => undefined);
  await execution.finishLocalHub(run).catch(() => undefined);
}
const lines = (run.output() as string).split("\n").filter((line) => line.startsWith('{"level":'));
let invalid = 0;
for (const line of lines) {
  const record = JSON.parse(line);
  const ok = validate(record) && vocabulary.events.includes(record.event);
  if (!ok) invalid++;
  console.log(`${ok ? "VALID  " : "INVALID"} ${line}${ok ? "" : ` ${JSON.stringify(validate.errors)}`}`);
}
rmSync(dataDir, { recursive: true, force: true });
const socketPairs = lines.map((line) => JSON.parse(line)).filter((record) => record.event === "server.directory.socket");
const opened = socketPairs.find((record) => record.outcome === "ok" && record.detail === "upgrade");
const closedRecord = socketPairs.find((record) => record.outcome === "cancelled" && record.detail === "closed" && record.requestId === opened?.requestId);
console.log(`trace lines=${lines.length} invalid=${invalid} directorySocketPairSharesRequestId=${Boolean(opened && closedRecord)}`);
process.exit(invalid === 0 && opened && closedRecord ? 0 : 1);
