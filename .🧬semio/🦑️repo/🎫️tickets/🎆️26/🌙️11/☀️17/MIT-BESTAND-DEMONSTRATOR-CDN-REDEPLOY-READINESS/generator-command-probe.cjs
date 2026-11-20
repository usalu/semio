const { createRequire } = require("node:module");
const { resolve } = require("node:path");
const { chromium } = createRequire(resolve("package.json"))("playwright");

function traceCommandSequence(source) {
  const marker = /programId:(\w+),commandId:(\w+)\.actionId\},async\(\)=>\{/u.exec(source);
  if (!marker) throw new Error("command stall-watch marker not found");
  const offset = marker.index + marker[0].length;
  const suffix = source.slice(offset);
  const counter = /\b(\w+)\.eventSeq\+=1;/u.exec(suffix);
  if (!counter) throw new Error("command sequence counter not found");
  const end = offset + counter.index + counter[0].length;
  const trace = `console.info("[DEBUG] generator ingress",${marker[1]},${counter[1]}.eventSeq,${marker[2]}.actionId);`;
  return source.slice(0, end) + trace + source.slice(end);
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
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    if (flags.includes("--diagnostics")) {
      await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
    }
    const reported = new Set();
    let instrumented = false;
    page.on("console", (message) => {
      const line = message.text();
      const known = /generator ingress|generator contribution pages|setContributions command failed|Framework OS boot failed|exchange\.unapplied|poll semantic failure|FAILED with|cleanup job body unreachable|maintenance|continuation|qualification/u.test(line);
      if ((!known && (message.type() !== "error" || line.includes("[DEBUG]"))) || reported.has(line)) return;
      reported.add(line);
      process.stdout.write(`${line}\n`);
    });
    await page.route(/\/assets\/[^/]+\.js(?:\?|$)/u, async (route) => {
      const response = await route.fetch();
      const source = await response.text();
      if (!source.includes("command ingress plugin.command-page-stalled:")) {
        await route.fulfill({ response });
        return;
      }
      const body = traceCommandSequence(flags.includes("--page-run") ? sendContributionPages(source) : source);
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
      return { status: "pending" };
    });
    if (!instrumented) throw new Error("command turn drive was not intercepted");
    process.stdout.write(`[DEBUG] generator shell: ${JSON.stringify(state)}\n`);
  } finally {
    await browser.close();
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
