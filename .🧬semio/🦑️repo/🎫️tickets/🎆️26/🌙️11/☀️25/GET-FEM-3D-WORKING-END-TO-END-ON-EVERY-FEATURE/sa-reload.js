await page.reload({waitUntil:"domcontentloaded", timeout:120000});
await page.waitForTimeout(2000);
globalThis.__hx = [];
const t0=Date.now();
for(let i=0;i<60;i++){ const xt=await page.locator("body").innerText().catch(()=>""); if(/Case: dead/.test(xt)) return {ms:Date.now()-t0, ok:true}; await page.waitForTimeout(5000);}
return {ok:false};
