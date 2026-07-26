import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(fileURLToPath(import.meta.url));
const AGGREGATOR_URL = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const OUT = join(ROOT, "verify-aggregator-e2e.json");
const SHOT = join(ROOT, "aggregator-viewport.png");

async function warm(url) {
  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      const res = await fetch(url);
      await res.arrayBuffer();
      return res.status;
    } catch (err) {
      if (attempt === 4) throw err;
      await Bun.sleep(500 * (attempt + 1));
    }
  }
  return 0;
}

for (const path of [
  "/js/index.ts",
  "/@fs/Users/ueli/Documents/semio/ui/js/react/index.tsx",
  "/@fs/Users/ueli/Documents/semio/framework/renderer/react/index.tsx",
  "/plugin-modules/puzzle/puzzle_plugin.js",
  "/mesh/hexagonal-cut-concrete-forest-left.glb",
]) {
  const status = await warm(new URL(path, AGGREGATOR_URL).href);
  console.log(`warm ${status} ${path}`);
}

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleLines = [];
const pageErrors = [];
page.on("console", (msg) => {
  const text = msg.text();
  consoleLines.push({ type: msg.type(), text });
  if (text.includes("[DEBUG]") || msg.type() === "error") console.log(`[console:${msg.type()}] ${text}`);
});
page.on("pageerror", (err) => {
  pageErrors.push(String(err));
  console.log(`[pageerror] ${err}`);
});

await page.goto(AGGREGATOR_URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForSelector("canvas", { timeout: 120_000 });

for (let i = 0; i < 4; i++) {
  const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
  if (await skip.count()) {
    await skip.first().click({ timeout: 5_000 }).catch(() => {});
    await page.waitForTimeout(400);
  } else break;
}

await page.waitForFunction(
  () => window.__AGGREGATOR_SCENE_DEBUG__?.instanceCount > 0 && window.__AGGREGATOR_REVEAL_DEBUG__?.rows?.length > 0,
  null,
  { timeout: 120_000 },
);

await page.waitForFunction(
  () => (window.__AGGREGATOR_GLB_DEBUG__ ?? []).some((row) => row.url?.includes("hexagonal-cut-concrete-forest-left") && row.meshCount > 0),
  null,
  { timeout: 120_000 },
).catch(() => null);

await page.waitForTimeout(2_000);
await page.screenshot({ path: SHOT, fullPage: false });

const snapshot = await page.evaluate(async () => {
  const scene = window.__AGGREGATOR_SCENE_DEBUG__ ?? null;
  const reveal = window.__AGGREGATOR_REVEAL_DEBUG__ ?? null;
  const glbDebug = window.__AGGREGATOR_GLB_DEBUG__ ?? [];
  const url = scene?.meshes?.find((m) => m.url?.includes("hexagonal-cut-concrete-forest-left"))?.url;
  const glbProbe = url
    ? await fetch(url).then(async (res) => ({
        url,
        ok: res.ok,
        status: res.status,
        bytes: (await res.arrayBuffer()).byteLength,
        contentType: res.headers.get("content-type"),
      }))
    : null;
  return {
    title: document.title,
    canvasCount: document.querySelectorAll("canvas").length,
    scene,
    reveal,
    glbDebug,
    glbProbe,
  };
});

const errors = [...pageErrors, ...consoleLines.filter((l) => l.type === "error").map((l) => l.text)];
const seed = snapshot.reveal?.rows?.find((row) => row.id === "seed-left-001");
const leftGlb = snapshot.glbDebug.find((row) => String(row.url).includes("hexagonal-cut-concrete-forest-left"));
const ok =
  snapshot.scene?.instanceCount === 1 &&
  snapshot.scene.instances?.[0]?.rawHasRevealIndex === false &&
  snapshot.reveal?.cutoff === 0 &&
  seed?.visible === true &&
  seed?.rootVisible === true &&
  (leftGlb?.meshCount ?? 0) >= 10 &&
  snapshot.glbProbe?.ok === true &&
  (snapshot.glbProbe?.bytes ?? 0) > 1000 &&
  errors.length === 0;

const result = {
  ok,
  url: AGGREGATOR_URL,
  at: new Date().toISOString(),
  shot: SHOT,
  snapshot,
  debugLogs: consoleLines.filter((l) => l.text.includes("[DEBUG]")).slice(-40),
  errors: errors.slice(0, 40),
};
writeFileSync(OUT, `${JSON.stringify(result, null, 2)}\n`, "utf8");
console.log(JSON.stringify({
  ok,
  out: OUT,
  shot: SHOT,
  summary: {
    title: snapshot.title,
    instanceCount: snapshot.scene?.instanceCount,
    revealIndexOmitted: snapshot.scene?.instances?.[0]?.rawHasRevealIndex === false,
    revealCutoff: snapshot.reveal?.cutoff,
    seed,
    leftGlb,
    glbProbe: snapshot.glbProbe,
    errorCount: errors.length,
  },
}, null, 2));

await browser.close();
process.exit(ok ? 0 : 1);
