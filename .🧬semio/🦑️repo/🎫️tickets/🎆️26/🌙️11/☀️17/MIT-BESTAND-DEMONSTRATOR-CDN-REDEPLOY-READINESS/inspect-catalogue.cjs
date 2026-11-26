const {createRequire} = require("node:module");
const {resolve} = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const {chromium} = project("@playwright/test");

/** 🪟️ Inspects mounted Catalogue controls and records the actual window compute state. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage({locale:"en-US", viewport:{width:1440,height:900}});
    await page.goto(process.argv[2],{waitUntil:"domcontentloaded"});
    const pane = page.locator('[data-shell-id="koordinator"]');
    await pane.locator(".semio-world-3d-host canvas").first().waitFor({timeout:150000});
    const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
    if(await skip.isVisible()) await page.keyboard.press("Escape");
    const states = await pane.locator('[role="tabpanel"]').evaluateAll(xs => xs.map(x => ({label:x.getAttribute("aria-label"),id:x.id,counts:[...x.querySelectorAll(".semio-world-3d-host")].map(w => [w.dataset.meshesJson?.length,w.dataset.instancesJson?.length]),phase:x.querySelector('[data-slot="world-compute-status"]')?.getAttribute("data-compute-phase")})));
    const button = pane.getByRole("button",{name:"Catalogue",exact:true});
    await button.waitFor({timeout:30000});
    console.log("[DEBUG]", states, await button.evaluate(x=>({id:x.id,tab:x.dataset.tabId,alias:x.dataset.elementAlias})));
    await button.click({timeout:10000});
    const id = "cad-play-catalogue.spatial.shape.primitive.box";
    const semantic = pane.locator('[id="'+id+'"],[data-element-alias~="'+id+'"]');
    const scoped = pane.locator('[id$="/'+id+'"]');
    console.log("[DEBUG] controls",await semantic.count(),await scoped.count(),await scoped.evaluate(x=>({outer:x.outerHTML.slice(0,1400),labels:x.querySelectorAll('[data-slot="tree-label"]').length})));
    await scoped.getByText("Box",{exact:true}).click({timeout:12000});
    await page.waitForTimeout(4000);
    console.log("[DEBUG] shape IDs",await pane.locator(".semio-world-3d-host").first().getAttribute("data-instances-json").then(s=>JSON.parse(s).map(i=>i.id)));
    console.log("[DEBUG]",await pane.locator("[id]").evaluateAll(xs=>xs.filter(x=>/catalogue|create|box|solid/i.test(x.id)&&x.getClientRects().length).map(x=>[x.id,x.textContent.slice(0,60)]).slice(0,50)));
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
