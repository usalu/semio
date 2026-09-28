/** ⏱️ SH2 probe: after Home's delete dialog is submitted, how long until the deleted hub space leaves the LIVE Home table
 * (polled 240 s), which directory traffic arrives meanwhile, and whether a reload lists it.
 * usage: zsh sh2-hub-env.sh bun sh2-probe-delete-live.ts <serve-url> */
import { writeFileSync } from "node:fs";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { activate, boot, clickRowAction, dialog, openSessions, pageWindowedTables, settleHome, signIn, submitDialog, waitNamedRow, type Session } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts";

const [url = "http://127.0.0.1:6540/"] = process.argv.slice(2);
const out = "/Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/generated/probe-delete-live";
ensureParityPlaywrightBrowsersPath();
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const [session] = (await openSessions(browser, [url], [{ label: "user", email: process.env.OS_HUB_PROBE_EMAIL!, password: process.env.OS_HUB_PROBE_PASSWORD! }], "en-US")) as [Session];
const page = session.page;
const t0 = Date.now();
await session.context.addInitScript(() => {
  const seen: string[] = [];
  (globalThis as unknown as { __sh2Timeline: string[] }).__sh2Timeline = seen;
  let last = "";
  const scan = (): void => {
    const status = [...document.querySelectorAll("[data-directory-bootstrap]")].map((element) => `${element.getAttribute("data-directory-bootstrap")}:${element.getAttribute("data-directory-bootstrap-code") ?? ""}:${(element.textContent ?? "").slice(0, 60)}`).join("|");
    const table = [...document.querySelectorAll("[data-tree-window-key]")].map((element) => `total=${element.getAttribute("data-tree-window-total")} offset=${element.getAttribute("data-tree-window-offset")}`).join("|");
    const rows = document.querySelectorAll('[data-ui-node-key^="space:"]').length;
    const now = `status=[${status}] table=[${table}] rows=${rows}`;
    if (now !== last) {
      last = now;
      seen.push(`${Math.round(performance.now())} ${now}`);
    }
  };
  const start = (): void => {
    scan();
    new MutationObserver(scan).observe(document.body, { subtree: true, childList: true, attributes: true });
  };
  if (document.body) start();
  else document.addEventListener("DOMContentLoaded", start);
});
const traffic: string[] = [];
page.on("request", (request) => {
  if (/directory/u.test(request.url())) traffic.push(`${Date.now() - t0} REQ ${request.method()} ${request.url().replace(/^https?:\/\/[^/]+/u, "").slice(0, 120)}`);
});
page.on("websocket", (socket) => {
  if (!/directory/u.test(socket.url())) return;
  traffic.push(`${Date.now() - t0} WS-OPEN ${socket.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 120)}`);
  socket.on("framereceived", (frame) => traffic.push(`${Date.now() - t0} WS-RX ${String(frame.payload).length}B`));
  socket.on("close", () => traffic.push(`${Date.now() - t0} WS-CLOSE`));
});
const stamps = (): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll("[data-tree-window-key]")].map((element) => [...element.attributes].filter((attribute) => attribute.name.startsWith("data-tree-window")).map((attribute) => `${attribute.name}=${attribute.value.slice(0, 60)}`).join(" ")));
const listed = (name: string): Promise<boolean> => pageWindowedTables(page, () => page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => (elements.some((element) => (element.textContent ?? "").includes(wanted)) ? true : null), name)).then((hit) => hit === true);
try {
  await boot(session);
  await signIn(session);
  await settleHome(session);
  const name = `SH2 live ${Date.now().toString(36)}`;
  await activate(page, "s-home-create-space");
  await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
  await page.locator("#name").fill(name);
  await submitDialog(page);
  const spaceId = await waitNamedRow(page, "space", name, 120_000);
  console.log(`${Date.now() - t0} created ${spaceId}`);
  console.log("buttons", JSON.stringify(await page.locator(`[data-ui-node-key="space:${spaceId}"] button`).evaluateAll((elements) => elements.map((element) => `${element.getAttribute("aria-label") ?? ""} | ${element.getAttribute("title") ?? ""} | ${(element.textContent ?? "").trim()}`))));
  console.log("pressed", await clickRowAction(page, "space", spaceId, /^delete\b/iu));
  await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
  console.log("dialog", (await dialog(page).innerText()).replace(/\s+/gu, " ").slice(0, 300));
  console.log("stamps-before", JSON.stringify(await stamps()));
  const submittedAt = Date.now();
  traffic.push(`${submittedAt - t0} SUBMIT`);
  await submitDialog(page);
  let goneAt: number | null = null;
  while (Date.now() - submittedAt < 150_000) {
    if (!(await listed(name))) {
      goneAt = Date.now();
      break;
    }
    await page.waitForTimeout(5_000);
  }
  console.log("stamps-after", JSON.stringify(await stamps()));
  console.log("submitted-at-perf", await page.evaluate(() => Math.round(performance.now())) - (Date.now() - submittedAt));
  console.log("timeline", JSON.stringify(await page.evaluate(() => (globalThis as unknown as { __sh2Timeline?: string[] }).__sh2Timeline ?? []), null, 1));
  console.log("live-gone-after-ms", goneAt === null ? null : goneAt - submittedAt);
  await page.screenshot({ path: `${out}-live.png` });
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
  await settleHome(session);
  console.log("listed-after-reload", await listed(name));
} finally {
  console.log("traffic", JSON.stringify(traffic, null, 1));
  writeFileSync(`${out}-console.txt`, session.lines.join("\n"));
  await browser.close();
}
