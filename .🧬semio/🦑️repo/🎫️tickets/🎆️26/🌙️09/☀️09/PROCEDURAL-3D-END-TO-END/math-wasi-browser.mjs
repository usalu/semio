import { chromium } from "playwright";
import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { readFile, readdir, stat, writeFile } from "node:fs/promises";
import { createServer, request as forward } from "node:http";
import { basename, join, resolve } from "node:path";

const upstream = new URL(process.env.GEOMETRY_PREVIEW_URL ?? "http://127.0.0.1:6018");
if (upstream.protocol !== "http:" || upstream.hostname !== "127.0.0.1") {
  throw new Error("The capability probe requires a loopback HTTP development host");
}
const assetPort = process.env.SEMIO_CAPABILITY_ASSETS_PORT;
if (assetPort !== undefined && !/^[1-9][0-9]{3,4}$/.test(assetPort)) {
  throw new Error("The Math asset host must be a loopback port");
}
const assetOrigin = assetPort === undefined ? upstream : new URL(`http://127.0.0.1:${assetPort}`);
const packageDir = process.env.SEMIO_PRIVATE_MATH && resolve(process.env.SEMIO_PRIVATE_MATH);
const root = resolve(".");
const cache = new Map();
const moduleStem = "semio_s_plugin_flow_math_component";
const exactNames = new RegExp(`^${moduleStem}\\.(?:js|core[0-9]*\\.wasm)$`);
for (const name of packageDir ? await readdir(packageDir) : []) {
  if (!exactNames.test(name)) continue;
  const path = join(packageDir, name);
  const digest = createHash("sha256");
  for await (const chunk of createReadStream(path)) digest.update(chunk);
  cache.set(name, { path, bytes: (await stat(path)).size, sha: digest.digest("hex") });
}
if (packageDir && (!cache.has(moduleStem + ".js") || !cache.has(moduleStem + ".core.wasm"))) {
  throw new Error("The private Math files are incomplete");
}
const prefix = "/🧩️extension-modules/🧮️flow-extension-math/";
const proxy = createServer((req, res) => {
  let target, pathname;
  try {
    target = new URL(req.url, upstream);
    pathname = decodeURIComponent(target.pathname);
    if (target.origin !== upstream.origin || !["GET", "HEAD"].includes(req.method)) throw new Error("Invalid probe request");
  } catch {
    res.writeHead(400);
    return res.end();
  }
  if (pathname === "/math-capability") {
    res.writeHead(200, { "content-type": "text/html" });
    return res.end("<!doctype html><title>Math capability verification</title>");
  }
  const name = basename(pathname);
  const artifact = pathname === prefix + name ? cache.get(name) : undefined;
  if (artifact) {
    console.log("[DEBUG] Private Math response", name, artifact.bytes, artifact.sha);
    res.writeHead(200, {
      "cache-control": "no-store",
      "content-length": artifact.bytes,
      "content-type": name.endsWith(".js") ? "text/javascript" : "application/wasm",
    });
    const stream = createReadStream(artifact.path);
    res.on("close", () => stream.destroy());
    return stream.on("error", error => res.destroy(error)).pipe(res);
  }
  const origin = pathname.startsWith(prefix) ? assetOrigin : upstream;
  const request = forward(new URL(target.pathname + target.search, origin), {
    method: req.method,
    headers: { ...req.headers, host: origin.host },
  }, response => {
    res.writeHead(response.statusCode, response.headers);
    res.on("close", () => response.destroy());
    response.on("error", error => res.destroy(error)).pipe(res);
  });
  request.on("error", () => {
    if (!res.headersSent) res.writeHead(502);
    res.end();
  });
  req.pipe(request);
});
await new Promise(resolve => proxy.listen(0, "127.0.0.1", resolve));
const url = `http://127.0.0.1:${proxy.address().port}`;
let server, timeout;
try {
  server = await chromium.launchServer({ host: "127.0.0.1", port: 0, headless: true, args: ["--renderer-process-limit=1"] });
  const browser = await chromium.connect(server.wsEndpoint());
  timeout = setTimeout(() => void server.kill(), Number(process.env.GEOMETRY_PROBE_MS ?? 600000));
  const page = await browser.newPage();
  page.on("console", event => {
    if (event.text().startsWith("[DEBUG]")) console.log(event.text());
  });
  page.on("pageerror", error => console.error("[DEBUG] Browser error", error.message));
  await page.goto(url + "/math-capability");
  const initialized = await page.evaluate(async ({ loader, packs }) => {
    window.$RefreshReg$ = () => {};
    window.$RefreshSig$ = () => value => value;
    window.__vite_plugin_react_preamble_installed__ = true;
    const [{ loadPluginModule }, { decodePackValue }] = await Promise.all([import(loader), import(packs)]);
    const extension = await loadPluginModule("flow-extension-math", "/🧩️extension-modules/🧮️flow-extension-math/🌉️bridge.js");
    window.__mathProbe = { extension, decodePackValue };
    return true;
  }, {
    loader: "/@fs/" + root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧩️module-loader/🟦️.ts",
    packs: "/@fs/" + root + "/🧰️framework/🔨️modules/🎁️packs/🟦️.ts",
  });
  if (initialized !== true) throw new Error("The Math worker did not initialize");

  const number = value => ({ $schema: "number", value });
  const evaluate = async (operatorId, nodeHash, input) => {
    const started = performance.now();
    let previous = -1;
    for (let round = 0; round < 320; round++) {
      const request = {
        neuronId: operatorId, operatorId, nodeHash, operatorVersion: "fixture",
        inputJson: round ? "" : JSON.stringify(input), dependencyJson: "",
        roundUnits: 256, resume: round > 0, windowId: "preview", windowKindId: "preview", extensionId: "math",
      };
      const entered = await page.evaluate(request => {
        const { extension, decodePackValue } = window.__mathProbe;
        window.__mathProbeReply = undefined;
        void (async () => {
          try {
            const bytes = await extension.invoke("evaluate", JSON.stringify(request));
            const raw = decodePackValue(bytes);
            const value = body => Number(typeof body === "object" ? body?.value : body);
            const output = raw.output;
            const reply = {
              phase: raw.phase,
              units: value(raw.unitsDone),
              done: Boolean(raw.done),
              kind: output?.sum?.$schema ?? output?.vectorOut?.$schema,
              sum: output?.sum && value(output.sum.value),
              coordinates: output?.vectorOut && ["x", "y", "z"].map(key => value(output.vectorOut[key])),
              initialWire: request.resume ? undefined : Array.from(bytes),
            };
            if (reply.done) {
              const hex = Array.from(bytes, entry => entry.toString(16).padStart(2, "0")).join("");
              console.log("[DEBUG] terminal Pack", request.operatorId, bytes.length, hex.slice(0, 1024), JSON.stringify(reply));
            }
            window.__mathProbeReply = reply;
          } catch (error) {
            const details = {
              type: typeof error,
              name: error?.constructor?.name,
              value: String(error),
              stack: error?.stack,
              wasm: error instanceof WebAssembly.Exception,
            };
            if (details.wasm && WebAssembly.JSTag && error.is(WebAssembly.JSTag)) {
              const original = error.getArg(WebAssembly.JSTag, 0);
              details.jsTag = { type: typeof original, value: String(original), stack: original?.stack };
            }
            const fault = JSON.stringify(details);
            console.log("[DEBUG] Math invocation failure", request.operatorId, fault);
            window.__mathProbeReply = { fault };
          }
        })();
        return true;
      }, request);
      if (entered !== true) throw new Error(`Unable to enter ${operatorId} round ${round}`);
      await page.waitForFunction(() => window.__mathProbeReply !== undefined, null, { timeout: 45000 });
      const reply = await page.evaluate(() => window.__mathProbeReply);
      if (!reply || reply.fault) throw new Error(`Math failed at ${operatorId} round ${round} - ${reply?.fault ?? "missing response"}`);
      if (round === 0 && process.env.MATH_CAPTURE_FIRST) {
        const bytes = Uint8Array.from(reply.initialWire);
        const path = resolve(process.env.MATH_CAPTURE_FIRST);
        await writeFile(path, bytes);
        await writeFile(`${path}.completed.json`, JSON.stringify([{ kind: "completed", payload: { req: "1", outcome: { tag: "ok", val: Array.from(bytes) } } }], null, 2) + "\n");
      }
      console.log("[DEBUG]", operatorId, round, reply.phase, reply.units, reply.done, Math.round(performance.now() - started));
      if (reply.units < previous) throw new Error("Math progress moved backwards");
      previous = reply.units;
      if (reply.done) {
        if (round === 0) throw new Error("Expected retained progress before completion");
        return reply;
      }
    }
    throw new Error(`Math did not finish ${operatorId}`);
  };

  try {
    const result = {};
    if (process.env.MATH_EXPECT_SCHEMA === "1") {
      const vector = await evaluate("math.vector", 774, { z: number(6) });
      if (vector.kind !== "vector" || vector.coordinates?.join(",") !== "0,0,6") {
        throw new Error(`Unexpected constructed vector ${JSON.stringify(vector)}`);
      }
      result.coordinates = vector.coordinates;
    }
    if (process.env.MATH_ONLY === "sum") {
      const fixture = JSON.parse(await readFile(join(root, "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🧪️tests/🧫️fixtures/⏱️retained-math/🔣️.json"), "utf8"));
      for (const [index, id] of ["sparseNumberSum", "emptySum", ...(process.env.MATH_VARIADIC === "1" ? ["sparseNumericAdd"] : [])].entries()) {
        const row = fixture.rows.find((entry) => entry.id === id);
        const sum = await evaluate(row.operator, 775 + index, row.input);
        if (sum.kind !== "number" || sum.sum !== row.output.sum.value) throw new Error(`Unexpected ${id} ${JSON.stringify(sum)}`);
        result[id] = sum.sum;
      }
    } else if (process.env.MATH_ONLY !== "vector") {
      const sum = await evaluate("math.add", 773, { a: number(2), b: number(3) });
      if (sum.kind !== "number" || sum.sum !== 5) throw new Error(`Expected 2 + 3 = 5, received ${JSON.stringify(sum)}`);
      result.sum = sum.sum;
    }
    console.log("[DEBUG] Math capability result", JSON.stringify(result));
  } finally {
    const close = setTimeout(() => void server.kill(), 8000);
    try {
      await page.evaluate(async () => await window.__mathProbe.extension.dispose());
    } finally {
      clearTimeout(close);
    }
  }
} finally {
  clearTimeout(timeout);
  const force = setTimeout(() => void server?.kill(), 5000);
  try {
    await server?.close();
  } finally {
    clearTimeout(force);
    proxy.closeAllConnections();
    proxy.close();
  }
}
