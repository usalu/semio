const { chromium } = require("playwright");

async function openDetails(page) {
  await page.goto("http://127.0.0.1:4173/", { waitUntil: "load" });
  await page.waitForTimeout(3000);
  const kitGuid = await page.evaluate(async () => {
    const store = window.__COMPOSE_STORE__;
    const existing = (store.kitShallows?.() ?? []).find((kit) =>
      String(kit?.name ?? "")
        .toLowerCase()
        .includes("metabolism"),
    );
    if (existing?.guid) return existing.guid;
    const kitModule = await import("/@fs/workspaces/semio/assets/compose/kit_metabolism.json");
    await store.execute("compose.sketchpad.createKit", "compose.sketchpad.test.ensureMetabolismKitLoaded", kitModule.default, false, false);
    for (let attempt = 0; attempt < 30; attempt += 1) {
      const match = (store.kitShallows?.() ?? []).find((candidate) =>
        String(candidate?.name ?? "")
          .toLowerCase()
          .includes("metabolism"),
      );
      if (match?.guid) return match.guid;
      await new Promise((resolve) => setTimeout(resolve, 500));
    }
    throw new Error("kit not loaded");
  });
  await page.evaluate((nextKitGuid) => {
    window.__COMPOSE_NAVIGATE__(`/kits/${nextKitGuid}`);
  }, kitGuid);
  await page.waitForURL(new RegExp(`/kits/${kitGuid}`), { timeout: 30000 });
  await page.waitForTimeout(1500);
  const designGuid = await page.evaluate((nextKitGuid) => {
    const store = window.__COMPOSE_STORE__;
    const kit = store.kit(nextKitGuid).snapshot();
    const designs = kit.designs ?? [];
    return designs.find((design) => design.guid?.includes("9a890dd4"))?.guid ?? designs[designs.length - 1]?.guid;
  }, kitGuid);
  await page.evaluate(
    ({ nextKitGuid, nextDesignGuid }) => {
      window.__COMPOSE_NAVIGATE__(`/kits/${nextKitGuid}/designs/${nextDesignGuid}`);
    },
    { nextKitGuid: kitGuid, nextDesignGuid: designGuid },
  );
  await page.waitForURL(new RegExp(`/kits/${kitGuid}/designs/${designGuid}`), { timeout: 30000 });
  await page.waitForTimeout(4000);

  const rightPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.rightSidePanel"]');
  if (await rightPanelToggle.isVisible().catch(() => false)) {
    const panel = page.locator('[data-panel="rightSidePanel"]').first();
    if (!(await panel.isVisible().catch(() => false))) {
      await rightPanelToggle.click({ force: true });
      await page.waitForTimeout(1000);
    }
  }

  const rightPanel = page.locator('[data-panel="rightSidePanel"]').first();
  for (const labelText of ["Authors", "Author 1"]) {
    const label = rightPanel.getByText(labelText, { exact: true }).first();
    if (await label.isVisible().catch(() => false)) {
      await label.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(250);
    }
  }
}

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1400, height: 1000 } });
  await openDetails(page);
  const rows = await page.evaluate(() => {
    const pick = (label) => Array.from(document.querySelectorAll('[data-panel="rightSidePanel"] [data-slot="tree-item-row"]')).find((row) => (row.textContent ?? "").trim().startsWith(label));
    return ["Authors", "Author 1"].map((label) => {
      const row = pick(label);
      if (!row) return { label, found: false };
      const gutter = row.querySelector('[data-slot="tree-gutter"]');
      const slot = gutter?.querySelector('[data-slot="tree-gutter-slot"]');
      const elbow = gutter?.querySelector('[data-slot="tree-branch-elbow"]');
      const stem = gutter?.querySelector('[data-slot="tree-branch-stem"]');
      return {
        label,
        found: true,
        slotTag: slot?.tagName ?? null,
        slotClass: slot?.getAttribute("class") ?? null,
        slotStyle: slot?.getAttribute("style") ?? null,
        elbowStyle: elbow?.getAttribute("style") ?? null,
        stemStyle: stem?.getAttribute("style") ?? null,
      };
    });
  });
  console.log(JSON.stringify(rows, null, 2));
  await browser.close();
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
