const { createRequire } = require("node:module");
const { resolve } = require("node:path");
const project = createRequire(resolve("♻️mit-bestand/🧺️demonstrator/package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const { chromium } = project("@playwright/test");

/** 🧾️ Records the concrete tree-row identifiers of each active document panel. */
async function inspect() {
  const browser = await chromium.launch();
  try {
    for (const id of process.argv.slice(3)) {
      const page = await browser.newPage({locale:"en-US",viewport:{width:1440,height:900}});
      try {
        await page.goto(process.argv[2]+"/#"+id,{waitUntil:"domcontentloaded"});
        const pane = page.locator('[data-shell-id="'+id+'"]');
        await page.locator('[data-shell-id="'+id+'"][data-shell-ready]').waitFor({timeout:140000});
        const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
        if(await skip.isVisible()) await skip.click({timeout:5000}).catch(()=>page.keyboard.press("Escape"));
        const tab = pane.locator('[data-tab-id="fem3d.panel.results"]:visible').first();
        if(!await tab.isVisible()) await pane.locator('[data-tab-id="framework.category.display"]:visible').first().click();
        if(await tab.getAttribute("aria-pressed")!=="true") await tab.click();
        const controls = pane.locator('[id*="fem3d-play-results"]:visible');
        await controls.first().waitFor({timeout:20000});
        console.log("[DEBUG]",JSON.stringify(await controls.evaluateAll(xs=>xs.map(x=>[x.id,x.tagName,x.type,x.textContent.slice(0,40),x.querySelectorAll('[role=slider]').length]))));
      } catch(error) { console.log("[DEBUG]",id,String(error)); }
      finally {await page.close();}
    }
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
