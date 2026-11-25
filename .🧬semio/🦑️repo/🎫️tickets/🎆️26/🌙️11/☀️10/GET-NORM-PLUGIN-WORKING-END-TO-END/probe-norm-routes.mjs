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
const operationItemsOverride = process.env.PLAY_TEST_OPERATION_ITEMS === undefined
  ? undefined : Number(process.env.PLAY_TEST_OPERATION_ITEMS);
const previewCore = process.env.PLAY_TEST_GUEST;
const previewBindings = process.env.PLAY_TEST_GUEST_JS;
const observeOwnership = process.env.PLAY_TEST_OBSERVE_OWNERSHIP === "1";
const observeLedger = process.env.PLAY_TEST_OBSERVE_LEDGER_SETTLEMENT === "1";
const observeWorkRows = process.env.PLAY_TEST_OBSERVE_WORK_ROWS === "1";
const previewSource = previewBindings && await readFile(previewBindings, "utf8").then(async source => {
  const { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } = await import(resolve(ticket, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts"));
  if (process.env.PLAY_TEST_OBSERVE_ARCHIVE_GRANT === "1") {
    const memory = "memory0 = exports1.memory;";
    if (!source.includes(memory)) throw Error("Guest archive stage observer unavailable");
    source = source.replace(memory, memory +
      "globalThis.__semioArchiveMemberStage = () => exports1.member_stage?.value;");
  }
  if (observeOwnership) {
    const memory = "memory0 = exports1.memory;";
    if (!source.includes(memory)) throw Error("Guest ownership observer unavailable");
    source = source.replace(memory, memory + `
      globalThis.__semioOwnershipRefusal = (message = false) => {
        const tracked = BigInt.asUintN(64, exports1.ownership_refusal.value);
        const stamp = Number(tracked >> 48n);
        if (!message) return stamp;
        const pointer = Number(tracked & 0xffffffffn);
        const length = Number(tracked >> 32n & 0xffffn);
        if (length === 0 || pointer + length > memory0.buffer.byteLength) return {stamp};
        return {stamp, message: new TextDecoder().decode(new Uint8Array(memory0.buffer, pointer, length))};
      };
    `);
  }
  if (observeLedger) {
    const memory = "memory0 = exports1.memory;";
    if (!source.includes(memory)) throw Error("Guest Source ledger observer unavailable");
    source = source.replace(memory, memory + `
      globalThis.__semioLedgerSettlement = () => {
        const read = name => exports1["ledger_" + name].value;
        const count = read("fault_count") >>> 0;
        if (count === 0) return {count};
        const physical = kind => Object.fromEntries(["items", "copy", "capacity", "release"].map(
          axis => [axis, read(kind + "_" + axis) >>> 0]));
        return {
          count, issued: physical("issued"), spent: physical("spent"), held: physical("held"), delta: physical("delta"),
          work: {issued: read("issued_work"), held: read("held_work"), spent: read("spent_work_low"), spentHigh: read("spent_work_high")},
          claims: read("claims") >>> 0, serial: read("serial"),
        };
      };
    `);
  }
  if (observeWorkRows) {
    const memory = "memory0 = exports1.memory;";
    if (!source.includes(memory)) throw Error("Guest Work custody observer unavailable");
    source = source.replace(memory, memory + `
      globalThis.__semioWorkCustody = () => {
        const read = name => exports1["ledger_" + name].value;
        const before = read("work_before");
        const after = read("work_after");
        const debit = {
          body: read("work_fault_body"), before, after, delta: after - before,
          held: read("work_held_on_fault"), issued: read("work_issued_on_fault"),
          site: exports1.ledger_work_failed_site?.value.toString(16),
        };
        const budget = (exports1.ledger_debit_budget_pointer?.value || read("budget_pointer")) >>> 0;
        const view = new DataView(memory0.buffer);
        if (!budget || budget + 1344 > view.byteLength) return {debit};
        const table = view.getUint32(budget + 1280, true);
        if (!table || table + 5976 > view.byteLength) return {debit};
        const number = (at, offset) => view.getBigUint64(at + offset, true);
        const rows = [];
        for (let index = 0; index < 32; index++) {
          if (view.getUint8(table + 5912 + index) !== 2) continue;
          const row = table + 24 + 184 * index;
          const held = number(row, 72);
          const redeemed = number(row, 112);
          const released = number(row, 152);
          rows.push({
            serial: number(row, 40), epoch: number(row, 48), extent: number(row, 32),
            activeScope: number(row, 24) === BigInt(budget + 416), held, redeemed, released,
            remaining: held - redeemed - released,
          });
        }
        return {debit, rows, scopeReservedWork: number(budget, 608), scopeNeedsRetirement: view.getUint8(budget + 617) === 1};
      };
    `);
  }
  return rewritePreview2ShimImportSource(source, `../${PREVIEW2_VENDOR_RELATIVE}/`);
});
const currentJobs = process.env.PLAY_TEST_CURRENT_JOBS === "1";
const traceJobs = process.env.PLAY_TEST_TRACE_JOBS === "1";
const observePollRefusal = process.env.PLAY_TEST_OBSERVE_POLL_REFUSAL === "1";
const windowlessNote = process.env.PLAY_TEST_WINDOWLESS_NOTE === "1";
const jobTracePrefix = "[DEBUG] reserved browser job";
const commandTracePrefix = "[DEBUG] browser command ingress";
let jobBridge;
let jobBridgeModule;
let reservedDriver;
function reservedDriverSpan(source) {
  const start = source.indexOf("const commitReservedToolJobWhileSerialized");
  const end = source.indexOf("const routeHostEffects", start);
  if (start < 0 || end <= start) throw Error("Reserved job renderer source unavailable");
  return [start, end];
}
function withPollRefusalObserver(source) {
  const rethrow = "throw nativeRefusalError(failure) ?? failure;";
  if (source.split(rethrow).length !== 2) throw Error("Original poll refusal observer unavailable");
  if (observeOwnership) {
    const poll = "let result;\n      try {\n        if (commandPage)";
    if (!source.includes(poll)) throw Error("Original poll ownership stamp unavailable");
    source = source.replace(poll, "let result;\n      const priorOwnership = globalThis.__semioOwnershipRefusal?.();\n      try {\n        if (commandPage)");
  }
  return source.replace(rethrow, `
        const refused = failure?.payload ?? failure;
        console.info("[DEBUG] observed archive poll refusal", JSON.stringify({
          category: refused?.failure?.tag,
          kind: refused?.failure?.val,
          events: events.map(event => event.kind),
          operation: turn.input.operation,
          generation: turn.input.generation,
          epoch: turn.input.epoch,
          spent: refused?.spent,
          workUnitsSpent: refused?.workUnitsSpent,
          remaining: originalSource.retainedRemaining,
          fuel: originalSource.fuelRemaining,
          ${observeOwnership ? "priorOwnership, ownership: globalThis.__semioOwnershipRefusal?.(true)," : ""}
          ${observeLedger ? "ledger: globalThis.__semioLedgerSettlement?.()," : ""}
          ${observeWorkRows ? "workCustody: globalThis.__semioWorkCustody?.()," : ""}
        }, (_, value) => typeof value === "bigint" ? value.toString() : value));
        ${rethrow}
  `);
}
function withHostOwnershipObserver(source) {
  const checks = [
    ["operationProgress", "owned > BigInt(originalTurn.input.grant.maximumCapacityBytes)", "completed, total, owned"],
    ["operationAllocation", "owned > capacity || next > capacity - owned", "bytes, owned, next, maximum"],
    ["operationReserveReturn", "count > BigInt(originalTurn.input.grant.maximumItems)", "kind, count"],
    ["operationFinish", "owned > BigInt(originalTurn.input.grant.maximumCapacityBytes)", "owned"],
  ];
  for (const [site, condition, fields] of checks) {
    const start = source.indexOf("export function " + site + "(");
    const end = source.indexOf("\n", start);
    const expression = "return " + condition + " ? 2 : 0;";
    const declaration = source.slice(start, end);
    if (!declaration.includes(expression)) throw Error("Host allocation observer unavailable at " + site);
    const observed = declaration.replace(expression, `return observeHostOwnership(${condition} ? 2 : 0, "${site}", {${fields}});`);
    source = source.slice(0, start) + observed + source.slice(end);
  }
  return source + `
    function observeHostOwnership(code, site, extent) {
      if (code === 2) console.info("[DEBUG] observed archive ownership host", JSON.stringify({
        site, extent, operation: originalTurn.input.operation, generation: originalTurn.input.generation,
        epoch: originalTurn.input.epoch, grant: originalTurn.input.grant,
      }, (_, value) => typeof value === "bigint" ? value.toString() : value));
      return code;
    }
  `;
}
if (currentJobs) {
  if (!previewCore || !previewBindings) throw Error("Current job transport requires a matching preview pair");
  const os = resolve(ticket, "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules");
  const {pluginComponentBridgeSource} = await import(resolve(os, "🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts"));
  const component = basename(previewBindings, ".js");
  jobBridgeModule = component.match(/^semio_hub_(norm(?:_\w+)*)_component$/u)?.[1].replaceAll("_", "-");
  if (!jobBridgeModule) throw Error("Current job transport requires a Norm component");
  jobBridge = pluginComponentBridgeSource(component, basename(previewCore));
  if (observePollRefusal) jobBridge = withPollRefusalObserver(jobBridge);
  const authored = await readFile(resolve(os, "📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts"), "utf8");
  const ts = require("typescript");
  let source = authored.slice(...reservedDriverSpan(authored));
  if (traceJobs) {
    source = source
      .replace("await shardClient.startJob(actorId, job, kind, input);", `console.info("${jobTracePrefix}", String(job), "start request", kind, input.byteLength);
      await shardClient.startJob(actorId, job, kind, input);
      console.info("${jobTracePrefix}", String(job), "start accepted");`)
      .replace("if (live()) await deliverJobCompletionTurn(instanceId, actorId, job, outcome);", `console.info("${jobTracePrefix}", String(job), "terminal", outcome.status);
          if (live()) {
            await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
            console.info("${jobTracePrefix}", String(job), "completion returned");
          }`);
  }
  reservedDriver = ts.transpileModule(source, {
    compilerOptions: {target:ts.ScriptTarget.ES2022, module:ts.ModuleKind.ESNext},
  }).outputText;
}
if (windowlessNote && !currentJobs) throw Error("Windowless note diagnostic requires the current matching guest and bridge");
if (operationCopyOverride !== undefined && (!Number.isSafeInteger(operationCopyOverride) || operationCopyOverride <= 0)) {
  throw new Error("PLAY_TEST_OPERATION_COPY_BYTES must be a positive safe integer");
}
if (operationItemsOverride !== undefined && (!Number.isSafeInteger(operationItemsOverride) || operationItemsOverride <= 0)) {
  throw new Error("PLAY_TEST_OPERATION_ITEMS must be a positive safe integer");
}
const workerSource = workerFile && await readFile(workerFile,"utf8");
const exercisePicker = process.env.PLAY_TEST_PICKER_ROUNDTRIP === "1";
const witnessInitialArchive = process.env.PLAY_TEST_INITIAL_ARCHIVE_WITNESS === "1";
const observeArchives = process.env.PLAY_TEST_OBSERVE_ARCHIVES === "1";
const requireDraw = process.env.PLAY_TEST_REQUIRE_DRAW === "1";
const observeArchiveGrant = process.env.PLAY_TEST_OBSERVE_ARCHIVE_GRANT === "1";
const shellWaitMs = Number(process.env.PLAY_TEST_SHELL_WAIT_MS ?? 120_000);
const pickerWaitMs = Number(process.env.PLAY_TEST_PICKER_WAIT_MS ?? 30_000);
if (!Number.isSafeInteger(shellWaitMs) || shellWaitMs <= 0) throw new Error("Shell deadline must be a positive safe integer");
if (!Number.isSafeInteger(pickerWaitMs) || pickerWaitMs <= 0) throw new Error("Picker deadline must be a positive safe integer");
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
  const archiveReceipts = [];
  try {
    browser = await chromium.launch({args:["--in-process-gpu","--enable-unsafe-swiftshader","--disable-renderer-backgrounding"]});
    page = await browser.newPage({locale:"en-US"});
    if (observeOwnership) {
      await page.context().route(url => {
        const parts = decodeURIComponent(url.pathname).split("/");
        return parts.at(-3) === "🔌️plugin-modules" && parts.at(-2)?.endsWith("norm-energy") && parts.at(-1) === "🟨️.js";
      }, async route => {
        const response = await route.fetch();
        await route.fulfill({response, body:withHostOwnershipObserver(await response.text())});
      });
    }
    if (observeArchives) {
      await page.route(url => decodeURIComponent(url.pathname).includes("/🧾️app-channel/🧳️archive-transfer/"), async route => {
        const response = await route.fetch();
        const source = await response.text();
        const transfer = 'await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);';
        if (!source.includes(transfer)) throw Error("Archive transfer acknowledgement site unavailable");
        await route.fulfill({response,body:source.replace(transfer, `
          let previousArchiveStatus;
          await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, status => {
            const current = JSON.stringify([status.operation, status.state, status.completed, status.total]);
            if (current !== previousArchiveStatus) console.info("[DEBUG] observed archive progress", current);
            previousArchiveStatus = current;
            progress?.(status);
          });
          globalThis.__semioArchiveReceipts = (globalThis.__semioArchiveReceipts ?? 0) + 1;
          console.info("[DEBUG] observed archive acknowledgement", globalThis.__semioArchiveReceipts);
        `)});
      });
    }
    page.on("response", response => {
      const url = new URL(response.url());
      if (!response.ok()) return;
      const path = decodeURIComponent(url.pathname);
      if (path.endsWith(".core.wasm")) coreUrls.add(path);
      if (path.endsWith(stockModulePath)) stockResponses.push(response.text().then(body => {
        const operation = body.match(/^export const operation = (.+);$/m);
        const issued = operation && JSON.parse(operation[1]);
        return {
          path,
          operationCopyBytes: issued?.grant.maximumCopyBytes,
          operationItems: issued?.grant.maximumItems,
          operationReleaseBytes: issued?.grant.maximumReleaseBytes,
          operationWork: issued?.fuel,
        };
      }).catch(error => ({ path, error: String(error) })));
    });
    page.on("requestfailed", request => {
      const url = new URL(request.url());
      if (url.hostname === "127.0.0.1" || url.hostname === "localhost") {
        failedRequests.add(`${decodeURIComponent(url.pathname)}: ${request.failure()?.errorText ?? "unknown"}`);
      }
    });
    if (operationCopyOverride !== undefined || operationItemsOverride !== undefined) {
      await page.route(url => decodeURIComponent(url.pathname).endsWith(stockModulePath), async route => {
        const response = await route.fetch();
        const source = await response.text();
        const operation = /^export const operation = (.+);$/m;
        if (!response.ok() || !operation.test(source)) throw new Error("Renderer operation stock module unavailable");
        const rewritten = source.replace(operation, (_, json) => {
          const stock = JSON.parse(json);
          if (operationCopyOverride !== undefined) stock.grant.maximumCopyBytes = operationCopyOverride;
          if (operationItemsOverride !== undefined) stock.grant.maximumItems = operationItemsOverride;
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
    if (currentJobs) {
      await page.context().route(url => {
        const parts = decodeURIComponent(url.pathname).split("/");
        return parts.at(-3) === "🔌️plugin-modules" && parts.at(-2)?.endsWith(jobBridgeModule) && parts.at(-1) === "🌉️bridge.js";
      }, async route => {
        await route.fulfill({status:200, contentType:"text/javascript", body:jobBridge});
      });
      await page.context().route(url => decodeURIComponent(url.pathname).includes("/🚚️handle-turn-drive/"), async route => {
        const response = await route.fetch();
        let source = await response.text();
        const [start,end] = reservedDriverSpan(source);
        source = source.slice(0,start) + reservedDriver + source.slice(end);
        if (traceJobs) {
          source = source
            .replace("const inspected = inspectEncodedAppCommand(events);", `const inspected = inspectEncodedAppCommand(events);
      const observeCommand = (...detail) => {
        if (inspected.actionId === "noteShellCommand" || inspected.actionId === "setActiveExample") console.info("${commandTracePrefix}", inspected.actionId, ...detail);
      };
      observeCommand("begin", inspected.seq);`)
            .replace(/const previousIngress = terminal;\s+await acceptTurn\(continued\);\s+terminal = foldIngress\(\);/, `$&
            if (continuations < 3 || continuations % 128 === 0 || terminal !== previousIngress) {
              observeCommand("continuation", continuations, terminal, wireTurnStatusTag(continued.status), continued.effects.length,
                Math.round(performance.now() - crossingStartedMs), continued.deferredEvents ?? 0,
                terminal === "fault" ? commandIngressFaultDisplay(results.at(-1)?.commandIngress) : "");
            }`)
            .replace("await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);", `observeCommand("settled", settled.effects.map(effect => effect.tag).join(","));
        await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
        observeCommand("returned");`);
        }
        if (windowlessNote) {
          const field = /command:\s*events\[commandIndex\]!?/;
          if (!field.test(source)) throw Error("Command source field unavailable for windowless note diagnostic");
          source = source.replace(field, `command: inspected.actionId === "noteShellCommand" ? isolateWindowlessNote(events[commandIndex]) : events[commandIndex]`);
          source += `\nfunction isolateWindowlessNote(bytes) {
  let at = 1;
  const next = () => {
    let value = 0, place = 1, byte;
    do {
      byte = bytes[at++];
      value += (byte & 127) * place;
      place *= 128;
    } while (byte & 128);
    return value;
  };
  next();
  const commandLength = next();
  const boundary = at + commandLength;
  const output = new Uint8Array(boundary + 1);
  output.set(bytes.subarray(0, boundary));
  console.info("${commandTracePrefix}", "windowless note source", bytes.length, output.length);
  return output;
}`;
        }
        await route.fulfill({response, body:source});
      });
    }
    if ((observeArchiveGrant || observePollRefusal) && !currentJobs) {
      await page.context().route(url => {
        const parts = decodeURIComponent(url.pathname).split("/");
        return parts.at(-3) === "🔌️plugin-modules" && parts.at(-2)?.endsWith("norm-energy") && parts.at(-1) === "🌉️bridge.js";
      }, async route => {
        const response = await route.fetch();
        let source = await response.text();
        if (observeArchiveGrant) {
          const receive = "function receiveOriginal(call, workUnitsSpent) {";
          if (!source.includes(receive)) throw Error("Original source receipt observer unavailable");
          source = source.replace(receive, `
          let archiveSamples = 0;
          ${receive}
          if (++archiveSamples % 2048 === 0) console.info("[DEBUG] observed archive grant", JSON.stringify({
            epoch: call.input?.epoch, remaining: originalSource.retainedRemaining, fuel: originalSource.fuelRemaining,
            untilDeadlineUs: Number(call.input?.work?.absoluteDeadlineUs ?? 0) - Number(hostShim.operationClockUs()),
            guest: call.remaining, spent: call.spent, workUnitsSpent, memberStage: globalThis.__semioArchiveMemberStage?.(),
          }, (_, value) => typeof value === "bigint" ? value.toString() : value));
          `);
        }
        if (observePollRefusal) source = withPollRefusalObserver(source);
        await route.fulfill({response,body:source});
      });
    }
    const record = (line, isError) => {
      if (isError || /semantic failure|cleanup job body unreachable|authority failure|boot failed|input #\d+ .*refused|load-failed/.test(line)) faults.add(line.split("\n")[0].slice(0,2500));
    };
    page.on("console", message => {
      record(message.text(), message.type() === "error");
      if (message.text().startsWith("[DEBUG] observed archive ")) console.error(Date.now()-started, message.text());
      if (observeArchiveGrant && message.type() === "error") console.error(Date.now()-started, message.text());
      if (message.text().startsWith("[DEBUG] observed archive acknowledgement ")) archiveReceipts.push(Date.now()-started);
      if (traceJobs && [jobTracePrefix, commandTracePrefix].some(prefix => message.text().startsWith(prefix))) console.log(message.text());
      if (traceJobs && message.type() === "error") console.log(message.text().slice(0, 5000));
    });
    page.on("pageerror", error => {
      faults.add(error.message);
      if (traceJobs || observeArchiveGrant) console.error(error.stack ?? error.message);
    });
    page.on("crash", () => faults.add("renderer crashed"));
    await page.goto(new URL("#"+id,process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033/").href,{waitUntil:"commit"});
    const shell = page.locator('[data-shell-id="'+id+'"]');
    await shell.locator('xpath=self::*[@data-shell-error or @data-shell-ready or @data-shell-not-found]').waitFor({timeout:shellWaitMs});
    await page.waitForTimeout(3000);
    const shellState = await shell.evaluate(el => ({ shell: { ...el.dataset }, shellErrorText: el.hasAttribute("data-shell-error") ? el.innerText.slice(0, 4000) : undefined }));
    if (shellState.shell.shellReady !== id) throw Error("The shell did not publish its ready state");
    const requireArchive = target => observeArchives && page.waitForFunction(
      count => (globalThis.__semioArchiveReceipts ?? 0) >= count, target, {timeout:pickerWaitMs});
    if (exercisePicker || witnessInitialArchive) await requireArchive(1);
    async function waitForDraw(previous = [], timeout = shellWaitMs) {
      const witness = await page.waitForFunction(({variant,previous}) => {
        const pane = document.querySelector(`[data-layered-pane="${variant}"]`);
        const observed = globalThis.__semioDrawSurfaces ??= { ids: new WeakMap(), next: 0 };
        const drawn = [];
        for (const canvas of pane?.querySelectorAll("canvas[data-world-geometry-drawn]") ?? []) {
          const surface = canvas.closest("[data-meshes-json][data-world-geometry-revision]");
          const vertices = Number(canvas.dataset.worldGeometryVertices);
          if (!surface || !canvas.getClientRects().length || vertices <= 0 || !Number.isFinite(vertices)) continue;
          if (canvas.dataset.worldGeometryRevision !== surface.dataset.worldGeometryRevision) continue;
          if (!observed.ids.has(canvas)) observed.ids.set(canvas, ++observed.next);
          const key = `${observed.ids.get(canvas)}:${surface.dataset.worldGeometryRevision}`;
          drawn.push({canvas,key});
        }
        const current = drawn.find(row => !previous.includes(row.key));
        return current && {canvas: current.canvas, keys: drawn.map(row => row.key)};
      }, {variant:id,previous}, {timeout});
      const keysHandle = await witness.getProperty("keys");
      const keys = await keysHandle.jsonValue();
      await keysHandle.dispose();
      const canvas = (await witness.getProperty("canvas")).asElement();
      const draw = await canvas.evaluate(element => ({
        revision: element.dataset.worldGeometryRevision,
        vertices: Number(element.dataset.worldGeometryVertices),
        width: element.width,
        height: element.height,
      }));
      if (witnessInitialArchive && previous.length === 0) {
        const image = require("pngjs").PNG.sync.read(await canvas.screenshot({timeout:20000}));
        const inset = Math.max(1, Math.floor(Math.min(image.width, image.height) / 5));
        const colors = new Set();
        for (let y = inset; y < image.height - inset; y += 2) {
          for (let x = inset; x < image.width - inset; x += 2) {
            colors.add(image.data.readUInt32BE((y * image.width + x) * 4));
          }
        }
        draw.interiorColors = colors.size;
        if (colors.size < 2) throw Error("The model canvas did not paint varying interior pixels");
      }
      await canvas.dispose();
      await witness.dispose();
      return {draw,keys};
    }
    let paint = requireDraw ? await waitForDraw() : undefined;
    const draw = paint?.draw;
    if (previewBindings && (!preview || !bindings)) throw new Error("Preview core and bindings must both be fetched");
    if ((exercisePicker || witnessInitialArchive) && shellState.shell.shellReady === id) {
      const pickerSelector = `[data-layered-pane="${id}"] [id="playground.navbar.fixture"]`;
      const picker = page.locator(pickerSelector);
      const initial = (await picker.innerText({timeout:10000})).trim();
      if (witnessInitialArchive && id === "din18599" && initial !== "Demo") {
        throw Error("The shell did not commit the curated Demo");
      }
      const canvases = await shell.locator("canvas").evaluateAll(nodes => nodes.map(node => [node.width,node.height]));
      const meshBytes = await shell.locator("[data-meshes-json]").evaluateAll(
        nodes => nodes.map(node => node.getAttribute("data-meshes-json")?.length ?? 0));
      const witnessPath = process.env.PLAY_TEST_WITNESS_PNG;
      if (witnessPath) await shell.screenshot({path:witnessPath,timeout:20000});
      pickerRoundTrip = {initial,canvases,meshBytes};
      if (exercisePicker) {
        await picker.click({timeout:10000});
        const listboxId = await picker.getAttribute("aria-controls");
        if (!listboxId) throw Error("The example picker does not name its listbox");
        const options = page.getByRole("listbox").and(page.locator(`[id=${JSON.stringify(listboxId)}]`)).getByRole("option");
        await options.first().waitFor({timeout:10000});
        const offered = await options.evaluateAll(nodes => nodes.map(node => ({
          value: node.dataset.value,
          label: node.textContent.trim(),
          selected: node.getAttribute("aria-selected") === "true",
          enabled: node.getAttribute("aria-disabled") !== "true",
        })));
        const named = offered.filter(option => option.value && option.value !== "__none__");
        const curated = named.find(option => option.selected && option.label === initial);
        const alternative = named.find(option => option.enabled && option.value !== curated?.value);
        if (!curated || !alternative) throw Error("The example picker has no selected curated example and distinct enabled named alternative");
        pickerRoundTrip.offered = named.map(({value,label}) => ({value,label}));
        const locateOption = option => options.and(page.locator(`[data-value=${JSON.stringify(option.value)}]`));
        for (const [stage,option,count] of [["selected",alternative,2],["restored",curated,3]]) {
          await locateOption(option).click({timeout:10000});
          await page.waitForFunction(([selector,text]) => document.querySelector(selector)?.textContent?.trim() === text,
            [pickerSelector,option.label], {timeout:pickerWaitMs});
          await requireArchive(count);
          pickerRoundTrip[stage] = option.label;
          pickerRoundTrip[`${stage}Value`] = option.value;
          if (paint) {
            paint = await waitForDraw(paint.keys, pickerWaitMs);
            pickerRoundTrip[`${stage}Draw`] = paint.draw;
          }
          await picker.click({timeout:10000});
          await locateOption(option).and(page.getByRole("option", {selected:true})).waitFor({timeout:10000});
        }
        await page.keyboard.press("Escape");
      }
    }
    const windows = await shell.locator('[data-slot="window-body"]').allInnerTexts();
    if (!windows.length) throw Error("The ready shell did not mount any app windows");
    if (witnessInitialArchive && archiveReceipts.length === 0) throw Error("The curated archive did not acknowledge its load");
    if (faults.size > 0 || failedRequests.size > 0) throw Error("The ready shell reported browser or guest failures");
    console.log(JSON.stringify({id,preview,bindings,worker,elapsedMs:Date.now()-started,cores:[...coreUrls],rendererStock:await Promise.all(stockResponses),operationCopyOverride,pickerRoundTrip,archiveReceipts,draw,...shellState,windows,faults:[...faults],failedRequests:[...failedRequests]}));
  } catch(error) {
    process.exitCode = 1;
    const pageState = await page?.locator('body').evaluate(element => ({
      status: Array.from(element.querySelectorAll('[role="status"], [role="progressbar"]'), row => row.textContent?.trim()),
      body: element.innerText.slice(0, 1200),
    }), undefined, {timeout: 2000}).catch(() => undefined);
    console.log(JSON.stringify({id,preview,bindings,worker,elapsedMs:Date.now()-started,cores:[...coreUrls],rendererStock:await Promise.all(stockResponses),operationCopyOverride,pickerRoundTrip,archiveReceipts,error:String(error),pageState,faults:[...faults],failedRequests:[...failedRequests]}));
  } finally {await browser?.close();}
}

let next = 0;
await Promise.all(Array.from({length: Math.min(ids.length, concurrency)}, async () => {
  while (next < ids.length) await probe(ids[next++]);
}));
