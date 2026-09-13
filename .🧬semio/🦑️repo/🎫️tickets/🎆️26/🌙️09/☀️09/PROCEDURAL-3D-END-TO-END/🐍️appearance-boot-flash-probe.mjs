/** 🌓️ Runtime proof for the pre-paint appearance bootstrap — the dark→light flash on reload.
 *
 * Measures the FIRST PAINTED background of a dark-configured session two ways against the same live
 * playground:
 *
 * - `before` — the page exactly as the server sends it today. If the served head still carries the
 *   retired `ui.chrome.appearance` read, a session whose `semio.os.config` says `dark` boots LIGHT,
 *   which is the defect (`📓️react-i18n-a11y-customization-2026-09-13.md` §3.2).
 * - `after` — the same page with the head's two boot scripts replaced, over the wire, by the ones the
 *   repo now ships (`PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT` / `..._THEME_SCRIPT`, read out of the
 *   source at probe time, never retyped). This is the only way to observe the fix without restarting
 *   a dev server owned by another lane: a Vite plugin's module is loaded once at server start.
 *
 * The measurement is taken at `domcontentloaded`, i.e. after the head ran and before the React bundle
 * has mounted anything — exactly the frame a user sees flash.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=boot-1 bun 🐍️appearance-boot-flash-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const outDir = join(import.meta.dir, "🗑️generated", "graph-keyboard", process.env.SEMIO_PROBE_OUT ?? "appearance-boot");
mkdirSync(outDir, { recursive: true });

const viteSource = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts"), "utf8");
const constantOf = (name) => {
  const open = viteSource.indexOf(`export const ${name} = \``);
  if (open < 0) throw new Error(`${name} is not exported by the styling vite builder`);
  const from = open + `export const ${name} = \``.length;
  return viteSource.slice(from, viteSource.indexOf("`;", from));
};
const shippedAppearance = constantOf("PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT");
const shippedTheme = constantOf("PLAYGROUND_PLAY_BOOT_THEME_SCRIPT");

/** 🌓️ A dark session exactly as `commitUiPreferencesConfigMutation` persists it. */
const darkConfig = JSON.stringify({
  version: 1,
  preferences: { "os.config.ui-preferences": JSON.stringify({ version: 1, events: [{ mutation: "setAppearance", appearance: "dark" }] }) },
  namedLayouts: {},
  dockLayouts: { apps: {} },
  dockUi: { apps: {} },
  windowPanes: { apps: {} },
});

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const results = [];

const measure = async (label, rewrite) => {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, colorScheme: "light" });
  const page = await context.newPage();
  await page.addInitScript((config) => {
    try {
      localStorage.setItem("semio.os.config", config);
    } catch {}
  }, darkConfig);
  let servedAppearanceKey = null;
  if (rewrite) {
    await page.route(url, async (route) => {
      const response = await route.fetch();
      let body = await response.text();
      servedAppearanceKey = body.includes("ui.chrome.appearance") ? "ui.chrome.appearance" : body.includes("semio.os.config") ? "semio.os.config" : "none";
      body = body.replace(/<script>\(function\(\)\{var d=document\.documentElement[\s\S]*?<\/script>/, `<script>${shippedAppearance}</script>`);
      body = body.replace(/<script>\(function\(\)\{try\{var raw=localStorage\.getItem\("ui\.chrome\.theme\.snapshot"\)[\s\S]*?<\/script>/, `<script>${shippedTheme}</script>`);
      await route.fulfill({ response, body, headers: { ...response.headers(), "content-type": "text/html" } });
    });
  }
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const painted = await page.evaluate(() => ({
    htmlDarkClass: document.documentElement.classList.contains("dark"),
    dataUiAppearance: document.documentElement.dataset.uiAppearance ?? null,
    htmlColorScheme: document.documentElement.style.colorScheme || null,
    bodyBackground: getComputedStyle(document.body).backgroundColor,
    bodyInlineBackground: document.body.style.backgroundColor || null,
    headScriptReads: [...document.querySelectorAll("head script:not([src])")].map((s) => (s.textContent ?? "").includes("semio.os.config") ? "semio.os.config" : (s.textContent ?? "").includes("ui.chrome.appearance") ? "ui.chrome.appearance" : (s.textContent ?? "").includes("ui.chrome.theme.snapshot") ? "ui.chrome.theme.snapshot" : "other"),
    storedConfig: (() => { try { return localStorage.getItem("semio.os.config"); } catch { return null; } })(),
  }));
  await page.screenshot({ path: join(outDir, `${label}.png`) });
  const entry = { label, servedAppearanceKey, ...painted };
  results.push(entry);
  console.log(`[DEBUG] ${label} ${JSON.stringify(entry).slice(0, 700)}`);
  await context.close();
};

await measure("1-served-head-as-is", false);
await measure("2-shipped-head", true);

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
const before = results[0];
const after = results[1];
console.log(`[DEBUG] VERDICT before=${before.dataUiAppearance}/${before.bodyBackground} after=${after.dataUiAppearance}/${after.bodyBackground}`);
await browser.close();
