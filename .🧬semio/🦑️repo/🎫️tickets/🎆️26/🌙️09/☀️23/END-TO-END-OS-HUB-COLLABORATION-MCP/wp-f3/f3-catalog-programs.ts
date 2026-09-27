/** 📋️ F3 — the catalog probe's programs for some plugins. usage: bun f3-catalog-programs.ts <baseUrl> <plugin,…> */
import { chromium } from "playwright";
import { awaitBeacon } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  await page.goto(process.argv[2]!, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  const wanted = process.argv[3]!.split(",");
  const probe = await page.evaluate(() => (window as unknown as { __semioOsCatalogProbe?: { programs: unknown[] } }).__semioOsCatalogProbe?.programs ?? []);
  for (const row of probe as Record<string, unknown>[]) if (wanted.some((id) => JSON.stringify(row).includes(`"${id}"`) || String(row.pluginId) === id)) console.log(JSON.stringify(row).slice(0, 300));
} finally {
  await browser.close();
}
