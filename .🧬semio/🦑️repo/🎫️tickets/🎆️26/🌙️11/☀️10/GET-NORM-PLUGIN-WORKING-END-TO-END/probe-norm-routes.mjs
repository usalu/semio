import {readFile} from "node:fs/promises";
import {createRequire} from "node:module";
import {basename, resolve} from "node:path";
import {fileURLToPath} from "node:url";
const ticket = fileURLToPath(new URL(".", import.meta.url));
const play = resolve(ticket, "../../../../../../../🏢️semio-tech/🎡️play/package.json");
const require = createRequire(play);
const {chromium} = require("playwright");
const workerFile = process.env.PLAY_TEST_SHARD_WORKER;
const stockModulePath = "/🧵️renderer-work/🔣️stock.json";
const operationCopyOverride = process.env.PLAY_TEST_OPERATION_COPY_BYTES === undefined
  ? undefined : Number(process.env.PLAY_TEST_OPERATION_COPY_BYTES);
const previewCore = process.env.PLAY_TEST_GUEST;
const previewBindings = process.env.PLAY_TEST_GUEST_JS;
const previewSource = previewBindings && await readFile(previewBindings, "utf8").then(async source => {
  const { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } = await import(resolve(ticket, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts"));
  return rewritePreview2ShimImportSource(source, `../${PREVIEW2_VENDOR_RELATIVE}/`);
});
if (operationCopyOverride !== undefined && (!Number.isSafeInteger(operationCopyOverride) || operationCopyOverride <= 0)) {
  throw new Error("PLAY_TEST_OPERATION_COPY_BYTES must be a positive safe integer");
}
const workerSource = workerFile && await readFile(workerFile,"utf8");
const exercisePicker = process.env.PLAY_TEST_PICKER_ROUNDTRIP === "1";
const ids = process.argv.slice(2);
const concurrency = Number.parseInt(process.env.PLAY_ROUTE_CONCURRENCY ?? "1", 10);
if (!Number.isInteger(concurrency) || concurrency < 1 || concurrency > 4) throw new Error("PLAY_ROUTE_CONCURRENCY must be 1–4");

async function probe(id) {
  const faults = new Set();
  const coreUrls = new Set();
  const failedRequests = new Set();
  const stockResponses = [];
  const started = Date.now();
  let browser;
  let page;
  let preview;
  let bindings;
  let worker;
  let pickerRoundTrip;
  try {
    browser = await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader","--disable-renderer-backgrounding"]});
    page = await browser.newPage({locale:"en-US"});
    page.on("response", response => {
      const url = new URL(response.url());
      if (!response.ok()) return;
      const path = decodeURIComponent(url.pathname);
      if (path.endsWith(".core.wasm")) coreUrls.add(path);
      if (path.endsWith(stockModulePath)) stockResponses.push(response.text().then(body => {
        const operation = body.match(/^export const operation = (.+);$/m);
        return { path, operationCopyBytes: operation ? JSON.parse(operation[1]).grant.maximumCopyBytes : undefined };
      }).catch(error => ({ path, error: String(error) })));
    });
    page.on("requestfailed", request => {
      const url = new URL(request.url());
      if (url.hostname === "127.0.0.1" || url.hostname === "localhost") {
        failedRequests.add(`${decodeURIComponent(url.pathname)}: ${request.failure()?.errorText ?? "unknown"}`);
      }
    });
    if (operationCopyOverride !== undefined) {
      await page.route(url => decodeURIComponent(url.pathname).endsWith(stockModulePath), async route => {
        const response = await route.fetch();
        const source = await response.text();
        const operation = /^export const operation = (.+);$/m;
        if (!response.ok() || !operation.test(source)) throw new Error("Renderer operation stock module unavailable");
        const rewritten = source.replace(operation, (_, json) => {
          const stock = JSON.parse(json);
          stock.grant.maximumCopyBytes = operationCopyOverride;
          return `export const operation = ${JSON.stringify(stock)};`;
        });
        await route.fulfill({response, body:rewritten});
      });
    }
    if (workerSource) {
      await page.context().route(url => decodeURIComponent(url.pathname).endsWith("/🟨️shard-worker.js"), async route => {
        worker = route.request().url();
        await route.fulfill({status:200, contentType:"text/javascript", body:workerSource});
      });
    }
    if (previewCore) {
      await page.context().route(url => url.pathname.endsWith("/" + basename(previewCore)), async route => {
        preview = previewCore;
        const url = new URL(route.request().url());
        url.pathname = "/@fs" + previewCore;
        await route.continue({url:url.href});
      });
    }
    if (previewSource !== undefined && previewBindings !== undefined) {
      await page.context().route(url => url.pathname.endsWith("/" + basename(previewBindings)), async route => {
        bindings = previewBindings;
        await route.fulfill({ contentType: "text/javascript", body: previewSource });
      });
    }
    const record = (line, isError) => {
      if (isError || /semantic failure|cleanup job body unreachable|authority failure|boot failed|input #\d+ .*refused|load-failed/.test(line)) faults.add(line.split("\n")[0].slice(0,2500));
    };
    page.on("console", message => record(message.text(), message.type() === "error"));
    page.on("pageerror", error => faults.add(error.message));
    page.on("crash", () => faults.add("renderer crashed"));
    await page.goto(new URL("#"+id,process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033/").href,{waitUntil:"commit"});
    const shell = page.locator('[data-shell-id="'+id+'"]');
    await shell.locator('xpath=self::*[@data-shell-error or @data-shell-ready or @data-shell-not-found]').waitFor({timeout:120000});
    await page.waitForTimeout(3000);
    const shellState = await shell.evaluate(el => ({ shell: { ...el.dataset }, shellErrorText: el.hasAttribute("data-shell-error") ? el.innerText.slice(0, 4000) : undefined }));
    if (previewBindings && (!preview || !bindings)) throw new Error("Preview core and bindings must both be fetched");
    if (exercisePicker && shellState.shell.shellReady === id) {
      const pickerSelector = `[data-layered-pane="${id}"] [id="playground.navbar.fixture"]`;
      const picker = page.locator(pickerSelector);
      const initial = (await picker.innerText({timeout:10000})).trim();
      const canvases = await shell.locator("canvas").evaluateAll(nodes => nodes.map(node => [node.width,node.height]));
      pickerRoundTrip = {initial,canvases};
      await picker.click({timeout:10000});
      const options = page.getByRole("option");
      await options.first().waitFor({timeout:10000});
      pickerRoundTrip.offered = (await options.allTextContents()).map(label => label.trim());
      const alternative = pickerRoundTrip.offered.find(label => label !== initial);
      if (alternative === undefined) throw new Error("Picker exposes no alternative to the curated example");
      await page.getByRole("option", {name:alternative,exact:true}).click({timeout:10000});
      for (const label of [alternative,initial]) {
        if (label === initial) {
          await picker.click({timeout:10000});
          await page.getByRole("option", {name:label,exact:true}).click({timeout:10000});
        }
        await page.waitForFunction(([selector,text]) => document.querySelector(selector)?.textContent?.trim() === text,
          [pickerSelector,label], {timeout:30000});
        pickerRoundTrip[label === initial ? "restored" : "selected"] = label;
      }
    }
    console.log(JSON.stringify({id,preview,bindings,worker,elapsedMs:Date.now()-started,cores:[...coreUrls],rendererStock:await Promise.all(stockResponses),operationCopyOverride,pickerRoundTrip,...shellState,windows:await shell.locator('[data-slot="window-body"]').allInnerTexts(),faults:[...faults],failedRequests:[...failedRequests]}));
  } catch(error) {
    const pageState = await page?.locator('body').evaluate(element => ({
      status: Array.from(element.querySelectorAll('[role="status"], [role="progressbar"]'), row => row.textContent?.trim()),
      body: element.innerText.slice(0, 1200),
    }), undefined, {timeout: 2000}).catch(() => undefined);
    console.log(JSON.stringify({id,preview,bindings,worker,elapsedMs:Date.now()-started,cores:[...coreUrls],rendererStock:await Promise.all(stockResponses),operationCopyOverride,pickerRoundTrip,error:String(error),pageState,faults:[...faults],failedRequests:[...failedRequests]}));
  } finally {await browser?.close();}
}

let next = 0;
await Promise.all(Array.from({length: Math.min(ids.length, concurrency)}, async () => {
  while (next < ids.length) await probe(ids[next++]);
}));
