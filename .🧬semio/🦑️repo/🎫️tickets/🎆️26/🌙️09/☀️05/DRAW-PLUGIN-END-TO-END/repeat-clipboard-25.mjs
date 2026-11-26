/** 📋️ Exercises back-to-back Draw clipboard shortcuts while replacing the active example between rounds. */
import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
try {
  const context = await browser.newContext({ locale: "de", viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  page.on("console", message => {
    if (message.type() === "error" || message.type() === "warning" && message.text().includes("refused:")) console.error("[DEBUG]", message.text());
  });
  page.on("pageerror", error => console.error("[DEBUG]", error.message));
  await page.goto("http://127.0.0.1:6064/?plugin=draw", { waitUntil: "domcontentloaded" });
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  await picker.getByText("Demo", { exact: true }).waitFor({timeout:90000});
  const status = page.locator('[id="drawing-composite"] [data-slot="engagement-status"]');
  const surface = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"]').first();
  const check = async (layers, selected) => {
    if (!await status.isVisible()) await page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first().click();
    const expected = new RegExp('^'+layers+' Ebenen? · '+selected+' ausgewählt');
    try { await status.filter({ hasText:expected }).waitFor({timeout:30000}); }
    catch (error) { console.error("[DEBUG] Expected", layers,selected,"observed",await status.textContent()); throw error; }
  };
  const panel = page.locator('[data-slot="panel"][id="framework.panelTab.framework.panel.artifact"]').first();
  const choose = async name => { await picker.click(); await page.getByRole("option", {name, exact:true}).click(); };
  const mod = process.platform === "darwin" ? "Meta" : "Control";
  for (let round=1;round<=4;round++) {
    await choose("Leere Zeichnung"); await check(0,0);
    await choose("Demo"); await check(1,0);
    if (!await panel.isVisible()) await page.locator('button[id="framework.panel.artifact"]').first().click();
    await panel.locator('[role="treeitem"][id="panel:framework.panel.artifact/emblem-orange"]').click();
    await check(1,1);
    await surface.focus();
    await page.keyboard.press(mod+"+c");
    await page.keyboard.press(mod+"+v");
    await check(2,1);
    await surface.focus();
    await page.keyboard.press(mod+"+x");
    await check(1,0);
    console.log("[DEBUG] Clipboard round",round,"passed");
  }
} finally { await browser.close(); }
