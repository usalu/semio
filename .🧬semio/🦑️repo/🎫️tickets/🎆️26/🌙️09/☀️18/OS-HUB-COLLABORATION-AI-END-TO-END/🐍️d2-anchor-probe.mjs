// 🎯️ D2 — do the first-run tour's anchors actually resolve in the live hub workspace DOM?
// The laws only check the id GRAMMAR; this asks the real page. Usage: node 🐍️d2-anchor-probe.mjs [ui]
import { chromium } from "playwright";

const ui = process.argv[2] ?? "http://127.0.0.1:7502";
const ANCHORS = [
  "os.hub.signIn.email",
  "os.hub.signIn.password",
  "os.hub.signIn.submit",
  "os.hub.signIn.hub",
  "os.hub.spaces.createName",
  "os.hub.spaces.createSubmit",
  "os.hub.spaces.list",
  "os.hub.invite.redeemField",
  "os.hub.invite.create",
  "os.hub.invite.role",
  "os.hub.invite.expiry",
  "os.hub.invite.copy",
];

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
try {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  await page.goto(ui, { waitUntil: "domcontentloaded" });
  await page.waitForSelector("[data-semio-hub-connection]", { timeout: 180_000 });
  await page.click('[data-semio-hub-sign-in=""]');
  await page.waitForSelector("[data-semio-hub-workspace]", { timeout: 30_000 });
  await page.waitForTimeout(1000);

  const found = await page.evaluate(
    (ids) => ids.map((id) => ({ id, n: document.querySelectorAll(`[id="${id}"], [data-element-alias~="${id}"]`).length })),
    ANCHORS,
  );
  for (const row of found) console.log(`${row.n > 0 ? "RESOLVES" : "MISSING "} ${row.id} (${row.n})`);
  console.log(`\n${found.filter((r) => r.n > 0).length}/${found.length} anchors resolve while signed out`);

  // 🔍️ What ids DOES the workspace publish? The answer is the repair list if anchors are missing.
  const published = await page.evaluate(() => {
    const root = document.querySelector("[data-semio-hub-workspace]");
    if (!root) return [];
    return [...root.querySelectorAll("[id]")].map((e) => e.id).filter((id) => id.startsWith("os.hub."));
  });
  console.log(`\n--- os.hub.* ids actually in the workspace (${published.length}) ---\n${published.join("\n")}`);
  await context.close();
} finally {
  await browser.close();
}
