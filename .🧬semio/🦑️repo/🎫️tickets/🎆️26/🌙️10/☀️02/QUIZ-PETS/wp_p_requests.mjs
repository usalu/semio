/** 🧮️ Ticket tool of work package P: counts the requests one fresh learner causes on a stack until the pets stand on
 * the overview, and how many of them fetch the pets (the menagerie and the render target).
 *
 * Usage (from the repository root, with a private stack up): node ".../wp_p_requests.mjs" [http://127.0.0.1:6193]
 */
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6193";
const browser = await chromium.launch();
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" });
  const page = await context.newPage();
  const seen = [];
  page.on("request", (request) => seen.push(decodeURIComponent(request.url())));
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  await front("introduction").waitFor({ timeout: 150000 });
  await front("introduction").locator('[data-overview-card-action="primary"]').click();
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await front("identity").locator('[data-overview-card-action="primary"]').click();
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 60000 });
  const pets = seen.filter((address) => address.includes("🐾️pets"));
  process.stdout.write(`${url}: ${seen.length} requests, ${pets.length} of them for the pets (${pets.filter((address) => address.includes("🏛️architecture/🐾️pets")).length} menagerie documents and module, ${pets.filter((address) => address.includes("🛍️products/🐾️pets")).length} product modules)\n`);
} finally {
  await browser.close();
}
