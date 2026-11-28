const res={};
if(!globalThis.__h5){globalThis.__h5=true;globalThis.__all=[];page.on("console",m=>{if(globalThis.__rec)globalThis.__all.push(m.type()+" "+m.text().slice(0,1200));});page.on("pageerror",e=>{if(globalThis.__rec)globalThis.__all.push("pageerror "+String(e).slice(0,1200));});}
res.reload=await fresh(); await row(/e5 · Beam/); await page.waitForTimeout(2500);
const mat=async()=>(await insp()).match(/Material \| ([^|]+)\|/)?.[1]?.trim();
if(!(await mat())){await clickBtn("Inspection",{exact:true}); await page.waitForTimeout(3000);}
const waitM=async(re,ms)=>{const t=Date.now();while(Date.now()-t<ms){const m=await mat();if(re.test(m||""))return Date.now()-t;await page.waitForTimeout(300);}return "FAIL:"+await mat();};
res.start=await mat();
if(/C30/.test(res.start)){await pickOpt(IS("element.material"),/C24/); await waitM(/C24/,20000);}
await pickOpt(IS("element.material"),/C30\/37/); res.edit=await waitM(/C30/,25000);
await page.waitForTimeout(4000);
await clickBtn("History",{exact:true}); await page.waitForTimeout(3000);
globalThis.__all.length=0; globalThis.__rec=true;
await page.getByRole("button",{name:"Undo",exact:true}).first().click({timeout:8000});
res.undo=await waitM(/C24/,25000);
await page.waitForTimeout(4000);
globalThis.__rec=false;
res.lines=globalThis.__all.filter(l=>!/retire exit|store initializer|archive\/replacement/.test(l)).slice(0,60);
res.health=await health();
res.shot=await shot("hu5");
return res;
