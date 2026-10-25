import{chromium}from"/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import{writeFileSync,mkdirSync}from"node:fs";
import{join}from"node:path";
const url=process.env.PROBE_URL,out=process.env.PROBE_OUT,seconds=Number(process.env.PROBE_SECONDS??60),settle=process.env.PROBE_SELECTOR??"body";
mkdirSync(out,{recursive:true});
const lines=[],t0=Date.now();
const browser=await chromium.launch({headless:true,args:["--use-angle=metal","--ignore-gpu-blocklist"]});
const page=await browser.newPage({viewport:{width:1440,height:900}});
page.on("console",m=>lines.push(`${Date.now()-t0} ${m.type()} ${m.text().slice(0,1500)}`));
page.on("pageerror",e=>lines.push(`${Date.now()-t0} pageerror ${String(e).slice(0,1500)}`));
page.on("requestfailed",r=>lines.push(`${Date.now()-t0} requestfailed ${r.url().slice(0,200)}`));
const snap=()=>page.evaluate(()=>({title:document.title,svg:document.querySelectorAll("svg").length,buttons:document.querySelectorAll("button").length,canvases:document.querySelectorAll("canvas").length,rootChildren:document.getElementById("root")?.childElementCount??null,text:document.body.innerText.replace(/\s+/g," ").slice(0,800)}));
await page.goto(url,{waitUntil:"domcontentloaded"});
let last=null;
for(let i=0;i<seconds;i++){await page.waitForTimeout(1000);last=await snap();if(last.rootChildren&&last.svg>0&&i>5)break;}
writeFileSync(join(out,"console.txt"),lines.join("\n"));writeFileSync(join(out,"state.json"),JSON.stringify(last,null,2));
try{await page.screenshot({path:join(out,"final.png"),animations:"disabled",timeout:20000});}catch(e){lines.push("screenshot failed "+String(e).slice(0,200));writeFileSync(join(out,"console.txt"),lines.join("\n"));}
console.log("[DEBUG] PROBE",JSON.stringify(last),"errors",lines.filter(l=>/ error | pageerror /.test(l)).length);
await browser.close();
