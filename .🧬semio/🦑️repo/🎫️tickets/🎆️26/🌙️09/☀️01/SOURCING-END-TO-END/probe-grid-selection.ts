import { awaitBeacon, dismissIntroduction, runJourneys, session, url, type Browser } from "./journey-kit.ts";
import { createHash } from "node:crypto";
const G = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/🗑️generated/journey/";
async function journey(browser: Browser, language: "en" | "de") {
  const s = await session(browser, language);
  const { page, t, pool, settle } = s;
  await page.goto(url);
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await settle(3000);
  const active = () => pool().locator('tbody tr[data-active="true"]').evaluateAll(r => r.map(x => x.getAttribute("data-row-id")));
  const set = async (id: string, n: string) => { const b = pool().locator(`tbody tr[data-row-id="${id}"]`).getByRole("spinbutton").first(); await b.click(); await b.fill(n); await b.press("Enter"); await settle(1500); };
  await set("beam-kvh-c24", "2"); await set("beam-steel-ipe200", "1");
  const grid = page.getByRole("tabpanel", { name: t.grid, exact: true });
  const box = (await grid.boundingBox())!;
  await page.locator('[id="framework.window.sourcingGrid.utilityBar.unfold"]').click(); await settle(1500);
  const els = await page.locator("[id^='framework.window.sourcingGrid.utilityBar']").evaluateAll(e => e.map(x => `${x.id}|${x.getAttribute("aria-label") ?? ""}|${(x.textContent ?? "").slice(0, 30)}`));
  console.log("[DEBUG] els", JSON.stringify(els));
  await page.screenshot({ path: G + "probe-util.png" });
}
await runJourneys("probe-grid", journey);
