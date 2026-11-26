/** 🏪️ Checks the published Marketplace transport capability from real Chromium. */
const { createRequire } = require("node:module");
const { resolve, join } = require("node:path");
const root = resolve(__dirname, "../../../../../../..");
const app = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium, expect } = app("@playwright/test");

(async () => {
  const [origin, mode = "static"] = process.argv.slice(2);
  const base = new URL(origin);
  if (!["cdn.localhost", "127.0.0.1", "localhost"].includes(base.hostname)) throw Error("Use a private release origin");
  if (!["static", "dev"].includes(mode)) throw Error("Expected static or dev Marketplace");
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({locale:"en-US", viewport:{width:1440, height:900}});
    const writes = [];
    page.on("request", request => {
      if (!["POST","DELETE"].includes(request.method())) return;
      const url = new URL(request.url());
      if (url.origin === base.origin && url.pathname.endsWith("/install")) writes.push(request.method());
    });
    const pane = new URL("/#koordinator", base);
    await page.goto(pane.href, {waitUntil:"domcontentloaded"});
    const shell=page.locator('[data-shell-id="koordinator"][data-shell-ready]');
    await expect(shell).toHaveCount(1,{timeout:30_000});
    const skip = shell.locator('[id$="ui.introduction.skip"]:visible').first();
    if (await skip.isVisible()) {
      await page.keyboard.press("Escape");
      await expect(skip).toBeHidden({timeout:5_000});
    }
    await shell.locator('[data-tab-id="framework.marketplace"]:visible').first().click({timeout:10_000});
    const panel = shell.locator('[id$="framework.marketplace.extension.cad-extension-aec-building.enable"]').first();
    if (mode === "static") {
      await expect(shell.locator('[id*="framework.marketplace.extensions.install."]')).toHaveCount(0);
      await expect(shell.locator('[id*="framework.marketplace.extension."][id$=".uninstall"]')).toHaveCount(0);
    } else await expect(shell.locator('[id$="framework.marketplace.extensions.install.url"]').first()).toBeVisible();
    await expect(panel).toBeVisible({timeout:10_000});
    await expect(panel).toBeEnabled();
    const label = async () => (await panel.getAttribute("aria-label")) ?? (await panel.getAttribute("title")) ?? (await panel.textContent());
    const before = await label();
    if (!before?.trim()) throw Error("Bundled extension toggle has no label");
    await panel.click();
    await expect.poll(label).not.toBe(before);
    if (writes.length) throw Error("Unexpected extension mutations in the static action path: "+writes.join(","));
    console.log(JSON.stringify({mode, extensionTransportControls:mode==="dev", bundledToggle:"cad-extension-aec-building", sameOriginExtensionWrites:writes.length}));
  } finally { await browser.close(); }
})().catch(error=>{ console.error(error); process.exitCode=1; });
