#!/usr/bin/env bun
import { spawn } from "child_process";
import { readdirSync, writeFileSync } from "fs";
import { join } from "path";

const ROOT = process.cwd();
const ticket = process.argv[2];
function findNamed(dir, needle) {
  return readdirSync(dir).find((n) => n.includes(needle));
}
const fw = findNamed(ROOT, "framework");
const products = findNamed(join(ROOT, fw), "products");
const repo = findNamed(join(ROOT, fw, products), "repo");
const modules = findNamed(join(ROOT, fw, products, repo), "modules");
const lib = findNamed(join(ROOT, fw, products, repo, modules), "lib");
const packages = findNamed(join(ROOT, fw, products, repo, modules, lib), "packages");
const typescript = findNamed(join(ROOT, fw, products, repo, modules, lib, packages), "typescript");
const indexFile = readdirSync(join(ROOT, fw, products, repo, modules, lib, packages, typescript)).find((n) => n.includes("index.ts"));
const { loadFrameworkOsPlaygroundCatalog, frameworkOsPlaygroundDefaultPort } = await import(
  join(ROOT, fw, products, repo, modules, lib, packages, typescript, indexFile),
);
const catalog = loadFrameworkOsPlaygroundCatalog();
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

const apps = [
  ["puzzle3d", ["3d"]],
  ["cad", ["cad"]],
  ["flow", ["flow"]],
  ["s", ["s"]],
  ["animate", ["animate"]],
  ["dag", ["dag"]],
  ["draw", ["draw"]],
  ["forms", ["forms"]],
  ["aggregator", ["aggregator"]],
  ["procedural2d", ["procedural", "2d"]],
];

async function waitFor(url, ms) {
  const start = Date.now();
  while (Date.now() - start < ms) {
    try {
      const r = await fetch(url, { signal: AbortSignal.timeout(1500) });
      if (r.ok) return true;
    } catch {}
    await sleep(1000);
  }
  return false;
}

const results = [];
for (const [variant, segs] of apps) {
  const port = frameworkOsPlaygroundDefaultPort(catalog, variant, "react");
  const log = join(ticket, "🧪smoke2-" + variant + ".log");
  writeFileSync(log, "");
  const script = readdirSync(ROOT).find((n) => /script\.ts$/.test(n));
  const child = spawn(process.execPath, [join(ROOT, script), "dev", ...segs], {
    cwd: ROOT,
    env: { ...process.env, SEMIO_RENDERER: "react", FORCE_COLOR: "0" },
    stdio: ["ignore", "pipe", "pipe"],
  });
  let out = "";
  const append = (b) => {
    out += b.toString();
    writeFileSync(log, out);
  };
  child.stdout.on("data", append);
  child.stderr.on("data", append);
  const ok = await waitFor("http://127.0.0.1:" + port + "/", 90000);
  const logReady = /VITE .* ready|Local:\s+http/i.test(out);
  const row = { variant, port, ok: ok || logReady, fetchOk: ok, logReady, tail: out.slice(-800) };
  results.push(row);
  console.log(JSON.stringify(row));
  try {
    spawn("pkill", ["-P", String(child.pid)], { stdio: "ignore" });
  } catch {}
  try {
    child.kill("SIGTERM");
  } catch {}
  await sleep(2000);
  try {
    child.kill("SIGKILL");
  } catch {}
  try {
    spawn("pkill", ["-f", "framework-os-dev"], { stdio: "ignore" });
  } catch {}
  try {
    spawn("pkill", ["-f", "vite"], { stdio: "ignore" });
  } catch {}
  await sleep(1500);
  writeFileSync(join(ticket, "🧪smoke2-results.json"), JSON.stringify(results, null, 2));
}
const bad = results.filter((r) => !r.ok);
console.log("SUMMARY", { ok: results.length - bad.length, bad: bad.map((r) => r.variant) });
process.exit(bad.length ? 1 : 0);
