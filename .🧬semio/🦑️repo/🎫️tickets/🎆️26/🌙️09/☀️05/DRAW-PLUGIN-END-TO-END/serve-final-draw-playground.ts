/** 🌐️ Starts the local Draw playground once the certified component is activated. */
import { join } from "node:path";
const generated = Bun.argv[2];
if (!generated) throw Error("Expected the Draw ticket generated directory");
const deadline = Date.now() + 10_800_000;
const activation = join(generated, "draw-native-activate-final.log");
let activated = false;
while (Date.now() < deadline) {
  const log = Bun.file(activation);
  if (await log.exists() && /"command":"activate","stage":"complete"/.test(await log.slice(Math.max(0, log.size - 65_536)).text())) {
    activated = true;
    break;
  }
  await Bun.sleep(2000);
}
if (!activated) throw Error("The final Draw component never activated");
try {
  const ready = await fetch("http://127.0.0.1:6064/", { signal: AbortSignal.timeout(2500) });
  await ready.body?.cancel();
  if (ready.ok && ready.headers.get("content-type")?.startsWith("text/html")) process.exit(0);
} catch {}
const source = process.cwd();
const child = Bun.spawn(["repo", "./📜️script.ts", "dev", "playground", "draw", "react", "prototype"], {
  cwd: source,
  env: { ...process.env, S_OS_PORT: "16064", S_LOCAL_ONLY: "1", SEMIO_LANGUAGE: "en", SEMIO_PLUGIN_CATALOG: join(source, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated") },
  stdout: "inherit",
  stderr: "inherit",
});
const interrupt = () => child.kill("SIGINT");
const terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt);
process.once("SIGTERM", terminate);
process.exitCode = await child.exited.finally(() => {
  process.off("SIGINT", interrupt);
  process.off("SIGTERM", terminate);
});
