import { readdir, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { chromium } from "playwright";
import { pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";

const staging = join(import.meta.dir, "🗑️generated", "browser-materialization");
const editor = process.argv.includes("--editor");
const picker = process.argv.includes("--editor-picker");
const read = process.argv.includes("--editor-read");
const editorMode = read ? "read" : picker ? "picker" : editor ? "editor" : undefined;
const report = join(staging, read ? "isolated-editor-read.json" : picker ? "isolated-editor-picker.json" : editor ? "isolated-editor.json" : "isolated-guest.json");
const started = performance.now();
const progress: string[] = [];
const diagnose = process.argv.includes("--diagnose");
try {
  const build = await Bun.build({
    entrypoints: [join(import.meta.dir, "🧪️browser-remodel-guest-web.ts")],
    target: "browser",
    format: "esm",
    define: { "import.meta.vitest": "undefined" },
    plugins: [{
      name: "external-test-modules",
      setup(builder) {
        builder.onResolve({ filter: /🧪️tests\//u }, ({ path }) => ({ path, external: true }));
      },
    }],
  });
  if (!build.success || build.outputs.length !== 1) throw Error("Browser probe did not bundle");
  await writeFile(join(staging, "guest-probe.js"), await build.outputs[0].text());
  let bridge = pluginComponentBridgeSource("remodel", "remodel.core.wasm");
  if (diagnose) {
    for (const [call, name] of [
      ['await receiveContext("admitActorContext")', "admit-context"],
      ["await reactor.admitActorInput(wireBytes, originalContextBudgetWit(turn))", "admit-input"],
      ["await reactor.poll(slice)", "poll-reactor"],
    ]) {
      if (bridge.split(call).length !== 2) throw Error("Expected one bridge " + name + " call");
      const expression = call.slice("await ".length);
      bridge = bridge.replace(call, `await (console.debug("[DEBUG] Guest probe ${name}:start"), ${expression}.then(result => { console.debug("[DEBUG] Guest probe ${name}:returned"); return result; }))`);
    }
  }
  await writeFile(join(staging, "module", "probe-bridge.mjs"), bridge);
  const assets = new Map([["/guest-probe.js", join(staging, "guest-probe.js")]]);
  if (editor) assets.set("/frame.png", join(import.meta.dir, "../../../../../../../✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/📷️two-by-two.png"));
  for (const role of ["module", "vendor"]) {
    for (const file of await readdir(join(staging, role))) {
      if (/\.(?:m?js|wasm)$/u.test(file)) assets.set("/" + role + "/" + file, join(staging, role, file));
    }
  }
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    fetch(request) {
      const path = decodeURIComponent(new URL(request.url).pathname);
      if (path === "/") return new Response("<title>Remodel guest probe</title>", { headers: { "content-type": "text/html" } });
      const file = assets.get(path);
      if (!file) return new Response("Unknown probe asset", { status: 404 });
      const contentType = path.endsWith(".wasm") ? "application/wasm" : path.endsWith(".png") ? "image/png" : "text/javascript";
      return new Response(Bun.file(file), { headers: { "content-type": contentType } });
    },
  });
  try {
    const browser = await chromium.launch({
      headless: true,
      timeout: 8_000,
      args: diagnose ? ["--js-flags=--liftoff-only --wasm-lazy-compilation"] : [],
    });
    try {
      const page = await browser.newPage();
      page.on("console", (message) => {
        if (message.text().startsWith("[DEBUG] ")) progress.push(message.text());
      });
      page.on("requestfailed", (request) => progress.push(new URL(request.url()).pathname + " " + request.failure()?.errorText));
      page.on("pageerror", (error) => progress.push(error.message));
      await page.goto(server.url.href, { timeout: 8_000 });
      let deadline: ReturnType<typeof setTimeout> | undefined;
      const timeout = new Promise<never>((_, reject) => { deadline = setTimeout(() => reject(Error("Chromium guest timed out")), 50_000); });
      try {
        const probe = page.evaluate(async (editorMode) => {
          let frameData: string | undefined;
          if (editorMode === "editor") {
            const frame = await fetch("/frame.png");
            if (!frame.ok) throw Error("Editor PNG fixture unavailable");
            const bytes = new Uint8Array(await frame.arrayBuffer());
            frameData = "data:image/png;base64," + btoa(String.fromCharCode(...bytes));
          }
          return (await import(location.origin + "/guest-probe.js")).run(frameData, editorMode);
        }, editorMode);
        const result = await Promise.race([probe, timeout]);
        await writeFile(report, JSON.stringify({ chromium: browser.version(), elapsedMs: Math.round(performance.now() - started), progress, ...result }, null, 2) + "\n");
        if (!result.ok) process.exitCode = 1;
      } finally {
        clearTimeout(deadline);
      }
    } finally {
      await browser.close();
    }
  } finally {
    await server.stop(true);
  }
} catch (error) {
  await writeFile(report, JSON.stringify({ ok: false, progress, message: error instanceof Error ? error.stack : String(error) }, null, 2) + "\n");
  throw error;
}
