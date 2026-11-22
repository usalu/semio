import { join } from "node:path";

const [generated, rawPort] = Bun.argv.slice(2);
const port = Number(rawPort);
if (!generated || !Number.isInteger(port) || port < 1 || port > 65535) throw Error("Expected the ticket output directory and Draw localhost port");
const url = "http://127.0.0.1:" + port + "/";
const deadline = Date.now() + 10_800_000;
let available = false;
while (Date.now() < deadline) {
  try {
    const response = await fetch(url, { signal: AbortSignal.timeout(3000) });
    available = response.ok && (response.headers.get("content-type") ?? "").startsWith("text/html");
    await response.body?.cancel();
    if (available) break;
  } catch {}
  await Bun.sleep(3000);
}
if (!available) throw Error("Draw's HTTP listener did not become ready on " + port);

const launcher = join(generated, "../draw-native-dev.ts");
const browser = "✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts";
const stamp = Date.now().toString(36);
let status = 0;
for (const locale of ["en", "de"]) {
  const output = join(generated, "br-" + port + "-" + stamp + "-" + locale);
  const stage = "browser-" + (process.env.DRAW_BROWSER_STAGE ?? (port === 6064 ? "canonical" : port === 16064 ? "native" : "installed")) + "-" + locale;
  const task = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", launcher, generated, stage, browser, "browser-acceptance", "--serve", url, "--locale", locale, "--output", output], {
    cwd: process.cwd(), stdout: "inherit", stderr: "inherit",
  });
  const interrupt = () => task.kill("SIGINT"), terminate = () => task.kill("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const exit = await task.exited.finally(() => {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  });
  console.log("[DEBUG] Draw browser journey", port, locale, exit, output);
  status ||= exit;
  if (exit >= 128) break;
}
process.exitCode = status;
