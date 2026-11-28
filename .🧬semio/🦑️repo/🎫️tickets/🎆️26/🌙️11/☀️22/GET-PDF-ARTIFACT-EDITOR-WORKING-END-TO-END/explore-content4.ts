import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore");
  try {
    await pane.open(content);
    await pane.page.waitForTimeout(3000);
    const empty = async () => (await pane.text()).includes("Select an object");
    await pane.page.getByRole("button", { name: /Vector graphic/ }).first().click();
    await pane.page.waitForTimeout(2000);
    console.log("after row click empty:", await empty());
    await pane.page.locator('[id="mode-dock-tab-3-pdf.object"]').click();
    await pane.page.waitForTimeout(2000);
    console.log("after Object tab empty:", await empty());
    await pane.page.getByRole("button", { name: /Image:/ }).first().click();
    await pane.page.waitForTimeout(2000);
    console.log("after image row empty:", await empty());
    await pane.shot("explore4");
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
