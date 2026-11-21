import { createRequire } from "node:module";
import { createReadStream, readFileSync, statSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { once } from "node:events";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const require = createRequire(process.cwd() + "/package.json");
const { chromium } = require("playwright");
const origin = new URL(process.argv[2] ?? "http://127.0.0.1:6033/");
const entry = new URL("#din4108", origin);
const afterPicker = process.env.ROLE_AFTER_ROUND_TRIP !== "0";
const copyBytes = Number(process.env.ROLE_OPERATION_COPY_BYTES ?? 0);
const itemStock = Number(process.env.ROLE_OPERATION_ITEMS ?? 0);
const rootCopyBytes = Number(process.env.ROLE_ROOT_COPY_BYTES ?? 0);
if (![copyBytes, itemStock, rootCopyBytes].every(value => Number.isSafeInteger(value) && value >= 0)) {
  throw new Error("Invalid resource stock");
}
const counterCore = process.env.ROLE_NATIVE_COUNTER_CORE;
const profileClose = process.env.ROLE_PROFILE_CLOSE === "1";
const waitRetirement = process.env.ROLE_WAIT_RETIREMENT === "1";
const unsignedStage = process.env.ROLE_UNSIGNED_STAGE;
if (unsignedStage && counterCore) throw new Error("Select one guest overlay");

const started = Date.now();
const trace = (message, ...values) => console.log("[DEBUG]", ((Date.now() - started) / 1000).toFixed(1), message, ...values);
const browser = await chromium.launch({ args: ["--in-process-gpu", "--enable-unsafe-swiftshader", "--disable-renderer-backgrounding"] });
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1440, height: 1000 } });
await page.addInitScript(() => {
  globalThis.__roleProbeDraws = 0;
  for (const [prototype, methods] of [
    [globalThis.GPURenderPassEncoder?.prototype, ["draw", "drawIndexed"]],
    [globalThis.WebGLRenderingContext?.prototype, ["drawArrays", "drawElements"]],
    [globalThis.WebGL2RenderingContext?.prototype, ["drawArrays", "drawElements", "drawArraysInstanced", "drawElementsInstanced"]],
  ]) {
    for (const method of methods) {
      const draw = prototype?.[method];
      if (typeof draw !== "function") continue;
      prototype[method] = function (...args) {
        const result = Reflect.apply(draw, this, args);
        globalThis.__roleProbeDraws++;
        return result;
      };
    }
  }
});
let archiveAcknowledgements = 0;
const predecessors = new Map();
const retirementTerminals = new Map();
const retirementResults = () => Array.from(predecessors, ([role, instance]) => ({ role, instance,
  result: retirementTerminals.get(instance) ?? "pending" }));


async function workerProfiler() {
  const cdp = await page.context().newCDPSession(page);
  const workers = new Map();
  const pending = new Map();
  let id = 0;
  cdp.on("Target.attachedToTarget", ({ sessionId, targetInfo }) => {
    if (targetInfo.type === "worker") workers.set(sessionId, targetInfo.url);
  });
  cdp.on("Target.detachedFromTarget", ({ sessionId }) => workers.delete(sessionId));
  cdp.on("Target.receivedMessageFromTarget", ({ message }) => {
    const reply = JSON.parse(message);
    if (reply.id !== undefined) pending.get(reply.id)?.(reply);
  });
  const call = async (sessionId, method) => {
    const requestId = ++id;
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => { pending.delete(requestId); reject(new Error(`${method} timed out`)); }, 5000);
      pending.set(requestId, reply => {
        clearTimeout(timer);
        pending.delete(requestId);
        if (reply.error) reject(new Error(reply.error.message));
        else resolve(reply.result);
      });
      cdp.send("Target.sendMessageToTarget", { sessionId, message: JSON.stringify({ id: requestId, method }) })
        .catch(error => { clearTimeout(timer); pending.delete(requestId); reject(error); });
    });
  };
  await cdp.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: false });
  return async (purpose = "third-worker", duration = 8000) => {
    const delay = Number(process.env.ROLE_PROFILE_DELAY_MS ?? 0);
    if (delay) await page.waitForTimeout(delay);
    trace("profiling exact worker targets", JSON.stringify([...workers.values()]));
    await Promise.allSettled([...workers.entries()].map(async ([session, url], index) => {
      try {
        await call(session, "Profiler.enable");
        await call(session, "Profiler.start");
        await page.waitForTimeout(duration);
        const { profile } = await call(session, "Profiler.stop");
        writeFileSync(new URL(`./🗑️generated/din4108-${purpose}-${delay ? "late-" : ""}${index}.cpuprofile`, import.meta.url), JSON.stringify(profile));
        trace("worker profile complete", index, url, profile.samples?.length);
      } catch (error) { trace("worker profile refused", index, url, String(error)); }
    }));
  };
}

let previewServer;
try {
  if (unsignedStage) {
    const coreName = "semio_hub_norm_component.core.wasm";
    const bindingsName = "semio_hub_norm_component.js";
    const previewCore = join(unsignedStage, coreName);
    const importsFile = join(process.cwd(), "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts");
    const { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } = await import(pathToFileURL(importsFile).href);
    const code = rewritePreview2ShimImportSource(readFileSync(join(unsignedStage, bindingsName), "utf8"), `../${PREVIEW2_VENDOR_RELATIVE}/`);
    const size = statSync(previewCore).size;
    previewServer = createServer((request, response) => {
      if (request.url !== `/${coreName}`) {
        response.writeHead(404).end();
        return;
      }
      trace("unsigned core served", request.method, size);
      response.writeHead(200, {
        "Content-Type": "application/wasm",
        "Content-Length": size,
        "Access-Control-Allow-Origin": origin.origin,
        "Cross-Origin-Resource-Policy": "cross-origin",
        "Cache-Control": "no-store",
      });
      const bytes = createReadStream(previewCore);
      bytes.on("error", error => response.destroy(error));
      response.on("close", () => bytes.destroy());
      bytes.pipe(response);
    }).listen(0, "127.0.0.1");
    await once(previewServer, "listening");
    const modulePath = "/🔌️plugin-modules/📕️norm/";
    await page.context().route(url => decodeURIComponent(url.pathname) === modulePath + coreName,
      route => route.continue({ url: `http://127.0.0.1:${previewServer.address().port}/${coreName}` }));
    await page.context().route(url => decodeURIComponent(url.pathname) === modulePath + bindingsName,
      route => {
        trace("unsigned bindings served", code.length);
        return route.fulfill({ contentType: "text/javascript", body: code });
      });
    trace("unsigned overlay", unsignedStage);
  }
  const profileWorkers = process.env.ROLE_PROFILE_WORKERS === "1" || profileClose ? await workerProfiler() : null;
  let closingProfile;
  let switchingRole;
  let profileStarted = false;
  page.on("console", message => {
    const archivePrefix = "[DEBUG] role transfer archive input ";
    if (message.text().startsWith(archivePrefix)) {
      const { index, pack, spr } = JSON.parse(message.text().slice(archivePrefix.length));
      const archive = `./🗑️generated/din4108-${afterPicker ? "round-trip" : "initial-role"}-${index}${copyBytes ? `-${copyBytes}` : ""}${itemStock ? `-${itemStock}items` : ""}${rootCopyBytes ? `-${rootCopyBytes}root` : ""}`;
      writeFileSync(new URL(`${archive}.pack`, import.meta.url), Uint8Array.from(pack));
      writeFileSync(new URL(`${archive}.spr`, import.meta.url), Uint8Array.from(spr));
      trace("captured archive input", index, pack.length, spr.length);
      return;
    }
    const transfer = message.text().match(/^\[DEBUG\] role transfer begin \S+ \S+#(viewer|editor) (\d+)$/);
    if (transfer) predecessors.set(transfer[1] === "viewer" ? "editor" : "viewer", Number(transfer[2]));
    const closed = message.text().match(/^\[DEBUG\] role transfer lifecycle closed (\d+) complete$/);
    if (closed) retirementTerminals.set(Number(closed[1]), "complete");
    const failed = message.text().match(/switchToPluginApp: predecessor \S+#(editor|viewer) retirement failed/);
    if (failed) {
      const instance = predecessors.get(failed[1]);
      if (instance !== undefined) retirementTerminals.set(instance, "failed");
    }
    if (message.text() === "[DEBUG] role transfer archive acknowledged") archiveAcknowledgements++;
    if (profileClose && profileWorkers && !closingProfile && switchingRole === "editor" && message.text() === "[DEBUG] role transfer restore returned") {
      closingProfile = profileWorkers("viewer-retirement", 18_000).catch(error => trace("close profile failed", String(error)));
    }
    if (!profileClose && profileWorkers && !profileStarted && archiveAcknowledgements === 2 && /archive progress \[\d+,"running",118,118\]/.test(message.text())) {
      profileStarted = true;
      void profileWorkers().catch(error => trace("worker profile failed", String(error)));
    }
    if (message.text().includes("[DEBUG] role transfer") || ["error", "warning"].includes(message.type())) trace(`[${message.type()}] ${message.text().slice(0, 3500)}`);
  });
  page.on("pageerror", error => trace("page error", error.stack ?? String(error)));
  page.on("response", response => {
    const path = decodeURIComponent(new URL(response.url()).pathname);
    if (path.endsWith(".core.wasm")) trace("core", response.status(), path);
  });
  if (counterCore) {
    await page.context().route(url => url.pathname.endsWith("/semio_hub_norm_component.core.wasm"), route => {
      const next = new URL(route.request().url());
      next.pathname = "/@fs" + counterCore;
      return route.continue({ url: next.href });
    });
    await page.context().route(url => url.pathname.endsWith("/semio_hub_norm_component.js"), async route => {
      const response = await route.fetch();
      const source = await response.text();
      const diagnostic = source.replace("memory0 = exports1.memory;",
        'memory0 = exports1.memory; globalThis.__roleArchiveCounters = Object.entries(exports1).filter(([name]) => /^archive(Count|Value)/.test(name));');
      if (diagnostic === source) throw new Error("No bootstrap attachment for the exact core counters");
      await route.fulfill({ response, body: diagnostic });
    });
  }
  if (copyBytes || itemStock || rootCopyBytes) await page.route(url => decodeURIComponent(url.pathname).endsWith("/🧵️renderer-work/🔣️stock.json"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const field = /^export const operation = (.+);$/m;
    if (!response.ok() || !field.test(source)) throw new Error("Operation admission stock module unavailable");
    let diagnostic = source.replace(field, (_, json) => {
      const admission = JSON.parse(json);
      if (copyBytes) {
        trace("isolated operation copy stock", admission.grant.maximumCopyBytes, copyBytes);
        admission.grant.maximumCopyBytes = copyBytes;
      }
      if (itemStock) {
        trace("isolated operation item stock", admission.grant.maximumItems, itemStock);
        admission.grant.maximumItems = itemStock;
      }
      return `export const operation = ${JSON.stringify(admission)};`;
    });
    if (rootCopyBytes) {
      const stockField = /^export const stock = (.+);$/m;
      if (!stockField.test(source)) throw new Error("Kernel admission stock module unavailable");
      diagnostic = diagnostic.replace(stockField, (_, json) => {
        const stock = JSON.parse(json);
        trace("isolated kernel copy stock", stock.grant.maximumCopyBytes, rootCopyBytes);
        stock.grant.maximumCopyBytes = rootCopyBytes;
        return `export const stock = ${JSON.stringify(stock)};`;
      });
    }
    await route.fulfill({ response, body: diagnostic });
  });
  if (profileClose || waitRetirement) await page.route(url => decodeURIComponent(url.pathname).includes("/🧺️handle-ui-intake/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source.replace(
      "lease.progress().kind !== \"complete\"; step += 1) {",
      `lease.progress().kind !== "complete"; step += 1) {
      if ((step & (step - 1)) === 0) console.info("[DEBUG] role transfer lifecycle close", instanceId, step, lease.progress().kind, lease.pendingReceipt?.kind);`,
    );
    const body = diagnostic.replace(/\blease\.dispose\(\);/, `console.info("[DEBUG] role transfer lifecycle closed", instanceId, lease.progress().kind);
    lease.dispose();`);
    trace("lifecycle close observer", diagnostic !== source, "terminal", body !== diagnostic);
    await route.fulfill({ response, body });
  });
  await page.route(url => decodeURIComponent(url.pathname).includes("/🟠️switch-plugin-app/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source
      .replace("setSurfaceSwitchBusy(true);", `console.info("[DEBUG] role transfer begin", pluginId, appId, documentSource?.instanceId);
      setSurfaceSwitchBusy(true);`)
      .replace("return quiet;", `console.info("[DEBUG] role transfer quiesce", retiring.instanceId, quiet.settled, quiet.pending);
              return quiet;`)
      .replace('if (outcome.status === "draining" || outcome.status === "busy") {', `console.info("[DEBUG] role transfer outcome", outcome.status, outcome.session?.app.id, outcome.session?.instanceId, outcome.pending);
        if (outcome.status === "draining" || outcome.status === "busy") {`);
    trace("switch gate observer", diagnostic !== source);
    await route.fulfill({ response, body: diagnostic });
  });
  await page.route(url => decodeURIComponent(url.pathname).includes("/🔀️surface-switch/📄️document/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    let diagnostic = source;
    for (const [call, stage] of [
      ["const archive = await ports.capture();", "capture"],
      ["const surface = await ports.create();", "create"],
      ["await ports.restore(surface, archive);", "restore"],
      ["await ports.attach(surface);", "attach"],
      ["await ports.release(surface);", "rollback"],
    ]) {
      diagnostic = diagnostic.replace(call, `console.info("[DEBUG] role transfer ${stage} start");\n${call}\nconsole.info("[DEBUG] role transfer ${stage} returned");`);
    }
    trace("document transfer observer", diagnostic !== source);
    await route.fulfill({ response, body: diagnostic });
  });
  await page.route(url => decodeURIComponent(url.pathname).includes("/🚀️program-dispatch/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source
      .replace("const releaseActionWork = sessionWorkRef.current.begin", `console.info("[DEBUG] role transfer typed work begin", targetSession.instanceId, action.action, entry.provenance.inputSeq);
  const releaseActionWork = sessionWorkRef.current.begin`)
      .replace("const startedOperation = startedTypedOperationV1(response.output);", `const startedOperation = startedTypedOperationV1(response.output);
    console.info("[DEBUG] role transfer typed work admitted", targetSession.instanceId, action.action, entry.provenance.inputSeq, startedOperation?.operationId, response.requestedEffects?.length);`)
      .replace("releaseActionWork();", `releaseActionWork();
    console.info("[DEBUG] role transfer typed work released", targetSession.instanceId, action.action, entry.provenance.inputSeq);`);
    trace("typed work observer", diagnostic !== source);
    await route.fulfill({ response, body: diagnostic });
  });
  await page.route(url => decodeURIComponent(url.pathname).includes("/📥️document-load-effects/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source.replace("const sprBytes = coerceWireBytes(payload.spr);", `const sprBytes = coerceWireBytes(payload.spr);
    globalThis.__roleProbeArchiveCount = (globalThis.__roleProbeArchiveCount ?? 0) + 1;
    console.info("[DEBUG] role transfer archive input " + JSON.stringify({ index: globalThis.__roleProbeArchiveCount, pack: Array.from(packBytes), spr: Array.from(sprBytes) }));`);
    trace("archive capture observer", diagnostic !== source);
    await route.fulfill({ response, body: diagnostic });
  });
  await page.route(url => decodeURIComponent(url.pathname).includes("/🧾️app-channel/🧳️archive-transfer/"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source.replace('await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);', `console.info("[DEBUG] role transfer archive load begin");
    let prior;
    await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, status => {
      const key = JSON.stringify([status.operation, status.state, status.completed, status.total]);
      if (key !== prior) console.info("[DEBUG] role transfer archive progress", key);
      prior = key;
      progress?.(status);
    });
    console.info("[DEBUG] role transfer archive acknowledged");`);
    if (diagnostic !== source) trace("archive progress observer true");
    await route.fulfill({ response, body: diagnostic });
  });
  await page.route(url => decodeURIComponent(url.pathname).endsWith("/📕️norm/🌉️bridge.js"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const diagnostic = source.replace("function receiveOriginal(call, workUnitsSpent) {", `let roleProbeSamples = 0;
  function receiveOriginal(call, workUnitsSpent) {
    if (++roleProbeSamples % 1024 === 0) {
      console.info("[DEBUG] role transfer native stock", String(call.input?.epoch), JSON.stringify(originalSource.retainedRemaining), originalSource.fuelRemaining, String(call.spent?.copiedItems), String(call.spent?.copiedBytes));
      if (globalThis.__roleArchiveCounters) console.info("[DEBUG] role transfer native counters", String(call.input?.epoch),
        JSON.stringify(Object.fromEntries(globalThis.__roleArchiveCounters.filter(([name, count]) => name.startsWith("archiveValue") || count.value !== 0).map(([name, count]) => [name.replace("archiveCount", "").replace("archiveValue", "value"), count.value]))));
    }`);
    trace("native stock observer", diagnostic !== source);
    await route.fulfill({ response, body: diagnostic });
  });
  await page.goto(entry.href, { waitUntil: "commit" });
  const shell = page.locator('[data-shell-id="din4108"]');
  await shell.locator("xpath=self::*[@data-shell-ready or @data-shell-error or @data-shell-not-found]").waitFor({ timeout: 125_000 });
  if (await shell.getAttribute("data-shell-ready") === null) {
    throw new Error(`DIN 4108 shell failed: ${await shell.getAttribute("data-shell-error")}`);
  }
  const picker = page.locator('[data-layered-pane="din4108"] [id="playground.navbar.fixture"]');
  const choose = async label => {
    await picker.click();
    await page.getByRole("option", { name: label, exact: true }).click();
    await page.waitForFunction(expected => document.getElementById("playground.navbar.fixture")?.innerText.trim() === expected,
      label, { timeout: 100_000 });
    trace("picker settled", label);
  };
  await page.getByText("✓ Complies", { exact: true }).first().waitFor({ timeout: 125_000 });
  const introduction = shell.locator('[id="ui.introduction.skip"]');
  if (await introduction.count()) await introduction.first().click();
  if (afterPicker) {
    await choose("Failing thin insulation");
    await choose("Compliant ETICS dwelling");
    if (process.env.ROLE_WAIT_ARCHIVE_ACK === "1") {
      if (archiveAcknowledgements < 3) await page.waitForEvent("console", {
        predicate: message => message.text() === "[DEBUG] role transfer archive acknowledged" && archiveAcknowledgements >= 3,
        timeout: 125_000,
      });
      await page.getByText("✓ Complies", { exact: true }).first().waitFor({ timeout: 125_000 });
      trace("restored archive, verdict and picker acknowledged", archiveAcknowledgements);
    }
  }
  for (const role of ["viewer", "editor"]) {
    switchingRole = role;
    const button = shell.locator(`[id="playground.navbar.roles.${role}"]`);
    trace("role click", role, await button.getAttribute("aria-pressed"));
    const drawsBefore = await page.evaluate(() => globalThis.__roleProbeDraws);
    const acknowledgementsBefore = archiveAcknowledgements;
    const refused = page.waitForEvent("console", {
      predicate: message => /role transfer outcome (draining|busy) /.test(message.text()),
      timeout: 90_000,
    }).then(() => { throw new Error("The previous app declined the role switch while its work was pending"); });
    await button.click();
    await Promise.race([refused, page.waitForFunction(expected => {
      const controls = document.getElementById("playground.navbar.roles");
      return controls?.getAttribute("aria-busy") !== "true" &&
        document.getElementById(`playground.navbar.roles.${expected}`)?.getAttribute("aria-pressed") === "true";
    }, role, { timeout: 90_000 })]);
    const chip = shell.locator('[data-slot="surface-role-chip"]');
    await page.waitForFunction(before => globalThis.__roleProbeDraws > before, drawsBefore, { timeout: 15_000 });
    const appId = await chip.getAttribute("data-app-id");
    const canvasCount = await shell.locator("canvas:visible").count();
    const roleAcknowledgements = archiveAcknowledgements - acknowledgementsBefore;
    trace("role settled", role, appId, "canvases", canvasCount,
      "draws", (await page.evaluate(() => globalThis.__roleProbeDraws)) - drawsBefore, "archive acknowledgements", roleAcknowledgements);
    if (waitRetirement && (!appId?.endsWith(`#${role}`) || canvasCount < 1 || roleAcknowledgements !== 1)) {
      throw new Error(`Role ${role} requires its own archive acknowledgement and painted surface`);
    }
  }
  if (waitRetirement) {
    if (afterPicker && archiveAcknowledgements !== 5) throw new Error("Expected all five archive acknowledgements");
    const settled = () => predecessors.size === 2 && retirementResults().every(row => row.result !== "pending");
    trace("retirement wait", JSON.stringify(retirementResults()));
    if (!settled()) await page.waitForEvent("console", { predicate: settled, timeout: 120_000 });
    const results = retirementResults();
    trace("retirement results", JSON.stringify(results));
    if (results.some(row => row.result !== "complete")) throw new Error("A predecessor failed its lifecycle close");
  }
  await closingProfile;
} catch (error) {
  trace("probe failed", String(error));
  if (waitRetirement) trace("retirement frontier", JSON.stringify(retirementResults()));
  const state = await page.evaluate(() => ({
    roles: Array.from(document.querySelectorAll("[data-slot=surface-role-chip], [id^=playground\\.navbar\\.roles]"), node => ({ id: node.id, role: node.getAttribute("data-role"), pressed: node.getAttribute("aria-pressed"), busy: node.getAttribute("aria-busy") })),
    body: document.body.innerText.slice(0, 1600),
  })).catch(() => undefined);
  trace("final page", JSON.stringify(state));
  process.exitCode = 1;
} finally {
  await browser.close();
  if (previewServer) await new Promise((resolve, reject) => previewServer.close(error => error ? reject(error) : resolve()));
}
