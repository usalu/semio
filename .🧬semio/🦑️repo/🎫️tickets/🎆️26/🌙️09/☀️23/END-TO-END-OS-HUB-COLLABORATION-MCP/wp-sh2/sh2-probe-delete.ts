/** 🗑️ SH2 probe: Home's hub-space delete dialog — its fields, the submit, the directory command reply and whether the row leaves.
 * usage: OS_HUB_PROBE_EMAIL=… OS_HUB_PROBE_PASSWORD=… bun sh2-probe-delete.ts <serve-url> <space-name> */
import { writeFileSync } from "node:fs";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { boot, clickRowAction, dialog, openSessions, pageWindowedTables, settleHome, signIn, waitNamedRow, type Session } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🟦️.ts";

const [url = "http://127.0.0.1:6540/", name = "Home E2E hub"] = process.argv.slice(2);
const out = "/Users/ueli/Documents/semio/.tmp-ticket/wp-sh2/generated/probe-delete";
ensureParityPlaywrightBrowsersPath();
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const [session] = (await openSessions(browser, [url], [{ label: "user", email: process.env.OS_HUB_PROBE_EMAIL!, password: process.env.OS_HUB_PROBE_PASSWORD! }], "en-US")) as [Session];
const page = session.page;
const replies: string[] = [];
page.on("response", async (response) => {
  if (/directory\/commands/u.test(response.url())) replies.push(`${response.status()} ${response.request().method()} ${response.url().replace(/^https?:\/\/[^/]+/u, "")} ${(await response.text().catch(() => "")).slice(0, 400)}`);
});
page.on("request", (request) => {
  if (/directory\/commands/u.test(request.url())) replies.push(`REQ ${request.method()} ${(request.postData() ?? "").slice(0, 600)}`);
});
try {
  await boot(session);
  await signIn(session);
  await settleHome(session);
  const listed = await pageWindowedTables(page, async () => null).then(() => page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements) => elements.map((element) => (element.textContent ?? "").slice(0, 40))));
  console.log("rows", listed.length, JSON.stringify(listed.filter((row) => row.includes("Home E2E"))));
  const spaceId = await waitNamedRow(page, "space", name, 60_000);
  const buttons = await page.locator(`[data-ui-node-key="space:${spaceId}"] button`).evaluateAll((elements) => elements.map((element) => `${element.getAttribute("aria-label") ?? ""} | ${element.getAttribute("title") ?? ""} | ${element.textContent ?? ""}`));
  console.log("row", spaceId, JSON.stringify(buttons));
  const pressed = await clickRowAction(page, "space", spaceId, /\bdelete\b/iu);
  console.log("pressed", pressed);
  await dialog(page).waitFor({ state: "visible", timeout: 20_000 });
  console.log("dialog", (await dialog(page).innerText()).replace(/\s+/gu, " ").slice(0, 600));
  console.log("fields", JSON.stringify(await dialog(page).locator("input, select, button, textarea").evaluateAll((elements) => elements.map((element) => ({ tag: element.tagName, id: element.id, type: element.getAttribute("type"), disabled: (element as HTMLButtonElement).disabled, text: (element.textContent ?? "").trim().slice(0, 40) })))));
  await page.screenshot({ path: `${out}-dialog.png` });
  await page.locator('[id="ui.dialog.submit"]').click();
  await page.waitForTimeout(15_000);
  console.log("dialog-after", await dialog(page).count());
  const still = await page.locator('[data-ui-node-key^="space:"]').evaluateAll((elements, wanted) => elements.some((element) => (element.textContent ?? "").includes(wanted)), name);
  console.log("row-still-listed", still);
  await page.screenshot({ path: `${out}-after.png` });
} finally {
  console.log("replies", JSON.stringify(replies, null, 1));
  writeFileSync(`${out}-console.txt`, session.lines.join("\n"));
  await browser.close();
}
