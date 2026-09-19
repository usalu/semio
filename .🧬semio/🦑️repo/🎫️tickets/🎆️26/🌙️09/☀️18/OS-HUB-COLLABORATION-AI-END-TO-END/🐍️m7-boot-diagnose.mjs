/** 🩺️ Slice M7 — why the note React session at 6080 does not reach `data-semio-os-ready`, and what
 * the agent-bridge seam is doing while it boots. Dumps every console line (unfiltered), the shell's
 * own boot attributes over time, and the discovery endpoint's answer. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const port = process.env.SEMIO_M7_PORT ?? "6080";
const plugin = process.env.SEMIO_M7_PLUGIN ?? "note";
const seconds = Number(process.env.SEMIO_M7_SECONDS ?? 180);
const ticketDir = dirname(fileURLToPath(import.meta.url));
const url = `http://127.0.0.1:${port}/?plugin=${plugin}`;

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 2000)}`));
page.on("pageerror", (error) => lines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 2000)}`));
page.on("requestfailed", (request) => lines.push(`${Date.now() - t0} requestfailed ${request.url().slice(0, 200)} ${request.failure()?.errorText ?? ""}`));

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });

const view = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  attrs: [...document.documentElement.attributes].map((attribute) => attribute.name).filter((name) => name.startsWith("data-")),
  bodyChars: document.body?.innerText?.length ?? 0,
  surfaces: document.querySelectorAll("[data-surface-id]").length,
  windows: document.querySelectorAll("[data-window-id]").length,
  chatPanel: document.querySelector("[data-semio-agent-chat-panel]") !== null,
  presence: document.querySelector("[data-semio-agent-presence-tone]")?.getAttribute("data-semio-agent-presence-tone") ?? null,
  head: (document.body?.innerText ?? "").replace(/\s+/g, " ").slice(0, 300),
}));

const timeline = [];
for (let elapsed = 0; elapsed < seconds; elapsed += 10) {
  const snapshot = await view();
  timeline.push(`${elapsed}s ${JSON.stringify(snapshot)}`);
  console.log(`${elapsed}s ready=${snapshot.ready} error=${snapshot.error} windows=${snapshot.windows} surfaces=${snapshot.surfaces} chars=${snapshot.bodyChars} chat=${snapshot.chatPanel} presence=${snapshot.presence}`);
  if (snapshot.ready === "true") break;
  await new Promise((resolve) => setTimeout(resolve, 10_000));
}

writeFileSync(join(ticketDir, "🗑️generated", "m7-boot-console.txt"), `${timeline.join("\n")}\n\n=== console\n${lines.join("\n")}\n`);
console.log(`\nconsole lines: ${lines.length} → 🗑️generated/m7-boot-console.txt`);
console.log(lines.filter((line) => /error|pageerror|requestfailed|trap|unreachable|panic|refus/i.test(line)).slice(0, 30).join("\n"));
await browser.close();
