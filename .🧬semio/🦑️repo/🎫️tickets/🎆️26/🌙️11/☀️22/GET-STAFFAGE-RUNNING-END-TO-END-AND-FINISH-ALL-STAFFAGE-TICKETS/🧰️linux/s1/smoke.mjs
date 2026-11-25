/** 🚦️ Smokes the built Staffage site in Chromium as a reader does, in English and German, on a desktop and on a phone viewport: catalog, detail, the three viewpoints and their sheet, the 3D model, print and downloads. Every console error, page error, failed request and response from 400 up fails the run.
 * Usage: node smoke.mjs <release-directory> <output-directory> [--only=<context>[,<context>]]
 * @see ../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs — the static host stand-in (exact files, `404.html` for misses) */
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { chromium, devices } from "playwright";
import { expect } from "playwright/test";
import { serve } from "../../../../☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/serve.mjs";

const [release, out] = process.argv.slice(2);
if (!release || !out) throw new Error("Usage: node smoke.mjs <release-directory> <output-directory> [--only=<context>]");
const only = process.argv.find((argument) => argument.startsWith("--only="))?.slice(7).split(",");
mkdirSync(out, { recursive: true });
const manifest = JSON.parse(readFileSync(resolve(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json"), "utf8"));
const local = await serve(release);
const base = new URL(local.url);
const walker = "#/humans/walking-commuter";
const WORDS = {
  en: { choose: "English", overview: "Overview", home: "Archive overview", walker: "Walking Commuter", all: "All assets", play: "Play", viewpoint: "Viewpoint", elevation: "Side elevation", front: "Front elevation", plan: "Plan (top view)", three: "Three views", model: "3D model", collections: "Collections", unavailable: /Unavailable: failed validation/u },
  de: { choose: "Deutsch", overview: "Übersicht", home: "Archivübersicht", walker: "Pendler zu Fuß", all: "Alle Assets", play: "Abspielen", viewpoint: "Blickrichtung", elevation: "Seitenansicht", front: "Vorderansicht", plan: "Draufsicht", three: "Drei Ansichten", model: "3D-Modell", collections: "Sammlungen", unavailable: /Nicht verfügbar: Prüfung fehlgeschlagen/u },
};
const SIGNATURES = {
  svg: /<svg\b[\s\S]*<metadata\b[\s\S]*walking-commuter/u,
  pdf: /^%PDF-1\.7/u,
  dxf: /^\s*999[\s\S]*AC1015/u,
  dwg: /^AC1024/u,
  glb: /^glTF/u,
  fbx: /^Kaydara FBX Binary[\s\S]*AnimationCurve/u,
  webm: /^\x1a\x45\xdf\xa3/u,
};
const CONTEXTS = [
  { id: "desktop-en", language: "en", phone: false, options: { viewport: { width: 1440, height: 900 } } },
  { id: "desktop-de", language: "de", phone: false, options: { viewport: { width: 1440, height: 900 } } },
  { id: "phone-en", language: "en", phone: true, options: { ...devices["iPhone 13"], viewport: { width: 390, height: 844 } } },
  { id: "phone-de", language: "de", phone: true, options: { ...devices["iPhone 13"], viewport: { width: 320, height: 640 } } },
].filter((context) => only === undefined || only.includes(context.id));

const results = [];
const issues = {};

async function check(context, name, run) {
  const started = Date.now();
  try {
    const note = await run();
    results.push({ context, name, status: "pass", seconds: (Date.now() - started) / 1000, ...(note === undefined ? {} : { note }) });
    console.log("[DEBUG] PASS", context, name, note ?? "");
  } catch (error) {
    const details = String(error?.message ?? error).replace(/\u001B\[[\d;]*m/gu, "").slice(0, 1500);
    results.push({ context, name, status: "fail", seconds: (Date.now() - started) / 1000, details });
    console.error("[DEBUG] FAIL", context, name, details);
  }
}

function observe(page, id) {
  const found = (issues[id] ??= []);
  page.on("pageerror", (error) => found.push("pageerror " + error.message));
  page.on("console", (message) => {
    if (message.type() === "error" || /Content Security Policy/u.test(message.text())) found.push("console " + message.text());
  });
  page.on("requestfailed", (request) => {
    const reason = request.failure()?.errorText;
    if (reason !== "net::ERR_ABORTED") found.push(reason + " " + decodeURIComponent(request.url()));
  });
  page.on("response", (response) => {
    if (response.status() >= 400) found.push(response.status() + " " + decodeURIComponent(response.url()));
  });
}

async function open(page, route = "") {
  const response = await page.goto(new URL(route, base).href, { waitUntil: "domcontentloaded", timeout: 60000 }) ?? await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  assert.equal(response.status(), 200);
}

const shot = (page, name) => page.screenshot({ path: join(out, name + ".png") });

async function detailOf(page, slug) {
  const detail = page.locator('[data-page="asset:' + slug + '"]:visible');
  await expect(detail).toBeVisible({ timeout: 60000 });
  return detail;
}

async function planar(page, slug, viewpoint, words, view = "dark-line") {
  const detail = await detailOf(page, slug);
  const figure = detail.locator('[data-viewer] [data-stage] [data-staffage-figure][data-viewpoint="' + viewpoint + '"][data-view="' + view + '"] svg').first();
  const unavailable = detail.getByText(words.unavailable).first();
  await expect(figure.or(unavailable).first()).toBeVisible({ timeout: 90000 });
  assert.equal(await unavailable.count(), 0, slug + " is unavailable: " + (await unavailable.count() ? await unavailable.first().innerText() : ""));
  await expect(figure).toBeVisible();
  const drawn = await detail.locator('[data-viewer] [data-stage] [data-staffage-figure][data-viewpoint="' + viewpoint + '"][data-view="' + view + '"]').first().evaluate((node) => {
    const svg = node.querySelector("svg");
    const box = svg?.getBoundingClientRect();
    return { width: box?.width ?? 0, height: box?.height ?? 0, marks: svg?.querySelectorAll("path, polygon, polyline, line, circle, ellipse, rect").length ?? 0 };
  });
  assert.ok(drawn.width > 20 && drawn.height > 20 && drawn.marks > 0, "drawing of " + slug + " " + viewpoint + " is " + JSON.stringify(drawn));
  return detail;
}

async function download(page, detail, format, signature = SIGNATURES[format]) {
  const button = detail.locator('[data-format="' + format + '"] button').first();
  await button.scrollIntoViewIfNeeded({ timeout: 30000 });
  await expect(button).toBeEnabled({ timeout: 30000 });
  const [file] = await Promise.all([page.waitForEvent("download", { timeout: 120000 }), button.click()]);
  const chunks = [];
  for await (const chunk of await file.createReadStream()) chunks.push(chunk);
  const payload = Buffer.concat(chunks);
  assert.match(file.suggestedFilename(), new RegExp("^walking-commuter[_.].*\\." + format + "$", "u"));
  assert.ok(signature.test(payload.toString("latin1")), "unexpected " + format + " content in " + file.suggestedFilename());
  writeFileSync(join(out, "download-" + file.suggestedFilename()), payload);
  return file.suggestedFilename() + " " + payload.length + " B";
}

let browser;
try {
  browser = await chromium.launch({ channel: "chromium", args: ["--disable-crash-reporter", "--disable-crashpad", "--enable-unsafe-swiftshader", "--ignore-gpu-blocklist"] });
  console.log("[DEBUG] Chromium", browser.version(), "serving", release, "at", base.href);
  for (const { id, language, phone, options } of CONTEXTS) {
    const words = WORDS[language];
    const context = await browser.newContext({ ...options, locale: "fr-FR", acceptDownloads: true });
    try {
      await context.addInitScript(() => document.addEventListener("securitypolicyviolation", (event) => console.error("[DEBUG] CSP " + event.violatedDirective + " at " + location.hash + " from " + event.sourceFile + ":" + event.lineNumber + ":" + event.columnNumber + " sample " + event.sample)));
      const page = await context.newPage();
      observe(page, id);
      const press = (locator) => phone ? locator.tap() : locator.click();

      await check(id, "language gate states both languages and the choice sets the document language", async () => {
        await open(page);
        await expect(page.getByRole("button", { name: "English", exact: true })).toBeVisible({ timeout: 60000 });
        await expect(page.getByRole("button", { name: "Deutsch", exact: true })).toBeVisible();
        assert.ok(!(await page.locator("html").getAttribute("lang")));
        await shot(page, id + "-0-gate");
        await press(page.getByRole("button", { name: words.choose, exact: true }));
        await expect(page.locator("html")).toHaveAttribute("lang", language);
        await expect(page).toHaveTitle(new RegExp(words.overview, "u"));
      });

      await check(id, "overview lists every category and the choice survives a reload", async () => {
        await page.reload({ waitUntil: "domcontentloaded" });
        await expect(page.locator("html")).toHaveAttribute("lang", language, { timeout: 60000 });
        const links = page.locator('a[href^="#/"]:visible');
        await expect(links.first()).toBeVisible({ timeout: 60000 });
        const hrefs = new Set(await links.evaluateAll((nodes) => nodes.map((node) => node.getAttribute("href"))));
        for (const { id: category } of manifest.categories) assert.ok(hrefs.has("#/" + category), "overview has no link to " + category);
        assert.ok(hrefs.has("#/all"), "overview has no link to all assets");
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), "overview overflows horizontally");
        await shot(page, id + "-1-overview");
        return hrefs.size + " destinations";
      });

      await check(id, "category Humans opens from the overview and lists its assets", async () => {
        const link = page.locator('a[href="#/humans"]:visible').first();
        if (phone) {
          const box = await link.boundingBox();
          assert.ok(box.height >= 44, "touch target of the category link is " + box.height + " px high");
          await link.tap();
        } else await link.press("Enter");
        await expect(page).toHaveURL(/#\/humans$/u);
        const category = page.locator('[data-page="category:humans"]:visible');
        const asset = category.locator('a[href="' + walker + '"]:visible').first();
        await asset.scrollIntoViewIfNeeded({ timeout: 60000 });
        const humans = manifest.entries.filter((entry) => entry.category === "humans").length;
        const listed = new Set(await category.locator('a[href^="#/humans/"]').evaluateAll((nodes) => nodes.map((node) => node.getAttribute("href")))).size;
        if (phone) assert.equal(listed, humans, "the phone list of Humans");
        else assert.ok(listed > 0);
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), "category overflows horizontally");
        await shot(page, id + "-2-humans");
        if (phone) await asset.tap();
        else await asset.press("Enter");
        await expect(page).toHaveURL(new RegExp(walker + "$", "u"));
        return listed + " of " + humans + " humans linked";
      });

      await check(id, "detail draws the side elevation, names the asset and plays its clip", async () => {
        const detail = await planar(page, "walking-commuter", "elevation", words);
        await expect(detail.getByRole("heading", { name: words.walker, exact: true }).first()).toBeVisible();
        const viewer = detail.locator("[data-viewer]");
        const play = viewer.getByRole("button", { name: words.play, exact: true });
        const pause = viewer.getByRole("button", { name: "Pause", exact: true });
        await expect(play.or(pause).first()).toBeVisible();
        if (await play.count()) await press(play);
        await expect(pause).toBeVisible();
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), "detail overflows horizontally");
        await shot(page, id + "-3-detail");
      });

      for (const viewpoint of ["elevation", "front", "plan"]) {
        await check(id, "viewpoint " + viewpoint + " by address", async () => {
          await open(page, walker + "?vp=" + viewpoint);
          const detail = await planar(page, "walking-commuter", viewpoint, words);
          await expect(detail.locator("[data-viewpoint-selector]").getByRole("radio", { name: words[viewpoint], exact: true }).first()).toBeChecked();
          await shot(page, id + "-4-" + viewpoint);
        });
      }

      await check(id, "viewpoint selector moves from side elevation to front elevation and plan", async () => {
        await open(page, walker);
        const detail = await planar(page, "walking-commuter", "elevation", words);
        const selector = detail.locator("[data-viewpoint-selector]").first();
        await selector.scrollIntoViewIfNeeded();
        await press(selector.getByRole("radio", { name: words.front, exact: true }));
        await expect(page).toHaveURL(/vp=front/u);
        await planar(page, "walking-commuter", "front", words);
        await press(selector.getByRole("radio", { name: words.plan, exact: true }));
        await expect(page).toHaveURL(/vp=plan/u);
        await planar(page, "walking-commuter", "plan", words);
      });

      await check(id, "three views together on one sheet", async () => {
        await open(page, walker + "?vl=all");
        const detail = await detailOf(page, "walking-commuter");
        const sheet = detail.locator("[data-three-views]").first();
        await expect(sheet).toBeVisible({ timeout: 90000 });
        for (const viewpoint of ["plan", "elevation", "front"]) {
          const cell = sheet.locator('figure[data-viewpoint="' + viewpoint + '"]');
          await expect(cell).toHaveAttribute("data-available", "true");
          await expect(cell.locator("svg").first()).toBeVisible({ timeout: 60000 });
        }
        await shot(page, id + "-5-three-views");
      });

      await check(id, "3D model renders on a canvas", async () => {
        await open(page, walker + "?vp=model");
        const detail = await detailOf(page, "walking-commuter");
        const stage = detail.locator('[data-model-stage][data-state="ready"]');
        await expect(stage).toBeVisible({ timeout: 120000 });
        const canvas = stage.locator("canvas").first();
        await expect(canvas).toBeVisible();
        await page.waitForTimeout(1500);
        const picture = await canvas.screenshot({ path: join(out, id + "-6-model-canvas.png") });
        const colours = await page.evaluate(async (data) => {
          const image = new Image();
          image.src = "data:image/png;base64," + data;
          await image.decode();
          const canvas = document.createElement("canvas");
          canvas.width = image.width;
          canvas.height = image.height;
          const context = canvas.getContext("2d");
          context.drawImage(image, 0, 0);
          const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
          const seen = new Set();
          for (let at = 0; at < pixels.length; at += 4 * 7) seen.add((pixels[at] << 16) | (pixels[at + 1] << 8) | pixels[at + 2]);
          return seen.size;
        }, picture.toString("base64"));
        assert.ok(colours >= 4, "the model canvas shows " + colours + " colours");
        await shot(page, id + "-6-model");
        return colours + " colours on the canvas";
      });

      await check(id, "print: PDF sheet of the drawing and the browser's own print", async () => {
        await open(page, walker + "?pose=standing");
        const detail = await planar(page, "walking-commuter", "elevation", words);
        const exported = await download(page, detail, "pdf");
        assert.match(exported, /_standing_elevation_/u);
        await page.emulateMedia({ media: "print" });
        const printed = await page.pdf({ format: "A4" });
        await page.emulateMedia({ media: null });
        assert.match(printed.subarray(0, 8).toString("latin1"), /^%PDF-/u);
        writeFileSync(join(out, id + "-7-browser-print.pdf"), printed);
        return exported + "; browser print " + printed.length + " B";
      });

      await check(id, "downloads: " + (phone ? "SVG" : "SVG, DXF, DWG, GLB"), async () => {
        const detail = await planar(page, "walking-commuter", "elevation", words);
        const notes = [];
        for (const format of phone ? ["svg"] : ["svg", "dxf", "dwg", "glb"]) notes.push(await download(page, detail, format));
        return notes.join("; ");
      });

      if (!phone) {
        await check(id, "downloads: animated FBX and WebM of the walking clip", async () => {
          await open(page, walker + "?clip=walk-cycle");
          const detail = await planar(page, "walking-commuter", "elevation", words);
          await expect(detail.locator('[data-format="fbx"]').getByRole("combobox")).toHaveValue("walk-cycle");
          const notes = [await download(page, detail, "fbx")];
          await expect(detail.locator('[data-format="webm"]')).toHaveAttribute("data-state", "available");
          notes.push(await download(page, detail, "webm"));
          return notes.join("; ");
        });

        await check(id, "one asset of every category draws its side elevation", async () => {
          const drawn = [];
          for (const { id: category } of manifest.categories) {
            const entry = manifest.entries.find((candidate) => candidate.category === category);
            await open(page, "#/" + category + "/" + entry.id);
            await planar(page, entry.id, "elevation", words);
            drawn.push(entry.id);
          }
          await shot(page, id + "-8-last-category");
          return drawn.join(", ");
        });

        await check(id, "collections, comparison and playground open", async () => {
          await open(page, "#/collections");
          await expect(page.locator('[data-page="collections"]:visible').getByRole("heading", { name: words.collections, exact: true })).toBeVisible({ timeout: 60000 });
          await open(page, "#/compare?p=walking-commuter~dark-line&p=walking-commuter~thermal");
          const compare = page.locator('[data-page="compare"]:visible');
          await expect(compare.locator('[data-compare-figure="0"] [data-view="dark-line"] svg').first()).toBeVisible({ timeout: 90000 });
          await expect(compare.locator('[data-compare-figure="1"] [data-view="thermal"] svg').first()).toBeVisible({ timeout: 90000 });
          await shot(page, id + "-9-compare");
          await open(page, "#/playground");
          await expect(page.locator('[data-page="playground"]:visible')).toBeVisible({ timeout: 60000 });
        });
      }

      await check(id, "all assets: " + (id === "desktop-en" ? "every card loads and none is unavailable" : "the index lists the catalog"), async () => {
        await open(page, "#/all");
        const all = page.locator('[data-page="all"]:visible');
        await expect(all.getByRole("heading", { name: words.all, exact: true })).toBeVisible({ timeout: 60000 });
        await expect(all.locator('a[href="' + walker + '"]').first()).toBeAttached({ timeout: 60000 });
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), "all assets overflows horizontally");
        await shot(page, id + "-10-all");
        if (phone) {
          const listed = new Set(await all.locator("[data-staffage-asset-list] a[href^='#/']").evaluateAll((nodes) => nodes.map((node) => node.getAttribute("href")))).size;
          assert.equal(listed, manifest.entries.length, "the phone index of all assets");
          return listed + " assets listed";
        }
        if (id !== "desktop-en") return;
        const cards = all.locator('[data-card^="asset:"]');
        await expect(cards).toHaveCount(manifest.entries.length, { timeout: 180000 });
        await expect(cards.locator('[aria-busy="true"]')).toHaveCount(0, { timeout: 900000 });
        const unavailable = await cards.filter({ has: page.getByText(/^(?:Unavailable|Not loaded|Not authored)$/u) }).evaluateAll((nodes) => nodes.map((node) => node.getAttribute("data-card").slice("asset:".length) + " " + (node.querySelector("[data-notice]")?.textContent ?? "").slice(0, 160)));
        assert.equal(unavailable.length, 0, "unavailable catalog assets: " + unavailable.join("\n"));
        await shot(page, id + "-10-all-loaded");
        return manifest.entries.length + " cards, 0 unavailable";
      });
    } finally {
      await context.close();
    }
    await check(id, "no console error, page error, failed request or response from 400 up", () => assert.deepEqual([...new Set(issues[id] ?? [])], []));
  }
} finally {
  await browser?.close();
  local.server.close();
}
const failed = results.filter((result) => result.status === "fail");
writeFileSync(join(out, "s1-smoke.json"), JSON.stringify({ release, chromium: "playwright chromium", results, issues }, null, 2) + "\n");
console.log("[DEBUG] " + (results.length - failed.length) + " of " + results.length + " checks passed");
if (failed.length > 0) process.exitCode = 1;
