const o={};
await openForm("setSelectionMode");
await runForm("setSelectionMode",{domainId:"fem3d",mode:"Multiple"});
await page.waitForTimeout(3000);
o.hx1=__hx.slice(-6);
await closeAct(); await page.waitForTimeout(800);
await actClick("selectAll"); await page.waitForTimeout(2500);
o.hx2=__hx.slice(-6);
return o;
