import { spawn } from "node:child_process";
import { createWriteStream, existsSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

let workspace = import.meta.dir;
while (!existsSync(join(workspace, "🧰️framework"))) {
  const parent = dirname(workspace);
  if (parent === workspace) throw new Error("Workspace root not found");
  workspace = parent;
}
const generated = join(import.meta.dir, "🗑️generated");
const python = join(generated, "v", process.platform === "win32" ? "Scripts/python.exe" : "bin/python");
const bundledProbe = join(generated, "f", process.platform === "win32" ? "ffprobe.exe" : "ffprobe");
const ffprobe = process.env.SEMIO_FFPROBE_BINARY ?? (existsSync(bundledProbe) ? bundledProbe : "ffprobe");
const binaries = join(generated, "v", process.platform === "win32" ? "Lib/site-packages" : "lib/python3.12/site-packages", "imageio_ffmpeg/binaries");
const ffmpeg = process.env.SEMIO_FFMPEG_BINARY ?? join(binaries, readdirSync(binaries).find(name => name.startsWith("ffmpeg-"))!);
const output = join(generated, "taxonomy-fixed");
const profile = [output, "--teaching-root", join(workspace, "🎓️teaching"), "--ffprobe", ffprobe, "--width", "320", "--height", "180", "--fps", "5"];
const steps = [
  { name: "verification", args: [join(import.meta.dir, "verify-videos.py"), ...profile, "--ffmpeg", ffmpeg, "--report", join(generated, "taxonomy-verification.json")] },
  { name: "previews", args: [join(import.meta.dir, "make-previews.py"), ...profile, "--output", join(generated, "taxonomy-previews")] },
];
for (const step of steps) {
  const started = Date.now();
  const log = createWriteStream(join(generated, `taxonomy-${step.name}-${started}.log`), { flags: "wx" });
  const child = spawn(python, step.args, { cwd: workspace, env: { ...process.env, PYTHONUTF8: "1", PYTHONIOENCODING: "utf-8", PYTHONDONTWRITEBYTECODE: "1" }, windowsHide: true, detached: process.platform !== "win32" });
  const cancel = () => {
    if (!child.pid) return;
    if (process.platform === "win32") spawn("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
    else process.kill(-child.pid, "SIGTERM");
  };
  process.on("SIGINT", cancel);
  process.on("SIGTERM", cancel);
  for (const stream of [child.stdout, child.stderr]) stream.on("data", chunk => { log.write(chunk); process.stdout.write(chunk); });
  const code = await new Promise<number>((resolve, reject) => { child.on("error", reject); child.on("close", value => resolve(value ?? 1)); });
  process.off("SIGINT", cancel);
  process.off("SIGTERM", cancel);
  await new Promise<void>(resolve => log.end(resolve));
  writeFileSync(join(generated, `taxonomy-${step.name}-process.json`), JSON.stringify({ pid: child.pid, started, finished: Date.now(), exit_code: code, command: [python, ...step.args] }, null, 2));
  console.log("[DEBUG]", step.name, "exit", code, "PID", child.pid);
  if (code) process.exit(code);
}
