const t0=Date.now();
for(let i=0;i<80;i++){ const txt=await page.locator("body").innerText().catch(()=>""); if(/Case: dead/.test(txt)) return {ms:Date.now()-t0, ok:true}; await page.waitForTimeout(5000);}
return {ok:false};
