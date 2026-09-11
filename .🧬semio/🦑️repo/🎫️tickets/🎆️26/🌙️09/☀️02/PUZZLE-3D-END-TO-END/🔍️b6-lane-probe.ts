/** 🔬️ Wave B6 lane probe — drives ONLY the four lanes wave B6 owns (first-pick Inspection, Brush
 * preview, clipboard paste latency, import distinct) against the puzzle3d React serve, and reports the
 * host-side `[DEBUG] b6 refresh panels` tap alongside the DOM readout so a law-vs-browser gap can be
 * attributed to the guest render or to the host lane. Sibling of `🔍️browser-probe.ts` (owned by B1,
 * mid-refactor) — deliberately standalone so the two never contend for one tab.
 *
 * Run: `bun 🔍️b6-lane-probe.ts [--port=6013] [--selection] [--brush] [--clipboard] [--import]`. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const want = (flag: string) => process.argv.includes(flag) || process.argv.includes("--all");
const t0 = Date.now();
const lines: string[] = [];
const consoleBuf: string[] = [];
const log = (message: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${message}`;
  lines.push(row);
  console.log(row);
};

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
await page.context().grantPermissions(["clipboard-read", "clipboard-write"], { origin: `http://127.0.0.1:${port}` });
page.on("console", (message) => {
  if (consoleBuf.length >= 6000) consoleBuf.shift();
  consoleBuf.push(`${message.type()}: ${message.text().slice(0, 600)}`);
});
page.on("pageerror", (error) => consoleBuf.push(`pageerror: ${String(error).slice(0, 400)}`));

const grep = (pattern: RegExp, count = 8) => consoleBuf.filter((row) => pattern.test(row)).slice(-count);

log(`navigating :${port}`);
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 });

const snapshot = async () =>
  page.evaluate(() => ({
    windows: Array.from(document.querySelectorAll('[data-slot="window"]')).map((node) => node.id),
    canvases: document.querySelectorAll("canvas").length,
    dialogs: Array.from(document.querySelectorAll('[role="dialog"]')).length,
  }));

let booted = false;
for (let attempt = 0; attempt < 40; attempt++) {
  await page.waitForTimeout(3000);
  const state = await snapshot();
  if (state.dialogs > 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) await skip.click({ timeout: 2000 }).catch(() => {});
  }
  if (state.windows.length >= 2 && state.canvases >= 2) {
    booted = true;
    log(`booted windows=${JSON.stringify(state.windows)} canvases=${state.canvases}`);
    break;
  }
}
const skipTour = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
if (await skipTour.count()) await skipTour.click({ timeout: 3000 }).catch(() => {});
await page.waitForTimeout(1500);

const dumpInstances = async () =>
  page.evaluate(() => {
    const host = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
    try {
      const parsed = JSON.parse(host?.getAttribute("data-instances-json") || "[]") as Array<{ id?: string }>;
      return { count: parsed.length, ids: parsed.map((entry) => entry.id ?? "?").slice(0, 12) };
    } catch {
      return { count: -1, ids: [] as string[] };
    }
  });

const dumpInspection = async () =>
  page.evaluate(() => {
    const nodes = Array.from(document.querySelectorAll("[id]"));
    const bySuffix = (end: string) => {
      const node = nodes.find((entry) => entry.id === end || entry.id.endsWith(end) || entry.id.endsWith(`/${end}`));
      return node ? (node as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 60) : null;
    };
    return {
      objectId: bySuffix("puzzle3d-play-inspector.object.id"),
      empty: bySuffix("puzzle3d-play-inspector.empty"),
      fields: Array.from(document.querySelectorAll("[id*='puzzle3d-play-inspector']")).map((node) => node.id).slice(0, 20),
    };
  });

const dumpInteraction = async () =>
  page.evaluate(() => {
    const host = document.querySelector("#puzzle3d-main-perspective") as HTMLElement | null;
    const raw = (host?.querySelector("[data-interaction-json]") as HTMLElement | null)?.getAttribute("data-interaction-json") || "";
    const preview = (host?.querySelector("[data-brush-preview-json]") as HTMLElement | null)?.getAttribute("data-brush-preview-json") || "";
    let parsed: { activeUtility?: string; hoveredVortexFullId?: string } = {};
    try {
      parsed = raw ? JSON.parse(raw) : {};
    } catch {
      parsed = {};
    }
    return { utility: parsed.activeUtility ?? null, hover: parsed.hoveredVortexFullId ?? null, previewLen: preview.length };
  });

const selectedFromLeftover = () => {
  const rows = consoleBuf.filter((row) => /leftover InteractionView/.test(row));
  const last = rows[rows.length - 1];
  if (!last) return [] as string[];
  const match = last.match(/"selectedIds":\[([^\]]*)\]/);
  return match && match[1] ? match[1].split(",").map((id) => id.replace(/"/g, "")) : [];
};

/** 🎯️ Frames the scene, then sweeps a coarse grid until a canvas pick actually lands a selection. */
const frameAndPick = async () => {
  const frame = page.locator("button", { hasText: /^frame$/i }).last();
  if (await frame.count()) await frame.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(3000);
  const canvas = page.locator("canvas").last();
  const box = await canvas.boundingBox();
  if (!box) return null;
  for (const fy of [0.5, 0.42, 0.58, 0.35, 0.65]) {
    for (const fx of [0.5, 0.42, 0.58, 0.34, 0.66]) {
      const spot = { x: Math.round(box.width * fx), y: Math.round(box.height * fy) };
      await canvas.hover({ position: spot, timeout: 4000, force: true }).catch(() => {});
      await page.waitForTimeout(250);
      await canvas.click({ position: spot, timeout: 5000, force: true }).catch(() => {});
      await page.waitForTimeout(1400);
      const ids = selectedFromLeftover();
      if (ids.length > 0) {
        log(`pick landed at ${JSON.stringify(spot)} selectedIds=${JSON.stringify(ids)}`);
        return spot;
      }
    }
  }
  log("pick never landed a selection");
  return null;
};

const openInspection = async () => {
  const tab = page.locator("button").filter({ hasText: /^inspection$/i }).first();
  if (await tab.count()) await tab.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(700);
};

if (booted && want("--selection")) {
  await openInspection();
  log(`selection inspection before ${JSON.stringify(await dumpInspection())}`);
  const spot = await frameAndPick();
  log(`selection picked at ${JSON.stringify(spot)}`);
  for (let poll = 0; poll < 10; poll++) {
    await page.waitForTimeout(1200);
    const inspection = await dumpInspection();
    if (poll % 3 === 0 || inspection.objectId) log(`selection poll ${poll} objectId=${inspection.objectId} empty=${inspection.empty}`);
    if (inspection.objectId) break;
  }
  log(`selection inspection after ${JSON.stringify(await dumpInspection())}`);
  const canvas = page.locator("canvas").last();
  const box = await canvas.boundingBox();
  for (let round = 0; round < 10; round++) {
    if (box) {
      await page.mouse.move(box.x + box.width * 0.2, box.y + box.height * 0.8);
      await page.mouse.down({ button: "middle" });
      await page.mouse.move(box.x + box.width * 0.24, box.y + box.height * 0.78, { steps: 4 });
      await page.mouse.up({ button: "middle" });
    }
    await page.waitForTimeout(5000);
    const inspection = await dumpInspection();
    log(`selection forced refresh ${round} objectId=${inspection.objectId} empty=${inspection.empty} refreshes=${grep(/b6 refresh panels/, 999).length}`);
    if (inspection.objectId) break;
  }
  log(`selection late b6-refresh ${JSON.stringify(grep(/b6 refresh panels.*inspection/, 6))}`);
  log(`selection leftovers ${JSON.stringify(grep(/leftover InteractionView/, 6))}`);
  log(`selection inspection-refresh ${JSON.stringify(grep(/leftover Inspection/, 6))}`);
  log(`selection b6-refresh ${JSON.stringify(grep(/b6 refresh panels/, 10))}`);
}

if (booted && want("--brush")) {
  const unfold = page.locator('[id="framework.window.puzzle3dMainPerspective.utilityBar.unfold"]').first();
  if (await unfold.count()) await unfold.click({ timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(900);
  const brush = page.locator("#brush").last();
  if (await brush.count()) await brush.click({ force: true, timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(1600);
  log(`brush armed ${JSON.stringify(await dumpInteraction())}`);
  const canvas = page.locator("canvas").last();
  const box = await canvas.boundingBox();
  if (box) {
    for (const fraction of [0.5, 0.55, 0.6, 0.5]) {
      await canvas.hover({ position: { x: Math.round(box.width * fraction), y: Math.round(box.height * 0.4) }, timeout: 4000, force: true }).catch(() => {});
      await page.waitForTimeout(600);
      log(`brush hover ${fraction} ${JSON.stringify(await dumpInteraction())}`);
    }
  }
  log(`brush hop-census place=${grep(/brush-place hop/, 200).length} utilityHops=${JSON.stringify(grep(/setActiveUtility hop/, 4))}`);
}

if (booted && want("--clipboard")) {
  const before = await dumpInstances();
  await frameAndPick();
  await page.keyboard.press("Meta+c");
  await page.waitForTimeout(1500);
  await page.keyboard.press("Meta+v");
  log(`clipboard pasted, instances before=${JSON.stringify(before)}`);
  for (let poll = 0; poll < 20; poll++) {
    await page.waitForTimeout(1500);
    const now = await dumpInstances();
    log(`clipboard poll ${poll} t=+${(poll + 1) * 1.5}s instances=${JSON.stringify(now)}`);
    if (now.count !== before.count) break;
  }
  log(`clipboard menus ${JSON.stringify(await page.evaluate(() => Array.from(document.querySelectorAll("[data-action]")).map((node) => `${node.id}=${node.getAttribute("data-action")}`).filter((row) => /copy|paste|duplicate/i.test(row)).slice(0, 20)))}`);
}

if (booted && want("--import")) {
  const exports = readdirSync(OUT).filter((name) => name.endsWith("-distinct.json"));
  const feed = exports.length > 0 ? join(OUT, exports[exports.length - 1]!) : "";
  if (!feed) log("import skipped — no *-distinct.json in generated");
  else {
    const before = await dumpInstances();
    log(`import feed=${feed} before=${JSON.stringify(before)} fixtureObjects=${(JSON.parse(readFileSync(feed, "utf8")).objects ?? []).length}`);
    const canvasForMenu = page.locator("canvas").last();
    const menuBox = await canvasForMenu.boundingBox();
    if (menuBox) await page.mouse.click(menuBox.x + 200, menuBox.y + 160, { button: "right" });
    await page.waitForTimeout(900);
    const chooser = page.waitForEvent("filechooser", { timeout: 20000 });
    const byAction = page.locator('[data-menu-action="openImportFixture"]').last();
    const byId = page.locator('[id="shell-menu.action.openImportFixture"]').last();
    log(`import menu byAction=${await byAction.count()} byId=${await byId.count()}`);
    if (await byAction.count()) await byAction.click({ force: true, timeout: 4000 }).catch(() => {});
    else if (await byId.count()) await byId.click({ force: true, timeout: 4000 }).catch(() => {});
    const picked = await chooser.catch(() => null);
    if (!picked) log("import chooser never opened");
    else {
      await picked.setFiles(feed);
      for (let poll = 0; poll < 20; poll++) {
        await page.waitForTimeout(1500);
        const now = await dumpInstances();
        log(`import poll ${poll} t=+${(poll + 1) * 1.5}s instances=${JSON.stringify(now)}`);
        if (JSON.stringify(now.ids) !== JSON.stringify(before.ids)) break;
      }
      log(`import ingress ${JSON.stringify(grep(/importFixture ingress|import-picker/, 6))}`);
    }
  }
}

writeFileSync(
  join(OUT, `b6-lane-probe-${stamp}.md`),
  `# b6 lane probe ${stamp}\n\n## timeline\n${lines.join("\n")}\n\n## console tail\n\`\`\`\n${consoleBuf.slice(-900).join("\n")}\n\`\`\`\n`,
);
log(`done → 🗑️generated/b6-lane-probe-${stamp}.md`);
await browser.close();
process.exit(0);
