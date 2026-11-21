/** 🛠️ Runs the live browser lifecycle against the component retained from the repaired Cargo link. */
import { join } from "node:path";

const [generated, browsers, build = "draw-export-contract"] = Bun.argv.slice(2);
if (!generated || !browsers) throw Error("Expected retained Draw ticket output and managed browser locations");
const root = process.cwd();
const retained = join(generated, build + "-linked.wasm");
const retentionLog = join(generated, build + "-retention.log");
const moduleRoot = join(generated, build + "-browser");
const deadline = Date.now() + 10_800_000;

while (Date.now() < deadline && !await Bun.file(retained).exists()) {
  const log = Bun.file(retentionLog);
  if (await log.exists()) {
    const tail = await log.slice(Math.max(0, log.size - 8192)).text();
    if (/Cargo ended before linking Draw|Timed out|lacks the WASI component signature/.test(tail)) {
      throw Error("The repaired Cargo component was not retained");
    }
  }
  await Bun.sleep(2000);
}
if (!await Bun.file(retained).exists()) throw Error("The repaired Draw link remained unavailable");

const operations = [
  [join(generated, "../probe-linked-draw-browser.ts"), retained, join(generated, "current-draw-descriptor"), moduleRoot, "module-only"],
  [join(generated, "../probe-linked-draw-lifecycle.ts"), moduleRoot],
];
for (const [script, ...args] of operations) {
  const task = Bun.spawn(["repo", "capacity", "run", "--lang=en", "--memory=1300", "--cpu=1", "--", "repo", "tools", "exec", "--light", "bun", "--", script, ...args], {
    cwd: root, env: { ...process.env, PLAYWRIGHT_BROWSERS_PATH: browsers }, stdout: "inherit", stderr: "inherit",
  });
  const interrupt = () => task.kill("SIGINT");
  const terminate = () => task.kill("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const status = await task.exited.finally(() => {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  });
  if (status !== 0) throw Error("Repaired Draw browser stage failed with " + status);
}
