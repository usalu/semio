const { createRequire } = require("node:module");
const { join } = require("node:path");
const root = process.cwd();
const project = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = project("@playwright/test");
(async () => {
  const [url, seconds = "25"] = process.argv.slice(2);
  const browser = await chromium.launch();
  const context = await browser.newContext({ locale: "en-US" });
  await context.route(u => u.pathname.endsWith("/semio_hub_demonstrator_component.js"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const env = "const { getEnvironment } = environment;";
    await route.fulfill({ response, body: source.replace(env, 'const getEnvironment = () => [...environment.getEnvironment(), ["SEMIO_RUNTIME_DIAGNOSTICS", "1"]];') });
  });
  const page = await context.newPage();
  const shapes = new Map();
  page.on("console", m => { const t = m.text(); const k = t.replace(/[0-9]+/g, "N").slice(0, 160); const e = shapes.get(k) ?? { n: 0, sample: t.slice(0, 700) }; e.n++; shapes.set(k, e); });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(Number(seconds) * 1000);
  for (const [k, e] of [...shapes].sort((a, b) => b[1].n - a[1].n).slice(0, 60)) console.log(e.n, "|", e.sample);
  await browser.close();
})();
