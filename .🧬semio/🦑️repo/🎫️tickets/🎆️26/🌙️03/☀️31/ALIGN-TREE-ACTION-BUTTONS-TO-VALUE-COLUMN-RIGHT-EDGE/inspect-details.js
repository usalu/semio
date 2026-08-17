import { chromium } from "playwright";

async function main() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1400, height: 1000 } });

  page.on("console", (message) => {
    if (message.type() === "error" || message.type() === "warning") {
      console.log(`[BROWSER ${message.type().toUpperCase()}] ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => {
    console.log(`[BROWSER PAGEERROR] ${error.message}`);
  });

  const baseUrl = "http://127.0.0.1:4173";
  await page.goto(`${baseUrl}/`, { waitUntil: "load" });
  await page.waitForTimeout(3000);

  const importedKitGuid = await page.evaluate(async () => {
    const store = window.__COMPOSE_STORE__;
    if (!store) throw new Error("no store");
    const existing = (store.kitShallows?.() ?? []).find((kit) =>
      String(kit?.name ?? "")
        .toLowerCase()
        .includes("metabolism"),
    );
    if (existing?.guid) return existing.guid;

    const kitModule = await import("/@fs/workspaces/semio/assets/compose/kit_metabolism.json");
    const kit = kitModule.default;
    await store.execute("compose.sketchpad.createKit", "compose.sketchpad.test.ensureMetabolismKitLoaded", kit, false, false);

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

  await page.evaluate((kitGuid) => {
    window.__COMPOSE_NAVIGATE__(`/kits/${kitGuid}`);
  }, importedKitGuid);
  await page.waitForURL(new RegExp(`/kits/${importedKitGuid}`), { timeout: 30000 });
  await page.waitForTimeout(2000);

  const designGuid = await page.evaluate((kitGuid) => {
    const store = window.__COMPOSE_STORE__;
    const kit = store.kit(kitGuid).snapshot();
    const designs = kit.designs ?? [];
    return designs.find((design) => design.guid?.includes("9a890dd4"))?.guid ?? designs[designs.length - 1]?.guid;
  }, importedKitGuid);

  await page.evaluate(
    ({ designGuid, kitGuid }) => {
      window.__COMPOSE_NAVIGATE__(`/kits/${kitGuid}/designs/${designGuid}`);
    },
    { designGuid, kitGuid: importedKitGuid },
  );
  await page.waitForURL(new RegExp(`/kits/${importedKitGuid}/designs/${designGuid}`), { timeout: 30000 });
  await page.waitForTimeout(5000);

  const rightPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.rightSidePanel"]');
  if (await rightPanelToggle.isVisible().catch(() => false)) {
    const panel = page.locator('[data-panel="rightSidePanel"]').first();
    if (!(await panel.isVisible().catch(() => false))) {
      await rightPanelToggle.click({ force: true });
      await page.waitForTimeout(1000);
    }
  }

  const rightPanel = page.locator('[data-panel="rightSidePanel"]').first();
  for (const labelText of ["Location", "Attributes"]) {
    const row = rightPanel
      .locator('[data-slot="tree-item-row"]')
      .filter({ has: rightPanel.getByText(labelText, { exact: true }) })
      .first();
    const actionButton = row.locator('[data-slot="tooltip-trigger"]').first();
    if ((await row.isVisible().catch(() => false)) && (await actionButton.isVisible().catch(() => false))) {
      await actionButton.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(300);
    }
  }
  for (const labelText of ["Location", "Authors", "Author 1", "Author 2", "Attributes"]) {
    const label = rightPanel.getByText(labelText, { exact: true }).first();
    if (await label.isVisible().catch(() => false)) {
      await label.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(250);
    }
  }

  await page.screenshot({
    path: "/workspaces/semio/.repo/🎫️/26/03/31/ALIGN-TREE-ACTION-BUTTONS-TO-VALUE-COLUMN-RIGHT-EDGE/design-page.png",
  });

  const data = await page.evaluate(() => {
    const panel = document.querySelector('[data-panel="rightSidePanel"]');
    if (!panel) throw new Error("no right panel");

    const rowSelector = '[data-slot="property-row"], [data-slot="tree-section-row"], [data-slot="tree-item-row"], [data-slot="tree-property-item"]';
    const widgetSelector =
      '[data-slot="input"], [data-slot="textarea"], [role="combobox"], [data-slot="stepper-group"], [data-slot="stepper-plus"], [data-slot="slider-row"], [data-slot="slider-value"], [data-slot="toggle-group"], [data-slot="button-group"], [data-slot="tree-action-rail"], [data-slot="tree-item-row-right"], [data-slot="tree-action-rail"] > *, [data-slot="tree-item-row-right"] > *';

    return Array.from(panel.querySelectorAll(rowSelector))
      .slice(0, 50)
      .map((row) => {
        const rowRect = row.getBoundingClientRect();
        const widgets = Array.from(row.querySelectorAll(widgetSelector))
          .map((element) => {
            const rect = element.getBoundingClientRect();
            const style = window.getComputedStyle(element);
            return {
              display: style.display,
              right: rect.right,
              slot: element.getAttribute("data-slot") || element.getAttribute("role"),
              text: (element.textContent || "").trim().slice(0, 60),
              width: rect.width,
            };
          })
          .filter((widget) => widget.width > 0);

        return {
          label: row.querySelector('[data-slot="tree-label"], [data-slot="property-label"]')?.textContent?.trim() ?? "",
          rowRight: rowRect.right,
          rowSlot: row.getAttribute("data-slot"),
          widgets,
        };
      })
      .filter((row) => row.widgets.length > 0);
  });

  console.log(JSON.stringify(data, null, 2));
  await browser.close();
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
