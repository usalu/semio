import assert from "node:assert/strict";
import { copyFile,mkdir,readFile,writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
const generated=resolve(import.meta.dirname,"🗑️generated/inspector-text-browser");
await mkdir(generated,{recursive:true});
const fixtureFile=resolve(import.meta.dirname,"inspector-text-contract.json");
const fixture=JSON.parse(await readFile(fixtureFile,"utf8"));
const compose=({prefix,unit,repeat,suffix})=>prefix+unit.repeat(repeat)+suffix;
const renderer=resolve("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
await copyFile(resolve(import.meta.dirname,"inspector-text-browser-mount.tsx"),resolve(generated,"mount.tsx"));
await writeFile(resolve(generated,"index.html"),'<!doctype html><meta charset="utf-8"><div id="root"></div><script type="module" src="/mount.tsx"></script>');
const server=await createServer({
 configFile:false,root:generated,cacheDir:resolve(generated,"vite-cache"),esbuild:{jsx:"automatic"},
 server:{host:"127.0.0.1",fs:{allow:[process.cwd()]}},
 optimizeDeps:{entries:[resolve(generated,"index.html")],include:["react","react-dom","react-dom/client","react/jsx-runtime","react/jsx-dev-runtime","scheduler"]},
 resolve:{alias:{
  "probe:interpreter":resolve(renderer,"🧱️elements/🗣️Interpreter/🟦️.tsx"),
  "probe:document-store":resolve(renderer,"🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
  "probe:action-bridge":resolve(renderer,"🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"),
  "probe:styles":resolve(renderer,"🎨️.css"),
  "probe:fixture":fixtureFile,
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
 const field=page.getByRole("textbox",{name:"Note value",exact:true});
 assert.equal(await field.inputValue(),compose(fixture.edit.first));
 const next=compose(fixture.edit.next);
 await field.fill(next);
 await field.dispatchEvent("keydown",{key:"Enter",isComposing:true,bubbles:true,ctrlKey:true});
 assert.equal(await page.evaluate(()=>window.inspectorProof.actions.length),0);
 await field.press("Control+Enter");
 await page.waitForFunction(()=>window.inspectorProof.actions.length===1);
 const observed=await page.evaluate(()=>window.inspectorProof.actions[0]);
 assert.equal(observed.owner,"procedural.play.inspection");
 assert.equal(observed.action.action,"setWidgetInput");
 assert.equal(observed.action.args.widgetId,"note-雪");
 assert.equal(observed.action.args.channel,"text");
 assert.equal(observed.action.args.value.flat().join(""),next);
 assert.ok(observed.action.args.value.flat().every(chunk=>Buffer.byteLength(chunk)<=512));
 await page.reload({waitUntil:"domcontentloaded"});
 assert.equal(await field.inputValue(),next);
 await field.fill("discard");
 await field.press("Escape");
 assert.equal(await field.inputValue(),next);
 const short="short";
 await field.fill(short);
 await field.press("Control+Enter");
 await page.waitForFunction(()=>window.inspectorProof.actions.length===1);
 assert.equal(await page.evaluate(()=>window.inspectorProof.actions[0].action.args.value),short);
 await page.goto(server.resolvedUrls.local[0]+"?locale=de",{waitUntil:"domcontentloaded"});
 const german=page.getByRole("textbox",{name:"Notizwert",exact:true});
 assert.equal(await german.inputValue(),short);
 await german.fill(next);
 await german.press("Control+Enter");
 await page.waitForFunction(()=>window.inspectorProof.actions[0]?.action.args.value.flat().join("")===window.sessionStorage.getItem("note-text"));
 await page.reload({waitUntil:"domcontentloaded"});
 assert.equal(await german.inputValue(),next);
 assert.deepEqual(errors,[]);
 console.log("[DEBUG] Mounted Inspector text edit retained its widget, channel, locale, Unicode, IME, Escape and reopen behavior");
}finally{await browser?.close();await server.close();}
