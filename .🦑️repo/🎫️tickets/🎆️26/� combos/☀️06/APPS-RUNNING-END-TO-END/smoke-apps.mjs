import { spawn } from "child_process";
import { writeFileSync, readdirSync } from "fs";
import { join } from "path";

const month = readdirSync(".🦑️repo/🎫️tickets/🎆️26").find((x) => x.includes("08"));
const ticket = join(".🦑️repo/🎫️tickets/🎆️26", month, "☀️06", "APPS-RUNNING-END-TO-END");
const fw = readdirSync(".").find((n) => n.includes("framework") && !n.startsWith("."));
const pkgDir = join(fw, "🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages");
const pkg = readdirSync(pkgDir).find((n) => n.includes("typescript"));
const mod = await import(join(process.cwd(), pkgDir, pkg, "📦️index.ts"));
const catalog = mod.loadFrameworkOsPlaygroundCatalog();
const resolve = mod.resolveFrameworkOsPlaygroundPlugin;
const smokeList = [
  ["cad"],
  ["puzzle", "3d"],
  ["flow"],
  ["animate"],
  ["sourcing"],
  ["dag"],
  ["procedural", "3d"],
  ["gis", "2d"],
];
const results = [];

for (const segs of smokeList) {
  const resolved = resolve(catalog, segs);
  if (!resolved) {
    results.push({ segs, ok: false, error: "unresolved" });
    console.log(JSON.stringify(results.at(-1)));
    continue;
  }
  const row = catalog.find((r) => r.variant === resolved.plugin);
  const port = row.ports.react;
  let already = false;
  try {
    const r = await fetch("http://127.0.0.1:" + port + "/", { signal: AbortSignal.timeout(400) });
    already = r.status > 0;
  } catch {}
  if (already) {
    results.push({ segs, plugin: resolved.plugin, port, ok: true, mode: "already-up" });
    console.log(JSON.stringify(results.at(-1)));
    continue;
  }
  const child = spawn("bun", ["./📜️script.ts", "dev", ...segs], {
    cwd: process.cwd(),
    env: { ...process.env, SEMIO_RENDERER: "react" },
    stdio: ["ignore", "pipe", "pipe"],
  });
  let log = "";
  child.stdout.on("data", (d) => {
    log += d.toString();
  });
  child.stderr.on("data", (d) => {
    log += d.toString();
  });
  const started = Date.now();
  let ok = false;
  let status = 0;
  let err = "";
  while (Date.now() - started < 90000) {
    if (log.includes("Local:")) {
      try {
        const r = await fetch("http://127.0.0.1:" + port + "/", { signal: AbortSignal.timeout(3000) });
        status = r.status;
        ok = status > 0 && status < 500;
        break;
      } catch (e) {
        err = String(e);
      }
    }
    if (/EADDRINUSE|exited with status/.test(log) && Date.now() - started > 25000) {
      err = log.slice(-800);
      break;
    }
    await Bun.sleep(500);
  }
  try {
    child.kill("SIGTERM");
  } catch {}
  await Bun.sleep(400);
  try {
    child.kill("SIGKILL");
  } catch {}
  results.push({
    segs,
    plugin: resolved.plugin,
    port,
    ok,
    status,
    err: err || undefined,
    logTail: log.slice(-600),
  });
  writeFileSync(join(ticket, "🧪smoke-" + resolved.plugin + ".log"), log);
  console.log(JSON.stringify(results.at(-1)));
}

writeFileSync(join(ticket, "🧪smoke-results.json"), JSON.stringify(results, null, 2));
console.log("DONE", results.filter((r) => r.ok).length + "/" + results.length);
