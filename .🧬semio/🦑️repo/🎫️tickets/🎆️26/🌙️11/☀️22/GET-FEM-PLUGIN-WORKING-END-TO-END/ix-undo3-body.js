const res={};
await fresh();
await row(/e5 · Beam/); await page.waitForTimeout(1500);
if(!/ID e5/.test(await insp())){await clickBtn("Inspection",{exact:true});} await page.waitForTimeout(2500);
res.start=(await insp()).match(/Material \| ([^|]+)\|/)?.[1];
await pickOpt(IS("element.material"),/C30\/37/);
await page.waitForTimeout(10000);
res.afterEdit=(await insp()).match(/Material \| ([^|]+)\|/)?.[1];
const t0=Date.now(); const utr=[];
await page.keyboard.press("Control+z");
for(let i=0;i<30;i++){await page.waitForTimeout(1000);const m=(await insp()).match(/Material \| ([^|]+)\|/)?.[1];if(utr.at(-1)?.[1]!==m)utr.push([Date.now()-t0,m]);}
res.trace=utr;
await clickBtn("History",{exact:true}); await page.waitForTimeout(2500);
res.redoEnabled=await page.getByRole("button",{name:"Redo",exact:true}).first().isEnabled().catch(()=>"?");
res.rows=(await page.getByRole("treeitem").allTextContents()).filter(t=>/Change|Undo|Redo|Set|Add/.test(t)).slice(0,12);
await shot("ix/undo3");
res.health=await health();
res.log=log.slice(0,10);
return res;
