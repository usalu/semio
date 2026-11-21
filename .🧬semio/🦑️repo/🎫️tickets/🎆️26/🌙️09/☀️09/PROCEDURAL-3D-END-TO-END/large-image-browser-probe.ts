import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { createServer } from "vite";
import { chromium } from "playwright";
import { WINDOW_RESOURCE } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🚚️window-resource/🟦️.ts";

const generated = resolve(import.meta.dirname, "🗑️generated", "large-image-browser");
const native = resolve(generated, "../retained-resource");
const [data, keys] = await Promise.all(["source-uri.txt","native-pages.json"].map(name => readFile(resolve(native,name), "utf8")));
const checkout = process.cwd();
const renderer = resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements");
await mkdir(generated,{recursive:true});
await copyFile(resolve(import.meta.dirname,"large-image-browser-mount.tsx"),resolve(generated,"mount.tsx"));
await writeFile(resolve(generated,"index.html"),'<!doctype html><meta charset="utf-8"><div id="root"></div><script type="module" src="/mount.tsx"></script>');
const server = await createServer({
  configFile:false, root:generated, cacheDir:resolve(generated,"vite-cache"),
  esbuild:{jsx:"automatic",target:"esnext"},
  server:{host:"127.0.0.1",fs:{allow:[checkout]}},
  optimizeDeps:{entries:[resolve(generated,"index.html")],include:["react","react-dom","react-dom/client","react/jsx-runtime","react/jsx-dev-runtime","scheduler"]},
  resolve:{alias:{
    "probe:interpreter":resolve(renderer,"🗣️Interpreter/🟦️.tsx"),
    "probe:document-store":resolve(renderer,"📃️UiDocumentStore/🟦️.tsx"),
    "probe:tree-context":resolve(renderer,"🗣️Interpreter/🪟️tree-windows/🟦️.ts"),
    "probe:tree-scheduler":resolve(renderer,"🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"),
  }},
  plugins:[{name:"resource-fixture",configureServer(server) {
    server.middlewares.use((req,res,next) => {
      if (req.url === "/fixture/pages") { res.setHeader("content-type","application/json"); res.end(keys); return; }
      if (!req.url?.startsWith("/fixture/page/")) return next();
      const start = (Number(req.url.slice(14)) - 1) * WINDOW_RESOURCE.pageBytes;
      res.end(data.slice(start,start + WINDOW_RESOURCE.pageBytes));
    });
  }}],
});
let browser;
try {
  await server.listen();
  browser = await chromium.launch();
  const page = await browser.newPage();
  const errors = [];
  page.on("pageerror", err=>errors.push(err.message));
  await page.goto(server.resolvedUrls.local[0],{waitUntil:"domcontentloaded"});
  const image = page.getByRole("img",{name:"Authored procedural texture",exact:true});
  await page.waitForFunction(()=>document.querySelector("img")?.naturalWidth === 1024);
  const result = await image.evaluate(async image => {
    const bytes = await fetch(image.currentSrc).then(response => response.arrayBuffer());
    const digest = await crypto.subtle.digest("SHA-256",bytes);
    return { source:image.currentSrc, width:image.naturalWidth, height:image.naturalHeight, hash:Array.from(new Uint8Array(digest),byte=>byte.toString(16).padStart(2,"0")).join("") };
  });
  assert.match(result.source,/^blob:/);
  assert.deepEqual([result.width,result.height],[1024,768]);
  assert.equal(result.hash,createHash("sha256").update(Buffer.from(data.slice(data.indexOf(",") + 1),"base64")).digest("hex"));
  const want = Array.from({length:JSON.parse(keys).length},(_,i)=>i + 1).concat(0);
  assert.deepEqual(await page.evaluate(()=>window.imageResourceProof.revisionEvents),want);
  await page.evaluate(()=>window.imageResourceProof.remove());
  await image.waitFor({state:"detached"});
  assert.equal(await page.evaluate(async src=>{try{await fetch(src);return false;}catch{return true;}},result.source),true);
  await page.evaluate(()=>window.imageResourceProof.restore());
  await page.waitForFunction(count => window.imageResourceProof.revisionEvents.length > count,want.length);
  await page.evaluate(()=>window.imageResourceProof.remove());
  await page.waitForFunction(count => window.imageResourceProof.revisionEvents.length > count && window.imageResourceProof.revisionEvents.at(-1) === 0,want.length);
  assert.deepEqual(errors,[]);
  console.log("[DEBUG] Mounted Chromium decoded the exact 1024x768 PNG, released its URL, and cancelled a restarted partial transfer");
} finally { await browser?.close(); await server.close(); }
