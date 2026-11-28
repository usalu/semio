import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
const objFields = (pane: Awaited<ReturnType<typeof session>>) => pane.page.evaluate(() => [...document.querySelectorAll('[id^="window:pdf.object/"]')].map(e => `${e.tagName}|${e.id.replace("window:pdf.object/", "")}|${e.getAttribute("aria-label")}|${(e as HTMLInputElement).value ?? ""}`).filter(l => !/pdf-(page|doc|entry|add|merge|replace|extract|insert|move|remove)/.test(l)));
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore");
  try {
    console.log((await pane.open(content)).notice);
    await pane.page.waitForTimeout(3000);
    await pane.page.getByRole("button", { name: /Text: Rich Fixture Helvetica/ }).first().click();
    for (const ms of [500, 2000, 4000]) { await pane.page.waitForTimeout(ms); console.log(ms, (await objFields(pane)).join(" ; "), (await pane.text()).includes("Select an object")); }
    await pane.page.mouse.click(300, 200);
    await pane.page.waitForTimeout(2000);
    console.log("after canvas click", (await objFields(pane)).join(" ; "), (await pane.text()).includes("Select an object"));
    await pane.shot("explore2");
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
