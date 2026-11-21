// Opens a pane URL headless and dumps console errors plus the structured error payload as JSON. Usage: node boot-probe.cjs <url> <out.json>
const { createRequire } = require("node:module");
const { resolve, join } = require("node:path");
const root = resolve(__dirname, "../../../../../../..");
const appRequire = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = appRequire("@playwright/test");
(async () => {
  const [url, out] = process.argv.slice(2);
  const b = await chromium.launch();
  const p = await b.newPage();
  const logs = [];
  p.on("console", async (m) => {
    const t = m.text();
    if (m.type() === "error" && /boot failed/.test(t)) {
      const vals = [];
      for (const a of m.args()) vals.push(await a.evaluate((v) => v instanceof Error ? { message: v.message, payload: v.payload === undefined ? null : JSON.parse(JSON.stringify(v.payload, (k, x) => typeof x === "bigint" ? String(x) : x)) } : String(v)).catch(() => "?"));
      logs.push({ type: "boot-error", vals });
    } else if (!/turn phase/.test(t)) logs.push({ type: m.type(), text: t.slice(0, 4000) });
  });
  await p.goto(url);
  await p.waitForTimeout(45000);
  require("fs").writeFileSync(out, JSON.stringify(logs, null, 1));
  await b.close();
})();
