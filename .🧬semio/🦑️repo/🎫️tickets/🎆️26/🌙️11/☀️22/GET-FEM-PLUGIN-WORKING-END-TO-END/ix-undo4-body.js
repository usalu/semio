const res={};
await fresh();
await row(/e5 · Beam/); await page.waitForTimeout(1500);
if(!/ID e5/.test(await insp())){await clickBtn("Inspection",{exact:true});} await page.waitForTimeout(2500);
const mat=async()=>(await insp()).match(/Material \| ([^|]+)\|/)?.[1]?.trim();
const waitM=async(re,ms=25000)=>{const t=Date.now();while(Date.now()-t<ms){const m=await mat();if(re.test(m||""))return Date.now()-t;await page.waitForTimeout(400);}return "FAIL:"+await mat();};
res.start=await mat();
await pickOpt(IS("element.material"),/C30\/37/); res.edit=await waitM(/C30/);
res.cycles=[];
for(let i=0;i<3;i++){
  await page.keyboard.press("Control+z"); const u=await waitM(/C24/);
  await page.keyboard.press("Control+Shift+z"); const r=await waitM(/C30/);
  res.cycles.push([u,r]);
}
await page.keyboard.press("Control+z"); await page.waitForTimeout(500);
await row(/e4 · Beam/); await page.waitForTimeout(500); await row(/e5 · Beam/);
res.churn=await waitM(/C24/,25000);
res.health=await health();
await row(/e4 · Beam/); await page.waitForTimeout(2500); res.e4=(await insp()).slice(0,60);
await page.mouse.move(700,500);await page.waitForTimeout(1500);await page.mouse.down();await page.mouse.up();await page.waitForTimeout(4000);res.pick=(await insp()).slice(0,60);
res.log=log.slice(0,10);
return res;
