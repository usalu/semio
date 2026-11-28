import { awaitBeacon, dismissIntroduction, chromium, url } from "./journey-kit.ts";
const browser = await chromium.launch();
for (const [lang, locale] of [["de", "de-DE"], ["en", "en-US"]] as const) {
  const context = await browser.newContext({ locale, viewport: { width: 1800, height: 1100 } });
  const page = await context.newPage();
  await page.goto(url);
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await page.waitForTimeout(3000);
  for (const nm of ["Pool", "Kuratiert", "Curated"]) console.log(lang, nm, await page.getByRole("tabpanel", { name: nm, exact: true }).count(), await page.getByRole("tabpanel").count());
  console.log(lang, await page.getByRole("tabpanel").first().evaluate(e => e.outerHTML.slice(0, 700)));
  console.log(lang, "rows", await page.locator("tbody tr").count(), await page.evaluate(() => document.body.innerText.slice(0, 600)));
  await context.close();
}
await browser.close();
