const res={};
globalThis.__all ??= []; if(!globalThis.__h){globalThis.__h=true;page.on("console",m=>{const t=m.text();if(globalThis.__rec)globalThis.__all.push(m.type()+" "+t.slice(0,900));});}
//nofresh
await row(/e5 · Beam/); await page.waitForTimeout(1500);
if(!/ID e5/.test(await insp())){await clickBtn("Inspection",{exact:true});} await page.waitForTimeout(2500);
const mat=async()=>(await insp()).match(/Material \| ([^|]+)\|/)?.[1]?.trim();
const waitM=async(re,ms)=>{const t=Date.now();while(Date.now()-t<ms){const m=await mat();if(re.test(m||""))return Date.now()-t;await page.waitForTimeout(300);}return "FAIL:"+await mat();};
res.start=await mat();
await pickOpt(IS("element.material"),/C30\/37/); res.edit=await waitM(/C30/,20000);
await page.waitForTimeout(8000);
globalThis.__all.length=0; globalThis.__rec=true;
await page.keyboard.press("Control+z");
res.undo=await waitM(/C24/,15000);
globalThis.__rec=false;
res.n=globalThis.__all.length;
res.lines=globalThis.__all.filter(l=>!/retire exit|store initializer|archive\/replacement/.test(l)).slice(0,60);
return res;
