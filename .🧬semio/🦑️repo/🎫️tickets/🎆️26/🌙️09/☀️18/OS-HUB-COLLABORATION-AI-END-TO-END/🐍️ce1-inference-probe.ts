/** 💡️ CE1 — which declared inference can `client-e2e` pin to? Spawns the real `semio` server the
 * way `.mcp.json` does (headless, `--no-bridge`), lists the declared roster and runs ONE named
 * service with a wall clock, so the journey's own 240 s budget is never spent discovering that a
 * candidate's component does not answer. `🀄️wfc`'s `s.wfc.bitmap.solve` did not return within
 * 900 s (2026-09-20 23:08) and `🌍️gis`'s guest refuses to instantiate at all, so the pin has to be
 * measured rather than guessed.
 *
 * usage: bun 🐍️ce1-inference-probe.ts <artifactKind> <inferenceSchema> <pluginId> [budgetMs]
 */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { McpClientSession, mcpServerEntries } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const [artifactKind, inferenceSchema, pluginId, budget] = process.argv.slice(2);
if (!artifactKind || !inferenceSchema || !pluginId) throw new Error("usage: <artifactKind> <inferenceSchema> <pluginId> [budgetMs]");
const budgetMs = Number(budget ?? 600_000);

const folder = mkdtempSync(join(tmpdir(), "ce1-inference-probe-"));
const session = new McpClientSession(mcpServerEntries(repoRoot).semio!, ["--folder", folder, "--no-bridge"], repoRoot);
try {
  await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "ce1-inference-probe", title: "CE1 inference probe", version: "1" } });
  session.notify("notifications/initialized", {});
  const listed = await session.call("inference_list", {});
  const declared = (listed.structuredContent?.declared ?? []) as Array<Record<string, any>>;
  console.log(`declared: ${declared.map((row) => `${row.artifactKind}/${row.inferenceSchema}@${row.contributor || row.owner}`).join(", ")}`);
  const started = Date.now();
  const envelope = await session.request("tools/call", { name: "inference_run", arguments: { artifactKind, inferenceSchema, pluginId, cancellationId: "ce1-probe-cancel" }, _meta: { progressToken: "ce1-probe" } }, budgetMs);
  const reply = (envelope.error ? { isError: true, structuredContent: envelope.error } : (envelope.result ?? {})) as { isError?: boolean; structuredContent?: Record<string, any> };
  const progress = session.serverNotifications().filter((row) => row.method === "notifications/progress");
  console.log(`inference_run ${artifactKind}/${inferenceSchema} by ${pluginId}: ${Math.round((Date.now() - started) / 1000)} s, isError=${reply.isError === true}, progressRows=${progress.length}`);
  console.log(JSON.stringify(reply.structuredContent).slice(0, 400));
} finally {
  session.stop();
}
