import { chromium } from "playwright";
import { mkdir, writeFile } from "node:fs/promises";
import { join } from "node:path";

const url = new URL(process.argv[2] ?? "http://127.0.0.1:6097");
const house = process.argv.includes("--house");
const switchExample = house || process.argv.includes("--switch");
const expected = house ? "House" : "Concrete Forest";
const exampleId = house ? "house" : "concrete-forest";
const hostTrace = process.argv.includes("--host-trace");
const witUiTrace = process.argv.includes("--wit-ui");
const watchBootstrap = process.argv.includes("--watch-bootstrap");
const houseObservationMs = witUiTrace ? 15_000 : 240_000;
const traceName = process.argv.find(value => value.startsWith("--trace-name="))?.slice("--trace-name=".length);
if (traceName !== undefined && !/^[a-z][a-z0-9-]*$/.test(traceName)) throw Error("Invalid trace name");
const output = join(import.meta.dirname, "..", "🗑️generated", traceName ? `${traceName}.txt` : hostTrace ? "browser-terminal-host.txt" : switchExample ? "browser-terminal-switch-geometry.txt" : "browser-terminal-ack.txt");
const marker = "[DEBUG] guest ";
const poll = "result = await reactor.poll(slice);";
const capture = `
        const __ackEvent = events.find(({kind,payload}) => kind === "message" && payload.source?.tag === "shell" &&
          String.fromCharCode(...Array.from(payload.payload).slice(0, 29)) === "semio.typed-operation-ack.v1\\0");
        const __probe = __ackEvent || commandPage;
        const __log = (kind, value) => console.info("[DEBUG] guest " + kind + " " + JSON.stringify(value, (_, part) =>
          typeof part === "bigint" ? String(part) : part instanceof Uint8Array ? [...part] : part));
        if (__ackEvent) __log("[DEBUG] command", {events:[__ackEvent]});
        if (__probe) __log("grant", {slice,events:__ackEvent ? ["message"] : [], command:commandPage?.cursor});
        if (commandPage) {
          const __body = new TextDecoder().decode(Uint8Array.from(commandPage.bytes));
          __log("wit-command", {epoch:slice.input.epoch,cursor:commandPage.cursor,example:__body.includes("setActiveExample") ? ["demo","concrete-forest","house"].find(id=>__body.includes(id)) ?? "unknown" : null});
        }
        try {
          result = await reactor.poll(slice);
        } catch (__failure) {
          __log("poll refusal", {payload:__failure?.payload ?? __failure});
          throw __failure;
        }
        const __effects = result.effects.filter(({tag,val}) => tag === "load-document" ||
          tag === "send-message" && val.target?.tag === "shell" &&
          String.fromCharCode(...Array.from(val.payload).slice(0, 30)) === "semio.typed-operation-page.v1\\0");
        if (__effects.length) __log("effects", {epoch:slice.input.epoch,effects:__effects.map(effect => effect.tag === "load-document" ? {tag:effect.tag} : effect)});
        if (${witUiTrace} && result.uiPatches?.length) {
          __log("wit-ui", {epoch:slice.input.epoch, patches:result.uiPatches.map(({surface,revision,baseRevision,ops})=>({
            surface,revision,baseRevision,ops:ops.reduce((counts,op)=>{
              counts[op.tag] = (counts[op.tag] ?? 0) + 1;
              return counts;
            },{})
          })),receipt:result.uiPatchReceipt});
        }
        if (__ackEvent) __log("ack-returned", {epoch:slice.input.epoch, payload:__ackEvent.payload.payload});
        if (commandPage || result.commandIngress.kind === 3 || result.commandIngress.kind === 4)
          __log("wit-command-result", {epoch:slice.input.epoch, state:result.commandIngress});
        if (${watchBootstrap} && (++__observedPolls % 1000 === 0 ||
          result.effects.some(effect => ["spawn-job", "cancel-job", "load-document"].includes(effect.tag)))) {
          __log("bootstrap-poll", {epoch:slice.input.epoch,status:result.status,ingress:result.commandIngress.kind,
            inputKinds:events.map(event=>event.kind),effects:result.effects.map(effect=>effect.tag),
            patches:result.uiPatches?.length ?? 0});
        }
        if (!__probe && __terminalAckObserved) {
          __log("grant", {slice,events:[]});
          __terminalAckObserved = false;
        }
        if (__ackEvent && new DataView(Uint8Array.from(__ackEvent.payload.payload).buffer).getUint32(49, true) === 2) __terminalAckObserved = true;
`;

const browser = await chromium.launch({ args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const entries = [];
const diagnostics = new Set();
let observedBridge = false;
let returnedAcks = 0;
let laterPoll = false;
let refused = false;
let switchClicked = false;
let switchTerminalEpoch;
await mkdir(join(output, ".."), { recursive: true });
if (switchExample) {
  await page.addInitScript(() => {
    const original = Worker.prototype.postMessage;
    Worker.prototype.postMessage = function(message, ...options) {
      const page = message?.commandPage;
      if (page?.bytes) {
        const text = new TextDecoder().decode(Uint8Array.from(page.bytes));
        console.info("[DEBUG] guest host-command " + JSON.stringify({kind:message.kind,cursor:page.cursor,
          example:text.includes("setActiveExample") ? ["demo","concrete-forest","house"].find(id=>text.includes(id)) ?? "unknown" : null},
          (_, value) => typeof value === "bigint" ? String(value) : value));
      }
      return Reflect.apply(original, this, [message, ...options]);
    };
  });
}
if (hostTrace) {
  const instrument = [
    {
      suffix: "/🧭️input-funnel/🟦️.ts",
      points: [
        ["const directOutcome =", "dispatch-enter", "{inputSeq:entry.provenance.inputSeq,direct:directBrowserActor!==null,target:{pluginId:targetSession.pluginId,instanceId:targetSession.instanceId,appId:targetSession.app.id}}"],
        ['if (__ACTION__.action === SET_ACTIVE_EXAMPLE_ACTION_ID && outcome.kind === "applied")', "dispatch-outcome", "{outcome,direct:directOutcome!==undefined}"],
        ["if (!active || core.loadedPluginsRef.current", "guard-active", "{active,samePlugin:core.loadedPluginsRef.current.find(entry=>entry.handle.pluginId===targetSession.pluginId)?.handle===plugin,currentInstance:currentSession?.instanceId}"],
        ["const memory = core.committedExamplesByProgramRef.current;", "guard-focused", "{focused,key:__KEY__,focusedExampleKey:core.focusedExampleProgramRef.current,liveFocusKey:programHistoryKeyV1(liveFocus) }"],
        ["if (selection) {", "picker-selection", "{selection,previous:memory.get(__KEY__)}"],
      ],
    },
    {
      suffix: "/🧭️input-funnel/🚀️program-dispatch/🟦️.ts",
      points: [
        ["const result = await dispatchDirectBrowserActorCommand(", "direct-start", "{}"],
        ["releaseSourceWork?.admit(void 0);", "direct-result", "{keys:Object.keys(result),commit:result.commit}"],
        ["const mailboxReason = browserActorActionRefusalReasonV1(actionError);", "direct-refusal", "{error:String(actionError)}"],
        ["const response = await invokeProgramInputV1(", "program-start", "{}"],
        ["releaseSourceWork?.admit(startedOperation?.operation);", "program-response", "{keys:Object.keys(response),started:startedOperation?.operation,output:response.output,effects:response.requestedEffects?.map(effect=>Object.keys(effect))}"],
        ["await applyHostEffects(response.requestedEffects", "program-effects-start", "{current:isCurrentEffectOwner(actionOwner)}"],
        ["const completion = await awaitOperationSettle", "program-before-settle", "{started:startedOperation?.operation}"],
        ["return programInputResultV1(applied(completion ===", "program-operation-settled", "{completion}"],
      ],
    },
    {
      suffix: "/🏛️ShellHost/🧩️shell-core-state/🟦️.ts",
      points: [
        ["if (navbarExample !== void 0) {", "history-picker", "{navbarExample,remembered,replace,key,upserts:patch.upserts?.filter(({actionId})=>actionId==='setActiveExample').map(({actionId,revertible,seq,applied})=>({actionId,revertible,seq,applied}))}"],
      ],
    },
  ];
  for (const { suffix, points } of instrument) {
    await page.context().route(requestUrl => requestUrl.origin === url.origin && decodeURIComponent(requestUrl.pathname).endsWith(suffix), async route => {
      const response = await route.fetch();
      let source = await response.text();
      const action = source.match(/const directOutcome = (\w+)\.commandAddress/)?.[1] ?? "action";
      const key = source.match(/const focused = (\w+) === core\.focusedExampleProgramRef/)?.[1] ?? "key";
      const substitute = (value) => value.replaceAll("__ACTION__", action).replaceAll("__KEY__", key);
      for (const [search, stage, value] of points) {
        const needle = substitute(search);
        const data = substitute(value);
        if (!source.includes(needle)) {
          console.log(`[DEBUG] unavailable host trace point ${stage}`);
          continue;
        }
        source = source.replaceAll(needle, `__femHostTrace(${stage === "history-picker" ? "null" : action},${JSON.stringify(stage)},()=>(${data}));\n${needle}`);
      }
      const prelude = `function __femHostTrace(action,stage,read) {
        if (action !== null && action.action !== "setActiveExample") return;
        try {
          const data = read();
          if (stage === "history-picker" && !data.upserts?.length) return;
          console.info("[DEBUG] guest host-dispatch " +
            JSON.stringify({stage,args:action?.args,...data},(_,v)=>typeof v==="bigint"?String(v):v));
        }
        catch(error) { console.info("[DEBUG] guest host-dispatch observer " + String(error)); }
      }\n`;
      await route.fulfill({ response, body: prelude + source });
    });
  }
}
await page.context().route(requestUrl => requestUrl.origin === url.origin && decodeURIComponent(requestUrl.pathname) === "/🔌️plugin-modules/🏗️fem/🌉️bridge.js", async route => {
  const response = await route.fetch();
  const source = await response.text();
  if (source.split(poll).length !== 2) throw Error("The FEM WIT poll call changed");
  observedBridge = true;
  await route.fulfill({ response, body: "let __terminalAckObserved = false; let __observedPolls = 0;\n" + source.replace(poll, capture) });
});
page.on("console", message => {
  const text = message.text();
  if (house && (message.type() === "error" || message.type() === "warning" || /schema|refusal/i.test(text.slice(0, 200))) && !text.startsWith(marker)) {
    const diagnostic = `${message.type()}: ${text.slice(0, 1000).replace(/token=[^'"& ]+/g, "token=<redacted>")}`;
    if (!diagnostics.has(diagnostic)) {
      diagnostics.add(diagnostic);
      entries.push(marker + "diagnostic " + diagnostic);
      console.log(marker + "diagnostic " + diagnostic);
    }
  }
  if (!text.startsWith(marker) && !(switchExample && /input #|Set Active Example|note shell command/i.test(text))) return;
  entries.push(text);
  if (hostTrace && text.startsWith(marker + "host-dispatch ")) console.log(text);
  if (watchBootstrap && (text.startsWith(marker + "bootstrap-poll ") || text.startsWith(marker + "ack-returned ") || text.startsWith(marker + "poll refusal "))) {
    console.log(text);
  }
  if (text.startsWith(marker + "ack-returned ")) {
    returnedAcks++;
    if (switchClicked) {
      const ack = JSON.parse(text.slice((marker + "ack-returned ").length));
      if (Buffer.from(ack.payload).readUInt32LE(49) === 2) switchTerminalEpoch = ack.epoch;
    }
  }
  if (text.startsWith(marker + "poll refusal ")) refused = true;
  if (text.startsWith(marker + "grant ") && returnedAcks >= 3) laterPoll = true;
});
page.on("pageerror", error => {
  if (!house) return;
  const diagnostic = String(error);
  diagnostics.add(diagnostic);
  entries.push(marker + "page-error " + diagnostic);
  console.log(marker + "page-error " + diagnostic);
});
page.on("response", response => {
  if (!house || response.status() < 400) return;
  const diagnostic = `HTTP ${response.status()} ${response.url()}`;
  diagnostics.add(diagnostic);
  entries.push(marker + "diagnostic " + diagnostic);
  console.log(marker + "diagnostic " + diagnostic);
});

async function sampleModel(picker, phase) {
  const model = await page.getByRole("tabpanel", { name: "Model", exact: true }).evaluate(panel =>
    [...panel.querySelectorAll(".semio-world-3d-host")].map(host => {
      function summarize(raw) {
        let digest = 2166136261;
        for (let index = 0; index < raw.length; index++) digest = Math.imul(digest ^ raw.charCodeAt(index), 16777619);
        try {
          return { count: Object.keys(JSON.parse(raw) ?? {}).length, chars: raw.length, digest: (digest >>> 0).toString(16) };
        } catch (error) {
          return { chars: raw.length, digest: (digest >>> 0).toString(16), parseError: String(error) };
        }
      }
      return { surface: host.dataset.surfaceId, canvas: host.querySelector("canvas") !== null,
        meshes: summarize(host.dataset.meshesJson || "[]"), instances: summarize(host.dataset.instancesJson || "[]") };
    }));
  const snapshot = { selected: (await picker.innerText()).trim(), model };
  if (phase) {
    entries.push(marker + "geometry " + JSON.stringify({ phase, epoch: switchTerminalEpoch, ...snapshot }));
    console.log(JSON.stringify({ phase, epoch: switchTerminalEpoch, ...snapshot }));
  }
  return snapshot;
}

try {
  await page.goto(url.href, { waitUntil: "domcontentloaded", timeout: 120_000 });
  const deadline = Date.now() + (watchBootstrap ? 180_000 : 420_000);
  let previousBootstrap = "";
  while (Date.now() < deadline && !refused && (!laterPoll || watchBootstrap)) {
    if (watchBootstrap) {
      const observed = await page.evaluate(() => ({
        status: [...document.querySelectorAll('[role="status"]')].map(node => node.getAttribute("aria-label") || node.textContent).filter(Boolean),
        results: document.querySelector('[role="tabpanel"][aria-label="Results"] p')?.textContent,
        canvases: document.querySelectorAll("canvas").length,
      }));
      const summary = JSON.stringify(observed);
      if (summary !== previousBootstrap) {
        previousBootstrap = summary;
        const line = marker + "bootstrap-visible " + summary;
        entries.push(line);
        console.log(line);
        await writeFile(output, entries.join("\n") + "\n", "utf8");
      }
    }
    await new Promise(resolve => setTimeout(resolve, watchBootstrap ? 1500 : 500));
  }
  if (!observedBridge || refused || returnedAcks < 3 || !laterPoll) {
    throw Error(`Terminal ACK not observed: bridge=${observedBridge}, refusal=${refused}, returned ACKs=${returnedAcks}, continued poll=${laterPoll}`);
  }
  console.log(`Observed ${returnedAcks} acknowledged result pages and a later WIT poll.`);
  if (switchExample) {
    const picker = page.getByRole("combobox", { name: "Example" });
    await picker.waitFor({ timeout: 60_000 });
    await page.waitForFunction(() => {
      const selected = document.querySelector('[role="combobox"]')?.textContent;
      const hasGeometry = [...document.querySelectorAll(".semio-world-3d-host")].some(host =>
        host.querySelector("canvas") && [host.dataset.meshesJson, host.dataset.instancesJson].some(raw => raw && Object.keys(JSON.parse(raw)).length));
      return selected?.includes("Demo") && hasGeometry;
    }, undefined, { timeout: 180_000 });
    entries.push(marker + "switch-ready " + JSON.stringify({ selected: await picker.innerText() }));
    let geometry = JSON.stringify(await sampleModel(picker, "before-click"));
    if (house) await page.screenshot({ path: output.replace(/\.txt$/, "-demo.png"), timeout: 30_000 });
    await picker.click();
    switchClicked = true;
    await page.getByRole("option", { name: expected, exact: true }).click({ timeout: 30_000 });
    entries.push(marker + "switch-click " + JSON.stringify({ selected: await picker.innerText() }));
    await writeFile(output, entries.join("\n") + "\n", "utf8");
    let deadline = Date.now() + 300_000;
    let labelAt;
    let savedEntries = entries.length;
    let terminalSampled = false;
    while (Date.now() < deadline && !refused) {
      if (!labelAt && (await picker.innerText()).trim() === expected) {
        if (!house) break;
        labelAt = Date.now();
        deadline = labelAt + houseObservationMs;
        await sampleModel(picker, "house-label");
        await page.screenshot({ path: output.replace(/\.txt$/, "-house-label.png"), timeout: 30_000 });
      }
      await new Promise(resolve => setTimeout(resolve, 1_000));
      if (switchTerminalEpoch && !terminalSampled) {
        geometry = JSON.stringify(await sampleModel(picker, "after-terminal-ack"));
        terminalSampled = true;
      } else {
        const current = JSON.stringify(await sampleModel(picker));
        if (current !== geometry) {
          await sampleModel(picker, "model-changed");
          geometry = current;
        }
      }
      if (entries.length !== savedEntries) {
        savedEntries = entries.length;
        await writeFile(output, entries.join("\n") + "\n", "utf8");
      }
    }
    const selected = (await picker.innerText()).trim();
    await sampleModel(picker, "switch-end");
    if (house) await page.screenshot({ path: output.replace(/\.txt$/, "-house-final.png"), timeout: 30_000 });
    const alerts = await page.getByRole("alert").allTextContents();
    const boundary = entries.filter(line => line.includes(`"example":"${exampleId}"`));
    const summary = { selected, alerts, boundary, ...(house ? { labelObservedMs: labelAt ? Date.now() - labelAt : null, diagnostics: [...diagnostics] } : {}) };
    entries.push(marker + "switch-end " + JSON.stringify(summary));
    console.log(JSON.stringify(summary));
  }
} finally {
  await writeFile(output, entries.join("\n") + "\n", "utf8");
  await browser.close();
}
