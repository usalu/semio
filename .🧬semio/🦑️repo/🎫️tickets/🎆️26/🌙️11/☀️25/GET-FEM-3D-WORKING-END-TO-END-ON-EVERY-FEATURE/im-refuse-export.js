const o={};
const tog='[id="framework.window.fem3dModel.engagement.toggle"]';
const open=async()=>{ if(!(await page.locator('[id="action.exportArtifactDocument"]').count())) { await page.locator(tog).evaluate(e=>e.click()); await page.waitForTimeout(900); } };
await open();
await page.locator('[id="action.setSelectionMode"]').evaluate(e=>e.click()); await page.waitForTimeout(1200);
const dom=page.locator('[id="framework.window.fem3dModel.action.setSelectionMode.arg.domainId.input"]');
o.dom=await dom.count(); if(o.dom) await dom.fill("fem3d");
o.args=await page.evaluate(()=>[...document.querySelectorAll('[id^="framework.window.fem3dModel.action.setSelectionMode.arg."][id$=".input"]')].map(e=>e.id.split(".arg.")[1]+":"+e.tagName+"="+e.value));
const mode=page.locator('[id="framework.window.fem3dModel.action.setSelectionMode.arg.mode.input"]'); if(await mode.count()){ await mode.click(); await page.waitForTimeout(500); await page.getByRole("option").first().click().catch(()=>{}); }
await page.waitForTimeout(500);
await page.locator('[id="framework.window.fem3dModel.action.setSelectionMode.execute"]').click({timeout:5000}).catch(e=>{o.clickErr=String(e).slice(0,100)}); await page.waitForTimeout(4000);
o.hx1=globalThis.__hx.splice(0);
await open();
const dl=page.waitForEvent("download",{timeout:15000}).catch(()=>null);
await page.locator('[id="action.exportArtifactDocument"]').evaluate(e=>e.click());
o.dl=!!(await dl); await page.waitForTimeout(2000);
o.hx2=globalThis.__hx.splice(0);
return o;
