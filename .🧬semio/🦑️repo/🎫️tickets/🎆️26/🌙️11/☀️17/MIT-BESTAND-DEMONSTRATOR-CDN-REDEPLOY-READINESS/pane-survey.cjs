/** 🔭️ Opens every demonstrator pane of a served site, then records a screenshot, console errors and the visible text per pane as JSON. Usage: node pane-survey.cjs <origin> <out-directory> <locale> <seconds> [pane...] */
const { createRequire } = require("node:module");
const { resolve, join } = require("node:path");
const fs = require("node:fs");
const root = resolve(__dirname, "../../../../../../..");
const appRequire = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = appRequire("@playwright/test");
const [origin, out, locale, seconds, ...chosen] = process.argv.slice(2);
const panes = chosen.length ? chosen : ["koordinator", "generator", "aggregator", "aussuchen", "bearbeiten", "verfolgen", "energie", "statik"];
(async () => {
  fs.mkdirSync(out, { recursive: true });
  const browser = await chromium.launch({ args: [process.platform === "darwin" ? "--use-angle=metal" : "--use-angle=gl", "--enable-gpu", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] }), results = [];
  for (const pane of panes) {
    const page = await (await browser.newContext({ locale: locale === "de" ? "de-DE" : "en-US", viewport: { width: 1440, height: 900 } })).newPage(), errors = [], warnings = new Map();
    page.on("pageerror", error => errors.push(`pageerror ${String(error).slice(0, 600)}`));
    page.on("crash", () => errors.push("page crashed"));
    page.on("console", message => { const text = message.text().slice(0, 600); if (message.type() === "error") errors.push(text); else if (message.type() === "warning") warnings.set(text.slice(0, 120), (warnings.get(text.slice(0, 120)) ?? 0) + 1); });
    const start = Date.now();
    await page.goto(`${origin}/#${pane}`).catch(error => errors.push(`goto ${error.message}`));
    await page.waitForTimeout(Number(seconds) * 500);
    await page.locator('[id="ui.introduction.skip"]').first().click({ timeout: 3000 }).catch(() => {});
    await page.waitForTimeout(Number(seconds) * 500);
    await page.screenshot({ path: join(out, `${pane}-${locale}.png`), timeout: 20000 }).catch(error => errors.push(`screenshot ${error.message.slice(0, 200)}`));
    const text = await Promise.race([page.evaluate(() => document.body.innerText.slice(0, 2500)), new Promise((_, reject) => setTimeout(() => reject(new Error("page unresponsive for 15 s")), 15000))]).catch(error => `evaluate ${error.message.slice(0, 200)}`);
    results.push({ pane, locale, milliseconds: Date.now() - start, errors: [...new Set(errors)].slice(0, 12), errorCount: errors.length, warnings: [...warnings].slice(0, 8), text });
    console.log(`${pane} ${locale} errors=${errors.length} ${errors[0]?.slice(0, 220) ?? ""}`);
    fs.writeFileSync(join(out, `survey-${locale}.json`), JSON.stringify(results, null, 1));
    await Promise.race([page.context().close(), new Promise(resolve => setTimeout(resolve, 5000))]);
  }
  await browser.close();
})();
