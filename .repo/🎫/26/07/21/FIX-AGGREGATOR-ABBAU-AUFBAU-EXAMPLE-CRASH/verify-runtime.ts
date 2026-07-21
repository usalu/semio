import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
const logs: string[] = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));
page.on("pageerror", (err) => logs.push(`[pageerror] ${err.message}`));
page.on("crash", () => logs.push("[CRASH] page crashed"));

await page.goto(url, { waitUntil: "networkidle", timeout: 180_000 });
await page.waitForTimeout(6000);
await page.getByRole("button", { name: "Überspringen" }).click().catch(() => {});
await page.waitForTimeout(1000);

const before = await page.evaluate(() => {
  const canvas = document.querySelector("canvas") as HTMLCanvasElement | null;
  return {
    closed: false,
    hasAbbau: document.body.innerText.includes("Abbau Aufbau"),
    dataUrlLen: canvas ? canvas.toDataURL("image/png").length : 0,
  };
});
console.log(`[DEBUG] before=${JSON.stringify(before)}`);

// Open example combobox and re-select Abbau Aufbau (forces setActiveExample)
await page.getByRole("combobox").filter({ hasText: "Abbau Aufbau" }).click();
await page.waitForTimeout(400);
// If Nakagin available, switch away then back to stress reload
const nakagin = page.getByRole("option", { name: /Nakagin/i });
if (await nakagin.count()) {
  await nakagin.click();
  await page.waitForTimeout(4000);
  console.log(`[DEBUG] afterNakagin closed=${page.isClosed()}`);
  if (!page.isClosed()) {
    await page.getByRole("combobox").click();
    await page.waitForTimeout(300);
    await page.getByRole("option", { name: /Abbau Aufbau/i }).click();
    await page.waitForTimeout(6000);
  }
} else {
  // re-click same
  await page.getByRole("option", { name: /Abbau Aufbau/i }).click().catch(() => {});
  await page.waitForTimeout(4000);
}

console.log(`[DEBUG] finalClosed=${page.isClosed()}`);
if (!page.isClosed()) {
  const after = await page.evaluate(() => {
    const canvas = document.querySelector("canvas") as HTMLCanvasElement | null;
    return {
      hasAbbau: document.body.innerText.includes("Abbau Aufbau"),
      dataUrlLen: canvas ? canvas.toDataURL("image/png").length : 0,
      canvas: canvas ? { w: canvas.width, h: canvas.height } : null,
    };
  });
  console.log(`[DEBUG] after=${JSON.stringify(after)}`);
  const crashed = logs.some((l) => l.includes("[CRASH]"));
  const ok = !crashed && after.hasAbbau && after.dataUrlLen > 20_000;
  console.log(`[DEBUG] crashed=${crashed} ok=${ok}`);
  if (!ok) {
    console.log(`[DEBUG] logs=${JSON.stringify(logs.slice(-40), null, 2)}`);
    await browser.close();
    process.exit(1);
  }
  console.log("[DEBUG] runtime verification passed — Abbau Aufbau selectable without tab crash");
} else {
  console.error("[DEBUG] page closed/crashed");
  console.log(`[DEBUG] logs=${JSON.stringify(logs.slice(-40), null, 2)}`);
  await browser.close();
  process.exit(1);
}

await browser.close();
