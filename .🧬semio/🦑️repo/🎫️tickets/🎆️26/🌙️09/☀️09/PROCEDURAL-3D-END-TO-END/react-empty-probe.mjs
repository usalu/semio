import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe for the empty Flow canvas on `?plugin=generation3d`: console, network, page globals, flow DOM, worker turns. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/react-empty"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "a";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 230000);
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
page.on("console", m => lines.push(`${m.type()} ${m.text().slice(0, 9000)}`));
page.on("pageerror", e => lines.push(`pageerror ${e.message}`));
page.on("requestfailed", r => lines.push(`requestfailed ${r.url()} ${r.failure()?.errorText}`));
page.on("response", r => { if (r.status() >= 400) lines.push(`http ${r.status()} ${r.url()}`); });
const t0 = Date.now();
page.context().on("request", r => { if (/core\.wasm|component\.js|shard/i.test(r.url())) lines.push(`ctxreq +${Date.now() - t0}ms ${decodeURIComponent(r.url()).slice(-110)}`); });
page.on("request", r => { if (/\.wasm|node_graph|flow_core|flow-core|extension|brep|\.json/i.test(r.url())) lines.push(`req ${r.url().slice(-120)}`); });
page.on("worker", w => lines.push(`worker ${w.url()}`));
await page.addInitScript(() => {
  window.__msgs = [];
  window.addEventListener("unhandledrejection", e => console.log("[probe] unhandledrejection " + String(e.reason?.stack ?? e.reason).slice(0, 500)));
  window.addEventListener("error", e => console.log("[probe] error " + e.message));
  window.addEventListener("message", e => { try { window.__msgs.push(typeof e.data === "string" ? e.data.slice(0, 200) : JSON.stringify(e.data).slice(0, 200)); } catch {} });
});
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(Number(process.env.PROBE_WAIT ?? 30000));
const gpu = await page.evaluate(async () => { try { const a = await navigator.gpu?.requestAdapter(); return a ? "adapter" : "no-adapter"; } catch (e) { return String(e); } });
const canv = await page.evaluate(() => [...document.querySelectorAll("canvas")].map(c => { let t = "?"; try { t = c.getContext("2d") ? "2d" : "other"; } catch {} return { w: c.width, cls: c.className, parent: c.parentElement?.className?.slice?.(0,60), t }; }));
const ng = await page.evaluate(async () => { try { const m = await import("/@fs/Users/ueli/Documents/semio/.cache/b/p/m/browser/prototype/wasm-packages/0eac234ec2c308c96333b196/framework_os_node_graph.js"); return Object.keys(m).slice(0, 40).join(","); } catch (e) { return "ERR " + e.message; } });
const rt = await page.evaluate(async () => { try { const r = await import("/@fs/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%93%BA%EF%B8%8Frenderer/%F0%9F%A7%91%E2%80%8D%F0%9F%8E%A8engine/%F0%9F%A7%B1%EF%B8%8Felements/%F0%9F%95%B8%EF%B8%8FNodeGraph/%F0%9F%93%8B%EF%B8%8Fruntime/%F0%9F%9F%A6%EF%B8%8F.ts"); const m = await import("/@fs/Users/ueli/Documents/semio/.cache/b/p/m/browser/prototype/wasm-packages/0eac234ec2c308c96333b196/framework_os_node_graph.js"); const run = r.requireNodeGraphRuntimeV1(m); await run.initialize(); const s = await run.create(); return "created " + typeof s; } catch (e) { return "ERR " + e.message.slice(0, 600); } });
const fl = await page.evaluate(async () => { try { const core = await import("/@id/__x00__playground-wasm-stub/@semio-tech__flow-core"); const br = await import("/@fs/Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%9B%8D%EF%B8%8Fproducts/%F0%9F%92%BB%EF%B8%8Fos/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%8A%EF%B8%8Fflow/%F0%9F%95%B8%EF%B8%8Fwasm/%F0%9F%8C%90%EF%B8%8Fbrowser/%F0%9F%8F%83%EF%B8%8Fruntime/%F0%9F%9F%A8%EF%B8%8F.js"); const ex = await core.default(); await br.createFlowBrowserRuntime({ source: ex, bindings: core }); return "runtime ok"; } catch (e) { return "ERR " + String(e.message).slice(0, 400); } });
console.log("NG", ng, "RT", rt, "FLOW", fl);
if (process.env.PROBE_ASK === "1") {
  for (const nodeKind of ["brep.solid.extrude", "math.vector"]) console.log("ASK", nodeKind, await page.evaluate((k) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: k })), nodeKind));
  lines.push(`ask-done +${Date.now() - t0}ms`);
  if (process.env.PROBE_TIMELINE === "1") for (let t = 0; t <= 110; t += 5) {
    console.log("T+" + t, await page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); const m = g("window:procedural-main")?.getAttribute("data-status-json") ?? ""; return `${p.phase} ${p.fault?.code ?? ""} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal} nodes=${p.progress?.nodesDone}/${p.progress?.nodesTotal} meshes=${(g("window:procedural-preview")?.getAttribute("data-meshes-json") ?? "").length} main=${m.replace(/[{}"]/g, "").slice(0, 160)}`; }));
    await page.waitForTimeout(5000);
  }
  await page.waitForTimeout(20000);
  if (process.env.PROBE_NUDGE === "1") {
    console.log("PRE", await page.evaluate(() => document.querySelector('[data-surface-id="window:procedural-preview"]')?.getAttribute("data-status-json")?.slice(0, 80)));
    if (process.env.PROBE_CLICK !== "0") await page.mouse.click(500, 400);
    await page.waitForTimeout(8000);
    console.log("POST", await page.evaluate(() => document.querySelector('[data-surface-id="window:procedural-preview"]')?.getAttribute("data-status-json")?.slice(0, 80)));
  }
  await page.waitForTimeout(20000);
}
const hops = await page.evaluate(() => { const c = window.__semioOsCatalogProbe; const inst = {}; for (const id of [1, 2, 3, 4, 5]) { try { const v = window.__semioOsInstalledContributions?.(id); { const rows = v == null ? null : JSON.parse(v); inst[id] = rows == null ? null : rows.map(r => ({ id: r.pluginId, keys: Object.keys(r.topicContribution?.payload ?? {}), ops: (() => { try { return (JSON.parse(r.topicContribution.payload.manifestJson).operators ?? JSON.parse(r.topicContribution.payload.manifestJson).kinds ?? []).length; } catch (e) { return "parse " + e.message; } })(), size: JSON.stringify(r).length })); } } catch (e) { inst[id] = "ERR " + e.message; } } return { catalog: JSON.stringify(c).slice(0, 1800), inst }; });
console.log("HOPS", JSON.stringify(hops));
const mani = await page.evaluate(() => { try { const rows = JSON.parse(window.__semioOsInstalledContributions(1)); return rows.map(r => { const m = JSON.parse(r.topicContribution.payload.manifestJson); return { id: r.pluginId, topic: r.topicContribution.topic, keys: Object.keys(m), counts: Object.fromEntries(Object.entries(m).map(([k, v]) => [k, Array.isArray(v) ? v.length : typeof v === "object" && v ? Object.keys(v).length : typeof v])), sample: JSON.stringify(m).slice(0, 400) }; }); } catch (e) { return "ERR " + e.message; } });
console.log("MANI", JSON.stringify(mani).slice(0, 200));
const cont = await page.evaluate(() => { const rows = JSON.parse(window.__semioOsInstalledContributions(1)); return rows.map(r => { const c = JSON.parse(r.topicContribution.payload.manifestJson).contributes; return Object.fromEntries(Object.entries(c).map(([k, v]) => [k, Array.isArray(v) ? v.length + ":" + JSON.stringify(v.slice(0, 2).map(x => x.id ?? x.kind ?? Object.keys(x))).slice(0, 160) : typeof v])); }); });
console.log("CONT", JSON.stringify(cont));
const snap = await page.evaluate(() => { const raw = window.__semioFlowGraphProbe["window:procedural-main"].hostSnapshotJson(); try { const j = JSON.parse(raw); return JSON.stringify(j).match(/"(?:kind|type|nodeKind)":"[a-z.]+"/g)?.slice(0, 20); } catch (e) { return raw.slice(0, 300); } });
console.log("SNAP", JSON.stringify(snap));
const hosts = await page.evaluate(() => [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map(el => ({ id: el.getAttribute("data-surface-id"), status: (el.getAttribute("data-status-json") ?? "").slice(0, 500), meshes: (el.getAttribute("data-meshes-json") ?? "").length, faults: [...document.querySelectorAll("[data-fault-code]")].map(e => e.getAttribute("data-fault-code")) })));
console.log("HOSTS", JSON.stringify(hosts));
const info = await page.evaluate(() => ({
  flowProbe: Object.fromEntries(Object.entries(window.__semioFlowGraphProbe ?? {}).map(([k, v]) => [k, { nodes: v.nodeIds().length, viewport: v.viewport(), rect: v.rect(), snap: (v.hostSnapshotJson() ?? "").length }])),
  contributions: String(JSON.stringify(window.__semioOsInstalledContributions) ?? typeof window.__semioOsInstalledContributions).slice(0, 600),
  catalog: String(JSON.stringify(window.__semioOsCatalogProbe) ?? typeof window.__semioOsCatalogProbe).slice(0, 400),
  mounted: String(JSON.stringify(window.__semioMountedServiceProbe) ?? typeof window.__semioMountedServiceProbe).slice(0, 600),
  globals: Object.keys(window).filter(k => /semio|wasm|flow|__/i.test(k)).slice(0, 80),
  msgs: window.__msgs.slice(0, 40),
  canvases: [...document.querySelectorAll("canvas")].map(c => `${c.width}x${c.height}`),
  flowDom: (document.querySelector('[data-window-id*="flow" i], [id*="flow" i]')?.outerHTML ?? "none").slice(0, 1500),
  title: document.title,
  testids: [...new Set([...document.querySelectorAll("[data-testid],[data-lane],[data-kind]")].map(e => e.getAttribute("data-testid") ?? e.getAttribute("data-lane") ?? e.getAttribute("data-kind")))].slice(0, 80),
}));
await page.screenshot({ path: join(out, `${tag}.png`) });
writeFileSync(join(out, `${tag}-console.txt`), lines.join("\n"));
writeFileSync(join(out, `${tag}-info.json`), JSON.stringify(info, null, 1));
console.log("gpu", gpu, JSON.stringify(canv)); console.log(lines.length, "lines");
console.log(lines.filter(l => /^error|pageerror|http [45]|requestfailed|worker|panick/.test(l)).slice(0, 30).join("\n"));
clearTimeout(cap); await browser.close();
