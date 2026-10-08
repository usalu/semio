import { strict as assert } from "node:assert";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import { chromium } from "playwright";
import { URL as OracleURL } from "whatwg-url";
import fixture from "./🔣️.json";
import schema from "./🧬️schema.json";

if (process.argv[2] !== "probe") throw new Error("Use probe");
assert.equal(new Ajv({ strict: true }).compile(schema)(fixture), true);
const workspace = resolve(import.meta.dirname, "../../../../../../../..");
const ticket = resolve(import.meta.dirname, "..");
const pages = join(ticket, "🗑️generated/worker-route-probe/pages");
const report = join(ticket, "📓️worker-route-adapter-proof.md");
const { servePublishedPlay } = await import(pathToFileURL(join(workspace, "🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📦️release/🟦️.ts")).href);
const { installPublishedPlayRoutes } = await import(pathToFileURL(join(workspace, "🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/📦️release/🎭️fixture/🟦️.ts")).href);
const moduleURL = `https://${fixture.module.host}/${fixture.module.path}`;
const requests = fixture.assets.map(row => ({ url: `https://${row.host}/${row.path}`, token: row.content.token }));
rmSync(pages, { recursive: true, force: true });
for (const page of ["play", "map", "media", "modules"]) {
  const directory = join(pages, page);
  mkdirSync(directory, { recursive: true });
  writeFileSync(join(directory, "CNAME"), `${page === "play" ? fixture.pageHost : `${page}.assets.semio-tech.com`}\n`);
  writeFileSync(join(directory, ".nojekyll"), "");
  if (page !== "play") writeFileSync(join(directory, "_headers"), "/*\n  Access-Control-Allow-Origin: *\n");
}
writeFileSync(join(pages, "play/index.html"), '<!doctype html><html lang="en"><title>Published Worker Routing Probe</title></html>');
for (const row of fixture.assets) {
  const file = join(pages, row.page, row.path);
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, JSON.stringify(row.content));
}
const moduleFile = join(pages, fixture.module.page, fixture.module.path);
mkdirSync(dirname(moduleFile), { recursive: true });
writeFileSync(moduleFile, `export const token=${JSON.stringify(fixture.module.token)}; console.log("[DEBUG] CDN dynamic module evaluated");`);
const worker = join(pages, "play", fixture.workerPath);
mkdirSync(dirname(worker), { recursive: true });
writeFileSync(worker, `try {
  const results=[];
  for(const row of ${JSON.stringify(requests)}) {
    const response=await fetch(row.url);
    const value=await response.json();
    results.push({url:response.url,status:response.status,token:value.token});
    console.log("[DEBUG] Worker CDN fetch "+response.url+" "+response.status+" "+value.token);
  }
  const imported=await import(${JSON.stringify(moduleURL)});
  console.log("[DEBUG] Worker CDN dynamic import "+${JSON.stringify(moduleURL)}+" "+imported.token);
  self.postMessage({ok:true,origin:self.location.origin,fetches:results,imported:imported.token});
} catch(error) { self.postMessage({ok:false,error:String(error)}); }`);
const service = servePublishedPlay(pages);
process.env.PLAYWRIGHT_BASE_URL = service.baseURL;
const { default: config } = await import(pathToFileURL(join(workspace, "🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts")).href);
let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;
const consoleProof: string[] = [], responses: { url: string; status: number; cors?: string; mime?: string }[] = [], errors: string[] = [];
let result: unknown, failure: unknown;
const stop = (): void => { void browser?.close(); };
process.once("SIGINT", stop); process.once("SIGTERM", stop);
try {
  browser = await chromium.launch(config.projects[0].use.launchOptions);
  const context = await browser.newContext();
  await installPublishedPlayRoutes(context, service.origins);
  context.on("response", response => {
    if (!/^https:\/\/(?:map|media|modules)\.assets\.semio-tech\.com\//u.test(response.url())) return;
    const headers = response.headers();
    responses.push({ url: response.url(), status: response.status(), cors: headers["access-control-allow-origin"], mime: headers["content-type"] });
  });
  const page = await context.newPage();
  page.on("console", message => { consoleProof.push(message.text()); console.log(message.text()); });
  page.on("pageerror", error => errors.push(String(error)));
  await page.goto(service.baseURL);
  result = await page.evaluate(async path => await new Promise((resolve, reject) => {
    const worker = new Worker(`/${path}`, { type: "module" });
    const timeout = setTimeout(() => { worker.terminate(); reject(new Error("Worker route probe exceeded 20 seconds")); }, 20_000);
    worker.onmessage = event => { clearTimeout(timeout); worker.terminate(); resolve(event.data); };
    worker.onerror = event => { clearTimeout(timeout); worker.terminate(); reject(new Error(event.message)); };
  }), fixture.workerPath);
  const observed = result as { ok: boolean; origin: string; fetches: { url: string; status: number; token: string }[]; imported: string };
  assert.equal(observed.ok, true, JSON.stringify(result));
  assert.equal(observed.origin, new URL(service.baseURL).origin);
  assert.deepEqual(observed.fetches, requests.map(row => ({ url: new OracleURL(row.url).href, status: 200, token: row.token })));
  assert.equal(observed.imported, fixture.module.token);
  assert.deepEqual(responses.map(row => row.url).sort(), [...requests.map(row => new OracleURL(row.url).href), new OracleURL(moduleURL).href].sort());
  for (const row of responses) { assert.equal(row.status, 200); assert.equal(row.cors, "*"); assert.match(row.mime ?? "", row.url.endsWith(".js") ? /javascript/u : /json/u); }
  assert.equal(errors.length, 0);
  assert.equal(consoleProof.some(row => row.includes("Worker CDN dynamic import")), true);
  console.log(`[DEBUG] Same-origin module worker route adapter verified: ${JSON.stringify(result)}`);
} catch (error) { failure = error; throw error; }
finally {
  process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop);
  await browser?.close(); await service.stop();
  writeFileSync(report, `# Worker Route Adapter Proof\n\nRan ${new Date().toISOString()} using the exact production \`installPublishedPlayRoutes\` installer and actual acceptance browser-launch settings. All pages in this probe are explicitly synthetic.\n\nStatus: ${failure ? `FAILED: ${String(failure)}` : "PASSED"}.\n\n${failure ? "The browser probe failed; recorded observations below identify which checks completed." : "The worker launched from the app’s local origin; fetches and the dynamic module import retained their exact baked HTTPS CDN origins and percent-encoded emoji paths in browser responses. The production adapter fulfilled them from the corresponding independent static satellite listeners."}\n\nObserved worker result:\n\n\`\`\`json\n${JSON.stringify(result, null, 2)}\n\`\`\`\n\nObserved CDN responses:\n\n\`\`\`json\n${JSON.stringify(responses, null, 2)}\n\`\`\`\n\nReal browser console proof:\n\n\`\`\`text\n${consoleProof.join("\n")}\n\`\`\`\n\nThis validates the route adapter and worker browser behavior, not the fresh build's generated components or actual hosting-provider configuration.\n`);
}
