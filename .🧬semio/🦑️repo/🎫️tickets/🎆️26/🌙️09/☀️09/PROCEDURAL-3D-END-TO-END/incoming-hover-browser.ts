import assert from "node:assert/strict";
import { copyFile,mkdir,readFile,writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
const fixtureFile=resolve(import.meta.dirname,"incoming-channel-hover.json");
const fixture=JSON.parse(await readFile(fixtureFile,"utf8"));
const generated=resolve(import.meta.dirname,"🗑️generated/incoming-hover-browser");
const native=resolve(generated,"../incoming-channel-hover/native.json");
const renderer=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
await mkdir(generated,{recursive:true});
await copyFile(resolve(import.meta.dirname,"incoming-hover-browser-mount.tsx"),resolve(generated,"mount.tsx"));
await writeFile(resolve(generated,"index.html"),'<!doctype html><meta charset="utf-8"><div id="root"></div><script type="module" src="/mount.tsx"></script>');
const server=await createServer({
 configFile:false,root:generated,cacheDir:resolve(generated,"vite-cache"),esbuild:{jsx:"automatic"},
 server:{host:"127.0.0.1",fs:{allow:[process.cwd()]}},
 optimizeDeps:{entries:[resolve(generated,"index.html")],include:["react","react-dom","react-dom/client","react/jsx-runtime","react/jsx-dev-runtime","scheduler"]},
 resolve:{alias:{
  "probe:interpreter":resolve(renderer,"🧱️elements/🗣️Interpreter/🟦️.tsx"),
  "probe:document-store":resolve(renderer,"🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
  "probe:styles":resolve(renderer,"🎨️.css"),
  "probe:fixture":fixtureFile,
  "probe:native":native,
 }},
 plugins:[tailwind()],
});
let browser;
try{
 await server.listen();
 browser=await chromium.launch();
 const page=await browser.newPage();
 const errors=[];
 page.on("pageerror",error=>errors.push(error.message));
 await page.goto(server.resolvedUrls.local[0],{waitUntil:"domcontentloaded"});
 const row=id=>page.getByText(id,{exact:true});
 await row("source@surface").waitFor({state:"visible",timeout:12000});
 const origin=page.getByRole("heading",{name:"Plane origin",exact:true}).locator("xpath=ancestor::section[1]");
 const normal=page.getByRole("heading",{name:"Plane normal",exact:true}).locator("xpath=ancestor::section[1]");
 await page.getByRole("spinbutton",{name:"Plane origin x"}).focus();
 for(const [index,entry] of fixture.cases.entries()){
  await page.evaluate(index=>window.incomingHoverProof.publish(index),index);
  await page.waitForFunction(({candidates,expected})=>{
    const rows=[...document.querySelectorAll('[data-slot="tree-item-row"][data-highlighted=true]')];
    return JSON.stringify(candidates.filter(id=>rows.some(row=>row.textContent?.trim()===id)))===JSON.stringify(expected);
  },{candidates:fixture.candidates.map(row=>row.id),expected:entry.expected},{timeout:10000});
  const groupPaint=entry.expected.includes("free@!iplane")?"hover":null;
  assert.equal(await origin.getAttribute("data-shared-border-state"),groupPaint,entry.name);
  assert.equal(await normal.getAttribute("data-shared-border-state"),groupPaint,entry.name);
  assert.equal(await row("source@!isurface").evaluate(node=>node.closest('[role="treeitem"]')?.getAttribute("aria-selected")),"true");
  assert.equal(await page.getByRole("spinbutton",{name:"Plane origin x"}).evaluate(node=>node===document.activeElement),true);
 }
 assert.deepEqual(await page.evaluate(()=>window.incomingHoverProof.intents),[]);
 assert.deepEqual(errors,[]);
 console.log("[DEBUG] Mounted incoming highlights matched native directional sets, painted both coordinate groups and kept selection and axis focus");
} finally { await browser?.close(); await server.close(); }
