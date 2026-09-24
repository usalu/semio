#!/usr/bin/env bun
/** Throwaway stdio JSON-RPC probe for semio-os-mcp (GJ2 / audit ticket). */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ensureMcpBinary, spawnRawMcp } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const REPO = join(import.meta.dir, "../../../../../../../..");
const OUT = join(import.meta.dir, "../wp-gj2/generated/mcp-probe");
const TIMEOUT_MS = 120_000;
const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write";

type Line = { ts: string; label: string; payload: unknown };

const log: Line[] = [];
function record(label: string, payload: unknown): void {
  log.push({ ts: new Date().toISOString(), label, payload });
  console.log(`[probe] ${label}`);
}

async function main(): Promise<void> {
  mkdirSync(OUT, { recursive: true });
  const bin = ensureMcpBinary(REPO);
  const tmp = join(OUT, "probe-workspace");
  mkdirSync(tmp, { recursive: true });

  const args = ["stdio", "--folder", tmp, "--no-bridge", "--scopes", SCOPES];
  record("spawn", { bin, args });

  const mcp = spawnRawMcp(bin, args);

  try {
    const init = await mcp.request(
      "initialize",
      {
        protocolVersion: "2025-06-18",
        capabilities: { roots: { listChanged: true } },
        clientInfo: { name: "mcp-audit-probe", version: "1" },
      },
      TIMEOUT_MS,
    );
    record("response:initialize", init);

    mcp.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} }));
    record("notify", "notifications/initialized");

    for (const method of ["tools/list", "resources/list", "prompts/list", "ping"] as const) {
      const r = await mcp.request(method, {}, TIMEOUT_MS);
      record(`response:${method}`, r);
    }

    const ctx = await mcp.request("tools/call", { name: "context_resolve", arguments: {} }, TIMEOUT_MS);
    record("response:tools/call context_resolve", ctx);

    const ws = await mcp.request("resources/read", { uri: "semio://workspace" }, TIMEOUT_MS);
    record("response:resources/read semio://workspace", ws);

    const search = await mcp.request("tools/call", { name: "capabilities_search", arguments: { query: "read workspace" } }, TIMEOUT_MS);
    record("response:tools/call capabilities_search", search);

    const inferences = await mcp.request("tools/call", { name: "inference_list", arguments: {} }, TIMEOUT_MS);
    record("response:tools/call inference_list", inferences);
  } finally {
    await mcp.close();
    record("stderr", mcp.stderrText().slice(0, 4000));
    writeFileSync(join(OUT, "probe-transcript.json"), JSON.stringify(log, null, 2));
    writeFileSync(join(OUT, "probe-stderr.txt"), mcp.stderrText());
  }
}

await main().catch((error) => {
  record("fatal", String(error));
  writeFileSync(join(OUT, "probe-transcript.json"), JSON.stringify(log, null, 2));
  process.exit(1);
});
