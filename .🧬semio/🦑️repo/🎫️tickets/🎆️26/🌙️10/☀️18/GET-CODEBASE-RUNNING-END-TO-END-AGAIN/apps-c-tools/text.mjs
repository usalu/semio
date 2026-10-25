import{chromium}from"/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const b=await chromium.launch({headless:true});const p=await b.newPage();
p.on("console",m=>console.log("[DEBUG] console",m.type(),m.text().slice(0,300)));
await p.goto(process.env.PROBE_URL,{waitUntil:"domcontentloaded"});await p.waitForTimeout(Number(process.env.PROBE_WAIT??8000));
console.log("[DEBUG] BODY",(await p.evaluate(()=>document.body.innerText)).slice(0,1500));
console.log("[DEBUG] svgs",await p.evaluate(()=>document.querySelectorAll("svg").length));await b.close();
