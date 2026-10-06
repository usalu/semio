/** 🧪️ Probe: does a native 2D scroll-snap grid of screen-sized cells take touch swipes in both directions in Chromium,
 * over inert pages, one cell per fling, and let a tall card scroll inside its cell first? Run: bun snap_probe.ts */
import { chromium } from "playwright";

const html = `<!doctype html><html><head><meta name="viewport" content="width=device-width,initial-scale=1"><style>
html,body{margin:0;height:100%;overflow:hidden}
#s{position:relative;height:100%;width:100%;overflow:auto;scroll-snap-type:both mandatory;overscroll-behavior:contain}
#g{display:grid;width:300%;height:300%;grid-template-columns:repeat(3,minmax(0,1fr));grid-template-rows:repeat(3,minmax(0,1fr))}
section{position:relative;overflow:hidden;scroll-snap-align:start;scroll-snap-stop:always}
.p{position:absolute;inset:0;background:#ddd}
.l{pointer-events:none;position:absolute;inset:0;display:flex;align-items:safe center;justify-content:center;overflow-y:auto}
.c{pointer-events:auto;width:80%;background:#fff;border:1px solid #000}
</style></head><body><div id="s"><div id="g"></div></div><script>
const g=document.getElementById('g');
const cells=[[0,0],[1,0],[2,0],[0,1],[1,1],[2,1],[1,2]];
cells.forEach(([c,r],i)=>{const s=document.createElement('section');s.style.gridColumn=c+1;s.style.gridRow=r+1;s.id='c'+c+r;
s.innerHTML='<div class="p" inert aria-hidden="true">page '+i+'</div><div class="l"><div class="c" style="height:'+(i===4?1600:200)+'px">card '+i+'</div></div>';g.append(s)});
</script></body></html>`;

const browser = await chromium.launch({ channel: "chromium" });
const context = await browser.newContext({ viewport: { width: 375, height: 812 }, hasTouch: true, isMobile: true, deviceScaleFactor: 2 });
const page = await context.newPage();
await page.setContent(html);
const cdp = await context.newCDPSession(page);
const at = () => page.evaluate(() => { const s = document.getElementById("s")!; return { x: s.scrollLeft / s.clientWidth, y: s.scrollTop / s.clientHeight, card: (document.querySelector("#c11 .l") as HTMLElement).scrollTop }; });
const swipe = async (x: number, y: number, dx: number, dy: number) => {
  const steps = 12;
  await cdp.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ x, y }] });
  for (let index = 1; index <= steps; index++) {
    await cdp.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ x: x + (dx * index) / steps, y: y + (dy * index) / steps }] });
    await page.waitForTimeout(8);
  }
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
  await page.waitForTimeout(900);
  const where = await at();
  console.log(`[DEBUG] swipe ${dx},${dy} from ${x},${y} ->`, JSON.stringify(where));
};
console.log("[DEBUG] start", JSON.stringify(await at()));
await swipe(187, 400, -200, 0);
await swipe(187, 100, 0, -200);
await swipe(187, 400, 0, -200);
await swipe(187, 400, 0, -1500);
await swipe(187, 400, 0, -400);
await swipe(187, 400, 200, 0);
await swipe(30, 30, -250, 0);
await swipe(30, 30, -250, 0);
await swipe(30, 30, 0, -250);
await swipe(30, 30, 0, 250);
await swipe(30, 30, 250, 250);
await browser.close();
