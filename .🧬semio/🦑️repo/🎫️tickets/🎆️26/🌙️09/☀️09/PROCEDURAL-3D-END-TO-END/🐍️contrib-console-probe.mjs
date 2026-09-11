
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "contrib-console");
mkdirSync(outDir, { recursive: true });
const rows = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("console", async (msg) => {
  const text = msg.text();
  if (/contribution|setContributions|exampleArtifact|invokeExtension|extension-not|scoped from/i.test(text)) {
    rows.push({ t: Date.now(), level: msg.type(), text: text.slice(0, 2000) });
    console.log(msg.type(), text.slice(0, 400));
  }
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(15000);
writeFileSync(join(outDir, "hits.json"), JSON.stringify(rows, null, 2));
console.log("hits", rows.length);
await browser.close();
