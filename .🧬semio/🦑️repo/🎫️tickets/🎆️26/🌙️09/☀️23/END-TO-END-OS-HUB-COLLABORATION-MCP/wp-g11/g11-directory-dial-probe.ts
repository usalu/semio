#!/usr/bin/env bun
/** 🔭️ G11: time an agent's directory dial legs against a hub (delegation → agent session → socket grant → WS upgrade),
 * printing statuses and latencies only (never a token). usage: bun g11-directory-dial-probe.ts <credential-file> */
import { readFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
const credential = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as { hubOrigin: string; spaceId: string; audience: string; token: string };
const hub = credential.hubOrigin;
const timed = async <T>(label: string, run: () => Promise<T>): Promise<T> => {
  const started = performance.now();
  const value = await run();
  console.log(`${label} ${(performance.now() - started).toFixed(1)} ms`);
  return value;
};
const session = await timed("agent-session", async () => {
  const response = await fetch(`${hub}/auth/agent-sessions`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${credential.token}` }, body: JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: credential.audience, agentInstanceId: `g11probe${randomBytes(6).toString("hex")}` }) });
  const body = await response.json().catch(() => ({}));
  console.log(`  status ${response.status} keys ${Object.keys(body).join(",")} error ${body.error ?? ""}`);
  return String(body.token ?? "");
});
const grant = await timed("socket-grant", async () => {
  const response = await fetch(`${hub}/directory/socket-grants`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${session}` }, body: "{}" });
  const text = await response.text();
  let body: any = {};
  try { body = JSON.parse(text); } catch {}
  console.log(`  status ${response.status} keys ${Object.keys(body).join(",")} body ${response.status >= 300 ? text.slice(0, 300) : ""}`);
  return body;
});
if (grant.grant) {
  await timed("ws-upgrade", () => new Promise<void>((resolve) => {
    const socket = new WebSocket(`${hub.replace(/^http/u, "ws")}/directory/socket?since=0`, [String(grant.protocol), String(grant.grant)]);
    socket.onopen = () => { console.log("  open"); socket.close(); resolve(); };
    socket.onerror = (event) => { console.log(`  error ${String((event as any).message ?? "")}`); resolve(); };
    socket.onclose = (event) => { console.log(`  close ${event.code} ${event.reason}`); resolve(); };
  }));
}
