import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";
import { mkdtemp } from "node:fs/promises";
import { join } from "node:path";
import assert from "node:assert/strict";

const [binary, directory] = process.argv.slice(2);
const folder = await mkdtemp(join(directory, "ui-"));
const transport = new StdioClientTransport({ command: binary, args: ["stdio", "--host-driver-policy", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/🏛️host-driver-policy.json", "--operation-policy", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🎚️config/💡️host-operation-policy.json", "--folder", folder, "--scopes", "ui.control"], env: { ...process.env, S_AGENT_BRIDGE_DIR: folder } });
const client = new Client({ name: "semio-ui-verification", version: "1" });
try {
  await client.connect(transport);
  const list = await client.listTools();
  for (const name of ["ui_focus", "ui_reveal"]) {
    console.log(name, JSON.stringify(list.tools.find(row => row.name === name)?.inputSchema));
    const result = await client.callTool({ name, arguments: { windowId: "w1", anchor: "left", path: [] } });
    console.log(name, JSON.stringify(result.structuredContent));
    assert.equal(result.structuredContent?.code, "INPUT_INVALID");
    assert.equal(result.structuredContent?.retryable, false);
    const valid = await client.callTool({ name, arguments: name === "ui_focus" ? { windowId: "w1" } : { anchor: "left", path: [] } });
    console.log(name, JSON.stringify(valid.structuredContent));
    assert.equal(valid.structuredContent?.code, "PLUGIN_UNAVAILABLE");
    assert.equal(valid.structuredContent?.retryable, true);
  }
  await client.ping();
} finally { await client.close(); }
