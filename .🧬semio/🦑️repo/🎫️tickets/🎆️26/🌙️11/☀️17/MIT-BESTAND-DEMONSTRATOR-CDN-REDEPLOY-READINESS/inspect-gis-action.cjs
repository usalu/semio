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
    await pane.locator('[id="framework.window.gis2dMain.engagement.toggle"],[data-element-alias~="framework.window.gis2dMain.engagement.toggle"]').first().click();
    const add = pane.getByRole("button",{name:"Add Feature…"});
    await add.waitFor();
    console.log("[DEBUG]", await add.evaluate(x=>[x.closest("[id]")?.id,x.closest("[data-slot=window-action-pane]")?.outerHTML.slice(0,300)]));
    await add.click();
    console.log("[DEBUG]", await pane.locator("input,button").evaluateAll(xs=>xs.filter(x=>x.offsetWidth && (x.tagName==="INPUT" || JSON.stringify(x.outerHTML).includes("execute"))).map(x=>x.outerHTML.slice(0,600))));
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
