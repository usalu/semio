/** 🔎️ F3 — lists a cold boot's resources of some kinds (per the boot-budget classifier), to calibrate the budget.
 * usage: bun f3-boot-kinds.ts <baseUrl> <kind,…> */
import { chromium } from "playwright";
import { awaitBeacon } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { bootResourceKindV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🥾️boot-budget/🟦️.ts";
const [baseUrl, kindsArg] = process.argv.slice(2) as [string, string];
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage();
  await page.addInitScript(() => performance.setResourceTimingBufferSize(100_000));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await page.waitForTimeout(4_000);
  const rows = await page.evaluate(() => (performance.getEntriesByType("resource") as PerformanceResourceTiming[]).map((e) => ({ url: e.name, initiatorType: e.initiatorType, decoded: e.decodedBodySize })));
  const wanted = kindsArg.split(",");
  for (const row of rows) {
    const kind = bootResourceKindV1(row.url, row.initiatorType);
    if (wanted.includes(kind)) console.log(kind, row.initiatorType, (row.decoded / 1048576).toFixed(2), decodeURIComponent(new URL(row.url).pathname).split("/").slice(-3).join("/"), new URL(row.url).search.slice(0, 30));
  }
} finally {
  await browser.close();
}
