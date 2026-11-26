const { createRequire } = require("node:module");
const { resolve } = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const { chromium } = project("@playwright/test");

/** ⚡️ Reads actual run controls on the selected Energie tool surface. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({ locale:"en-US",viewport:{width:1440,height:900} });
    await page.goto(process.argv[2] + "/#energie", {waitUntil:"domcontentloaded"});
    const pane=page.locator('[data-shell-id="energie"][data-shell-ready]');
    await pane.waitFor({timeout:150000});
    const skip=pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
    if(await skip.isVisible()) await skip.click({timeout:5000}).catch(()=>page.keyboard.press("Escape"));
    const tree=pane.locator('[id="framework.window.energySimulation"],[data-element-alias~="framework.window.energySimulation"]').getByRole("tree");
    await tree.click();
    const tool=pane.locator('[data-tab-id="tool.energySimulation"]:visible').first();
    if(!await tool.isVisible()) await pane.locator('[data-tab-id="framework.category.tool"]:visible').first().click();
    if(await tool.getAttribute("aria-pressed")!=="true") await tool.click();
    const controls=pane.locator('[id*="framework.toolRun."],[data-ui-node-key^="framework.toolRun."]');
    const sample=()=>controls.evaluateAll(xs=>xs.slice(0,24).map(x=>[x.id,x.getAttribute("data-ui-node-key"),x.tagName]));
    const runsTab=pane.locator('[data-tab-id="framework.panel.toolRun"]:visible').first();
    if(await runsTab.getAttribute("aria-pressed")!=="true") await runsTab.click();
    await controls.first().waitFor({timeout:10000}).catch(()=>{});
    console.log("[DEBUG] before",JSON.stringify(await sample()));
    await tree.click();
    await page.keyboard.press("ControlOrMeta+Enter");
    await controls.filter({visible:true}).first().waitFor({timeout:12000}).catch(()=>{});
    console.log("[DEBUG] after",JSON.stringify(await sample()));
    const tabs=()=>pane.locator("[data-tab-id]:visible").evaluateAll(xs=>xs.map(x=>[x.getAttribute("data-tab-id"),x.getAttribute("aria-pressed")]));
    console.log("[DEBUG] tabs",JSON.stringify(await tabs()));
    const tab=pane.locator('[data-tab-id="framework.panel.toolRun"]:visible').first();
    if(await tab.isVisible() && await tab.getAttribute("aria-pressed")!=="true") await tab.click();
    await controls.first().waitFor({timeout:8000}).catch(()=>{});
    console.log("[DEBUG] panel",JSON.stringify(await sample()));

  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
