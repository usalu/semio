import { chromium } from "playwright";

const browser = await chromium.launch({
  args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
});
const page = await browser.newPage();

// Capture console messages
const consoleLogs = [];
page.on("console", (msg) => {
  consoleLogs.push(`[${msg.type()}] ${msg.text()}`);
});
page.on("pageerror", (err) => {
  consoleLogs.push(`[PAGE ERROR] ${err.message}`);
});

console.log("[DIAG] Going to sketchpad...");
await page.goto("http://localhost:5173/", { waitUntil: "networkidle", timeout: 60000 });
console.log("[DIAG] Page loaded, URL:", page.url());
await page.waitForTimeout(3000);

// Check if store is available
const storeAvailable = await page.evaluate(() => !!window.__COMPOSE_STORE__);
console.log("[DIAG] Store available:", storeAvailable);

const navigateAvailable = await page.evaluate(() => !!window.__COMPOSE_NAVIGATE__);
console.log("[DIAG] Navigate available:", navigateAvailable);

// Load kit fixture
console.log("[DIAG] Loading metabolism kit...");
const kitLoaded = await page.evaluate(async () => {
  const store = window.__COMPOSE_STORE__;
  if (!store) return { error: "no store" };

  // Check existing kits
  const existingKits = store.kitShallows?.() ?? [];
  if (existingKits.length > 0) {
    return { existing: true, guid: existingKits[0].guid, name: existingKits[0].name };
  }

  try {
    // Try to fetch the kit asset
    const resp = await fetch("/assets/compose/metabolism.kit.compose.json");
    if (!resp.ok) return { error: `fetch failed: ${resp.status}` };
    const kit = await resp.json();
    await store.execute("compose.sketchpad.createKit", "compose.sketchpad.diag", kit, false, false);

    // Wait for it to appear
    for (let i = 0; i < 20; i++) {
      const kitsMap = store.kits;
      if (kitsMap && kitsMap.size > 0) {
        const entries = [];
        kitsMap.forEach((v, k) => {
          try {
            const snap = v?.getSnapshot?.();
            const snapKit = snap?.kit;
            entries.push({
              key: String(k),
              kitGuid: snapKit?.guid,
              kitName: snapKit?.name,
              designCount: (snapKit?.designs || []).length,
              typeCount: (snapKit?.types || []).length,
            });
          } catch (e2) {
            entries.push({ key: String(k), error: e2.message });
          }
        });
        return { loaded: true, entries };
      }
      await new Promise((r) => setTimeout(r, 500));
    }
    return { error: "kit did not appear after loading" };
  } catch (e) {
    return { error: e.message };
  }
});
console.log("[DIAG] Kit load result:", JSON.stringify(kitLoaded));

if (kitLoaded.error) {
  console.log("[DIAG] CONSOLE LOGS:", consoleLogs.join("\n"));
  await browser.close();
  process.exit(1);
}

const kitGuid = kitLoaded.entries?.[0]?.kitGuid || kitLoaded.entries?.[0]?.key;
console.log("[DIAG] Navigating to kit:", kitGuid);

await page.evaluate((kg) => {
  window.__COMPOSE_NAVIGATE__(`/kits/${kg}`);
}, kitGuid);
await page.waitForTimeout(3000);
console.log("[DIAG] Kit URL:", page.url());

// Check if designs are visible in the table
const designs = await page.evaluate(() => {
  const store = window.__COMPOSE_STORE__;
  if (!store) return [];
  const kitGuids = Array.from(store.kits?.keys() ?? []);
  if (kitGuids.length === 0) return [];
  const kitStore = store.kit(kitGuids[0]);
  if (!kitStore) return [];
  const kit = kitStore.snapshot();
  return (kit.designs ?? []).map((d) => ({ guid: d.guid, name: d.name }));
});
console.log(`[DIAG] Designs in store (${designs.length}):`, JSON.stringify(designs.slice(0, 5)));

// Check table rows
const rowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]"))
    .map((el) => el.getAttribute("data-row-id"))
    .slice(0, 20),
);
console.log(`[DIAG] Table row IDs (${rowIds.length}):`, JSON.stringify(rowIds));

const designRowIds = rowIds.filter((id) => id?.startsWith("design-"));
console.log(`[DIAG] Design row IDs (${designRowIds.length}):`, JSON.stringify(designRowIds));

// Try double-clicking a design row
if (designRowIds.length > 0) {
  const targetRow = designRowIds.find((id) => id?.includes("9a890dd4")) ?? designRowIds[0];
  console.log("[DIAG] Target design row:", targetRow);

  const rowEl = page.locator(`[data-row-id="${targetRow}"]`);
  const isVisible = await rowEl.isVisible().catch(() => false);
  console.log("[DIAG] Row visible:", isVisible);

  if (isVisible) {
    // Double-click via Playwright
    console.log("[DIAG] Double-clicking design row...");
    await rowEl.dblclick();
    await page.waitForTimeout(2000);
    console.log("[DIAG] URL after double-click:", page.url());

    const isOnDesign = page.url().includes("/designs/");
    console.log("[DIAG] Navigated to design:", isOnDesign);
  }
}

// Print console logs
const recentLogs = consoleLogs.filter((l) => l.includes("ERROR") || l.includes("error") || l.includes("navigat"));
if (recentLogs.length > 0) {
  console.log("[DIAG] Relevant console logs:");
  recentLogs.forEach((l) => console.log("  ", l));
}

await browser.close();
console.log("[DIAG] Done");
