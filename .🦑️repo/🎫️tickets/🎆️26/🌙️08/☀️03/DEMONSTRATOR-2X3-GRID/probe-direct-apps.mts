import { chromium } from "playwright";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const apps = [
  ["generator", "http://127.0.0.1:6027/"],
  ["koordinator", "http://127.0.0.1:6028/"],
  ["aggregator", "http://127.0.0.1:6023/"],
  ["aussuchen", "http://127.0.0.1:6030/"],
  ["bearbeiten", "http://127.0.0.1:6031/"],
  ["verfolgen", "http://127.0.0.1:6032/"],
];
const results: Record<string, unknown> = {};
for (const [name, url] of apps) {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 } });
  const errors: string[] = [];
  const logs: string[] = [];
  page.on("pageerror", (e) => errors.push(e.message.slice(0, 300)));
  page.on("console", (m) => {
    if (m.type() === "error") errors.push(m.text().slice(0, 300));
    if (m.type() === "log" || m.type() === "warning") logs.push(`${m.type()}: ${m.text().slice(0, 200)}`);
  });
  try {
    await page.goto(url, { waitUntil: "networkidle", timeout: 90000 });
  } catch (e) {
    errors.push(`goto: ${(e as Error).message.slice(0, 200)}`);
  }
  await page.waitForTimeout(15000);
  const snap = await page.evaluate(() => {
    const root = document.querySelector("#root");
    return {
      title: document.title,
      readyState: document.readyState,
      rootChildren: root?.children.length ?? -1,
      canvas: [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height, cw: c.clientWidth, ch: c.clientHeight })),
      bodyText: (document.body?.innerText || "").slice(0, 300),
      bg: getComputedStyle(document.body).backgroundColor,
    };
  });
  await page.screenshot({ path: join(outDir, `probe-direct-${name}.png`) });
  results[name] = { url, snap, errors: errors.slice(0, 15), logs: logs.filter((l) => /wasm|error|fail|boot|plugin/i.test(l)).slice(0, 20) };
  await page.close();
  console.log(name, JSON.stringify(results[name]));
}
writeFileSync(join(outDir, "probe-direct-out.json"), JSON.stringify(results, null, 2));
await browser.close();
