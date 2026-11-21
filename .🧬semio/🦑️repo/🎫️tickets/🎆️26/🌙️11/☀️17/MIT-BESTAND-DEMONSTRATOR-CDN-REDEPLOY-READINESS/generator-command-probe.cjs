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

function traceCommandTurns(source) {
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
  return pageSource.slice(0, end) + trace + pageSource.slice(end);
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

function traceShardWire(stopOnControlTurns) {
  const NativeWorker = window.Worker;
  const counts = { sent: 0, returned: 0, turns: 0, lastRequest: "", lastReply: "", lastStatus: "" };
  const currencies = new Map();
  const originalSources = new Map();
  const decoder = new TextDecoder();
  const pageMagic = "semio.typed-operation-page.v1\0";
  let controlWindows = 0;
  window.Worker = class extends NativeWorker {
    constructor(...args) {
      super(...args);
      this.addEventListener("message", ({ data }) => {
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
      counts.sent += 1;
      counts.lastRequest = `${data.kind}/${data.requestId ?? ""}/${data.events?.map(event => event.kind).join(",") ?? ""}`;
      return super.postMessage(data, ...args);
    }
  };
  setInterval(() => {
    const frequent = [...currencies].sort((a, b) => b[1] - a[1]).slice(0, 5);
    console.info("[DEBUG] generator shard wire", JSON.stringify({ ...counts, receiptCurrencies: frequent, originalSources: Object.fromEntries(originalSources) }));
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
  try {
    const locale = flags.find((flag) => flag.startsWith("--locale="))?.split("=")[1];
    const page = await browser.newPage({ locale });
    const profile = flags.find((flag) => flag.startsWith("--profile="))?.slice("--profile=".length);
    const profilingSession = profile ? await browser.newBrowserCDPSession() : null;
    if (flags.includes("--wire")) await page.addInitScript(traceShardWire, flags.includes("--stop-on-control-turns"));
    if (flags.includes("--diagnostics")) {
      await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
    }
    const reported = new Set();
    let instrumented = false;
    page.on("console", (message) => {
      const line = message.text();
      const known = /generator ingress|generator command (page|progress|handoff|frames)|generator reserved job|generator shell action|generator (completion|result page|shard wire|wire control-only)|generator contribution pages|setContributions command failed|Framework OS boot failed|exchange\.unapplied|poll semantic failure|FAILED with|cleanup job body unreachable|typed-operation (slots|.*advanced)|more-work.*typed_operation|maintenance|continuation|qualification/u.test(line);
      if ((!known && (message.type() !== "error" || line.includes("[DEBUG]"))) || reported.has(line)) return;
      reported.add(line);
      const prefix = flags.includes("--timestamps") ? `${new Date().toISOString()} ` : "";
      process.stdout.write(`${prefix}${line}\n`);
      const phase = line.includes("typed-operation setActiveExample advanced 64 units with no change") ? "store-stall"
        : line.includes("generator shell action apply-effects [] 128") && !flags.includes("--diagnostics") ? "publishing"
        : line.endsWith("generator wire control-only 1") ? "blocked" : null;
      if (profilingSession && phase) {
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
        if (flags.includes("--turn-status")) body = traceCommandTurns(body);
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
    await browser.close();
    await server?.close();
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
