import { resolve } from "node:path";
process.env.PLAYWRIGHT_BROWSERS_PATH ??= resolve(process.cwd(), ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright");
const { chromium } = await import("playwright");
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const pdfjs = require.resolve("pdfjs-dist/build/pdf.mjs");
const worker = require.resolve("pdfjs-dist/build/pdf.worker.mjs");
const root = new URL("./🗑️generated/", import.meta.url);
const server = Bun.serve({port:0, fetch(req) { const path = new URL(req.url).pathname; if(path === "/pdf.mjs") return new Response(Bun.file(pdfjs), {headers:{"Content-Type":"text/javascript"}}); if(path === "/worker.mjs") return new Response(Bun.file(worker), {headers:{"Content-Type":"text/javascript"}}); if(/^\/sample-(18|24)\.pdf$/.test(path)) return new Response(Bun.file(fileURLToPath(new URL("pdfium-input-" + path.match(/\d+/)[0] + ".pdf", root))), {headers:{"Content-Type":"application/pdf"}}); return new Response("<!doctype html><html><body></body></html>",{headers:{"Content-Type":"text/html"}}); }});
let browser;
const deadline = setTimeout(async () => { await browser?.close(); server.stop(true); process.exit(124); }, 30000);
try {
 browser = await chromium.launch({headless:true,args:["--use-gl=angle","--use-angle=swiftshader","--enable-unsafe-swiftshader"]});
 const page = await browser.newPage();
 await page.goto(server.url.href);
 const result = await page.evaluate(async () => {
  const pdfjs = await import("/pdf.mjs"); pdfjs.GlobalWorkerOptions.workerSrc = "/worker.mjs";
  const results = {};
  for (const number of [18,24]) { const task=pdfjs.getDocument("/sample-"+number+".pdf"); const doc=await task.promise; const sheet=await doc.getPage(1); const viewport=sheet.getViewport({scale:256}); const canvas=document.createElement("canvas"); canvas.width=Math.ceil(viewport.width); canvas.height=Math.ceil(viewport.height); const context=canvas.getContext("2d"); await sheet.render({canvas,canvasContext:context,viewport}).promise; const points=[0.25,0.5,0.75].map(x=>({x,rgba:Array.from(context.getImageData(Math.floor(canvas.width*x),Math.floor(canvas.height/2),1,1).data)})); results[number]={width:canvas.width,height:canvas.height,points}; await task.destroy(); }
  return results;
 });
 console.log(JSON.stringify(result));
} finally {clearTimeout(deadline); await browser?.close(); server.stop(true)}
