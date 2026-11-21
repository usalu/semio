import { createRequire } from "node:module";
import { basename } from "node:path";
import { readFileSync, writeFileSync } from "node:fs";
import { pathToFileURL } from "node:url";

const require = createRequire(process.cwd() + "/package.json");
const { chromium } = require("playwright");
const [url = "http://127.0.0.1:6033/#din4108", seconds = "1080"] = process.argv.slice(2);
const transferOnly = process.env.TRANSFER_ONLY === "1";
const expectedCopyStock = Number(process.env.EXPECT_ORIGINAL_COPY_STOCK ?? 0);
const captureExample = process.env.TRACE_EXAMPLE_COMMAND === "1";
const verifyPicker = process.env.VERIFY_EXAMPLE_WORKFLOW === "1";
const omitNoteView = process.env.TRACE_NOTE_WITHOUT_VIEW === "1";
const began = Date.now();
const print = (message) => console.log(`[DEBUG] ${((Date.now() - began) / 1_000).toFixed(1)}s ${message}`);
const browser = await chromium.launch({
  args: [
    "--js-flags=--max-old-space-size=8192",
    "--enable-unsafe-swiftshader",
    "--in-process-gpu",
    "--disable-background-timer-throttling",
    "--disable-renderer-backgrounding",
    "--disable-backgrounding-occluded-windows",
    "--disable-features=IntensiveWakeUpThrottling",
  ],
});
const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
if (process.env.TRACE_MODEL_DRAW) await page.addInitScript(() => {
  const metrics = globalThis.__semioProbeModelDraw = { webgpu: 0, webgl: 0, canvas2d: 0 };
  for (const [backend, proto, methods] of [
    ["webgpu", globalThis.GPURenderPassEncoder?.prototype, ["draw", "drawIndexed", "drawIndirect", "drawIndexedIndirect"]],
    ["webgl", globalThis.WebGLRenderingContext?.prototype, ["drawArrays", "drawElements"]],
    ["webgl", globalThis.WebGL2RenderingContext?.prototype, ["drawArrays", "drawElements", "drawArraysInstanced", "drawElementsInstanced"]],
    ["canvas2d", globalThis.CanvasRenderingContext2D?.prototype, ["fill", "stroke", "drawImage"]],
  ]) {
    if (!proto) continue;
    for (const method of methods) {
      const original = proto[method];
      if (typeof original !== "function") continue;
      proto[method] = function (...args) {
        const result = Reflect.apply(original, this, args);
        metrics[backend]++;
        return result;
      };
    }
  }
});
const preview = process.env.PLAY_TEST_GUEST;
const previewJs = process.env.PLAY_TEST_GUEST_JS;
const previewImports = previewJs ? await import(pathToFileURL(`${process.cwd()}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🕸️imports/🟦️.ts`)) : null;
let previewFetched = false;
let previewJsFetched = false;
if (preview) {
  await page.route((requestUrl) => requestUrl.pathname.endsWith("/" + basename(preview)), async (route) => {
    const guestUrl = new URL(route.request().url());
    guestUrl.pathname = "/@fs" + preview;
    previewFetched = true;
    print(`loading preview guest ${preview}`);
    await route.continue({ url: guestUrl.href });
  });
}
let transferFinishedAt = null;
let acknowledgedTransfers = 0;
let settledNotes = 0;
let completedReservedJobs = 0;
let exampleRequested = false;
let fatal = null;
const notable = /play-archive|boot failed|semantic failure|authority failure|source-not-ready|shell\.documentTransfer\.load|archive (?:load deferred|await replacement|maintenance result missing)|document archive parent Pack and SPR hydration was rejected|close demand|cleanup job body unreachable|failed-decoded/i;
page.on("console", (message) => {
  const line = message.text();
  if (process.env.TRACE_NOTE_INPUT && line.startsWith("[DEBUG] play-archive shell note wire ")) {
    const [bytes] = JSON.parse(line.slice("[DEBUG] play-archive shell note wire ".length));
    writeFileSync(new URL("./🗑️generated/din4108-note.bin", import.meta.url), Uint8Array.from(bytes));
  }
  if (exampleRequested && line.startsWith("[DEBUG] play-archive example wire ")) {
    const [bytes] = JSON.parse(line.slice("[DEBUG] play-archive example wire ".length));
    writeFileSync(new URL("./🗑️generated/din4108-example.bin", import.meta.url), Uint8Array.from(bytes));
    print(`captured exact setActiveExample wire ${bytes.length} bytes`);
  }
  if (notable.test(line)) print(line.slice(0, process.env.TRACE_NOTE_INPUT && line.includes("play-archive shell note wire") ? 16_384 : 2_600));
  if (expectedCopyStock > 0 && line.includes("play-archive grant initial")) {
    const initialCopy = Number(line.match(/"maximumCopyBytes":(\d+)/)?.[1]);
    if (initialCopy !== expectedCopyStock) fatal ??= `expected original copy stock ${expectedCopyStock}, received ${initialCopy}`;
  }
  if (/boot failed|semantic failure|authority failure|source-not-ready|shell\.documentTransfer\.load-failed|cleanup job body unreachable/i.test(line)) fatal ??= line;
  if (line.includes("[DEBUG] play-archive acknowledged")) {
    acknowledgedTransfers++;
    transferFinishedAt = Date.now();
  }
  if (line.includes("[DEBUG] play-archive shell note reserved tool drives handed off")) settledNotes++;
  if (line.startsWith("[DEBUG] play-archive reserved job ") && line.endsWith("completion event returned")) completedReservedJobs++;
});
page.on("pageerror", (error) => {
  print(`page error: ${error}`);
  if (/original-turn|work.limit/i.test(String(error))) fatal ??= String(error);
});
page.on("crash", () => { fatal ??= "renderer crashed"; print("renderer crashed"); });

await page.route((requestUrl) => decodeURIComponent(requestUrl.pathname).includes("/🧾️app-channel/🧳️archive-transfer/"), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  const instrumented = source
    .replace('await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, progress);', `console.info("[DEBUG] play-archive begin");
      let lastStatus;
      await this.drive("loadDocumentArchive", new DocumentArchiveLoadHost(archive), [], signal, status => {
        const current = JSON.stringify([status.operation, status.state, status.completed, status.total]);
        if (current !== lastStatus) console.info("[DEBUG] play-archive status", current);
        lastStatus = current;
        progress?.(status);
      });
      console.info("[DEBUG] play-archive acknowledged");`);
  print(`archive observer attached: ${instrumented !== source}`);
  await route.fulfill({ response, body: instrumented });
});

const probeCopyStock = Number(process.env.TRACE_COPY_BYTES ?? (process.env.TRACE_DOUBLE_COPY === "1" ? 1_073_741_824 : 0));
if (probeCopyStock > 0) await page.context().route((requestUrl) => decodeURIComponent(requestUrl.pathname).endsWith("/🧵️renderer-work/🔣️stock.json"), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  const instrumented = source.replace(/("?maximumCopyBytes"?\s*:\s*)536870912\b/, (_, prefix) => `${prefix}${probeCopyStock}`);
  print(`copy admission diagnostic attached: ${instrumented !== source}`);
  await route.fulfill({ response, body: instrumented });
});

if (process.env.TRACE_PICKER_SETTLEMENT) await page.context().route((url) => decodeURIComponent(url.pathname).includes("/🧭️input-funnel/"), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  const instrumented = source
    .replace(/if \((action\d*)\.action === SET_ACTIVE_EXAMPLE_ACTION_ID && outcome.kind === "applied"\) \{/, `if ($1.action === SET_ACTIVE_EXAMPLE_ACTION_ID) console.info("[DEBUG] play-archive picker dispatch outcome", outcome.kind, outcome.inputSeq, outcome.reason, outcome.detail);
    $&`)
    .replace("if (!active || core.loadedPluginsRef.current.find", `console.info("[DEBUG] play-archive picker live owner", active, targetSession.instanceId, currentSession?.instanceId, core.loadedPluginsRef.current.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle === plugin);
      if (!active || core.loadedPluginsRef.current.find`)
    .replace("if (selection) {", `console.info("[DEBUG] play-archive picker selection", programHistoryKeyV1(targetSession), core.focusedExampleProgramRef.current, programHistoryKeyV1(liveFocus), focused, selection?.visibleExampleId, memory.get(programHistoryKeyV1(targetSession))?.inputSeq);
      if (selection) {`)
    .replace("await applyHostEffects(response.requestedEffects ?? [],", `if (action.action === "setActiveExample") console.info("[DEBUG] play-archive picker host effects start", startedOperation?.operationId, response.requestedEffects?.length);
    await applyHostEffects(response.requestedEffects ?? [],`)
    .replace(/;(\s+)const completion = await awaitOperationSettle\(targetSession.instanceId, startedOperation\?\.operation\);/g, `;\nif (action.action === "setActiveExample") console.info("[DEBUG] play-archive picker host effects returned", startedOperation?.operationId);
    const completion = await awaitOperationSettle(targetSession.instanceId, startedOperation?.operation);
    if (action.action === "setActiveExample") console.info("[DEBUG] play-archive picker wait returned", completion?.revision);`);
  if (instrumented !== source) print(`picker settlement observer attached: ${decodeURIComponent(new URL(route.request().url()).pathname).split("/").slice(-3).join("/")}`);
  await route.fulfill({ response, body: instrumented });
});

if (process.env.TRACE_COMMAND_INGRESS || verifyPicker) await page.context().route((url) => decodeURIComponent(url.pathname).includes("/🚚️handle-turn-drive/"), async (route) => {
  const response = await route.fetch();
  let source = await response.text();
  if (process.env.TRACE_JOB_BUDGET === "current") {
    const authored = readFileSync(`${process.cwd()}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`, "utf8");
    const find = (text, name) => text.indexOf(`const ${name}`);
    const start = find(source,"commitReservedToolJobWhileSerialized");
    const end = find(source,"commitReservedToolSpawnsWhileSerialized");
    const authoredStart = find(authored,"commitReservedToolJobWhileSerialized");
    const authoredEnd = find(authored,"commitReservedToolSpawnsWhileSerialized");
    if (start < 0 || end <= start || authoredStart < 0 || authoredEnd <= authoredStart) throw Error("Missing reserved job driver frontier");
    const ts = require("typescript");
    const fresh = ts.transpileModule(authored.slice(authoredStart, authoredEnd), {
      compilerOptions: {target:ts.ScriptTarget.ES2022, module:ts.ModuleKind.ESNext},
    }).outputText;
    source = source.slice(0,start) + fresh + source.slice(end);
  }
  const instrumented = source
    .replace("const inspected = inspectEncodedAppCommand(events);", `const inspected = inspectEncodedAppCommand(events);
      const traceNote = (...detail) => { if (inspected.actionId === "noteShellCommand") console.info("[DEBUG] play-archive shell note", ...detail); };
      traceNote("begin", inspected.seq);
      ${process.env.TRACE_NOTE_INPUT ? 'traceNote("wire", JSON.stringify(events.map(bytes => [...bytes])));' : ""}
      ${captureExample ? 'if (inspected.actionId === "setActiveExample") console.info("[DEBUG] play-archive example wire " + JSON.stringify(events.map(bytes => [...bytes])));' : ""}`)
    .replace(/command:\s*events\[commandIndex\]!?/, `command: ${omitNoteView ? `inspected.actionId === "noteShellCommand" ? windowlessNote(events[commandIndex]) : ` : ""}events[commandIndex]`)
    .replace(/const previousIngress = terminal;\s+await acceptTurn\(continued\);\s+terminal = foldIngress\(\);/, `const previousIngress = terminal;
            await acceptTurn(continued);
            terminal = foldIngress();
            if (continuations < 4 || continuations % 512 === 0 || terminal !== previousIngress) traceNote("continuation", continuations, terminal, wireTurnStatusTag(continued.status), continued.effects.length, continued.uiPatches.length, acknowledgements.length, terminal === "fault" ? commandIngressFaultDisplay(results.at(-1)?.commandIngress) : "");`)
    .replace("const settled = await settleAcknowledgedPluginTurns(actorId, results, acknowledgements,", `traceNote("ingress complete");
        const settled = await settleAcknowledgedPluginTurns(actorId, results, acknowledgements,`)
    .replace("await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);", `traceNote("settled", settled.effects.length);
        await commitReservedToolSpawnsWhileSerialized(instanceId, actorId, settled.effects);
        traceNote("reserved tool drives handed off");`);
  const reservedStart = instrumented.indexOf("const commitReservedToolJobWhileSerialized");
  const reservedEnd = instrumented.indexOf("const commitReservedToolSpawnsWhileSerialized", reservedStart);
  const jobObserved = process.env.TRACE_RESERVED_JOB && reservedStart >= 0 && reservedEnd > reservedStart ? (
    instrumented.slice(0, reservedStart)
    + instrumented.slice(reservedStart, reservedEnd)
      .replace("const key =", `const traceReserved = (...detail) => console.info("[DEBUG] play-archive reserved job", String(job), ...detail);
    const key =`)
      .replace("await shardClient.startJob(actorId, job, kind, input);", `traceReserved("start request", kind, input.byteLength);
      await shardClient.startJob(actorId, job, kind, input);
      traceReserved("start accepted");`)
      .replace("step += 1;", 'step += 1; traceReserved("step", step, last.status);')
      .replace(/const outcome = await shardClient\.stepJob\(actorId, job, jobStepBudget\);/, '$& traceReserved("step", outcome.status);')
      .replace("await deliverJobCompletionTurn(instanceId, actorId, job, outcome);", `traceReserved("completion event start", outcome.status);
      await deliverJobCompletionTurn(instanceId, actorId, job, outcome);
      traceReserved("completion event returned");`)
      .replace("} finally {", `} catch (error) {
      traceReserved("failure", String(error), error?.stack);
      throw error;
    } finally {`)
    + instrumented.slice(reservedEnd)
  ) : instrumented;
  const rewritten = omitNoteView ? `${jobObserved}\nfunction windowlessNote(bytes) {
    let at = 1;
    const next = () => { let value = 0, n = 1, byte; do { byte = bytes[at++]; value += (byte & 127) * n; n *= 128; } while (byte & 128); return value; };
    next();
    const commandLength = next();
    const end = at + commandLength;
    const output = new Uint8Array(end + 1);
    output.set(bytes.subarray(0, end));
    console.info("[DEBUG] play-archive shell note empty view wire", bytes.length, output.length);
    return output;
  }` : jobObserved;
  print(`command ingress observer attached: ${instrumented !== source}`);
  if (process.env.TRACE_RESERVED_JOB) print(`reserved job observer attached: ${jobObserved !== instrumented}`);
  await route.fulfill({ response, body: rewritten });
});

if (process.env.TRACE_JOB_BUDGET) await page.context().route((url) => decodeURIComponent(url.pathname).endsWith("/📕️norm/🌉️bridge.js"), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  let current = source;
  if (process.env.TRACE_JOB_BUDGET === "current") {
    const fresh = await import(pathToFileURL(`${process.cwd()}/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🟦️.ts`));
    const start = current.indexOf("class OriginalJobFunds {");
    const end = current.indexOf("class OriginalSelfIssuedTurns {", start);
    if (start < 0 || end <= start) throw Error("Missing original job budgets in browser bridge");
    current = current.slice(0,start) + fresh.OriginalJobFunds.toString() + "\n" + fresh.originalSourceJob.toString() + "\n" + current.slice(end);
  }
  let instrumented = current.replace('throw Object.assign(Error("original-job.call-refused"), { refusal: call.refusal });',
    '{ console.info("[DEBUG] play-archive reserved job budget refusal", JSON.stringify({refusal:call.refusal, epoch:String(input.epoch), deadline:String(input.work.absoluteDeadlineUs)})); throw Object.assign(Error("original-job.call-refused"), { refusal: call.refusal }); }');
  if (process.env.TRACE_JOB_BUDGET === "current") instrumented = instrumented
    .replace("if(refusal!==undefined)throw refusal;", 'if(refusal!==undefined){console.info("[DEBUG] play-archive reserved job budget refusal", JSON.stringify({refusal:refusal.refusal, epoch:String(turn.input.epoch)}));throw refusal;}')
    .replace("if(reply.admitted)return;", `if(pending < 4 || reply.admitted) console.info("[DEBUG] play-archive reserved job start receipt", String(job), pending, String(reply.input.epoch), source !== null, reply.admitted);
   if(reply.admitted)return;`)
    .replace("const chunk=optionValue(call.chunk);", `const chunk=optionValue(call.chunk);
if(chunk === undefined || chunk.outputClosed || call.spent.releasedBytes > 0n || optionValue(chunk.bytes) !== undefined) console.info("[DEBUG] play-archive reserved job output read", String(job), String(call.input.epoch), String(chunk?.offset ?? 0n), optionValue(chunk?.bytes)?.length ?? 0, String(call.spent.releasedBytes), chunk?.outputClosed ?? false);`)
    .replace("if(ack.acknowledged)break;", `if(pending < 4 || ack.acknowledged) console.info("[DEBUG] play-archive reserved job output receipt", String(job), pending, String(ack.input.epoch), String(held.length), String(ack.spent.releasedBytes), ack.acknowledged);
if(ack.acknowledged)break;`);
  if (process.env.TRACE_JOB_BUDGET === "preserve-deadline") instrumented = instrumented
    .replace("class OriginalJobFunds {", "class OriginalJobFunds {\n  #deadlineUs;")
    .replace("this.#input = { ...wit, work: { ...wit.work, absoluteDeadlineUs: wall < wit.work.absoluteDeadlineUs ? wall : wit.work.absoluteDeadlineUs } };",
      "this.#input = wit; this.#deadlineUs = wall < wit.work.absoluteDeadlineUs ? wall : wit.work.absoluteDeadlineUs;")
    .replace("deadlineUs: this.#input.work.absoluteDeadlineUs, cancelled:", "deadlineUs: this.#deadlineUs ?? this.#input.work.absoluteDeadlineUs, cancelled:")
    .replace("this.#clock() >= this.#input.work.absoluteDeadlineUs", "this.#clock() >= (this.#deadlineUs ?? this.#input.work.absoluteDeadlineUs)");
  print(`job budget diagnostic attached: ${instrumented !== source}`);
  await route.fulfill({ response, body: instrumented });
});

if (process.env.TRACE_GRANT) await page.context().route((requestUrl) => decodeURIComponent(requestUrl.pathname).endsWith("/📕️norm/🌉️bridge.js"), async (route) => {
  const response = await route.fetch();
  const source = await response.text();
  const watchedMessages = process.env.TRACE_GUEST_FAILURE_LITERALS ? JSON.parse(readFileSync(process.env.TRACE_GUEST_FAILURE_LITERALS, "utf8")) : [];
  const instrumented = source
    .replace("function receiveOriginal(call, workUnitsSpent) {", `const grantTrace = {};
  let grantSamples = 0;
  let priorGuestReferences;
  const watchedMessages = ${JSON.stringify(watchedMessages)};
  const baselineEpoch = ${Number(process.env.TRACE_GUEST_BASELINE_EPOCH ?? 0)};
  function guestMessageReferences() {
    const memory = globalThis.__semioArchiveMemory;
    if (!memory || !watchedMessages.length) return [];
    const bytes = new Uint8Array(memory.buffer);
    const initial = new TextDecoder("latin1").decode(bytes.subarray(1_048_576, 25_165_824));
    const addresses = new Map();
    for (const message of watchedMessages) {
      let found = initial.indexOf(message);
      while (found >= 0) {
        addresses.set(found + 1_048_576, message);
        found = initial.indexOf(message, found + 1);
      }
    }
    const words = new Uint32Array(memory.buffer);
    const refs = [];
    for (let i = 1; i < words.length - 1; i++) {
      const message = addresses.get(words[i]);
      if (message && words[i + 1] === message.length) refs.push([i * 4, message, words[i - 1]]);
    }
    return refs;
  }
  console.info("[DEBUG] play-archive grant initial", JSON.stringify({ remaining: originalSource.retainedRemaining, fuel: originalSource.fuelRemaining }));
  function receiveOriginal(call, workUnitsSpent, kind = "context-or-failure") {
    if (baselineEpoch && Number(call.input?.epoch) === baselineEpoch) {
      priorGuestReferences = new Set(guestMessageReferences().map(JSON.stringify));
      console.info("[DEBUG] play-archive guest reference baseline", priorGuestReferences.size);
    }
    const bytes = Number(call.spent?.copiedBytes ?? 0);
    const lane = grantTrace[kind] ??= { count: 0, bytes: 0, max: 0, work: 0 };
    lane.count += 1;
    lane.bytes += bytes;
    lane.max = Math.max(lane.max, bytes);
    lane.work += Number(workUnitsSpent);
    if (++grantSamples % 512 === 0) console.info("[DEBUG] play-archive grant", JSON.stringify({ trace: grantTrace, remaining: originalSource.retainedRemaining, fuel: originalSource.fuelRemaining }));`)
    .replace("receiveOriginal(call, call.workUnitsSpent);\n    if (optionValue(call.refusal)", 'receiveOriginal(call, call.workUnitsSpent, "input");\n    if (optionValue(call.refusal)')
    .replace("receiveOriginal(result.retainedReceipt, result.retainedReceipt.workUnitsSpent);", 'receiveOriginal(result.retainedReceipt, result.retainedReceipt.workUnitsSpent, `poll:${result.result?.tag ?? result.nextWake?.tag ?? result.nextWake ?? "unknown"}`);')
    .replace("result = await reactor.poll(slice);", `result = await reactor.poll(slice);
        if (grantSamples % 512 === 0) console.info("[DEBUG] play-archive slice", JSON.stringify({
          event: events[0]?.kind,
          offeredFuel: slice.fuel,
          originalFuel: originalSource.fuelRemaining,
          status: result.status,
          nextWake: result.nextWake,
          guestRemaining: result.retainedReceipt?.remaining,
          effects: result.effects?.length,
        }, (_, value) => typeof value === "bigint" ? value.toString() : value));`)
    .replace("const receipt = failure?.payload ?? failure;", `const receipt = failure?.payload ?? failure;
          const memory = globalThis.__semioArchiveMemory;
          if (memory) {
            const bytes = new Uint8Array(memory.buffer);
            const decoder = new TextDecoder();
            const messages = new Set();
            const candidates = /(?:items \\d+ copy \\d+ capacity \\d+ released \\d+; owner grant|runtime (?:live cleanup|close) faulted for instance \\d+|document archive [ -~]{8,140}: [A-Z][a-z]+)[ -~]{0,660}/g;
            for (let at = 0; at < bytes.length; at += 1_048_576) {
              const text = decoder.decode(bytes.subarray(at, at + 1_049_536));
              for (const match of text.matchAll(candidates)) messages.add(match[0]);
            }
            console.info("[DEBUG] play-archive guest memory", JSON.stringify([...messages].slice(0, 16)));
            const refs = guestMessageReferences().filter(value => !priorGuestReferences?.has(JSON.stringify(value)));
            if (refs.length) console.info("[DEBUG] play-archive guest references", JSON.stringify(refs.slice(-60)));
          }
          console.info("[DEBUG] play-archive poll refusal", JSON.stringify({ name: failure?.name, message: failure?.message, failure: receipt?.failure, spent: receipt?.spent, input: receipt?.input }, (_, value) => typeof value === "bigint" ? value.toString() : value));`);
  print(`grant observer attached: ${instrumented !== source}`);
  await route.fulfill({ response, body: instrumented });
});
if (previewJs || process.env.TRACE_GUEST_MEMORY) await page.context().route((requestUrl) => requestUrl.pathname.endsWith("/semio_hub_norm_component.js"), async (route) => {
  const response = await route.fetch();
  const source = previewImports
    ? previewImports.rewritePreview2ShimImportSource(readFileSync(previewJs, "utf8"), `../${previewImports.PREVIEW2_VENDOR_RELATIVE}/`)
    : await response.text();
  if (previewJs) {
    previewJsFetched = true;
    print(`loading preview bindings ${previewJs}`);
  }
  const instrumented = process.env.TRACE_GUEST_MEMORY ? source.replace("memory0 = exports1.memory;", "memory0 = exports1.memory; globalThis.__semioArchiveMemory = memory0;") : source;
  if (process.env.TRACE_GUEST_MEMORY) print(`guest memory observer attached: ${instrumented !== source}`);
  await route.fulfill({ response, body: instrumented });
});

try {
  await page.goto(url, { waitUntil: "commit", timeout: 120_000 });
  if (captureExample) {
    await page.getByText("Outliner", {exact: true}).first().waitFor({timeout: 120_000});
    const cancel = page.getByRole("button", {name: "Cancel command", exact: true}).first();
    await cancel.waitFor({timeout: 45_000});
    await cancel.click();
    const picker = page.locator('[id="playground.navbar.fixture"]');
    const prior = (await picker.innerText()).trim();
    await picker.click();
    const selected = page.getByRole("option").filter({hasNotText: prior}).first();
    print(`switching example from ${prior} to ${(await selected.innerText()).trim()}`);
    exampleRequested = true;
    const observedWire = page.waitForEvent("console", { predicate: message => message.text().startsWith("[DEBUG] play-archive example wire "), timeout: 15_000 });
    await selected.click();
    await observedWire;
    process.exitCode = transferFinishedAt === null ? 1 : 0;
  } else if (verifyPicker) {
    const timeout = Number(seconds) * 1_000;
    await page.getByText("✓ Complies", {exact: true}).first().waitFor({timeout});
    await page.getByText("Outliner", {exact: true}).first().waitFor({timeout});
    if (settledNotes === 0) await page.waitForEvent("console", {
      predicate: message => message.text().includes("[DEBUG] play-archive shell note reserved tool drives handed off"),
      timeout,
    });
    if (process.env.TRACE_RESERVED_JOB && completedReservedJobs === 0) {
      throw new Error("The shell note did not return a native reserved job completion");
    }
    const picker = page.locator('[id="playground.navbar.fixture"]');
    const previousTransfers = acknowledgedTransfers;
    await picker.click();
    await page.getByRole("option", {name: "Failing thin insulation", exact: true}).click();
    await page.getByText("✗ Does not comply", {exact: true}).first().waitFor({timeout});
    if (acknowledgedTransfers <= previousTransfers) await page.waitForEvent("console", {
      predicate: message => message.text().includes("[DEBUG] play-archive acknowledged"),
      timeout,
    });
    print(`picker immediately after acknowledgement: ${JSON.stringify((await picker.innerText()).trim())}`);
    await page.waitForFunction(() => document.getElementById("playground.navbar.fixture")?.innerText.trim() === "Failing thin insulation",
      null, { timeout: Math.min(timeout, 45_000) });
    await page.locator("canvas:visible").first().waitFor({timeout});
    const canvases = await page.locator("canvas").evaluateAll(nodes => nodes.filter(node => {
      const rect = node.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0;
    }).length);
    if (canvases === 0 || fatal || (preview && !previewFetched) || (previewJs && !previewJsFetched)) {
      throw new Error(fatal ?? "The failing example did not keep a visible model with the expected guest");
    }
    if (process.env.TRACE_MODEL_DRAW) print(`renderer draw calls: ${JSON.stringify(await page.evaluate(() => globalThis.__semioProbeModelDraw))}`);
    if (process.env.SHOT) await page.screenshot({ path: process.env.SHOT, timeout: 10_000 });
    print(`example workflow complete with ${acknowledgedTransfers} archive receipts, ${settledNotes} shell notes, ${completedReservedJobs} reserved job completions and ${canvases} visible canvases`);
  } else {
    const deadline = Date.now() + Number(seconds) * 1_000;
    let lastText;
    let modelAlive = false;
    let inputBlocked = false;
    while (Date.now() < deadline && !fatal) {
      const body = await page.locator("body").innerText({ timeout: 10_000 }).catch(() => "");
      modelAlive = body.includes("Compliant ETICS dwelling") && body.includes("Complies") && body.includes("Outliner");
      inputBlocked = body.includes("has not answered") && body.includes("Cancel command");
      const synopsis = JSON.stringify({ modelAlive, inputBlocked, text: body.slice(0, modelAlive ? 900 : 380) });
      if (synopsis !== lastText) print(synopsis);
      lastText = synopsis;
      if (transferFinishedAt !== null && (transferOnly || (modelAlive && !inputBlocked && Date.now() - transferFinishedAt >= 60_000))) break;
      await page.waitForTimeout(5_000);
    }
    if (process.env.SHOT) await page.screenshot({ path: process.env.SHOT, timeout: 10_000 }).catch((error) => print(`screenshot: ${error}`));
    if (fatal || (preview && !previewFetched) || (previewJs && !previewJsFetched) || transferFinishedAt === null || (!transferOnly && (!modelAlive || inputBlocked || Date.now() - transferFinishedAt < 60_000))) {
      throw new Error(fatal ?? `archive acknowledged=${transferFinishedAt !== null} model visible=${modelAlive} input blocked=${inputBlocked} preview core fetched=${previewFetched} preview bindings fetched=${previewJsFetched}`);
    }
    if (!transferOnly) {
      const canvases = await page.locator("canvas").evaluateAll(nodes => nodes.flatMap(canvas => {
        const rect = canvas.getBoundingClientRect();
        return rect.width && rect.height ? [[canvas.width, canvas.height]] : [];
      }));
      print(`visible canvas dimensions ${JSON.stringify(canvases)}`);
    }
    print(transferOnly ? "archive acknowledged" : "archive acknowledged, model visible and input available after 60 seconds");
  }
} finally {
  await browser.close();
}
