/** 🔍️ WG11 exploration (ticket-local, expendable): what the React `s` shell exposes for one open hub note — node keys, block rows,
 * the add-text control — so the permanent harness can address them. Usage: bun wg11-explore-react.mjs <reactUrl> <space> <document>
 * Env: SEMIO_TWO_HUMAN_USER2_EMAIL / _PASSWORD. Output: wp-wg11/generated/explore-react-*. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const [URL, SPACE, DOCUMENT] = process.argv.slice(2);
const OUT = join(import.meta.dir, "generated");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-US" })).newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()} ${m.text().slice(0, 300)}`));
try {
  await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(process.env.SEMIO_TWO_HUMAN_USER2_EMAIL);
  await form.locator('input[type="password"]').fill(process.env.SEMIO_TWO_HUMAN_USER2_PASSWORD);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.goto(new globalThis.URL(`/spaces/${SPACE}`, URL).href, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator(`[data-ui-node-key="artifact:${DOCUMENT}"]`).first().waitFor({ state: "attached", timeout: 180_000 });
  const buttons = page.locator(`[data-ui-node-key="artifact:${DOCUMENT}"] button`);
  for (let index = 0; index < (await buttons.count()); index += 1) {
    const name = `${(await buttons.nth(index).getAttribute("aria-label")) ?? ""} ${(await buttons.nth(index).textContent()) ?? ""}`;
    if (/open|öffnen/iu.test(name)) {
      await buttons.nth(index).focus();
      await buttons.nth(index).press("Enter");
      break;
    }
  }
  await page.waitForTimeout(45_000);
  const allIds = await page.evaluate(() => [...document.querySelectorAll("[id]")].map((el) => `${el.id} ${el.getAttribute("data-slot") ?? ""} ${el.tagName}`));
  writeFileSync(join(OUT, "explore-react-ids-before.txt"), allIds.join("\n"));
  const artifactTab = page.locator('[id="framework.panel.artifact"]');
  console.log("artifact tab count", await artifactTab.count());
  if (await artifactTab.count()) await artifactTab.first().click({ force: true });
  await page.waitForTimeout(5_000);
  const afterIds = await page.evaluate(() => [...document.querySelectorAll("[id],[data-row-id],[data-ui-node-key]")].map((el) => `${el.id} | ${el.getAttribute("data-row-id") ?? ""} | ${el.getAttribute("data-ui-node-key") ?? ""} | ${el.getAttribute("data-slot") ?? ""} | ${el.tagName} | ${(el.innerText ?? "").replace(/\s+/g, " ").slice(0, 40)}`));
  writeFileSync(join(OUT, "explore-react-ids-after.txt"), afterIds.join("\n"));
  const keys = await page.evaluate(() => [...document.querySelectorAll("[data-ui-node-key]")].map((el) => ({ key: el.getAttribute("data-ui-node-key"), tag: el.tagName, text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 60), role: el.getAttribute("role"), label: el.getAttribute("aria-label") })));
  writeFileSync(join(OUT, "explore-react-keys.json"), JSON.stringify(keys, null, 1));
  const summary = await page.evaluate(() => ({
    sync: document.querySelector('[id="s-sync-status"]')?.innerText,
    peers: [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id]')].map((el) => el.getAttribute("data-row-id") + " " + el.innerText),
    ids: [...document.querySelectorAll("[id]")].map((el) => el.id).filter((id) => /note|block|action\.|history/u.test(id)).slice(0, 80),
  }));
  writeFileSync(join(OUT, "explore-react-summary.json"), JSON.stringify(summary, null, 1));
  await page.screenshot({ path: join(OUT, "explore-react.png") });
  console.log(JSON.stringify(summary).slice(0, 3000));
  console.log(keys.filter((row) => /note|block|add/u.test(String(row.key))).slice(0, 60).map((row) => JSON.stringify(row)).join("\n"));
} catch (error) {
  console.log("ERROR", String(error).slice(0, 800));
  await page.screenshot({ path: join(OUT, "explore-react-error.png") });
} finally {
  writeFileSync(join(OUT, "explore-react-console.txt"), lines.join("\n"));
  await browser.close();
}
