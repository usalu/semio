import assert from "node:assert/strict";
import {mkdtemp,readFile,rm} from "node:fs/promises";
import {join,resolve} from "node:path";
import {Client} from "@modelcontextprotocol/sdk/client/index.js";
import {StdioClientTransport} from "@modelcontextprotocol/sdk/client/stdio.js";
import {takeMcpPluginCatalog,withMcpHostDriverPolicy} from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const executable = process.argv[2];
assert(executable, "Native MCP executable required");
const cwd = process.cwd();
const config = JSON.parse(await readFile(join(cwd, ".mcp.json"), "utf8"));
const {args, env} = takeMcpPluginCatalog(config.mcpServers.semio.args.slice(4), cwd);
const folder = await mkdtemp(join(import.meta.dirname, "🗑️generated", "native-sdk-"));
args[args.indexOf("--folder") + 1] = folder;
const denyWrite = process.argv.includes("--deny-write");
if (denyWrite) args[args.indexOf("--scopes") + 1] = "workspace.read";
const client = new Client({name:"semio-native-sdk-smoke",version:"1"});
try {
  await client.connect(new StdioClientTransport({command:resolve(executable),args:withMcpHostDriverPolicy(["stdio", ...args, "--no-bridge"]),cwd,env,stderr:"inherit"}));
  assert.equal(client.getServerVersion()?.name, "semio");
  const {tools} = await client.listTools();
  const {prompts} = await client.listPrompts();
  const {resources} = await client.listResources();
  assert(tools.some(tool => tool.name === "context_resolve"));
  assert(prompts.length);
  assert(resources.some(resource => resource.uri === "semio://workspace"));
  const context = await client.callTool({name:"context_resolve",arguments:{locale:"en"}});
  assert.equal(context.structuredContent?.locale, "en");
  if (denyWrite) {
    const artifactId = "sdk-denied-probe";
    const created = await client.callTool({name:"artifact_create",arguments:{artifactId,kind:"os.agent.probe/v1",initial:{note:"scope denial witness"}}});
    assert.equal(created.isError, true);
    assert.equal(created.structuredContent?.code, "PERMISSION_DENIED");
    const snapshot = await client.callTool({name:"artifact_snapshot",arguments:{artifactId}});
    assert.equal(snapshot.isError, true);
    assert.equal(snapshot.structuredContent?.code, "NOT_FOUND");
  }
  await client.ping();
  console.log(JSON.stringify({server:client.getServerVersion(),tools:tools.length,prompts:prompts.length,resources:resources.length,context:"en",ping:true,...(denyWrite ? {writeDenied:true,noArtifact:true} : {})}));
} finally {
  await client.close();
  await rm(folder, {recursive:true,force:true});
}
