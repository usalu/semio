/** 🛰️ Live wgpu-shell hub probe (slice WGr).
 *
 * Boots the already-serving wgpu playground in headless chromium with `--use-angle=metal`
 * (SwiftShader gives false negatives on this renderer), waits for the frame Worker's readiness
 * beacon, and reads the retained chrome + accessibility projection looking for WG6's hub surface:
 * the footer connection pill (`s-hub-connection` / `framework.hub.signIn`) and the `framework.hub`
 * dock leaf. Writes screenshots and dumps into this ticket's `🗑️generated/wgr-live-*`.
 *
 * Paths go through `fileURLToPath` — `new URL(...).pathname` percent-encodes the emoji segments and
 * silently writes into a stray `.%F0%9F%A7%ACsemio/` tree (preamble rule 24).
 *
 * Usage (from the ticket folder):
 *   SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d bun 🐍️wgr-live-wgpu-hub-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const outDir = join(here, "🗑️generated");
mkdirSync(outDir, { recursive: true });
const out = (name) => join(outDir, `wgr-live-${name}`);

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6213/?plugin=puzzle3d";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 60);

const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"],
});
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });

const console_lines = [];
const failed_requests = [];
page.on("console", (message) => console_lines.push(`${message.type()} ${message.text()}`));
page.on("pageerror", (error) => console_lines.push(`pageerror ${String(error)}`));
page.on("requestfailed", (request) => failed_requests.push(`${request.failure()?.errorText ?? "?"} ${request.url()}`));

await page.goto(url, { waitUntil: "domcontentloaded" });

const booted = await page
  .waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: bootSeconds * 1000 })
  .then(() => true)
  .catch(() => false);

await page.waitForTimeout(6000);
await page.screenshot({ path: out("boot.png"), type: "png" });

const dumps = await page.evaluate(async () => {
  const introspection = globalThis.semioWgpuIntrospection;
  if (!introspection) return { available: false };
  const read = async (kind, windowId) => {
    try {
      return (await introspection[kind](windowId)) ?? "";
    } catch (error) {
      return `<failed: ${String(error)}>`;
    }
  };
  const structure = await read("dumpStructure");
  let windowIds = [];
  try {
    windowIds = JSON.parse(structure).windowIds ?? [];
  } catch {}
  return {
    available: true,
    windowIds,
    structure,
    chrome: await read("dumpChrome"),
    accessibility: await read("dumpAccessibility"),
    hubProjection: typeof globalThis.semioWgpuHubProjection?.publishDocumentStatus === "function",
  };
});

writeFileSync(out("dumps.json"), JSON.stringify(dumps, null, 2));
writeFileSync(out("console.txt"), console_lines.join("\n"));
writeFileSync(out("failed-requests.txt"), failed_requests.join("\n"));

const haystack = `${dumps.chrome ?? ""}\n${dumps.accessibility ?? ""}`;
const markers = ["s-hub-connection", "framework.hub.signIn", "framework.hub", "s-sync-status", "s-presence-peers"];
const found = Object.fromEntries(markers.map((marker) => [marker, haystack.includes(marker)]));

const verdict = {
  url,
  booted,
  introspectionAvailable: dumps.available === true,
  hubProjectionGlobal: dumps.hubProjection === true,
  windowIds: dumps.windowIds ?? [],
  chromeBytes: (dumps.chrome ?? "").length,
  accessibilityBytes: (dumps.accessibility ?? "").length,
  markers: found,
  consoleErrors: console_lines.filter((line) => line.startsWith("error") || line.startsWith("pageerror")).slice(0, 20),
  failedRequests: failed_requests.slice(0, 20),
};
writeFileSync(out("verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2));

await browser.close();
