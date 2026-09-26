/** 🧷️ G10 bridge live proof, shell half: one fresh page on a local `s` serve whose rendezvous holds
 * exactly the gateway under test. Records every `/bridge` websocket the shell opens, the first byte of
 * every frame it receives (0 = welcome, 12 = refused), the footer notice, and every console message.
 *
 * usage: bun g10-bridge-live.ts <port> <en|de> <observeMs> <label>
 */
import { writeFileSync } from "node:fs";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";

const [port, locale, observeMsArg, label] = process.argv.slice(2);
const observeMs = Number(observeMsArg);
const out = `/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/🗑️generated/s12/bridge-live-${label}`;
const started = Date.now();
const since = (): number => Date.now() - started;
const sockets: { url: string; openedAtMs: number; closedAtMs: number | null; framesIn: number[] }[] = [];
const consoleRows: { type: string; text: string; atMs: number }[] = [];
const timeline: { atMs: number; status: string | null; text: string | null }[] = [];

const browser = await chromium.launch({ headless: true });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: locale === "de" ? "de-DE" : "en-US" })).newPage();
page.on("console", (message) => consoleRows.push({ type: message.type(), text: message.text().slice(0, 400), atMs: since() }));
page.on("pageerror", (error) => consoleRows.push({ type: "pageerror", text: String(error).slice(0, 400), atMs: since() }));
page.on("websocket", (socket) => {
  if (!socket.url().includes("/bridge")) return;
  const row = { url: socket.url(), openedAtMs: since(), closedAtMs: null as number | null, framesIn: [] as number[] };
  sockets.push(row);
  socket.on("framereceived", (frame) => {
    const payload = frame.payload;
    row.framesIn.push(typeof payload === "string" ? -1 : payload[0] ?? -1);
  });
  socket.on("close", () => (row.closedAtMs = since()));
});
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
let last = "";
while (since() < observeMs) {
  const notice = await page
    .locator("[data-semio-agent-bridge-status]")
    .first()
    .evaluate((node) => ({ status: node.getAttribute("data-semio-agent-bridge-status"), text: node.textContent }))
    .catch(() => ({ status: null, text: null }));
  const key = `${notice.status}|${notice.text}`;
  if (key !== last) {
    timeline.push({ atMs: since(), ...notice });
    last = key;
  }
  await page.waitForTimeout(1000);
}
const notices = await page.locator("[data-semio-agent-bridge-status]").count();
await page.screenshot({ path: `${out}.png` });
const bridgeConsole = consoleRows.filter((row) => /bridge|websocket|agent/i.test(row.text));
const report = {
  label,
  locale,
  observeMs,
  bridgeSockets: sockets.length,
  sockets,
  welcomes: sockets.reduce((count, row) => count + row.framesIn.filter((tag) => tag === 0).length, 0),
  refusals: sockets.reduce((count, row) => count + row.framesIn.filter((tag) => tag === 12).length, 0),
  noticeCount: notices,
  timeline,
  consoleErrors: consoleRows.filter((row) => row.type === "error" || row.type === "pageerror").length,
  bridgeConsole,
};
writeFileSync(`${out}.json`, `${JSON.stringify({ ...report, console: consoleRows }, null, 2)}\n`);
console.log(JSON.stringify(report));
await browser.close();
