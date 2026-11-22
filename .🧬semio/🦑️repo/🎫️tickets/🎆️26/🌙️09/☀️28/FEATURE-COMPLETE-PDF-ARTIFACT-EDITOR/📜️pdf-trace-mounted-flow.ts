/** 🔬 Traces the mounted PDF flow at its existing Browser dispatch and queue boundaries. */
import type { Page } from "playwright";

const probes: Readonly<Record<string,readonly [string,string,string][]>> = {
  "🚚️handle-turn-drive/🟦️.ts": [
    ["importSignal?.throwIfAborted();\n        const results","turn-enter","actorId,instanceId,events.length"],
    ["const pages = createShardCommandIngressPages","page-build","actorId,commandIndex,events[commandIndex].byteLength"],
    ["const crossingStartedMs = performance.now()","page-built","actorId,pages.length"],
    ["const offered = await submitTurn","page-submit","actorId,pageIndex"],
    ["await acceptTurn(offered)","page-return","actorId,pageIndex,offered.commandIngress?.tag"],
    ["const previousIngress = terminal","poll-return","actorId,continuations,continued.status.tag??continued.status,continued.commandIngress?.tag,continued.effects.map(effect=>[effect.tag,Object.keys(effect.val??{}),effect.val?.to?.tag??effect.val?.target?.tag,Array.from((effect.val?.bytes??effect.val?.message??[]).slice(0,16)),(effect.val?.bytes??effect.val?.message??[]).length]),continued.uiPatches.length,continued.deferredEvents??0"],
  ],
  "🧵️turn-dispatch/🟦️.ts": [
    ['runTurn: async (actorId, payload, budget) => {',"scheduler-enter","actorId,payload.kind"],
    ["const raw = await (payload.activation","worker-wait","actorId,payload.kind"],
    ["const result = coerceTurnResult(raw)","worker-return","actorId"],
  ],
  "📂️picked-file-opening/🟦️.ts": [
    ["const owner = await port.resolveOwner(route)","owner-lookup","picked.name"],
    ["const instanceId = await port.importInto","import-start","picked.name"],
    ["created = {","import-return","picked.name,instanceId"],
  ],
};
export async function traceMountedPdfFlow(page: Page): Promise<void> {
  const matches = (url:URL) => Object.keys(probes).some(path => decodeURIComponent(url.pathname).endsWith(path));
  await page.route(matches, async route => {
    const path = decodeURIComponent(new URL(route.request().url()).pathname);
    const plan = Object.entries(probes).find(([suffix]) => path.endsWith(suffix))?.[1];
    const response = await route.fetch();
    if (plan === undefined || !response.ok()) return route.fulfill({response});
    let body = await response.text();
    const marks: string[] = [];
    for (const [anchor,label,args] of plan) {
      const start = body.indexOf(anchor);
      if (start < 0) continue;
      const after = anchor.endsWith("{") && label === "scheduler-enter" || anchor.endsWith(";");
      const offset = after ? start + anchor.length : start;
      body = body.slice(0,offset) + "\n__pdfMountedTrace(" + JSON.stringify(label) + ", " + args + ");\n" + body.slice(offset);
      marks.push(label);
    }
    const prefix = 'const __pdfMountedTrace = (label, ...data) => { const seen = globalThis.__pdfMountedProbes ??= new Map(); const key = label+":"+data[0]; const n = seen.get(key) ?? 0; seen.set(key,n+1); if(n<10||label==="poll-return"&&n%256===0){console.warn("[DEBUG] pdf-flow "+JSON.stringify([Math.round(performance.now()),label,...data]));} };\n';
    const headers = {...response.headers()};
    delete headers["content-encoding"];
    delete headers["content-length"];
    console.log("[DEBUG] PDF probes",path.split("/").slice(-2).join("/"),marks.join(","));
    await route.fulfill({response,headers,body:prefix+body});
  });
}
