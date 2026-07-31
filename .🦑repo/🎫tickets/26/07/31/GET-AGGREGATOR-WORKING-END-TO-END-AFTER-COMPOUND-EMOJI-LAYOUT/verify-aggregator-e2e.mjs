import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(fileURLToPath(import.meta.url));
const AGGREGATOR_URL = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const OUT = join(ROOT, "verify-aggregator-e2e.json");
const SHOT = join(ROOT, "aggregator-viewport.png");
const EXPECTED_TITLE = "Entwerfen mit Bestand · Aggregator";
const MESH_PATH = "/mesh/🧊hexagonal-cut-concrete-forest-left.glb";

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
  "/📦index.ts",
  "/plugin-modules/puzzle/puzzle_plugin.js",
  MESH_PATH,
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
await page.waitForFunction((title) => document.title === title, EXPECTED_TITLE, { timeout: 60_000 });

for (let i = 0; i < 4; i++) {
  const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
  if (await skip.count()) {
    await skip.first().click({ timeout: 5_000 }).catch(() => {});
    await page.waitForTimeout(400);
  } else break;
}

await page.waitForFunction(() => document.body?.innerText?.includes("Abbau Aufbau") === true, null, { timeout: 60_000 });
await page.waitForTimeout(3_000);
await page.screenshot({ path: SHOT, fullPage: false });

const snapshot = await page.evaluate(async (meshPath) => {
  const meshRes = await fetch(meshPath);
  const meshBytes = (await meshRes.arrayBuffer()).byteLength;
  return {
    title: document.title,
    canvasCount: document.querySelectorAll("canvas").length,
    hasAbbauAufbau: document.body?.innerText?.includes("Abbau Aufbau") === true,
    hasPerspective: document.body?.innerText?.includes("Perspective") === true,
    glbProbe: {
      url: meshPath,
      ok: meshRes.ok,
      status: meshRes.status,
      bytes: meshBytes,
      contentType: meshRes.headers.get("content-type"),
    },
  };
}, MESH_PATH);

const ignoredError = (text) =>
  /Download the React DevTools|GL Driver Message|GPU stall/i.test(text);

const errors = [
  ...pageErrors,
  ...consoleLines.filter((l) => l.type === "error" && !ignoredError(l.text)).map((l) => l.text),
];

const ok =
  snapshot.title === EXPECTED_TITLE &&
  snapshot.canvasCount >= 1 &&
  snapshot.hasAbbauAufbau === true &&
  snapshot.glbProbe.ok === true &&
  snapshot.glbProbe.bytes > 1000 &&
  String(snapshot.glbProbe.contentType ?? "").includes("gltf") &&
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
console.log(
  JSON.stringify(
    {
      ok,
      out: OUT,
      shot: SHOT,
      summary: {
        title: snapshot.title,
        canvasCount: snapshot.canvasCount,
        hasAbbauAufbau: snapshot.hasAbbauAufbau,
        glbBytes: snapshot.glbProbe.bytes,
        errors: errors.length,
      },
    },
    null,
    2,
  ),
);

await browser.close();
process.exit(ok ? 0 : 1);
