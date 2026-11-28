const res={};
if(!globalThis.__h){globalThis.__h=true;globalThis.__all=[];page.on("console",m=>{if(globalThis.__rec)globalThis.__all.push(m.type()+" "+m.text().slice(0,900));});}
await row(/e5 · Beam/); await page.waitForTimeout(1500);
await clickBtn("Inspection",{exact:true}); await page.waitForTimeout(3000); res.i0=(await insp()).slice(0,80);
const mat=async()=>(await insp()).match(/Material \| ([^|]+)\|/)?.[1]?.trim();
const waitM=async(re,ms)=>{const t=Date.now();while(Date.now()-t<ms){const m=await mat();if(re.test(m||""))return Date.now()-t;await page.waitForTimeout(300);}return "FAIL:"+await mat();};
res.start=await mat();
await pickOpt(IS("element.material"),/C30\/37/); res.edit=await waitM(/C30/,20000);
await page.waitForTimeout(3000);
globalThis.__all.length=0; globalThis.__rec=true;
await page.keyboard.press("Control+z");
res.undo=await waitM(/C24/,15000);
await page.waitForTimeout(3000);
globalThis.__rec=false;
res.n=globalThis.__all.length;
res.lines=globalThis.__all.filter(l=>!/retire exit|store initializer|archive\/replacement/.test(l)).slice(0,80);
return res;
