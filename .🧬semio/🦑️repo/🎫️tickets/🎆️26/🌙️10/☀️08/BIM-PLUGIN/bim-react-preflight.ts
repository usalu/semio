import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const runner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts");
const { browserLaunchOptionsV1, launchBrowserV1 } = await import(pathToFileURL(runner).href);
const { repoPlaywrightBrowsersPath } = await import(pathToFileURL(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts")).href);
process.env.PLAYWRIGHT_BROWSERS_PATH = repoPlaywrightBrowsersPath(process.cwd());
const browser = await launchBrowserV1(browserLaunchOptionsV1());
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 960 } });
  page.on("pageerror", error => console.error("[DEBUG] React page fault", error.message));
  page.on("console", message => { if (message.type() === "error" || message.type() === "warning") console.error("[DEBUG] React console", message.text()); });
  page.on("response", response => {
    if (response.status() >= 400) console.error("[DEBUG] React HTTP", response.status(), decodeURIComponent(response.url()));
  });
  const guest = page.waitForResponse(response => /🏬️bim\/(?:🔣️\.json|🛂️\.descriptor\.semio|🌉️bridge\.js)/u.test(decodeURIComponent(response.url())), { timeout: 100000 });
  await page.goto("http://127.0.0.1:16302/?example=house", { waitUntil: "domcontentloaded", timeout: 120000 });
  const response = await guest;
  console.log("[DEBUG] Production React shell reached the declared BIM guest URL", JSON.stringify({ status: response.status(), url: decodeURIComponent(response.url()) }));
} finally {
  await browser.close();
}
