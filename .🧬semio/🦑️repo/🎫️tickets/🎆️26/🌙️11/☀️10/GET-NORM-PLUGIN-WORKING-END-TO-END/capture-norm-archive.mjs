import {createRequire} from "node:module";
import {writeFile} from "node:fs/promises";
import {join,basename} from "node:path";
import {fileURLToPath} from "node:url";
const {chromium}=createRequire(process.cwd()+"/package.json")("playwright");
const folder=fileURLToPath(new URL("🗑️generated/",import.meta.url));
const id=process.argv[2];
if (!["din4108","din16798","iso16757","vdi3805","din18599"].includes(id) && !/^en199[0-9]$/.test(id)) throw Error("Expected a norm archive family");
const browser=await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader"]});
try {
 const context=await browser.newContext({locale:"en-US"});
 const page=await context.newPage();
 if (process.env.PLAY_TEST_GUEST) await context.route(url=>url.pathname.endsWith("/"+basename(process.env.PLAY_TEST_GUEST)),async route=>{
   const url=new URL(route.request().url());url.pathname="/@fs"+process.env.PLAY_TEST_GUEST;await route.continue({url:url.href});
 });
 let done;
 const saved=new Promise(resolve=>{done=resolve});
 await context.route("**/__ticket_norm_archive",async route=>{
   const archive=route.request().postDataJSON();
   for (const [key,ext] of [["parent_pack","pack"],["parent_spr","spr"]]) await writeFile(join(folder,id+"-observed."+ext),Buffer.from(archive[key]));
   console.log(JSON.stringify({id,pack:archive.parent_pack.length,spr:archive.parent_spr.length,members:archive.members.length,keys:Object.keys(archive)}));
   await route.fulfill({status:200,body:"ok"});done();
 });
 await context.route(url=>decodeURIComponent(url.pathname).includes("/🧾️app-channel/🧳️archive-transfer/"),async route=>{
   const source=await route.fetch();const text=await source.text();
   const anchor='await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);';
   const capture='await fetch("/__ticket_norm_archive",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify({parent_pack:Array.from(archive.parent_pack),parent_spr:Array.from(archive.parent_spr),members:archive.members})});';
   await route.fulfill({response:source,body:text.replace(anchor,capture+anchor)});
 });
 await page.goto(new URL("#"+id,process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6034/").href,{waitUntil:"commit"});
 await Promise.race([saved,new Promise((_,reject)=>setTimeout(()=>reject(Error("Archive capture timed out")),120000).unref())]);
} finally {await browser.close();}
