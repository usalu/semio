/** 📤️ Drives the real served Draw document's PDF form for selector and download diagnostics. */
import { chromium } from "playwright";
import { readFileSync } from "node:fs";
import { assertPdfPainting } from "../../../../../../../✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️oracle.mjs";
const [url,output]=Bun.argv.slice(2);
const browser=await chromium.launch({headless:true,args:["--use-angle=swiftshader","--enable-unsafe-swiftshader","--enable-unsafe-webgpu"]});
try {
  const page=await browser.newPage({viewport:{width:1600,height:1000},acceptDownloads:true});
  page.on("console",m=>{if(m.type()==="error"||m.type()==="warning")console.log("[DEBUG] Draw browser",m.type(),m.text().slice(0,1100));});
  page.on("pageerror",e=>console.log("[DEBUG] Draw page error",e.message));
  await page.goto(url,{waitUntil:"domcontentloaded"});
  const toggle=page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
  await toggle.click({timeout:45000});
  const controls=()=>page.evaluate(()=>[...document.querySelectorAll('[id*="exportDocument"],[role="combobox"],[data-slot="engagement-status"]')].map(n=>[n.id,n.getAttribute("role"),(n.innerText??"").slice(0,100)]));
  console.log("[DEBUG] Draw export controls",JSON.stringify(await controls()));
  await page.locator('[id="action.exportDocument"]').first().click({timeout:6000});
  console.log("[DEBUG] Draw export form",JSON.stringify(await controls()));
  await page.locator('[id$=".action.exportDocument.arg.format.input"][role="combobox"]').first().click({timeout:6000});
  await page.getByRole("option",{name:"PDF",exact:true}).click({timeout:6000});
  const [download]=await Promise.all([
    page.waitForEvent("download",{timeout:50000}),
    page.locator('[id^="framework.window."][id$=".action.exportDocument.execute"]').first().click({timeout:6000}),
  ]);
  await download.saveAs(output);
  const report=await assertPdfPainting(readFileSync(output),1);
  console.log("[DEBUG] Draw PDF bytes",readFileSync(output).length,JSON.stringify(report).slice(0,500));
} finally {await browser.close();}
