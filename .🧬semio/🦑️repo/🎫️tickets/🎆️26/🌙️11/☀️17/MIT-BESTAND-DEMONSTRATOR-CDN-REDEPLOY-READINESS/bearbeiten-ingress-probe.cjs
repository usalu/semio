const { createHash } = require("node:crypto");
const { createRequire } = require("node:module");
const { createWriteStream } = require("node:fs");
const { once } = require("node:events");
const { join } = require("node:path");

const root = process.cwd();
const project = createRequire(join(root, "♻️mit-bestand", "🧺️demonstrator", "package.json"));
process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(root, ".🧬semio", "🦑️repo", "⚡️cache", "tools", "ms-playwright");
const { chromium } = project("@playwright/test");

/** 📥️ Observe physical command page, status and paid epoch across the worker boundary. */
function observeWire(capturePages) {
  const capturedPages = new Set();
  const native = window.Worker;
  const pending = new Map();
  const totals = { commands: 0, replies: 0, receipts: 0 };
  const statuses = new Map();
  const currencies = new Map();
  const print = (label, value) => console.info("[DEBUG]", label, JSON.stringify(value, (_, row) => typeof row === "bigint" ? String(row) : row));
  const seen = new Set();
  let cursor;
  let last;
  window.Worker = class extends native {
    constructor(...args) {
      super(...args);
      this.addEventListener("message", ({ data }) => {
        const owner = pending.get(data.requestId);
        if (data.requestId) pending.delete(data.requestId);
        if (!/^demonstrator#/.test(data.actorId ?? owner?.actor ?? "")) return;
        if (data.receipt) {
          totals.receipts++;
          const input = data.receipt.input, spent = data.receipt.spent;
          if (input && spent) {
            last = { epoch: input.epoch, operation: input.operation, grant: input.grant, spent, work: data.receipt.workUnitsSpent };
            const key = [spent.copiedItems, spent.copiedBytes, spent.retainedCapacityBytes, spent.releasedBytes, data.receipt.workUnitsSpent].join("/");
            currencies.set(key, (currencies.get(key) ?? 0) + 1);
          }
        }
        if (data.ok === false || /fault|trap|refusal/.test(data.kind)) print("Bearbeiten first refusal", [data.kind, data.requestId, data.error ?? data.message ?? data.refusal ?? data]);
        const result = data.value ?? data.result;
        if (!result?.commandIngress) return;
        totals.replies++;
        const status = result.commandIngress;
        cursor = [data.requestId, status, last?.epoch, result.status, data.timings?.driveStopped];
        statuses.set(status.tag, (statuses.get(status.tag) ?? 0) + 1);
        const key = JSON.stringify(status, (_, row) => typeof row === "bigint" ? String(row) : row);
        if (!seen.has(key)) {
          seen.add(key);
          print("Bearbeiten cursor", cursor);
        }
      });
    }
    postMessage(data, ...args) {
      if (data.requestId && /^demonstrator#/.test(data.actorId ?? "")) {
        pending.set(data.requestId, { actor: data.actorId, kind: data.kind, since: performance.now() });
        if (data.commandPage) {
          totals.commands++;
          const { cursor, page } = data.commandPage;
          print("Bearbeiten offer", [data.requestId, cursor, page?.length]);
          const bytes = data.commandPage.bytes;
          if (capturePages && (bytes instanceof Uint8Array || Array.isArray(bytes)) && bytes.length <= 4096 && capturedPages.size < 256) {
            const key = JSON.stringify(cursor, (_, value) => typeof value === "bigint" ? String(value) : value);
            if (!capturedPages.has(key)) {
              capturedPages.add(key);
              print("command-page-bytes", [cursor, btoa(String.fromCharCode(...bytes))]);
            }
          }
        }
      }
      return super.postMessage(data, ...args);
    }
  };
  setInterval(() => {
    const recentCurrencies = [...currencies].sort((a, b) => b[1] - a[1]).slice(0, 10);
    const unresolved = [...pending].slice(-6).map(([id, row]) => [id, row.kind, Math.round(performance.now() - row.since)]);
    print("Bearbeiten wire", { totals, last, cursor, unresolved, statuses: Object.fromEntries(statuses), currencies: recentCurrencies });
    currencies.clear();
  }, 1000);
}

/** 🎞️ Captures the shard's physical CPU frames while the ingress request is unanswered. */
async function sample(browser, path) {
  const cdp = await browser.newBrowserCDPSession();
  try {
    await cdp.send("Tracing.start", { categories: "disabled-by-default-v8.cpu_profiler", transferMode: "ReturnAsStream" });
    await new Promise(resolve => setTimeout(resolve, 1500));
    const ready = new Promise(resolve => cdp.once("Tracing.tracingComplete", resolve));
    await cdp.send("Tracing.end");
    const { stream } = await ready;
    const output = createWriteStream(path);
    try {
      for (;;) {
        const part = await cdp.send("IO.read", { handle: stream });
        if (!output.write(part.base64Encoded ? Buffer.from(part.data, "base64") : part.data)) await once(output, "drain");
        if (part.eof) break;
      }
      output.end();
      await once(output, "finish");
    } finally {
      await cdp.send("IO.close", { handle: stream });
      output.destroy();
    }
  } finally {
    await cdp.detach();
  }
}

/** 🔧️ Exposes the guest's own diagnostics without changing its core or its turn grant. */
async function arm(context) {
  if (process.env.SEMIO_CAPTURE_WORKER_TRAFFIC === "1") {
    await context.addInitScript(require("./capture-worker-traffic.cjs").captureWorkerTraffic);
  }
  await context.addInitScript(observeWire, Boolean(process.argv[4]));
  await context.route(url => url.pathname.endsWith("/semio_hub_demonstrator_component.js"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const env = "const { getEnvironment } = environment;";
    if (source.split(env).length !== 2) throw new Error("the JCO environment export changed");
    console.log("[DEBUG] Bearbeiten original component JS SHA-256", createHash("sha256").update(source).digest("hex"));
    await route.fulfill({ response, body: source.replace(env, 'const getEnvironment = () => [...environment.getEnvironment(), ["SEMIO_RUNTIME_DIAGNOSTICS", "1"]];') });
  });
}

/** ▶️ Opens one pane for a bounded observation and always closes its browser. */
async function main() {
  const [url, seconds = "35", profile] = process.argv.slice(2);
  const browser = await chromium.launch();
  try {
    const context = await browser.newContext({ locale: "en-US" });
    await arm(context);
    const page = await context.newPage();
    let lines = 0;
    page.on("console", message => {
      const line = message.text();
      if (line.startsWith("[DEBUG] authenticated-") || line.startsWith("[DEBUG] command-page-bytes ")) { console.log(line); return; }
      if (/Bearbeiten|command.failed|classification lost|command stall|plugin\.command-|original-turn\./i.test(line)) console.log(new Date().toISOString(), line.slice(0, 2200));
      else if (/more-work.*command_ingress/.test(line) && ++lines % 1024 === 0) console.log(new Date().toISOString(), line.slice(0, 900));
    });
    page.on("pageerror", error => console.log("[DEBUG] Bearbeiten page error", String(error)));
    await page.goto(url, { waitUntil: "domcontentloaded" });
    const duration = page.waitForTimeout(Number(seconds) * 1000);
    if (profile) {
      await page.waitForTimeout(12000);
      await sample(browser, profile);
    }
    await duration;
    const pane = new URL(url).hash.slice(1) || "bearbeiten";
    console.log("[DEBUG] Ingress shell", pane, await page.locator(`[data-shell-id="${pane}"]`).getAttribute("data-shell-ready", { timeout: 2000 }).catch(() => null), lines);
  } finally {
    await browser.close();
  }
}

main().catch(error => { console.error(error); process.exitCode = 1; });
