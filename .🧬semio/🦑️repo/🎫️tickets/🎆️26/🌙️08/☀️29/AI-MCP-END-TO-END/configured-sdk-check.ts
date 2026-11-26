import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js";

const { command, args } = JSON.parse(readFileSync(".mcp.json", "utf8")).mcpServers.semio;
const env = { ...process.env };
if (!process.argv.includes("--explicit-native-binary")) delete env.SEMIO_OS_MCP_BIN;
const transport = new StdioClientTransport({ command, args, env, stderr: "inherit" });
const client = new Client({ name: "semio-configured-client-check", version: "1" });
const errors: Error[] = [];
client.onerror = error => errors.push(error);
try {
  await client.connect(transport);
  assert.equal(client.getServerVersion()?.name, "semio");
  const deadline = Date.now() + 120000;
  let tools = await client.listTools();
  while (!tools.tools.length && Date.now() < deadline) {
    await Bun.sleep(250);
    tools = await client.listTools();
  }
  assert.ok(tools.tools.some(tool => tool.name === "context_resolve"));
  const prompts = await client.listPrompts();
  const resources = await client.listResources();
  assert.ok(prompts.prompts.length);
  assert.ok(resources.resources.some(resource => resource.uri === "semio://workspace"));
  const workspace = await client.readResource({ uri: "semio://workspace" });
  assert.ok(workspace.contents.length);
  for (const locale of ["en", "de"]) {
    const context = await client.callTool({ name: "context_resolve", arguments: { locale } });
    assert.ok(!context.isError);
    assert.equal(context.structuredContent?.locale, locale);
  }
  const refusal = await client.callTool({ name: "capabilities_search", arguments: { query: "note", limit: 0 } });
  assert.equal(refusal.isError, true);
  assert.equal(refusal.structuredContent?.code, "INPUT_INVALID");
  assert.equal(errors.length, 0);
  console.log(JSON.stringify({ tools: tools.tools.length, prompts: prompts.prompts.length, resources: resources.resources.length, locales: ["en", "de"], rejectedInvalidInput: true }));
} finally {
  await client.close();
}
