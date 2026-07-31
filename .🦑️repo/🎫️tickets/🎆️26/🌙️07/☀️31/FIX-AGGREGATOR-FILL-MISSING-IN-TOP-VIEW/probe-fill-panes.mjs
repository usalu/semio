import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(fileURLToPath(import.meta.url));
const URL = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs = [];
page.on("console", (m) => {
  const t = m.text();
  if (t.includes("[DEBUG]") || m.type() === "error") {
    logs.push(t);
    console.log(t);
  }
});

await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForSelector("canvas", { timeout: 120_000 });
for (let i = 0; i < 4; i++) {
  const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
  if (await skip.count()) {
    await skip.first().click({ timeout: 5_000 }).catch(() => {});
    await page.waitForTimeout(300);
  } else break;
}
await page.waitForFunction(() => document.body?.innerText?.includes("Abbau Aufbau") === true, null, { timeout: 60_000 });
await page.waitForTimeout(2000);

// Activate fill tool via keyboard or UI
const fillToggle = page.getByRole("button", { name: /Füllen|Fill/i });
if (await fillToggle.count()) {
  await fillToggle.first().click().catch(() => {});
}
await page.waitForTimeout(500);

// Try clicking tool.fill in the bottom bar
const fillTool = page.locator('[id*="fill"], [data-utility*="fill"], button:has-text("Fill"), button:has-text("Füllen")');
console.log("fill-like controls", await fillTool.count());

// Wait for fillBuild planning by polling window UI via evaluating React fiber is hard;
// instead look for the fill count slider
for (let i = 0; i < 40; i++) {
  const slider = page.locator('#puzzle3d-fill-count, [id*="puzzle3d-fill-count"], [id*="fill-count"]');
  if (await slider.count()) {
    console.log("found fill slider at attempt", i);
    break;
  }
  // Click Fill tool button if present
  const tools = page.getByText(/^(Fill|Füllen)$/);
  if (await tools.count()) await tools.first().click().catch(() => {});
  await page.waitForTimeout(250);
}

const before = await page.evaluate(() => {
  const canvases = [...document.querySelectorAll("canvas")];
  return {
    canvasCount: canvases.length,
    titles: [...document.querySelectorAll("[data-window-title], .window-title, [class*='window']")].slice(0, 20).map((el) => el.textContent?.slice(0, 40)),
    bodySnippet: document.body?.innerText?.slice(0, 1500),
  };
});
console.log(JSON.stringify(before, null, 2));

// Probe R3F roots: walk __r3f and collect instance groups with revealIndex userdata if any
const probe = await page.evaluate(() => {
  const result = [];
  for (const canvas of document.querySelectorAll("canvas")) {
    const fiber = canvas.__r3f;
    const root = fiber?.root ?? fiber?.getState?.() ?? null;
    let scene = null;
    try {
      scene = fiber?.getState?.()?.scene ?? null;
    } catch {}
    if (!scene && canvas.parentElement) {
      // try drei store
      const key = Object.keys(canvas).find((k) => k.startsWith("__reactFiber") || k.startsWith("__reactContainer"));
    }
    const store = canvas.__r3f;
    let state = null;
    try {
      state = typeof store?.getState === "function" ? store.getState() : store;
    } catch {}
    const cam = state?.camera;
    const scn = state?.scene;
    let visibleMeshes = 0;
    let hiddenMeshes = 0;
    let groups = 0;
    const sample = [];
    if (scn) {
      scn.traverse((obj) => {
        if (obj.isGroup && obj.children?.length) groups += 1;
        if (obj.isMesh) {
          if (obj.visible && obj.parent?.visible !== false) visibleMeshes += 1;
          else hiddenMeshes += 1;
          if (sample.length < 8) {
            sample.push({
              name: obj.name,
              visible: obj.visible,
              parentVisible: obj.parent?.visible,
              pos: obj.getWorldPosition?.(new (window.THREE?.Vector3 || function(){this.x=0;this.y=0;this.z=0;})()) ,
            });
          }
        }
      });
    }
    result.push({
      width: canvas.width,
      height: canvas.height,
      hasState: Boolean(state),
      camType: cam?.type ?? cam?.isOrthographicCamera ? "Ortho" : cam?.isPerspectiveCamera ? "Persp" : typeof cam,
      camPos: cam?.position ? [cam.position.x, cam.position.y, cam.position.z] : null,
      visibleMeshes,
      hiddenMeshes,
      groups,
    });
  }
  return result;
});

writeFileSync(join(ROOT, "probe-fill-panes.json"), JSON.stringify({ before, probe, logs }, null, 2));
console.log(JSON.stringify(probe, null, 2));
await page.screenshot({ path: join(ROOT, "probe-fill-panes.png") });
await browser.close();
