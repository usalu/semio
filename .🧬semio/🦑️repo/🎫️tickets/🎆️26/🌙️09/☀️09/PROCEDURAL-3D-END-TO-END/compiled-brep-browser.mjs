import { chromium } from "playwright";
import { resolve } from "node:path";
const root = resolve(".");
const loader = "/@fs/" + root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧩️module-loader/🟦️.ts";
const packs = "/@fs/" + root + "/🧰️framework/🔨️modules/🎁️packs/🟦️.ts";
const url = "http://127.0.0.1:6018";
const browser = await chromium.launch({headless:true});
const timeout = setTimeout(() => { console.error("[DEBUG] BRep capability deadline"); void browser.close(); }, 360000);
try {
  const page = await browser.newPage();
  page.context().on("requestfailed", r => console.error("[DEBUG] request failed",r.url(),r.failure()?.errorText));
  page.context().on("response", r => { if(r.status()>=400) console.error("[DEBUG] HTTP",r.status(),r.url()); });
  page.on("pageerror", error => console.error("[DEBUG]", error.message));
  page.on("console", text => { if (text.text().startsWith("[DEBUG]")) console.log(text.text()); });
  await page.route(url + "/geometry-capability", route => route.fulfill({contentType:"text/html",body:"<!doctype html><title>BRep capability verification</title>"}));
  await page.goto(url + "/geometry-capability");
  const result = await page.evaluate(async ({loader,packs}) => {
    window.$RefreshReg$ = () => {};
    window.$RefreshSig$ = () => value => value;
    window.__vite_plugin_react_preamble_installed__ = true;
    const [{loadPluginModule},{decodePackValue}] = await Promise.all([import(loader),import(packs)]);
    console.log("[DEBUG] Runtime ready");
    const ext = await loadPluginModule("flow-extension-brep","/🧩️extension-modules/🧊️flow-extension-brep/🌉️bridge.js");
    const number = value => ({$schema:"number",value});
    const point = (x,y,z) => ({$schema:"point",x,y,z});
    async function evaluate(operatorId, nodeHash, input) {
      const started = performance.now();
      for (let round=0; round<256; round++) {
        const body = {neuronId:operatorId,operatorId,nodeHash,operatorVersion:"fixture",inputJson:round ? "" : JSON.stringify(input),dependencyJson:"",roundUnits:256,resume:round>0,windowId:"preview",windowKindId:"preview",extensionId:"brep"};
        const bytes = await ext.invoke("evaluate",JSON.stringify(body));
        const reply = decodePackValue(bytes);
        console.log("[DEBUG]",operatorId,round,reply.phase,String(reply.unitsDone),String(reply.unitsTotal),Boolean(reply.done),Math.round(performance.now()-started),bytes.length);
        if (reply.done) return reply.output;
      }
      throw Error("Extension continuation exceeded 256 replies for "+operatorId);
    }
    try {
      const profile = await evaluate("brep.curve.circle",771,{center:point(0,0,0),normal:point(0,0,1),radius:number(2)});
      if (!profile.curve?.handle) throw Error("Missing curve handle "+JSON.stringify(profile));
      const extrusion = await evaluate("brep.solid.extrude",772,{wire:profile.curve,vector:{$schema:"vector",x:0,y:0,z:3}});
      if (!extrusion.solid?.handle) throw Error("Missing solid handle "+JSON.stringify(extrusion));
      return {curve:profile.curve.handle,solid:extrusion.solid.handle};
    } finally { await ext.dispose?.(); }
  },{loader,packs});
  console.log("[DEBUG] Actual browser BRep solid",JSON.stringify(result));
} finally {
  clearTimeout(timeout);
  await browser.close();
}
