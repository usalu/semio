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
 const boot = await page.waitForFunction(() => {
   const html = document.documentElement;
   const ready = html.getAttribute("data-semio-os-ready");
   const fault = html.getAttribute("data-semio-os-error");
   return ready !== null || fault !== null ? { ready, fault } : null;
 }, undefined, {timeout: 180_000});
 const {ready, fault} = await boot.jsonValue();
 assert.equal(fault, null, `Shooting boot reported ${fault}`);
 assert.equal(ready, "shooting", "Shooting reaches the ready beacon");
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


 const inspector = page.locator('[id="framework.panel.inspection"]').first();
 if (await inspector.getAttribute("aria-pressed") !== "true") await inspector.click();
 const selectSetting = async (label, option) => {
   const control = page.getByRole("combobox", {name: label, exact: true});
   await control.click();
   await page.getByRole("option", {name: option, exact: true}).click();
 };
 const editSetting = async (label, value) => {
   const control = page.getByLabel(label, {exact: true});
   await control.fill(value);
   await control.press("Enter");
 };
 await selectSetting("Projection", "Orthographic");
 await selectSetting("Framing", "Fit model");
 await selectSetting("Hidden lines", "On");
 await selectSetting("Merge connected lines", "On");
 await editSetting("Fill color (empty for material)", "#6040c0");
 await editSetting("Fill opacity", "0.6");
 await editSetting("Visible line width (px)", "2.5");
 await editSetting("Hidden line width (px)", "1.5");
 await editSetting("Corner radius (px)", "2");
 const styled = await snapshot("drawing-settings");
 const shot = styled.shots.find(item => item.id === styled.activeShotId);
 assert.equal(shot.id, initialSnapshot.activeShotId, "styled shot is the original fixture shot");
 assert.equal(initialSnapshot.shots.find(item => item.id === shot.id)?.cameraId, undefined, "fixture begins with an uncaptured shot");
 assert.ok(shot.cameraId, "first projection edit captured a saved camera");
 assert.deepEqual(styled.shots.filter(item => item.id !== shot.id), initialSnapshot.shots.filter(item => item.id !== shot.id));
 assert.equal(styled.savedCameras.find(camera => camera.id === shot.cameraId)?.camera.projection, "orthographic");
 assert.equal(shot.drawing.hiddenLines, true);
 assert.equal(shot.autoFit, true);
 assert.equal(shot.drawing.mergeLines, true);
 assert.equal(shot.drawing.fillColor, "#6040c0");
 assert.equal(shot.drawing.fillOpacity, 0.6);
 assert.equal(shot.drawing.visibleWidth, 2.5);
 assert.equal(shot.drawing.hiddenWidth, 1.5);
 assert.equal(shot.drawing.lineEasing, 2);
 const pendingSvg = page.waitForEvent("download", {predicate: item => item.suggestedFilename()===`${shot.id}.svg`});
 await invoke("Export Active Shot");
 const drawingFile = join(output, "styled.svg");
 await (await pendingSvg).saveAs(drawingFile);
 const exportedSvg = readFileSync(drawingFile, "utf8");
 const layers = await page.evaluate(source => {
   const image = new DOMParser().parseFromString(source, "image/svg+xml");
   return Object.fromEntries([...image.querySelectorAll("g[id]")].map(group => [group.id, group.getAttribute("stroke-width")]));
 }, exportedSvg);
 assert.equal(layers["visible-lines"], "2.5");
 assert.equal(layers["hidden-lines"], "1.5");
 await page.waitForFunction(source => {
   const signature = markup => {
     const image = new DOMParser().parseFromString(markup, "image/svg+xml");
     return JSON.stringify([...image.querySelectorAll("g[id]")].map(group => ({
       id: group.id,
       opacity: group.getAttribute("opacity"),
       stroke: group.getAttribute("stroke"),
       width: group.getAttribute("stroke-width"),
       paths: [...group.querySelectorAll("path")].map(path => [path.getAttribute("d"), path.getAttribute("style")]),
     })));
   };
   const expected = signature(source);
   if (!expected.includes('"opacity":"0.6"') || !expected.includes('"id":"visible-lines"')) return false;
   return [...document.querySelectorAll(".semio-icon-render-host img")].some(img => {
     if (!img.complete || img.naturalWidth === 0 || !img.src.startsWith("data:image/svg+xml")) return false;
     const separator = img.src.indexOf(",");
     const data = img.src.slice(separator + 1);
     const svg = img.src.slice(0, separator).endsWith(";base64") ? atob(data) : decodeURIComponent(data);
     return signature(svg) === expected;
   });
 }, exportedSvg, {timeout: 30000});
 assert.equal((await sharp(drawingFile).metadata()).width, shot.width);

 await editSetting("Zoom", "1.75");
 const manual = await snapshot("manual-framing");
 const manualShot = manual.shots.find(item => item.id === shot.id);
 assert.equal(manualShot.autoFit, false, "editing zoom uses manual framing for that shot");
 assert.equal(manual.savedCameras.find(camera => camera.id === manualShot.cameraId)?.camera.zoom, 1.75);
 assert.deepEqual(manual.shots.filter(item => item.id !== shot.id), initialSnapshot.shots.filter(item => item.id !== shot.id));
 await selectSetting("Framing", "Fit model");

 await selectSetting("Face fills", "Off");
 await selectSetting("Visible lines", "Off");
 await selectSetting("Background layer", "Off");
 const isolated = await snapshot("hidden-lines-only");
 const isolatedShot = isolated.shots.find(item => item.id === shot.id);
 assert.equal(isolatedShot.drawing.fill, false);
 assert.equal(isolatedShot.drawing.background, false);
 assert.equal(isolatedShot.drawing.visibleLines, false);
 assert.equal(isolatedShot.drawing.hiddenLines, true);
 assert.equal(isolatedShot.autoFit, true, "fit can be restored without changing the saved camera");
 const pendingHidden = page.waitForEvent("download", {predicate: item => item.suggestedFilename()===`${shot.id}.svg`});
 await invoke("Export Active Shot");
 const hiddenFile = join(output, "hidden.svg");
 await (await pendingHidden).saveAs(hiddenFile);
 const hiddenSvg = readFileSync(hiddenFile, "utf8");
 assert.match(hiddenSvg, /id="hidden-lines"/);
 assert.doesNotMatch(hiddenSvg, /id="(?:visible-lines|fills|background)"/);
 const hiddenPixels = await sharp(hiddenFile).ensureAlpha().raw().toBuffer();
 const hiddenAlpha = hiddenPixels.filter((_, index) => index % 4 === 3);
 assert.ok(hiddenAlpha.some(value => value === 0) && hiddenAlpha.some(value => value > 0));

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
