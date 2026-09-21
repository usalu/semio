import { chromium } from "playwright";
const b = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const p = await (await b.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await p.goto("http://127.0.0.1:6190/", { waitUntil: "domcontentloaded", timeout: 300000 });
await p.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180000 });
await p.locator('[data-semio-hub-sign-in=""]').first().click();
const w = p.locator("[data-semio-hub-workspace]");
await w.waitFor({ state: "visible", timeout: 60000 });
await w.locator('input[type="email"]').fill("user1@semio.dev");
await w.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
await w.locator('button[type="submit"][aria-label="Sign in"]').click();
await p.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120000 });
await p.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click().catch(() => {});
for (let i = 0; i < 24; i++) { await p.waitForTimeout(5000); if (await p.evaluate(() => /GM1 Shared Map/.test(document.body.innerText))) break; }
console.log(JSON.stringify(await p.evaluate(() => {
  const out = [];
  for (const el of document.querySelectorAll("*")) {
    const t = (el.textContent ?? "").trim();
    if (t === "open" && el.children.length <= 2) out.push({ tag: el.tagName, id: el.id, cls: String(el.className).slice(0,60), aria: el.getAttribute("aria-label"), title: el.getAttribute("title") });
  }
  const host = document.querySelector(".semio-table-host, [data-slot='window-body']");
  const rows = host ? [...host.querySelectorAll("[id]")].map(e => e.id).slice(0, 60) : [];
  return { opens: out.slice(0, 8), rowIds: rows };
}), null, 1));
await b.close();
