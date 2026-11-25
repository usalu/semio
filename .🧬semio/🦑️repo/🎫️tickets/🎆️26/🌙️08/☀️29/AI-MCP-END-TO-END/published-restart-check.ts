import assert from "node:assert/strict";
import { randomUUID } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";

const entry = JSON.parse(readFileSync(".mcp.json", "utf8")).mcpServers.semio;
const option = (key: string): string => entry.args[entry.args.indexOf(key) + 1];
const workspace = mkdtempSync(join(import.meta.dir, "🗑️generated", "restart-"));
const id = randomUUID();
const session = async (create: boolean) => {
  const client = new Client({ name: "semio-restart-check", version: "1" });
  const transport = new StdioClientTransport({
    command: resolve(process.argv[2]),
    args: ["stdio", "--folder", workspace, "--no-bridge", "--scopes", option("--scopes"), "--operation-policy", option("--operation-policy"), "--host-driver-policy", join(dirname(option("--operation-policy")), "🏛️host-driver-policy.json")],
    env: { ...process.env, SEMIO_PLUGIN_CATALOG: resolve(option("--catalog")) },
    stderr: "ignore",
  });
  const call = async (name: string, args: Record<string, unknown>) => {
    const result = await client.callTool({name, arguments: args});
    assert.ok(!result.isError, JSON.stringify(result.structuredContent));
    assert.equal(result.structuredContent?.artifactId, id);
    return result.structuredContent!;
  };
  try {
    await client.connect(transport);
    if (create) await call("artifact_create", {artifactId: id, kind: "os.agent.probe/v1", initial: { note: id }});
    else await call("artifact_open", {artifactId: id});
    const snapshot = await call("artifact_snapshot", {artifactId: id});
    assert.ok(Number(snapshot.sprBytes) > 0);
    assert.ok(Number(snapshot.packBytes) > 0);
    return snapshot.untrusted.content;
  } finally {
    await client.close();
  }
};
try {
  const committed = await session(true);
  const reopened = await session(false);
  assert.deepEqual(reopened, committed);
  assert.ok(typeof reopened.packBase64 === "string" && typeof reopened.sprBase64 === "string");
  console.log("Independent server process reopened the same artifact and returned identical pack and journal bytes.");
} finally {
  rmSync(workspace, { recursive: true, force: true });
}
