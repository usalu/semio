const o={};
await page.locator('[id="action.addNode"]').evaluate(e=>e.click()); await page.waitForTimeout(1200);
o.args=await page.evaluate(()=>[...document.querySelectorAll('[id^="framework.window.fem3dModel.action.addNode.arg."][id$=".input"]')].map(e=>e.id.split(".arg.")[1]+"="+e.value));
for(const [k,v] of Object.entries({id:"zz1",x:"1",y:"2",z:"3"})){const l=page.locator(`[id="framework.window.fem3dModel.action.addNode.arg.${k}.input"]`); if(await l.count()) await l.fill(v);}
await page.locator('[id="framework.window.fem3dModel.action.addNode.execute"]').click();
await page.waitForTimeout(6000);
o.hx=globalThis.__hx.splice(0);
o.txt=(await page.locator("body").innerText()).replace(/\n+/g," | ").match(/Unsaved[^|]*/)?.[0];
await shot("im-after-edit");
return o;
