/** 🚦️ Exercises Staffage in Chromium as a reader does. Usage: node smoke.mjs <release-directory-or-site-url>. */
import assert from "node:assert/strict";
import { chromium, devices } from "playwright";
import { expect } from "playwright/test";
import { serve } from "./serve.mjs";
import { exerciseRecovery } from "./smoke-recovery.mjs";

const release = process.argv[2];
if (!release) throw new Error("Usage: node smoke.mjs <release-directory-or-site-url>");
const remote = /^https?:\/\//.test(release);
const local = remote ? undefined : await serve(release);
const base = remote ? new URL(release + (release.endsWith("/") ? "" : "/")) : new URL(local.url);
const walker = "#/humans/walking-commuter";
const formats = {
  svg: /<svg\b[\s\S]*<metadata\b[\s\S]*walking-commuter/u,
  pdf: /^%PDF-1\.7/u,
  dxf: /^\s*999[\s\S]*AC1015/u,
  dwg: /^AC1024/u,
  glb: /^glTF/u,
  fbx: /^Kaydara FBX Binary[\s\S]*AnimationCurve/u,
  webm: /^\x1a\x45\xdf\xa3/u,
};
const browserIssues = [];
const failures = [];

async function check(name, run) {
  try {
    await run();
    console.log("[DEBUG] PASS", name);
  } catch (error) {
    const details = error.message.replace(/\u001B\[[\d;]*m/gu, "").slice(0, 1200);
    failures.push(name + "\n" + details);
    console.error("[DEBUG] FAIL", name, details);
  }
}

function observe(page) {
  page.on("pageerror", (error) => browserIssues.push("pageerror " + error.message));
  page.on("console", (message) => {
    if (message.type() === "error" || /Content Security Policy/u.test(message.text())) {
      browserIssues.push("console " + message.text());
    }
  });
  page.on("requestfailed", (request) => {
    const reason = request.failure()?.errorText;
    if (reason !== "net::ERR_ABORTED") browserIssues.push(reason + " " + request.url());
  });
  page.on("response", (response) => {
    if (response.status() >= 400) browserIssues.push(response.status() + " " + response.url());
  });
}

async function open(page, route = "") {
  const response = await page.goto(new URL(route, base).href, { waitUntil: "domcontentloaded", timeout: 30000 });
  if (response !== null) assert.equal(response.status(), 200);
}

async function drawing(page, view = "dark-line") {
  const detail = page.locator('[data-page="asset:walking-commuter"]:visible');
  const figure = detail.locator('[data-viewer] [data-stage] [data-view="' + view + '"] svg').first();
  const unavailable = detail.getByText(/Unavailable: failed validation|Nicht verfügbar: Prüfung fehlgeschlagen/u).first();
  await expect(figure.or(unavailable).first()).toBeVisible({ timeout: 25000 });
  assert.equal(await unavailable.count(), 0, "Walking Commuter failed catalog validation");
  await expect(figure).toBeVisible();
  return detail;
}

async function exportFile(page, detail, format) {
  const button = detail.locator('[data-format="' + format + '"] button').first();
  await expect(button).toBeEnabled({ timeout: 15000 });
  const [file] = await Promise.all([page.waitForEvent("download", { timeout: 30000 }), button.click()]);
  assert.match(file.suggestedFilename(), new RegExp("^walking-commuter[_.].+\\." + format + "$", "u"));
  const chunks = [];
  for await (const chunk of await file.createReadStream()) chunks.push(chunk);
  const payload = Buffer.concat(chunks);
  assert.ok(formats[format].test(payload.toString("latin1")), "Unexpected " + format + " signature in " + file.suggestedFilename());
  return payload;
}

async function verifyMotion(context, bytes) {
  const playback = await context.newPage();
  try {
    const result = await playback.evaluate(async (base64) => {
      const video = document.createElement("video");
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 64;
      const context = canvas.getContext("2d");
      const bytes = Uint8Array.from(atob(base64), (value) => value.charCodeAt(0));
      const url = URL.createObjectURL(new Blob([bytes], { type: "video/webm" }));
      const listeners = new AbortController();
      const once = (type) => new Promise((done, fail) => {
        const options = { once: true, signal: listeners.signal };
        video.addEventListener(type, done, options);
        video.addEventListener("error", () => fail(new Error(video.error?.message)), options);
      });
      const frameAt = async (time) => {
        const seeked = once("seeked");
        video.currentTime = time;
        await seeked;
        context.clearRect(0, 0, 64, 64);
        context.drawImage(video, 0, 0, 64, 64);
        return context.getImageData(0, 0, 64, 64).data;
      };
      const decode = async () => {
        const loaded = once("loadeddata");
        video.muted = true;
        video.src = url;
        await loaded;
        const first = await frameAt(0.15);
        const later = await frameAt(0.65);
        return { width: video.videoWidth, height: video.videoHeight, motion: first.some((byte, index) => byte !== later[index]) };
      };
      let timer;
      try {
        const timeout = new Promise((_, fail) => {
          timer = setTimeout(() => fail(new Error("Exported WebM did not decode in time")), 15000);
        });
        return await Promise.race([decode(), timeout]);
      } finally {
        clearTimeout(timer);
        listeners.abort();
        URL.revokeObjectURL(url);
      }
    }, bytes.toString("base64"));
    assert.ok(result.width > 0 && result.height > 0 && result.motion, "Exported WebM did not contain two different decoded frames");
  } finally {
    await playback.close();
  }
}

let browser;
try {
  const executablePath = process.env.STAFFAGE_CHROME_EXECUTABLE;
  browser = await chromium.launch({
    ...(executablePath ? { executablePath } : { channel: process.env.STAFFAGE_CHROME_CHANNEL ?? "chrome" }),
    args: ["--disable-crash-reporter", "--disable-crashpad"],
  });
  const recoveryOnly = process.argv.includes("--recovery-only");
  for (const [device, language, labels, viewport] of recoveryOnly ? [] : [
    ["desktop", "en", { choose: "English", walker: "Walking Commuter", all: "All assets", overview: "Overview" }, { viewport: { width: 1440, height: 900 } }],
    ["phone", "de", { choose: "Deutsch", walker: "Pendler zu Fuß", all: "Alle Assets", overview: "Übersicht" }, { ...devices["iPhone 13"], viewport: { width: 320, height: 640 } }],
  ]) {
    const context = await browser.newContext({ ...viewport, locale: "fr-FR", acceptDownloads: true, reducedMotion: "reduce" });
    try {
      const page = await context.newPage();
      observe(page);
      await check(device + " bilingual language choice and catalog navigation", async () => {
        await open(page);
        const choose = page.getByRole("button", { name: labels.choose, exact: true });
        await expect(page.getByRole("button", { name: "English", exact: true })).toBeVisible();
        await expect(page.getByRole("button", { name: "Deutsch", exact: true })).toBeVisible();
        assert.ok(!(await page.locator("html").getAttribute("lang")));
        await choose.click();
        await expect(page.locator("html")).toHaveAttribute("lang", language);
        await expect(page).toHaveTitle(new RegExp(labels.overview, "u"));
        const category = device === "phone" && !remote
          ? page.getByRole("navigation", { name: "Archivübersicht", exact: true }).locator('a[href="#/humans"]')
          : page.locator('a[href="#/humans"]:visible').first();
        if (device === "phone" && !remote) {
          await expect(category).toBeVisible();
          assert.ok((await category.boundingBox()).height >= 44);
          await category.tap();
        } else {
          await category.press("Enter");
        }
        await expect(page).toHaveURL(/#\/humans$/u);
        const asset = page.locator('[data-page="category:humans"] a[href="' + walker + '"]:visible').first();
        if (device === "phone" && !remote) await asset.tap({ timeout: 20000 });
        else await asset.press("Enter", { timeout: 20000 });
        await expect(page).toHaveURL(new RegExp(walker + "$", "u"));
      });
      await check(device + " authored drawing and playback", async () => {
        await open(page, walker);
        const detail = await drawing(page);
        await expect(detail.getByRole("heading", { name: labels.walker, exact: true }).first()).toBeVisible();
        const play = detail.locator("[data-viewer]").getByRole("button", { name: language === "en" ? "Play" : "Abspielen", exact: true });
        await play.click();
        await expect(detail.locator("[data-viewer]").getByRole("button", { name: "Pause", exact: true })).toBeVisible();
      });
      for (const view of ["thermal", "forces"]) {
        await check(device + " direct " + view + " drawing", async () => {
          await open(page, walker + "?view=" + view);
          await page.reload({ waitUntil: "domcontentloaded" });
          await expect(page.locator("html")).toHaveAttribute("lang", language);
          await drawing(page, view);
        });
      }
      await check(device + " all assets", async () => {
        await open(page, "#/all");
        const all = page.locator('[data-page="all"]:visible');
        await expect(all.getByRole("heading", { name: labels.all, exact: true })).toBeVisible();
        await expect(all.locator('a[href="' + walker + '"]:visible').first()).toBeVisible({ timeout: 20000 });
        assert.ok(await all.locator('a[href^="#/humans/"]').count() > 10);
        assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
        if (device === "desktop") {
          const cards = all.locator('[data-card^="asset:"]');
          await expect(cards).toHaveCount(152, { timeout: 30000 });
          await expect(cards.locator('[aria-busy="true"]')).toHaveCount(0, { timeout: 60000 });
          const unavailable = await cards.filter({
            has: page.getByText(/^(?:Unavailable|Not loaded|Not authored)$/u),
          }).evaluateAll((nodes) => nodes.map((node) => ({
            id: node.getAttribute("data-card").slice("asset:".length),
            notice: node.querySelector("[data-notice]")?.textContent,
          })));
          assert.ok(unavailable.length === 0, "Unavailable catalog assets: " + unavailable.map(({ id }) => id).join(", ") + "\n" +
            unavailable.map(({ id, notice }) => id + " " + notice).join("\n"));
        }
      });
      if (device === "desktop") {
        if (!remote) await check("desktop settings keyboard focus and Escape", async () => {
          await open(page);
          const trigger = page.getByRole("button", { name: "Display settings", exact: true });
          await trigger.press("Enter");
          const settings = page.getByRole("group", { name: "Display settings", exact: true });
          await expect(settings.getByRole("radiogroup", { name: "Text size" }).getByRole("radio", { checked: true })).toBeFocused();
          await page.keyboard.press("Tab");
          await expect(settings.getByRole("radiogroup", { name: "Motion" }).getByRole("radio", { checked: true })).toBeFocused();
          await page.keyboard.press("Escape");
          await expect(trigger).toBeFocused();
        });
        await check("desktop SVG, PDF, DXF, DWG and GLB downloads", async () => {
          await open(page, walker + "?pose=standing");
          const detail = await drawing(page);
          for (const format of ["svg", "pdf", "dxf", "dwg", "glb"]) await exportFile(page, detail, format);
        });
        await check("desktop animated walking clip FBX download", async () => {
          await open(page, walker + "?clip=walk-cycle");
          const detail = await drawing(page);
          await expect(detail.locator('[data-format="fbx"]').getByRole("combobox")).toHaveValue("walk-cycle");
          await exportFile(page, detail, "fbx");
        });
        await check("desktop walking clip WebM download and decoded motion", async () => {
          const detail = await drawing(page);
          await expect(detail.locator('[data-format="webm"]')).toHaveAttribute("data-state", "available");
          await verifyMotion(context, await exportFile(page, detail, "webm"));
        });
        await check("desktop 3D model and lazy renderer", async () => {
          await open(page, walker + "?vp=model");
          const model = page.locator('[data-page="asset:walking-commuter"]:visible [data-model-stage][data-state="ready"]');
          await expect(model).toBeVisible({ timeout: 30000 });
          await expect(model.locator("canvas")).toBeVisible();
        });
        await check("desktop shared two-pane comparison", async () => {
          await open(page, "#/compare?p=walking-commuter~dark-line&p=walking-commuter~thermal");
          const compare = page.locator('[data-page="compare"]:visible');
          await expect(compare.locator('[data-compare-figure="0"] [data-view="dark-line"] svg').first()).toBeVisible({ timeout: 25000 });
          await expect(compare.locator('[data-compare-figure="1"] [data-view="thermal"] svg').first()).toBeVisible();
        });
        await check("desktop playground section and plan share a placement", async () => {
          await open(page, "#/playground");
          const playground = page.locator('[data-page="playground"]:visible');
          await playground.getByRole("button", { name: "Place Walking Commuter on the ground line", exact: true }).click({ timeout: 25000 });
          await expect(playground.getByRole("group", { name: /^Ground line with 1 placed figure/u })).toBeVisible();
          await playground.getByRole("link", { name: "Floor plan", exact: true }).click();
          await expect(page).toHaveURL(/#\/playground\/plan(?:\?|$)/u);
          await expect(playground.getByRole("group", { name: /^Floor plan with 1 placed figure/u })).toBeVisible();
        });
        if (!remote) await check("desktop playground focus and selection are independent", async () => {
          const playground = page.locator('[data-page="playground"]:visible');
          await playground.getByRole("button", { name: "Place Walking Commuter on the ground line", exact: true }).click();
          const box = playground.getByRole("group", { name: /^Floor plan with 2 placed figures/u });
          const first = box.locator("[data-placement]").first();
          const second = box.locator("[data-placement]").last();
          await expect(second).toHaveAttribute("aria-pressed", "true");
          await page.keyboard.press("Tab");
          await first.focus();
          await expect(first).toBeFocused();
          assert.ok(await first.evaluate((button) => {
            const style = getComputedStyle(button);
            return button.matches(":focus-visible") && style.outlineStyle !== "none" && parseFloat(style.outlineWidth) >= 2;
          }));
          await expect(first).toHaveAttribute("aria-pressed", "false");
          await page.keyboard.press("Enter");
          await expect(first).toHaveAttribute("aria-pressed", "true");
          await expect(second).toHaveAttribute("aria-pressed", "false");
        });
      } else {
        await check("phone localized collections", async () => {
          await open(page, "#/collections");
          await expect(page.locator('[data-page="collections"]:visible').getByRole("heading", { name: "Sammlungen", exact: true })).toBeVisible();
        });
      }
    } finally {
      await context.close();
    }
  }
  if (local !== undefined) await exerciseRecovery(browser, release, base, check);
  await check("missing module returns an HTML 404", async () => {
    const miss = await fetch(new URL("assets/staffage-smoke-missing.js", base));
    assert.equal(miss.status, 404);
    assert.match(miss.headers.get("Content-Type") ?? "", /^text\/html/u);
    await miss.arrayBuffer();
  });
  await check("browser runtime and network", () => assert.deepEqual([...new Set(browserIssues)], []));
} finally {
  await browser?.close();
  local?.server.close();
}
if (failures.length > 0) process.exitCode = 1;
