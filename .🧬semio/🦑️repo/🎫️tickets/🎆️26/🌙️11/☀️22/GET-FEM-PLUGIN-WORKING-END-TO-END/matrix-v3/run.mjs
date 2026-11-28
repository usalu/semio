/** 🏁 Headless end-to-end matrix runner. Usage: node run.mjs <2d|3d|viewer2d|viewer3d> [--only ids|areas|E3D-16,...] [--skip-slow] [--boot-timeout 180] [--list] [--headed] [--url URL] [--append] [--force]
 * --legacy-caption: smoke against a wasm without the DOM results caption (2D before the caption fix): boot and solved waits ignore the caption (fixed 15 s solve wait); caption-reading checks are then meaningless.
 * --force keeps running checks after a boot-hang (smoke of the shell selectors only; rows are marked forced). */
import { appendFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
import { APPS } from "./apps.mjs";
import { BootHang, Ctx } from "./lib.mjs";
import { coreChecks } from "./checks-core.mjs";
import { modelChecks } from "./checks-model.mjs";
import { resultsChecks } from "./checks-results.mjs";
import { viewerChecks } from "./checks-viewer.mjs";

const argv = process.argv.slice(2);
const flag = (n, dflt) => { const i = argv.indexOf(`--${n}`); return i < 0 ? dflt : argv[i + 1]?.startsWith("--") || argv[i + 1] === undefined ? true : argv[i + 1]; };
const appKey = { "2d": "2d", "3d": "3d", v2d: "viewer2d", v3d: "viewer3d", viewer2d: "viewer2d", viewer3d: "viewer3d" }[argv[0]];
if (!appKey) { console.error("usage: node run.mjs <2d|3d|viewer2d|viewer3d> [--only ids] [--skip-slow] [--boot-timeout s] [--list] [--headed] [--url u] [--append] [--force] [--legacy-caption] [--out dir]"); process.exit(2); }
const app = { ...APPS[appKey], url: flag("url", APPS[appKey].url) };
const here = fileURLToPath(new URL("../🗑️generated/matrix-v3/", import.meta.url));
const out = flag("out", false) ? String(flag("out")).replaceAll("\\", "/") : `${here}${appKey}`, dirs = { out, shots: `${out}/shots` };
const bootMs = Number(flag("boot-timeout", 180)) * 1000;
const only = String(flag("only", "")).split(",").map(s => s.trim()).filter(Boolean);
const RANK = { viewer: 0, shell: 0, selection: 1, utilities: 1, camera: 1, delete: 2, history: 2, inspection: 3, tree: 3, actions: 4, results: 5, playback: 5, "import-export": 6, clipboard: 6, roles: 7, language: 8, settings: 8, windows: 9, examples: 10 };

const all = app.kind === "viewer" ? viewerChecks(app) : [...coreChecks(app), ...modelChecks(app), ...resultsChecks(app)];
const dimKey = app.dim;
const invIds = c => (c.inv?.[dimKey] ?? []).map(i => typeof i === "number" ? `${app.inv}-${String(i).padStart(2, "0")}` : i);
let checks = all.map((c, i) => ({ ...c, order: i, invIds: invIds(c) })).sort((p, q) => (RANK[p.area] ?? 5) - (RANK[q.area] ?? 5) || p.order - q.order);
if (only.length) checks = checks.filter(c => only.some(t => c.id === t || c.id.startsWith(t.endsWith(".") ? t : `${t}.`) || c.id.startsWith(t) && t.endsWith(".") || c.area === t || c.invIds.includes(t.toUpperCase())));
if (flag("skip-slow", false)) checks = checks.filter(c => !c.slow);
if (flag("list", false)) { for (const c of checks) console.log(`${c.id}\t${c.area}\t${c.invIds.join(",")}\t${c.expected}`); console.log(`${checks.length} checks`); process.exit(0); }

if (!flag("append", false)) rmSync(out, { recursive: true, force: true });
mkdirSync(dirs.shots, { recursive: true });
const jsonl = `${out}/results.jsonl`;
const emit = row => { appendFileSync(jsonl, JSON.stringify(row) + "\n"); console.log(`${row.status.padEnd(9)} ${row.id} (${row.ms} ms) ${String(row.observed).slice(0, 160)}`); };
const withTimeout = (p, ms) => new Promise((resolve, reject) => { const t = setTimeout(() => reject(new Error(`timeout after ${Math.round(ms / 1000)} s`)), ms); p.then(v => { clearTimeout(t); resolve(v); }, e => { clearTimeout(t); reject(e); }); });

const browser = await chromium.launch({ headless: !flag("headed", false) });
const stop = async () => { await browser.close().catch(() => {}); };
process.on("SIGINT", async () => { await stop(); process.exit(130); });
let code = 0;
try {
  const x = new Ctx(app, browser, dirs); x.legacyCaption = flag("legacy-caption", false) === true; await x.init();
  const t0 = Date.now();
  try { const ms = await x.boot(bootMs); emit({ id: `${app.key}.boot`, app: app.key, area: "boot", action: `Open ${app.url}, wait for Demo`, expected: "Model window shows the Demo model (Nodes > 0, Results caption not Pending)", observed: `ready after ${ms} ms; calibration ${JSON.stringify(x.calibration ?? null)}`, pass: true, status: "PASS", ms, shot: await x.shot(`${app.key}.boot`), inv: [], console: x.consoleSince(0) }); }
  catch (e) {
    const shot = await x.shot(`${app.key}.boot-hang`); const tail = x.logs.slice(-8).map(l => `${l.type} ${l.text.slice(0, 160)}`);
    writeFileSync(`${out}/boot-hang-console.txt`, x.logs.map(l => `${l.type} ${l.text}`).join("\n"));
    emit({ id: `${app.key}.boot-hang`, app: app.key, area: "boot", action: `Open ${app.url}`, expected: "Model window shows the Demo model within the boot timeout", observed: e instanceof BootHang ? e.message : String(e), pass: false, status: "BOOT-HANG", ms: Date.now() - t0, shot, inv: [], console: { tail } });
    code = 3; if (!flag("force", false)) throw e;
    console.log("--force: continuing without a booted model (smoke of shell selectors only)");
  }
  let fresh = true; let firstRun = true; const forced = code === 3;
  for (const c of checks) {
    const row = { forced, id: `${app.key}.${c.id}`, app: app.key, area: c.area, action: c.action, expected: c.expected, inv: c.invIds };
    const t = Date.now(); let res;
    try {
      if ((c.fresh && !fresh && !forced) || x.needsReload) { await x.reload(bootMs); x.needsReload = false; x.exported = x.exported; }
      else if (!fresh && !forced && !(await x.healthy().catch(() => false))) await x.reload(bootMs);
      fresh = false; firstRun = false;
      const mark = x.mark(); x.freeLayout = false;
      try { res = await withTimeout(c.run(x), c.timeout ?? 120000); }
      catch (e) { res = { pass: false, observed: `${String(e.message ?? e).split("\n")[0].slice(0, 300)}` }; if (/timeout after/.test(res.observed)) x.needsReload = true; }
      if (c.after) await withTimeout(c.after(x), 60000).catch(() => {});
      const con = x.consoleSince(mark);
      if (res.pass === true && con.fatal > 0) { res.pass = false; res.observed += ` | CONSOLE FATAL: ${con.fatalFirst.join(" || ")}`; }
      row.console = con;
    } catch (e) {
      if (e instanceof BootHang) { row.observed = `reload ${e.message}`; row.pass = false; row.status = "BOOT-HANG"; row.ms = Date.now() - t; row.shot = await x.shot(row.id); emit(row); code = 3; break; }
      res = { pass: false, observed: `harness error ${String(e).slice(0, 300)}` };
    }
    row.observed = res.observed; row.pass = res.pass; row.status = res.pass === true ? "PASS" : res.pass === null ? "SKIP" : "FAIL";
    row.ms = Date.now() - t; row.shot = await x.shot(row.id); emit(row);
  }
} catch (e) { if (code === 0) { code = 1; console.error(e); } }
finally { await stop(); }
console.log(`done, results in ${jsonl}`);
process.exit(code);
