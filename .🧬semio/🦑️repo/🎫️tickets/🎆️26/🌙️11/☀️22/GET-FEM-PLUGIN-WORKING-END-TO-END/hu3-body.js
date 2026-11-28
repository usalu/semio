const res={};
res.ms=await fresh();
await row(/e5 · Beam/); await page.waitForTimeout(2500);
res.insp=(await insp()).slice(0,300);
res.shot=await shot("hu3");
res.log=log.splice(0).filter(l=>!/\[DEBUG\]/.test(l)).slice(0,10);
return res;
