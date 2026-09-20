#!/usr/bin/env bun
/** 📤️ WR3 — drives `artifact_create` + `artifact_export` against ONE named plugin kind over the real
 * `.mcp.json` `semio` server, so the export leg can be measured without the shared `client-e2e`
 * journey's own capability pick (which a peer's `describe` sweep moved from `draw` to `animate`
 * mid-session). Same client class the gate uses — never a reconstruction of it. */
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { McpClientSession, mcpServerEntries } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const kind = process.argv[2] ?? "s.draw.drawing";
const folder = mkdtempSync(join(tmpdir(), "semio-wr3-export-"));
const entry = mcpServerEntries(repoRoot).semio;
if (!entry) throw new Error(".mcp.json declares no `semio` server");
const session = new McpClientSession(entry, ["--folder", folder], repoRoot);
try {
  await session.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wr3-export-probe", version: "1" } });
  session.notify("notifications/initialized", {});
  const resolvedAtStart = await session.call("context_resolve", {}, 60_000);
  console.log(`channel@t0 ${String(resolvedAtStart.structuredContent?.channel ?? JSON.stringify(resolvedAtStart.structuredContent).slice(0, 160))}`);
  const delayMs = Number(process.argv[3] ?? "0");
  if (delayMs > 0) {
    await new Promise((resolve) => setTimeout(resolve, delayMs));
    const resolvedAfter = await session.call("context_resolve", {}, 60_000);
    console.log(`channel@t${delayMs}ms ${String(resolvedAfter.structuredContent?.channel ?? JSON.stringify(resolvedAfter.structuredContent).slice(0, 160))}`);
  }
  const artifactId = `wr3-export-${Date.now().toString(36)}`;
  const created = await session.call("artifact_create", { artifactId, kind }, 300_000);
  console.log(`create kind=${kind} isError=${created.isError === true} ${JSON.stringify(created.structuredContent).slice(0, 300)}`);
  if (created.isError === true) process.exit(1);
  const exported = await session.call("artifact_export", { artifactId }, 300_000);
  const base64 = String(exported.structuredContent?.contentBase64 ?? "");
  console.log(`export isError=${exported.isError === true} port=${exported.structuredContent?.format} base64Bytes=${base64.length} availablePorts=${JSON.stringify(exported.structuredContent?.availablePorts ?? [])}`);
  if (exported.isError === true) console.log(`export refusal ${JSON.stringify(exported.structuredContent).slice(0, 400)}`);
  else {
    const bytes = Buffer.from(base64, "base64");
    const text = bytes.toString("utf8");
    console.log(`export bytes=${bytes.length} svgMarker=${text.includes("<svg") ? "yes" : "no"} head=${JSON.stringify(text.slice(0, 220))}`);
  }
  process.exit(exported.isError === true || base64.length === 0 ? 1 : 0);
} finally {
  session.stop();
}
