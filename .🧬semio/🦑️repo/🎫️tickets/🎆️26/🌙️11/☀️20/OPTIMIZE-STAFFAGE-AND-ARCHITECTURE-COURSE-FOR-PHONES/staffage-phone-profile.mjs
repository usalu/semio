import { chromium, devices } from "playwright";
import fs from "node:fs";
import path from "node:path";
import {fileURLToPath} from "node:url";

const browser = await chromium.launch({ channel: "chromium" });
try {
  const page = await browser.newPage({ ...devices["iPhone 13"], locale: "en-US", viewport: { width: 320, height: 640 } });
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Profiler.enable");
  await cdp.send("Profiler.start");
  const folder = path.join(path.dirname(fileURLToPath(import.meta.url)), "🗑️generated");
  fs.mkdirSync(folder, {recursive:true});
  await page.addInitScript(() => {
    window.staffageFrames = 0;
    window.staffageLong = [];
    new PerformanceObserver(e => window.staffageLong.push(...e.getEntries().map(({ duration }) => duration))).observe({type:"longtask"});
    const count = ()=>{window.staffageFrames++;requestAnimationFrame(count);};
    requestAnimationFrame(count);
  });
  await page.goto("http://127.0.0.1:6271", {waitUntil:"domcontentloaded"});
  const next = page.locator('[data-page="overview"] a[href="#/humans"]');
  await next.waitFor();
  console.log("[DEBUG] before",await page.evaluate(()=>({frames:window.staffageFrames,viewport:[visualViewport.width,visualViewport.height],rect:document.querySelector('[data-page="overview"] a[href="#/humans"]').getBoundingClientRect().toJSON(), long:window.staffageLong.slice(-10)})));
  try { await next.tap({timeout:5000}); } catch (e) { console.log("[DEBUG] tap timed out",e.message); }
  console.log("[DEBUG] after", await page.evaluate(()=>({url: location.href, frames:window.staffageFrames,svg:document.querySelectorAll("svg").length,nodes:document.querySelectorAll("*").length, long:window.staffageLong.slice(-10)})));
  const {profile} = await cdp.send("Profiler.stop");
  const cost = new Map();
  profile.samples.forEach((id, i) => cost.set(id,(cost.get(id)??0)+profile.timeDeltas[i]));
  console.log("[DEBUG] CPU hot frames", profile.nodes.filter(node => cost.has(node.id)).map(({id,callFrame})=>[Math.round(cost.get(id)/1000),callFrame.functionName,decodeURIComponent(callFrame.url),callFrame.lineNumber]).sort((a,b)=>b[0]-a[0]).slice(0,12));
  fs.writeFileSync(path.join(folder,"staffage-browser-profile.json"),JSON.stringify(profile));
} finally {
  await browser.close();
}
