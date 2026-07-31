import { chromium } from "playwright";
import { readFile } from "node:fs/promises";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const metabolismKit = JSON.parse(await readFile("/workspaces/semio/assets/compose/kit_metabolism.json", "utf8"));

page.on("console", (msg) => {
  console.log(`[browser:${msg.type()}] ${msg.text()}`);
});
page.on("pageerror", (err) => {
  console.log(`[pageerror] ${err.message}`);
});

const baseUrl = "http://127.0.0.1:5173";
await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForFunction(() => Boolean(window.__COMPOSE_STORE__ && window.__COMPOSE_NAVIGATE__), { timeout: 60000 });

const importedKit = await page.evaluate(async (kit) => {
  const store = window.__COMPOSE_STORE__;
  if (!store || typeof store.kitShallows !== "function") {
    throw new Error("Sketchpad store is not available");
  }

  const existingKit = (store.kitShallows?.() ?? []).find((entry) =>
    String(entry?.name ?? "")
      .toLowerCase()
      .includes("metabolism"),
  );
  if (existingKit?.guid) {
    const kitStore = store.kit?.(existingKit.guid);
    const snapshot = kitStore?.snapshot?.();
    const designs = snapshot?.designs ?? [];
    const targetDesign = designs
      .map((design) => ({
        guid: design.guid,
        piecesWithPlaneCount: (design.pieces ?? []).filter((piece) => Boolean(piece?.plane)).length,
      }))
      .sort((left, right) => right.piecesWithPlaneCount - left.piecesWithPlaneCount)[0];
    return { kitGuid: existingKit.guid, designGuid: targetDesign?.guid ?? null };
  }

  await store.execute("compose.sketchpad.createKit", "compose.sketchpad.test.measureDetailPanelSpacing", kit, false, false);

  const match = (store.kitShallows?.() ?? []).find((entry) =>
    String(entry?.name ?? "")
      .toLowerCase()
      .includes("metabolism"),
  );
  if (!match?.guid) {
    return { kitGuid: null, designGuid: null };
  }

  const kitStore = store.kit?.(match.guid);
  const snapshot = kitStore?.snapshot?.();
  const designs = snapshot?.designs ?? [];
  const targetDesign = designs
    .map((design) => ({
      guid: design.guid,
      piecesWithPlaneCount: (design.pieces ?? []).filter((piece) => Boolean(piece?.plane)).length,
    }))
    .sort((left, right) => right.piecesWithPlaneCount - left.piecesWithPlaneCount)[0];
  return { kitGuid: match.guid, designGuid: targetDesign?.guid ?? null };
}, metabolismKit);

if (!importedKit?.kitGuid || !importedKit?.designGuid) {
  throw new Error("Metabolism kit import failed");
}

await page.evaluate(({ kitGuid, designGuid }) => {
  const navigate = window.__COMPOSE_NAVIGATE__;
  if (typeof navigate !== "function") {
    throw new Error("Sketchpad navigation bridge is not available");
  }
  navigate(`/kits/${kitGuid}/designs/${designGuid}`);
}, importedKit);
await page.waitForURL(new RegExp(`.*/kits/${importedKit.kitGuid}/designs/${importedKit.designGuid}`), { timeout: 30000 });
console.log(`[measure] Design URL: ${page.url()}`);

const detailsToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.details.show"]').first();
if (await detailsToggle.count()) {
  await detailsToggle.click({ force: true }).catch(() => {});
}
await page.waitForSelector('[data-panel="rightSidePanel"]', { timeout: 10000 });

const panel = page.locator('[data-panel="rightSidePanel"]').first();
const selectionResult = await page.evaluate(() => {
  const actor = window.__COMPOSE_ACTOR__;
  const store = window.__COMPOSE_STORE__;
  if (!actor || !store) return { ok: false, reason: "missing-actor-or-store" };
  const path = window.location.pathname;
  const designGuid = path.match(/\/designs\/([^/]+)/)?.[1];
  const kitGuid = path.match(/\/kits\/([^/]+)/)?.[1];
  if (!designGuid || !kitGuid) return { ok: false, reason: "missing-scope" };
  const kitStore = store.kit?.(kitGuid);
  const kit = kitStore?.snapshot?.();
  const design = kit?.designs?.find((entry) => entry.guid === designGuid);
  const pieceGuid = (design?.pieces ?? []).find((entry) => Boolean(entry?.plane))?.guid ?? null;
  if (!pieceGuid) return { ok: false, reason: "missing-piece-with-plane" };
  actor.send({ type: "DESIGN.SET_ACTIVE_TOOL", kitGuid, designGuid, tool: "selection-normal" });
  actor.send({
    type: "DESIGN.SET_SELECTION",
    kitGuid,
    designGuid,
    selection: { pieces: [pieceGuid], connections: [], connectors: [] },
  });
  return { ok: true, kitGuid, designGuid, pieceGuid };
});

console.log(`[measure] Selection result: ${JSON.stringify(selectionResult)}`);
if (!selectionResult.ok) {
  throw new Error(`Selection failed: ${selectionResult.reason}`);
}
const pieceSection = panel.locator('[id="compose.sketchpad.app.design.panel.details.section.piece.properties"]').first();
await pieceSection.waitFor({ state: "visible", timeout: 15000 });
console.log("[measure] Piece details visible");

for (const itemId of ["compose.sketchpad.app.design.piece.plane", "compose.sketchpad.app.design.piece.planeOrigin", "compose.sketchpad.app.design.piece.planeXAxis", "compose.sketchpad.app.design.piece.planeYAxis"]) {
  const item = panel.locator(`[id="${itemId}"]`).first();
  if ((await item.count()) === 0) continue;
  await item.scrollIntoViewIfNeeded().catch(() => {});
  const state = await item.getAttribute("data-state");
  if (state === "closed") {
    await item.click({ force: true });
    await page.waitForTimeout(200);
  }
}

const spacing = await page.evaluate(() => {
  const groupIds = ["compose.sketchpad.app.design.piece.planeOrigin", "compose.sketchpad.app.design.piece.planeXAxis", "compose.sketchpad.app.design.piece.planeYAxis"];
  const readPx = (value) => {
    const parsed = Number.parseFloat(value ?? "0");
    return Number.isFinite(parsed) ? parsed : 0;
  };

  return groupIds.map((groupId) => {
    const sectionRow = document.getElementById(groupId);
    const sectionRoot = sectionRow?.parentElement;
    const sectionContent = sectionRoot?.querySelector('[data-slot="tree-section-content"]');
    const propertyRows = sectionContent ? Array.from(sectionContent.querySelectorAll('[data-slot="property-row"]')) : [];
    const firstPropertyRow = propertyRows[0];
    const secondPropertyRow = propertyRows[1];
    if (!(sectionRow instanceof HTMLElement) || !(sectionContent instanceof HTMLElement) || !(firstPropertyRow instanceof HTMLElement)) {
      return { groupId, present: false };
    }
    const sectionRect = sectionRow.getBoundingClientRect();
    const firstRect = firstPropertyRow.getBoundingClientRect();
    const secondRect = secondPropertyRow instanceof HTMLElement ? secondPropertyRow.getBoundingClientRect() : null;
    return {
      groupId,
      present: true,
      sectionToFirstRowGap: firstRect.top - sectionRect.bottom,
      rowGap: secondRect ? secondRect.top - firstRect.bottom : null,
      sectionMarginTop: readPx(getComputedStyle(sectionRow).marginTop),
      sectionMarginBottom: readPx(getComputedStyle(sectionRow).marginBottom),
      contentGap: readPx(getComputedStyle(sectionContent).rowGap || getComputedStyle(sectionContent).gap),
    };
  });
});

console.log(`[measure] Plane spacing: ${JSON.stringify(spacing)}`);

const visible = spacing.filter((entry) => entry.present);
if (visible.length !== 3) {
  throw new Error(`Expected 3 visible plane sections, got ${visible.length}`);
}

for (const entry of visible) {
  if (entry.sectionToFirstRowGap < 24) {
    throw new Error(`sectionToFirstRowGap too small for ${entry.groupId}: ${entry.sectionToFirstRowGap}`);
  }
  if (entry.sectionMarginTop < 32) {
    throw new Error(`sectionMarginTop too small for ${entry.groupId}: ${entry.sectionMarginTop}`);
  }
  if (entry.sectionMarginBottom < 24) {
    throw new Error(`sectionMarginBottom too small for ${entry.groupId}: ${entry.sectionMarginBottom}`);
  }
  if (entry.contentGap < 8) {
    throw new Error(`contentGap too small for ${entry.groupId}: ${entry.contentGap}`);
  }
}

console.log("[measure] Spacing thresholds satisfied");
await browser.close();
