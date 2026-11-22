import assert from "node:assert/strict";
import { mkdir, mkdtemp, rm, rmdir } from "node:fs/promises";
import { join } from "node:path";
import { chromium, type Page } from "playwright";
import { dismissIntroduction, seatLocale, windowIds } from "@semio-tech/framework-os-dev/testing/program-matrix";

const url = process.argv[2] ?? "http://127.0.0.1:6060/?plugin=raster";
const surfaces = '[data-view-mode][data-surface-id][data-layers-json][data-assets-json]';
const generated = join(import.meta.dir, "🗑️generated");
await mkdir(generated, { recursive: true });
const temp = await mkdtemp(join(generated, "raster-windows-"));
process.env.TMPDIR = temp;
process.env.TMP = temp;
process.env.TEMP = temp;

async function witness(page: Page) {
  const rendered = await page.locator(surfaces).evaluateAll(elements => elements.map(surface => {
    const layers = JSON.parse(surface.getAttribute("data-layers-json") ?? "[]") as { name: string; imageKey?: string }[];
    const assets = JSON.parse(surface.getAttribute("data-assets-json") ?? "{}") as Record<string, unknown>;
    return {
      mode: surface.getAttribute("data-view-mode"), id: surface.getAttribute("data-surface-id"),
      layers: layers.map(layer => ({ name: layer.name, imageKey: layer.imageKey })),
      assets,
      canvases: Array.from(surface.querySelectorAll("canvas")).map(canvas => {
        const box = canvas.getBoundingClientRect();
        return { width: canvas.width, height: canvas.height, visible: box.width > 0 && box.height > 0 };
      }),
      alerts: Array.from(surface.querySelectorAll('[role="alert"]')).map(alert => alert.textContent?.trim()),
    };
  }));
  return { locale: await page.locator("html").getAttribute("lang"), windows: await windowIds(page), rendered };
}

async function sceneFingerprint(page: Page) {
  return page.locator(surfaces).evaluateAll(elements => elements.map(surface =>
    [surface.getAttribute("data-layers-json"), surface.getAttribute("data-assets-json")]));
}

const browser = await chromium.launch({ headless: true, downloadsPath: temp, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1050 }, locale: "en-US" })).newPage();
try {
  const preview = new URL(url);
  await page.routeWebSocket(address => address.host === preview.host && address.pathname === "/" && address.searchParams.has("token"), socket => { void socket.close(); });
  const errors: string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("console", message => {
    if (message.type() !== "error" && message.type() !== "warning") return;
    if (!/validation|webgpu|raster|example|dispatch|texture/i.test(message.text())) return;
    if (errors.length < 35 && !errors.includes(message.text().slice(0, 350))) errors.push(message.text().slice(0, 350));
  });
  const response = await page.goto(url, { waitUntil: "domcontentloaded", timeout: 30_000 });
  assert(response?.ok(), "Raster preview did not return a successful response");
  console.log("[DEBUG] Raster windows preview HTTP ready");
  await page.waitForSelector("[data-semio-os-ready],[data-semio-os-error]", { timeout: 120_000, state: "attached" });
  assert.equal(await page.locator("[data-semio-os-error]").count(), 0);
  console.log("[DEBUG] Raster windows shell ready");
  await dismissIntroduction(page);
  const locale = await seatLocale(page, "en");
  console.log("[DEBUG] Raster windows locale", locale);
  const trigger = page.getByRole("combobox", { name: "Example", exact: true });
  await trigger.waitFor({ state: "visible", timeout: 20_000 });
  for (const mode of ["composite", "navigator"]) {
    await page.locator(`${surfaces}[data-view-mode="${mode}"] canvas`).first().waitFor({ state: "visible", timeout: 35_000 });
    console.log("[DEBUG] Raster canvas visible", mode);
  }
  const initial = { label: (await trigger.textContent())?.trim(), ...await witness(page) };
  console.log("[DEBUG] Raster initial declared windows", JSON.stringify({ locale, initial }));
  await trigger.press("ArrowDown");
  await page.getByRole("listbox").waitFor({ state: "visible", timeout: 10_000 });
  const options = await page.getByRole("listbox").getByRole("option").allTextContents();
  await page.keyboard.press("Escape");
  console.log("[DEBUG] Raster example catalogue", JSON.stringify(options));
  for (const label of [...options.filter(option => option.trim() !== initial.label), ...options.filter(option => option.trim() === initial.label)]) {
    const choice = label.trim();
    const before = await sceneFingerprint(page);
    await trigger.press("ArrowDown");
    await page.getByRole("listbox").getByRole("option", { name: choice, exact: true }).press("Enter");
    const result = await page.waitForFunction(({ selector, before }) => {
      const after = [...document.querySelectorAll(selector)].map(surface =>
        [surface.getAttribute("data-layers-json"), surface.getAttribute("data-assets-json")]);
      return JSON.stringify(after) !== JSON.stringify(before);
    }, { selector: surfaces, before }, { timeout: 12_000 }).then(() => true, () => false);
    console.log("[DEBUG] Raster example selection", JSON.stringify({ choice, selected: (await trigger.textContent())?.trim(), changed: result, ...await witness(page) }));
  }
  const hops = await page.evaluate(() => performance.getEntriesByName("semio.hop.invoke").flatMap(entry => {
    const detail = (entry as PerformanceMeasure).detail;
    return detail?.actionId === "setActiveExample" ? [{ duration: entry.duration, ...detail }] : [];
  }));
  console.log("[DEBUG] Raster window browser observations", JSON.stringify({ hops, errors }));
} catch (error) {
  const snapshot = await witness(page).catch(() => null);
  const status = await page.getByRole("status").allTextContents().catch(() => []);
  console.error("[DEBUG] Raster window probe stopped", JSON.stringify({ snapshot, status: status.map(text => text.trim().slice(0, 200)) }));
  throw error;
} finally {
  try { await browser.close(); }
  finally {
    await rm(temp, { recursive: true, force: true });
    await rmdir(generated).catch((error: NodeJS.ErrnoException) => {
      if (!["ENOENT", "ENOTEMPTY"].includes(error.code ?? "")) throw error;
    });
  }
}
