import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { execFileSync } from "node:child_process";

/** 🛡️ One capped real-GPU Chromium probe: renderer heap and process count limited, browser tree killed past 2 GB RSS or at its timeout, console kept to a bounded tail. */
const out = join(import.meta.dir, "🗑️generated/wgpu-paint"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const tag = process.env.PAINT_TAG ?? "capped";
const ms = Number(process.env.SMOKE_MS ?? 60000);
const capKb = Number(process.env.RSS_CAP_MB ?? 2048) * 1024;
const marker = `p${process.pid}`;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", "--enable-features=Vulkan,WebGPU", "--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", `--semio-probe=${marker}`] });
const rootPid = (() => { for (let i = 0; i < 20; i++) { try { const pids = execFileSync("pgrep", ["-f", `semio-probe=${marker}`], { encoding: "utf8" }).split("\n").filter(Boolean).map(Number).filter(p => p !== process.pid); if (pids.length) return Math.min(...pids); } catch {} execFileSync("sleep", ["0.5"]); } throw Error("browser pid not found"); })();
const tree = (pid) => { try { const kids = execFileSync("pgrep", ["-P", String(pid)], { encoding: "utf8" }).split("\n").filter(Boolean).map(Number); return [pid, ...kids.flatMap(tree)]; } catch { return [pid]; } };
const rss = () => { try { return execFileSync("ps", ["-o", "rss=", "-p", tree(rootPid).join(",")], { encoding: "utf8" }).split("\n").reduce((a, l) => a + (Number(l.trim()) || 0), 0); } catch { return 0; } };
let peak = 0, killed = "";
const kill = (why) => { killed = why; for (const pid of tree(rootPid).reverse()) { try { process.kill(pid, "SIGKILL"); } catch {} } };
const watchdog = setInterval(() => { const kb = rss(); peak = Math.max(peak, kb); if (kb > capKb) kill(`rss ${Math.round(kb / 1024)} MB`); }, 1500);
const hard = setTimeout(() => kill("timeout"), ms + 30000);
const lines = []; const keep = (l) => { lines.push(l); if (lines.length > 4000) lines.splice(0, 2000); };
try {
  const page = await (await browser.newContext({ viewport: { width: 1280, height: 800 } })).newPage();
  page.on("console", m => keep(`${m.type()} ${m.text().slice(0, 400)}`));
  page.on("pageerror", e => keep(`pageerror ${e.message}`));
  await page.goto(url, { waitUntil: "load" });
  await page.waitForTimeout(ms);
  await page.screenshot({ path: join(out, `${tag}.png`) });
} catch (e) { keep(`probe-error ${String(e).slice(0, 300)}`); }
clearInterval(watchdog); clearTimeout(hard);
writeFileSync(join(out, `${tag}-console.txt`), lines.join("\n") + `\npeakRssMb=${Math.round(peak / 1024)} killed=${killed}`);
console.log(`peakRssMb=${Math.round(peak / 1024)} killed=${killed || "no"} lines=${lines.length}`);
try { await browser.close(); } catch {}
