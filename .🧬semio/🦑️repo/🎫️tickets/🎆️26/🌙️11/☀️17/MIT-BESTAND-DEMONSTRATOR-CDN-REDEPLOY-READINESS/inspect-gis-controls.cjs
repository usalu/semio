const {createRequire} = require("node:module");
const {resolve} = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const {chromium} = project("@playwright/test");

/** 🗺️ Inspects the live count rows and their authored panel scopes. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({locale:"en-US"});
    await page.goto(process.argv[2],{waitUntil:"domcontentloaded"});
    const pane = page.locator('[data-shell-id="verfolgen"]');
    await page.locator('[data-shell-id="verfolgen"][data-shell-ready]').waitFor({timeout:140000});
    const button = pane.locator('[data-tab-id="framework.panel.inspection"]:visible').first();
    await button.waitFor({timeout:150000});
    const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
    if(await skip.isVisible()) await skip.click({timeout:5000}).catch(()=>page.keyboard.press("Escape"));
    if(await button.getAttribute("aria-pressed") !== "true") await button.click();
    const rows = pane.getByRole("treeitem");
    await rows.first().waitFor({timeout:60000});
    console.log("[DEBUG]",JSON.stringify(await rows.evaluateAll(xs=>xs.map(x=>({id:x.id,aria:x.getAttribute("aria-label"),text:x.textContent,children:[...x.querySelectorAll("[data-slot]")].map(y=>[y.dataset.slot,y.textContent])})))));
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
