const tog='[id="framework.window.fem3dModel.engagement.toggle"]';
if(!(await page.locator('[id="action.exportArtifactDocument"]').count())) { await page.locator(tog).evaluate(e=>e.click()); await page.waitForTimeout(900); }
const ids=await page.evaluate(()=>[...document.querySelectorAll('[id^="action."]')].map(e=>e.id));
const dl=page.waitForEvent("download",{timeout:15000}).catch(()=>null);
await page.locator('[id="action.exportArtifactDocument"]').evaluate(e=>e.click());
const d=await dl; if(d) await d.saveAs(IMDIR+"/export.semio-archive");
return {ids:ids.filter(i=>/xport|mport/.test(i)),dl:!!d,hx:globalThis.__hx.splice(0)};
