import { open } from "node:fs/promises";
import { join } from "node:path";

const [generated, launchLog, url = "http://127.0.0.1:6064/", lane = "direct"] = Bun.argv.slice(2);
const browserTest = "✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🟦️.mjs";
const chromium = ".🧬semio/🦑️repo/⚡️cache/tools/ms-playwright";
const stop = Date.now() + 3 * 60 * 60 * 1000;
for (;;) {
  try {
    const response = await fetch(new URL("?plugin=draw", url), { signal: AbortSignal.timeout(3000) });
    if (response.ok && response.headers.get("content-type")?.includes("text/html")) {
      await response.body?.cancel();
      break;
    }
    await response.body?.cancel();
  } catch {}
  const launch = Bun.file(launchLog);
  if (await launch.exists()) {
    const tail = await launch.slice(Math.max(0, launch.size - 8192)).text();
    if (/repository:command failed with exit code|Failed to complete tasks|Error: closed/i.test(tail)) throw Error("Draw launch ended before serving the editor");
  }
  if (Date.now() > stop) throw Error("Draw editor did not start within the observation window");
  await Bun.sleep(3000);
}
console.log("[DEBUG] Draw returned HTTP 200 HTML",url);
let failure = false;
for (const locale of ["en", "de"]) {
  const output = join(generated, lane + "-" + locale);
  const log = await open(join(generated, lane + "-" + locale + ".log"), "wx");
  const task = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", browserTest,
    "--serve", url, "--locale", locale, "--output", output], {
      env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: join(process.cwd(), chromium) },
      stdout: log.fd, stderr: log.fd,
  });
  try {
    const code = await task.exited;
    console.log("[DEBUG] Draw browser", locale, code);
    failure ||= code !== 0;
  } finally { await log.close(); }
}
process.exitCode = Number(failure);
