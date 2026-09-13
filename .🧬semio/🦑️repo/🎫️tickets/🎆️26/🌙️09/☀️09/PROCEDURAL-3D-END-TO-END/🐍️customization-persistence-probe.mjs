/** 🎨 Does a user's customization of the generation3d editor survive a reload? Flips locale →
 * German, appearance → Dark, and closes a dock panel, then reloads the SAME browser context and
 * reports what came back. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "persistence");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();

async function boot(p, tag = "") {
  let hit = false;
  for (let i = 0; i < 150; i++) {
    await p.waitForTimeout(1000);
    if (await p.locator('[data-surface-id="window:procedural-preview"]').count()) { hit = true; console.log(`[DEBUG] boot${tag} surface after ${i + 1}s`); break; }
  }
  if (!hit) console.log(`[DEBUG] boot${tag} TIMED OUT waiting for preview surface`);
  await p.waitForTimeout(6000);
  return hit;
}
async function openSettings(p) {
  if (!(await p.locator("#framework\\.settings\\.language").count())) { await p.locator("#framework\\.settings").first().click(); await p.waitForTimeout(1500); }
}
async function setCombo(p, id, optionText) {
  const c = p.locator(`#${id.replace(/\./g, "\\.")}`).filter({ has: p.locator(":scope") }).last();
  await p.locator(`button#${id.replace(/\./g, "\\.")}`).first().click();
  await p.waitForTimeout(800);
  const opt = p.locator("[role='option']").filter({ hasText: optionText }).first();
  const n = await opt.count();
  if (n) await opt.click(); else console.log(`[DEBUG] no option ${optionText} for ${id}`);
  await p.waitForTimeout(2000);
  return n > 0;
}
async function snapshot(p, tag) {
  const s = await p.evaluate(() => ({
    html: document.documentElement.className,
    dataTheme: document.documentElement.getAttribute("data-theme"),
    colorScheme: getComputedStyle(document.documentElement).colorScheme,
    rootBg: getComputedStyle(document.documentElement).backgroundColor,
    appearanceAttrs: Object.fromEntries([...document.documentElement.attributes].map((a) => [a.name, a.value.slice(0, 120)])),
    osConfig: (() => { try { return JSON.parse(localStorage.getItem("semio.os.config") ?? "{}"); } catch { return "<unparsable>"; } })(),
    bg: getComputedStyle(document.body).backgroundColor,
    // 🎨️ The appearance a user picked is painted by the SCOPE element, never by `body` — reading the
    // body's background reported "unchanged" for a flip that had in fact applied.
    scopeAppearance: document.querySelector(".semio-scope")?.getAttribute("data-ui-appearance") ?? null,
    lang: document.documentElement.lang,
    text: document.body.innerText.replace(/\s+/g, " ").slice(0, 900),
    localStorageKeys: (() => { try { return Object.keys(localStorage); } catch { return ["<blocked>"]; } })(),
    localStorageDump: (() => { try { return Object.fromEntries(Object.keys(localStorage).map((k) => [k, (localStorage.getItem(k) ?? "").slice(0, 300)])); } catch { return {}; } })(),
    idb: typeof indexedDB !== "undefined",
  }));
  writeFileSync(join(outDir, `${tag}.json`), JSON.stringify(s, null, 2));
  await p.screenshot({ path: join(outDir, `${tag}.png`) });
  return s;
}

await page.goto(url, { waitUntil: "domcontentloaded" });
await boot(page, "-1");
const before = await snapshot(page, "1-baseline-en");
console.log("[DEBUG] baseline lsKeys", JSON.stringify(before.localStorageKeys));
console.log("[DEBUG] baseline bg", before.bg, "text head", before.text.slice(0, 140));

await openSettings(page);
const okLang = await setCombo(page, "framework.settings.language", /Deutsch/);
const okAppearance = await setCombo(page, "framework.settings.appearance", /Dunkel|Dark/);
console.log("[DEBUG] set language", okLang, "appearance", okAppearance);
const after = await snapshot(page, "2-customized-de-dark");
console.log("[DEBUG] customized bg", after.bg, "text head", after.text.slice(0, 160));
console.log("[DEBUG] customized lsKeys", JSON.stringify(after.localStorageKeys));

await page.reload({ waitUntil: "domcontentloaded" });
await boot(page, "-reload");
const reloaded = await snapshot(page, "3-after-reload");
console.log("[DEBUG] reloaded bg", reloaded.bg, "text head", reloaded.text.slice(0, 200));
console.log("[DEBUG] reloaded lsKeys", JSON.stringify(reloaded.localStorageKeys));

const localeKept = /Dokument|Katalog|Bearbeiten|Workflow/.test(reloaded.text);
const appearanceKept = after.scopeAppearance === "dark" && reloaded.scopeAppearance === after.scopeAppearance;
console.log("=== VERDICT ===");
console.log("localeSurvivedReload", localeKept);
console.log("appearanceSurvivedReload", appearanceKept, `(baseline=${before.scopeAppearance} customized=${after.scopeAppearance} reloaded=${reloaded.scopeAppearance})`);
writeFileSync(join(outDir, "verdict.json"), JSON.stringify({ localeKept, appearanceKept, okLang, okAppearance, appearanceBefore: before.scopeAppearance, appearanceAfter: after.scopeAppearance, appearanceReloaded: reloaded.scopeAppearance, before: before.bg, after: after.bg, reloaded: reloaded.bg, lsKeysAfter: after.localStorageKeys, lsKeysReloaded: reloaded.localStorageKeys, lsDumpAfter: after.localStorageDump }, null, 2));
console.log("DONE");
await browser.close();
