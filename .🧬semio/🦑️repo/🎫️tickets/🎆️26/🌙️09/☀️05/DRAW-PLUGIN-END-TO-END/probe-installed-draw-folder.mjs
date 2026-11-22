/** 📂️ Checks the real Draw folder attachment and utility controls in the currently served shell. */
import { chromium } from "playwright";
const [url,folder] = Bun.argv.slice(2);
const browser=await chromium.launch({headless:true,args:["--use-angle=swiftshader","--enable-unsafe-swiftshader","--enable-unsafe-webgpu"]});
try {
  const context=await browser.newContext({locale:"de",viewport:{width:1600,height:1000}});
  const page=await context.newPage();
  page.on("pageerror",e=>console.log("[DEBUG] Draw page error",e.message));
  page.on("console",message=>{if(message.type()==="error"||message.type()==="warning")console.log("[DEBUG] Draw browser",message.type(),message.text().slice(0,900));});
  const now=()=>page.evaluate(()=>({body:document.body.innerText.slice(-3300),ids:[...document.querySelectorAll('[id*="utility"],[id*="sync."],[id*="s-sync-"],[id*="shapeRect"]')].map(n=>[n.id,n.getAttribute("aria-pressed"),n.dataset.slot])}));
  await page.goto(url,{waitUntil:"domcontentloaded"});
  const sync=page.locator('[id="s-sync-status"]').first();
  await sync.click({timeout:45000});
  console.log("[DEBUG] Draw sync panel",JSON.stringify(await now()));
  const utility=page.locator('[id="framework.sync.folder"]').first();
  if(!(await utility.isVisible())) await page.locator('[id="ui.utilities.group.sync"]').first().click({timeout:5000});
  await utility.click({timeout:5000});
  await page.locator('[id="framework.sync.folder.path"]').fill(folder,{timeout:5000});
  console.log("[DEBUG] Draw folder card",JSON.stringify(await now()));
  await page.getByRole("button",{name:/^(attach|verbinden)$/i}).click({timeout:5000});
  await page.locator('[id="framework.sync.folder.path"]').waitFor({state:"hidden",timeout:5000});
  await utility.click({timeout:5000});
  await page.waitForFunction(()=>document.querySelector("[data-semio-sync-status]")?.textContent?.toLowerCase().includes("gespeichert"),undefined,{timeout:18000}).catch(async e=>{console.log("[DEBUG] Draw folder after attachment",JSON.stringify(await now()));throw e;});
  console.log("[DEBUG] Draw folder saved",JSON.stringify(await now()));
} catch(error) {
  console.error(error);
  process.exitCode=1;
} finally {await browser.close();}
