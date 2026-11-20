import { mkdir, open, rm, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";

const generated = join(dirname(import.meta.path), "🗑️generated");
const serve = "http://127.0.0.1:6070/";
const pane = "http://127.0.0.1:6272/";
const acceptance = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🟦️.ts";
const xlsx = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx";
const request = process.argv[2] ?? "io-matrix";
const locale = request === "acceptance-de-on-ready" ? "de" : "en";
const direct = request.endsWith("-direct");
const readyUrl = /^acceptance(?:-de)?-on-ready$/u.test(request) || request === "io-matrix-on-ready" ? serve : request === "pane-on-ready" ? pane : undefined;
const mode = request.replace(/(?:-de)?-on-ready$|-direct$/u, "");
const args = mode === "io-matrix"
  ? ["run", "os-hub:io-matrix", "--", "--serve", serve, "--locale", "en", "--only", "stdio-office/xlsx", "--out", join(generated, mode)]
  : mode === "acceptance" ? [`./${acceptance}`, "run", "--serve", serve, "--entry", "natural-file", "--locale", locale, "--only", "xlsx", "--out", join(generated, locale === "en" ? mode : `${mode}-${locale}`)]
  : mode === "pane" ? [join(dirname(import.meta.path), "probe-xlsx-pane.ts")]
  : mode === "host" ? ["run", "@semio-tech/framework-os-dev:dev-s-react-prototype"]
  : mode === "xlsx-host" ? ["run", "@semio-tech/framework-os-dev:dev-stdio-xlsx-react-prototype"]
  : mode === "describe-office" ? ["run", "workspace:components", "--", "describe", "stdio-office"]
  : mode === "build-office" ? ["cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--", "build", "--locked", "-p", "semio-hub-stdio-office-component", "--lib", "--target", "wasm32-wasip2"]
  : mode === "test-demo" ? ["cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--", "test", "-p", "semio-s-artifact-stdio-xlsx-base", "--lib", "examples::demo"]
  : undefined;
if (args === undefined) throw new Error(`Unknown browser task: ${mode}`);
if (direct) args.splice(2, 0, "--exclude-dependencies");
await mkdir(generated, { recursive: true });
const statusPath = join(generated, `${request}.status`);
await rm(statusPath, { force: true });
const log = await open(join(generated, `${request}.log`), "w");
try {
  if (readyUrl) {
    const owner = Number(process.argv[3]);
    if (!Number.isSafeInteger(owner) || owner < 2) throw new Error("The managed host process ID is required");
    await log.write(`[DEBUG] Waiting for managed host ${owner} at ${readyUrl}\n`);
    for (;;) {
      process.kill(owner, 0);
      const response = await fetch(readyUrl, { signal: AbortSignal.timeout(3000) }).catch(() => undefined);
      await response?.body?.cancel();
      if (response?.ok) break;
      await delay(5000);
    }
    await log.write("[DEBUG] Managed host is ready. Running the XLSX journey.\n");
  }
  const env = mode === "host" || mode === "xlsx-host" ? { ...process.env, S_OS_PORT: mode === "host" ? "6070" : "6272", S_LOCAL_ONLY: "1", SEMIO_VITE_HMR: "0" }
    : mode === "build-office" ? { ...process.env, CARGO_UNSTABLE_FEATURE_UNIFICATION: "true", CARGO_RESOLVER_FEATURE_UNIFICATION: "package" }
    : process.env;
  const cwd = mode === "test-demo" ? join(process.cwd(), xlsx) : mode === "build-office" ? join(process.cwd(), "🌎️hub") : process.cwd();
  const test = Bun.spawn(["repo", ...args], { env, cwd, stdin: "inherit", stdout: log.fd, stderr: log.fd });
  if (mode === "host" || mode === "xlsx-host") await writeFile(join(generated, `${request}.pid`), `${test.pid}\n`);
  const status = await test.exited;
  await writeFile(statusPath, `${status}\n`);
  process.exitCode = status;
} catch (error) {
  await log.write(`[DEBUG] ${String(error)}\n`);
  await writeFile(statusPath, "1\n");
  process.exitCode = 1;
} finally {
  await log.close();
}
