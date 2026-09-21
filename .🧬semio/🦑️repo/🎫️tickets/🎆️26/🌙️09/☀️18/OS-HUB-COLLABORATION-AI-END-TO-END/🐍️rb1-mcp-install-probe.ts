#!/usr/bin/env bun
/** 🧭️ RB1 §2 — does the RELEASE `semio-os-mcp` actually serve an end user who copies the documented
 * Claude Desktop / Claude Code block?
 *
 * Not a unit test and not a repo-internal `.mcp.json` run: this reads a config file written in the
 * exact shape the READMEs hand a user — absolute path to `dist/build-release/semio-os-mcp`, `stdio`,
 * `--folder <their work>`, `--scopes <the documented string>` — spawns whatever that config names,
 * and drives the handshake a host performs on startup:
 *
 *   1. `initialize` → protocolVersion, serverInfo, capabilities
 *   2. `notifications/initialized`
 *   3. `tools/list` → the tool names a client will show
 *   4. one policy-gated tool call, to prove the granted scopes are the ones the user thinks they are
 *
 * Usage: bun 🐍️rb1-mcp-install-probe.ts <config.json> <serverName> [--label <text>]
 * Exits non-zero on a red row.
 */
import { readFileSync } from "node:fs";

type ServerEntry = { readonly command: string; readonly args?: readonly string[]; readonly env?: Record<string, string> };
type Envelope = Record<string, any>;

const [configPath, serverName, ...rest] = process.argv.slice(2);
if (!configPath || !serverName) throw new Error("usage: 🐍️rb1-mcp-install-probe.ts <config.json> <serverName> [--label <text>]");
const label = rest[0] === "--label" ? rest[1]! : serverName;

const config = JSON.parse(readFileSync(configPath, "utf8")) as { mcpServers: Record<string, ServerEntry> };
const entry = config.mcpServers[serverName];
if (!entry) throw new Error(`${configPath} declares no \`${serverName}\` server`);

let red = 0;
const row = (ok: boolean, name: string, evidence: string): void => {
  if (!ok) red += 1;
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}  ${evidence}`);
};

console.log(`=== ${label} ===`);
console.log(`config   ${configPath} :: mcpServers.${serverName}`);
console.log(`command  ${entry.command}`);
console.log(`args     ${JSON.stringify(entry.args ?? [])}`);

const child = Bun.spawn([entry.command, ...(entry.args ?? [])], {
  stdin: "pipe",
  stdout: "pipe",
  stderr: "pipe",
  env: { ...process.env, ...(entry.env ?? {}) },
});

const pending = new Map<number, (envelope: Envelope) => void>();
const diagnostics: string[] = [];
let buffer = "";
let nextId = 1;

void (async () => {
  const decoder = new TextDecoder();
  for await (const chunk of child.stdout as ReadableStream<Uint8Array>) {
    buffer += decoder.decode(chunk, { stream: true });
    let cut = buffer.indexOf("\n");
    while (cut >= 0) {
      const line = buffer.slice(0, cut).trim();
      buffer = buffer.slice(cut + 1);
      if (line) {
        try {
          const envelope = JSON.parse(line) as Envelope;
          const resolve = typeof envelope.id === "number" ? pending.get(envelope.id) : undefined;
          if (resolve) {
            pending.delete(envelope.id);
            resolve(envelope);
          }
        } catch {
          diagnostics.push(`unparsable stdout: ${line.slice(0, 200)}`);
        }
      }
      cut = buffer.indexOf("\n");
    }
  }
})();

void (async () => {
  const decoder = new TextDecoder();
  for await (const chunk of child.stderr as ReadableStream<Uint8Array>) diagnostics.push(decoder.decode(chunk, { stream: true }));
})();

const send = (envelope: Envelope): void => {
  child.stdin.write(`${JSON.stringify(envelope)}\n`);
  child.stdin.flush();
};

const call = async (method: string, params: Envelope, budgetMs = 180_000): Promise<Envelope> => {
  const id = nextId++;
  const answered = new Promise<Envelope>((resolve, reject) => {
    pending.set(id, resolve);
    setTimeout(() => {
      if (pending.delete(id)) reject(new Error(`${method} did not answer within ${budgetMs} ms`));
    }, budgetMs).unref?.();
  });
  send({ jsonrpc: "2.0", id, method, params });
  return answered;
};

try {
  const initialized = await call("initialize", {
    protocolVersion: "2025-11-25",
    capabilities: {},
    clientInfo: { name: "rb1-install-probe", version: "1.0.0" },
  });
  const result = initialized.result ?? {};
  row(typeof result.protocolVersion === "string", "initialize answers", `protocolVersion=${result.protocolVersion} serverInfo=${JSON.stringify(result.serverInfo ?? {})}`);
  row(!!result.capabilities?.tools, "declares a tools capability", JSON.stringify(Object.keys(result.capabilities ?? {})));

  send({ jsonrpc: "2.0", method: "notifications/initialized", params: {} });

  const listed = await call("tools/list", {});
  const tools: Envelope[] = listed.result?.tools ?? [];
  row(tools.length > 0, "tools/list answers", `${tools.length} tools`);
  console.log(`      tools: ${tools.map((tool) => tool.name).join(", ")}`);

  const capabilities = await call("tools/call", { name: "capabilities_describe", arguments: {} }, 240_000);
  const capabilityText = JSON.stringify(capabilities.result ?? capabilities.error ?? {});
  row(!capabilities.error, "capabilities_describe (workspace.read)", `${capabilities.result?.isError ? "isError" : "ok"} ${capabilityText.slice(0, 220)}`);

  for (const probe of [
    { tool: "inference_list", args: {}, gate: "artifacts.read (workspace.read grants it)" },
    { tool: "artifact_create", args: { kind: "s.note.note" }, gate: "artifacts.write + jobs.spawn (artifact.write / inference.execute grant them)" },
    { tool: "inference_run", args: { inferenceId: "probe", artifactId: "probe" }, gate: "jobs.spawn (inference.execute grants it)" },
  ]) {
    const answered = await call("tools/call", { name: probe.tool, arguments: probe.args }, 240_000);
    const text = JSON.stringify(answered.result ?? answered.error ?? {});
    const denied = text.includes("PERMISSION_DENIED");
    row(!denied, `${probe.tool} is not PERMISSION_DENIED`, `${denied ? "PERMISSION_DENIED" : "reached the tool"} — gate: ${probe.gate}${denied ? "" : ` :: ${text.slice(0, 160)}`}`);
  }
} catch (error) {
  row(false, "probe completed", String(error));
} finally {
  child.kill();
  const noise = diagnostics.join("").split("\n").filter((line) => line.trim()).slice(0, 12);
  if (noise.length) console.log(`      stderr: ${noise.join(" | ").slice(0, 900)}`);
  console.log(`=== ${label}: ${red === 0 ? "all rows green" : `${red} RED row(s)`} ===\n`);
  process.exit(red === 0 ? 0 : 1);
}
