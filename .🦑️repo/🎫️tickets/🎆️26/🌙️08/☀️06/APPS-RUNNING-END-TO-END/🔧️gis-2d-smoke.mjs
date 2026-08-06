import { writeFileSync } from "fs";
import { chromium } from "playwright";

const url = "http://127.0.0.1:6040/";
const errors = [];
const logs = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("pageerror", (err) => errors.push(`pageerror: ${err}`));
page.on("console", (msg) => {
  const text = msg.text();
  logs.push({ type: msg.type(), text: text.slice(0, 500) });
  if (msg.type() === "error") errors.push(`console.error: ${text.slice(0, 500)}`);
});

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(12000);

const title = await page.title();
const rootHTML = await page.evaluate(() => document.getElementById("root")?.innerHTML?.slice(0, 1500) ?? "");
const bodyText = await page.evaluate(() => document.body?.innerText?.slice(0, 2000) ?? "");
const styled = await page.evaluate(() => document.documentElement.dataset.semioStyled ?? null);
const childCount = await page.evaluate(() => document.getElementById("root")?.childElementCount ?? 0);

await browser.close();

const joined = errors.join("\n");
const blocking = errors.filter((e) => !e.includes("NoCompatibleDevice"));
const out = {
  engine: "playwright",
  title,
  styled,
  childCount,
  rootHTML,
  bodyText,
  errors,
  logs: logs.filter((l) => l.type === "error" || /\[DEBUG\]|GIS|controlLabel|Shell|plugin/i.test(l.text)).slice(0, 80),
  ok:
    childCount > 0 &&
    blocking.length === 0 &&
    !joined.includes("controlLabelIdResolver") &&
    !joined.includes("is not a function") &&
    !joined.includes("FrameworkOsShellInner"),
};
writeFileSync(new URL("./🧪gis-2d-smoke.json", import.meta.url), JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
process.exit(out.ok ? 0 : 1);
