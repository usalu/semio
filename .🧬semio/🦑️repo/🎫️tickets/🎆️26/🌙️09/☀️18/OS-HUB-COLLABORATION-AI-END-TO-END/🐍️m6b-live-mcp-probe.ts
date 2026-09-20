/** 🧪️ M6b live MCP phase — spawns the real `semio-os-mcp stdio --hub <origin> --space <id>
 * --credential-file <path>` against the running hub, speaks the JSON-RPC handshake over its stdio,
 * calls `context_resolve`, and reports what the agent principal looks like from inside the client.
 * Then, after the delegation is revoked, the same spawn must fail to acquire a session.
 *
 * `--use <binary> <origin> <space> <credentialPath>` — M6 §8 steps 4 and 5.
 * `--refused <binary> <origin> <space> <credentialPath>` — step 7's client half.
 *
 * Nothing here reaches into the hub: the only inputs are the binary, the origin, the space and a
 * credential file at mode 0600. Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M6b.
 */
import { spawn } from "node:child_process";

interface Outcome {
  readonly code: number | null;
  readonly stderr: string;
  readonly responses: readonly any[];
}

async function drive(binary: string, args: readonly string[], requests: readonly unknown[], settleMs: number): Promise<Outcome> {
  const child = spawn(binary, [...args], { stdio: ["pipe", "pipe", "pipe"] });
  let stdout = "";
  let stderr = "";
  child.stdout.on("data", (chunk: Buffer) => { stdout += chunk.toString("utf8"); });
  child.stderr.on("data", (chunk: Buffer) => { stderr += chunk.toString("utf8"); });
  const exited = new Promise<number | null>((resolve) => child.on("exit", (code) => resolve(code)));
  for (const request of requests) {
    child.stdin.write(`${JSON.stringify(request)}\n`);
    await new Promise((resolve) => setTimeout(resolve, settleMs));
  }
  child.stdin.end();
  const code = await Promise.race([exited, new Promise<number | null>((resolve) => setTimeout(() => { child.kill("SIGTERM"); resolve(null); }, settleMs * 4))]);
  const responses = stdout
    .split("\n")
    .filter((line) => line.trim().length > 0)
    .map((line) => {
      try {
        return JSON.parse(line);
      } catch {
        return { raw: line };
      }
    });
  return { code, stderr, responses };
}

const [phase, binary, origin, space, credentialPath] = process.argv.slice(2);
if (binary === undefined || origin === undefined || space === undefined || credentialPath === undefined) {
  throw new Error("usage: probe --use|--refused <binary> <origin> <space> <credentialPath>");
}

const args = ["stdio", "--hub", origin, "--space", space, "--credential-file", credentialPath, "--no-bridge"];
const handshake = [
  { jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m6b-live", version: "1" } } },
  { jsonrpc: "2.0", method: "notifications/initialized" },
  { jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "context_resolve", arguments: {} } },
];

const outcome = await drive(binary, args, handshake, 2500);
console.log(`--- ${phase} — exit ${outcome.code} ---`);
console.log(`STDERR ${outcome.stderr.split("\n").filter((line) => line.trim().length > 0).slice(0, 12).join(" | ")}`);
for (const response of outcome.responses.slice(0, 6)) {
  const text = JSON.stringify(response);
  console.log(`RPC ${text.length > 1400 ? `${text.slice(0, 1400)}…` : text}`);
}

const contextText = outcome.responses
  .filter((response) => response?.id === 2)
  .flatMap((response) => (Array.isArray(response?.result?.content) ? response.result.content : []))
  .map((entry: any) => (typeof entry?.text === "string" ? entry.text : ""))
  .join("\n");
if (contextText.length > 0) console.log(`CONTEXT ${contextText.length > 2000 ? `${contextText.slice(0, 2000)}…` : contextText}`);
