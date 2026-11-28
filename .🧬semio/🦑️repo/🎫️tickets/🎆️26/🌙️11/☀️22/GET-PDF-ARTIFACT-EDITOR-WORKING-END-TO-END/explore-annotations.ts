import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const doc = join(import.meta.dir, "🗑️generated/fixtures/ann-doc.pdf");
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore");
  try {
    const o = await pane.open(doc);
    console.log("open", o.notice);
    await pane.page.waitForTimeout(2000);
    const dump = async (label: string) => { const t = await pane.text(); const i = t.indexOf("Select an object"); console.log(label, "selectHint", i >= 0, (await pane.page.evaluate(() => [...document.querySelectorAll('[id*="pdf.object"] input')].map(e => `${e.getAttribute("aria-label")}=${(e as HTMLInputElement).value}`)).then(l => l.slice(0, 12).join(" ; ")))); };
    await pane.page.getByRole("button", { name: /Text: Annotation Fixture/ }).first().click();
    await pane.page.waitForTimeout(3000);
    await dump("text-row");
    await pane.page.getByRole("button", { name: /Annotation: Note body/ }).first().click();
    await pane.page.waitForTimeout(3000);
    await dump("note-row");
    await pane.page.mouse.click(600, 140);
    await pane.page.waitForTimeout(3000);
    await dump("canvas-click-note");
    await pane.press("zoomIn").catch(e => console.log("zoom press fail", String(e).slice(0, 100)));
    await pane.page.waitForTimeout(3000);
    await dump("after-zoom");
    await pane.act("set-annotation", { page: "1", object: "p0:a0", text: "Probe", x: "10", y: "10", width: "20", height: "20" }).then(r => console.log(JSON.stringify(r).slice(0, 400)));
    await pane.page.waitForTimeout(2000);
    await dump("after-set-annotation");
    console.log(await pane.page.evaluate(() => [...document.querySelectorAll('[id*="pdf.object"]')].map(e => e.id.slice(-50)).slice(0, 8).join("\n")));
    await pane.shot("explore-selected");
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
