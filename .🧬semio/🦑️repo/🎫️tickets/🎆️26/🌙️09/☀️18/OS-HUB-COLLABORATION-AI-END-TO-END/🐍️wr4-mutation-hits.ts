/** 🔎️ WR4: which mutation capabilities `client-e2e`'s own discovery offers, in its own order —
 * the roster the gate walks when hit 0 cannot be called. Read-only; opens no plugin. */
import { mcpServerEntries, McpClientSession } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio";
const entry = mcpServerEntries(repoRoot).semio!;
const session = new McpClientSession(entry, ["--folder", repoRoot, "--no-bridge"], repoRoot);
await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wr4-hits", version: "1" } });
session.notify("notifications/initialized", {});
const hits = ((await session.call("capabilities_search", { query: "set", kind: ["mutation"] })).structuredContent?.results ?? []) as Array<Record<string, any>>;
for (const [index, hit] of hits.entries()) console.log(`${index} ${hit.pluginId} ${hit.capabilityId}`);
session.stop();
