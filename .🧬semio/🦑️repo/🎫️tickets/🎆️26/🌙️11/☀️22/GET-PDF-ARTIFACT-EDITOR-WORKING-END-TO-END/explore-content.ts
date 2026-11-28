import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";

const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");

async function dump(pane: Awaited<ReturnType<typeof session>>, label: string) {
  const fields = await pane.page.evaluate(() => [...document.querySelectorAll("input,select,textarea,button")].filter(e => !/^action\./.test(e.id) && e.closest('[data-slot]') !== null && !e.closest('[data-slot="window-action-pane"]')).map(e => `${e.tagName}|${e.id.slice(-60)}|${e.getAttribute("aria-label")}|${(e as HTMLInputElement).value ?? ""}|${(e.textContent ?? "").trim().slice(0, 30)}`));
  console.log(`--- ${label}\n${fields.join("\n")}`);
}

async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore");
  try {
    console.log((await pane.open(content)).notice);
    await pane.page.waitForTimeout(3000);
    await pane.page.getByRole("button", { name: /Text: Rich Fixture Helvetica/ }).first().click();
    await pane.page.waitForTimeout(1500);
    await pane.shot("explore-sel-text");
    await dump(pane, "text selected");
    await pane.page.getByRole("button", { name: /Vector graphic/ }).first().click();
    await pane.page.waitForTimeout(1500);
    await dump(pane, "vector selected");
    await pane.page.getByRole("button", { name: /Image:/ }).first().click();
    await pane.page.waitForTimeout(1500);
    await dump(pane, "image selected");
    await pane.shot("explore-sel-image");
  } finally { await pane.context.close(); }
}

await runSteps(["explore"], { explore });
