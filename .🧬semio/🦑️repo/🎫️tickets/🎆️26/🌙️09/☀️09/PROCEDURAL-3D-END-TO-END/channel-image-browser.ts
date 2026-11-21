import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { copyFile,mkdir,readFile,writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { WINDOW_RESOURCE } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/🚚️window-resource/🟦️.ts";

const generated=resolve(import.meta.dirname,"🗑️generated/channel-image-browser");
const native=resolve(generated,"../channel-image");
const [data,keys]=await Promise.all(["source-uri.txt","native-pages.json"].map(name=>readFile(resolve(native,name),"utf8")));
const checkout=process.cwd();
const renderer=resolve(checkout,"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const elements=resolve(renderer,"🧱️elements");
const fixtureFile=resolve(checkout,"✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/🖼️images.json");
const fixture=JSON.parse(await readFile(fixtureFile,"utf8"));
await mkdir(generated,{recursive:true});
await copyFile(resolve(import.meta.dirname,"channel-image-browser-mount.tsx"),resolve(generated,"mount.tsx"));
await writeFile(resolve(generated,"index.html"),'<!doctype html><meta charset="utf-8"><div id="root"></div><script type="module" src="/mount.tsx"></script>');
const server=await createServer({
  configFile:false,root:generated,cacheDir:resolve(generated,"vite-cache"),
  esbuild:{jsx:"automatic",target:"esnext"},
  server:{host:"127.0.0.1",fs:{allow:[checkout]}},
  optimizeDeps:{entries:[resolve(generated,"index.html")],include:["react","react-dom","react-dom/client","react/jsx-runtime","react/jsx-dev-runtime","scheduler"]},
  resolve:{alias:{
    "probe:interpreter":resolve(elements,"🗣️Interpreter/🟦️.tsx"),
    "probe:document-store":resolve(elements,"📃️UiDocumentStore/🟦️.tsx"),
    "probe:tree-context":resolve(elements,"🗣️Interpreter/🪟️tree-windows/🟦️.ts"),
    "probe:tree-scheduler":resolve(elements,"🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"),
    "probe:styles":resolve(renderer,"🎨️.css"),
    "probe:fixture":fixtureFile,
  }},
  plugins:[tailwind(),{name:"resource-fixture",configureServer(server) {
    server.middlewares.use((req,res,next)=>{
      if(req.url==="/fixture/native"){res.setHeader("content-type","application/json");res.end(keys);return;}
      if(!req.url?.startsWith("/fixture/page/"))return next();
      const start=(Number(req.url.slice(14))-1)*WINDOW_RESOURCE.pageBytes;
      res.end(data.slice(start,start+WINDOW_RESOURCE.pageBytes));
    });
  }}],
});
let browser;
try {
  await server.listen();
  browser=await chromium.launch();
  const page=await browser.newPage({viewport:{width:740,height:660}});
  page.setDefaultTimeout(8500);
  const errors=[];
  page.on("pageerror",err=>errors.push(err.message));
  await page.goto(server.resolvedUrls.local[0],{waitUntil:"domcontentloaded"});
  const open=async label=>page.getByText(label,{exact:true}).click({timeout:12000});
  const output=page.getByRole("img",{name:fixture.channels.output.alt,exact:true});
  const input=page.getByRole("img",{name:fixture.channels.input.alt,exact:true});
  const debug=async()=>page.evaluate(()=>({text:document.body.innerText,proof:window.channelImageProof}));
  assert.equal(await output.count(),0);
  console.log("[DEBUG] opening output");
  await open("source@image");
  await open("Output");
  console.log("[DEBUG] opened output");
  await output.waitFor({state:"attached",timeout:12000}).catch(async error=>{throw new Error(JSON.stringify({errors,state:await debug()}),{cause:error});});
  await page.waitForFunction(name=>Array.from(document.images).some(img=>img.alt===name&&img.naturalWidth===256),fixture.channels.output.alt);
  console.log("[DEBUG] large output decoded");
  assert.equal(await output.evaluate(img=>img.closest("[data-tree-window-length]").getAttribute("data-tree-window-length")),"3");
  const actual=await output.evaluate(async img=>{
    const bytes=await fetch(img.currentSrc).then(response=>response.arrayBuffer());
    return {src:img.currentSrc,digest:Array.from(new Uint8Array(await crypto.subtle.digest("SHA-256",bytes)),byte=>byte.toString(16).padStart(2,"0")).join("")};
  });
  assert.match(actual.src,/^blob:/);
  assert.equal(actual.digest,createHash("sha256").update(Buffer.from(data.split(",")[1],"base64")).digest("hex"));
  await open("dataUrl");
  await page.getByText("Raw data URL",{exact:true}).waitFor({state:"visible"});
  const expectHover=async(image,target)=>{
    await page.mouse.move(720,655);
    await page.evaluate(()=>window.channelImageProof.intents.length=0);
    await image.hover();
    await page.waitForFunction(id=>window.channelImageProof.intents.some(action=>action.action==="interactionHover"&&action.args.targetId===id&&action.args.granularityId==="handle"),target);
  };
  console.log("[DEBUG] testing picture hover");
  await expectHover(output,fixture.channels.output.id);
  await open("Output");
  await output.waitFor({state:"detached"});
  assert.equal(await page.evaluate(async url=>{try{await fetch(url);return false;}catch{return true;}},actual.src),true);
  await open("sink@image");
  await open("Input");
  await page.waitForFunction(name=>Array.from(document.images).some(img=>img.alt===name&&img.naturalWidth===2),fixture.channels.input.alt);
  console.log("[DEBUG] testing input picture hover");
  await expectHover(input,fixture.channels.input.id);
  assert.deepEqual(errors,[]);
  console.log("[DEBUG] Chromium Values tree rendered exact short input and paged output images, retained metadata, and emitted both channel hover addresses");
} catch (error) {
  const pages = browser?.contexts().flatMap(context=>context.pages()) ?? [];
  for (const page of pages) console.log("[DEBUG]",await page.evaluate(()=>({text:document.body.innerText,intents:window.channelImageProof?.intents,requests:window.channelImageProof?.requests(),revisions:window.channelImageProof?.revisions})));
  throw error;
} finally {await browser?.close();await server.close();}
