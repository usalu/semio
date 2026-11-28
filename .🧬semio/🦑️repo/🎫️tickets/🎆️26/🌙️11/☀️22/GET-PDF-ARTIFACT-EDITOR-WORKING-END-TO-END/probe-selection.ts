import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const doc = join(import.meta.dir, "🗑️generated/fixtures/ann-doc.pdf");
async function probe(browser: Browser) {
  const pane = await session(browser, "en-US", "probe");
  const logs: string[] = [];
  pane.page.on("console", m => logs.push(`${m.type()}: ${m.text().slice(0, 300)}`));
  try {
    await pane.open(doc);
    await pane.page.waitForTimeout(2000);
    const dump = async (label: string) => console.log(label, JSON.stringify(await pane.page.evaluate(() => {
      const o = document.querySelector('[id="window:pdf.object/pdf-object"]') ?? document.querySelector('[id*="pdf-object"]');
      const h = [...document.querySelectorAll('[id*="pdf.object"]')].filter(e => /kind|empty/.test(e.id)).map(e => `${e.id.slice(-30)}:${(e.textContent ?? "").trim().slice(0, 40)}`);
      const sel = [...document.querySelectorAll('[aria-selected="true"],[aria-pressed="true"],[data-selected="true"]')].map(e => `${e.id.slice(-40)}|${(e.textContent ?? "").slice(0, 30)}`).slice(0, 6);
      return { h, sel, has: !!o };
    })));
    await dump("start");
    await pane.page.getByRole("button", { name: /Annotation: Note body/ }).first().click();
    await pane.page.waitForTimeout(3000);
    await dump("note-row");
    const vis = () => pane.page.evaluate(() => [...document.querySelectorAll('[id^="window:pdf.object/"]')].filter(e => (e as HTMLElement).offsetParent !== null).length);
    console.log("object visible nodes", await vis());
    await pane.page.locator('[id="mode-dock-tab-3-pdf.object"]').click();
    await pane.page.waitForTimeout(2500);
    console.log("object visible nodes after tab", await vis());
    console.log("heading", JSON.stringify(await pane.page.evaluate(() => [...document.querySelectorAll('[id^="window:pdf.object/"]')].filter(e => /kind|empty/.test(e.id)).map(e => e.id.slice(-24) + ":" + (e.textContent ?? "").trim().slice(0, 40)))));
    await pane.shot("probe-object-tab");
    console.log("interaction attrs", JSON.stringify(await pane.page.evaluate(() => [...document.querySelectorAll("[data-interaction-json]")].map(e => `${e.id.slice(0, 60)} => ${(e.getAttribute("data-interaction-json") ?? "").slice(0, 200)}`))));
    await pane.shot("probe-selected");
  } finally { await pane.context.close(); }
}
await runSteps(["probe"], { probe });
