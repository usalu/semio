const { createRequire } = require("node:module");
const { createWriteStream } = require("node:fs");
const { createHash } = require("node:crypto");
const { once } = require("node:events");
const { resolve } = require("node:path");
const browserPath = process.argv.find((argument) => argument.startsWith("--browsers="))?.slice("--browsers=".length);
if (browserPath) process.env.PLAYWRIGHT_BROWSERS_PATH = browserPath;
const { chromium } = createRequire(resolve("package.json"))("playwright");

function traceCommandSequence(source) {
  const marker = /programId:([\w$]+),commandId:([\w$]+)\.actionId\},async\(\)=>\{/u.exec(source);
  if (!marker) throw new Error("command stall-watch marker not found");
  const offset = marker.index + marker[0].length;
  const suffix = source.slice(offset);
  const counter = /([\w$]+)\.eventSeq\+=1;/u.exec(suffix);
  if (!counter) throw new Error("command sequence counter not found");
  const end = offset + counter.index + counter[0].length;
  const trace = `console.info("[DEBUG] generator ingress",${marker[1]},${counter[1]}.eventSeq,${marker[2]}.actionId);`;
  return source.slice(0, end) + trace + source.slice(end);
}

function traceCommandTurns(source, traceHost) {
  const drive = /state\.eventSeq|command ingress plugin\.command-page-stalled:/u.exec(source);
  if (!drive) throw new Error("command turn drive marker not found");
  const page = /\(action=\$\{([\w$]+)\.actionId\?\?"none"\}\)\x60\);const ([\w$]+)=await .*?;if\(await ([\w$]+)\(\2\),([\w$]+)=([\w$]+)\(\),/u.exec(source.slice(drive.index));
  if (!page) throw new Error("command page response marker not found");
  const [rawPage, inspected, offered, accept, terminal, fold] = page;
  const pageEnd = drive.index + page.index + rawPage.length;
  const pageLog = `${inspected}.actionId&&console.info("[DEBUG] generator command page",${inspected}.actionId,${terminal},${offered}.status?.tag),`;
  const pageSource = source.slice(0, pageEnd) + pageLog + source.slice(pageEnd);
  const resumed = new RegExp(`let ([\\w$]+)=([\\w$]+)\\([^;]+\\),([\\w$]+)=0;for\\(;[\\w$]+!=="command-complete".{0,1500}?${RegExp.escape(accept)}\\(([\\w$]+)\\),${RegExp.escape(terminal)}=${RegExp.escape(fold)}\\(\\),.*?\\1=\\2\\(\\4\\.status\\)\\}`, "u").exec(pageSource.slice(pageEnd));
  if (!resumed) throw new Error("command continuation response marker not found");
  const [raw, status, , count, result] = resumed;
  const end = pageEnd + resumed.index + raw.length - 1;
  const trace = `,${inspected}.actionId&&(${count}%100===0||${terminal}==="command-complete"||${terminal}==="fault")&&console.info("[DEBUG] generator command progress",${inspected}.actionId,${count},${terminal},${status},${result}.effects.length,${result}.uiPatches.length)`;
  const traced = pageSource.slice(0, end) + trace + pageSource.slice(end);
  if (!traceHost) return traced;
  const phase = (name) => `${inspected}.actionId==="flowEvalTick"&&(globalThis.probeFlowHost=[${JSON.stringify(name)},${count},${terminal},${status},performance.now()]);`;
  const continuation = traced.slice(pageEnd, end).match(new RegExp(`;const ${result}=await [\\w$]+\\([\\w$]+,[\\w$]+,\\{activation:[\\w$]+\\}\\),[\\w$]+=${terminal};await ${RegExp.escape(accept)}\\(${result}\\),`, "u"));
  if (!continuation) throw new Error("command submit and accept markers not found");
  const boundary = continuation[0].replace(";const ", `;${phase("submit")}const `).replace(";await ", `;${phase("accept")}await `);
  const loop = `break;const `;
  const error = /catch\(([\w$]+)\)\{([\w$]+)\.push\(\{instanceId:([\w$]+),error:\1\}\)\}\},[\w$]+=new Set,[\w$]+=async/u.exec(traced.slice(end));
  if (!error || !traced.slice(pageEnd, end).includes(loop)) throw new Error("command continuation exit markers not found");
  const errorStart = end + error.index;
  const errorTrace = `console.info("[DEBUG] generator command queue error",String(${error[1]}),${error[1]}?.stack);`;
  const withError = traced.slice(0, errorStart) + error[0].replace(`catch(${error[1]}){`, `catch(${error[1]}){${errorTrace}`) + traced.slice(errorStart + error[0].length);
  const driveBody = withError.slice(pageEnd, end).replace(loop, `break;${phase("yield")}const `).replace(continuation[0], boundary);
  return withError.slice(0, pageEnd) + driveBody + withError.slice(end);
}

function traceCommandHandoff(source, acknowledgementsOnly) {
  const boundary = /const ([\w$]+)=await (\w+)\((\w+),(\w+),(\w+),(\w+)=>\w+\((\w+),\6\),(\w+),(\w+)\);return await (\w+)\(\7,\3,\1\.effects\),\1\}\),([\w$]+)\.lane,/u;
  const match = boundary.exec(source);
  if (!match) throw new Error("command settlement boundary not found");
  const [raw, result, settle, actor, results, acknowledgements, patch, instance, activation, call, commit, inspected] = match;
  const replacement = raw.replace(`const ${result}=await ${settle}`, `if(${inspected}.actionId)console.info("[DEBUG] generator command handoff",${inspected}.actionId,"settle-start",${results}.length,${acknowledgements}.length);const ${result}=await ${settle}`).replace(`return await ${commit}(${instance},${actor},${result}.effects),${result}`, `if(${inspected}.actionId)console.info("[DEBUG] generator command handoff",${inspected}.actionId,"settle-end",${result}.effects.map(x=>x.tag),${result}.status?.tag);await ${commit}(${instance},${actor},${result}.effects);if(${inspected}.actionId)console.info("[DEBUG] generator command handoff",${inspected}.actionId,"commit-end");return ${result}`);
  let patched = source.slice(0, match.index) + replacement + source.slice(match.index + raw.length);
  if (acknowledgementsOnly) {
    const entry = patched.indexOf(`function ${settle}(`);
    const option = /"Interactive",new Set,[\w$]+=>[^,]+,![01],/u.exec(patched.slice(entry, entry + 800));
    if (entry < 0 || !option) throw new Error("command operation drain flag not found");
    const offset = entry + option.index + option[0].length - 4;
    patched = patched.slice(0, offset) + ",!1," + patched.slice(offset + 4);
  }
  const reserved = /for\((\w+)\(\),await (\w+)\.startJob\((\w+),(\w+),(\w+),(\w+)\);(\w+)\(\);\)\{(const \w+=\w+\(\w+\);for\(let \w+=0;\w+<\w+&&\7\(\);\w+\+=1\)\{const (\w+)=await \2\.stepJob\(\3,\4,\w+\);)/u;
  const job = reserved.exec(patched);
  if (!job) throw new Error("serialized reserved job loop not found");
  const [jobRaw, begin, shard, owner, id, kind, input, live, step, outcome] = job;
  const jobStart = `console.info("[DEBUG] generator reserved job",${owner},String(${id}),"start");let probeJobSteps=0;for(${begin}(),await ${shard}.startJob(${owner},${id},${kind},${input});${live}();){`;
  const jobStep = `if(++probeJobSteps%250===0||${outcome}.status!=="running")console.info("[DEBUG] generator reserved job",${owner},String(${id}),probeJobSteps,${outcome}.status,${outcome}.progress?.byteLength??0);`;
  return patched.slice(0, job.index) + jobStart + step + jobStep + patched.slice(job.index + jobRaw.length);
}

function traceCommandFrames(source) {
  const decoder = /=>\s*"OperationCompleted"\s*in\s*([\w$]+)\(/u.exec(source)?.[1];
  const pack = /([\w$]+)\(new Uint8Array\([\w$]+\.Command\.command\)\),[\w$]+=[\w$]+\?\.address\?\.actionId/u.exec(source)?.[1];
  const frames = /,([\w$]+)\.push\(\{instanceId:([\w$]+),frames:([\w$]+)\}\),[\w$]+\([\w$]+\.status\)==="more-work"&&[\w$]+\(\2\)/u.exec(source);
  if (!decoder || !pack || !frames) throw new Error("command reply frame boundary not found");
  const [raw, , , queue] = frames;
  const trace = `,console.info("[DEBUG] generator command frames",JSON.stringify(${queue}.map(bytes=>{try { const frame=${decoder}(bytes),reply=frame.Invocation; return {kind:Object.keys(frame)[0], scope:reply?.ui_scope?.length,output:reply?.output?.length&&reply.output.length<2048?${pack}(new Uint8Array(reply.output)):undefined,seq:reply?.in_reply_to??frame.Done?.in_reply_to}; } catch (error) {return {decodeError:String(error)};}}),(key,value)=>typeof value==="bigint"?String(value):value))`;
  return source.slice(0, frames.index) + trace + raw + source.slice(frames.index + raw.length);
}

function tracePanelAction(source) {
  const entry = /async function [\w$]+\(\{sessionWorkRef:[\w$]+,[^}]+applyHostEffects:([\w$]+),[^}]+awaitOperationSettle:([\w$]+),[^}]+,action:([\w$]+),/u.exec(source);
  if (!entry) throw new Error("shell action marker not found");
  const [raw, effects, settle, action] = entry;
  const begin = entry.index + raw.length;
  const body = source.slice(begin, begin + 4500);
  const calls = new RegExp(`await ${RegExp.escape(effects)}\\(([\\w$]+)\\.requestedEffects\\?\\?\\[\\],.*?\\);const ([\\w$]+)=await ${RegExp.escape(settle)}\\(([\\w$]+)\\.instanceId,([\\w$]+)\\?\\.operation\\);`, "u").exec(body);
  if (!calls) throw new Error("shell host invocation boundaries not found");
  const [span, response, result, , operation] = calls;
  const offset = begin + calls.index;
  const check = `if(${action}.action==="setActiveExample")console.info("[DEBUG] generator shell action",`;
  const replacement = `${check}"apply-effects",JSON.stringify(${response}.requestedEffects?.map(effect=>Object.keys(effect)[0])??[]),String(${operation}?.operation));${span.replace(`const ${result}=await`, `${check}"effects-applied");const ${result}=await`)}${check}"operation-wait-return",String(${result}?.operation??"missing"));`;
  return source.slice(0, offset) + replacement + source.slice(offset + span.length);
}

function yieldOperationDrain(source) {
  const entry = source.indexOf("`operation-drain#");
  const offset = source.indexOf(',!0,', source.indexOf('"UserVisible",new Set,', entry));
  if (entry < 0 || offset < entry || offset - entry > 475) throw new Error("command background drain flag not found");
  return source.slice(0, offset) + ",!1," + source.slice(offset + 4);
}

function increaseOriginalItemGrant(source) {
  const operation = /maximumItems:8388608(?=,maximumCopyBytes:2147483648,maximumCapacityBytes:536870912)/gu;
  if ([...source.matchAll(operation)].length !== 1) throw new Error("renderer original item grant does not match the measured release");
  return source.replace(operation, "maximumItems:16777216");
}

function traceCompletionFrame(source) {
  const completion = /(case [\w$]+\.OperationCompleted:\{const ([\w$]+)=[^;]+;)(return\{OperationCompleted:)/u;
  const match = completion.exec(source);
  if (!match) throw new Error("operation completion decoder not found");
  return source.replace(completion, (_whole, begin, operation, end) => `${begin}console.info("[DEBUG] generator completion decoded",String(${operation}));${end}`);
}

function traceGuestTurn(source, file) {
  const trace = (phase, fields = "") => `globalThis.probeGuestTurn&&console.info("[DEBUG] generator guest turn",globalThis.probeGuestTurn,${JSON.stringify(phase)}${fields});`;
  const transforms = {
    "🟨️shard-worker.js": [
      ["leaveTurn = await enterActorTurn(actorId);", `if(actorId==="procedural#1"&&kind==="turn")globalThis.probeGuestTurn=Number(requestId.slice(1))>=10485?requestId:null;${trace("worker-enter", ",actorId")}leaveTurn = await enterActorTurn(actorId);${trace("worker-entered")}`],
      ["let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);", `let result = await actor.api.poll(admitted, msg.commandPage, undefined, msg.budget);${trace("bridge-return", ",result.status?.tag,result.commandIngress?.tag")}`],
      ["result = await actor.api.poll([], undefined, undefined, msg.budget);", `result = await actor.api.poll([], undefined, undefined, msg.budget);${trace("worker-repoll", ",drivePolls,result.status?.tag")}`],
      ['reply(requestId, result, timings, beat("turn-step"));', `${trace("worker-reply", ",timings.driveStopped")}reply(requestId, result, timings, beat("turn-step"));${trace("worker-replied")}`],
    ],
    "🌉️bridge.js": [
      ["await admitContext();\n      if (events.length", `${trace("bridge-enter")}await admitContext();\n      if (events.length`],
      ["try { call = await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn)); }", `try { ${trace("native-input-issued", ",String(turn.input.epoch)")}call = await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn));${trace("native-input-returned", ",call.ready")} }`],
      ["const turn = originalSource.issue();\n      hostShim.__checkoutOriginal(turn);", `const turn = originalSource.issue();${trace("native-turn-issued", ",String(turn.input.epoch),turn.fuel")}\n      hostShim.__checkoutOriginal(turn);`],
      ["result = await reactor.poll(slice);", `result = await reactor.poll(slice);${trace("native-turn-returned", ",result.status?.tag,optionValue(result.commandIngress)?.tag")}`],
      ["return { ...result, nextWake:", `${trace("bridge-returning")}return { ...result, nextWake:`],
    ],
    "🟨️.js": [
      ["pendingEffects.set(requestId, { resolve, reject });", `${trace("host-effect-await", ",effect,requestId")}pendingEffects.set(requestId, { resolve, reject });`],
      ["export function log(level, message) {", `export function log(level, message) { if(message.startsWith("[TRACE] turn phase")){${trace("native-phase", ",message")}}`],
    ],
    "semio_hub_procedural_component.js": [
      ["const started = await task.enter();", `${trace("jco-task-enter")}const started = await task.enter();${trace("jco-task-entered", ",started")}`],
      ["fn: () => reactor100Poll(ptr0),", `fn: () => { ${trace("jco-core-call")}if(globalThis.probeGuestTurn==="r10492")setTimeout(()=>{if(globalThis.probeGuestTurn==="r10492"){${trace("jco-worker-timer")}}},300);return reactor100Poll(ptr0); },`],
      ["task.endFirstSlice();", `${trace("jco-first-slice-returned", ",ret")}task.endFirstSlice();`],
      ['_debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });', `${trace("jco-driver-error", ",String(err),err?.stack")}_debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });`],
    ],
  };
  if (file.endsWith("_component.js")) {
    const from = source.indexOf("async function poll$1(arg0)");
    const to = source.indexOf("\nasync function ", from + 1);
    if (from < 0 || to < 0) throw new Error("component poll markers missing");
    const poll = source.slice(from, to);
    const instrumented = transforms[file].reduce((body, [before, after]) => {
      if (body.split(before).length !== 2) throw new Error("component poll phase marker is missing or ambiguous");
      return body.replace(before, () => after);
    }, poll);
    const pendingImport = "const result = await fn.apply(null, args);";
    const suspending = source.indexOf("function _suspendingImport(");
    const imported = source.indexOf(pendingImport, suspending);
    if (imported < suspending || imported > from) throw new Error("JCO suspending import marker missing");
    const capture = `const probeStack=globalThis.probeGuestTurn==="r10492"?new Error("guest suspending import").stack:undefined;if(probeStack)console.info("[DEBUG] generator guest turn",globalThis.probeGuestTurn,"jco-suspending-import",fn.name,probeStack);${pendingImport}if(probeStack)console.info("[DEBUG] generator guest turn",globalThis.probeGuestTurn,"jco-suspending-import-returned",fn.name);`;
    return source.slice(0, imported) + capture + source.slice(imported + pendingImport.length, from) + instrumented + source.slice(to);
  }
  for (const [before, after] of transforms[file] ?? []) {
    if (source.split(before).length !== 2) throw new Error(`guest phase marker is missing or ambiguous in ${file}`);
    source = source.replace(before, () => after);
  }
  return source;
}

function traceShardWire({ stopOnControlTurns, traceHost, traceHostAlways = false }) {
  const NativeWorker = window.Worker;
  const counts = { sent: 0, returned: 0, turns: 0, lastRequest: "", lastReply: "", lastStatus: "" };
  const currencies = new Map();
  const originalSources = new Map();
  const pending = new Map();
  const recent = [];
  const remember = (row) => {
    recent.push(row);
    if (recent.length > 16) recent.shift();
  };
  const decoder = new TextDecoder();
  const pageMagic = "semio.typed-operation-page.v1\0";
  let controlWindows = 0;
  const wire = value => {
    const val = value?.val ?? value;
    if (!Array.isArray(val) && !ArrayBuffer.isView(val)) return null;
    const bytes = Uint8Array.from(val);
    return {
      bytes: bytes.length,
      prefix: Array.from(bytes.subarray(0, 32)),
      base64: bytes.length <= 2304 ? btoa(String.fromCharCode(...bytes)) : undefined,
    };
  };
  window.Worker = class extends NativeWorker {
    constructor(...args) {
      super(...args);
      this.addEventListener("message", ({ data }) => {
        if (traceHost && data.requestId) {
          const sent = pending.get(data.requestId);
          pending.delete(data.requestId);
          remember(["receive", data.requestId, data.kind, data.ok, data.value?.status?.tag, data.value?.commandIngress?.tag, data.timings?.driveStopped, data.ok === false ? String(data.error) : undefined, sent?.actor]);
        }
        if (traceHost && ["worker-fault", "trap", "original-producer-held"].includes(data.kind)) {
          remember([data.kind, data.actorId, data.phase, String(data.message ?? data.refusal)]);
        }
        if (traceHost && globalThis.probeFlowHost && data.kind === "frame" && !data.requestId) {
          remember(["worker-frame", data.actorId, data.frame?.kind, data.frame?.requestId]);
        }
        counts.returned += 1;
        counts.lastReply = `${data.kind}/${data.requestId ?? ""}`;
        if (data.kind === "original-turn-receipt") {
          const receipt = data.receipt;
          const spent = receipt?.spent;
          if (spent) {
            const grant = receipt.input.grant;
            const operation = String(receipt.input.operation);
            const previous = originalSources.get(operation);
            const nativeBudget = receipt.nativeReceipt?.input.work?.fuel;
            originalSources.set(operation, {
              actor: data.actorId,
              epoch: String(receipt.input.epoch),
              items: grant.maximumItems - spent.copiedItems,
              copy: grant.maximumCopyBytes - spent.copiedBytes,
              capacity: grant.maximumCapacityBytes - spent.retainedCapacityBytes,
              release: grant.maximumReleaseBytes - spent.releasedBytes,
              work: (previous?.work ?? 0) + receipt.workUnitsSpent,
              fuel: nativeBudget === undefined ? null : Number(nativeBudget) - receipt.workUnitsSpent,
            });
            const key = `${spent.copiedItems}/${spent.copiedBytes}/${spent.retainedCapacityBytes}/${spent.releasedBytes}/${receipt.workUnitsSpent}`;
            currencies.set(key, (currencies.get(key) ?? 0) + 1);
          }
        }
        for (const turn of [data, data.value, data.turn, data.value?.turn, data.result, data.result?.turn]) {
          if (!turn?.effects) continue;
          counts.turns += 1;
          counts.lastStatus = turn.status?.tag ?? String(turn.status ?? "");
          for (const effect of turn.effects) {
            if (traceHostAlways && effect.tag === "respond") {
              const reply = effect.val ?? effect;
              console.info("[DEBUG] generator extension wire", JSON.stringify({
                direction: "actor-effect", actor: data.actorId ?? pending.get(data.requestId)?.actor, kind: effect.tag, id: String(reply.req ?? reply.id ?? ""),
                requestId: data.requestId, tag: reply.outcome?.tag ?? reply.result?.tag, return: wire(reply.outcome ?? reply.result),
              }));
            }
            if (effect.tag !== "send-message" || effect.val?.target?.tag !== "shell") continue;
            const payload = effect.val.payload;
            if (payload?.[0] === 25) console.info("[DEBUG] generator completion raw", JSON.stringify(Array.from(payload.slice(0, 12))));
            if (payload?.[0] === 115 && payload.length >= pageMagic.length + 30 &&
                decoder.decode(Uint8Array.from(payload.slice(0, pageMagic.length))) === pageMagic) {
              const head = Uint8Array.from(payload.slice(pageMagic.length, pageMagic.length + 30));
              const token = new DataView(head.buffer);
              const operation = String(token.getBigUint64(4, true));
              const lane = token.getUint8(25);
              console.info("[DEBUG] generator result page raw", JSON.stringify({
                receiver: token.getUint32(0, true),
                operation,
                generation: String(token.getBigUint64(12, true)),
                sequence: token.getUint32(20, true),
                attempt: token.getUint8(24),
                lane,
                bodyBytes: token.getUint32(26, true),
              }));
              if (lane === 11) console.info("[DEBUG] generator result page fault", operation);
            }
          }
        }
      });
    }

    postMessage(data, ...args) {
      if (traceHost && data.requestId) {
        pending.set(data.requestId, { kind: data.kind, actor: data.actorId, started: performance.now() });
        remember(["send", data.requestId, data.kind, data.actorId]);
      }
      if (traceHost && data.kind === "original-root-refusal") remember(["root-refusal", data.actorId, String(data.refusal)]);
      if (traceHost && globalThis.probeFlowHost && !data.requestId && data.actorId) {
        remember(["host-frame", data.actorId, data.kind, data.frame?.kind]);
      }
      if (traceHostAlways && Array.isArray(data.events)) {
        for (const event of data.events) {
          const body = event.payload ?? event.val ?? event;
          if (event.kind === "completed" || event.kind === "http-chunk" ||
              (event.kind === "request" && String(data.actorId).includes("flow-extension"))) {
            console.info("[DEBUG] generator extension wire", JSON.stringify({
              direction: "host-event", actor: data.actorId, kind: event.kind, id: String(body.req ?? body.id ?? ""),
              requestId: data.requestId, tag: body.outcome?.tag ?? body.result?.tag,
              return: wire(body.outcome ?? body.result ?? body.params?.bytes ?? body.payload),
            }));
          }
        }
      }
      counts.sent += 1;
      counts.lastRequest = `${data.kind}/${data.requestId ?? ""}/${data.events?.map(event => event.kind).join(",") ?? ""}`;
      return super.postMessage(data, ...args);
    }
  };
  setInterval(() => {
    const frequent = [...currencies].sort((a, b) => b[1] - a[1]).slice(0, 5);
    console.info("[DEBUG] generator shard wire", JSON.stringify({ ...counts, receiptCurrencies: frequent, originalSources: Object.fromEntries(originalSources) }));
    if (traceHost && (traceHostAlways || globalThis.probeFlowHost)) {
      console.info("[DEBUG] generator host tail", JSON.stringify({ flow: globalThis.probeFlowHost, recent, pending: Object.fromEntries(pending) }));
    }
    const control = currencies.size === 2 && currencies.has("1/89/89/0/0") && currencies.has("0/0/0/0/10");
    controlWindows = control ? controlWindows + 1 : 0;
    if (stopOnControlTurns && controlWindows <= 3 && controlWindows > 0) console.info("[DEBUG] generator wire control-only", controlWindows);
    currencies.clear();
  }, 5000);
}

async function sampleWorker(session, path) {
  await session.send("Tracing.start", {
    categories: "disabled-by-default-v8.cpu_profiler",
    transferMode: "ReturnAsStream",
  });
  await new Promise((complete) => setTimeout(complete, 1200));
  const completed = new Promise((complete) => session.once("Tracing.tracingComplete", complete));
  await session.send("Tracing.end");
  const { stream } = await completed;
  const output = createWriteStream(path);
  for (;;) {
    const { data, base64Encoded, eof } = await session.send("IO.read", { handle: stream });
    if (!output.write(base64Encoded ? Buffer.from(data, "base64") : data)) await once(output, "drain");
    if (eof) break;
  }
  output.end();
  await once(output, "finish");
  await session.send("IO.close", { handle: stream });
  process.stdout.write(`[DEBUG] generator profile stored ${path}\n`);
}

async function pauseWorkerTargets(session) {
  const { targetInfos } = await session.send("Target.getTargets");
  const shardTargets = targetInfos.filter(({ type, url }) =>
    type === "worker" && url.includes("shard-worker.js"));
  process.stdout.write(`[DEBUG] generator worker targets ${JSON.stringify(shardTargets)}\n`);
  await Promise.allSettled(shardTargets.map(async ({ targetId }) => {
    const { sessionId } = await session.send("Target.attachToTarget", { targetId, flatten: false });
    try {
      let nextId = 0;
      const dispatch = (method, params) => session.send("Target.sendMessageToTarget", {
        sessionId, message: JSON.stringify({ id: ++nextId, method, params }),
      });
      const scripts = new Map();
      const receive = ({ sessionId: source, message }) => {
        if (source !== sessionId) return;
        const reply = JSON.parse(message);
        if (reply.method === "Debugger.scriptParsed") scripts.set(reply.params.scriptId, reply.params.url);
        if (reply.method === "Debugger.paused") {
          const frames = reply.params.callFrames.map(({ functionName, location, url }) => ({ functionName, ...location, url: url || scripts.get(location.scriptId) }));
          process.stdout.write(`[DEBUG] generator worker pause ${targetId} ${JSON.stringify(frames)}\n`);
          void dispatch("Debugger.resume");
        } else if (reply.result?.result) {
          process.stdout.write(`[DEBUG] generator worker evaluation ${targetId} ${JSON.stringify(reply.result.result)}\n`);
        } else if (reply.error) {
          process.stdout.write(`[DEBUG] generator worker debugger error ${targetId} ${JSON.stringify(reply.error)}\n`);
        }
      };
      session.on("Target.receivedMessageFromTarget", receive);
      try {
        await dispatch("Runtime.evaluate", { expression: "globalThis.probeGuestTurn", returnByValue: true });
        await dispatch("Debugger.enable");
        await dispatch("Debugger.pause");
        await dispatch("Runtime.evaluate", { expression: "globalThis.probeGuestTurn", returnByValue: true });
        await new Promise((resolve) => setTimeout(resolve, 1000));
      } finally {
        session.off("Target.receivedMessageFromTarget", receive);
      }
    } finally {
      await session.send("Target.detachFromTarget", { sessionId });
    }
  })).then((outcomes) => {
    for (const outcome of outcomes) if (outcome.status === "rejected") process.stdout.write(`[DEBUG] generator worker debugger error ${String(outcome.reason)}\n`);
  });
}

async function watchWasmRefusal(browser, breakpoint) {
  const [rawLength, ...offsets] = breakpoint.split(",").map(Number);
  const session = await browser.newBrowserCDPSession();
  const targets = new Map();
  session.on("Target.receivedMessageFromTarget", ({ sessionId, message }) => {
    const worker = targets.get(sessionId);
    if (!worker) return;
    const reply = JSON.parse(message);
    if (reply.method === "Debugger.scriptParsed") {
      const { scriptId, scriptLanguage, length, url } = reply.params;
      if (scriptLanguage === "WebAssembly") {
        process.stdout.write(`[DEBUG] generator Wasm script ${worker.targetId} ${length} ${url}\n`);
      }
      if (scriptLanguage === "WebAssembly" && length === rawLength) {
        worker.scriptId = scriptId;
        for (const columnNumber of offsets) {
          void worker.dispatch("Debugger.setBreakpoint", { location: { scriptId, lineNumber: 0, columnNumber } });
        }
      }
    } else if (reply.method === "Debugger.paused") {
      const frames = reply.params.callFrames.map(({ functionName, location, url }) => ({ functionName, ...location, url }));
      process.stdout.write(`[DEBUG] generator refusal Wasm stack ${worker.targetId} ${JSON.stringify(frames)}\n`);
      void worker.dispatch("Debugger.resume");
    } else if (reply.result?.actualLocation || reply.error) {
      process.stdout.write(`[DEBUG] generator refusal breakpoint ${JSON.stringify(reply.result?.actualLocation ?? reply.error)}\n`);
    }
  });
  const attached = new Set();
  let scanning = false;
  const timer = setInterval(async () => {
    if (scanning) return;
    scanning = true;
    try {
      const { targetInfos } = await session.send("Target.getTargets");
      for (const { type, url, targetId } of targetInfos) {
        if (type !== "worker" || !url.includes("shard-worker.js") || attached.has(targetId)) continue;
        attached.add(targetId);
        const { sessionId } = await session.send("Target.attachToTarget", { targetId, flatten: false });
        let id = 0;
        const dispatch = (method, params) => session.send("Target.sendMessageToTarget", { sessionId, message: JSON.stringify({ id: ++id, method, params }) });
        targets.set(sessionId, { targetId, dispatch });
        await dispatch("Debugger.enable");
      }
    } catch (error) {
      process.stderr.write(`[DEBUG] generator Wasm refusal debugger ${error.message}\n`);
    } finally {
      scanning = false;
    }
  }, 400);
  return () => clearInterval(timer);
}

function sendContributionPages(source) {
  const marker = /const (\w+)=(\w+)\?\{json:(\w+),page:0,pageCount:1\}:\{json:\3\};try\{/u.exec(source);
  if (!marker) throw new Error("whole contribution pack marker not found");
  const [call, args, takesPageRun, json] = marker;
  const after = marker.index + call.length;
  const close = source.indexOf("}catch(", after);
  if (close < 0) throw new Error("contribution command catcher not found");
  const split = `const probePages=${takesPageRun}?(()=>{const output=[],encoder=new TextEncoder(),scratch=new Uint8Array(49152);for(let offset=0;offset<${json}.length;){const {read}=encoder.encodeInto(${json}.slice(offset),scratch);output.push(${json}.slice(offset,offset+read));offset+=read}if(output.length===0)output.push("");console.info("[DEBUG] generator contribution pages",output.length);return output.map((json,page)=>({json,page,pageCount:output.length}))})():[{json:${json}}];try{for(const ${args} of probePages){`;
  return source.slice(0, marker.index) + split + source.slice(after, close) + "}" + source.slice(close);
}

async function main() {
  const [url, timeout = "45000", ...flags] = process.argv.slice(2);
  const launch = { args: flags.includes("--liftoff-only") ? ["--js-flags=--liftoff-only"] : [] };
  const server = flags.includes("--browser-pid") ? await chromium.launchServer(launch) : undefined;
  if (server) process.stdout.write(`[DEBUG] generator Chromium PID ${server.process().pid}\n`);
  const browser = server ? await chromium.connect(server.wsEndpoint()) : await chromium.launch(launch);
  let stopWasmWatch;
  try {
    const locale = flags.find((flag) => flag.startsWith("--locale="))?.split("=")[1];
    const breakpoint = flags.find((flag) => flag.startsWith("--wasm-breakpoint="))?.split("=")[1];
    if (breakpoint) stopWasmWatch = await watchWasmRefusal(browser, breakpoint);
    const page = await browser.newPage({ locale });
    const profile = flags.find((flag) => flag.startsWith("--profile="))?.slice("--profile=".length);
    const profilingSession = profile ? await browser.newBrowserCDPSession() : null;
    if (flags.includes("--wire")) await page.addInitScript(traceShardWire, { stopOnControlTurns: flags.includes("--stop-on-control-turns"), traceHost: flags.includes("--trace-host") });
    if (flags.includes("--diagnostics")) {
      await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
    }
    if (flags.includes("--trace-guest")) {
      await page.context().route((url) => {
        const path = decodeURIComponent(url.pathname);
        return path.endsWith("/🧵️shard/🟨️shard-worker.js") || /\/🌀️procedural\/(🌉️bridge|🟨️|semio_hub_procedural_component)\.js$/u.test(path);
      }, async (route) => {
        const response = await route.fetch();
        const source = await response.text();
        const file = decodeURIComponent(new URL(route.request().url()).pathname).split("/").at(-1);
        process.stdout.write(`[DEBUG] generator guest turn asset SHA-256 ${createHash("sha256").update(source).digest("hex")} ${file}\n`);
        await route.fulfill({ response, body: traceGuestTurn(source, file) });
      });
    }
    const reported = new Set();
    let instrumented = false;
    page.on("console", (message) => {
      const line = message.text();
      const known = /generator ingress|generator command (page|progress|handoff|frames|queue error)|generator reserved job|generator shell action|generator (completion|result page|shard wire|wire control-only|host tail|guest turn)|generator contribution pages|setContributions command failed|Framework OS boot failed|exchange\.unapplied|poll semantic failure|FAILED with|cleanup job body unreachable|typed-operation (slots|.*advanced)|Document replacement|more-work.*typed_operation|maintenance|continuation|qualification/u.test(line);
      if ((!known && (message.type() !== "error" || line.includes("[DEBUG]"))) || reported.has(line)) return;
      reported.add(line);
      const prefix = flags.includes("--timestamps") ? `${new Date().toISOString()} ` : "";
      process.stdout.write(`${prefix}${line}\n`);
      const phase = line.includes("typed-operation setActiveExample advanced 64 units with no change") ? "store-stall"
        : line.includes("generator shell action apply-effects [] 128") && !flags.includes("--diagnostics") ? "publishing"
        : line.includes("Document replacement") && line.includes("empty turns 8") ? "document-replacement"
        : line.endsWith("generator wire control-only 1") ? "blocked" : null;
      if (profilingSession && flags.includes("--pause-guest") && line.endsWith("r10492 jco-core-call")) {
        void (async () => {
          for (let sample = 1; sample <= 2; sample += 1) {
            process.stdout.write(`[DEBUG] generator unreplied poll sample ${sample}\n`);
            await Promise.allSettled([
              pauseWorkerTargets(profilingSession),
              sampleWorker(profilingSession, `${profile}-unreplied-poll-${sample}.json`),
            ]).then((outcomes) => {
              for (const outcome of outcomes) if (outcome.status === "rejected") process.stderr.write(`[DEBUG] generator profile error ${String(outcome.reason)}\n`);
            });
          }
        })();
      }
      if (profilingSession && !flags.includes("--pause-guest") && phase && !reported.has(phase)) {
        reported.add(phase);
        sampleWorker(profilingSession, `${profile}-${phase}.json`).catch((error) => process.stderr.write(`[DEBUG] generator profile error ${error.message}\n`));
      }
    });
    page.on("pageerror", (error) => process.stdout.write(`[DEBUG] generator page error: ${error.stack}\n`));
    await page.route(/\/assets\/[^/]+\.js(?:\?|$)/u, async (route) => {
      const response = await route.fetch();
      const source = await response.text();
      if (!source.includes("command ingress plugin.command-page-stalled:")) {
        await route.fulfill({ response });
        return;
      }
      process.stdout.write(`[DEBUG] generator renderer SHA-256 ${createHash("sha256").update(source).digest("hex")} ${route.request().url()}\n`);
      let body = source;
      try {
        body = traceCommandSequence(flags.includes("--page-run") ? sendContributionPages(source) : source);
        if (flags.includes("--turn-status")) body = traceCommandTurns(body, flags.includes("--trace-host"));
        if (flags.includes("--handoff-status")) body = traceCommandHandoff(body, flags.includes("--acknowledgements-only"));
        if (flags.includes("--frames")) body = traceCommandFrames(body);
        if (flags.includes("--shell-action")) body = tracePanelAction(body);
        if (flags.includes("--yield-drain")) body = yieldOperationDrain(body);
        if (flags.includes("--wire")) body = traceCompletionFrame(body);
        if (flags.includes("--double-item-grant")) body = increaseOriginalItemGrant(body);
      } catch (error) {
        if (flags.includes("--page-run")) throw error;
        process.stdout.write(`[DEBUG] generator command sequence marker unavailable ${error.message}\n`);
        await route.fulfill({ response });
        return;
      }
      instrumented = true;
      await route.fulfill({ response, body });
    });
    await page.goto(url, { waitUntil: "domcontentloaded" });
    const state = await page.waitForFunction(() => {
      const shell = document.querySelector('[data-shell-id="generator"]');
      const status = ["ready", "error", "not-found"].find((value) => shell?.hasAttribute(`data-shell-${value}`));
      return status ? { status, text: shell.textContent?.slice(0, 500) } : false;
    }, undefined, { timeout: Number(timeout) }).then((handle) => handle.jsonValue(), (error) => {
      if (error.name !== "TimeoutError") throw error;
      return page.evaluate(() => ({
        status: "pending",
        locale: document.documentElement.lang,
        text: document.querySelector('[data-shell-id="generator"]')?.textContent?.trim().slice(0, 500),
      }));
    });
    if (!instrumented && flags.includes("--page-run")) throw new Error("command turn drive was not intercepted");
    process.stdout.write(`[DEBUG] generator shell: ${JSON.stringify(state)}\n`);
  } finally {
    stopWasmWatch?.();
    await browser.close();
    await server?.close();
  }
}

module.exports.traceShardWire = traceShardWire;

if (require.main === module) main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
