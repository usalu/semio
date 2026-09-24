
import { spawn } from "node:child_process";
const [origin, spaceId, credentialPath, docId] = process.argv.slice(2);
const binary = process.env.M10_MCP_BINARY!;
const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
let buffer = "";
const pending = new Map<number, (m: any) => void>();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let i: number;
  while ((i = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, i).trim();
    buffer = buffer.slice(i + 1);
    if (!line) continue;
    let message: any;
    try { message = JSON.parse(line); } catch { continue; }
    const resolve = pending.get(message.id);
    if (resolve) { pending.delete(message.id); resolve(message); }
  }
});
let nextId = 1;
const call = (method: string, params: unknown) => new Promise<any>((resolve) => {
  const id = nextId++;
  pending.set(id, resolve);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10b-cap", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
await call("tools/call", { name: "artifact_open", arguments: { artifactId: docId } });
const searched = await call("tools/call", { name: "capabilities_search", arguments: { query: "add", kind: "mutation" } });
console.log(JSON.stringify(searched?.result?.structuredContent, null, 2));
child.kill();
