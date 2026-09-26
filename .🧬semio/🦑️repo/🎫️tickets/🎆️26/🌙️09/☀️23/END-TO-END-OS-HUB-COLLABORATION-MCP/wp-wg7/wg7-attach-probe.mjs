/** 🔬️ WG7 — one browser signs in, attaches the hub document, and records every document-socket frame's PAYLOAD (base64) for
 * 45 s plus the shell's hub projection, so the exchange can be decoded with the product's own frame codecs.
 * Usage: SEMIO_PROBE_URL=… WG7_HUB=… WG7_SPACE=… WG7_DOCUMENT=… WG7_USER=user1@semio.dev bun wg7-attach-probe.mjs <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const tag = process.argv[2] ?? "p1";
const SHELL = process.env.SEMIO_PROBE_URL, HUB = process.env.WG7_HUB, SPACE = process.env.WG7_SPACE, DOCUMENT = process.env.WG7_DOCUMENT;
const USER = process.env.WG7_USER ?? "user1@semio.dev";
const PASSWORDS = { "user1@semio.dev": "gm1-local-dev-pass-1", "user2@semio.dev": "gm1-local-dev-pass-2" };
const MIRROR = "#semio-wgpu-accessibility";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const frames = [], lines = [];
const t0 = Date.now();
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 400)}`));
page.on("websocket", (socket) => {
  if (!socket.url().includes("/document/ws")) return;
  const opened = Date.now() - t0;
  frames.push({ at: opened, event: "open" });
  const keep = (direction) => (frame) => frames.push({ at: Date.now() - t0, direction, base64: Buffer.from(frame.payload).toString("base64") });
  socket.on("framesent", keep("sent"));
  socket.on("framereceived", keep("received"));
  socket.on("close", () => frames.push({ at: Date.now() - t0, event: "close" }));
});
const projection = async () => { try { return (JSON.parse(await page.evaluate(async () => (await globalThis.semioWgpuIntrospection?.dumpAccessibility?.()) ?? "")).windows ?? []).flatMap((w) => w.nodes ?? []); } catch { return []; } };
const key = async (suffix, budget = 20000) => { for (const end = Date.now() + budget; Date.now() < end; await page.waitForTimeout(700)) { const hit = (await projection()).find((n) => String(n.key).endsWith(suffix)); if (hit) return hit.key; } };
const act = async (k, ms = 2500) => { if (!k) return "absent"; const r = await page.evaluate(({ s, k }) => { const e = document.querySelector(`${s} [data-node-key="${k}"]`); if (!e) return "absent"; e.focus(); e.dispatchEvent(new MouseEvent("click", { bubbles: true })); return "activated"; }, { s: MIRROR, k }); await page.waitForTimeout(ms); return r; };
const type = async (k, v) => { if (!k) return false; for (const end = Date.now() + 15000; Date.now() < end; ) { const ok = await page.evaluate(({ s, k, v }) => { const e = document.querySelector(`${s} [data-node-key="${k}"]`); if (!(e instanceof HTMLInputElement)) return false; e.focus(); e.value = v; e.dispatchEvent(new Event("input", { bubbles: true })); e.dispatchEvent(new Event("change", { bubbles: true })); return true; }, { s: MIRROR, k, v }); await page.waitForTimeout(ok ? 1500 : 500); if (ok && (await projection()).some((n) => n.key === k && n.valueText === v)) return true; } return false; };
await page.goto(SHELL, { waitUntil: "domcontentloaded", timeout: 180000 });
await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180000 });
await page.waitForTimeout(8000);
await act("framework.hub.signIn", 2000);
await type(await key("framework.hub.address"), HUB);
for (let i = 0; i < 4 && (await act(await key("framework.hub.sign-in.add", 5000), 4000)) !== "activated"; i += 1) {}
for (let i = 0; i < 4; i += 1) {
  await act((await projection()).find((n) => String(n.key).includes("sign-in.connection") && String(n.key).includes("remote:"))?.key, 2000);
  await type(await key("framework.hub.email", 5000), USER);
  await type(await key("framework.hub.password", 5000), PASSWORDS[USER]);
  if ((await act(await key("framework.hub.sign-in.submit", 5000), 12000)) === "activated" && !(await key("framework.hub.sign-in.submit", 3000))) break;
}
await act((await projection()).find((n) => String(n.key).endsWith("framework.hub.close"))?.key, 1500);
await act("s-sync-status", 2000);
await act(await key("framework.sync.remote", 10000), 2000);
await type(await key("framework.sync.remote.path", 10000), `${HUB.replace(/^https?:\/\//u, "")}/${SPACE}/${DOCUMENT}`);
for (let i = 0; i < 30 && (await projection()).find((n) => String(n.key).endsWith("framework.sync.attach"))?.disabled !== false; i += 1) await page.waitForTimeout(500);
await act((await projection()).find((n) => String(n.key).endsWith("framework.sync.attach"))?.key, 1000);
const attachedAt = Date.now() - t0;
await page.waitForTimeout(45000);
for (let i = 0; i < 3 && !(await projection()).some((n) => String(n.key).endsWith("note-play-blocks.add.text")); i += 1) {
  if ((await projection()).find((n) => n.key === "framework.panel.artifact")?.checked !== true) await act("framework.panel.artifact", 3000);
  await key("note-play-blocks.add.text", 10000);
}
const nodes = await projection();
const blocks = nodes.filter((n) => String(n.key).startsWith("note-play-block:")).map((n) => n.label);
console.log(`BLOCKS ${blocks.length} ${JSON.stringify(blocks.slice(0, 8))} sync=${nodes.find((n) => n.key === "s-sync-status")?.label}`);
const hubProjection = await page.evaluate(async () => { try { return await globalThis.semioWgpuHubProjection?.(); } catch (e) { return String(e); } });
writeFileSync(`generated/attach-${tag}.json`, JSON.stringify({ attachedAt, frames, sync: nodes.filter((n) => String(n.key).startsWith("framework.sync") || n.key === "s-sync-status").map((n) => ({ key: n.key, label: n.label })), hubProjection }, null, 1));
writeFileSync(`generated/attach-${tag}-console.txt`, lines.join("\n"));
console.log(JSON.stringify({ attachedAt, frames: frames.length, opens: frames.filter((f) => f.event === "open").length, closes: frames.filter((f) => f.event === "close").length }));
await browser.close();
