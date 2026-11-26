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
        const tab = pane.locator('[data-tab-id="framework.panel.artifact"]:visible').first();
        await tab.waitFor({timeout:150000});
        const skip = pane.locator('[id="ui.introduction.skip"],[data-element-alias~="ui.introduction.skip"]').first();
        if(await skip.isVisible()) await skip.click({timeout:5000}).catch(()=>page.keyboard.press("Escape"));
        if(await tab.getAttribute("aria-pressed") !== "true") await tab.click();
        const rows = pane.locator('[role="treeitem"]:visible');
        await rows.first().waitFor({timeout:30000});
        console.log("[DEBUG]",id,JSON.stringify(await rows.evaluateAll(nodes=>nodes.map(x=>[x.id,x.getAttribute("data-element-alias"),x.textContent.slice(0,80)]).slice(0,50))));
      } catch(error) { console.log("[DEBUG]",id,String(error)); }
      finally {await page.close();}
    }
  } finally {await browser.close();}
}
inspect().catch(error=>{console.error(error);process.exitCode=1;});
