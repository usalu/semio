const res={};
await fresh();
await row(/e5 · Beam/); await page.waitForTimeout(1500);
if(!/ID e5/.test(await insp())){await clickBtn("Inspection",{exact:true});} await page.waitForTimeout(2500);
const mat=async()=>(await insp()).match(/Material \| ([^|]+)\|/)?.[1]?.trim();
const waitM=async(re,ms)=>{const t=Date.now();while(Date.now()-t<ms){const m=await mat();if(re.test(m||""))return Date.now()-t;await page.waitForTimeout(300);}return "FAIL:"+await mat();};
res.runs=[];
for(const D of [300,2500,6000]){
  await pickOpt(IS("element.material"),/C30\/37/); await waitM(/C30/,20000);
  await page.waitForTimeout(D);
  await page.keyboard.press("Control+z");
  const u=await waitM(/C24/,14000);
  res.runs.push({D,undo:u});
  if(String(u).startsWith("FAIL")){ await page.waitForTimeout(1500); await page.keyboard.press("Control+z"); res.runs.at(-1).retry=await waitM(/C24/,14000);}
  await page.waitForTimeout(2000);
}
res.log=log.slice(0,10);
return res;
