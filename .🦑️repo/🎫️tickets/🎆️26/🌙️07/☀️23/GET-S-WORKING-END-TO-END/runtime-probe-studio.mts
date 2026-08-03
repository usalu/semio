import { chromium } from "playwright";

const logs: { type: string; text: string }[] = [];
const browser = await chromium.launch({
  headless: true,
  args: [
    "--enable-unsafe-webgpu",
    "--enable-features=Vulkan,UseSkiaRenderer",
    "--use-angle=swiftshader",
    "--ignore-gpu-blocklist",
  ],
});
const page = await browser.newPage();
page.on("console", (msg) => logs.push({ type: msg.type(), text: msg.text() }));
page.on("pageerror", (err) => logs.push({ type: "pageerror", text: String(err) }));
await page.goto("http://127.0.0.1:6070/?nocache=" + Date.now(), { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(5000);
console.log("HOME_TITLE", await page.title());
await page.getByText("Demo Studio", { exact: true }).first().dblclick({ timeout: 10000 });
await page.waitForURL(/\/spaces\//, { timeout: 15000 });
await page.waitForTimeout(10000);
console.log("AFTER_TITLE", await page.title());
console.log("AFTER_URL", page.url());
console.log("AFTER_BODY", (await page.innerText("body")).slice(0, 2000).replace(/\n/g, " | "));
const interesting = logs.filter((l) =>
  /DEBUG|bad magic|Render|NoCompatible|Workflow|loadDocument|openSpace|applyShell|host effect|pageerror/i.test(l.text) ||
  l.type === "error" ||
  l.type === "pageerror"
);
console.log("INTERESTING_COUNT", interesting.length);
for (const row of interesting.slice(0, 80)) console.log(`[${row.type}] ${row.text.slice(0, 400)}`);
await browser.close();
