/** 🛑️ Observes real host image-export progress and cancellation through accessible UI. */
import {loadPlaywrightV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {browserLaunchOptionsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {composeSpecificOsCatalogV1} from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/🟦️.ts";
import {PLAYGROUND_BUILD_TARGETS} from "../../../../../../../✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/📚️catalog/🎮️playgrounds/🟦️.ts";
import {mkdirSync,writeFileSync,readFileSync} from "node:fs";
import {join} from "node:path";
const output=join(import.meta.dir,"🗑️generated","browser-cancel");
mkdirSync(output,{recursive:true});
const lines=[],downloads=[];
const {chromium}=await loadPlaywrightV1(findWorkspaceRoot());
const browser=await chromium.launch({...browserLaunchOptionsV1(),downloadsPath:output});
const context=await browser.newContext({viewport:{width:1600,height:1000},acceptDownloads:true,locale:"en-US"});
const page=await context.newPage();
page.on("console",message=>lines.push(message.type()+" "+message.text()));
page.on("pageerror",error=>lines.push("pageerror "+error.message));
page.on("download",download=>downloads.push(download));
const notices=[],noticeKeys=new Set();
await page.exposeFunction("__semioShootingProbeRecordNotice",notice=>{
 const key=JSON.stringify(notice);
 if(!noticeKeys.has(key)){noticeKeys.add(key);notices.push(notice);}
});
await page.addInitScript(()=>{
 const selector="[data-semio-transient-notice]",seen=new Set(),pending=new Set();
 globalThis.__semioShootingProbeFlushNotices=()=>Promise.all([...pending]);
 const record=element=>{
  const notice={code:element.getAttribute("data-notice-code"),text:(element.innerText??element.textContent??"").trim()},key=JSON.stringify(notice);
  if(seen.has(key))return;
  seen.add(key);
  const delivery=globalThis.__semioShootingProbeRecordNotice(notice);
  pending.add(delivery);
  delivery.then(()=>pending.delete(delivery),()=>pending.delete(delivery));
 };
 const capture=(node,descendants)=>{
  const element=node instanceof Element?node:node.parentElement;
  if(!element)return;
  const notice=element.closest(selector);
  if(notice)record(notice);
  if(descendants)for(const nested of element.querySelectorAll(selector))record(nested);
 };
 new MutationObserver(records=>{
  for(const mutation of records){
   capture(mutation.target,false);
   for(const added of mutation.addedNodes)capture(added,true);
  }
 }).observe(document,{childList:true,subtree:true,characterData:true,attributes:true,attributeFilter:["data-notice-code"]});
});
const flushNotices=()=>page.evaluate(()=>globalThis.__semioShootingProbeFlushNotices());
const failedNotices=()=>notices.filter(notice=>notice.code==="icon-render-export.failed"||/image export failed/i.test(notice.text));
const invoke=async(label)=>{
 const buttons=page.getByRole("button",{name:label,exact:true});
 if(await buttons.count())await buttons.first().click();
 else{
  await page.keyboard.press(process.platform==="darwin"?"Meta+k":"Control+k");
  await page.getByRole("combobox").last().fill(label);
  await page.getByText(label,{exact:true}).last().click();
 }
};
const snapshot=async(name)=>{
 const pending=page.waitForEvent("download",{predicate:download=>download.suggestedFilename()==="shooting.shooting.ops",timeout:30000});
 await invoke("Save Download");
 const download=await pending,path=join(output,name+".json");
 await download.saveAs(path);
 return JSON.parse(readFileSync(path,"utf8"));
};

const total=50,settleMs=5000,requests=[],bootResponses=[],bootResponseWork=[];
try{
const workspace=findWorkspaceRoot(),targetUrl=new URL(process.env.SEMIO_PROBE_URL??"http://127.0.0.1:6019/?plugin=shooting");
assert.ok(["127.0.0.1","localhost","[::1]"].includes(targetUrl.hostname),"controlled local declared host");
assert.equal(targetUrl.pathname,"/");assert.equal(targetUrl.hash,"");
assert.deepEqual([...targetUrl.searchParams],[["plugin","shooting"]],"no space/capture/attachment route");
const variantRoot="✏️s/🧑‍💻dev/🎭️variants/🎥️shooting",entryPath=variantRoot+"/🚀️entry/🟦️.ts";
const selected=PLAYGROUND_BUILD_TARGETS.find(row=>row.variant==="shooting");
assert.equal(selected?.pluginId,"shooting");assert.equal(selected?.app,"s.shooting.shooting@1/*#editor");
assert.equal(selected?.devContribution,variantRoot+"/🧬️schema/🔣️.json","actual published catalogue must include the authored repair");
const composed=composeSpecificOsCatalogV1(targetUrl.href,{maxBytes:2097152,maxRows:512,maxEdges:4096,maxWork:65536,deadlineMs:performance.now()+30000,now:()=>performance.now(),cancelled:()=>false,progress:()=>{}});
assert.ok(composed.rows.targets.some(row=>row.pluginId==="shooting"&&row.role==="plugin"),"actual composed shooting guest target");
assert.ok(!composed.rows.hosts.some(row=>row.pluginId==="shooting"),"catalogue-derived shooting host mode is disabled");
const watched=[
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔒️identity-document-ports/📎️hub-port-and-document-backbone/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧩️shell-core-state/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/✅️check-in-sync/🔶️host-mode/🟦️.tsx",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔶️scope/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🛫️spawned-session/🟠️switch-plugin-app/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📬️artifact-opening/🔸️open-artifact-app/🟦️.ts",
 entryPath,variantRoot+"/🧬️schema/🔣️.json",variantRoot+"/🏗️builder/🌐️vite/🟦️.ts",
 "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts",
 "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts","✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document/🟦️.ts",
 "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts"
];
const sourceHashes=()=>Object.fromEntries(watched.map(path=>[path,createHash("sha256").update(readFileSync(join(workspace,path))).digest("hex")]));
const initialSourceHashes=sourceHashes(),emptyContext=await context.storageState();
assert.equal(emptyContext.cookies.length,0);assert.equal(emptyContext.origins.length,0,"new nonpersistent profile");
page.on("response",response=>{
 const path=decodeURIComponent(new URL(response.url()).pathname);
 if(!path.includes(entryPath))return;
 const work=response.body().then(bytes=>bootResponses.push({url:response.url(),status:response.status(),sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length}));
 bootResponseWork.push(work);
});
page.on("request",request=>{if(!["GET","HEAD","OPTIONS"].includes(request.method()))requests.push({method:request.method(),url:request.url()});});
 await page.goto(targetUrl.href,{waitUntil:"domcontentloaded"});
 await page.waitForFunction(()=>document.documentElement.getAttribute("data-semio-os-ready")==="shooting",{},{timeout:180000});
 await page.waitForFunction(()=>[...document.querySelectorAll("[data-surface-id]")].some(host=>host.querySelector("canvas")),{},{timeout:90000});
 const artifact=page.locator('[id="framework.panel.artifact"]').first(),catalogue=page.locator('[id="framework.panel.catalogue"]').first();
 await Promise.all(bootResponseWork);
 assert.ok(bootResponses.some(row=>row.status===200&&row.bytes>0),"actual repaired standalone entry was served to the new guest");
 assert.equal(page.url(),targetUrl.href,"boot did not select a shared/captured route");
 const attachmentBadge=page.locator("[data-hub-connection-documents]");
 await attachmentBadge.first().waitFor({state:"visible",timeout:30000});
 const attachmentCounts=await attachmentBadge.evaluateAll(elements=>elements.map(element=>element.getAttribute("data-hub-connection-documents")));
 assert.ok(attachmentCounts.length>0&&attachmentCounts.every(count=>count==="0"),"public zero documents corroborates the controlled no-auto-attachment construction");
 assert.deepEqual(sourceHashes(),initialSourceHashes,"proof source/catalogue inputs stayed unchanged before any mutation");
 const isolation={entryDelivery:"observed authored-entry URL delivery and transformed response hash; not raw source-content equality",selected,hosts:composed.rows.hosts,url:page.url(),newContext:{cookies:0,origins:0},ready:"shooting",attachmentCounts,bootResponses,sourceHashes:initialSourceHashes,construction:"standalone guest; audited writer trace; no space/capture route or open/import/Attach/Keep/sign-in action"};
 writeFileSync(join(output,"isolation.json"),JSON.stringify(isolation,null,2));
 const before=await snapshot("initial");
 assert.equal(before.shots.length,2,"fresh example document required");
 for(let index=2;index<total;index++){
  if(await catalogue.getAttribute("aria-pressed")!=="true")await catalogue.click({force:true});
  await page.getByRole("treeitem").filter({hasText:"PNG Rectangle"}).first().click({force:true});
  await artifact.click({force:true});
  await page.waitForFunction(expected=>[...document.querySelectorAll('[role="treeitem"]')].filter(row=>/Overview (?:Png|Svg)|Shot \d+/.test(row.textContent??"")).length===expected,index+1,{timeout:15000});
 }
 const prepared=await snapshot("prepared");
 assert.equal(prepared.shots.length,total,"supported catalogue actions persisted every shot");
 assert.equal(prepared.savedCameras.length,before.savedCameras.length+total-2,"every created shot retains its camera");
 await page.keyboard.press(process.platform==="darwin"?"Meta+k":"Control+k");
 await page.getByRole("combobox").last().fill("Task Manager");
 await page.getByText(/^(Open )?Task Manager$/).last().click();
 await page.locator("[data-semio-task-manager-window]").waitFor({state:"visible"});
 downloads.length=0;
 await invoke("Export All Shots");
 const row=page.locator('[data-semio-task-manager-task^="export:icon#"]').first();
 await row.waitFor({state:"visible",timeout:30000});
 const progress=row.getByRole("progressbar");
 await page.waitForFunction(()=>[...document.querySelectorAll('[data-semio-task-manager-task^="export:icon#"] [role="progressbar"]')].some(element=>Number(element.getAttribute("data-semio-task-manager-progress"))>0),{},{timeout:60000});
 const observed={id:await row.getAttribute("data-semio-task-manager-task"),value:await progress.getAttribute("data-semio-task-manager-progress"),text:await progress.getAttribute("aria-valuetext"),percent:await progress.getAttribute("aria-valuenow")};
 assert.ok(Number(observed.value)>0&&Number(observed.value)<total,"visible intermediate progress");
 assert.match(observed.text,/Image \d+ of 50/);
 await row.screenshot({path:join(output,"progress.png")});
 const cancel=row.getByRole("button",{name:/^Cancel /});
 assert.ok(await cancel.isEnabled(),"visible cancel control is enabled");
 await cancel.click();
 await row.waitFor({state:"detached",timeout:30000});
 for(let attempt=0;attempt<100&&!notices.some(notice=>notice.code==="icon-render-export.cancelled");attempt++){await page.waitForTimeout(100);await flushNotices();}
 assert.ok(notices.some(notice=>notice.code==="icon-render-export.cancelled"),"visible terminal cancelled notice");
 assert.ok(!notices.some(notice=>["icon-render-export.done","icon-render-export.failed"].includes(notice.code)),"batch neither completed nor failed");
 const delivered=downloads.length;
 assert.ok(delivered>0&&delivered<total,"cancelled batch delivered a strict prefix");
 await page.waitForTimeout(settleMs);
 await flushNotices();
 assert.equal(downloads.length,delivered,"no late deliveries after cancellation settled");
 assert.equal(await page.locator('[data-semio-task-manager-task^="export:icon#"]').count(),0,"cancelled row stays retired");
 assert.ok(!notices.some(notice=>["icon-render-export.done","icon-render-export.failed"].includes(notice.code)),"no late contradictory terminal notice");
 assert.deepEqual(await snapshot("after"),prepared,"export cancellation preserves the complete document");
 assert.deepEqual(sourceHashes(),initialSourceHashes,"isolation proof inputs remain unchanged through the actual scenario");
 assert.equal(lines.filter(line=>/pageerror|image export failed|trapped|panicked|shell fault/i.test(line)).length,0);
 writeFileSync(join(output,"report.json"),JSON.stringify({total,delivered,settleMs,progress:observed,notices,mutatingRequests:requests,isolation},null,2));
 console.log("[DEBUG] real UI export cancellation",JSON.stringify({total,delivered,progress:observed}));
}finally{
 try{
  writeFileSync(join(output,"console.txt"),lines.join("\n"));
  writeFileSync(join(output,"notices.json"),JSON.stringify(notices,null,2));
  writeFileSync(join(output,"requests.json"),JSON.stringify(requests,null,2));
  await page.content().then(content=>writeFileSync(join(output,"dom.html"),content)).catch(()=>{});
  await page.screenshot({path:join(output,"page.png")}).catch(()=>{});
 }finally{await browser.close();}
}
