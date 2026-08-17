#!/usr/bin/env bun
/**
 * Smoke-boot playground apps: start -> HTTP ready -> kill -> record JSON.
 * Usage: bun smoke-apps.mjs [variant...]
 */
import { spawn } from "child_process";
import { readdirSync, writeFileSync } from "fs";
import { join, resolve } from "path";

const ticketDir = import.meta.dir;
const ROOT = resolve(ticketDir, "../../../../../../");

function findNamed(dir, needle) {
  return readdirSync(dir).find((n) => n.includes(needle));
}

const framework = findNamed(ROOT, "framework");
const products = findNamed(join(ROOT, framework), "products");
const repo = findNamed(join(ROOT, framework, products), "repo");
const modules = findNamed(join(ROOT, framework, products, repo), "modules");
const lib = findNamed(join(ROOT, framework, products, repo, modules), "lib");
const packages = findNamed(join(ROOT, framework, products, repo, modules, lib), "packages");
const typescript = findNamed(join(ROOT, framework, products, repo, modules, lib, packages), "typescript");
const indexFile = readdirSync(join(ROOT, framework, products, repo, modules, lib, packages, typescript)).find((n) => n.includes("index.ts"));
const libPath = join(ROOT, framework, products, repo, modules, lib, packages, typescript, indexFile);

const { loadFrameworkOsPlaygroundCatalog: loadCat, frameworkOsPlaygroundDefaultPort: defaultPort } = await import(libPath);

const playgrounds = loadCat();
const FILTER = process.argv.slice(2);
const PRIORITY = [
  "procedural3d", "procedural2d", "puzzle3d", "puzzle2d", "cad", "flow", "s", "animate", "dag",
  "draw", "forms", "raster", "note", "vcs", "writer", "aggregator", "sourcing", "architect",
  "imperative", "sequence", "layout", "process3d", "gis2d", "fem2d", "block2d", "trinity-jack",
  "reasoning-wires", "playbook", "shooting", "remodel", "mathematical", "lowpoly",
];

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function waitFor(url, timeoutMs) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(2000) });
      if (res.ok || res.status === 404) return { ok: true, status: res.status };
    } catch {}
    await sleep(1000);
  }
  return { ok: false };
}

function variantToSegments(variant) {
  const special = {
    "reasoning-wires": ["wires"],
    "trinity-jack": ["trinity", "jack"],
    "trinity-rewrite": ["trinity", "rewrite"],
  };
  if (special[variant]) return special[variant];
  const m = variant.match(/^(.*?)(2d|3d|5d)$/);
  if (m) {
    const base = m[1];
    const dim = m[2];
    if (base === "puzzle") return [dim];
    if (["procedural", "process", "block", "gis", "fem"].includes(base)) return [base, dim];
  }
  return [variant];
}

function killTree(pid) {
  try { spawn("pkill", ["-P", String(pid)], { stdio: "ignore" }); } catch {}
  try { process.kill(pid, "SIGTERM"); } catch {}
}

async function runOne(variant) {
  const segments = variantToSegments(variant);
  const port = defaultPort(playgrounds, variant, "react");
  const logPath = join(ticketDir, `🧪smoke-${variant}.log`);
  writeFileSync(logPath, "");
  const started = Date.now();
  const script = readdirSync(ROOT).find((n) => /script\.ts$/.test(n));
  const child = spawn(process.execPath, [join(ROOT, script), "dev", ...segments], {
    cwd: ROOT,
    env: { ...process.env, SEMIO_RENDERER: process.env.SEMIO_RENDERER || "react", FORCE_COLOR: "0", CI: "1" },
    stdio: ["ignore", "pipe", "pipe"],
  });
  let output = "";
  const append = (buf) => { output += buf.toString(); writeFileSync(logPath, output); };
  child.stdout.on("data", append);
  child.stderr.on("data", append);
  let exitCode = null;
  child.on("exit", (c) => { exitCode = c; });

  const url = `http://127.0.0.1:${port}/`;
  const ready = await waitFor(url, 120000);
  const logReady = /VITE .* ready|Local:\s+http/i.test(output);
  const failedHard = /unknown playground|exited with status|Cannot find the package|ENOENT|panic/i.test(output) && !logReady && !ready.ok;
  const result = {
    variant, segments, port, url,
    fetchOk: ready.ok, fetchStatus: ready.status ?? null,
    logReady, failedHard, exitCode, ms: Date.now() - started, logTail: output.slice(-2000),
  };
  killTree(child.pid);
  await sleep(2000);
  try { child.kill("SIGKILL"); } catch {}
  return result;
}

const targets = (FILTER.length ? FILTER : PRIORITY).filter((v) => playgrounds.some((e) => e.variant === v) || FILTER.includes(v));
const results = [];
for (const v of targets) {
  console.log("SMOKE", v);
  try { results.push(await runOne(v)); }
  catch (e) { results.push({ variant: v, error: String(e) }); }
  writeFileSync(join(ticketDir, "🧪smoke-results.json"), JSON.stringify(results, null, 2));
  try { spawn("pkill", ["-f", "framework-os-dev"], { stdio: "ignore" }); } catch {}
  try { spawn("pkill", ["-f", "vite"], { stdio: "ignore" }); } catch {}
  await sleep(2000);
}
console.log(JSON.stringify(results.map((r) => ({
  variant: r.variant, fetchOk: r.fetchOk, logReady: r.logReady, failedHard: r.failedHard, exitCode: r.exitCode, error: r.error, port: r.port,
})), null, 2));
process.exit(results.some((r) => !(r.fetchOk || r.logReady)) ? 1 : 0);
