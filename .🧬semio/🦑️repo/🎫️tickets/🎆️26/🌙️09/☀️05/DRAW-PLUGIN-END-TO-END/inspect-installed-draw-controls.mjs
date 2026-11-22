/** 🎛️ Reads the actual Draw engagement and Layers controls from the served React shell. */
import { chromium } from "playwright";
const browser = await chromium.launch({ headless:true, args:["--use-angle=swiftshader","--enable-unsafe-swiftshader","--enable-unsafe-webgpu"] });
try {
  const page=await browser.newPage({viewport:{width:1600,height:1000}});
  page.on("pageerror",e=>console.log("[DEBUG] Draw page error",e.message));
  await page.goto(Bun.argv[2],{waitUntil:"domcontentloaded"});
  const toggle=page.locator('[id^="framework.window."][id$=".engagement.toggle"]').first();
  await toggle.waitFor({state:"visible",timeout:45000});
  await toggle.click({timeout:5000});
  const inspect=()=>page.evaluate(()=>({controls:[...document.querySelectorAll("[id]")].filter(n=>n.id.includes("drawing-")||n.id.startsWith("framework.panel")||n.id.endsWith("engagement.toggle")).map(n=>({id:n.id,slot:n.dataset.slot,role:n.getAttribute("role"),text:(n.innerText??"").slice(0,100)})),body:document.body.innerText.slice(0,3000),html:[document.documentElement.lang,document.documentElement.dataset.semioOsReady]}));
  console.log("[DEBUG] Draw expanded engagement",JSON.stringify(await inspect()));
  await page.locator('button[id="framework.panel.artifact"]').first().click({timeout:5000});
  await page.waitForTimeout(700);
  console.log("[DEBUG] Draw layers panel",JSON.stringify(await inspect()));
} finally { await browser.close(); }
