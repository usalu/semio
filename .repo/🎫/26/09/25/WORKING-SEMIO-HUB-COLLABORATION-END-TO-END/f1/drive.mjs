import { chromium } from "/home/user/semio/node_modules/playwright/index.mjs";
import { fileURLToPath } from "node:url";
const base = process.env.BASE ?? "http://127.0.0.1:5173";
const shots = fileURLToPath(new URL("./shots/", import.meta.url));
const steps = (process.env.STEPS ?? "home").split(",");
const args = "--use-angle=swiftshader,--enable-unsafe-swiftshader,--ignore-gpu-blocklist,--enable-unsafe-webgpu,--enable-features=Vulkan,--use-vulkan=swiftshader,--use-webgpu-adapter=swiftshader".split(",");
const browser = await chromium.launch({ executablePath: "/opt/pw-browsers/chromium", args });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const logs = [];
await page.addInitScript(() => { Error.stackTraceLimit = 60; });
page.on("console", (m) => { if (["error", "warning"].includes(m.type()) || m.text().startsWith("[DEBUG]")) logs.push(`[${m.type()}] ${m.text().slice(0, Number(process.env.MSG ?? 400))}`); });
page.on("worker", (w) => { logs.push(`[worker] ${w.url().slice(-60)}`); w.on("console", (m) => logs.push(`[worker:${m.type()}] ${m.text().slice(0, Number(process.env.MSG ?? 400))}`)); w.on("close", () => logs.push(`[worker-closed] ${w.url().slice(-60)}`)); });
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}\n${(e.stack ?? "").slice(0, Number(process.env.STACK ?? 800))}`));
const prefix = process.env.PREFIX ?? "";
const shot = async (name) => { await page.screenshot({ path: `${shots}${prefix}${name}.png` }); console.log("[DEBUG] shot", name, page.url()); };
const palette = async (text) => {
  await page.locator('[id="ui.search.toggle"]').click();
  const dialog = page.getByRole("dialog");
  await dialog.getByPlaceholder("Search...").fill(text);
  await dialog.getByText(text).first().click();
};
const rowDbl = async (text, exact = false) => {
  const row = exact ? page.locator("tr[data-row-id]").filter({ has: page.locator("span.truncate", { hasText: new RegExp(`^${text}$`) }) }).last() : page.locator("tr[data-row-id]").filter({ hasText: text }).first();
  await row.waitFor({ timeout: 120000 });
  await row.dblclick();
};
const wait = (ms) => page.waitForTimeout(ms);
try {
  await page.goto(base + "/", { waitUntil: "networkidle", timeout: 180000 });
  await page.getByRole("columnheader", { name: "Name" }).waitFor({ timeout: 180000 });
  await wait(1500);
  await shot("01-home");
  for (const step of steps) {
    if (step === "fixture" || step === "nakagin") {
      await palette(step === "fixture" ? "Open metabolism fixture" : "Open Nakagin filtered fixture");
      await page.waitForURL(/\/kits\/[0-9a-f-]{36}/i, { timeout: 180000 });
      await wait(Number(process.env.KIT_WAIT ?? 8000));
      await shot(`02-kit-${step}`);
    } else if (step === "design") {
      await rowDbl(process.env.DESIGN ?? "Nakagin Capsule Tower");
      await page.waitForURL(/\/designs\//i, { timeout: 120000 });
      await wait(Number(process.env.DESIGN_WAIT ?? 10000));
      await shot("03-design");
    } else if (step === "type") {
      await rowDbl(process.env.TYPE ?? "Base", true);
      await page.waitForURL(/\/types\//i, { timeout: 120000 });
      await wait(10000);
      await shot("04-type");
    } else if (step === "back") {
      await page.goBack({ waitUntil: "networkidle" });
      await wait(4000);
    } else if (step === "docs") {
      await page.goto(base + "/docs/getting-started/index", { waitUntil: "networkidle" });
      await wait(4000);
      await shot("05-docs");
    } else if (step === "feedback") {
      await page.goto(base + "/feedback", { waitUntil: "networkidle" });
      await wait(3000);
      await shot("06-feedback");
    } else if (step.startsWith("panel:")) {
      const id = step.slice(6);
      await page.locator(`[id="ui.panelToggle.${id}"]`).first().click();
      await wait(2500);
      await shot(`07-panel-${id}`);
    } else if (step.startsWith("click:")) {
      const [, x, y] = step.split(":");
      await page.mouse.click(Number(x), Number(y));
      await wait(2000);
      await shot(`08-click-${x}-${y}`);
    } else if (step.startsWith("drag:")) {
      const [, x1, y1, x2, y2] = step.split(":").map(Number);
      await page.mouse.move(x1, y1);
      await page.mouse.down();
      await page.mouse.move((x1 + x2) / 2, (y1 + y2) / 2, { steps: 8 });
      await page.mouse.move(x2, y2, { steps: 8 });
      await page.mouse.up();
      await wait(2000);
      await shot(`09-drag-${x1}-${y1}`);
    } else if (step.startsWith("key:")) {
      await page.keyboard.press(step.slice(4));
      await wait(2000);
      await shot(`10-key-${step.slice(4).replace(/\W/g, "")}`);
    } else if (step === "eval") {
      console.log("[DEBUG] eval", await page.evaluate(process.env.EVAL ?? "document.title"));
    } else if (step === "url") {
      console.log("[DEBUG] url", page.url());
    }
  }
} catch (e) {
  console.log("[DEBUG] failed", e.message.slice(0, 600));
  await shot("zz-failure").catch(() => {});
} finally {
  console.log([...new Set(logs)].slice(0, Number(process.env.MAXLOG ?? 30)).join("\n"));
  await browser.close();
}
