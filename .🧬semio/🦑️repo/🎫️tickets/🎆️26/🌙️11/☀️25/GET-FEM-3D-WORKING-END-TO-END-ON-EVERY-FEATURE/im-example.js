const o={};
const crypto=await import("node:crypto");
const hash=async()=>crypto.createHash("md5").update(await page.screenshot({clip:{x:320,y:60,width:470,height:860}})).digest("hex").slice(0,8);
const cap=async()=>(await page.getByRole("tabpanel",{name:"Results"}).locator("p").first().innerText().catch(()=>"?")).trim();
const counts=async()=>await page.evaluate(()=>[...document.querySelectorAll("[role=button]")].map(b=>b.textContent.trim()).filter(x=>/^(Nodes|Elements|Solids|Load Cases) \(\d+\)/.test(x)).join(","));
o.h0=await hash(); o.c0=await counts(); o.cap0=await cap();
const t0=Date.now();
const combo=page.getByRole("combobox",{name:/Example/i}).first();
o.comboCount=await page.getByRole("combobox",{name:/Example/i}).count();
await combo.click(); await page.waitForTimeout(500);
await page.getByRole("option",{name:EXAMPLE,exact:true}).click();
const t=Date.now(); o.steps=[];
for(let i=0;i<MAXS;i++){ await page.waitForTimeout(2000); const h=await hash(), c=await cap(); if(i%5===0||h!==o.h0&&!o.firstChange){ o.steps.push([Math.round((Date.now()-t)/1000),h,c.slice(0,40),await counts()]); } if(h!==o.h0&&!o.firstChange) o.firstChange=Math.round((Date.now()-t)/1000); if(/^Case: [^·]+$/.test(c)&&o.firstChange){o.solved=Math.round((Date.now()-t)/1000);break;} }
o.hx=globalThis.__hx.splice(0).slice(0,6);
await shot("im-ex-"+EXAMPLE.replace(/\W/g,""));
return o;
