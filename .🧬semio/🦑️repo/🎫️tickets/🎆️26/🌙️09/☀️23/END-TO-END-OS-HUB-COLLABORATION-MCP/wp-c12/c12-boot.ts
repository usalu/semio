/** 🏠️ C12 (rule 20): one `serve s react dev` through S18's shared `ensureDevServe`, booted to Home in a headless browser
 * (pageerror-free), then stopped. usage: bun c12-boot.ts <port> [hubUrl] */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const repoRoot = "/Users/ueli/Documents/semio";
const { ensureDevServe } = await import(`${repoRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts`);
const port = Number(process.argv[2] ?? 6520);
const hubUrl = process.argv[3];
const serve = await ensureDevServe({ repoRoot, port, variant: "s", locale: "en", hubUrl, bootBoundMs: 900_000, logPath: `${repoRoot}/.🧬semio/🌐hub/s14-c12-logs/serve-${port}-14c.txt`, onProgress: (_status: unknown, text: string) => console.log(`[serve] ${text}`) });
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const errors: string[] = [];
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
  page.on("pageerror", (error) => errors.push(String(error).slice(0, 300)));
  const started = Date.now();
  await page.goto(serve.url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 240_000 });
  console.log(`BOOT HOME ok in ${Date.now() - started} ms; page errors ${errors.length} ${JSON.stringify(errors)}`);
} catch (error) {
  console.log(`BOOT FAIL ${String(error).split("\n")[0]}; page errors ${JSON.stringify(errors)}`);
} finally {
  await browser.close();
  await serve.stop();
  console.log(`serve ${serve.reused ? "reused (left running)" : "stopped"}`);
}
