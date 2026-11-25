const tog='[id="framework.window.fem3dModel.engagement.toggle"]';
if(!(await page.locator('[id="action.importArtifactDocument"]').count())) { await page.locator(tog).evaluate(e=>e.click()); await page.waitForTimeout(900); }
const ch=page.waitForEvent("filechooser",{timeout:8000}).catch(()=>null);
await page.locator('[id="action.importArtifactDocument"]').evaluate(e=>e.click());
const fc=await ch; if(fc) await fc.setFiles(IMDIR+"/export.semio-archive");
await page.waitForTimeout(8000);
await shot("im-after-import");
return {chooser:!!fc,hx:globalThis.__hx.splice(0),txt:(await page.locator("body").innerText()).replace(/\n+/g," | ").slice(0,600)};
