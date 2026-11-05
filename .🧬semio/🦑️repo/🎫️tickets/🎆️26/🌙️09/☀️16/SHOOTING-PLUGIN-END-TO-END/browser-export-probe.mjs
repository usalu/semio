/** 📸️ Proves the served shooting host emits PNG/SVG downloads with an independent decoder. */
import {loadPlaywrightV1} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {browserLaunchOptionsV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";
import sharp from "sharp";
import assert from "node:assert/strict";
import {mkdirSync,writeFileSync,readFileSync} from "node:fs";
import {join} from "node:path";
const output=join(import.meta.dir,"🗑️generated","browser-export");
mkdirSync(output,{recursive:true});
const lines=[],downloads=[];
const {chromium}=await loadPlaywrightV1(findWorkspaceRoot());
const browser=await chromium.launch({...browserLaunchOptionsV1(),downloadsPath:output});
const page=await browser.newPage({viewport:{width:1600,height:1000},acceptDownloads:true,locale:"en-US"});
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
try{
 await page.goto(process.env.SEMIO_PROBE_URL??"http://127.0.0.1:6019/?plugin=shooting",{waitUntil:"domcontentloaded"});
 await page.waitForFunction(()=>document.documentElement.getAttribute("data-semio-os-ready")==="shooting",{},{timeout:180000});
 await page.waitForFunction(()=>[...document.querySelectorAll("[data-surface-id]")].some(host=>host.querySelector("canvas")),{},{timeout:90000});
 const artifact=page.locator('[id="framework.panel.artifact"]').first(),catalogue=page.locator('[id="framework.panel.catalogue"]').first();
 const shotRows=()=>page.getByRole("treeitem").filter({hasText:/Overview (?:Png|Svg)|Shot \d+/});
 if(await artifact.getAttribute("aria-pressed")!=="true")await artifact.click({force:true});
 await page.waitForTimeout(1000);
 const initialShots=await shotRows().count();
 assert.equal(initialShots,2,"base example has two shots");
 const initialSnapshot=await snapshot("initial");
 await catalogue.click({force:true});
 await page.getByRole("treeitem").filter({hasText:"PNG Rectangle"}).first().click({force:true});
 await artifact.click({force:true});
 await page.waitForFunction(()=>[...document.querySelectorAll('[role="treeitem"]')].filter(row=>/Overview (?:Png|Svg)|Shot \d+/.test(row.textContent??"")).length===3,{},{timeout:15000});
 const createdSnapshot=await snapshot("created");
 assert.equal(createdSnapshot.shots.length,3,"current document store contains new shot");
 assert.ok(Array.isArray(initialSnapshot.savedCameras)&&Array.isArray(createdSnapshot.savedCameras),"document exposes persisted saved-camera rows");
 assert.equal(createdSnapshot.savedCameras.length,initialSnapshot.savedCameras.length+1,"Add Shot persists one captured camera");
 const initialShotIds=new Set(initialSnapshot.shots.map(shot=>shot.id));
 const capturedShot=createdSnapshot.shots.find(shot=>!initialShotIds.has(shot.id));
 assert.ok(capturedShot,"new shot is identifiable in the actual document");
 assert.ok(typeof capturedShot.cameraId==="string"&&capturedShot.cameraId.length>0,"new shot references its captured camera");
 assert.ok(createdSnapshot.savedCameras.some(camera=>camera.id===capturedShot.cameraId),"referenced camera is persisted");
 assert.ok(!initialSnapshot.savedCameras.some(camera=>camera.id===capturedShot.cameraId),"captured camera is new");

 let undoPresses=0;
 await artifact.focus();
 for(;undoPresses<4;undoPresses++){
  await page.keyboard.press(process.platform==="darwin"?"Meta+z":"Control+z");
  await page.waitForTimeout(1000);
  if(await artifact.getAttribute("aria-pressed")!=="true")await artifact.click({force:true});
  await page.waitForTimeout(500);
  if(await shotRows().count()===initialShots)break;
 }
 assert.equal(await shotRows().count(),initialShots,"undo removes persisted new shot");
 const undoneSnapshot=await snapshot("undone");
 assert.equal(undoneSnapshot.shots.length,2,"current document store undo readback has two shots");
 assert.deepEqual(undoneSnapshot,initialSnapshot,"undo restores the complete document including saved cameras and active ids");
 downloads.length=0;
 const mutation={capturedCameraId:capturedShot.cameraId,savedCamerasBefore:initialSnapshot.savedCameras.length,savedCamerasAfterCreate:createdSnapshot.savedCameras.length,documentAfterCreate:createdSnapshot.shots.length,documentAfterUndo:undoneSnapshot.shots.length,initialShots,afterCreate:3,afterUndo:await shotRows().count(),undoPresses:undoPresses+1};
 await invoke("Export All Shots");
 for(let attempt=0;attempt<90&&downloads.length<2;attempt++)await page.waitForTimeout(1000);
 assert.ok(downloads.length>=2,"both example shots download");
 const results=[];
 for(const download of downloads){
  assert.equal(await download.failure(),null);
  const name=download.suggestedFilename(),path=join(output,name);
  await download.saveAs(path);
  const bytes=readFileSync(path),metadata=await sharp(bytes).metadata();
  assert.equal(metadata.width,256,"base example export width");
  assert.equal(metadata.height,256,"base example export height");
  const pixels=await sharp(bytes).ensureAlpha().raw().toBuffer();
  let visible=0,foreground=0;const colors=new Set();
  for(let index=3;index<pixels.length;index+=4)if(pixels[index]>0){visible++;if(pixels[index]===255){const rgb=pixels.subarray(index-3,index);colors.add(rgb.toString("hex"));if([...rgb].every(value=>value<240)&&[...rgb].some(value=>value>20))foreground++;}}
  assert.ok(visible>0,"rendered pixels are visible");
  assert.ok(colors.size>1,"opaque foreground RGB differs from a blank background");
  assert.ok(foreground>256,"expected nonwhite mesh occupies visible foreground pixels");
  if(metadata.format==="svg")assert.match(bytes.toString("utf8"),/<path\b|<polygon\b/,"SVG contains rendered geometry");
  results.push({name,format:metadata.format,width:metadata.width,height:metadata.height,bytes:bytes.length,visible,foreground,opaqueColors:colors.size});
 }
 assert.ok(results.some(row=>row.format==="png"));
 assert.ok(results.some(row=>row.format==="svg"));
 for(let attempt=0;attempt<50&&!notices.some(notice=>notice.code==="icon-render-export.done");attempt++){await page.waitForTimeout(100);await flushNotices();}
 assert.deepEqual(failedNotices(),[],"no visible image-export failure notice");
 assert.ok(notices.some(notice=>notice.code==="icon-render-export.done"),"visible terminal image-export success notice observed");


 const viewer=page.locator('[id="playground.navbar.roles.viewer"]').first();
 await viewer.click({force:true});
 await page.waitForFunction(()=>[...document.querySelectorAll("[data-surface-id]")].some(host=>(host.getAttribute("data-surface-id")??"").includes("shooting-view-scene")&&host.querySelector("canvas")&&JSON.parse(host.getAttribute("data-meshes-json")??"[]").length>0),{},{timeout:90000});
 const viewerState=await page.locator('[data-surface-id*="shooting-view-scene"]').first().evaluate(host=>({surfaceId:host.getAttribute("data-surface-id"),meshes:JSON.parse(host.getAttribute("data-meshes-json")??"[]").length,canvases:host.querySelectorAll("canvas").length}));
 assert.ok(viewerState.meshes>0&&viewerState.canvases>0,"viewer receives a nonempty scene and renders its canvas");
 const faults=await page.locator("[data-status-json]").evaluateAll(hosts=>hosts.flatMap(host=>{const status=JSON.parse(host.getAttribute("data-status-json")??"null");return status?.fault?[status.fault]:[]}));
 assert.deepEqual(faults,[],"all viewer surfaces are fault-free");
 await flushNotices();
 assert.deepEqual(failedNotices(),[],"no late visible image-export failure notice");
 assert.equal(lines.filter(line=>/pageerror|icon render export failed|image export failed|trapped|panicked|shell fault/i.test(line)).length,0);
 writeFileSync(join(output,"report.json"),JSON.stringify({images:results,mutation,viewer:viewerState,notices},null,2));
 console.log("[DEBUG] shooting exports",JSON.stringify(results));
}finally{
 try{
  writeFileSync(join(output,"console.txt"),lines.join("\n"));
  writeFileSync(join(output,"notices.json"),JSON.stringify(notices,null,2));
  await page.content().then(content=>writeFileSync(join(output,"dom.html"),content)).catch(()=>{});
  await page.screenshot({path:join(output,"page.png")}).catch(()=>{});
 }finally{
  await browser.close();
 }
}
