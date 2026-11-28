import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { fixture, out, runSteps, session, unfoldActionsRail } from "./journey-kit.ts";

/** 🔎️ Live UI inventory of the PDF 1.7 editor: every rendered action button, toolbar, menu item, tab, heading and window title, then the context menu of the canvas and of a page row, on a new and on an opened document.
 * Usage: `PDF_JOURNEY_OUT=ui-inventory repo <this file>`; writes `ui-inventory.json` into the evidence folder.
 * @see ./🗺️feature-inventory.md */

const census = (pane: Awaited<ReturnType<typeof session>>) => pane.page.evaluate(() => {
  const name = (element: Element) => (element.getAttribute("aria-label") ?? element.textContent ?? "").trim().replace(/\s+/gu, " ").slice(0, 80);
  const pick = (selector: string) => [...document.querySelectorAll(selector)].map(element => `${element.id || element.getAttribute("data-ui-node-key") || element.tagName}|${element.getAttribute("role") ?? ""}|${name(element)}`);
  return { actions: pick('[id^="action."],[id*=".action."]').filter(line => !line.includes(".arg.")), toolbars: pick('[role="toolbar"]'), menus: pick('[role="menu"],[role="menubar"],[role="menuitem"]'), tabs: pick('[role="tab"]'), headings: pick('h1,h2,h3,[role="heading"]'), buttons: pick("button").slice(0, 400), dialogs: pick('[role="dialog"],[role="alertdialog"]'), windows: pick('[data-window-kind],[data-slot="window"]') };
});

const inventory: Record<string, unknown> = {};
await runSteps(["inventory"], {
  inventory: async browser => {
    const pane = await session(browser, "en-US", "inventory");
    try {
      inventory.initial = await census(pane);
      await unfoldActionsRail(pane.page);
      inventory.unfolded = await census(pane);
      const canvas = pane.page.locator("canvas").first();
      if (await canvas.count() > 0) { await canvas.click({ button: "right" }); await pane.page.waitForTimeout(800); inventory.canvasContext = await census(pane); await pane.page.keyboard.press("Escape"); }
      await pane.open(fixture);
      await pane.page.waitForTimeout(3000);
      inventory.opened = await census(pane);
      const row = pane.page.getByRole("treeitem").first();
      if (await row.count() > 0) { await row.click({ button: "right" }); await pane.page.waitForTimeout(800); inventory.rowContext = await census(pane); await pane.page.keyboard.press("Escape"); }
      await pane.shot("ui-inventory");
      inventory.text = (await pane.text()).slice(0, 6000);
    } finally {
      await writeFile(join(out, "ui-inventory.json"), `${JSON.stringify(inventory, null, 1)}\n`);
      await pane.context.close();
    }
  },
});
